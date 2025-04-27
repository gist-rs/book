## IndexMap

> 🤔 refer to [IndexMap](https://docs.rs/indexmap/latest/indexmap/)

### Preserving the Order of Yummy Emoji Goodness! 😋

![](/assets/kat.png) <span class="speech-bubble">Just like before, the order in which these tasty emoji treats arrive matters! Maybe we want to serve the 🍕 pizza first, followed by the 🍔 burgers. Let's see how `IndexMap` helps us keep track of the order of these delightful dishes!</span>

### Without `IndexMap`

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut emoji_buffet = HashMap::new();

    emoji_buffet.insert("Sally", "🥗 Salad");
    emoji_buffet.insert("Bob", "🍔 Burgers");
    emoji_buffet.insert("Alice", "🍎 Apple Pie");
    emoji_buffet.insert("Charlie", "🍗 Chicken Wings");
    emoji_buffet.insert("Eve", "🍣 Sushi");
    emoji_buffet.insert("Bob", "🍖 BBQ Ribs"); // Bob's extra contribution!

    println!("The emoji buffet (order might be jumbled 😵):");
    for (name, dish) in emoji_buffet.iter() {
        println!("{}: {}", name, dish);
    }

    let emoji_dishes: Vec<&str> = emoji_buffet.values().copied().collect();
    println!("\nJust the emoji dishes: {:?}", emoji_dishes);
}
```

### With `IndexMap` (A Perfectly Ordered Spread ✨)

```rust,editable
use indexmap::IndexMap;

fn main() {
    let mut emoji_arrivals = IndexMap::new();

    emoji_arrivals.insert("Sally", "🥗 Salad");
    emoji_arrivals.insert("Bob", "🍔 Burgers");
    emoji_arrivals.insert("Alice", "🍎 Apple Pie");
    emoji_arrivals.insert("Charlie", "🍗 Chicken Wings");
    emoji_arrivals.insert("Eve", "🍣 Sushi");
    emoji_arrivals.insert("Bob", "🍖 BBQ Ribs"); // Bob brought even more! Order of arrival for Bob is maintained.

    println!("The emoji arrivals (in perfect order! 🤩):");
    for (name, dish) in emoji_arrivals.iter() {
        println!("{}: {}", name, dish);
    }

    let dishes_in_order = emoji_arrivals.values().copied().collect::<Vec<&str>>();
    println!("\nThe emoji dishes in their arrival sequence: {:?}", dishes_in_order);
}
```
## Output
```
The emoji arrivals (in perfect order! 🤩):
Sally: 🥗 Salad
Bob: 🍖 BBQ Ribs
Alice: 🍎 Apple Pie
Charlie: 🍗 Chicken Wings
Eve: 🍣 Sushi

The emoji dishes in their arrival sequence: ["🥗 Salad", "🍖 BBQ Ribs", "🍎 Apple Pie", "🍗 Chicken Wings", "🍣 Sushi"]
```
![](/assets/duck.png) <span class="speech-bubble">🎉 `IndexMap` helps us see the food in the order it arrived! 😋</span>

> 💡 Read more [here](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html)
