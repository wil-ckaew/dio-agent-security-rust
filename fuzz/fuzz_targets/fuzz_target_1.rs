#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::Write;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 { return; }
    let usuario = String::from_utf8_lossy(&data[..data.len()/2]);
    let senha = String::from_utf8_lossy(&data[data.len()/2..]);
    let _ = dio_agent_security_rust::fazer_login(&usuario, &senha);
    let _ = std::io::stdout().flush();
});
