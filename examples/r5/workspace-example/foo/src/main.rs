use utils::bar::add;

fn main() {
    println!("{:?}", serde_json::to_string(&add(1, 2)).unwrap());
}
