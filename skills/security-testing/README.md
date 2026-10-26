# Skill: Testes de Segurança

## O que faz

Orienta o DIO Agent a montar um plano de testes de segurança a partir do contexto, perguntando autorização e ambiente antes de sugerir qualquer comando.

## Por que existe

Testar segurança sem autorização é ilegal e perigoso. Esta skill garante que o agente nunca sugira ferramentas ofensivas sem confirmar que o ambiente é de laboratório e que o estudante tem autorização.

## Quando é usada

Quando o estudante pede para testar a segurança de um cenário ou infraestrutura.

## Como funciona

1. O agente pergunta contexto, autorização e escopo.
2. Monta um plano em ordem: passivo → defensivo → ativo (se autorizado).
3. Usa o formato fixo da skill `vulnerability-analysis` para cada achado.

## Exemplo

"Tenho um servidor Ubuntu de laboratório exposto na internet." O agente pergunta se é proposital, quais serviços rodam e se há autorização, antes de sugerir qualquer verificação.
