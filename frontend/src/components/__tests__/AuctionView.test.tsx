import { render, screen } from "@testing-library/react";
import { AuctionView } from "../AuctionView";
import {
  AuctionData,
  BESSNode,
  AggregatorNode,
} from "../../types/energy-trading";

const mockBessNodes: BESSNode[] = [
  {
    device_id: 1,
    name: "BESS-1",
    capacity: 100,
    current_energy_level: 80,
    reserve_price: 15,
    percentage_for_sale: 50,
    battery_voltage: 400,
    max_discharge_rate: 10,
    is_online: true,
    last_updated: "2024-01-01T10:00:00Z",
  },
  {
    device_id: 2,
    name: "BESS-2",
    capacity: 80,
    current_energy_level: 60,
    reserve_price: 16,
    percentage_for_sale: 40,
    battery_voltage: 380,
    max_discharge_rate: 8,
    is_online: true,
    last_updated: "2024-01-01T10:00:00Z",
  },
];

const mockAggregators: AggregatorNode[] = [
  {
    device_id: 100,
    name: "AGG-100",
    strategy: "Intelligent",
    is_online: true,
    success_rate: 0.85,
    total_bids: 25,
    average_bid_price: 18.5,
    last_updated: "2024-01-01T10:00:00Z",
  },
  {
    device_id: 101,
    name: "AGG-101",
    strategy: "Aggressive",
    is_online: true,
    success_rate: 0.75,
    total_bids: 30,
    average_bid_price: 20.0,
    last_updated: "2024-01-01T10:00:00Z",
  },
];

const mockAuctions: AuctionData[] = [
  {
    id: 1,
    start_time: "2024-01-01T10:00:00Z",
    total_energy: 100,
    reserve_price: 15,
    current_highest_bid: 18,
    current_lowest_bid: 16,
    total_bids: 5,
    status: "active",
    bess_nodes: mockBessNodes,
    aggregators: mockAggregators,
  },
  {
    id: 2,
    start_time: "2024-01-01T09:00:00Z",
    total_energy: 80,
    reserve_price: 14,
    current_highest_bid: 17,
    current_lowest_bid: 15,
    total_bids: 3,
    status: "completed",
    bess_nodes: mockBessNodes,
    aggregators: mockAggregators,
  },
];

describe("AuctionView", () => {
  it("should render with empty state when no auctions", () => {
    render(<AuctionView auctions={[]} bessNodes={[]} aggregators={[]} />);

    expect(
      screen.getByText("No auctions available. Waiting for auction events...")
    ).toBeInTheDocument();
  });

  it("should display auction statistics", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("2")).toBeInTheDocument(); // Total Auctions
    expect(screen.getByText("1")).toBeInTheDocument(); // Active Auctions
    expect(screen.getByText("8")).toBeInTheDocument(); // Total Bids
    expect(screen.getByText("2")).toBeInTheDocument(); // BESS Nodes
  });

  it("should display auction details", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("Auction #1")).toBeInTheDocument();
    expect(screen.getByText("Auction #2")).toBeInTheDocument();
    expect(screen.getByText("ACTIVE")).toBeInTheDocument();
    expect(screen.getByText("COMPLETED")).toBeInTheDocument();
  });

  it("should display auction information correctly", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("100.0 kWh")).toBeInTheDocument(); // Total Energy
    expect(screen.getByText("$15.00")).toBeInTheDocument(); // Reserve Price
    expect(screen.getByText("5")).toBeInTheDocument(); // Total Bids
  });

  it("should display price improvement calculation", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // Price improvement should be 20% for auction 1: (18-15)/15 * 100 = 20%
    expect(screen.getByText("Price Improvement: +20.0%")).toBeInTheDocument();
  });

  it("should display BESS nodes information", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("BESS Nodes (2)")).toBeInTheDocument();
    expect(screen.getByText("BESS-1")).toBeInTheDocument();
    expect(screen.getByText("BESS-2")).toBeInTheDocument();
    expect(screen.getByText("80.0 kWh @ $15.00")).toBeInTheDocument();
  });

  it("should display aggregators information", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("Aggregators (2)")).toBeInTheDocument();
    expect(screen.getByText("AGG-100")).toBeInTheDocument();
    expect(screen.getByText("AGG-101")).toBeInTheDocument();
    expect(screen.getByText("Intelligent (85.0%)")).toBeInTheDocument();
    expect(screen.getByText("Aggressive (75.0%)")).toBeInTheDocument();
  });

  it("should handle auctions with no price improvement", () => {
    const auctionWithNoImprovement: AuctionData = {
      ...mockAuctions[0],
      current_highest_bid: 15, // Same as reserve price
    };

    render(
      <AuctionView
        auctions={[auctionWithNoImprovement]}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("Price Improvement: +0.0%")).toBeInTheDocument();
  });

  it("should display correct status colors", () => {
    const { container } = render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    const activeStatus = container.querySelector(
      ".bg-green-100.text-green-800"
    );
    const completedStatus = container.querySelector(
      ".bg-blue-100.text-blue-800"
    );

    expect(activeStatus).toBeInTheDocument();
    expect(completedStatus).toBeInTheDocument();
  });

  it("should limit displayed BESS nodes and aggregators", () => {
    const manyBessNodes = Array.from({ length: 5 }, (_, i) => ({
      ...mockBessNodes[0],
      device_id: i + 1,
      name: `BESS-${i + 1}`,
    }));

    const manyAggregators = Array.from({ length: 5 }, (_, i) => ({
      ...mockAggregators[0],
      device_id: i + 100,
      name: `AGG-${i + 100}`,
    }));

    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={manyBessNodes}
        aggregators={manyAggregators}
      />
    );

    expect(screen.getByText("+2 more nodes")).toBeInTheDocument();
    expect(screen.getByText("+2 more aggregators")).toBeInTheDocument();
  });

  it("should format time correctly", () => {
    render(
      <AuctionView
        auctions={mockAuctions}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // The time should be formatted and displayed
    expect(screen.getByText(/10:00:00/)).toBeInTheDocument();
  });
});
