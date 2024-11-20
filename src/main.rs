use clap::{ArgAction::{SetTrue, Help}, Parser};
use wol::config::WOLConfig;
use wol::utils::send_wol_packet;

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
    let wolcfg = WOLConfig::new("config.ini");

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
            Some(x) => {
                let br = wolcfg.get_broadcast();
                send_wol_packet(&x, &br);
                println!(
                    "已向{}发送唤醒包，mac地址：{}，广播地址：{}",
                    &hostname, &x, &br
                )
            }
            _ => println!("没有这个主机"),
        }
    }

}
