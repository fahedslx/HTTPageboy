use server_base::{ ServerBase, request_type::Rt };
use server_base::routes::{ demo_handle_home_get, demo_handle_test_get };


fn main() {
	let mut server = ServerBase::new("127.0.0.1:7878", 10, None);

	server.add_route("/", Rt::GET, demo_handle_home_get);
	server.add_route("/test", Rt::GET, demo_handle_test_get);

	server.serve();
}
