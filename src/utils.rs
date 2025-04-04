//! 主要提供发送wol报文的功能

use std::{collections::HashMap, net::UdpSocket};
use sysinfo::Networks;

/**列出系统的网卡

*/
pub fn netinfo() -> Vec<HashMap<&'static str, String>> {
    let mut a: Vec<HashMap<&'static str, String>> = vec![];
    let b = Networks::new_with_refreshed_list();
    for (name, data) in &b {
        let ip = match data.ip_networks().iter().find(|&&x| x.addr.is_ipv4()) {
            Some(ip) => format!("{}/{}", ip.addr, ip.prefix),
            None => "接口没有IP".to_string(),
        };
        let mac = data.mac_address().to_string();
        let desc = name.clone();
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

/**发送wol报文，使用UDP协议，目的端口是7和9

参数m 是冒号分割的mac地址，不区分大小写，如：11:22:33:44:55:ff

参数broadcast是发送的目的地址，一般是网络的广播地址，比如255.255.255.255
*/
pub fn send_wol_udp(m: &str, broadcast: &str) {
    let mac = mac_to_u8(m);
    let mut magic_packet = Vec::new();
    for _ in 1..=6 {
        magic_packet.push(255);
    }
    for _ in 1..=16 {
        magic_packet = [magic_packet, mac.clone()].concat();
        // magic_packet.append(mac.clone().as_mut());
    }

    let socket = UdpSocket::bind("0.0.0.0:0").expect("绑定接口失败");
    socket.set_broadcast(true).expect("启用广播模式失败");
    socket
        .send_to(&magic_packet, format!("{}:7", broadcast))
        .expect("发送失败");
    socket
        .send_to(&magic_packet, format!("{}:9", broadcast))
        .expect("发送失败");
}

/** 发送ehtertype是0x0842类型的WOL报文

nic是网口名称或网口的MAC地址，MAC地址是冒号分隔的格式

target_mac是将要唤醒的MAC地址
 */
pub fn send_wol_eth(nic: &str, target_mac: &str) {
    use pnet::datalink::Channel::Ethernet;
    use pnet::datalink::{self, NetworkInterface};

    let interface_match = |iface: &NetworkInterface| {
        iface.name == nic || iface.mac.unwrap().to_string() == nic.to_lowercase()
    };

    let interface = datalink::interfaces()
        .into_iter()
        .filter(interface_match)
        .next()
        .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut _rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    //构造并发送WOL报文
    let mac = mac_to_u8(target_mac);
    let mut magic_packet = Vec::new();
    for _ in 1..=6 {
        magic_packet.push(255);
    }
    for _ in 1..=16 {
        magic_packet = [magic_packet, mac.clone()].concat();
    }
    let mut eth_packet = [
        [0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        interface.mac.unwrap().octets(),
    ]
    .concat();
    eth_packet.append(&mut vec![0x08, 0x42]);
    eth_packet.append(&mut magic_packet);
    tx.send_to(&eth_packet, None);
}

/**把mac地址转换为u8列表，不区分大小写

比如 `01:02:03:dd:ee:ff => [1, 2, 3, 221, 238, 255]`
 */
fn mac_to_u8(s: &str) -> Vec<u8> {
    let s1 = s.replace(":", "");
    bytes_str_to_u8(&s1)
}

/** 把字节字符串转为u8列表，不区分大小写

比如 `"ff00ff10" => [255, 0, 255, 16]`
 */
fn bytes_str_to_u8(s: &str) -> Vec<u8> {
    let mut return_val = Vec::new();

    let mut siter = s.chars().enumerate();
    while let Some((i, v)) = siter.next() {
        if i % 2 != 0 {
            continue;
        };
        let h = match v.to_digit(16) {
            Some(x) => x,
            None => 255,
        };

        let l = match siter.next().unwrap().1.to_digit(16) {
            Some(x) => x,
            None => 255,
        };

        let a: u8 = (h * 16 + l).try_into().unwrap();
        return_val.push(a);
    }
    return_val
}

#[cfg(test)]
mod tests {
    use crate::utils::bytes_str_to_u8;
    use crate::utils::mac_to_u8;
    use crate::utils::send_wol_eth;
    #[test]
    fn test_to_u8() {
        let b = bytes_str_to_u8("ff00ff10");
        let a = vec![255, 00, 255, 16];
        assert_eq!(
            a, b,
            "期望是{:?}, hex_str_to_u8(\"aa:bb:cc:dd:ee:ff\")函数实际返回{:?}",
            a, b
        );

        assert_eq!(vec![1, 2, 3, 221, 238, 255], mac_to_u8("01:02:03:dd:ee:ff"));
        assert_eq!(vec![1, 2, 3, 221, 238, 255], mac_to_u8("01:02:03:DD:ee:ff"));

        assert_eq!(
            (),
            send_wol_eth(&"BC:24:11:11:A0:F7".to_string(), "11:22:33:44:55:66")
        );
    }
}
