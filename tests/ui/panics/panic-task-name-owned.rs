//@ run-fail
//@ error-pattern:thread 'owned name' panicked
//@ error-pattern:test
//@ ignore-emscripten Needs threads.

use std::thread::Builder;

fn main() {
    let r: () = Builder::new()
                    .name("owned name".to_string())
                    .spawn(move || {
                        panic!("test");
                        ()
                    })
                    .unwrap()
                    .join()
                    .unwrap();
    panic!();
}
