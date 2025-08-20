(async function schema() {
  const output = document.getElementById('output')

  try {
    const response = await apiGet('/api/schema')
    if (response.error) {
      return showError(output, response.error)
    }

    const orderedKeys = Array.from(Object.keys(response.tables)).sort();

    for (const key of orderedKeys) {
      const value = response.tables[key]
      const cols = ["Name", "Type"]

      const tblWrapper = document.createElement('div')
      tblWrapper.classList.add('schema-table-wrapper')

      const title = document.createElement('h3')
      title.innerText = key
      tblWrapper.appendChild(title)

      const fks = value.foreign_keys

      const tbl = buildTable(cols, value.columns.map(v => {
        let props = ""
        if (v.is_pk) {
          props += " PRIMARY KEY"
        }

        if (v.not_null) {
          props += " NOT NULL"
        }

        let fkInfo = ""
        if (fks.length > 0) {
          const findFk = fks.filter(f => f.from == v.name)
          if (findFk.length == 1) {
            const fk = findFk[0]
            fkInfo += `<span class="fk-info"><a href="#${fk.table}">(${fk.table}.${fk.to})</a></span>`
          }
        }

        return [`<span>${v.name}</span>` + fkInfo, v.type + props]
      }))
      tblWrapper.appendChild(tbl)
      tblWrapper.id = key

      output.appendChild(tblWrapper)
    }
  } catch (ex) {
    showError(output, ex.message)
  }
})()