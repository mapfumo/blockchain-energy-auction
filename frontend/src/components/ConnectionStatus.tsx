import React from "react";
import { WebSocketConnection } from "../types/energy-trading";

interface ConnectionStatusProps {
  connection: WebSocketConnection;
}

export const ConnectionStatus: React.FC<ConnectionStatusProps> = ({
  connection,
}) => {
  const getStatusColor = () => {
    if (connection.isConnected) return "text-green-600";
    if (connection.error) return "text-red-600";
    return "text-yellow-600";
  };

  const getStatusText = () => {
    if (connection.isConnected) return "Connected";
    if (connection.error) return "Error";
    if (connection.reconnectAttempts > 0)
      return `Reconnecting (${connection.reconnectAttempts})`;
    return "Connecting...";
  };

  const getStatusIcon = () => {
    if (connection.isConnected) return "🟢";
    if (connection.error) return "🔴";
    return "🟡";
  };

  return (
    <div className="flex items-center space-x-2">
      <div className="flex items-center">
        <span className="mr-2">{getStatusIcon()}</span>
        <span className={`text-sm font-medium ${getStatusColor()}`}>
          {getStatusText()}
        </span>
      </div>
      {connection.error && (
        <div className="flex items-center">
          <span className="text-xs text-red-500 bg-red-50 dark:bg-red-900/20 px-2 py-1 rounded">
            {connection.error}
          </span>
        </div>
      )}
      {connection.isConnected && (
        <div className="text-xs text-gray-500 dark:text-gray-400">
          Connected
        </div>
      )}
    </div>
  );
};
