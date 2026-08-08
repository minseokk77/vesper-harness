# Vesper Harness

계획, 위험 분석, 사용자 승인, 코드 실행, 검증을 하나의 흐름으로 연결하는 자율 AI 코딩 코어 엔진입니다.

Vesper Harness는 작업을 바로 수정하지 않습니다. 먼저 계획과 위험 보고서를 만들고 사용자의 명시적인 승인을 받은 뒤, Codex나 Aider 같은 실제 코딩 에이전트를 실행합니다. 각 단계는 `.vesper/state.json`에 저장되므로 중단된 작업을 다시 이어갈 수 있습니다.

## 빠른 시작

### 1. 설치

Node.js 18 이상이 설치된 Windows 또는 Linux x64 환경에서 다음 명령을 실행합니다.

```powershell
npm install --global vesper-harness
```

설치를 확인합니다.

```powershell
vesper --help
```

Windows/Linux x64에서는 GitHub Release의 사전 빌드 바이너리를 자동으로 내려받습니다. 바이너리를 받을 수 없으면 로컬 Rust 빌드를 시도하므로 이 경우에는 Rust와 Cargo가 필요합니다.

### 2. 코딩 에이전트 선택

Vesper는 기본적으로 `aider`를 실행합니다. ChatGPT 계정의 Codex 사용 한도를 이용하려면 먼저 Codex에 로그인합니다.

```powershell
codex login
codex login status
```

현재 PowerShell 창에서 Codex를 Vesper 실행 에이전트로 설정합니다.

```powershell
$env:VESPER_AGENT_PROGRAM = "codex"
$env:VESPER_AGENT_ARGS_JSON = '["exec","--sandbox","workspace-write","--color","never","--skip-git-repo-check","{instruction}"]'
```

Windows 사용자 환경변수로 영구 저장하려면 다음 명령을 사용한 뒤 PowerShell을 새로 엽니다.

```powershell
[Environment]::SetEnvironmentVariable("VESPER_AGENT_PROGRAM", "codex", "User")
[Environment]::SetEnvironmentVariable("VESPER_AGENT_ARGS_JSON", '["exec","--sandbox","workspace-write","--color","never","--skip-git-repo-check","{instruction}"]', "User")
```

이 방식은 OpenAI API 키를 사용하지 않으며, 로그인한 계정의 Codex 사용 한도가 적용됩니다.

### 3. 프로젝트에서 실행

AI가 수정할 프로젝트 폴더로 이동한 다음 Vesper를 실행합니다.

```powershell
cd C:\path\to\your-project
vesper "로그인 폼의 유효성 검사를 추가하고 테스트해줘"
```

Vesper는 분석, 위험 검사, 계획 수립을 완료한 뒤 승인을 기다립니다. 생성된 `.vesper/plan.md`와 `.vesper/risk.md`를 확인하고 다음 중 하나를 입력합니다.

| 입력 | 동작 |
|---|---|
| `y`, `yes`, `승인` | 계획을 승인하고 코드 실행 시작 |
| `n`, `no`, `거절`, `취소` | 작업 취소 |
| 일반 문장 | 피드백을 반영해 `plan.md` 수정 후 다시 승인 대기 |

예시:

```text
Vesper> 테스트에 cargo clippy도 추가해줘
Vesper> y
```

## 작동 흐름

```text
Idle
  → Analyze
  → Risk Scan
  → Plan
  → Pending Approval
  → Execute
  → Verify
  → Idle
```

| 단계 | 역할 |
|---|---|
| Analyze | 작업과 프로젝트 컨텍스트 분석 |
| Risk Scan | 보안 및 변경 위험 확인 |
| Plan | 실행 계획 생성 |
| Pending Approval | 사용자의 승인, 거절 또는 수정 피드백 대기 |
| Execute | 설정된 실제 코딩 에이전트 실행 및 로그 스트리밍 |
| Verify | 설정된 테스트·빌드 명령 실행 |

검증에 실패하면 에러 로그를 바탕으로 `.vesper/fix_plan.md`를 만들고 최대 3회까지 수정과 검증을 다시 시도합니다.

## 실행 방법

### 대화형 TUI

작업 설명 없이 실행하면 TUI 입력창이 열립니다.

```powershell
vesper
```

```text
/task README의 설치 설명을 개선해줘
/sprocket 신규 API 엔드포인트를 설계해줘
/export-ci
```

`Esc`, `/quit`, `/exit`로 종료할 수 있습니다.

### 작업 설명을 바로 전달

```powershell
vesper "파서 오류 처리를 개선하고 테스트해줘"
```

계획 생성 후 TUI에서 승인을 기다립니다.

### Headless 자동 승인

CI나 자동화 스크립트에서는 `--yes`를 사용할 수 있습니다.

```powershell
vesper --yes "cargo fmt와 cargo test를 실행하고 실패 원인을 수정해줘"
```

`--yes`는 계획을 화면에서 검토하지 않고 한 번 자동 승인합니다. 신뢰할 수 있는 저장소와 작업에서만 사용하세요.

### 중단된 작업 재개

```powershell
vesper --resume
```

Execute 또는 Verify 도중 종료된 작업은 중복 실행을 방지하기 위해 다시 승인 대기로 복원됩니다. 계획을 확인한 뒤 `y`를 입력하면 안전하게 재시도합니다.

## 환경 설정

