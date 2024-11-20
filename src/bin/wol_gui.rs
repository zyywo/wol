
use enums::{Align, Color};
use fltk::{prelude::*, *};
use std::net::UdpSocket;
use wol::config::WOLConfig;

static WIDTH: i32 = 500;
static HEIGHT: i32 = 400;

fn main() {
    let mut wolcfg = WOLConfig::new("config.ini");
    let app = app::App::default().load_system_fonts();
    let mut win = window::Window::default()
        .with_label(format!("Wake On LAN - rust v{}", env!("CARGO_PKG_VERSION")).as_str())
        .with_size(WIDTH, HEIGHT);
    let mut col = group::Flex::default_fill().column();
    main_panel(&mut col, &mut wolcfg);
    col.end();
    win.resizable(&col);
    win.end();
    win.show();
    win.size_range(WIDTH, HEIGHT, 0, 0);
    app.run().unwrap();
}

fn main_panel(parent: &mut group::Flex, wolcfg: &mut WOLConfig) {
    let w = frame::Frame::default()
        .with_label("机器列表")
        .with_align(Align::Left | Align::Inside)
        .with_size(150, 20);

    let mut middle_row = group::Flex::default().row();
    {
        let mut host_list = browser::HoldBrowser::default();
        host_list.set_column_widths(&[150, 20]);
        host_list.set_column_char(',');
        host_list.set_selection_color(Color::Yellow);

        for i in wolcfg.get_hosts() {
            host_list.add(&i);
        }

        let mut op_col = group::Flex::default().column();
        let broadcast_text = frame::Frame::default().with_label("广播地址:");
        let mut broadcast_input = input::Input::default();
        broadcast_input.set_value("255.255.255.255");
        let p = frame::Frame::default();
        let mut selected_host = output::Output::default();
        let mut btn = button::Button::default()
            .with_label("唤醒")
            .with_size(50, 25);

        op_col.fixed(&broadcast_text, 25);
        op_col.fixed(&broadcast_input, 25);
        op_col.fixed(&p, 50);
        op_col.fixed(&selected_host, 25);
        op_col.fixed(&btn, 25);
        op_col.end();

        middle_row.add_resizable(&host_list);
        middle_row.add_resizable(&op_col);
        middle_row.end();

        host_list.set_callback(move |host_list: &mut browser::HoldBrowser| {
            if let Some(x) = host_list.selected_text() {
                let host: Vec<&str> = x.split(',').collect();
                selected_host.set_value(host[1]);
            }
        });

        btn.set_callback(move |_btn| {
            let broadcast = broadcast_input.value();
            if let Some(x) = host_list.selected_text() {
                let host: Vec<&str> = x.split(",").collect();
                send_wol_packet(&host[0].to_string(), &broadcast);
            }
        });
    }

    parent.fixed(&w, 20);
    parent.add_resizable(&middle_row);
}

fn send_wol_packet(m: &String, broadcast: &String) {
    let mac = mac_str_to_u8(m);
    let mut packet = Vec::new();
    for _ in 1..=6 {
        packet.push(255);
    }
    for _ in 1..=16 {
        packet = [packet.clone(), mac.to_vec()].concat();
    }

    let socket = UdpSocket::bind("0.0.0.0:0").expect("绑定接口失败");
    socket.set_broadcast(true).expect("启用广播模式失败");
    socket
        .send_to(&packet, format!("{}:7", broadcast))
        .expect("发送失败");
    socket
        .send_to(&packet, format!("{}:9", broadcast))
        .expect("发送失败");
}

fn mac_str_to_u8(s: &str) -> [u8; 6] {
    let s1 = s.replace(":", "");
    let val: [u8; 6] = hex_str_to_u8(s1.as_str()).try_into().unwrap();
    val
}

fn hex_str_to_u8(s: &str) -> Vec<u8> {
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
