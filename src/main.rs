use roto::Runtime;

fn main() {
    #[cfg(feature = "logger")]
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .init();

    let mut rt = Runtime::new();
    rt.add_io_functions();
    rt.cli();
}
