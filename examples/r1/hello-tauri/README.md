## Broken

This is an example from official but it's stuck at build page forever.

```
"build": {
    "beforeDevCommand": "dx serve --port 1420",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "dx build --release",
    "frontendDist": "../dist"
  },
```

## Workaround

Delete `beforeDevCommand` and run `dx serve --port 1420` ourself

```
"build": {
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "dx build --release",
    "frontendDist": "../dist"
  },
```

## Dev

```
dx serve --port 1420
cargo tauri dev
```
