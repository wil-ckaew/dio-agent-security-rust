# Skill: Auditoria de Dependências Rust

## Quando usar

Quando o estudante pedir para verificar vulnerabilidades conhecidas nas dependências do projeto Rust.

## Regra de ouro

**A PRIMEIRA resposta deve conter APENAS perguntas de contexto.** Pergunte: ambiente (laboratório/produção), autorização para rodar comandos, e se o projeto pode ser modificado (para sugerir `cargo update`).

## Roteiro (após o contexto)

1. Rode `cargo audit` no diretório do projeto.
2. Se houver vulnerabilidades, formate a saída no **formato fixo** da skill `vulnerability-analysis`.
3. Para cada vulnerabilidade, sugira a correção:
   - Se houver versão corrigida: recomende `cargo update -p <crate>` ou ajuste no `Cargo.toml`.
   - Se não houver correção: recomende documentar a exceção no `deny.toml` com justificativa e data de expiração [citation:2].
4. Ofereça rodar `cargo deny check advisories` como segunda camada.

## Formato fixo (copie os cabeçalhos)

    ## Análise de Vulnerabilidade

    **Contexto:** ...
    **Descrição da falha:** ...
    **Impacto:** ...
    **Probabilidade:** ...
    **Severidade:** ...
    **Técnica MITRE ATT&CK:** ...
    **Recomendação de correção:** ...
    **Validação:** ...
    **Próximo passo sugerido:** ...
