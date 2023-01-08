// import init, { get_nft_content } from './solana-toolbox.js'

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
  if (typeof init === 'function') {
    const nft_elements = document.getElementsByTagName('nft')
    if (nft_elements.length > 0) {
      init().then(async () => {
        ;[...nft_elements].map((e) => {
          const viewer_address = window.__SESSION__.pubkey
          e.innerHTML = get_nft_content(e.dataset.nft_address, viewer_address)
        })
      })
    }
  }
}
