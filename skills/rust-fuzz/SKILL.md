# Skill: Fuzzing com cargo-fuzz

## Quando usar

Quando o estudante pedir para fazer fuzzing de código Rust que processa entrada não confiável (parsers, validação, autenticação).

## Regra de ouro (obrigatória)

**NUNCA rode fuzzing no diretório principal do projeto.** O cargo-fuzz gera dezenas de GB em `fuzz/target/`. Sempre:
1. Pergunte o contexto (ambiente, autorização, tempo de execução).
2. Rode em um diretório de trabalho separado ou com `CARGO_TARGET_DIR` apontando para `/tmp`.
3. Limite o tempo de execução (`-- -max_total_time=60` para testes rápidos).
4. Avise o estudante sobre o consumo de disco.

## Pré-requisitos

- Rust nightly instalado (`rustup install nightly`).
- cargo-fuzz instalado (`cargo install cargo-fuzz`).
- Código a ser fuzzado em `src/lib.rs` (não em binários).

## Roteiro

1. Rode `cargo fuzz init` no diretório do projeto.
2. Edite `fuzz/fuzz_targets/fuzz_target_1.rs` para chamar a função alvo.
3. Rode `cargo +nightly fuzz run fuzz_target_1 -- -max_total_time=60` (60 segundos).
4. Se encontrar crash, o artefato fica em `fuzz/artifacts/`.
5. Formate o achado no formato fixo da skill vulnerability-analysis.

## Guardrails

- Nunca commite `fuzz/target/` nem `fuzz/corpus/` no Git.
- Sempre adicione `fuzz/target/` ao .gitignore.
- Se o disco estiver com pouco espaço, use `CARGO_TARGET_DIR=/tmp/fuzz-target`.
