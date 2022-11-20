window.onload = () => {
  // Open external link as new windows.
  ;[...document.getElementsByTagName('a')].map((e) => {
    if (e.href.startsWith('https://')) e.target = '_blank'
    return e
  })
}
