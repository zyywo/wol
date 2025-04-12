# wol_rust

局域网网络唤醒工具，支持linux和windows，支持命令行和窗口界面。

实现了三种方式发送唤醒包:

1. 以太网类型为0x0842的数据帧

2. 目的端口为UDP端口7的数据包

3. 目的端口为UDP端口9的数据包

三种方式的唤醒包构造都相同。

需要唤醒的电脑预先在配置文件里设置，配置文件路径（如果配置文件不存在运行程序时会自动生成）：

**Windows**: `%HOMEPATH%\.config\wol_rust\config.ini`

**Linux**: `$HOME/.config/wol_rust/config.ini`

> linux平台：
因为有发送以太网报文、UDP发送目的地址为广播地址，所以需要root权限才可以运行，
如果不想使用root用户则可以通过setcap命令给可执行文件设置cap_net_raw权限即可。

> windows平台：
可执行文件和Packet.lib需要在同一目录
