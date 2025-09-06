import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { Dashboard } from "../Dashboard";
import { useWebSocket } from "../../hooks/useWebSocket";
import { SystemEvent } from "../../types/energy-trading";

// Mock the useWebSocket hook
jest.mock("../../hooks/useWebSocket");
const mockUseWebSocket = useWebSocket as jest.MockedFunction<
  typeof useWebSocket
>;

// Mock the child components
jest.mock("../AuctionView", () => ({
  AuctionView: ({ auctions, bessNodes, aggregators }: any) => (
    <div data-testid="auction-view">
      <div>Auctions: {auctions.length}</div>
      <div>BESS Nodes: {bessNodes.length}</div>
      <div>Aggregators: {aggregators.length}</div>
    </div>
  ),
}));

jest.mock("../BESSNodeMap", () => ({
  BESSNodeMap: ({ bessNodes, aggregators }: any) => (
    <div data-testid="bess-node-map">
      <div>BESS Nodes: {bessNodes.length}</div>
      <div>Aggregators: {aggregators.length}</div>
    </div>
  ),
}));

jest.mock("../PriceAnalytics", () => ({
  PriceAnalytics: ({ priceHistory, auctions }: any) => (
    <div data-testid="price-analytics">
      <div>Price History: {priceHistory.length}</div>
      <div>Auctions: {auctions.length}</div>
    </div>
  ),
}));

jest.mock("../SystemMetrics", () => ({
  SystemMetrics: ({ metrics, bessNodes, aggregators }: any) => (
    <div data-testid="system-metrics">
      <div>Metrics: {metrics ? "Present" : "None"}</div>
      <div>BESS Nodes: {bessNodes.length}</div>
      <div>Aggregators: {aggregators.length}</div>
    </div>
  ),
}));

jest.mock("../ConnectionStatus", () => ({
  ConnectionStatus: ({ connection }: any) => (
    <div data-testid="connection-status">
      <div>Connected: {connection.isConnected ? "Yes" : "No"}</div>
      <div>Error: {connection.error || "None"}</div>
    </div>
  ),
}));

