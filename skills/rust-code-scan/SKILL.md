# Skill: Varredura Estática de Código Rust

## Quando usar

Quando o estudante pedir para procurar padrões inseguros no código-fonte Rust (SQL Injection, uso de `format!` com input, `unsafe` suspeito).

## Regra de ouro

**A PRIMEIRA resposta deve conter APENAS perguntas de contexto.** Pergunte: ambiente, autorização, e se pode rodar ferramentas externas.

## Roteiro (após o contexto)

1. Rode: `semgrep --config=p/rust --config=p/owasp-top-ten src/` no diretório do projeto.
2. Filtre os resultados para linguagem Rust.
3. Apresente cada achado no **formato fixo** da skill `vulnerability-analysis`.
4. Para o SQL Injection no `format!`, a correção é `params![]` com `?1`, `?2`.
5. Ofereça rodar `semgrep --config=p/secrets .` para procurar segredos hardcoded.

## Observação importante

O Semgrep tem taxa de falsos positivos em Rust (uma pesquisa acadêmica mostrou 96.6% de repositórios fora do escopo de detecção) [citation:5]. **Sempre valide o achado lendo o código** antes de recomendar correção.
