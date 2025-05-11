# hello-world-cloudflare

## Setup (once)
```
rustup target add wasm32-unknown-unknown
cargo install cargo-generate
```

## Setup new project
```
# From template
cargo generate cloudflare/workers-rs

# Or
npx wrangler init
```

## Run
```
npx wrangler dev
```

## Deploy
```
npx wrangler deploy
```

## Build and Deploy on Cloudflare
```
npx wrangler deploy -e production
```

## Try
```
http://localhost:8787/?wallet_address=gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq
```
