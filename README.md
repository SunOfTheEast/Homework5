# Homework5
使用方法： cargo run --bin server（启动服务端） cargo run --bin client \<command\>（启动客户端并执行对应指令）
命令前添加选项 env RUST_LOG = trace 可以打印调试信息
服务端数据利用一个简单的HashMap来进行管理
支持的指令类型：
set \<key\> \<value\> （对应键值设置值）

get \<key\> （获得键值对应的值）

del \<key\> （删除对应键值所对应的值）

ping（尝试能否与服务端连接）

subscribe \<key\> 订阅指定channel，等待下一个发送到channel的消息

publish \<key\> \<value\> 向指定的channel发送信息

filter：屏蔽了一些常见的脏话，有小彩蛋
