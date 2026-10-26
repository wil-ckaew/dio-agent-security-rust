# Skill: Verificação de Políticas de Dependência Rust

## Quando usar

Quando o estudante pedir para verificar políticas de segurança nas dependências Rust: vulnerabilidades, licenças, crates proibidas ou fontes não confiáveis.

## Regra de ouro

**A PRIMEIRA resposta deve conter APENAS perguntas de contexto.** Pergunte: ambiente (laboratório/produção), autorização para rodar comandos, e se o projeto pode ser modificado (para sugerir correções).

## Roteiro (após o contexto)

1. Rode `cargo deny check` no diretório do projeto.
2. Interprete a saída por categoria:
   - **advisories**: vulnerabilidades conhecidas (RUSTSEC). Formate no **formato fixo** da skill `vulnerability-analysis`.
   - **licenses**: licenças não permitidas. Recomende ajustar `deny.toml` ou substituir a crate.
   - **bans**: crates proibidas ou versões duplicadas. Recomende remover ou unificar.
   - **sources**: fontes não permitidas. Recomende ajustar `Cargo.toml` ou `deny.toml`.
3. Se houver falhas, sugira a correção:
   - Vulnerabilidade: `cargo update -p <crate>` ou documentar exceção em `ignore` com justificativa.
   - Licença: adicionar ao `allow` (se compatível) ou substituir a crate.
   - Fonte: ajustar `allow-git` no `deny.toml` ou remover a dependência git.
4. Ofereça rodar `cargo deny check --format sarif` para integração com GitHub Code Scanning [citation:1].

## Formato fixo (para cada vulnerabilidade)

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

## Observação

O `cargo-deny` é mais poderoso que o `cargo-audit` sozinho porque cobre quatro categorias: advisories, licenses, bans e sources [citation:1][citation:6].
