# Hello Wasm Sleep and Time

## Sleep

> How to delay as sleep in `Wasm`.

- std

  ```rust,no_run
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(1));
  ```

- wasm
  ```rust,no_run
    use fluvio_wasm_timer::Delay;
    Delay::new(Duration::from_secs(1)).await.ok();
  ```

## SystemTime

> How to get current time as `UNIX_EPOCH` in `Wasm`.

- std

  ```rust,no_run
  use std::time::SystemTime;

  let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
  ```

- wasm

  ```rust,no_run
  use fluvio_wasm_timer::SystemTime;

  let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_secs();
  ```
