// Add heart
const div_right = document.querySelector('#menu-bar > div.right-buttons')
const sponsor_html = '<a href="https://patreon.com/gist_rs" title="Sponsor" aria-label="Sponsor" target="_blank" rel="noopener"><i id="sponsor-button" class="fa fa-heart fa-beat beat-fade"></i></a>'
div_right.innerHTML = sponsor_html + div_right.innerHTML
