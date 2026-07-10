# AutoFeat

[![CI](https://github.com/DeathSurfing/autofeat/actions/workflows/ci.yml/badge.svg)](https://github.com/DeathSurfing/autofeat/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Interactive AI-powered feature engineering CLI.

Unlike traditional AutoML tools that operate as black boxes, AutoFeat
keeps the user in control. The AI proposes preprocessing workflows,
explains every decision, and executes transformations through Rust
tools powered by `featrs`, while the user can inspect, modify, accept,
or reject every step.

```text
AI proposes → User reviews → User edits → AI explains → Pipeline evaluated → Repeat
```

## Installation

```bash
cargo install autofeat
```

Or build from source:

```bash
git clone https://github.com/DeathSurfing/autofeat
cd autofeat
cargo build --release
```

## Usage

```bash
autofeat --dataset data.csv
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `A` | Agent — live reasoning, tool execution, conversation |
| `D` | Dataset — schema, statistics, null counts, distributions |
| `W` | Workflow — interactive DAG editor |
| `T` | Tools — execution history, runtime, outputs |
| `S` | Settings — general, LLM, pipeline, evaluation config |
| `H` | Help — keyboard shortcuts |
| `Enter` | Edit node |
| `Space` | Disable / enable node |
| `I` | AI suggestions |
| `R` | Review pipeline |
| `Ctrl+S` | Save |
| `Q` | Quit |

## Architecture

```
                    User
                      │
                      ▼
               Ratatui Interface
                      │
                      ▼
              Workflow Controller
                      │
          ┌───────────┴───────────┐
          ▼                       ▼
     AI Planning Agent      Workflow Graph
          │                       │
          ▼                       ▼
     Tool Registry          Editable Pipeline
          │
          ▼
     featrs
          │
          ▼
     Polars
```

The LLM only plans and reasons — every computation is performed by Rust
tools through `featrs`.

## Tech Stack

- **Rust** — performance and safety
- **Polars** — DataFrame operations
- **featrs** — feature engineering transformers
- **Ratatui** — terminal UI
- **rig** — LLM orchestration (OpenRouter)
- **Tokio** — async runtime

## License

MIT
