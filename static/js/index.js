(function index() {
  const output = document.getElementById('output')
  const button = document.getElementById('run-button')
  const input = document.getElementById('sql-input')

  input.addEventListener('keypress', e => {
    if (e.key === 'Enter' && e.ctrlKey) {
      button.click()
      e.preventDefault()
    }
  })

  button.addEventListener('click', async e => {
    const query = input.value.trim()
    if (!query) return

    output.replaceChildren()

    const response = await apiGet('/api/query', { q: query })
    if (response.error) {
      return showError(output, response.error)
    }

    const pre = document.createElement('pre')
    const code = document.createElement('code')

    pre.appendChild(code)
    code.textContent = JSON.stringify(response, null, 2)

    output.appendChild(buildTable(response.columns, response.rows))
    output.appendChild(pre)
    e.preventDefault()
  })
})()