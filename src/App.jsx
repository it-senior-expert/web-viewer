import { useEffect, useState, useLayoutEffect } from "react";
import { Box, Tabs, TabList, Tab, Button, Input, IconButton } from '@chakra-ui/react';
import { ChakraProvider } from "@chakra-ui/react";
import { useToast } from "@chakra-ui/react";
import { FiRefreshCcw, FiX, FiMinimize2, FiMaximize2 } from "react-icons/fi";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { load } from '@tauri-apps/plugin-store';
import "./App.css";

import { getUrlById } from "./utils";

function App() {
  const [url_, setUrl_] = useState("https://google.com");
  const [width_, setWidth_] = useState();
  const [tabs, setTabs] = useState([{ id: 1, name: 'Google', url: 'https://google.com' }]);
  const [activeTab, setActiveTab] = useState(0);

  const toast = useToast();

  const removeTab = (tabId) => {
    const updatedTabs = tabs.filter((tab) => tab.id !== tabId);
    setTabs(updatedTabs);
    if (updatedTabs.length === 0) {
      setActiveTab(0);
    } else if (activeTab >= updatedTabs.length) {
      setActiveTab(updatedTabs.length - 1);
    }
  };

  const isValidUrl = (str) => {
    try {
      new URL(str);
      return true;
    } catch (_) {
      return false;
    }
  };

  const leftURLSubmit = async (e) => {
    e.preventDefault();
    if (!isValidUrl(url_)) {
      toast({
        title: "Invalid URL",
        description: "Input Exact URL!",
        status: "warning",
        duration: 2000,
        isClosable: true,
      });
      return;
    }

    const domain = new URL(url_).hostname.split('.')[0];
    const newTab = { id: tabs.length + 1, name: domain, url: url_ };
    setTabs([...tabs, newTab]);
    try {
      await invoke('new_left_url', { url: url_ });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  const refrechSubmit = async () => {
    try {
      await invoke('new_left_url', { url: url_ });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  }

  const rightURLSubmit = async (index) => {
    const right_url = getUrlById(tabs, index + 1);
    try {
      await invoke('new_right_url', { url: right_url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  useLayoutEffect(() => {
    const fetchData = async () => {
      const store = await load('store.json', { autoSave: false });
      const val = await store.get('ratio_of_screen');
      const newWidth = 100.0 - 100.0 / val["value"];
      setWidth_(`calc(${newWidth}% - 16px)`);
    };
    fetchData();
  }, []);

  useEffect(() => {
    listen('new-width', (event) => {
      setWidth_(event.payload - 16);
    });
  }, []);

  // Placeholder functions for button actions (minimize, maximize, close)
  const handleMinimize = () => {
    console.log("Minimize clicked");
    // Add Tauri command for minimize here
  };

  const handleMaximize = () => {
    console.log("Maximize clicked");
    // Add Tauri command for maximize here
  };

  const handleClose = () => {
    console.log("Close clicked");
    // Add Tauri command for close here
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
          padding="5px 5px 0 5px"
          backgroundColor="#F1F1F1" // Light gray background (like Chrome)
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
              maxWidth="80px"
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
            top="0"
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
              onClick={refrechSubmit}
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
    </ChakraProvider>
  );
}

export default App;
