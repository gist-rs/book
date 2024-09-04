use derive_more::{Add, Display, From, FromStr};

#[derive(FromStr, Display)]
pub struct PhoneNumber(String);

#[derive(Clone, Copy, From, Display, Add)]
#[display("{} years", _0)]
pub struct Years(u32);

fn main() {
    // Use parse.
    let num = "123-4567".parse::<PhoneNumber>().unwrap();
    println!("1️⃣ Phone number is {}", num);

    // Use from_str.
    let num = PhoneNumber::from_str("123-4567").unwrap();
    println!("2️⃣ Phone number is {}", num);

    // Use from and Add.
    let age_1 = Years::from(5);
    let age_2 = Years::from(2);
    println!("3️⃣ {} + {} = {}", age_1, age_2, age_1 + age_2);
}
