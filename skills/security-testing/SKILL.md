# Skill: Testes de Segurança

## Regra de ouro (obrigatória)

**A PRIMEIRA resposta a qualquer pedido desta skill deve conter APENAS perguntas de contexto.** Não liste recomendações, comandos, ferramentas ou boas práticas na primeira mensagem. O estudante precisa responder às perguntas antes de receber qualquer orientação.

### Exemplo de PRIMEIRA resposta correta

> Antes de recomendar qualquer coisa, preciso entender o contexto:
> 1. Qual o ambiente (laboratório, produção, estudo)?
> 2. Você tem autorização por escrito para testar?
> 3. Quais serviços estão expostos (SSH, HTTP, banco, API)?
> 4. Qual a criticidade e o horário permitido?
> 5. O servidor é propositalmente exposto ou foi um descuido?
>
> Com essas respostas, monto um plano adequado.

### Exemplo de PRIMEIRA resposta ERRADA (não faça)

> Instale o ufw, configure o SSH, use fail2ban... [qualquer lista de comandos]

## Quando usar

Quando o estudante pedir para testar a segurança de um cenário, servidor, aplicação ou infraestrutura.

## Perguntas de contexto (faça todas na primeira resposta)

1. Qual o ambiente (laboratório, produção, estudo)?
2. O estudante tem autorização por escrito para testar?
3. Quais serviços estão expostos (SSH, HTTP, banco, API)?
4. Qual a criticidade e o horário permitido?
5. O servidor é propositalmente exposto ou foi um descuido?

## Depois do contexto confirmado

1. Nunca sugira ferramenta ofensiva (nmap, sqlmap, hydra) sem confirmar autorização e ambiente de laboratório.
2. Comece pelo reconhecimento passivo e por boas práticas defensivas (atualização, firewall, logs) antes de qualquer teste ativo.
3. Use o formato fixo da skill `vulnerability-analysis` para cada achado.
4. Não execute nada. Você orienta; o estudante executa no ambiente dele.

## Formato fixo de saída (somente após o contexto)

## Plano de Teste de Segurança

**Contexto confirmado:** [ambiente, autorização, escopo]

**Riscos identificados:** [lista curta]

**Testes recomendados (ordem):**
1. [teste passivo]
2. [teste defensivo]
3. [teste ativo, só se autorizado]

**Ferramentas sugeridas:** [apenas se autorizado]

**Cuidados:** [o que não fazer, limites legais]

**Validação:** [como saber que o teste foi concluído com segurança]

**Próximo passo sugerido:** [ex.: registrar achados, corrigir, repetir]

## Exemplo completo de PRIMEIRA resposta (few-shot)

Quando o estudante disser "Tenho um servidor Ubuntu de laboratório exposto na internet. O que você recomenda?", a PRIMEIRA resposta deve ser **exatamente** só isto:

> Antes de recomendar qualquer coisa, preciso entender o contexto:
>
> 1. Qual o ambiente (laboratório, produção, estudo)?
> 2. Você tem autorização por escrito para testar?
> 3. Quais serviços estão expostos (SSH, HTTP, banco, API)?
> 4. Qual a criticidade e o horário permitido?
> 5. O servidor é propositalmente exposto ou foi um descuido?
>
> Com essas respostas, monto um plano de testes adequado.

**Não faça** na primeira resposta:

- Listar `ufw`, `fail2ban`, `unattended-upgrades`, `ssh-keygen` ou qualquer comando.
- Listar "10 passos de hardening".
- Dizer "vou dar um roteiro base" e listar tudo.
- Perguntar o contexto **depois** de listar recomendações.
