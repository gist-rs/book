window.onload = () => {
  // Open external link as new windows.
  ;[...document.getElementsByTagName('a')].map((e) => {
    // Only external link and not targeted yet will open new window
    if (e.href.startsWith('https://') && !e.href.startsWith(`https://${window.location.origin}`)) {
      e.target = e.target || '_blank'
      // Chrome < 88 still need this even targeted _blank.
      e.rel = 'noopener'
    }
    return e
  })
}
