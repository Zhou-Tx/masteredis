use std::env;

mod mainloop;
mod parser;
mod redis;

fn main() -> std::io::Result<()> {
    let host = env::var("REDIS_HOST").expect("Environment variable unset: host");
    let port = env::var("REDIS_PORT")
        .expect("Environment variable unset: port")
        .parse::<u16>()
        .expect("Unresolved value: port");
    let username = env::var("REDIS_USER").unwrap_or(String::new());
    let auth = match env::var("REDIS_PASSWORD") {
        Ok(password) => Some(format!("{username} {password}")),
        _ => None,
    };
    let check_interval = env::var("CHECK_INTERVAL")
        .unwrap_or(String::from("5000"))
        .parse::<u64>()
        .expect("Unresolved value: check_interval");
    let nodes = redis::get_node_list(&host, port, &auth)?;
    println!("Initialized node list:");
    for node in &nodes {
        println!("- {}:{}", node.0, node.1);
    }
    mainloop::mainloop(&nodes, &auth, check_interval)?;
    Ok(())
}
