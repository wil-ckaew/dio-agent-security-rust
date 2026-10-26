use dio_agent_security_rust::fazer_login;
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conexao = Connection::open("usuarios.db")?;
    conexao.execute("CREATE TABLE IF NOT EXISTS usuarios (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)", [])?;
    conexao.execute("INSERT OR IGNORE INTO usuarios (id, username, password) VALUES (1, 'admin', 'senha123')", [])?;
    println!("Login normal: {}", fazer_login("admin", "senha123")?);
    println!("SQLi (bypass): {}", fazer_login("admin' --", "qualquer")?);
    Ok(())
}
