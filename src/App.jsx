import { useEffect, useState, useLayoutEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { load } from '@tauri-apps/plugin-store';
import "./App.css";

function App() {

  const [url, setUrl] = useState("");
  const [width_, setWidth_] = useState();

  async function urlFunction() {
    try {
      await invoke('new_url', { url });
    } catch (error) {
      console.error("Error invoking Tauri command:", error);
    }
  }

  useLayoutEffect(() => {
    const fetchData = async () => {
      const store = await load('store.json', { autoSave: false });
      const val = await store.get('ratio_of_screen');
      const newWidth = 100.0 - 100.0 / val["value"];
      setWidth_(`calc(${newWidth}% - 16px)`);
    };
    fetchData();
  })

  useEffect(() => {
    listen('new-width', (event) => {
      setWidth_(event.payload - 16);
    });
  })

  return (
    <main
      style={{ width: width_ }}
      className="input-container">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          urlFunction();
        }}
      >
        <input
          id="url-input"
          className="input"
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="Enter a URL..."
        />
        <button
          className="open-button"
          style={{ width: '80px' }}
          type="submit"
        >
          Open
        </button>
      </form>
    </main>
  );
}

export default App;
