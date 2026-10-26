# Skill: Correção Assistida de Vulnerabilidades Rust

## Quando usar

Quando o estudante pedir para **corrigir** uma vulnerabilidade já identificada (por cargo audit, cargo deny ou semgrep).

## Regra de ouro (obrigatória)

**NUNCA aplique uma correção sem aprovação explícita do estudante.** O fluxo é sempre:

1. Propor a correção.
2. Mostrar o diff.
3. Pedir aprovação.
4. Só então aplicar e validar.

## Roteiro

### Etapa 1: Identificar o tipo de correção

- **Dependência vulnerável** (cargo audit ou cargo deny advisories):
  - Rode cargo update -p <crate> (ou --precise <versão> se precisar fixar).
  - Mostre o diff do Cargo.lock.
- **Código inseguro** (semgrep):
  - Proponha o patch no arquivo (ex: format! -> params![]).
  - Mostre o diff do .rs.

### Etapa 2: Validar localmente

Antes de commitar, rode:

- cargo build (compila)
- cargo test (testes passam)
- cargo deny check (política continua ok)
- cargo audit (vulnerabilidade sumiu)

Se qualquer um falhar, **reverta** e proponha outra abordagem.

### Etapa 3: Mostrar o diff e pedir aprovação

Use git diff para mostrar o que mudou. Pergunte:

> "A correção proposta é esta. Posso aplicar e commitar? (sim/não)"

Só prossiga com "sim".

### Etapa 4: Commit e push (após aprovação)

    git add <arquivos>
    git commit -m "fix(security): corrige <vulnerabilidade> em <crate/arquivo>"
    git push origin main

## Formato da proposta de correção

    ## Proposta de Correção

    **Vulnerabilidade:** [nome/CVE]
    **Tipo:** [dependência / código]
    **Correção proposta:** [comando ou patch]
    **Diff:**
    [git diff]
    **Validação:**
    - cargo build: [ok/falhou]
    - cargo test: [ok/falhou]
    - cargo deny check: [ok/falhou]
    - cargo audit: [vulnerabilidade removida / ainda presente]

    **Posso aplicar? (sim/não)**

## Guardrails (obrigatórios)

1. **Nunca commite sem aprovação explícita.**
2. **Nunca altere testes** para "fazer passar" — se um teste falhar, reverta e proponha outra correção.
3. **Nunca use --force** em git push nem em cargo update.
4. **Se a correção não for possível** (ex: crate sem versão corrigida), documente a exceção no deny.toml com justificativa e data de expiração.
