# Skill: Cobertura de Testes com cargo-llvm-cov

## Quando usar

Quando o estudante pedir para medir a cobertura de testes (quanto do código é executado pelos testes).

## Regra de ouro

**A PRIMEIRA resposta deve conter APENAS perguntas de contexto.** Pergunte: ambiente, autorização, e qual limite de cobertura deseja.

## Roteiro

1. Rode `cargo llvm-cov --text` para resumo.
2. Rode `cargo llvm-cov --html` para relatório visual (abrir `target/llvm-cov/html/index.html`).
3. Se a cobertura for baixa, identifique linhas não cobertas e sugira testes.
4. Ofereça rodar com `--fail-under-lines 80` para forçar limite no CI [citation:3].

## Integração com nextest

    cargo llvm-cov nextest --html

Isso usa o nextest como runner e gera cobertura [citation:10].

## Guardrails

- Não commitar `target/llvm-cov/`.
- Cobertura não é garantia de qualidade (testes podem ser fracos); combine com mutation testing.
