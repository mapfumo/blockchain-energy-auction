import { useState, useEffect, useRef, useCallback } from "react";
import { SystemEvent, WebSocketConnection } from "../types/energy-trading";

interface UseWebSocketOptions {
  url: string;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
  onMessage?: (event: SystemEvent) => void;
  onError?: (error: Event) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

export const useWebSocket = ({
  url,
  reconnectInterval = 3000,
  maxReconnectAttempts = 5,
  onMessage,
  onError,
  onOpen,
  onClose,
}: UseWebSocketOptions) => {
  const [connection, setConnection] = useState<WebSocketConnection>({
    isConnected: false,
    lastMessage: null,
    error: null,
    reconnectAttempts: 0,
  });

  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const reconnectAttemptsRef = useRef(0);

  const connect = useCallback(() => {
    try {
      const ws = new WebSocket(url);
      wsRef.current = ws;

      ws.onopen = () => {
        console.log("WebSocket connected");
        setConnection((prev) => ({
          ...prev,
          isConnected: true,
          error: null,
          reconnectAttempts: 0,
        }));
        reconnectAttemptsRef.current = 0;
        onOpen?.();
      };

      ws.onmessage = (event) => {
        try {
          const data: SystemEvent = JSON.parse(event.data);
          setConnection((prev) => ({
            ...prev,
            lastMessage: data,
          }));
          onMessage?.(data);
        } catch (error) {
          console.error("Failed to parse WebSocket message:", error);
        }
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        setConnection((prev) => ({
          ...prev,
          error: "WebSocket connection error",
        }));
        onError?.(error);
      };

      ws.onclose = () => {
        console.log("WebSocket disconnected");
        setConnection((prev) => ({
          ...prev,
          isConnected: false,
        }));
        onClose?.();

        // Attempt to reconnect
        if (reconnectAttemptsRef.current < maxReconnectAttempts) {
          reconnectAttemptsRef.current += 1;
          setConnection((prev) => ({
            ...prev,
            reconnectAttempts: reconnectAttemptsRef.current,
          }));

          reconnectTimeoutRef.current = setTimeout(() => {
            console.log(
              `Attempting to reconnect (${reconnectAttemptsRef.current}/${maxReconnectAttempts})`
            );
            connect();
          }, reconnectInterval);
        } else {
          setConnection((prev) => ({
            ...prev,
            error: "Max reconnection attempts reached",
          }));
        }
      };
    } catch (error) {
      console.error("Failed to create WebSocket connection:", error);
      setConnection((prev) => ({
        ...prev,
        error: "Failed to create WebSocket connection",
      }));
    }
  }, [
    url,
    reconnectInterval,
    maxReconnectAttempts,
    onMessage,
    onError,
    onOpen,
    onClose,
  ]);

  const disconnect = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }

    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    setConnection({
      isConnected: false,
      lastMessage: null,
      error: null,
      reconnectAttempts: 0,
    });
  }, []);

  const sendMessage = useCallback((message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    } else {
      console.warn("WebSocket is not connected");
    }
  }, []);

  useEffect(() => {
    connect();

    return () => {
      disconnect();
    };
  }, [connect, disconnect]);

  return {
    connection,
    sendMessage,
    connect,
    disconnect,
  };
};
