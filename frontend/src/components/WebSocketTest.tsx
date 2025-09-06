import React, { useState, useEffect } from "react";

const WebSocketTest: React.FC = () => {
  const [status, setStatus] = useState<string>("Disconnected");
  const [messages, setMessages] = useState<string[]>([]);
  const [ws, setWs] = useState<WebSocket | null>(null);

  const addMessage = (message: string) => {
    setMessages((prev) => [
      ...prev,
      `${new Date().toLocaleTimeString()}: ${message}`,
    ]);
  };

  const connect = () => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      addMessage("Already connected");
      return;
    }

    setStatus("Connecting...");
    addMessage("Attempting to connect to ws://localhost:8080/ws");

    try {
      const websocket = new WebSocket("ws://localhost:8080/ws");
      setWs(websocket);

      websocket.onopen = () => {
        setStatus("Connected ✅");
        addMessage("WebSocket connection established");
      };

      websocket.onmessage = (event) => {
        addMessage(`Received: ${event.data}`);
        try {
          const data = JSON.parse(event.data);
          addMessage(`Parsed: ${JSON.stringify(data, null, 2)}`);
        } catch (e) {
          addMessage("Could not parse as JSON");
        }
      };

      websocket.onclose = () => {
        setStatus("Disconnected");
        addMessage("WebSocket connection closed");
      };

      websocket.onerror = (error) => {
        setStatus("Error ❌");
        addMessage(`WebSocket error: ${error}`);
      };
    } catch (error) {
      setStatus("Connection Failed");
      addMessage(`Failed to create WebSocket: ${error}`);
    }
  };

  const disconnect = () => {
    if (ws) {
      ws.close();
      setWs(null);
      setStatus("Disconnected");
      addMessage("Manually disconnected");
    }
  };

  const clearMessages = () => {
    setMessages([]);
  };

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">🔌 WebSocket Connection Test</h1>

      <div className="mb-4">
        <div
          className={`p-3 rounded ${
            status.includes("Connected")
              ? "bg-green-100 text-green-800"
              : status.includes("Error")
              ? "bg-red-100 text-red-800"
              : "bg-yellow-100 text-yellow-800"
          }`}
        >
          Status: {status}
        </div>
      </div>

      <div className="mb-4 space-x-2">
        <button
          onClick={connect}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Connect
        </button>
        <button
          onClick={disconnect}
          className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Disconnect
        </button>
        <button
          onClick={clearMessages}
          className="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
        >
          Clear Messages
        </button>
      </div>

      <div className="bg-gray-100 p-4 rounded max-h-96 overflow-y-auto">
        <h3 className="font-semibold mb-2">Messages:</h3>
        {messages.length === 0 ? (
          <p className="text-gray-500">No messages yet...</p>
        ) : (
          messages.map((msg, index) => (
            <div key={index} className="text-sm mb-1 font-mono">
              {msg}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default WebSocketTest;
