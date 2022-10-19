use gloo_worker::Registrable;
use mipsy_web::agent::MipsyWebWorker;

fn main() {
    //console_error_panic_hook::set_once();

    //wasm_logger::init(wasm_logger::Config::default());
    MipsyWebWorker::registrar().register();
}
