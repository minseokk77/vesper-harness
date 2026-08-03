use colored::Colorize;
use tokio::time::{sleep, Duration};
use tokio::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VesperStage {
    Idle,
    Analyze,
    RiskScan,
    Plan,
    Execute,
    Verify,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SkillSource {
    Obsidian,
    Notion,
    Hybrid,
}

pub struct VesperEngine {
    pub current_stage: VesperStage,
    pub task_description: String,
    pub retry_count: u32,
    pub workspace_dir: String,
    pub skill_source: SkillSource,
    pub rules_dir: String, // 자동 발동 룰(Rules) 경로 추가
}

impl VesperEngine {
    pub fn new() -> Self {
        Self {
            current_stage: VesperStage::Idle,
            task_description: String::new(),
            retry_count: 0,
            workspace_dir: ".vesper".to_string(),
            skill_source: SkillSource::Hybrid,
            rules_dir: "C:\\Users\\minse\\Documents\\Min\\Min\\ai agent\\rules".to_string(),
        }
    }

    async fn write_ipc_file(&self, filename: &str, content: &str) {
        let path = Path::new(&self.workspace_dir);
        if !path.exists() {
            let _ = fs::create_dir_all(path).await;
        }
        
        let file_path = path.join(filename);
        match fs::write(&file_path, content).await {
            Ok(_) => println!("{}", format!("   [IPC] 📝 {} 파일 업데이트 완료", filename).dimmed()),
            Err(e) => println!("{}", format!("   [IPC Error] {} 작성 실패: {}", filename, e).red()),
        }
    }

    async fn scan_obsidian(&self, required_tags: &Vec<&str>) -> Option<String> {
        if required_tags.contains(&"execute") {
            return Some(String::from("<<PONYTAIL SKILL INJECTED FROM OBSIDIAN>>"));
        }
        None
    }

    async fn scan_notion(&self, required_tags: &Vec<&str>) -> Option<String> {
        if required_tags.contains(&"risk") {
            return Some(String::from("<<CAVEMAN SKILL INJECTED FROM NOTION>>"));
        } else if required_tags.contains(&"frontend") {
            return Some(String::from("<<LIQUID GLASS FRONTEND SKILL FROM NOTION>>"));
        }
        None
    }

    async fn fetch_skill(&self, task_context: &str, required_tags: Vec<&str>) -> String {
        println!("{}", format!("   [Skill Router] 🌐 '{}' 작업을 위한 최적 스킬 검색 시작...", task_context).magenta());
        sleep(Duration::from_millis(500)).await;
        let mut final_skills = String::new();

        match self.skill_source {
            SkillSource::Obsidian => { if let Some(s) = self.scan_obsidian(&required_tags).await { final_skills.push_str(&s); } }
            SkillSource::Notion => { if let Some(s) = self.scan_notion(&required_tags).await { final_skills.push_str(&s); } }
            SkillSource::Hybrid => {
                if let Some(s_obs) = self.scan_obsidian(&required_tags).await { final_skills.push_str(&format!("{}\n", s_obs)); }
                if let Some(s_not) = self.scan_notion(&required_tags).await { final_skills.push_str(&format!("{}\n", s_not)); }
            }
        }
        
        if final_skills.is_empty() { String::from("<<NO SPECIFIC SKILL INJECTED>>") } else { final_skills }
    }

    /// (NEW) 파일 확장자 기반 자동 룰(Rules) 매칭 모듈
    async fn fetch_auto_rules(&self, target_files: Vec<&str>) -> String {
        println!("{}", format!("   [Rule Engine] ⚙️ 타겟 파일({:?})의 YAML globs 자동 스캔 중...", target_files).bright_cyan());
        sleep(Duration::from_millis(1000)).await;
        
        let mut rules_injected = String::new();
        
        // Mock 로직: 파일 확장자에 따라 숨겨진 룰 자동 주입
        for file in target_files {
            if file.ends_with(".svelte") {
                println!("   [Rule Engine] 🎯 '*.svelte' 감지 ➡️ 'Svelte-State' 컨벤션 룰 암묵적 장착 완료!");
                rules_injected.push_str("<< AUTO-RULE: SVELTE STATE MANAGEMENT PATTERN >>\n");
            }
            if file.ends_with(".rs") || file.ends_with(".ts") {
                println!("   [Rule Engine] 🎯 '*.rs/*.ts' 감지 ➡️ 'Core-Security' 보안 룰 암묵적 장착 완료!");
                rules_injected.push_str("<< AUTO-RULE: CORE SECURITY POLICIES (No SQLi, No Plaintext passwords) >>\n");
            }
        }
        
        rules_injected
    }

    pub async fn run_pipeline(&mut self, task: &str) {
        self.task_description = task.to_string();
        self.current_stage = VesperStage::Analyze;

        loop {
            match self.current_stage {
                VesperStage::Idle => break,
                VesperStage::Analyze => {
                    println!("\n{}", "=============================================".cyan());
                    println!("{}", "🟡 [Stage 1: Analyze] 가재코드 심층 분석 & 스킬+룰 융합".bold().yellow());
                    println!("목표: {}", self.task_description);
                    
                    // 1. 스킬 라우터 호출
                    let skill_context = self.fetch_skill(&self.task_description, vec!["frontend"]).await;
                    
                    // 2. 타겟 파일 분석 (Mock: src/App.svelte 와 src/main.rs 를 건드린다고 가정)
                    let target_files = vec!["src/App.svelte", "src/main.rs"];
                    
                    // 3. 룰(Rules) 라우터 호출
                    let auto_rules = self.fetch_auto_rules(target_files).await;
                    
                    let context_content = format!(
                        "# Project Context\n\nTask: {}\n\n## Auto-Injected Skills\n{}\n\n## Auto-Triggered Rules\n{}\n\n## File Map\n- src/main.rs\n- src/App.svelte", 
                        self.task_description, skill_context, auto_rules
                    );
                    self.write_ipc_file("context.md", &context_content).await;
                    
                    self.current_stage = VesperStage::RiskScan;
                }
                VesperStage::RiskScan => {
                    println!("\n{}", "=============================================".cyan());
                    println!("{}", "🟠 [Stage 2: Risk Scan] 저비용 취약점 우선 탐색".bold().truecolor(255, 165, 0));
                    
                    let caveman_skill = self.fetch_skill("risk scan", vec!["risk"]).await;
                    let risk_content = format!("# Risk Assessment\n\n{}\n\n⚠️ **WARNING**: `src/engine.rs` is highly coupled.", caveman_skill);
                    self.write_ipc_file("risk.md", &risk_content).await;

                    self.current_stage = VesperStage::Plan;
                }
                VesperStage::Plan => {
                    println!("\n{}", "=============================================".cyan());
                    println!("{}", "🔵 [Stage 3: Plan] 레이지코덱스 작업 계획 수립".bold().blue());
                    sleep(Duration::from_millis(1000)).await;
                    
                    let plan_content = "# Execution Plan\n\n- [ ] 1. Apply skills\n- [ ] 2. Apply auto-rules\n- [ ] 3. Verify";
                    self.write_ipc_file("plan.md", plan_content).await;
                    
                    self.current_stage = VesperStage::Execute;
                }
                VesperStage::Execute => {
                    println!("\n{}", "=============================================".cyan());
                    println!("{}", "🟣 [Stage 4: Execute] 작업 실행 및 에이더 자동 세이브".bold().magenta());
                    println!("- (Mock) 안전망: Git 임시 커밋 완료 (WIP_BEFORE_EXECUTE)");
                    
                    let execution_skills = self.fetch_skill("coding loop", vec!["execute"]).await;
                    let instruction_content = format!("# ACTIVE INSTRUCTION\n\n{}\n\nRead `plan.md` and execute step by step.", execution_skills);
                    self.write_ipc_file("instruction.md", &instruction_content).await;
                    
                    sleep(Duration::from_millis(2500)).await;
                    println!("- (Mock) AI 에이전트 코딩 완료 신호 감지!");
                    
                    self.current_stage = VesperStage::Verify;
                }
                VesperStage::Verify => {
                    println!("\n{}", "=============================================".cyan());
                    println!("{}", "🟢 [Stage 5: Verify] 가재코드 깐깐 검증 및 롤백 시스템".bold().green());
                    sleep(Duration::from_millis(1000)).await;
                    
                    let success = true; 
                    if success {
                        println!("- (Mock) 모든 테스트 통과! 증거 확보 완료 ✅");
                        self.write_ipc_file("verify_log.md", "# Verification\nAll tests passed successfully.").await;
                        println!("{}", "\n🎉 Vesper Harness: 스킬과 자동 발동 룰(Rules)이 융합된 임무를 성공적으로 마쳤습니다!".bold().green());
                        self.current_stage = VesperStage::Idle;
                    } else {
                        self.retry_count += 1;
                        if self.retry_count >= 3 {
                            println!("{}", "🚨 (Mock) 3회 연속 검증 실패! 롤백 중...".bold().red());
                            self.current_stage = VesperStage::Idle;
                        } else {
                            println!("{}", format!("⚠️ (Mock) 테스트 실패 ({}회). Stage 4로 회귀합니다.", self.retry_count).yellow());
                            self.current_stage = VesperStage::Execute;
                        }
                    }
                }
            }
        }
    }
}
