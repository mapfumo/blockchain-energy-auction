import { renderHook, act } from "@testing-library/react";
import { useWebSocket } from "../useWebSocket";
import { SystemEvent } from "../../types/energy-trading";

// Mock WebSocket
const mockWebSocket = {
  close: jest.fn(),
  send: jest.fn(),
  readyState: WebSocket.OPEN,
  onopen: jest.fn(),
  onclose: jest.fn(),
  onmessage: jest.fn(),
  onerror: jest.fn(),
};

// Mock WebSocket constructor
const mockWebSocketConstructor = jest.fn(() => mockWebSocket);
(global as any).WebSocket = mockWebSocketConstructor;

describe("useWebSocket", () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockWebSocket.readyState = WebSocket.OPEN;
  });

  it("should initialize with disconnected state", () => {
    const { result } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    expect(result.current.connection.isConnected).toBe(false);
    expect(result.current.connection.error).toBe(null);
    expect(result.current.connection.reconnectAttempts).toBe(0);
  });

  it("should connect to WebSocket on mount", () => {
    renderHook(() => useWebSocket({ url: "ws://localhost:8080" }));

    expect(mockWebSocketConstructor).toHaveBeenCalledWith(
      "ws://localhost:8080"
    );
  });

  it("should handle connection open", () => {
    const { result } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    act(() => {
      if (mockWebSocket.onopen) {
        mockWebSocket.onopen(new Event("open"));
      }
    });

    expect(result.current.connection.isConnected).toBe(true);
    expect(result.current.connection.error).toBe(null);
    expect(result.current.connection.reconnectAttempts).toBe(0);
  });

  it("should handle connection close and attempt reconnection", () => {
    jest.useFakeTimers();
    const { result } = renderHook(() =>
      useWebSocket({
        url: "ws://localhost:8080",
        maxReconnectAttempts: 3,
        reconnectInterval: 1000,
      })
    );

    act(() => {
      if (mockWebSocket.onclose) {
        mockWebSocket.onclose(new Event("close"));
      }
    });

    expect(result.current.connection.isConnected).toBe(false);
    expect(result.current.connection.reconnectAttempts).toBe(1);

    // Fast-forward time to trigger reconnection
    act(() => {
      jest.advanceTimersByTime(1000);
    });

    expect(mockWebSocketConstructor).toHaveBeenCalledTimes(2);
    jest.useRealTimers();
  });

  it("should handle incoming messages", () => {
    const onMessage = jest.fn();
    const { result } = renderHook(() =>
      useWebSocket({
        url: "ws://localhost:8080",
        onMessage,
      })
    );

    const testEvent: SystemEvent = {
      type: "AuctionStarted",
      data: { auction_id: 1, total_energy: 100, reserve_price: 15 },
      timestamp: new Date().toISOString(),
    };

    act(() => {
      if (mockWebSocket.onmessage) {
        mockWebSocket.onmessage({
          data: JSON.stringify(testEvent),
        } as MessageEvent);
      }
    });

    expect(onMessage).toHaveBeenCalledWith(testEvent);
    expect(result.current.connection.lastMessage).toEqual(testEvent);
  });

  it("should handle malformed JSON messages", () => {
    const consoleSpy = jest.spyOn(console, "error").mockImplementation();
    const onMessage = jest.fn();

    renderHook(() =>
      useWebSocket({
        url: "ws://localhost:8080",
        onMessage,
      })
    );

    act(() => {
      if (mockWebSocket.onmessage) {
        mockWebSocket.onmessage({
          data: "invalid json",
        } as MessageEvent);
      }
    });

    expect(consoleSpy).toHaveBeenCalledWith(
      "Failed to parse WebSocket message:",
      expect.any(Error)
    );
    expect(onMessage).not.toHaveBeenCalled();

    consoleSpy.mockRestore();
  });

  it("should handle connection errors", () => {
    const onError = jest.fn();
    const { result } = renderHook(() =>
      useWebSocket({
        url: "ws://localhost:8080",
        onError,
      })
    );

    act(() => {
      if (mockWebSocket.onerror) {
        mockWebSocket.onerror(new Event("error"));
      }
    });

    expect(result.current.connection.error).toBe("WebSocket connection error");
    expect(onError).toHaveBeenCalled();
  });

  it("should send messages when connected", () => {
    const { result } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    const testMessage = { type: "test", data: "test" };

    act(() => {
      result.current.sendMessage(testMessage);
    });

    expect(mockWebSocket.send).toHaveBeenCalledWith(
      JSON.stringify(testMessage)
    );
  });

  it("should not send messages when disconnected", () => {
    const consoleSpy = jest.spyOn(console, "warn").mockImplementation();
    (mockWebSocket as any).readyState = WebSocket.CLOSED;

    const { result } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    act(() => {
      result.current.sendMessage({ type: "test" });
    });

    expect(consoleSpy).toHaveBeenCalledWith("WebSocket is not connected");
    expect(mockWebSocket.send).not.toHaveBeenCalled();

    consoleSpy.mockRestore();
  });

  it("should stop reconnecting after max attempts", () => {
    jest.useFakeTimers();
    const { result } = renderHook(() =>
      useWebSocket({
        url: "ws://localhost:8080",
        maxReconnectAttempts: 2,
        reconnectInterval: 1000,
      })
    );

    // Trigger multiple close events
    act(() => {
      if (mockWebSocket.onclose) {
        mockWebSocket.onclose(new Event("close"));
      }
    });

    act(() => {
      jest.advanceTimersByTime(1000);
    });

    act(() => {
      if (mockWebSocket.onclose) {
        mockWebSocket.onclose(new Event("close"));
      }
    });

    act(() => {
      jest.advanceTimersByTime(1000);
    });

    act(() => {
      if (mockWebSocket.onclose) {
        mockWebSocket.onclose(new Event("close"));
      }
    });

    expect(result.current.connection.error).toBe(
      "Max reconnection attempts reached"
    );
    expect(result.current.connection.reconnectAttempts).toBe(2);

    jest.useRealTimers();
  });

  it("should disconnect and cleanup on unmount", () => {
    const { unmount } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    unmount();

    expect(mockWebSocket.close).toHaveBeenCalled();
  });

  it("should handle connection creation errors", () => {
    const consoleSpy = jest.spyOn(console, "error").mockImplementation();
    mockWebSocketConstructor.mockImplementation(() => {
      throw new Error("Connection failed");
    });

    const { result } = renderHook(() =>
      useWebSocket({ url: "ws://localhost:8080" })
    );

    expect(result.current.connection.error).toBe(
      "Failed to create WebSocket connection"
    );

    consoleSpy.mockRestore();
  });
});
