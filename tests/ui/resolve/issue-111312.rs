// edition: 2021

trait Has {
    fn has() {}
}

trait HasNot {}

fn main() {
    HasNot::has(); //~ ERROR
}
