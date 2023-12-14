use crate::parser::{
    get_master_from_slave, get_role_from_info_replication, get_slaves_from_master,
};
use std::io::{BufRead, BufReader, Result, Write};
use std::net::TcpStream;

pub fn get_node_list(host: &str, port: u16, auth: &Option<String>) -> Result<Vec<(String, u16)>> {
    let entry_info_replication = get_info_replication(host, port, auth)?;
    let entry_role = get_role_from_info_replication(&entry_info_replication);
    let (master_host, master_port) = if entry_role == "master" {
        (String::from(host), port)
    } else {
        get_master_from_slave(&entry_info_replication)
    };
    let mut node_list = if entry_role == "master" {
        get_slaves_from_master(&entry_info_replication)
    } else {
        let info_replication = get_info_replication(&master_host, master_port, auth)?;
        get_slaves_from_master(&info_replication)
    };
    node_list.push((master_host, master_port));
    Ok(node_list)
}

pub fn get_info_replication(host: &str, port: u16, auth: &Option<String>) -> Result<String> {
    let mut stream = TcpStream::connect(&format!("{host}:{port}"))?;
    match auth {
        Some(auth) => {
            stream.write(format!("auth {auth}\n info replication\n").as_bytes())?
        }
        None => stream.write(b"info replication\n")?,
    };
    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    loop {
        let size = reader.read_until(b'\n', &mut buffer)?;
        if size <= 2 {
            break;
        }
    }
    Ok(String::from_utf8(buffer).unwrap())
}
