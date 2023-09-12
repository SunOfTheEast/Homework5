#![feature(impl_trait_in_assoc_type)]

use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::anyhow;

pub struct S {
	kv: Mutex<HashMap<String, String>>
}

impl S {
	pub fn new() -> S {
		S {kv: Mutex::new(HashMap::new())}
	}
}
#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get_item(&self, _req: volo_gen::volo::example::GetItemRequest) -> core::result::Result<volo_gen::volo::example::GetItemResponse, volo_thrift::AnyhowError> {
		let mut resp = volo_gen::volo::example::GetItemResponse{op: " ".into(), key: " ".into(), val: " ".into(), status: false};
		println!("收到！");
		match _req.op.as_str() {
			"set" => {
				resp.op = "set".to_string().into();
				let k = _req.key.to_string();
				let v = _req.val.to_string();
				let mut flag = 0;
				if self.kv.lock().unwrap().get(&k) == None {
					flag = 1;
				}
				match flag {
					1 => {
						self.kv.lock().unwrap().insert(k, v);
						resp.status = true;
					}
					0 => {
						resp.status = false;
					}
					_ => {
						resp.status = false;
					}
				}
			}
			"get" => {
				resp.op = "get".to_string().into();
				let k = _req.key.to_string();
				match self.kv.lock().unwrap().get(&k)  {
					None => {
						resp.status = false;
					}
					Some(t) => {
						resp.val = t.clone().into();
						resp.status = true;
					}
				}
			}
			"del" => {
				resp.op = "del".to_string().into();
				let k = _req.key.to_string();
				match self.kv.lock().unwrap().remove(&k) {
					Some(t) => {
						resp.status = true;
					}
					None => {
						resp.status = false;
					}
				}
			}
			"ping" => {
				resp.op = "ping".to_string().into();
				resp.status = true;
			}
			_ => {
				panic!("INVALID!");
			}
		}
		println!("处理完毕，送回");
		Ok(resp)
		//Ok(Default::default())
				}
}
pub struct FilterLayer;
impl<S> volo::Layer<S> for FilterLayer {
	type Service = FilterService<S>;

	fn layer(self, inner: S) -> Self::Service {
		FilterService(inner)
	}
}
#[derive(Clone)]
pub struct FilterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
	where
		Req: std::fmt::Debug + Send + 'static,
		S: Send + 'static + volo::Service<Cx, Req> + Sync,
		Cx: Send + 'static,
		anyhow::Error: Into<S::Error>,
{
	async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let info = format!("{req:?}");
		let mut dirty = false;
		if info.contains("原神") || info.contains("傻逼") || info.contains("操你妈") {
			dirty = true;
		}
		match dirty {
			true => {
				Err(anyhow!("你怎么骂人呢？给我刷了牙再来").into())
			}
			false => {
				let resp =self.0.call(cx, req).await;
				resp
			}
		}
	}
}