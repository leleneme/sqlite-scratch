(async function schema() {
  const output = document.getElementById('output')
  console.log(output)

  try {
    const response = await apiGet('/api/schema')
    if (response.error) {
      return showError(output, response.error)
    }

    for (const [key, value] of Object.entries(response.tables)) {
      const cols = ["Name", "Type"]

      const tblWrapper = document.createElement('div')
      tblWrapper.classList.add('table-wrapper')

      const title = document.createElement('h3')
      title.innerText = key
      tblWrapper.appendChild(title)

      const tbl = buildTable(cols, value.map(v => {
        let props = ""
        if (v.is_pk) {
          props += " PRIMARY KEY"
        }

        if (v.not_null) {
          props += " NOT NULL"
        }

        return [v.name, v.type + props]
      }))
      tblWrapper.appendChild(tbl)

      output.appendChild(tblWrapper)
    }
  } catch (ex) {
    showError(output, ex.message)
  }
})()