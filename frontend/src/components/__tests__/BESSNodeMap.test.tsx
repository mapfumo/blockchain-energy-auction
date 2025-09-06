import { render, screen } from "@testing-library/react";
import { BESSNodeMap } from "../BESSNodeMap";
import { BESSNode, AggregatorNode } from "../../types/energy-trading";

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
    last_updated: "2024-01-01T10:00:00Z",
  },
];

describe("BESSNodeMap", () => {
  it("should render with empty state when no BESS nodes", () => {
    render(<BESSNodeMap bessNodes={[]} aggregators={[]} />);

    expect(
      screen.getByText(
        "No BESS nodes available. Waiting for node status events..."
      )
    ).toBeInTheDocument();
  });

  it("should display summary statistics", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    expect(screen.getByText("3")).toBeInTheDocument(); // Total BESS Nodes
    expect(screen.getByText("2")).toBeInTheDocument(); // Online Nodes
    expect(screen.getByText("160.0 kWh")).toBeInTheDocument(); // Total Available Energy
    expect(screen.getByText("53.3%")).toBeInTheDocument(); // Average Charge Level
  });

  it("should display BESS node details", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    expect(screen.getByText("BESS-1")).toBeInTheDocument();
    expect(screen.getByText("BESS-2")).toBeInTheDocument();
    expect(screen.getByText("BESS-3")).toBeInTheDocument();
  });

  it("should display online/offline status correctly", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    expect(screen.getAllByText("🟢 Online")).toHaveLength(2);
    expect(screen.getByText("🔴 Offline")).toBeInTheDocument();
  });

  it("should display energy level progress bars", () => {
    const { container } = render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // Should have 3 progress bars (one for each BESS node)
    const progressBars = container.querySelectorAll(
      ".bg-gray-200.rounded-full.h-3"
    );
    expect(progressBars).toHaveLength(3);
  });

  it("should display correct energy level colors", () => {
    const { container } = render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // BESS-1: 80% energy (green)
    const greenBars = container.querySelectorAll(".bg-green-500");
    expect(greenBars.length).toBeGreaterThan(0);

    // BESS-2: 25% energy (orange)
    const orangeBars = container.querySelectorAll(".bg-orange-500");
    expect(orangeBars.length).toBeGreaterThan(0);

    // BESS-3: 50% energy (yellow)
    const yellowBars = container.querySelectorAll(".bg-yellow-500");
    expect(yellowBars.length).toBeGreaterThan(0);
  });

  it("should display battery voltage with correct colors", () => {
    const { container } = render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // BESS-1: 400V (green)
    const greenVoltage = container.querySelector(".text-green-600");
    expect(greenVoltage).toBeInTheDocument();

    // BESS-2: 350V (yellow)
    const yellowVoltage = container.querySelector(".text-yellow-600");
    expect(yellowVoltage).toBeInTheDocument();

    // BESS-3: 420V (green)
    expect(container.querySelectorAll(".text-green-600")).toHaveLength(2);
  });

  it("should display available for sale energy correctly", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // BESS-1: 80 * 0.5 = 40 kWh
    expect(screen.getByText("40.0 kWh")).toBeInTheDocument();
    // BESS-2: 20 * 0.4 = 8 kWh
    expect(screen.getByText("8.0 kWh")).toBeInTheDocument();
    // BESS-3: 60 * 0.6 = 36 kWh
    expect(screen.getByText("36.0 kWh")).toBeInTheDocument();
  });

  it("should display aggregator information", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    expect(screen.getByText("Connected Aggregators")).toBeInTheDocument();
    expect(screen.getByText("AGG-100")).toBeInTheDocument();
    expect(screen.getByText("AGG-101")).toBeInTheDocument();
    expect(screen.getByText("Intelligent")).toBeInTheDocument();
    expect(screen.getByText("Aggressive")).toBeInTheDocument();
  });

  it("should display aggregator strategy badges with correct colors", () => {
    const { container } = render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    const intelligentBadge = container.querySelector(
      ".bg-purple-100.text-purple-800"
    );
    const aggressiveBadge = container.querySelector(".bg-red-100.text-red-800");

    expect(intelligentBadge).toBeInTheDocument();
    expect(aggressiveBadge).toBeInTheDocument();
  });

  it("should display aggregator statistics", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    expect(screen.getByText("85.0%")).toBeInTheDocument(); // Success Rate
    expect(screen.getByText("75.0%")).toBeInTheDocument(); // Success Rate
    expect(screen.getByText("25")).toBeInTheDocument(); // Total Bids
    expect(screen.getByText("30")).toBeInTheDocument(); // Total Bids
    expect(screen.getByText("$18.50")).toBeInTheDocument(); // Avg Bid Price
    expect(screen.getByText("$20.00")).toBeInTheDocument(); // Avg Bid Price
  });

  it("should handle empty aggregators list", () => {
    render(<BESSNodeMap bessNodes={mockBessNodes} aggregators={[]} />);

    expect(
      screen.getByText(
        "No aggregators connected. Waiting for aggregator status events..."
      )
    ).toBeInTheDocument();
  });

  it("should format time correctly", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // Should display formatted times
    expect(screen.getByText(/10:00:00/)).toBeInTheDocument();
    expect(screen.getByText(/09:00:00/)).toBeInTheDocument();
    expect(screen.getByText(/10:30:00/)).toBeInTheDocument();
  });

  it("should calculate total energy correctly", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // Total energy: 80 + 20 + 60 = 160 kWh
    expect(screen.getByText("160.0 kWh")).toBeInTheDocument();
  });

  it("should calculate average charge level correctly", () => {
    render(
      <BESSNodeMap bessNodes={mockBessNodes} aggregators={mockAggregators} />
    );

    // Average charge: (80/100 + 20/80 + 60/120) / 3 * 100 = 53.3%
    expect(screen.getByText("53.3%")).toBeInTheDocument();
  });
});
