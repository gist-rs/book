use futures::executor::block_on;
use futures_signals::signal::{Mutable, SignalExt};
use std::thread::{sleep, spawn};
use std::time::Duration;

async fn do_some_async_calculation(value: u32) -> u32 {
    // Simulate an async calculation by adding a delay
    println!("Starting async calculation (x2) for value: {}", value);
    sleep(Duration::from_secs(1));
    let result = value * 2;
    println!(
        "Async calculation (x2) completed for value: {}, result: {}",
        value, result
    );
    result
}

fn main() {
    // We create a Mutable with an initial value of 5.
    let my_state = Mutable::new(5);
    println!("Start with: {:?}", my_state);

    // We create a signal chain that:
    // 1. Adds 5 to the current value of my_state.
    // 2. Performs an async calculation that doubles the value.
    // 3. Deduplicates the result to avoid unnecessary updates.
    let output = my_state
        .signal()
        .map(|value| {
            let new_value = value + 5;
            println!("After map: {}", new_value);
            new_value
        })
        .map_future(|value| {
            println!("Before async calculation: {}", value);
            do_some_async_calculation(value)
        })
        .dedupe();

    // Watch for signal dispatch and print the new value.
    // The closure will be called with the current value of the signal chain.
    let future = output.for_each(|value| {
        println!("New value: {:?}", value);
        async {}
    });

    // Spawn the future to start watching for changes in a separate thread.
    spawn(move || {
        block_on(future);
    });

    // Change the state of my_state to 10 to trigger the signal.
    println!("Changing my_state to 10");
    my_state.set(10);

    // Keep the main thread alive to allow async tasks to run.
    sleep(Duration::from_secs(3));
}
