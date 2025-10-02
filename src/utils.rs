use std::collections::HashMap;
use sysinfo::Networks;

/**列出系统的网卡

*/
pub fn netinfo() -> Vec<HashMap<&'static str, String>> {
    let mut a: Vec<HashMap<&'static str, String>> = vec![];
    let b = Networks::new_with_refreshed_list();
    let c = b
        .iter()
        .filter(|x| !x.0.to_lowercase().contains("npcap"))
        .collect::<Vec<_>>();
    for (name, data) in &c {
        let ip = match data.ip_networks().iter().find(|&&x| x.addr.is_ipv4()) {
            Some(ip) => format!("{}/{}", ip.addr, ip.prefix),
            None => "接口没有IP".to_string(),
        };
        let mac = data.mac_address().to_string();
        let desc = name.to_string();
        let id = if ip.contains("IP") || mac == "00:00:00:00:00:00" {
            "-1".to_string()
        } else {
            "0".to_string()
        };

        a.push(
            [("id", id), ("ip", ip), ("mac", mac), ("desc", desc)]
                .into_iter()
                .collect(),
        );
    }
    // dbg!(&a);
    a
}
