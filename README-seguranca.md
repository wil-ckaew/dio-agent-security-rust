# Especialização em Segurança do DIO Agent

Este projeto estende o DIO Agent com uma especialização em segurança da informação, seguindo o desafio da DIO.

## O que foi criado

- `agent/knowledge/security-basics.md`: fundamentos de vulnerabilidade, ameaça, risco, tríade CID, MITRE ATT&CK, SQL Injection e boas práticas.
- `skills/vulnerability-analysis/`: skill de análise de vulnerabilidade, com formato fixo de saída, regra de ouro e exemplo few-shot.
- `skills/security-testing/`: skill de testes de segurança, com regra de ouro de perguntar o contexto antes de sugerir comandos.
- `agent/persona.md`: trecho de especialização em segurança, com exceção explícita à regra de "seja concreto".
- `AGENTS.md`: tabela de skills, regra global de segurança no topo, nota de prioridade e exemplo few-shot de formato fixo.

## Como testar

1. Abra o Hermes **na pasta do projeto** (`~/dio-agent`).
2. Pergunte: `"Tenho um servidor Ubuntu de laboratório exposto na internet. Como devo proceder?"` — o agente pergunta o contexto antes de listar comandos.
3. Pergunte: `"Ambiente de laboratório, tenho autorização. Analise o agent/app.py como especialista em AppSec."` — o agente identifica SQL Injection, explica a exploração, cita T1190 do MITRE ATT&CK e orienta a correção.
4. Pergunte: `"O SQL Injection foi mitigado no agent/app_corrigido.py? Responda no formato fixo da skill vulnerability-analysis."` — o agente responde no formato fixo, com os 9 campos literais.

## Resultados dos testes

| Teste | Comportamento esperado | Resultado |
|-------|------------------------|-----------|
| Contexto antes de comando | Perguntar antes de listar `ufw`/`fail2ban` | ✅ Perguntou o contexto primeiro |
| Identificação de SQL Injection | Apontar falha, exploração, MITRE | ✅ Identificou SQLi, CWE-89, T1190 |
| Correção com consulta parametrizada | Usar `?` e tupla | ✅ `app_corrigido.py` correto |
| Formato fixo (invocado) | 9 campos literais | ✅ Resposta exata no formato |
| Formato fixo (não invocado) | 9 campos literais | ⚠️ Usa formato próprio, com conteúdo correto |

## Limitação conhecida

O harness Hermes injeta o `AGENTS.md` no system prompt, mas não força o modelo a abrir o `SKILL.md`. Na prática, o modelo:

- **Acerta o conteúdo**: identifica SQLi, cita CWE-89/OWASP A03:2021, cita T1190, explica exploração e correção.
- **Pergunta o contexto** antes de sugerir comando (com a regra global no topo do `AGENTS.md`).
- **Usa o formato fixo quando invocado explicitamente** ("Responda no formato fixo da skill vulnerability-analysis").
- **Nem sempre usa o formato fixo literal sem invocação**, preferindo títulos próprios. Isso é uma limitação do modelo/harness, não da skill — a skill está escrita com o formato fixo, regra de ouro e exemplo few-shot.

## Estrutura

agent/
app.py # vulnerável (SQLi)
app_corrigido.py # corrigido (query parametrizada)
persona.md # persona com especialização em segurança
knowledge/
security-basics.md # fundamentos de segurança
skills/
vulnerability-analysis/
SKILL.md # formato fixo de análise
README.md # explicação humana
security-testing/
SKILL.md # regra de ouro: contexto antes de comando
README.md # explicação humana
AGENTS.md # fonte de verdade do agente
README-seguranca.md # este arquivo
text


## Evoluções possíveis

- Trilha de estudos em cibersegurança (skill nova).
- Análise no formato fixo em 100% dos casos (limitação atual).
- Revisão DevSecOps antes de produção.
- Versão em Rust do `app.py` (a skill é agnóstica de linguagem).

## Referências

- MITRE ATT&CK T1190: https://attack.mitre.org/techniques/T1190/
- OWASP Top 10 2021 A03 (Injection): https://owasp.org/Top10/A03_2021-Injection/
- CWE-89 (SQL Injection): https://cwe.mitre.org/data/definitions/89.html
