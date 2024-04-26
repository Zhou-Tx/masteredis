use crate::{parser, redis};
use std::collections::HashMap;
use std::io::Result;
use std::process::{Child, Command};
use std::{thread, time::Duration};

pub fn mainloop(
    listen_port: u16,
    node_list: &Vec<(String, u16)>,
    auth: &Option<String>,
    check_interval: u64,
) -> Result<()> {
    let mut master = get_master(node_list, auth)?;
    println!("init master = {}:{}", master.0, master.1);
    let mut process = forward(listen_port, &master.0, master.1)?;
    loop {
        thread::sleep(Duration::from_millis(check_interval));
        let new_master = get_master(node_list, auth)?;
        if master != new_master {
            println!("new  master = {}:{}", master.0, master.1);
            master = new_master;
            process.kill().unwrap();
            process = forward(listen_port, &master.0, master.1)?;
        }
    }
}

fn get_master(node_list: &Vec<(String, u16)>, auth: &Option<String>) -> Result<(String, u16)> {
    let mut master_list = HashMap::<(String, u16), usize>::new();
    for (host, port) in node_list {
        let info_replication = redis::get_info_replication(&host, *port, auth)?;
        let role = parser::get_role_from_info_replication(&info_replication);
        if role == "master" {
            let key = (host.to_string(), *port);
            let count = master_list.get(&key).unwrap_or(&0);
            master_list.insert(key, *count);
        } else {
            let key = parser::get_master_from_slave(&info_replication);
            let count = master_list.get(&key).unwrap_or(&0);
            master_list.insert(key, *count);
        }
    }
    let mut scores: Vec<(&(String, u16), &usize)> = master_list.iter().collect();
    scores.sort_by(|a, b| a.1.cmp(b.1));
    let ((host, port), _) = scores.first().unwrap();
    Ok((host.to_string(), *port))
}

fn forward(listen_port: u16, redis_host: &str, redis_port: u16) -> Result<Child> {
    Command::new("socat")
        .arg(format!("TCP4-LISTEN:{listen_port},reuseaddr,fork"))
        .arg(format!("TCP4:{redis_host}:{redis_port}"))
        .spawn()
}
