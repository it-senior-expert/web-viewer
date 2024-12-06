import { useEffect, useState, useLayoutEffect, useRef } from "react";
import { Box, Tabs, TabList, Tab, Button, Input, IconButton } from '@chakra-ui/react';
import { ChakraProvider } from "@chakra-ui/react";
import { useToast } from "@chakra-ui/react";
import { FiRefreshCcw, FiX, FiMinimize2, FiMaximize2 } from "react-icons/fi";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { load } from '@tauri-apps/plugin-store';
import { getCurrentWindow } from '@tauri-apps/api/window';
import "./App.css";

import { isValidUrl, saveToLocalStorage, loadFromLocalStorage } from "./utils";

function App() {
  const [url_, setUrl_] = useState("https://google.com");
  const [width_, setWidth_] = useState();
  const [tabs, setTabs] = useState([]);
  const [activeTab, setActiveTab] = useState(0);
  const containerRef = useRef(null);
  const toast = useToast();

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
    if (!left_url.includes("https://")) {
      left_url = "https://" + left_url
    }
    if (!isValidUrl(left_url)) {
      toast({
        title: "Invalid URL",
        status: "warning",
        position: 'top-left',
        duration: 2000,
        isClosable: true,
      });
      return;
    }
    const domain = new URL(left_url).hostname.split('.')[0];
    const newTab = { id: tabs.length + 1, name: domain, url: left_url };
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

  const rightURLSubmit = async (index) => {
    const right_url = tabs[index].url;
    try {
      await invoke('new_right_url', { url: right_url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  useLayoutEffect(() => {
    try {
      const newTabs = loadFromLocalStorage('tabs');
      console.log(newTabs)
      setTabs(newTabs);
    } catch (error) {
      setTabs([{ id: 1, name: 'Google', url: 'https://google.com' }])
    }
    const fetchData = async () => {
      const store = await load('store.json', { autoSave: false });
      try {
        const val = await store.get('left_width');
        const newWidth = val["value"];
        setWidth_(newWidth)
      } catch (error) {
        setWidth_(window.innerWidth * 4.0 / 5.0);
      }
    };
    fetchData();
  }, []);

  useEffect(() => {
    listen('new-width', (event) => {
      setWidth_(event.payload - 16);
    });
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      const store = await load('store.json', { autoSave: false });
      if (width_ !== undefined) {
        await store.set('left_width', { value: width_ });
      }
    };
    fetchData();
  }, [width_])

  useEffect(() => {
    const saveTabs = async () => {
      try {
        await saveToLocalStorage('tabs', tabs);
      } catch (error) {
        console.error('Error saving tabs:', error);
      }
    };

    if (tabs.length > 0) { // Optional condition to avoid running on initial render
      saveTabs();
    }
  }, [tabs])

  const handleMouseDown = (e) => {
    e.preventDefault();
    const startX = e.clientX; // Initial mouse position
    const initialWidth = width_; // Current width value

    const handleMouseMove = async (moveEvent) => {
      const deltaX = moveEvent.clientX - startX; // Difference in X position
      const newWidth = Math.max(0, initialWidth + deltaX); // Ensure width doesn't go negative

      setWidth_(newWidth);
      try {
        await invoke('new_left_width', { width: newWidth });
      } catch (error) {
        console.error("Error invoking Tauri command:", error);
      }
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
          backgroundColor="#a3c9ff" // Light gray background (like Chrome)
        >
          {tabs.map((tab) => (
            <Tab
              key={`tab-${tab.id}`}
              fontSize="xs"
              display="flex"
              alignItems="center"
              padding={0}
              bg={activeTab === tabs.indexOf(tab) ? "#FFFFFF" : "#E0E0E0"} // Active tab highlight
              borderRadius="3px"
              maxWidth="100px"
              margin="1px"
            >
              <Box
                onClick={() => rightURLSubmit(activeTab)}
                style={{
                  display: "flex",
                  justifyContent: "flex-start",
                  alignItems: "center",
                  maxWidth: "calc(100% - 20px)",
                  overflow: "hidden",
                  whiteSpace: "nowrap",
                  textOverflow: "ellipsis",
                  color: "#333333", // Darker text color for contrast
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
                color="#888888" // Light gray color for the "X"
                _hover={{
                  backgroundColor: "#D3D3D3", // Light gray hover effect
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
                backgroundColor: "#E0E0E0", // Light gray on hover
              }}
            />
            <Input
              onChange={(e) => setUrl_(e.target.value)}
              placeholder="Enter a URL..."
              size="xs"
              borderRadius="md"
              marginLeft="5px"
              _focus={{
                borderColor: "#888888", // Light gray border focus
              }}
            />
          </form>
        </Box>
      </Tabs>
      <Box
        width='100%'
        height='calc(100vh - 83px)'
        ref={containerRef}
      >
        <Box
          marginLeft={`${width_}px`}
          className="splitter"
          onMouseDown={handleMouseDown}
        ></Box>
      </Box>
    </ChakraProvider>
  );
}

export default App;
