#[macro_use] extern crate nickel;

use nickel::Nickel;
use nickel::JsonBody;
use std::io::Read;
extern crate url;
use url::*;
use std::collections::HashMap;

fn main() {

	let mut server = Nickel::new();

	server.utilize(router! {

		// ========== 全ての GET をひっかける ==========
		get "**" => |_req, _res| {
			println!("### REQUEST: [/] ###");
			let response = format!("{:?}: [/] Hello world!\n", std::thread::current().id());
			// let value1 = _req.param("key1");
			// println!("[key1]: {:?}", value1);
			response
		}

		// ========== LOGIN ==========
		post "/login" => |_req, _res| {
			let mut body = String::new();
			_req.origin.read_to_string(&mut body).unwrap();
			let mut data = HashMap::new();
			for (key, value) in form_urlencoded::parse(body.as_bytes()) {
				data.insert(key, value);
			}
			println!("### REQUEST: [/login] ###");
			println!("POST: {:?}", data);
			let response = "<!DOCTYPE>
<html>
	<body>
		hello
	</body>
</html>
";
			response
		}

		// 有効なルーティング。完全一致のようだ。
		post "/test1" => |_req, _res| {
			println!("### REQUEST: [/test1] ###");
			let response = format!("{:?}: [/test1] -> accepted post data.\n", std::thread::current().id());
			let payload = _req.json_as::<String>();
			if payload.is_err() {
				println!("{:?}", payload.err().unwrap());
			}
			else {
				println!("{:?}", payload.unwrap());
			}
			response
		}
	});

	let result = server.listen("127.0.0.1:6767");
	if result.is_err() {
		let error = result.err().unwrap();
		println!("{:?}", error);
	}
}
