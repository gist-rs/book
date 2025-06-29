# `Cow` 🐄 aka `Copy on write`

![](/assets/kat.png) <span class="speech-bubble"> Let's talk about `Cow` 🐄 aka `Copy on write`</span>

```rust,editable
use std::borrow::Cow;

fn main() {
    println!("🛸💨 Welcome to the Farm (and Alien Abduction) Fun Zone! 🐄\n");

    // --- 🐄 Our First Cow: Just Grazing Peacefully ---
    // This cow is just chillin' in the field, like a borrowed string.
    let mut peaceful_cow: Cow<'static, str> = Cow::Borrowed("Mooo, just grazing... 🌿");
    println!("1. Peaceful Cow: {}", peaceful_cow);
    println!("   (It's borrowed! Just observing this cow. 🧐)\n");

    // --- 👽 UFO Appears! Time to SUCK the Cow! ---
    // The UFO wants to "modify" the cow (i.e., abduct it!).
    // If it's a borrowed cow, the UFO needs to make its OWN copy to take!
    println!("2. OH NO! A UFO appears! 👽");

    // This forcing 👇 a copy for modification
    peaceful_cow.to_mut().push_str(" AIEEEE! I'm being beamed up! ⬆️");

    println!("   Cow after abduction attempt: {}", peaceful_cow);
    println!("   (Since it was borrowed, the UFO made a *copy* to abduct. The original is safe... for now! 🚀)\n");


    // --- 🐄 Our Second Cow: Already in Its Own Barn ---
    // This cow is already "owned" by us, like an owned String.
    let mut owned_cow: Cow<'_, str> = Cow::Owned(String::from("Mooo, I'm safe in MY barn! 🏡"));
    println!("3. Owned Cow: {}", owned_cow);
    println!("   (This cow is already owned! No initial copy needed. 😎)\n");

    // --- 👽 UFO Appears AGAIN! ---
    // The UFO wants to abduct THIS cow too!
    // But since this cow is *already owned*, the UFO can just grab *this specific cow*.
    // No new copy is needed because it's already "owned" for modification.
    println!("4. Another UFO! This one's persistent! 🛸");
    owned_cow.to_mut().push_str(" Help! They're taking MY barn too! 😱");
    println!("   Owned Cow after abduction: {}", owned_cow);
    println!("   (It was already owned, so the UFO just took *that* cow. Efficient abduction! 💯)\n");

    println!("🛸🐄💨 CoW: Keeping your original data safe from alien abduction, unless they need to make a copy! ✨");
}
```

> 💡 You can read more about `Cow` here 👉 [6 things you can do with the Cow 🐄 in Rust 🦀](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)
