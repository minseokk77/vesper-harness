mod engine;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use tokio::sync::mpsc;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut logs: Vec<String> = Vec::new();
    let mut input = Input::default();
    
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(10);
    
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut vesper_engine = engine::VesperEngine::new(tx_clone.clone());
        let _ = tx_clone.send("🚀 Vesper Harness (v3.0.0) - TUI Engine Started".to_string()).await;
        let _ = tx_clone.send("'/task [작업내용]' 또는 '/sprocket [설계내용]' 을 입력하여 무결성 AI 코딩을 시작하세요. (종료: ESC)".to_string()).await;
        
        while let Some(cmd) = cmd_rx.recv().await {
            if cmd == "/export-ci" {
                let _ = tx_clone.send("📦 GitHub Actions 워크플로우를 생성합니다...".to_string()).await;
                vesper_engine.export_github_workflow().await;
                let _ = tx_clone.send("✅ `.github/workflows/vesper-agent.yml` 생성 완료.".to_string()).await;
            } else if cmd.starts_with("/sprocket ") {
                let task_desc = cmd.trim_start_matches("/sprocket ").trim();
                let _ = tx_clone.send(format!("⚙️ Sprocket: 하드웨어 태스크 '{}' 접수 완료. 5단계 엔진 구동!", task_desc)).await;
                vesper_engine.run_pipeline(&format!("sprocket 하드웨어 {}", task_desc)).await;
            } else if cmd.starts_with("/task ") {
                let task_desc = cmd.trim_start_matches("/task ").trim();
                let _ = tx_clone.send(format!("🔥 중앙 통제실: 임무 '{}' 접수 완료. 엔진 구동!", task_desc)).await;
                vesper_engine.run_pipeline(task_desc).await;
            } else {
                let _ = tx_clone.send("[System] '/task [내용]' 또는 '/sprocket [내용]' 명령어를 사용하세요.".to_string()).await;
            }
        }
    });

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ].as_ref())
                .split(f.size());

            let header = Paragraph::new("Vesper Harness V3.0 - 5-Stage Core Engine (Press ESC to quit)")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(header, chunks[0]);

            // 최신 50개 로그만 보여주기 (스크롤 구현 대신)
            let log_len = logs.len();
            let display_logs = if log_len > 50 { &logs[log_len - 50..] } else { &logs[..] };
            let log_items: Vec<ListItem> = display_logs.iter()
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

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
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
    Ok(())
}
