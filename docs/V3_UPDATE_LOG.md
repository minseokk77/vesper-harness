# Vesper Harness v3.2.0 Core Engine 업데이트 로그

**작업 일시**: 2026-08-08
**대상 프로젝트**: `vesper-harness` (Rust TUI 코어 엔진 및 CLI 래퍼)

---

## 1. CLI(명령어 인터페이스) 기능 완벽 연동
이전에는 TUI에 진입해야만 `/task` 명령어를 사용할 수 있었으나, 이제 **AI 에이전트나 스크립트가 외부에서 직접 호출할 수 있도록 개선**되었습니다.

- **Rust 바이너리 파싱 (`src/main.rs`)**
  - 프로그램 실행 시 전달된 `std::env::args()`를 파싱하여, 즉시 `/task [입력값]` 형태로 변환 후 5단계 엔진 큐에 주입하도록 변경.
  - `--help` 옵션을 구현하여 CLI 사용법을 출력.
- **Node.js 래퍼 폴백 (`bin/run.js`)**
  - 사전 빌드된 바이너리(`vesper.exe`)가 없을 경우, 에러를 내뿜고 죽는 대신 `cargo run --release --` 명령으로 자동 우회(Fallback)하도록 수정.
- **설치 스크립트 예외 처리 (`scripts/install.js`)**
  - 폴백 빌드가 실패할 경우 침묵하지 않고 `process.exit(1)`로 에러를 전파하여 상위 스크립트(pnpm 등)가 실패를 인지할 수 있도록 보완.

## 2. 중간 승인(Human-in-the-Loop) 시스템 도입
AI가 짠 계획(Plan)을 무조건 실행(Execute)하는 위험을 방지하기 위해 **승인(Approval) 단계**를 신설했습니다.

- **상태 머신(Event Loop) 주도권 이양 (`src/engine.rs`)**
  - 기존에는 `run_pipeline` 함수가 블로킹(Blocking) 루프를 돌며 외부 입력을 무시했으나, 입력을 수신하는 `cmd_rx` 채널을 `VesperEngine` 내부(`run_loop`)로 이관.
- **`PendingApproval` 스테이지 신설**
  - `Plan(3단계)` 작성이 끝나면 엔진이 `PendingApproval` 상태로 일시 정지(Pause).
  - TUI에서 `y`, `yes`, `승인` 등을 입력하면 `Execute(4단계)`로 진입.
  - `n`, `no`, `거절` 입력 시 작업을 취소하고 `Idle` 상태로 롤백.
  - 기타 피드백 텍스트 입력 시 `plan.md`를 수정하고 승인 대기를 유지하여, 수정된 계획에 다시 명시적 승인을 받음.

## 3. 결과 및 기대 효과
- **완전한 자동화 가능**: `pnpm exec vesper "태스크"` 명령어로 Aider, SWE-agent, Antigravity 등 어떤 외부 시스템에서도 Vesper를 서브 모듈로 호출 가능해짐 (AI가 AI를 부르는 워크플로우 달성).
- **안전성 확보**: 코딩/실행 전에 인간(Human)이 개입하여 리스크를 확인할 수 있게 되어 프로덕션 환경에서의 안전성 대폭 상승.
- **구조적 확장성**: 이벤트 루프 구조를 리팩토링하여, 향후 '채팅 중단', '결과물 재요청' 등 다양한 양방향 소통 기능 추가가 용이해짐.

## 4. Phase 1 코어 보강 (GUI 제외)

- 일반 텍스트 피드백은 더 이상 승인으로 처리하지 않습니다. `plan.md`를 수정한 뒤 `PendingApproval`을 유지하며 명시적인 `y`가 필요합니다.
- `VESPER_AGENT_PROGRAM`과 `VESPER_AGENT_ARGS_JSON`으로 실제 Aider/SWE-agent/사용자 지정 실행 파일을 연결하고 stdout/stderr를 TUI 또는 headless stdout에 실시간 전달합니다.
- 각 단계 전환을 `.vesper/state.json`에 저장하며 `vesper --resume`으로 복원합니다. Execute/Verify 도중 중단된 작업은 외부 프로세스를 무작정 중복 실행하지 않도록 다시 승인 대기로 복원합니다.
- `vesper --yes "태스크"`는 TUI를 열지 않는 자동화 경로입니다. GUI/Svelte/Tauri 클라이언트는 이번 범위에서 진행하지 않았습니다.
