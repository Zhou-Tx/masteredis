use std::env;

mod mainloop;
mod parser;
mod redis;

fn main() -> std::io::Result<()> {
    let listen_port = env::var("LISTEN_PORT")
        .unwrap_or(String::from("6379"))
        .parse::<u16>()
        .expect("Unresolved value: LISTEN_PORT");
    let redis_host = env::var("REDIS_HOST").expect("Environment variable unset: REDIS_HOST");
    let redis_port = env::var("REDIS_PORT")
        .expect("Environment variable unset: REDIS_PORT")
        .parse::<u16>()
        .expect("Unresolved value: REDIS_PORT");
    let username = env::var("REDIS_USER").unwrap_or(String::new());
    let auth = match env::var("REDIS_PASSWORD") {
        Ok(password) => Some(format!("{username} {password}")),
        _ => None,
    };
    let check_interval = env::var("CHECK_INTERVAL")
        .unwrap_or(String::from("5000"))
        .parse::<u64>()
        .expect("Unresolved value: CHECK_INTERVAL");
    let nodes = redis::get_node_list(&redis_host, redis_port, &auth)?;
    println!("Initialized node list:");
    for node in &nodes {
        println!("- {}:{}", node.0, node.1);
    }
    mainloop::mainloop(listen_port, &nodes, &auth, check_interval)?;
    Ok(())
}
