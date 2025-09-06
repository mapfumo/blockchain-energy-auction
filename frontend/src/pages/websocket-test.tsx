import React, { useState, useEffect } from "react";

export default function WebSocketTest() {
  const [status, setStatus] = useState("Disconnected");
  const [messages, setMessages] = useState<string[]>([]);
  const [ws, setWs] = useState<WebSocket | null>(null);

  const connect = () => {
    if (ws) {
      ws.close();
    }

    setStatus("Connecting...");
    setMessages((prev) => [
      ...prev,
      "Attempting to connect to ws://localhost:8080/ws",
    ]);

    try {
      const websocket = new WebSocket("ws://localhost:8080/ws");

      websocket.onopen = () => {
        setStatus("Connected ✅");
        setMessages((prev) => [...prev, "WebSocket connected successfully!"]);
      };

      websocket.onmessage = (event) => {
        const message = `Received: ${event.data}`;
        setMessages((prev) => [...prev, message]);
        console.log("WebSocket message:", event.data);
      };

      websocket.onclose = () => {
        setStatus("Disconnected ❌");
        setMessages((prev) => [...prev, "WebSocket connection closed"]);
      };

      websocket.onerror = (error) => {
        setStatus("Error ❌");
        setMessages((prev) => [...prev, `WebSocket error: ${error}`]);
        console.error("WebSocket error:", error);
      };

      setWs(websocket);
    } catch (error) {
      setStatus("Error ❌");
      setMessages((prev) => [...prev, `Connection failed: ${error}`]);
      console.error("WebSocket connection failed:", error);
    }
  };

  const disconnect = () => {
    if (ws) {
      ws.close();
      setWs(null);
    }
  };

  useEffect(() => {
    return () => {
      if (ws) {
        ws.close();
      }
    };
  }, [ws]);

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold mb-6">
          🔌 WebSocket Connection Test
        </h1>

        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h2 className="text-xl font-semibold">Connection Status</h2>
              <p
                className={`text-lg font-bold ${
                  status.includes("Connected")
                    ? "text-green-600"
                    : status.includes("Error")
                    ? "text-red-600"
                    : "text-yellow-600"
                }`}
              >
                {status}
              </p>
            </div>
            <div className="space-x-2">
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
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">Messages</h2>
          <div className="bg-gray-100 rounded p-4 h-96 overflow-y-auto">
            {messages.length === 0 ? (
              <p className="text-gray-500">
                No messages yet. Click Connect to start.
              </p>
            ) : (
              messages.map((message, index) => (
                <div key={index} className="mb-2 text-sm font-mono">
                  {message}
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
