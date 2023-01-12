window.onload = () => {
  // Open external link as new windows.
  ;[...document.getElementsByTagName('a')].map((e) => {
    // Only external link and not targeted yet will open new window
    if (e.href.startsWith('https://') && !e.href.startsWith(window.location.origin)) {
      e.target = e.target || '_blank'
      // Chrome < 88 still need this even targeted _blank.
      e.rel = 'noopener'
    }
    return e
  })

  // Render NFT contents
  const nft_elements = document.getElementsByTagName('nft')
  if (nft_elements.length > 0) {
    ;[...nft_elements].map((e) => {
      const { access_token } = window?.__SESSION__ || { access_token: 'foo' }

      if (!access_token) {
        if (e.innerHTML) {
          return
        } else {
          e.innerHTML = `<div style="border-style: dashed; border-width: 1px;
          padding: 0.5em;">Apply <button>NFT membership</button> to view this content.</div>`
          return
        }
      }

      const { chain, cluster } = { chain: 'solana', cluster: 'mainnet-beta', ...e.dataset }
      const address = e.getAttribute('src')
      if (!address) {
        return 'expected: src'
      }

      const src = `https://gist.rs/nft/${address}/?chain=${chain}&network=${cluster}`

      // 2. Apply iframe
      e.innerHTML = `<iframe src="${src}" />`

      // 3. Remove parent style
      e.style.padding = 0
      e.style.borderWidth = 0
    })
  }
}
