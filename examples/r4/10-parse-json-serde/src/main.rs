use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;

// Derive serde enum
#[derive(Serialize, Deserialize, Debug)]
enum AnimalType {
    Cat,
    Duck,
}

// Derive serde struct
#[derive(Serialize, Deserialize, Debug)]
struct AnimalData {
    id: String,
    r#type: AnimalType,
    img_url: Url, // Parsed as Url.
}

fn main() {
    // Create JSON raw(r#) &str.
    let foo_str = r#"[
        {"id": "foo", "type": "Cat", "img_url": "http://localhost:3000/assets/kat.png"},
        {"id": "bar", "type": "Duck", "img_url": "http://localhost:3000/assets/duck.png"}
    ]"#;

    // Parse JSON from str to Value.
    let foo_json: Vec<Value> = serde_json::from_str::<Vec<Value>>(foo_str).unwrap();
    println!("1️⃣ foo_json = {foo_json:#?}");

    // Filter JSON elements by matched value.
    let filtered_foo_json = foo_json
        .iter()
        .filter(|v| v["id"] == "foo")
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    println!("2️⃣ filter_and_map_foo_json = {filtered_foo_json:#?}");

    // Filter JSON elements with filter_map by matched value.
    let filtered_foo_json = foo_json
        .iter()
        .filter_map(|v| {
            if v["id"] == "foo" {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("3️⃣ filter_map_foo_json = {filtered_foo_json:#?}");

    // Filter JSON element's value with filter_map by matched value.
    let filtered_foo_json = foo_json
        .iter()
        .filter(|v| v["id"] == "foo")
        .filter_map(|v| v["id"].as_str())
        .collect::<Vec<_>>();

    println!("4️⃣ filtered_foo_value_json = {filtered_foo_json:#?}");

    // Parse JSON from &str to struct.
    let foo_struct = serde_json::from_str::<Vec<AnimalData>>(foo_str).unwrap();

    println!("5️⃣ foo_struct = {foo_struct:#?}");

    // Create JSON value.
    let bar_value = json!({
        "id": "bar",
        "type": "Duck",
        "img_url": "http://localhost:3000/assets/duck.png"
    });

    // Parse JSON from value
    let bar_struct = serde_json::from_value::<AnimalData>(bar_value).unwrap();
    println!("6️⃣ bar_struct = {bar_struct:#?}");
}
