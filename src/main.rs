use std::env;

mod xinghuo;

fn main() {
    xh_api();
    // fun1();
}

async fn xh_api() {
    let url = xinghuo::functions::GetAuthUrlWs().unwrap();

}

fn fun1() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
}
