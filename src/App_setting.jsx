import React, { useState, useEffect } from 'react';
import {
  Box,
  Input,
  Button,
  VStack,
  HStack,
  Text,
  ChakraProvider,
} from '@chakra-ui/react';
import { invoke } from "@tauri-apps/api/core";
import { saveToLocalStorage, loadFromLocalStorage } from './utils';

const App_setting = () => {
  const [urlList, setUrlList] = useState(loadFromLocalStorage('urls') || []);
  const [urlInput, setUrlInput] = useState('');

  useEffect(() => {
    saveToLocalStorage('urls', urlList);
  }, [urlList]);

  const handleAddUrl = () => {
    const newUrl = urlInput.trim();
    if (newUrl) {
      setUrlList([...urlList, newUrl]);
      setUrlInput('');

    } else {
      console.log('There is no new url')
    }
  };

  const handleDeleteUrl = (index) => {
    const updatedUrls = urlList.filter((_, i) => i !== index);
    setUrlList(updatedUrls);
  };

  const handleOpenUrl = async (url) => {
    saveToLocalStorage('submit_url', url);
    
    try {
      await invoke('new_right_url', { url: url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  };

  return (
    <ChakraProvider>
      <Box p={4} bg="gray.100" minH="100vh">
        <VStack spacing={4} align="stretch">
          <HStack>
            <Input
              value={urlInput}
              onChange={(e) => setUrlInput(e.target.value)}
              placeholder="Enter new URL"
              size="sm"
              borderRadius="sm"
              focusBorderColor="blue.500"
            />
            <Button onClick={handleAddUrl} backgroundColor="#ddd" borderRadius="sm" size="sm" px={4} py={2}>
              Add
            </Button>
          </HStack>
          <VStack spacing={3} align="stretch">
            {urlList.map((url, index) => (
              <HStack
                key={index}
                justify="space-between"
                p={3}
                bg="white"
                borderRadius="xs"
                shadow="xs"
                _hover={{ shadow: 'lg' }}
              >
                <Text isTruncated flex="1" fontSize="sm">
                  {url}
                </Text>
                <HStack spacing={2}>
                  <Button
                    borderRadius="sm"
                    size="xs"
                    onClick={() => handleOpenUrl(url)}
                    px={2} // Shorter padding for compact button
                    py={1}
                    backgroundColor="#ddd"
                  >
                    Open
                  </Button>
                  <Button
                    size="xs"
                    borderRadius="sm"
                    colorScheme="red"
                    onClick={() => handleDeleteUrl(index)}
                    px={2} // Shorter padding for compact button
                    py={1}
                  >
                    Delete
                  </Button>
                </HStack>
              </HStack>
            ))}
          </VStack>
        </VStack>
      </Box>
    </ChakraProvider>
  );
};

export default App_setting;
