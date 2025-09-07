import { useState, useEffect, useRef, useCallback } from "react";

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

  // Store callbacks in refs to avoid recreating the connect function
  const onMessageRef = useRef(onMessage);
  const onOpenRef = useRef(onOpen);
  const onCloseRef = useRef(onClose);
  const onErrorRef = useRef(onError);

  // Update refs when callbacks change
  useEffect(() => {
    onMessageRef.current = onMessage;
    onOpenRef.current = onOpen;
    onCloseRef.current = onClose;
    onErrorRef.current = onError;
  }, [onMessage, onOpen, onClose, onError]);

  const connect = useCallback(() => {
    try {
      const ws = new WebSocket(url);
      wsRef.current = ws;

      ws.onopen = () => {
        setIsConnected(true);
        setError(null);
        onOpenRef.current?.();
      };

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          setLastMessage(data);
          onMessageRef.current?.(data);
        } catch (e) {
          console.error("Failed to parse message:", e);
          setLastMessage(event.data);
        }
      };

      ws.onclose = () => {
        setIsConnected(false);
        onCloseRef.current?.();
      };

      ws.onerror = (error) => {
        setError("WebSocket connection error");
        setIsConnected(false);
        onErrorRef.current?.(error);
      };
    } catch (error) {
      console.error("Failed to create WebSocket:", error);
      setError("Failed to create WebSocket connection");
    }
  }, [url, onOpen, onMessage, onClose, onError]);

  const disconnect = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
      setIsConnected(false);
    }
  }, []);

  const sendMessage = useCallback((message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  }, []);

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
