mod engine;

use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 웰컴 메시지 출력
    println!("{}", "=========================================================".bold().cyan());
    println!("{}", "🚀 Vesper Harness (v2.0.0) - The 5-Stage Core Engine".bold().green());
    println!("{}", "가재코드 + 레이지코덱스 + 에이더 + SWE-agent 마개조 통합판".dimmed());
    println!("'/task [작업내용]' 을 입력하여 무결성 AI 코딩을 시작하세요.");
    println!("{}", "=========================================================".bold().cyan());

    let mut rl = DefaultEditor::new()?;
    let history_file = ".vesper_history";
    let _ = rl.load_history(history_file);

    // 하네스 코어 엔진(State Machine) 초기화
    let mut vesper_engine = engine::VesperEngine::new();

    loop {
        // 현재 상태에 따른 프롬프트 색상 변경
        let prompt = match vesper_engine.current_stage {
            engine::VesperStage::Idle => "Vesper> ".bold().magenta().to_string(),
            _ => "Vesper(Running)> ".bold().red().to_string(), // 실행 중에는 입력을 다르게 처리 가능
        };

        let readline = rl.readline(&prompt);
        
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }
                let _ = rl.add_history_entry(trimmed);

                // Idle 상태일 때만 새로운 Task를 받음
                if vesper_engine.current_stage == engine::VesperStage::Idle {
                    if trimmed.starts_with("/task ") {
                        let task_desc = trimmed.trim_start_matches("/task ").trim();
                        println!("\n{}", format!("🔥 중앙 통제실: 임무 '{}' 접수 완료. 엔진 구동!", task_desc).yellow().bold());
                        
                        // 5단계 상태 머신 자율 주행 시작
                        vesper_engine.run_pipeline(task_desc).await;
                        
                    } else if trimmed == "/exit" || trimmed == "/quit" {
                        println!("Vesper 하네스를 종료합니다. 안녕히 계세요! 👋");
                        break;
                    } else {
                        println!("{}", "[System] Vesper V2 엔진입니다. '/task [내용]' 형식으로 임무를 하달하세요.".dimmed());
                    }
                } else {
                    println!("{}", "[System] 파이프라인이 가동 중입니다. 잠시 기다려 주세요...".red());
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("\n하네스 종료 중...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    let _ = rl.save_history(history_file);
    Ok(())
}
