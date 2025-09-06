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
    <div className="ml-4 flex items-center">
      <span className="mr-2">{getStatusIcon()}</span>
      <span className={`text-sm font-medium ${getStatusColor()}`}>
        {getStatusText()}
      </span>
      {connection.error && (
        <span className="ml-2 text-xs text-red-500">{connection.error}</span>
      )}
    </div>
  );
};
