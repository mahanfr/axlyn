import React from 'react'
import styles from './sidebarItem.module.css'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'


const SidebarItem = (props) => {
  return (
    <>
      <div 
        onClick={()=>props.onClick()} 
        className={`${styles.sidebarItem} ${props.active && styles.sidebarItem_active}`}>
        <FontAwesomeIcon className={styles.icon} icon={props.icon} />
        <h3 className={styles.title}>{props.name}</h3>
      </div>
    
    </>
  )
}

export default SidebarItem