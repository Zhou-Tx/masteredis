use regex::Regex;

pub fn get_role_from_info_replication(info_replication: &str) -> String {
    let role_cap = Regex::new("role:(.+?)\\s")
        .unwrap()
        .captures(info_replication)
        .unwrap();
    String::from(role_cap.get(1).unwrap().as_str())
}

pub fn get_master_from_slave(info_replication: &str) -> (String, u16) {
    let master_host = Regex::new("master_host:(.+?)\\s")
        .unwrap()
        .captures(info_replication)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let master_port = Regex::new("master_port:(\\d+?)\\s")
        .unwrap()
        .captures(info_replication)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    (String::from(master_host), master_port.parse().unwrap())
}

pub fn get_slaves_from_master(info_replication: &str) -> Vec<(String, u16)> {
    Regex::new("slave\\d+:ip=(.+?),port=(\\d+?),state")
        .unwrap()
        .captures_iter(info_replication)
        .map(|cap| {
            let ip = cap.get(1).unwrap().as_str();
            let port = cap.get(2).unwrap().as_str();
            (String::from(ip), port.parse().unwrap())
        })
        .collect()
}
