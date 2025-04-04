use configparser::ini::{Ini, WriteOptions};
use std::collections::HashMap;

#[derive(Debug)]
pub struct WOLConfig {
    path: String,
    hosts: Vec<String>,
    hosts_dict: HashMap<String, String>,
    hostname_max_size: usize,
    broadcast: String,
    interface: String,
}

impl WOLConfig {
    pub fn new(config_path: &str) -> Self {
        WOLConfig {
            path: config_path.to_string(),
            hosts: vec![], //"11:22:33:44:55:66,pc"
            hosts_dict: HashMap::new(),
            hostname_max_size: 0,
            broadcast: "255.255.255.255".to_string(),
            interface: "".to_string(),
        }
        .loadconfig()
    }

    /** 加载配置 */
    fn loadconfig(mut self) -> Self {
        let mut cfg = Ini::new();
        if let Err(_) = cfg.load(&self.path) {
            cfg.set("General", "broadcast", Some("255.255.255.255".to_string()));
            cfg.set("General", "interface", Some("ens18".to_string()));
            cfg.set("PC1", "ip", Some("192.168.1.2".to_string()));
            cfg.set("PC1", "mac", Some("11:22:33:44:55:66".to_string()));
            cfg.set("PC2", "ip", Some("192.168.1.3".to_string()));
            cfg.set("PC2", "mac", Some("aa:bb:cc:dd:ee:ff".to_string()));
            cfg.pretty_write(&self.path, &WriteOptions::new_with_params(true, 1, 1))
                .expect("保存配置文件失败");
        }

        for i in cfg.get_map_ref() {
            if i.0 == "general" {
                self.broadcast = i.1.get("broadcast").unwrap().as_ref().expect("配置文件中没有broadcast").clone();
                self.interface = i.1.get("interface").unwrap().as_ref().expect("配置文件中没有interface").clone();
            } else {
                let hostname_len = i.0.len();
                if self.hostname_max_size < hostname_len {
                    self.hostname_max_size = hostname_len;
                }
                let hostname = i.0.clone();
                let mac = i.1.get("mac").unwrap().as_ref().unwrap().clone();
                self.hosts_dict.insert(hostname.clone(), mac.clone());
                self.hosts.push(format!("{},{}", mac, hostname));
            }
        }
        self
    }

    pub fn get_hosts(&self) -> &Vec<String> {
        &self.hosts
    }

    /**主机名的最大长度 */
    pub fn get_hostname_max_size(&self) -> usize {
        self.hostname_max_size
    }

    pub fn get_host_dict(&self) -> HashMap<String, String> {
        self.hosts_dict.clone()
    }

    pub fn get_broadcast(&self) -> String {
        self.broadcast.clone()
    }

    pub fn get_interface(&self) -> String {
        self.interface.clone()
    }
}
