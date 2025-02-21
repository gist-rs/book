use futures::{
    stream::{self, BoxStream},
    StreamExt,
};

#[tokio::main]
async fn main() {
    // Create a stream of strings.  We'll box it later.
    let string_stream = stream::iter(vec![
        "Hello".to_string(),
        "World".to_string(),
        "!".to_string(),
    ]);

    // Process the stream before boxing if needed.  For this simple example,
    // we'll just map the strings to their uppercase versions.  This step is
    // important to illustrate that you can perform operations *before* boxing.
    let processed_stream = string_stream.map(|s| s.to_uppercase());

    // Now, box the stream.  This is the key part of the example.
    let boxed_stream: BoxStream<'static, String> = processed_stream.boxed();

    // Consume and print the elements of the boxed stream.
    boxed_stream
        .for_each(|s| async move {
            println!("{}", s);
        })
        .await;

    // Example with an error type
    let error_stream = stream::iter(vec![Ok(1), Err("Oops"), Ok(3)]);
    let boxed_error_stream: BoxStream<'static, Result<i32, &'static str>> = error_stream.boxed();

    boxed_error_stream
        .for_each(|res| async move {
            match res {
                Ok(n) => println!("Number: {}", n),
                Err(e) => println!("Error: {}", e),
            }
        })
        .await;
}
