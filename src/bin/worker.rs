use mipsy_web::agent::Worker;
use yew_agent::Threaded;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Worker::register();
}
