import React from "react";
import Navbar from "./Components/global/Navbar";
import Sidebar from "./Components/global/Sidebar";
import SidebarItem from "./Components/global/SidebarItem";
import { faCog, faDatabase, faNetworkWired, faTemperatureThreeQuarters, faCloudArrowUp, faPlugCirclePlus, faUsers } from "@fortawesome/free-solid-svg-icons";
import EndPointView from "./Components/EndPointView";
import DatabaseView from "./Components/DatabaseView";

function App() {
  const [sidebar, setSidebar] = React.useState(true);
  const [activeWindow, setActiveWindow] = React.useState("status");

  const setWindow = (name) => {
    setActiveWindow(name);
  };

  return (
    <>
      <Navbar isSidebarOpen={sidebar} onMenuClick={() => { setSidebar(!sidebar) }} />
      <div style={{ position: 'fixed', width: '100%', height: "100%", display: 'flex', flexDirection: 'row' }}>
        {sidebar &&
          <Sidebar>
            <SidebarItem icon={faTemperatureThreeQuarters} name="Status" active={activeWindow === "status"} onClick={() => setWindow("status")} />
            <SidebarItem icon={faDatabase} name="Database" active={activeWindow === "database"} onClick={() => setWindow("database")} />
            <SidebarItem icon={faNetworkWired} name="Endpoints" active={activeWindow === "endpoints"} onClick={() => setWindow("endpoints")} />
            <SidebarItem icon={faCloudArrowUp} name="Backup" active={activeWindow === "backup"} onClick={() => setWindow("backup")} />
            <SidebarItem icon={faUsers} name="Users" active={activeWindow === "users"} onClick={() => setWindow("users")} />
            <SidebarItem icon={faCog} name="Settings" active={activeWindow === "settings"} onClick={() => setWindow("settings")} />
            <SidebarItem icon={faPlugCirclePlus} name="Plugins" active={activeWindow === "plugins"} onClick={() => setWindow("plugins")} />
          </Sidebar>
        }
        {activeWindow === "endpoints" && 
          <EndPointView />
        }
        {activeWindow === "database" && 
          <DatabaseView />
        }
      </div>
    </>
  );
}

export default App;
