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

      const src = `https://gist.rs/nft/${address}?chain=${chain}&cluster=${cluster}`

      // 2. Apply iframe
      e.innerHTML = `<iframe src="${src}" />`

      // 3. Remove parent style
      e.style.padding = 0
      e.style.borderWidth = 0
    })
  }

  // Render tab
  render_tab()
}

function focus_tab(labels, contents, tab_index) {
  labels.forEach((e, i) => (e.style.cssText = i === tab_index ? e.classList.add('tab-selected') : e.classList.remove('tab-selected')))
  contents.forEach((e, i) => (e.style.cssText = i === tab_index ? `display:block` : `display:none`))
}

function render_tab() {
  const tab_elements = document.getElementsByTagName('tabs')
  if (tab_elements.length <= 0) return
  ;[...tab_elements].map((tabs) => {
    const tab = tabs.getElementsByTagName('tab')

    const labels = []
    const contents = []

    ;[...tab].map((e, i) => {
      const label_text = e.getAttribute('label')
      const content = e.getElementsByTagName('pre').item(0)
      contents.push(content)

      const label = document.createElement('span')
      label.classList.add('tab')
      label.innerText = label_text
      labels.push(label)

      label.onclick = () => focus_tab(labels, contents, i)
    })

    labels.map((e) => tabs.parentNode.appendChild(e))
    contents.map((e) => tabs.parentNode.appendChild(e))

    // Default to first tab
    focus_tab(labels, contents, 0)
  })
}
