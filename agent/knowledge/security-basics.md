# Fundamentos de Segurança da Informação

## Vulnerabilidade

Uma vulnerabilidade é uma falha em um sistema, aplicação ou processo que pode permitir comportamentos indesejados.

## Ameaça

Uma ameaça representa algo que pode explorar uma vulnerabilidade.

## Risco

Risco é a combinação entre:

- probabilidade;
- impacto.

## Tríade CID (Confidencialidade, Integridade e Disponibilidade)

### Confidencialidade

Garantir que apenas pessoas autorizadas tenham acesso à informação.

### Integridade

Garantir que informações não sejam alteradas indevidamente.

### Disponibilidade

Garantir que sistemas continuem funcionando e acessíveis quando necessário.

## Boas práticas

- manter sistemas atualizados;
- utilizar autenticação forte;
- aplicar o princípio do menor privilégio;
- validar e sanitizar toda entrada do usuário;
- usar consultas parametrizadas em banco de dados;
- monitorar eventos e logs;
- realizar backups;
- corrigir vulnerabilidades conhecidas.

## SQL Injection (exemplo didático)

SQL Injection (SQLi) ocorre quando a aplicação monta uma consulta SQL concatenando texto digitado pelo usuário. Um atacante pode inserir trechos como `' OR '1'='1` para burlar autenticação ou vazar dados.

**Correção canônica:** usar consultas parametrizadas (prepared statements), nunca concatenar entrada do usuário na query.

## MITRE ATT&CK

O MITRE ATT&CK é um catálogo público de táticas e técnicas usadas por atacantes. Ele ajuda a nomear e comparar comportamentos observados.

- **T1190 — Exploit Public-Facing Application:** exploração de aplicações expostas à internet (ex.: SQLi em um login público).

## DevSecOps (visão geral)

DevSecOps é a prática de inserir segurança em cada etapa do desenvolvimento, não só no final. Uma revisão de segurança deve acontecer antes da produção, analisando cada alteração de código.
