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

  get_recent_blockhash = async () =>
    new Promise(async (resolve, reject) => {
      const { result } = await this.call('getRecentBlockhash').catch(reject)
      console.log(result)
      return result?.value?.blockhash ? resolve(result?.value?.blockhash) : reject(new Error('No blockhash.'))
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
      console.log(method, params)
      const id = new Date().valueOf()
      window.addEventListener(
        'message',
        (event) => {
          if (!event.data) return
          const { data } = event
          if (!data.result) return

          console.log(id, event)
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

  sign_and_send_transaction = async (data) =>
    new Promise((resolve, reject) => {
      const params = {
        message: this.to_base64(data)
      }
      this.post_message('signAndSendTransaction', params).then((result) => {
        console.log('result:', result)
        if (!result?.signature) return reject('No signature')
        resolve(result.signature)
      })
    })

  connect_phantom = async () =>
    new Promise(async (resolve, reject) => {
      this.post_message('connect').then((result) => {
        return result?.publicKey ? resolve(result.publicKey) : reject('Not connected')
      })
    })

  transfer_native = async (from, to, ui_lamports = 1) => {
    // const lamports = ui_lamports * Math.pow(10, 9)
    const recent_blockhash = await this.client.get_recent_blockhash()
    const message_data = {
      header: { num_required_signatures: 1, num_readonly_signed_accounts: 0, num_readonly_unsigned_accounts: 1 },
      account_keys: [from, to, '11111111111111111111111111111111'],
      recent_blockhash,
      instructions: { program_id_index: 2, accounts: [0, 1], data: [2, 0, 0, 0, 0, 202, 154, 59, 0, 0, 0, 0] }
    }
    console.log(message_data)

    return this.sign_and_send_transaction(message_data)
  }
}

const GIST_PUBKEY = 'gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq'
const web3_lite_client = new Web3LiteClient()
web3_lite_client
  .get_balance(GIST_PUBKEY)
  .then((lamports) => {
    const sponsor_html = `<a href="#" onClick title="Sponsor" aria-label="Sponsor" target="_blank" rel="noopener"><pre class="balance">◎ ${lamports}</pre></a>`
    div_right.innerHTML = sponsor_html + div_right.innerHTML
  })
  .catch(console.error)

const web3_lite_wallet = new Web3LiteWallet(web3_lite_client)
web3_lite_wallet.connect_phantom().then(async (public_key) => {
  window.public_key = public_key
  const result = await web3_lite_wallet.transfer_native(public_key, GIST_PUBKEY)
  console.log(result)
})
