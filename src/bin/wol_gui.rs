#![windows_subsystem = "windows"]

use enums::{Align, Color};
use fltk::{prelude::*, *};
use home::home_dir;
use std::fs::create_dir_all;
use wol::config::WOLConfig;
use wol::utils::{netinfo, send_wol_eth, send_wol_udp};

static WIDTH: i32 = 500;
static HEIGHT: i32 = 400;

fn main() {
    let config_path = home_dir().unwrap().join(".config/wol_rust/config.ini");
    if !config_path.exists() {
        match create_dir_all(config_path.parent().unwrap()) {
            Ok(_) => {
                println!("OK")
            }
            Err(_) => println!("Err"),
        }
    }
    let mut wolcfg = WOLConfig::new(config_path.to_str().unwrap());

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
        let if_name_text = frame::Frame::default().with_label("网口:");
        let nic = netinfo();
        let mut nic_choice = menu::Choice::default();
        for i in &nic {
            nic_choice.add_choice(&i["desc"]);
        }
        let broadcast_text = frame::Frame::default().with_label("广播地址:");
        let mut broadcast_input = input::Input::default();
        broadcast_input.set_value(wolcfg.get_broadcast().as_str());
        let p = frame::Frame::default();
        let mut selected_host = output::Output::default();
        let mut btn = button::Button::default()
            .with_label("唤醒")
            .with_size(50, 25);

        op_col.fixed(&if_name_text, 25);
        op_col.fixed(&nic_choice, 25);
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
            let active_nic = &nic[nic_choice.value() as usize]["mac"];
            if let Some(x) = host_list.selected_text() {
                let host: Vec<&str> = x.split(",").collect();
                send_wol_udp(&host[0].to_string(), &broadcast_input.value());
                send_wol_eth(&active_nic, &host[0].to_string());
            }
        });
    }

    parent.fixed(&w, 20);
    parent.add_resizable(&middle_row);
}
