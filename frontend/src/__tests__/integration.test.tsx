import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { Dashboard } from "../components/Dashboard";
import { useWebSocket } from "../hooks/useWebSocket";

// Mock the useWebSocket hook
jest.mock("../hooks/useWebSocket");
const mockUseWebSocket = useWebSocket as jest.MockedFunction<
  typeof useWebSocket
>;

// Mock the child components with more detailed implementations
jest.mock("../components/AuctionView", () => ({
  AuctionView: ({ auctions, bessNodes, aggregators }: any) => (
    <div data-testid="auction-view">
      <h2>Live Auctions</h2>
      <div data-testid="auction-count">Auctions: {auctions.length}</div>
      <div data-testid="bess-count">BESS Nodes: {bessNodes.length}</div>
      <div data-testid="aggregator-count">
        Aggregators: {aggregators.length}
      </div>
      {auctions.map((auction: any) => (
        <div key={auction.id} data-testid={`auction-${auction.id}`}>
          <div>ID: {auction.id}</div>
          <div>Status: {auction.status}</div>
          <div>Total Bids: {auction.total_bids}</div>
          <div>Highest Bid: ${auction.current_highest_bid}</div>
        </div>
      ))}
    </div>
  ),
}));

jest.mock("../components/BESSNodeMap", () => ({
  BESSNodeMap: ({ bessNodes, aggregators }: any) => (
    <div data-testid="bess-node-map">
      <h2>BESS Node Map</h2>
      <div data-testid="bess-count">BESS Nodes: {bessNodes.length}</div>
      <div data-testid="aggregator-count">
        Aggregators: {aggregators.length}
      </div>
      {bessNodes.map((node: any) => (
        <div key={node.device_id} data-testid={`bess-node-${node.device_id}`}>
          <div>Name: {node.name}</div>
          <div>Energy: {node.current_energy_level} kWh</div>
          <div>Online: {node.is_online ? "Yes" : "No"}</div>
        </div>
      ))}
    </div>
  ),
}));

jest.mock("../components/PriceAnalytics", () => ({
  PriceAnalytics: ({ priceHistory, auctions }: any) => (
    <div data-testid="price-analytics">
      <h2>Price Analytics</h2>
      <div data-testid="price-history-count">
        Price History: {priceHistory.length}
      </div>
      <div data-testid="auction-count">Auctions: {auctions.length}</div>
      {priceHistory.map((price: any, index: number) => (
        <div key={index} data-testid={`price-point-${index}`}>
          <div>Price: ${price.price}</div>
          <div>Energy: {price.energy_amount} kWh</div>
        </div>
      ))}
    </div>
  ),
}));

jest.mock("../components/SystemMetrics", () => ({
  SystemMetrics: ({ metrics, bessNodes, aggregators }: any) => (
    <div data-testid="system-metrics">
      <h2>System Metrics</h2>
      <div data-testid="metrics-present">
        Metrics: {metrics ? "Present" : "None"}
      </div>
      <div data-testid="bess-count">BESS Nodes: {bessNodes.length}</div>
      <div data-testid="aggregator-count">
        Aggregators: {aggregators.length}
      </div>
      {metrics && (
        <div data-testid="metrics-details">
          <div>Total Auctions: {metrics.total_auctions}</div>
          <div>Total Bids: {metrics.total_bids}</div>
          <div>Price Improvement: {metrics.avg_price_improvement_percent}%</div>
        </div>
      )}
    </div>
  ),
}));

jest.mock("../components/ConnectionStatus", () => ({
  ConnectionStatus: ({ connection }: any) => (
    <div data-testid="connection-status">
      <div>Connected: {connection.isConnected ? "Yes" : "No"}</div>
      <div>Error: {connection.error || "None"}</div>
      <div>Reconnect Attempts: {connection.reconnectAttempts}</div>
    </div>
  ),
}));