describe("Dashboard", () => {
  const mockConnection = {
    isConnected: true,
    lastMessage: null,
    error: null,
    reconnectAttempts: 0,
  };

  const mockSendMessage = jest.fn();

  beforeEach(() => {
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  it("should render dashboard with header and navigation", () => {
    render(<Dashboard />);

    expect(screen.getByText("Energy Trading Dashboard")).toBeInTheDocument();
    expect(
      screen.getByText("Real-time energy auction monitoring")
    ).toBeInTheDocument();
    expect(screen.getByText("Live Auctions")).toBeInTheDocument();
    expect(screen.getByText("BESS Nodes")).toBeInTheDocument();
    expect(screen.getByText("Price Analytics")).toBeInTheDocument();
    expect(screen.getByText("System Metrics")).toBeInTheDocument();
  });

  it("should display connection status", () => {
    render(<Dashboard />);

    expect(screen.getByTestId("connection-status")).toBeInTheDocument();
    expect(screen.getByText("Connected: Yes")).toBeInTheDocument();
  });

  it("should start with auctions tab active", () => {
    render(<Dashboard />);

    expect(screen.getByTestId("auction-view")).toBeInTheDocument();
    expect(screen.queryByTestId("bess-node-map")).not.toBeInTheDocument();
    expect(screen.queryByTestId("price-analytics")).not.toBeInTheDocument();
    expect(screen.queryByTestId("system-metrics")).not.toBeInTheDocument();
  });

  it("should switch tabs when clicked", () => {
    render(<Dashboard />);

    // Click on BESS Nodes tab
    fireEvent.click(screen.getByText("BESS Nodes"));
    expect(screen.getByTestId("bess-node-map")).toBeInTheDocument();
    expect(screen.queryByTestId("auction-view")).not.toBeInTheDocument();

    // Click on Price Analytics tab
    fireEvent.click(screen.getByText("Price Analytics"));
    expect(screen.getByTestId("price-analytics")).toBeInTheDocument();
    expect(screen.queryByTestId("bess-node-map")).not.toBeInTheDocument();

    // Click on System Metrics tab
    fireEvent.click(screen.getByText("System Metrics"));
    expect(screen.getByTestId("system-metrics")).toBeInTheDocument();
    expect(screen.queryByTestId("price-analytics")).not.toBeInTheDocument();
  });

  it("should handle AuctionStarted events", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    // Simulate AuctionStarted event
    const auctionEvent: SystemEvent = {
      type: "AuctionStarted",
      data: { auction_id: 1, total_energy: 100, reserve_price: 15 },
      timestamp: "2024-01-01T10:00:00Z",
    };

    // We need to trigger the onMessage callback that was passed to useWebSocket
    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(auctionEvent);
    }

    // The auction should be added to the state
    expect(screen.getByText("Auctions: 1")).toBeInTheDocument();
  });

  it("should handle BidPlaced events", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    // First add an auction
    const auctionEvent: SystemEvent = {
      type: "AuctionStarted",
      data: { auction_id: 1, total_energy: 100, reserve_price: 15 },
      timestamp: "2024-01-01T10:00:00Z",
    };

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(auctionEvent);
    }

    // Then add a bid
    const bidEvent: SystemEvent = {
      type: "BidPlaced",
      data: {
        auction_id: 1,
        aggregator_id: 100,
        bess_id: 1,
        bid_price: 18,
        energy_amount: 50,
      },
      timestamp: "2024-01-01T10:01:00Z",
    };

    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(bidEvent);
    }

    // The price history should be updated
    expect(screen.getByText("Price History: 1")).toBeInTheDocument();
  });

  it("should handle BESSNodeStatus events", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const bessEvent: SystemEvent = {
      type: "BESSNodeStatus",
      data: {
        device_id: 1,
        energy_available: 80,
        battery_health: 0.9,
        is_online: true,
      },
      timestamp: "2024-01-01T10:00:00Z",
    };

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(bessEvent);
    }

    // Switch to BESS tab to see the node
    fireEvent.click(screen.getByText("BESS Nodes"));
    expect(screen.getByText("BESS Nodes: 1")).toBeInTheDocument();
  });

  it("should handle AggregatorStatus events", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const aggregatorEvent: SystemEvent = {
      type: "AggregatorStatus",
      data: {
        device_id: 100,
        strategy: "Intelligent",
        success_rate: 0.85,
        total_bids: 25,
      },
      timestamp: "2024-01-01T10:00:00Z",
    };

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(aggregatorEvent);
    }

    // Switch to BESS tab to see the aggregator
    fireEvent.click(screen.getByText("BESS Nodes"));
    expect(screen.getByText("Aggregators: 1")).toBeInTheDocument();
  });

  it("should handle SystemMetrics events", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const metricsEvent: SystemEvent = {
      type: "SystemMetrics",
      data: {
        total_auctions: 10,
        total_bids: 50,
        avg_price_improvement_percent: 15.5,
        active_bess_nodes: 3,
        active_aggregators: 2,
      },
      timestamp: "2024-01-01T10:00:00Z",
    };

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(metricsEvent);
    }

    // Switch to System Metrics tab to see the metrics
    fireEvent.click(screen.getByText("System Metrics"));
    expect(screen.getByText("Metrics: Present")).toBeInTheDocument();
  });

  it("should handle unknown event types", () => {
    const consoleSpy = jest.spyOn(console, "log").mockImplementation();
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const unknownEvent = {
      type: "UnknownEvent" as any,
      data: { test: "data" } as any,
      timestamp: "2024-01-01T10:00:00Z",
    };

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      useWebSocketCall.onMessage(unknownEvent);
    }

    expect(consoleSpy).toHaveBeenCalledWith(
      "Unknown event type:",
      "UnknownEvent"
    );
    consoleSpy.mockRestore();
  });

  it("should limit auctions to last 10", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      // Add 12 auctions
      for (let i = 1; i <= 12; i++) {
        useWebSocketCall.onMessage({
          type: "AuctionStarted" as const,
          data: { auction_id: i, total_energy: 100, reserve_price: 15 },
          timestamp: "2024-01-01T10:00:00Z",
        });
      }
    }

    // Should only show 10 auctions
    expect(screen.getByText("Auctions: 10")).toBeInTheDocument();
  });

  it("should limit price history to last 100 points", () => {
    const mockOnMessage = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    if (useWebSocketCall.onMessage) {
      // Add 102 bid events
      for (let i = 1; i <= 102; i++) {
        useWebSocketCall.onMessage({
          type: "BidPlaced" as const,
          data: {
            auction_id: 1,
            aggregator_id: 100,
            bess_id: 1,
            bid_price: 18,
            energy_amount: 50,
          },
          timestamp: "2024-01-01T10:00:00Z",
        });
      }
    }

    // Switch to analytics tab
    fireEvent.click(screen.getByText("Price Analytics"));
    expect(screen.getByText("Price History: 100")).toBeInTheDocument();
  });

  it("should handle WebSocket errors", () => {
    const mockOnError = jest.fn();
    mockUseWebSocket.mockReturnValue({
      connection: mockConnection,
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    render(<Dashboard />);

    // The onError callback should be called
    const useWebSocketCall = mockUseWebSocket.mock.calls[0][0];
    expect(useWebSocketCall.onError).toBeDefined();
  });

  it("should use correct WebSocket URL", () => {
    render(<Dashboard />);

    expect(mockUseWebSocket).toHaveBeenCalledWith(
      expect.objectContaining({
        url: "ws://localhost:8080/ws",
      })
    );
  });

  it("should handle connection state changes", () => {
    const { rerender } = render(<Dashboard />);

    // Test disconnected state
    mockUseWebSocket.mockReturnValue({
      connection: {
        ...mockConnection,
        isConnected: false,
        error: "Connection failed",
      },
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    rerender(<Dashboard />);

    expect(screen.getByText("Connected: No")).toBeInTheDocument();
    expect(screen.getByText("Error: Connection failed")).toBeInTheDocument();
  });
});
