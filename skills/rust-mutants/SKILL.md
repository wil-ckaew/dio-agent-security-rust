# Skill: Mutation Testing com cargo-mutants

## Quando usar

Quando o estudante pedir para avaliar a qualidade dos testes (se eles realmente detectam falhas).

## Regra de ouro (obrigatória)

**Mutation testing é caro.** Sempre:
1. Pergunte o contexto (ambiente, autorização, tempo disponível).
2. Limite o escopo com `--file` ou `--package` (nunca rode no projeto inteiro de primeira).
3. Avise que pode demorar (minutos a horas, dependendo do escopo).

## Roteiro

1. Rode `cargo mutants --file src/lib.rs` (escopo no arquivo da lib).
2. Interprete a saída:
   - `CAUGHT`: o teste detectou a mutação (bom).
   - `MISSED`: o teste NÃO detectou (teste fraco).
   - `TIMEOUT`: o mutante travou (pode ser loop infinito).
   - `UNVIABLE`: o mutante não compila (ignorar).
3. Se houver `MISSED`, sugira adicionar testes para cobrir aquele comportamento.
4. Formate os achados no formato fixo da skill vulnerability-analysis.

## Formato fixo

    ## Análise de Vulnerabilidade

    **Contexto:** mutation testing em src/lib.rs.
    **Descrição da falha:** [N] mutantes sobreviveram (MISSED), indicando testes fracos.
    **Impacto:** ...
    **Probabilidade:** ...
    **Severidade:** ...
    **Técnica MITRE ATT&CK:** N/A (qualidade de testes).
    **Recomendação de correção:** adicionar testes para os comportamentos não cobertos.
    **Validação:** rodar cargo mutants novamente após adicionar testes.
    **Próximo passo sugerido:** ...

## Guardrails

- Nunca commite `mutants.out/` no Git.
- Se demorar muito, interrompa e reduza o escopo.
