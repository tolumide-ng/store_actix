use store;

fn main() {
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }

    store::start();
}
