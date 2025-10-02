use clap::{
    ArgAction::{Help, SetTrue},
    Parser,
};
use home::home_dir;
use std::fs::create_dir_all;
use wake_on_lan::{send_wol_eth, send_wol_udp};
use wol::config::WOLConfig;

#[derive(Parser)]
#[command(version, about, disable_help_flag = true, arg_required_else_help(true))]
struct App {
    /// 唤醒主机
    #[arg(short, long, value_name = "HOSTNAME")]
    hostname: Option<String>,

    /// 列出可用的主机
    #[arg(short, long, action=SetTrue)]
    list: Option<bool>,

    /// 帮助
    #[arg(long, action=Help)]
    help: Option<bool>,
}

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
    let wolcfg = WOLConfig::new(config_path.to_str().unwrap());

    let args = App::parse();

    if let Some(x) = args.list {
        if x == true {
            let hosts = wolcfg.get_hosts();
            let mut hostname_max_size = wolcfg.get_hostname_max_size();
            if hostname_max_size < 8 {
                hostname_max_size = 8;
            }
            println!("{:2$}         {} ", "HOSTNAME", "MAC", hostname_max_size);
            for i in hosts {
                let a: Vec<&str> = i.split(",").collect();
                let (hostname, mac) = (a[1], a[0]);
                println!("{:2$}  ({})", hostname, mac, hostname_max_size);
            }
        }
    }

    if let Some(hostname) = args.hostname {
        match wolcfg.get_host_dict().get(&hostname) {
            Some(mac) => {
                let br = wolcfg.get_broadcast();
                send_wol_eth(&wolcfg.get_interface(), &mac);
                send_wol_udp(&mac, &br);
                println!(
                    "已向{}发送唤醒包，mac地址：{}，广播地址：{}",
                    &hostname, &mac, &br
                )
            }
            _ => println!("没有这个主机"),
        }
    }
}
