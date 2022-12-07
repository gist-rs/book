// Add heart
const div_right = document.querySelector('#menu-bar > div.right-buttons')
const sponsor_html = '<a href="https://patreon.com/gist_rs" title="Sponsor" aria-label="Sponsor" target="_blank" rel="noopener"><i id="sponsor-button" class="fa fa-heart fa-beat beat-fade"></i></a>'
div_right.innerHTML = sponsor_html + div_right.innerHTML

class Web3LiteClient {
  constructor(options) {
    const { rpc_url } = {
      rpc_url: 'https://rpc.ankr.com/solana',
      ...options
    }
    this.rpc_url = rpc_url
  }

  call = async (method, params) => {
    const id = new Date().valueOf()
    const result = await fetch(this.rpc_url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id,
        method,
        params
      })
    })

    return await result?.json()
  }

  get_balance = async (pubkey, options) =>
    new Promise(async (resolve, reject) => {
      const { maximumFractionDigits } = {
        maximumFractionDigits: 2,
        ...options
      }

      const { result } = await this.call('getBalance', [pubkey]).catch(reject)
      if (isNaN(result?.value)) {
        reject(new Error('No value.'))
      }

      const { value } = result
      resolve(
        (value / Math.pow(10, 9) || 0).toLocaleString('en-US', {
          maximumFractionDigits
        })
      )
    })
}
class Web3LiteWallet {
  constructor(client) {
    this.client = client
  }

  to_base64(str) {
    str = typeof str === 'object' ? JSON.stringify(str) : str
    return btoa(encodeURIComponent(str))
  }

  from_base64(str) {
    return decodeURIComponent(atob(str))
  }

  connect = () => this.post_message('connect')

  post_message = async (method, params = {}) =>
    new Promise((resolve, _reject) => {
      const id = new Date().valueOf()
      window.addEventListener(
        'message',
        (event) => {
          if (!event.data) return
          const { data } = event
          if (!data.result) return

          if (data.id === id) return resolve(data.result)
        },
        false
      )

      window.solana.postMessage({
        method,
        params,
        id,
        jsonrpc: '2.0'
      })
    })

  connect_phantom = async () =>
    new Promise(async (resolve, reject) => {
      this.post_message('connect').then((result) => {
        return result?.publicKey ? resolve(result.publicKey) : reject('Not connected')
      })
    })
}

const GIST_PUBKEY = 'gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq'
const web3_lite_client = new Web3LiteClient()
web3_lite_client
  .get_balance(GIST_PUBKEY)
  .then((lamports) => {
    const sponsor_html = `<a href="#" onClick="window.connect_phantom()" title="Sponsor" aria-label="Sponsor"><pre class="balance">◎ ${lamports}</pre></a>`
    div_right.innerHTML = sponsor_html + div_right.innerHTML
  })
  .catch(console.error)

window.connect_phantom = () => {
  const web3_lite_wallet = new Web3LiteWallet(web3_lite_client)
  web3_lite_wallet.connect_phantom().then(async (public_key) => {
    window.public_key = public_key
    console.log('connected:', public_key)
  })
}
