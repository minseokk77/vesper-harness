mod engine;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;
use tokio::sync::mpsc;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

struct TerminalModeGuard;

impl Drop for TerminalModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

async fn run_headless(
    initial_cmd: Option<String>,
    resume: bool,
    auto_approve: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(100);
    let (cmd_tx, cmd_rx) = mpsc::channel::<String>(10);
    let engine_task = tokio::spawn(async move {
        let mut vesper_engine = engine::VesperEngine::new(tx.clone());
        if resume && let Err(error) = vesper_engine.resume().await {
            let _ = tx.send(format!("[Resume Error] {error}")).await;
            return Err(error);
        }
        vesper_engine.run_loop(cmd_rx).await;
        Ok::<(), String>(())
    });

    if let Some(command) = initial_cmd {
        cmd_tx.send(command).await?;
    }
    if auto_approve {
        cmd_tx.send("y".to_string()).await?;
    }
    drop(cmd_tx);

    while let Some(message) = rx.recv().await {
        println!("{message}");
    }
    engine_task
        .await?
        .map_err(|error| io::Error::other(format!("Vesper engine failed: {error}")))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Vesper Harness 🚀 - 5-Stage Core Engine for Autonomous AI Coding");
        println!("\nUsage:");
        println!("  vesper [task description]");
        println!("  vesper --yes [task description]  # headless auto-approval");
        println!("  vesper --resume                 # restore interrupted task");
        println!("\nOptions:");
        println!("  -h, --help    Print help information");
        println!("  -y, --yes     Approve once and run without the TUI");
        println!("      --resume  Resume from .vesper/state.json");
        println!("\nExamples:");
        println!("  vesper \"Svelte 5로 버튼 컴포넌트 리팩토링해줘\"");
        println!("  vesper /sprocket \"신규 API 엔드포인트 설계\"");
        println!("\nRun without arguments to enter the interactive TUI mode.");
        return Ok(());
    }

    let resume = args.iter().any(|arg| arg == "--resume");
    let auto_approve = args.iter().any(|arg| arg == "--yes" || arg == "-y");
    let task_args: Vec<String> = args
        .iter()
        .filter(|arg| !matches!(arg.as_str(), "--resume" | "--yes" | "-y"))
        .cloned()
        .collect();
    if resume && !task_args.is_empty() {
        return Err("--resume은 새 작업 설명과 함께 사용할 수 없습니다.".into());
    }
    let initial_cmd = if task_args.is_empty() {
        None
    } else {
        let input = task_args.join(" ");
        Some(if input.starts_with('/') {
            input
        } else {
            format!("/task {input}")
        })
    };
    if auto_approve {
        if initial_cmd.is_none() && !resume {
            return Err("--yes에는 작업 설명 또는 --resume이 필요합니다.".into());
        }
        return run_headless(initial_cmd, resume, true).await;
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let terminal_mode_guard = TerminalModeGuard;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut logs: Vec<String> = Vec::new();
    let mut input = Input::default();

    let (cmd_tx, cmd_rx) = mpsc::channel::<String>(10);

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut vesper_engine = engine::VesperEngine::new(tx_clone.clone());
        let _ = tx_clone
            .send(format!(
                "🚀 Vesper Harness (v{}) - TUI Engine Started",
                env!("CARGO_PKG_VERSION")
            ))
            .await;
        let _ = tx_clone.send("'/task [작업내용]' 또는 '/sprocket [설계내용]' 을 입력하여 무결성 AI 코딩을 시작하세요. (종료: ESC)".to_string()).await;

        if resume && let Err(error) = vesper_engine.resume().await {
            let _ = tx_clone.send(format!("[Resume Error] {error}")).await;
        }
        vesper_engine.run_loop(cmd_rx).await;
    });

    if let Some(command) = initial_cmd {
        let _ = cmd_tx.try_send(command);
    }

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let header = Paragraph::new(format!(
                "Vesper Harness v{} - 5-Stage Core Engine (Press ESC to quit)",
                env!("CARGO_PKG_VERSION")
            ))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(header, chunks[0]);

            // 최신 50개 로그만 보여주기 (스크롤 구현 대신)
            let log_len = logs.len();
            let display_logs = if log_len > 50 {
                &logs[log_len - 50..]
            } else {
                &logs[..]
            };
            let log_items: Vec<ListItem> = display_logs
                .iter()
                .map(|msg| ListItem::new(msg.as_str()))
                .collect();
            let log_list = List::new(log_items)
                .block(Block::default().borders(Borders::ALL).title("Agent Logs"));
            f.render_widget(log_list, chunks[1]);

            let input_widget = Paragraph::new(input.value())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Vesper> "));
            f.render_widget(input_widget, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    let val = input.value().to_string();
                    if val == "/quit" || val == "/exit" {
                        break;
                    }
                    if !val.is_empty() {
                        logs.push(format!("Vesper> {}", val));
                        let _ = cmd_tx.send(val).await;
                        input.reset();
                    }
                }
                _ => {
                    input.handle_event(&Event::Key(key));
                }
            }
        }

        while let Ok(msg) = rx.try_recv() {
            logs.push(msg);
            if logs.len() > 200 {
                logs.remove(0);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    std::mem::forget(terminal_mode_guard);
    Ok(())
}
