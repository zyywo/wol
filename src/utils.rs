//! 主要提供发送wol报文的功能

use std::net::UdpSocket;

/**发送wol报文

参数m 是冒号分割的mac地址，不区分大小写，如：11:22:33:44:55:ff

参数broadcast是发送的目的地址，一般是网络的广播地址，比如255.255.255.255
*/
pub fn send_wol_packet(m: &String, broadcast: &String) {
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

/**把mac地址转换为u8列表

比如 `01:02:03:dd:ee:ff => [1, 2, 3, 221, 238, 255]`
 */
fn mac_to_u8(s: &str) -> Vec<u8> {
    let s1 = s.replace(":", "");
    let val: Vec<u8> = bytes_str_to_u8(s1.as_str()).try_into().unwrap();
    val
}

/** 把字节字符串转为u8列表

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
    }
}
