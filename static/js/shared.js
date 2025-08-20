async function apiGet(path, query = undefined) {
  let url = path
  if (query) url += `?${new URLSearchParams(query)}`

  try {
    const response = await fetch(url)
    return await response.json()
  } catch (err) {
    console.error(err)
    return null
  }
}

function buildTable(columns, rows) {
  const wrapper = document.createElement('div')
  wrapper.classList.add('table-wrapper')

  const tbl = document.createElement('table')
  wrapper.appendChild(tbl)

  const hrow = tbl.insertRow()
  for (const col of columns) {
    const th = document.createElement('th')
    th.innerHTML = col
    hrow.appendChild(th)
  }

  for (const row of rows) {
    const trow = tbl.insertRow()
    for (const value of row) {
      trow.insertCell().innerHTML = value
    }
  }

  if (rows.length == 0) {
    const cell = tbl.insertRow().insertCell()
    cell.innerHTML = 'No data'
    cell.setAttribute('colspan', columns.length)
    cell.style.textAlign = 'center'
  }

  return wrapper
}

function showError(output, message) {
  const errorWrapper = document.createElement('div')
  errorWrapper.classList.add('error-wrapper')

  const text = document.createElement('p')
  text.innerHTML = '<span>ERROR</span>: ' + message

  errorWrapper.appendChild(text)
  output.appendChild(errorWrapper)
}