| 환경변수 | 설명 | 기본값 |
|---|---|---|
| `VESPER_AGENT_PROGRAM` | 실행할 코딩 에이전트 명령 | `aider` |
| `VESPER_AGENT_ARGS_JSON` | 에이전트 인자 JSON 배열 | `["--message","{instruction}"]` |
| `VESPER_VERIFY_PROGRAM` | 검증 프로그램 | `cargo` |
| `VESPER_VERIFY_ARGS_JSON` | 검증 인자 JSON 배열 | `["test","--all-targets"]` |
| `VESPER_RULES_DIR` | 자동 규칙 폴더 | `./rules` |
| `VESPER_SKILLS_DIR` | 스킬 Markdown 폴더 | `./skills` |
| `GEMINI_API_KEY` | 위험 분석과 복구 계획에 사용할 선택적 Gemini 키 | 없음 |
| `GROQ_API_KEY` | Gemini 키가 없을 때 사용할 선택적 Groq 키 | 없음 |

`VESPER_AGENT_ARGS_JSON`에서는 다음 자리표시자를 사용할 수 있습니다.

| 자리표시자 | 내용 |
|---|---|
| `{task}` | 사용자가 입력한 원본 작업 설명 |
| `{instruction}` | 계획·위험 문서 위치를 포함한 전체 실행 지시문 |

### Aider 사용

```powershell
python -m pip install aider-chat
$env:VESPER_AGENT_PROGRAM = "aider"
$env:VESPER_AGENT_ARGS_JSON = '["--message","{instruction}"]'
vesper "오류 처리를 개선해줘"
```

사용할 모델과 API 인증은 Aider 설정을 따릅니다.

### 사용자 지정 에이전트 사용

표준 출력과 표준 오류를 사용하는 실행 파일이라면 연결할 수 있습니다.

```powershell
$env:VESPER_AGENT_PROGRAM = "my-agent"
$env:VESPER_AGENT_ARGS_JSON = '["run","--task","{task}","--prompt","{instruction}"]'
```

Vesper는 subprocess의 stdout과 stderr를 실시간으로 TUI 또는 headless 로그에 전달하고, 종료 코드가 0이 아니면 실패로 처리합니다.

### 검증 명령 변경

Node.js 프로젝트 예시:

```powershell
$env:VESPER_VERIFY_PROGRAM = "npm"
$env:VESPER_VERIFY_ARGS_JSON = '["test"]'
```

Rust 프로젝트 예시:

```powershell
$env:VESPER_VERIFY_PROGRAM = "cargo"
$env:VESPER_VERIFY_ARGS_JSON = '["test","--all-targets"]'
```

## `.vesper` 작업 파일

Vesper는 실행한 프로젝트 루트에 다음 파일을 생성합니다.

| 파일 | 용도 |
|---|---|
| `.vesper/context.md` | 작업에 주입할 컨텍스트 |
| `.vesper/risk.md` | 위험 검사 결과 |
| `.vesper/plan.md` | 승인 대상 실행 계획 |
| `.vesper/instruction.md` | 코딩 에이전트 지시문 |
| `.vesper/state.json` | 재개 가능한 파이프라인 상태 |
| `.vesper/fix_plan.md` | 실패 후 생성한 복구 계획 |
| `.vesper/verify_log.md` | 검증 결과 |

프로젝트 Git 저장소에서는 `.vesper/`를 `.gitignore`에 추가하는 것을 권장합니다.

## 소스에서 빌드

Rust 2024 edition을 지원하는 Rust 도구 체인이 필요합니다.

```powershell
git clone https://github.com/minseokk77/vesper-harness.git
cd vesper-harness
cargo test --all-targets
cargo build --release
.\target\release\vesper-harness.exe --help
```

스킬 폴더의 Markdown 호환성을 검사하려면 다음 명령을 실행합니다.

```powershell
cargo run --bin test_skills
```

## 업데이트 및 제거

최신 버전으로 업데이트:

```powershell
npm install --global vesper-harness@latest
```

제거:

```powershell
npm uninstall --global vesper-harness
```

## 문제 해결

### `에이전트 'codex' 실행 실패`

`codex`가 PATH에 있는지와 로그인 상태를 확인합니다.

```powershell
Get-Command codex
codex login status
```

### `VESPER_AGENT_ARGS_JSON은 JSON 문자열 배열이어야 합니다`

환경변수는 JSON 객체가 아니라 문자열 배열이어야 합니다.

```powershell
$env:VESPER_AGENT_ARGS_JSON = '["exec","--sandbox","workspace-write","{instruction}"]'
```

### `저장된 상태와 복구 가능한 백업을 읽을 수 없습니다`

현재 프로젝트에 `.vesper/state.json`이 없거나 재개할 작업이 없습니다. 새 작업을 시작하세요.

```powershell
vesper "새 작업 설명"
```

### npm 설치 중 바이너리 다운로드 실패

GitHub Release 접근이 차단됐는지 확인합니다. Rust가 설치되어 있다면 설치 스크립트가 로컬 Release 빌드를 시도합니다.

```powershell
rustc --version
cargo --version
```

## 보안 참고

- API 키나 로그인 토큰을 프롬프트, `.vesper` 문서 또는 Git 저장소에 넣지 마세요.
- 계획과 `risk.md`를 확인한 뒤 승인하세요.
- `--yes`는 격리된 CI나 신뢰할 수 있는 프로젝트에서만 사용하세요.
- Vesper가 실행하는 에이전트는 현재 프로젝트 파일을 변경하고 명령을 실행할 수 있습니다.

## 링크

- [GitHub 저장소](https://github.com/minseokk77/vesper-harness)
- [GitHub Releases](https://github.com/minseokk77/vesper-harness/releases)
- [npm 패키지](https://www.npmjs.com/package/vesper-harness)
- [업데이트 로그](docs/V3_UPDATE_LOG.md)

## License

MIT License. See [LICENSE](LICENSE).
