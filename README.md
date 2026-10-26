# DIO Agent — Especialização em Cibersegurança (Rust)

Versão em **Rust** da especialização em segurança do [DIO Agent](https://github.com/digitalinnovationone/dio-agent), usando o crate [rusqlite](https://crates.io/crates/rusqlite).

## Evolução: de mentor a assistente de segurança

O projeto começou como um mentor que analisa e orienta. Ao longo das evoluções, tornou-se um **assistente de segurança completo** que analisa, testa, protege e corrige (com aprovação humana).

### Skills

| Skill | O que faz |
|-------|-----------|
| `vulnerability-analysis` | Analisa código e cenários no formato fixo (Contexto, Descrição, Impacto, Probabilidade, Severidade, MITRE ATT&CK, Correção, Validação, Próximo passo). |
| `security-testing` | Regra de ouro: perguntar o contexto (ambiente, autorização, serviços expostos, criticidade) antes de sugerir qualquer comando. |
| `rust-dependency-audit` | Roda `cargo audit` e formata vulnerabilidades no formato fixo. |
| `rust-code-scan` | Roda `semgrep --config=p/rust` e valida falsos positivos lendo o código. |
| `rust-policy-check` | Roda `cargo deny check` e interpreta advisories, licenses, bans e sources. |
| `rust-autofix` | Propõe correção, mostra diff, valida (build/test/deny/audit) e **pede aprovação** antes de commitar. |

### CI/CD (GitHub Actions)

O workflow `.github/workflows/security.yml` roda a cada push/PR:

- `cargo-audit`: verifica vulnerabilidades no `Cargo.lock`.
- `cargo-deny`: verifica políticas (advisories, licenses, bans, sources).
- `build-and-test`: compila e roda testes (bloqueia merge se falhar).
- `security-summary`: resume os resultados.

## Resultado dos testes

    $ cargo run --bin vulneravel
    Login normal: Acesso Permitido
    SQLi (bypass): Acesso Negado    (corrigido pelo rust-autofix)

    $ cargo run --bin corrigido
    Login normal: Acesso Permitido
    SQLi (bypass): Acesso Negado

## O que tem aqui

- `src/bin/vulneravel.rs`: login com `format!` concatenando entrada (vulnerável, mas corrigido pelo rust-autofix).
- `src/bin/corrigido.rs`: login com consulta parametrizada (seguro).
- `agent/knowledge/security-basics.md`: fundamentos de segurança (CID, MITRE, SQLi).
- `skills/`: 6 skills de segurança (análise, testes, auditoria, SAST, políticas, correção).
- `AGENTS.md`: fonte de verdade do agente.
- `deny.toml`: política de dependências para `cargo-deny`.
- `.github/workflows/security.yml`: CI/CD de segurança.

## Como testar

    cargo run --bin vulneravel
    cargo run --bin corrigido
    cargo audit
    cargo deny check
    cargo build
    cargo test

## Comparação Python × Rust

| Conceito | Python (sqlite3) | Rust (rusqlite) |
|----------|------------------|-----------------|
| Conexão | `sqlite3.connect('db')` | `Connection::open("db")?` |
| Query vulnerável | `f"SELECT ... '{usuario}'"` | `format!("SELECT ... '{}'", usuario)` |
| Placeholder | `?` | `?1`, `?2` |
| Parâmetros | `(usuario, senha)` | `params![usuario, senha]` |
| Execução | `cursor.execute(query, params)` | `stmt.query_row(params, ...)` |
| Erro | `try/except` | `Result<T>` com `?` |

## Versão em Python

Este projeto tem uma versão em Python: [dio-agent-security](https://github.com/wil-ckaew/dio-agent-security).

## Limitação conhecida

O harness Hermes não força o modelo a abrir o `SKILL.md`; o formato fixo literal é usado quando invocado explicitamente. Detalhes no `AGENTS.md`.

## Licença

MIT.
