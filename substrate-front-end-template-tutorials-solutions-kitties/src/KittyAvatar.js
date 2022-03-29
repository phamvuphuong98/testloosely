import React from 'react'
const KittyAvatar = props => {
  const outerStyle = { height: '160px', position: 'relative', width: '100%', display: 'flex', 'justify-content': 'center', 'align-items': 'center', 'font-size': '50px' }
  const { domain } = props

  if (!domain) return null
  return (
    <div style={outerStyle}>
        <span>{domain}</span>
    </div>
  )
}

export default KittyAvatar
