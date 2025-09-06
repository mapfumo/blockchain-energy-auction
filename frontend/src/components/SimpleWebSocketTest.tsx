import React from "react";
import { useSimpleWebSocket } from "../hooks/useSimpleWebSocket";

const SimpleWebSocketTest: React.FC = () => {
  const { isConnected, error, lastMessage, connect, disconnect } =
    useSimpleWebSocket({
      url: "ws://localhost:8080/ws",
      onMessage: (data) => {
        console.log("📨 Message received:", data);
      },
      onOpen: () => {
        console.log("✅ Connected!");
      },
      onClose: () => {
        console.log("🔌 Disconnected");
      },
      onError: (error) => {
        console.error("❌ Error:", error);
      },
    });

  const getStatusColor = () => {
    if (isConnected) return "text-green-600";
    if (error) return "text-red-600";
    return "text-yellow-600";
  };

  const getStatusText = () => {
    if (isConnected) return "Connected ✅";
    if (error) return `Error: ${error}`;
    return "Connecting...";
  };

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">🔌 Simple WebSocket Test</h1>

      <div className="mb-4">
        <div
          className={`p-3 rounded text-center font-semibold ${getStatusColor()}`}
        >
          {getStatusText()}
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
      </div>

      {lastMessage && (
        <div className="bg-gray-100 p-4 rounded">
          <h3 className="font-semibold mb-2">Last Message:</h3>
          <pre className="text-sm font-mono overflow-x-auto">
            {JSON.stringify(lastMessage, null, 2)}
          </pre>
        </div>
      )}

      <div className="mt-4 text-sm text-gray-600">
        <p>Check the browser console for detailed logs.</p>
        <p>Open Developer Tools (F12) → Console tab</p>
      </div>
    </div>
  );
};

export default SimpleWebSocketTest;
