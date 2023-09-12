use lazy_static::lazy_static;
use pilota::lazy_static;
use std::net::SocketAddr;
use volo_gen::volo::example::GetItemRequest;
lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:33333".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();
    let mut req = GetItemRequest {op: " ".into(), key: " ".into(), val: " ".into()};
    let opcode = args.remove(1).clone().to_lowercase().to_string();
    match opcode.as_str() {
        "set" => {
            req = GetItemRequest {
                op: "set".into(),
                key: args.remove(1).clone().into(),
                val: args.remove(1).clone().into(),

            };
            println!("You set {} to {}", req.clone().key, req.clone().val);
        }
        "get" => {
            req = GetItemRequest {
                op: "get".into(),
                key: args.remove(1).clone().into(),
                val: " ".into(),
            };
        }
        "del" => {
            req = GetItemRequest {
                op: "del".into(),
                key: args.remove(1).clone().into(),
                val: " ".into(),
            };
        }
        "ping" => {
            req = GetItemRequest {
                op: "ping".into(),
                key: " ".into(),
                val: " ".into(),
            };
            println!("requeset ping!");
        }
        _ => {
            println!("ILLEGAL!");
        }
    }
    println!("request send!");
    let resp = CLIENT.get_item(req).await;
    println!("responsed!");
    match resp {
        Ok(info)=>{
            if info.op=="set".to_string(){
                if info.status{
                    println!("SET SUCCESS");
                } else {
                    println!("ALREADY EXISTED");
                }
            }
            else if info.op=="get".to_string() {
                if info.status{
                    println!("GET SUCCESS, {}", info.val);
                } else {
                    println!("NOT FOUUND");
                }
            }else if info.op=="del".to_string() {
                if info.status{
                    println!("DEL SUCCESS");
                } else {
                    println!("NOT FOUUND");
                }
            }else if info.op=="ping".to_string() {
                if info.status{
                    println!("pong");
                } else {
                    println!("FAILED");
                }
            }

        },
        Err(e) => tracing::error!("{:?}", e),
    }
}
