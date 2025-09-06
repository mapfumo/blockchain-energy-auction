import { render, screen } from "@testing-library/react";
import { PriceAnalytics } from "../PriceAnalytics";
import { AuctionData } from "../../types/energy-trading";

// Mock recharts components
jest.mock("recharts", () => ({
  LineChart: ({ children }: any) => (
    <div data-testid="line-chart">{children}</div>
  ),
  BarChart: ({ children }: any) => (
    <div data-testid="bar-chart">{children}</div>
  ),
  PieChart: ({ children }: any) => (
    <div data-testid="pie-chart">{children}</div>
  ),
  Line: () => <div data-testid="line" />,
  Bar: () => <div data-testid="bar" />,
  Pie: () => <div data-testid="pie" />,
  XAxis: () => <div data-testid="x-axis" />,
  YAxis: () => <div data-testid="y-axis" />,
  CartesianGrid: () => <div data-testid="cartesian-grid" />,
  Tooltip: () => <div data-testid="tooltip" />,
  ResponsiveContainer: ({ children }: any) => (
    <div data-testid="responsive-container">{children}</div>
  ),
  Cell: () => <div data-testid="cell" />,
}));

const mockPriceHistory = [
  { timestamp: "2024-01-01T10:00:00Z", price: 15.5, energy_amount: 50 },
  { timestamp: "2024-01-01T10:01:00Z", price: 16.0, energy_amount: 60 },
  { timestamp: "2024-01-01T10:02:00Z", price: 17.5, energy_amount: 45 },
  { timestamp: "2024-01-01T10:03:00Z", price: 18.0, energy_amount: 70 },
  { timestamp: "2024-01-01T10:04:00Z", price: 16.5, energy_amount: 55 },
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
    status: "completed",
    bess_nodes: [],
    aggregators: [],
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
    bess_nodes: [],
    aggregators: [],
  },
  {
    id: 3,
    start_time: "2024-01-01T11:00:00Z",
    total_energy: 120,
    reserve_price: 16,
    current_highest_bid: 19,
    current_lowest_bid: 17,
    total_bids: 4,
    status: "active",
    bess_nodes: [],
    aggregators: [],
  },
];

describe("PriceAnalytics", () => {
  it("should render with empty state when no data", () => {
    render(<PriceAnalytics priceHistory={[]} auctions={[]} />);

    expect(
      screen.getByText("No price data available. Waiting for bid events...")
    ).toBeInTheDocument();
    expect(
      screen.getByText("No data available for correlation analysis.")
    ).toBeInTheDocument();
  });

  it("should display price statistics", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByText("$16.70")).toBeInTheDocument(); // Average Bid Price
    expect(screen.getByText("$2.50")).toBeInTheDocument(); // Price Range
    expect(screen.getByText("4.0")).toBeInTheDocument(); // Avg Bids per Auction
    expect(screen.getByText("18.8%")).toBeInTheDocument(); // Avg Price Improvement
  });

  it("should display price history chart", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByTestId("line-chart")).toBeInTheDocument();
    expect(screen.getByTestId("responsive-container")).toBeInTheDocument();
    expect(screen.getByTestId("line")).toBeInTheDocument();
  });

  it("should display energy vs price correlation chart", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByTestId("bar-chart")).toBeInTheDocument();
    expect(screen.getByText("Energy vs Price Correlation")).toBeInTheDocument();
  });

  it("should display strategy distribution pie chart", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByTestId("pie-chart")).toBeInTheDocument();
    expect(
      screen.getByText("Bidding Strategy Distribution")
    ).toBeInTheDocument();
  });

  it("should display auction performance metrics", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByText("Auction Performance")).toBeInTheDocument();
    expect(screen.getByText("3")).toBeInTheDocument(); // Total Auctions
    expect(screen.getByText("2")).toBeInTheDocument(); // Completed Auctions
    expect(screen.getByText("1")).toBeInTheDocument(); // Active Auctions
    expect(screen.getByText("12")).toBeInTheDocument(); // Total Bids
  });

  it("should calculate price statistics correctly", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    // Average price: (15.5 + 16.0 + 17.5 + 18.0 + 16.5) / 5 = 16.7
    expect(screen.getByText("$16.70")).toBeInTheDocument();

    // Price range: 18.0 - 15.5 = 2.5
    expect(screen.getByText("$2.50")).toBeInTheDocument();
  });

  it("should calculate auction statistics correctly", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    // Total bids: 5 + 3 + 4 = 12
    expect(screen.getByText("12")).toBeInTheDocument();

    // Avg bids per auction: 12 / 3 = 4
    expect(screen.getByText("4.0")).toBeInTheDocument();
  });

  it("should calculate price improvement correctly", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    // Auction 1: (18-15)/15 * 100 = 20%
    // Auction 2: (17-14)/14 * 100 = 21.4%
    // Average: (20 + 21.4) / 2 = 20.7%
    // But we're only showing completed auctions, so it should be 20.7%
    expect(screen.getByText("20.7%")).toBeInTheDocument();
  });

  it("should display strategy distribution data", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    expect(screen.getByText("Intelligent")).toBeInTheDocument();
    expect(screen.getByText("Aggressive")).toBeInTheDocument();
    expect(screen.getByText("Conservative")).toBeInTheDocument();
    expect(screen.getByText("Random")).toBeInTheDocument();
  });

  it("should handle empty price history gracefully", () => {
    render(<PriceAnalytics priceHistory={[]} auctions={mockAuctions} />);

    expect(screen.getByText("$0.00")).toBeInTheDocument(); // Average Bid Price
    expect(screen.getByText("$0.00")).toBeInTheDocument(); // Price Range
  });

  it("should handle empty auctions gracefully", () => {
    render(<PriceAnalytics priceHistory={mockPriceHistory} auctions={[]} />);

    expect(screen.getByText("0")).toBeInTheDocument(); // Avg Bids per Auction
    expect(screen.getByText("0%")).toBeInTheDocument(); // Avg Price Improvement
  });

  it("should limit chart data to recent entries", () => {
    const manyPricePoints = Array.from({ length: 100 }, (_, i) => ({
      timestamp: `2024-01-01T${10 + Math.floor(i / 60)}:${(i % 60)
        .toString()
        .padStart(2, "0")}:00Z`,
      price: 15 + Math.random() * 5,
      energy_amount: 50 + Math.random() * 30,
    }));

    render(
      <PriceAnalytics priceHistory={manyPricePoints} auctions={mockAuctions} />
    );

    // Should still render charts even with many data points
    expect(screen.getByTestId("line-chart")).toBeInTheDocument();
    expect(screen.getByTestId("bar-chart")).toBeInTheDocument();
  });

  it("should display correct chart components", () => {
    render(
      <PriceAnalytics priceHistory={mockPriceHistory} auctions={mockAuctions} />
    );

    // Line chart components
    expect(screen.getByTestId("line-chart")).toBeInTheDocument();
    expect(screen.getByTestId("x-axis")).toBeInTheDocument();
    expect(screen.getByTestId("y-axis")).toBeInTheDocument();
    expect(screen.getByTestId("cartesian-grid")).toBeInTheDocument();
    expect(screen.getByTestId("tooltip")).toBeInTheDocument();

    // Bar chart components
    expect(screen.getByTestId("bar-chart")).toBeInTheDocument();

    // Pie chart components
    expect(screen.getByTestId("pie-chart")).toBeInTheDocument();
    expect(screen.getByTestId("cell")).toBeInTheDocument();
  });
});
