import { useState, useEffect, useRef } from "react";

interface UseSimpleWebSocketOptions {
  url: string;
  onMessage?: (data: any) => void;
  onOpen?: () => void;
  onClose?: () => void;
  onError?: (error: Event) => void;
}

export const useSimpleWebSocket = ({
  url,
  onMessage,
  onOpen,
  onClose,
  onError,
}: UseSimpleWebSocketOptions) => {
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastMessage, setLastMessage] = useState<any>(null);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = () => {
    console.log("🔌 Attempting to connect to:", url);

    try {
      const ws = new WebSocket(url);
      wsRef.current = ws;

      ws.onopen = () => {
        console.log("✅ WebSocket connected successfully");
        setIsConnected(true);
        setError(null);
        onOpen?.();
      };

      ws.onmessage = (event) => {
        console.log("📨 Received message:", event.data);
        try {
          const data = JSON.parse(event.data);
          console.log("📨 Parsed data:", data);
          setLastMessage(data);
          onMessage?.(data);
        } catch (e) {
          console.error("❌ Failed to parse message:", e);
          setLastMessage(event.data);
        }
      };

      ws.onclose = () => {
        console.log("🔌 WebSocket disconnected");
        setIsConnected(false);
        onClose?.();
      };

      ws.onerror = (error) => {
        console.error("❌ WebSocket error:", error);
        setError("WebSocket connection error");
        setIsConnected(false);
        onError?.(error);
      };
    } catch (error) {
      console.error("❌ Failed to create WebSocket:", error);
      setError("Failed to create WebSocket connection");
    }
  };

  const disconnect = () => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
      setIsConnected(false);
    }
  };

  const sendMessage = (message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    } else {
      console.warn("⚠️ WebSocket is not connected");
    }
  };

  useEffect(() => {
    connect();
    return () => disconnect();
  }, [url]);

  return {
    isConnected,
    error,
    lastMessage,
    connect,
    disconnect,
    sendMessage,
  };
};