describe("Dashboard Integration Tests", () => {
  const mockConnection = {
    isConnected: true,
    lastMessage: null,
    error: null,
    reconnectAttempts: 0,
  };

  const mockSendMessage = jest.fn();
  let mockOnMessage: ((event: any) => void) | undefined;

  beforeEach(() => {
    mockUseWebSocket.mockImplementation((options) => {
      mockOnMessage = options.onMessage;
      return {
        connection: mockConnection,
        sendMessage: mockSendMessage,
        connect: jest.fn(),
        disconnect: jest.fn(),
      };
    });
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  it("should handle complete energy trading flow", async () => {
    render(<Dashboard />);

    // 1. Start with empty state
    expect(screen.getByTestId("auction-count")).toHaveTextContent(
      "Auctions: 0"
    );
    expect(screen.getByTestId("bess-count")).toHaveTextContent("BESS Nodes: 0");
    expect(screen.getByTestId("aggregator-count")).toHaveTextContent(
      "Aggregators: 0"
    );

    // 2. Register BESS nodes
    if (mockOnMessage) {
      mockOnMessage({
        type: "BESSNodeStatus",
        data: {
          device_id: 1,
          energy_available: 80,
          battery_health: 0.9,
          is_online: true,
        },
        timestamp: "2024-01-01T10:00:00Z",
      });

      mockOnMessage({
        type: "BESSNodeStatus",
        data: {
          device_id: 2,
          energy_available: 60,
          battery_health: 0.8,
          is_online: true,
        },
        timestamp: "2024-01-01T10:00:00Z",
      });
    }

    // 3. Register aggregators
    if (mockOnMessage) {
      mockOnMessage({
        type: "AggregatorStatus",
        data: {
          device_id: 100,
          strategy: "Intelligent",
          success_rate: 0.85,
          total_bids: 0,
        },
        timestamp: "2024-01-01T10:00:00Z",
      });

      mockOnMessage({
        type: "AggregatorStatus",
        data: {
          device_id: 101,
          strategy: "Aggressive",
          success_rate: 0.75,
          total_bids: 0,
        },
        timestamp: "2024-01-01T10:00:00Z",
      });
    }

    // 4. Start an auction
    if (mockOnMessage) {
      mockOnMessage({
        type: "AuctionStarted",
        data: { auction_id: 1, total_energy: 100, reserve_price: 15 },
        timestamp: "2024-01-01T10:00:00Z",
      });
    }

    await waitFor(() => {
      expect(screen.getByTestId("auction-count")).toHaveTextContent(
        "Auctions: 1"
      );
    });

    // 5. Place bids
    if (mockOnMessage) {
      mockOnMessage({
        type: "BidPlaced",
        data: {
          auction_id: 1,
          aggregator_id: 100,
          bess_id: 1,
          bid_price: 16,
          energy_amount: 50,
        },
        timestamp: "2024-01-01T10:01:00Z",
      });

      mockOnMessage({
        type: "BidPlaced",
        data: {
          auction_id: 1,
          aggregator_id: 101,
          bess_id: 2,
          bid_price: 17,
          energy_amount: 40,
        },
        timestamp: "2024-01-01T10:02:00Z",
      });
    }

    // 6. Accept a bid
    if (mockOnMessage) {
      mockOnMessage({
        type: "BidAccepted",
        data: {
          auction_id: 1,
          aggregator_id: 100,
          bess_id: 1,
          final_price: 16,
          energy_amount: 50,
        },
        timestamp: "2024-01-01T10:03:00Z",
      });
    }

    // 7. Update system metrics
    if (mockOnMessage) {
      mockOnMessage({
        type: "SystemMetrics",
        data: {
          total_auctions: 1,
          total_bids: 2,
          avg_price_improvement_percent: 6.7,
          active_bess_nodes: 2,
          active_aggregators: 2,
        },
        timestamp: "2024-01-01T10:04:00Z",
      });
    }

    // Verify auction details
    expect(screen.getByTestId("auction-1")).toBeInTheDocument();
    expect(screen.getByText("ID: 1")).toBeInTheDocument();
    expect(screen.getByText("Status: completed")).toBeInTheDocument();
    expect(screen.getByText("Total Bids: 2")).toBeInTheDocument();
    expect(screen.getByText("Highest Bid: $17")).toBeInTheDocument();

    // Switch to BESS tab and verify nodes
    fireEvent.click(screen.getByText("BESS Nodes"));
    expect(screen.getByTestId("bess-node-1")).toBeInTheDocument();
    expect(screen.getByTestId("bess-node-2")).toBeInTheDocument();
    expect(screen.getByText("Name: BESS-1")).toBeInTheDocument();
    expect(screen.getByText("Energy: 80 kWh")).toBeInTheDocument();
    expect(screen.getByText("Online: Yes")).toBeInTheDocument();

    // Switch to Price Analytics and verify data
    fireEvent.click(screen.getByText("Price Analytics"));
    expect(screen.getByTestId("price-history-count")).toHaveTextContent(
      "Price History: 2"
    );
    expect(screen.getByTestId("price-point-0")).toBeInTheDocument();
    expect(screen.getByText("Price: $17")).toBeInTheDocument();
    expect(screen.getByText("Energy: 40 kWh")).toBeInTheDocument();

    // Switch to System Metrics and verify metrics
    fireEvent.click(screen.getByText("System Metrics"));
    expect(screen.getByTestId("metrics-present")).toHaveTextContent(
      "Metrics: Present"
    );
    expect(screen.getByText("Total Auctions: 1")).toBeInTheDocument();
    expect(screen.getByText("Total Bids: 2")).toBeInTheDocument();
    expect(screen.getByText("Price Improvement: 6.7%")).toBeInTheDocument();
  });

  it("should handle multiple auctions and maintain state correctly", async () => {
    render(<Dashboard />);

    // Start multiple auctions
    if (mockOnMessage) {
      for (let i = 1; i <= 5; i++) {
        mockOnMessage({
          type: "AuctionStarted",
          data: { auction_id: i, total_energy: 100, reserve_price: 15 },
          timestamp: `2024-01-01T10:0${i}:00Z`,
        });
      }
    }

    await waitFor(() => {
      expect(screen.getByTestId("auction-count")).toHaveTextContent(
        "Auctions: 5"
      );
    });

    // Verify all auctions are displayed
    for (let i = 1; i <= 5; i++) {
      expect(screen.getByTestId(`auction-${i}`)).toBeInTheDocument();
    }
  });

  it("should handle connection errors and reconnection", async () => {
    const { rerender } = render(<Dashboard />);

    // Simulate connection error
    mockUseWebSocket.mockReturnValue({
      connection: {
        isConnected: false,
        lastMessage: null,
        error: "Connection failed",
        reconnectAttempts: 1,
      },
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    rerender(<Dashboard />);

    expect(screen.getByText("Connected: No")).toBeInTheDocument();
    expect(screen.getByText("Error: Connection failed")).toBeInTheDocument();
    expect(screen.getByText("Reconnect Attempts: 1")).toBeInTheDocument();

    // Simulate successful reconnection
    mockUseWebSocket.mockReturnValue({
      connection: {
        isConnected: true,
        lastMessage: null,
        error: null,
        reconnectAttempts: 0,
      },
      sendMessage: mockSendMessage,
      connect: jest.fn(),
      disconnect: jest.fn(),
    });

    rerender(<Dashboard />);

    expect(screen.getByText("Connected: Yes")).toBeInTheDocument();
    expect(screen.getByText("Error: None")).toBeInTheDocument();
  });

  it("should handle rapid event sequences", async () => {
    render(<Dashboard />);

    // Send many events rapidly
    if (mockOnMessage) {
      for (let i = 1; i <= 20; i++) {
        mockOnMessage({
          type: "BidPlaced",
          data: {
            auction_id: 1,
            aggregator_id: 100,
            bess_id: 1,
            bid_price: 15 + i,
            energy_amount: 50,
          },
          timestamp: `2024-01-01T10:0${i}:00Z`,
        });
      }
    }

    await waitFor(() => {
      expect(screen.getByTestId("price-history-count")).toHaveTextContent(
        "Price History: 20"
      );
    });

    // Switch to analytics tab to verify
    fireEvent.click(screen.getByText("Price Analytics"));
    expect(screen.getByTestId("price-point-0")).toBeInTheDocument();
    expect(screen.getByText("Price: $35")).toBeInTheDocument(); // Last bid price
  });

  it("should maintain tab state during rapid updates", async () => {
    render(<Dashboard />);

    // Start on auctions tab
    expect(screen.getByTestId("auction-view")).toBeInTheDocument();

    // Switch to BESS tab
    fireEvent.click(screen.getByText("BESS Nodes"));
    expect(screen.getByTestId("bess-node-map")).toBeInTheDocument();

    // Send events while on BESS tab
    if (mockOnMessage) {
      mockOnMessage({
        type: "AuctionStarted",
        data: { auction_id: 1, total_energy: 100, reserve_price: 15 },
        timestamp: "2024-01-01T10:00:00Z",
      });

      mockOnMessage({
        type: "BidPlaced",
        data: {
          auction_id: 1,
          aggregator_id: 100,
          bess_id: 1,
          bid_price: 16,
          energy_amount: 50,
        },
        timestamp: "2024-01-01T10:01:00Z",
      });
    }

    // Should still be on BESS tab
    expect(screen.getByTestId("bess-node-map")).toBeInTheDocument();
    expect(screen.queryByTestId("auction-view")).not.toBeInTheDocument();

    // Switch back to auctions tab
    fireEvent.click(screen.getByText("Live Auctions"));
    expect(screen.getByTestId("auction-view")).toBeInTheDocument();
    expect(screen.getByTestId("auction-count")).toHaveTextContent(
      "Auctions: 1"
    );
  });

  it("should handle malformed events gracefully", async () => {
    const consoleSpy = jest.spyOn(console, "log").mockImplementation();
    render(<Dashboard />);

    // Send malformed event
    if (mockOnMessage) {
      mockOnMessage({
        type: "UnknownEvent",
        data: { invalid: "data" },
        timestamp: "2024-01-01T10:00:00Z",
      });
    }

    expect(consoleSpy).toHaveBeenCalledWith(
      "Unknown event type:",
      "UnknownEvent"
    );
    consoleSpy.mockRestore();
  });
});
