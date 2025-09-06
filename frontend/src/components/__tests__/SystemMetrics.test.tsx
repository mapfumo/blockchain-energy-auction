import { render, screen } from "@testing-library/react";
import { SystemMetrics } from "../SystemMetrics";
import {
  SystemMetrics as SystemMetricsType,
  BESSNode,
  AggregatorNode,
} from "../../types/energy-trading";

const mockSystemMetrics: SystemMetricsType = {
  total_events_broadcast: 1250,
  connected_clients: 5,
  average_events_per_second: 12.5,
  total_auctions: 15,
  total_bids: 45,
  avg_price_improvement_percent: 18.5,
  active_bess_nodes: 3,
  active_aggregators: 2,
};

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
    current_energy_level: 20,
    reserve_price: 16,
    percentage_for_sale: 40,
    battery_voltage: 350,
    max_discharge_rate: 8,
    is_online: false,
    last_updated: "2024-01-01T09:00:00Z",
  },
  {
    device_id: 3,
    name: "BESS-3",
    capacity: 120,
    current_energy_level: 60,
    reserve_price: 14,
    percentage_for_sale: 60,
    battery_voltage: 420,
    max_discharge_rate: 12,
    is_online: true,
    last_updated: "2024-01-01T10:30:00Z",
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
    last_updated: "2024-01-01T10:15:00Z",
  },
];

describe("SystemMetrics", () => {
  it("should render with empty state when no metrics", () => {
    render(<SystemMetrics metrics={null} bessNodes={[]} aggregators={[]} />);

    expect(screen.getAllByText("0")).toHaveLength(4); // All metrics show 0
  });

  it("should display system overview metrics", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("1,250")).toBeInTheDocument(); // Events Broadcast
    expect(screen.getByText("5")).toBeInTheDocument(); // Connected Clients
    expect(screen.getByText("12.5")).toBeInTheDocument(); // Events/sec
    expect(screen.getByText("18.5%")).toBeInTheDocument(); // Avg Price Improvement
  });

  it("should display system health metrics", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("2/3")).toBeInTheDocument(); // BESS Nodes Online
    expect(screen.getByText("2/2")).toBeInTheDocument(); // Aggregators Online
    expect(screen.getByText("160.0 kWh")).toBeInTheDocument(); // Total Energy Available
  });

  it("should display performance metrics", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("15")).toBeInTheDocument(); // Total Auctions
    expect(screen.getByText("45")).toBeInTheDocument(); // Total Bids
    expect(screen.getByText("3")).toBeInTheDocument(); // Active BESS Nodes
    expect(screen.getByText("2")).toBeInTheDocument(); // Active Aggregators
  });

  it("should display battery health overview", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("390.0V")).toBeInTheDocument(); // Average Battery Voltage
    expect(screen.getByText("Good")).toBeInTheDocument(); // Battery Health Status
  });

  it("should display battery health distribution", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("BESS-1")).toBeInTheDocument();
    expect(screen.getByText("BESS-2")).toBeInTheDocument();
    expect(screen.getByText("BESS-3")).toBeInTheDocument();
    expect(screen.getByText("400.0V")).toBeInTheDocument();
    expect(screen.getByText("350.0V")).toBeInTheDocument();
    expect(screen.getByText("420.0V")).toBeInTheDocument();
  });

  it("should display recent activity", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("Recent BESS Updates")).toBeInTheDocument();
    expect(screen.getByText("Recent Aggregator Updates")).toBeInTheDocument();
    expect(screen.getByText("BESS-3")).toBeInTheDocument(); // Most recent BESS
    expect(screen.getByText("AGG-101")).toBeInTheDocument(); // Most recent Aggregator
  });

  it("should display correct health status colors", () => {
    const { container } = render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // System health should be "Warning" (2/3 = 66.7% < 80%)
    const warningStatus = container.querySelector(".text-yellow-600");
    expect(warningStatus).toBeInTheDocument();

    // Battery health should be "Good" (390V > 350V)
    const goodStatus = container.querySelector(".text-green-600");
    expect(goodStatus).toBeInTheDocument();
  });

  it("should display online/offline status correctly", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("Online")).toBeInTheDocument();
    expect(screen.getByText("Offline")).toBeInTheDocument();
  });

  it("should calculate average battery voltage correctly", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // Average: (400 + 350 + 420) / 3 = 390V
    expect(screen.getByText("390.0V")).toBeInTheDocument();
  });

  it("should calculate total energy available correctly", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // Total: 80 + 20 + 60 = 160 kWh
    expect(screen.getByText("160.0 kWh")).toBeInTheDocument();
  });

  it("should handle empty BESS nodes and aggregators", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={[]}
        aggregators={[]}
      />
    );

    expect(screen.getByText("0/0")).toBeInTheDocument(); // BESS Nodes Online
    expect(screen.getByText("0/0")).toBeInTheDocument(); // Aggregators Online
    expect(screen.getByText("0.0 kWh")).toBeInTheDocument(); // Total Energy Available
  });

  it("should display battery health status correctly", () => {
    const { container } = render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // BESS-1: 400V (Good)
    const goodVoltages = container.querySelectorAll(".text-green-600");
    expect(goodVoltages.length).toBeGreaterThan(0);

    // BESS-2: 350V (Warning)
    const warningVoltages = container.querySelectorAll(".text-yellow-600");
    expect(warningVoltages.length).toBeGreaterThan(0);
  });

  it("should format numbers correctly", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("1,250")).toBeInTheDocument(); // Formatted number
    expect(screen.getByText("18.5%")).toBeInTheDocument(); // Formatted percentage
  });

  it("should display recent activity in chronological order", () => {
    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={mockBessNodes}
        aggregators={mockAggregators}
      />
    );

    // BESS-3 should be first (most recent: 10:30:00)
    // BESS-1 should be second (10:00:00)
    // BESS-2 should be third (09:00:00)
    const bessUpdates = screen.getByText("Recent BESS Updates").parentElement;
    const bessNames = bessUpdates?.querySelectorAll("span");
    expect(bessNames?.[0]).toHaveTextContent("BESS-3");
  });

  it("should handle zero average battery voltage", () => {
    const emptyBessNodes: BESSNode[] = [];

    render(
      <SystemMetrics
        metrics={mockSystemMetrics}
        bessNodes={emptyBessNodes}
        aggregators={mockAggregators}
      />
    );

    expect(screen.getByText("0.0V")).toBeInTheDocument();
  });
});
