import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import styles from './navbar.module.css';
import Banner from '../../svgs/Banner.svg'

const Navbar = (props) => {

  return (
    <nav className={styles.navbar}>
      <div className={styles.navbar_left}>
        <FontAwesomeIcon onClick={() => props.onMenuClick()} className={`${styles.menu} ${props.isSidebarOpen && styles.menu_selected}`} size='xl' icon={faBars} />
        <a className={styles.logo} href="/">
          <img src={Banner} alt="axlyn" width="90px" />
        </a>
      </div>
      <div className={styles.navbar_right}>
        <a className={styles.logout} href="/logout">logout</a>
      </div>
    </nav>
  )
}

export default Navbar