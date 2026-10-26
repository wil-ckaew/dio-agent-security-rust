# Skill: Testes Rápidos com cargo-nextest

## Quando usar

Quando o estudante pedir para rodar testes de forma mais rápida ou com melhor isolamento.

## Regra de ouro

**A PRIMEIRA resposta deve conter APENAS perguntas de contexto.** Pergunte: ambiente, autorização, e se o projeto pode ser modificado.

## Roteiro

1. Rode `cargo nextest run` no diretório do projeto.
2. Interprete a saída: PASS, FAIL, SKIP.
3. Se houver falhas, formate no formato fixo da skill vulnerability-analysis.
4. Compare a velocidade com `cargo test` (opcional, mas didático).
5. Ofereça configurar `.config/nextest.toml` para retries e timeouts [citation:1].

## Vantagens sobre cargo test

- Processo por teste (isolamento real).
- Até 3x mais rápido em paralelo [citation:7].
- Retries para testes flaky.
- Saída mais limpa.

## Guardrails

- Não commitar `target/nextest/`.
- Doctests continuam no `cargo test --doc` (nextest não suporta).
