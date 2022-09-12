import React from 'react'
import styles from './sidebar.module.css'

const Sidebar = (props) => {
  return (
    <nav className={styles.sidebar}>
      {props.children}
    </nav>
  )
}

export default Sidebar