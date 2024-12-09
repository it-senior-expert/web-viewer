import { useEffect, useState, useLayoutEffect, useRef } from "react";
import { Box, Tabs, TabList, Tab, Button, Input, IconButton } from '@chakra-ui/react';
import { ChakraProvider } from "@chakra-ui/react";
import { FiRefreshCcw, FiX, FiMinimize2, FiMaximize2 } from "react-icons/fi";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import "./App.css";

import { isValidUrl, saveToLocalStorage, loadFromLocalStorage } from "./utils";

function useLocalStorageWatcher(callback) {
  useEffect(() => {
    const handleStorageChange = (event) => {
      if (event.storageArea === localStorage) {
        callback(event);
      }
    };

    window.addEventListener('storage', handleStorageChange);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
    };
  }, [callback]);
}

function App() {
  const [url_, setUrl_] = useState("");
  const [width_, setWidth_] = useState();
  const [tabs, setTabs] = useState([]);
  const [activeTab, setActiveTab] = useState(0);
  const containerRef = useRef(null);
  
  useLocalStorageWatcher((event) => {
    console.log('Storage changed: ', event.key, event.newValue);
  });
  const removeTab = async (tabId) => {
    const updatedTabs = tabs.filter((tab) => tab.id !== tabId);
    setTabs(updatedTabs);
    if (updatedTabs.length === 0) {
      setActiveTab(0);
    } else if (activeTab >= updatedTabs.length) {
      setActiveTab(updatedTabs.length - 1);
    }
  };

  const leftURLSubmit = async (e) => {
    e.preventDefault();
    let left_url = url_
    if (!isValidUrl(left_url)) {
      left_url = `https://www.google.com/search?q=${url_}`
    }
    // const domain = new URL(left_url).hostname.split('.')[1];
    const domain = left_url.split('//')[1];
    const newTab = { id: tabs[tabs.length - 1].id + 1, name: domain, url: left_url };
    setActiveTab(tabs.length)
    setTabs([...tabs, newTab]);
    try {
      await invoke('new_left_url', { url: left_url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  const refreshSubmit = async () => {
    try {
      await invoke('new_left_url', { url: url_ });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  }

  const leftURLSubmitTab = async (index) => {
    const left_url = tabs[index].url;
    try {
      await invoke('new_left_url', { url: left_url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  const handleMouseDown = (e) => {
    e.preventDefault();
    const isMobile = window.innerWidth <= 390;
    const startValue = isMobile ? e.clientY : e.clientX; // Use Y-axis for mobile, X-axis otherwise
    const initialValue = width_; // Common state for both cases

    const handleMouseMove = async (moveEvent) => {
      const deltaValue = isMobile
        ? moveEvent.clientY - startValue
        : moveEvent.clientX - startValue;

      const newValue = Math.max(0, initialValue + deltaValue);
      await invoke('new_size', {
        leftWidth: isMobile ? width_ : newValue,
        width: window.innerWidth,
        height: isMobile ? newValue : window.innerHeight,
      });

      try {
        await saveToLocalStorage(isMobile ? 'top_height' : 'left_width', newValue);
      } catch (error) {
        console.error('Error saving size:', error);
      }

      setWidth_(newValue);
    };

    const handleMouseUp = () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  };
  const handleMinimize = async () => {
    await getCurrentWindow().minimize();
  };

  const handleMaximize = async () => {
    await getCurrentWindow().maximize();
  };

  const handleClose = async () => {
    await getCurrentWindow().destroy();
  };

  useLayoutEffect(() => {
    let newTabs = loadFromLocalStorage('tabs');
    if (newTabs == null) {
      setTabs([{ id: 1, name: 'Google', url: 'https://google.com' }]);
    } else {
      setTabs(newTabs);
    }

    let newSize = loadFromLocalStorage(window.innerWidth <= 390 ? 'top_height' : 'left_width');
    if (newSize == null) {
      newSize = window.innerWidth * 4.0 / 5.0;
      setWidth_(newSize);
    } else {
      setWidth_(newSize);
    }

    const fetchData = async () => {
      try {
        await invoke('new_size', {
          leftWidth: window.innerWidth <= 390 ? width_ : newSize,
          width: window.innerWidth,
          height: window.innerHeight,
        });
      } catch (error) {
        console.error('Error invoking Tauri command:', error);
      }
    };
    fetchData();
  }, []);

  useEffect(() => {
    const saveTabs = async () => {
      try {
        await saveToLocalStorage('tabs', tabs);
      } catch (error) {
        console.error('Error saving tabs:', error);
      }
    };

    if (tabs.length > 0) {
      saveTabs();
    }
  }, [tabs])

  return (
    <ChakraProvider>
      <Tabs
        index={activeTab}
        onChange={setActiveTab}
        isFitted
      >
        <TabList
          display="flex"
          justifyContent="left"
          alignItems="center"
          padding="10px 10px 0 10px"
          borderRadius="8px 8px 0 0"
          backgroundColor="#a3c9ff"
        >
          {tabs.map((tab) => (
            <Tab
              key={`tab-${tab.id}`}
              fontSize="xs"
              display="flex"
              alignItems="center"
              padding={0}
              bg={activeTab === tabs.indexOf(tab) ? "#FFFFFF" : "#E0E0E0"}
              borderRadius="3px"
              maxWidth="100px"
              onClick={() => leftURLSubmitTab(activeTab)}
              margin="1px"
            >
              <Box
                style={{
                  display: "flex",
                  justifyContent: "flex-start",
                  alignItems: "center",
                  maxWidth: "calc(100% - 20px)",
                  overflow: "hidden",
                  whiteSpace: "nowrap",
                  textOverflow: "ellipsis",
                  color: "#333333",
                }}
              >
                {tab.name}
              </Box>
              <Button
                variant="ghost"
                size="xs"
                onClick={() => removeTab(tab.id)}
                p={0}
                minWidth="20px"
                ml={1}
                color="#888888"
                _hover={{
                  backgroundColor: "#D3D3D3",
                }}
              >
                X
              </Button>
            </Tab>
          ))}
          <Box
            position="absolute"
            right="0"
            zIndex="999"
            display="flex"
            gap="5px"
          >
            <IconButton
              icon={<FiMinimize2 />}
              aria-label="Minimize"
              size="sm"
              onClick={handleMinimize}
              variant="ghost"
            />
            <IconButton
              icon={<FiMaximize2 />}
              aria-label="Maximize"
              size="sm"
              onClick={handleMaximize}
              variant="ghost"
            />
            <IconButton
              icon={<FiX />}
              aria-label="Close"
              size="sm"
              onClick={handleClose}
              variant="ghost"
            />
          </Box>
        </TabList>
        <Box style={{ width: width_ }} padding="7px" display="flex" alignItems="center" justifyContent="space-between">
          <form onSubmit={(e) => { leftURLSubmit(e); }} style={{ display: 'flex', width: '100%' }}>
            <IconButton
              icon={<FiRefreshCcw />}
              aria-label="Refresh"
              size="xs"
              onClick={refreshSubmit}
              _hover={{
                backgroundColor: "#E0E0E0",
              }}
            />
            <Input
              onChange={(e) => setUrl_(e.target.value)}
              placeholder="Enter a URL..."
              size="xs"
              borderRadius="md"
              marginLeft="5px"
              _focus={{
                borderColor: "#888888",
              }}
            />
          </form>
        </Box>
      </Tabs>
      <Box
        width='100%'
        height='calc(100vh - 107px)'
        ref={containerRef}
      >
        <Box
          className="splitter"
          onMouseDown={handleMouseDown}
          style={{
            marginLeft: window.innerWidth <= 390 ? 0 : width_,
            width: window.innerWidth <= 390 ? '100%' : '2px',
            height: window.innerWidth <= 390 ? '2px' : '100%',
            cursor: window.innerWidth <= 390 ? 'row-resize' : 'col-resize',
            backgroundColor: '#ccc',
          }}
        />
        <Box position="absolute">Hello</Box>
      </Box>
    </ChakraProvider>
  );
}

export default App;
