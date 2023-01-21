# Fetch balance

![](/assets/kat.png) Usually we will use [Solana SDK](https://solanacookbook.com/references/accounts.html#how-to-get-account-balance) but we doing vanilla here so...

## How to fetch account balance in `Javascript` vanilla.

```javascript
const get_balance = async (pubkey, options) =>
  new Promise(async (resolve, reject) => {
    const { rpc_url, id, maximumFractionDigits } = {
      id: parseInt(100 * Math.random()),
      maximumFractionDigits: 2,
      ...options
    }

    const result = await fetch(rpc_url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id,
        method: 'getBalance',
        params: [pubkey]
      })
    }).catch(reject)

    const json = await result?.json().catch(reject)

    if (isNaN(json?.result?.value)) {
      reject(new Error('No result.'))
    }

    resolve(
      (json?.result?.value / Math.pow(10, 9) || 0).toLocaleString('en-US', {
        maximumFractionDigits
      })
    )
  })

// Consider donate some SOL to this 👇 account to see some number show up 😆
get_balance('gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq', {
  rpc_url: 'https://rpc.ankr.com/solana'
})
  .then(console.log)
  .catch(console.error)
```

---

![](/assets/kat.png) Now let's try `Rust` version.

## How to fetch account balance in `Rust` vanilla.

<tabs>
<tab label="main.rs">

```rust,edition2021
{{#include ../../examples/solana/fetch-balance/src/main.rs}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../examples/solana/fetch-balance/Cargo.toml}}
```

</tab>
</tabs>
