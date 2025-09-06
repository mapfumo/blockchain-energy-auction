import React, { useMemo } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
} from "recharts";
import { AuctionData } from "../types/energy-trading";

interface PriceAnalyticsProps {
  priceHistory: Array<{
    timestamp: string;
    price: number;
    energy_amount: number;
  }>;
  auctions: AuctionData[];
}

export const PriceAnalytics: React.FC<PriceAnalyticsProps> = ({
  priceHistory,
  auctions,
}) => {
  const formatPrice = (price: number) => `${price.toFixed(1)}¢/kWh`;
  const formatEnergy = (energy: number) => `${energy.toFixed(1)} kWh`;

  // Process price history for charts
  const chartData = useMemo(() => {
    return priceHistory.slice(0, 50).map((point, index) => ({
      time: new Date(point.timestamp).toLocaleTimeString(),
      price: point.price,
      energy: point.energy_amount,
      index: index + 1,
    }));
  }, [priceHistory]);

  // Calculate price statistics
  const priceStats = useMemo(() => {
    if (priceHistory.length === 0) return null;

    const prices = priceHistory.map((p) => p.price);
    const avgPrice =
      prices.reduce((sum, price) => sum + price, 0) / prices.length;
    const minPrice = Math.min(...prices);
    const maxPrice = Math.max(...prices);
    const priceRange = maxPrice - minPrice;

    return { avgPrice, minPrice, maxPrice, priceRange };
  }, [priceHistory]);

  // Calculate auction statistics
  const auctionStats = useMemo(() => {
    if (auctions.length === 0) return null;

    const completedAuctions = auctions.filter((a) => a.status === "completed");
    const totalBids = auctions.reduce((sum, a) => sum + a.total_bids, 0);
    const avgBidsPerAuction = totalBids / auctions.length;
    const avgPriceImprovement =
      completedAuctions.reduce((sum, a) => {
        const improvement =
          ((a.current_highest_bid - a.reserve_price) / a.reserve_price) * 100;
        return sum + improvement;
      }, 0) / completedAuctions.length;

    return { totalBids, avgBidsPerAuction, avgPriceImprovement };
  }, [auctions]);

  // Strategy distribution data
  const strategyData = useMemo(() => {
    const strategies = ["Intelligent", "Aggressive", "Conservative", "Random"];
    return strategies.map((strategy) => ({
      name: strategy,
      value: Math.floor(Math.random() * 20) + 5, // Mock data for now
    }));
  }, []);

  const COLORS = ["#8884d8", "#82ca9d", "#ffc658", "#ff7300"];

  return (
    <div className="space-y-6">
      {/* Summary Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-gray-900">
            {priceStats ? formatPrice(priceStats.avgPrice) : "0.0¢/kWh"}
          </div>
          <div className="text-sm text-gray-500">Average Bid Price</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-green-600">
            {priceStats ? formatPrice(priceStats.priceRange) : "0.0¢/kWh"}
          </div>
          <div className="text-sm text-gray-500">Price Range</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-blue-600">
            {auctionStats ? auctionStats.avgBidsPerAuction.toFixed(1) : "0"}
          </div>
          <div className="text-sm text-gray-500">Avg Bids per Auction</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-purple-600">
            {auctionStats ? auctionStats.avgPriceImprovement.toFixed(1) : "0"}%
          </div>
          <div className="text-sm text-gray-500">Avg Price Improvement</div>
        </div>
      </div>

      {/* Price History Chart */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">Price History</h2>
        </div>
        <div className="p-6">
          {chartData.length === 0 ? (
            <div className="text-center text-gray-500 py-8">
              No price data available. Waiting for bid events...
            </div>
          ) : (
            <div className="h-80">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={chartData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis
                    dataKey="time"
                    tick={{ fontSize: 12 }}
                    angle={-45}
                    textAnchor="end"
                    height={60}
                  />
                  <YAxis
                    tick={{ fontSize: 12 }}
                    tickFormatter={(value) => `$${value.toFixed(0)}`}
                  />
                  <Tooltip
                    formatter={(value: any, name: string) => [
                      name === "price"
                        ? formatPrice(value)
                        : formatEnergy(value),
                      name === "price" ? "Price" : "Energy",
                    ]}
                    labelFormatter={(label) => `Time: ${label}`}
                  />
                  <Line
                    type="monotone"
                    dataKey="price"
                    stroke="#8884d8"
                    strokeWidth={2}
                    dot={{ r: 4 }}
                    activeDot={{ r: 6 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          )}
        </div>
      </div>

      {/* Energy vs Price Scatter */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            Energy vs Price Correlation
          </h2>
        </div>
        <div className="p-6">
          {chartData.length === 0 ? (
            <div className="text-center text-gray-500 py-8">
              No data available for correlation analysis.
            </div>
          ) : (
            <div className="h-80">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={chartData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis
                    dataKey="index"
                    tick={{ fontSize: 12 }}
                    label={{
                      value: "Bid Sequence",
                      position: "insideBottom",
                      offset: -5,
                    }}
                  />
                  <YAxis
                    yAxisId="price"
                    orientation="left"
                    tick={{ fontSize: 12 }}
                    tickFormatter={(value) => `$${value.toFixed(0)}`}
                  />
                  <YAxis
                    yAxisId="energy"
                    orientation="right"
                    tick={{ fontSize: 12 }}
                    tickFormatter={(value) => `${value.toFixed(0)}kWh`}
                  />
                  <Tooltip
                    formatter={(value: any, name: string) => [
                      name === "price"
                        ? formatPrice(value)
                        : formatEnergy(value),
                      name === "price" ? "Price" : "Energy",
                    ]}
                    labelFormatter={(label) => `Bid #${label}`}
                  />
                  <Bar
                    yAxisId="price"
                    dataKey="price"
                    fill="#8884d8"
                    opacity={0.7}
                  />
                  <Bar
                    yAxisId="energy"
                    dataKey="energy"
                    fill="#82ca9d"
                    opacity={0.7}
                  />
                </BarChart>
              </ResponsiveContainer>
            </div>
          )}
        </div>
      </div>

      {/* Strategy Distribution */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white shadow rounded-lg">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">
              Bidding Strategy Distribution
            </h2>
          </div>
          <div className="p-6">
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={strategyData}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ name, percent }) =>
                      `${name} ${(percent * 100).toFixed(0)}%`
                    }
                    outerRadius={80}
                    fill="#8884d8"
                    dataKey="value"
                  >
                    {strategyData.map((entry, index) => (
                      <Cell
                        key={`cell-${index}`}
                        fill={COLORS[index % COLORS.length]}
                      />
                    ))}
                  </Pie>
                  <Tooltip />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Auction Performance */}
        <div className="bg-white shadow rounded-lg">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-medium text-gray-900">
              Auction Performance
            </h2>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">Total Auctions</span>
                <span className="text-lg font-semibold">{auctions.length}</span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">
                  Completed Auctions
                </span>
                <span className="text-lg font-semibold text-green-600">
                  {auctions.filter((a) => a.status === "completed").length}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">Active Auctions</span>
                <span className="text-lg font-semibold text-blue-600">
                  {auctions.filter((a) => a.status === "active").length}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">Total Bids</span>
                <span className="text-lg font-semibold">
                  {auctionStats ? auctionStats.totalBids : 0}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500">
                  Avg Price Improvement
                </span>
                <span className="text-lg font-semibold text-purple-600">
                  {auctionStats
                    ? `${auctionStats.avgPriceImprovement.toFixed(1)}%`
                    : "0%"}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
