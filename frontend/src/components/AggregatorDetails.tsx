import React from "react";
import { AggregatorNode } from "../types/energy-trading";

interface AggregatorDetailsProps {
  aggregator: AggregatorNode | null;
  onClose: () => void;
}

export const AggregatorDetails: React.FC<AggregatorDetailsProps> = ({
  aggregator,
  onClose,
}) => {
  if (!aggregator) return null;

  const getStrategyColor = (strategy: string) => {
    switch (strategy.toLowerCase()) {
      case "random":
        return "text-purple-600 bg-purple-100";
      case "conservative":
        return "text-blue-600 bg-blue-100";
      case "aggressive":
        return "text-red-600 bg-red-100";
      case "intelligent":
        return "text-green-600 bg-green-100";
      default:
        return "text-gray-600 bg-gray-100";
    }
  };

  const getSuccessRateColor = (rate: number) => {
    if (rate >= 80) return "text-green-600";
    if (rate >= 60) return "text-yellow-600";
    return "text-red-600";
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center space-x-3">
            <div className="w-12 h-12 bg-accent/10 rounded-lg flex items-center justify-center">
              <span className="text-2xl">⚡</span>
            </div>
            <div>
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
                {aggregator.name}
              </h2>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Device ID: {aggregator.device_id}
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* Strategy & Performance */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Strategy & Performance
            </h3>
            <div className="space-y-4">
              {/* Strategy */}
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Bidding Strategy
                </span>
                <div
                  className={`px-3 py-1 rounded-full text-sm font-medium ${getStrategyColor(
                    aggregator.strategy
                  )}`}
                >
                  {aggregator.strategy}
                </div>
              </div>

              {/* Success Rate */}
              <div>
                <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-2">
                  <span>Success Rate</span>
                  <span
                    className={getSuccessRateColor(aggregator.success_rate)}
                  >
                    {aggregator.success_rate.toFixed(1)}%
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-3">
                  <div
                    className={`h-3 rounded-full transition-all duration-300 ${
                      aggregator.success_rate >= 80
                        ? "bg-green-500"
                        : aggregator.success_rate >= 60
                        ? "bg-yellow-500"
                        : "bg-red-500"
                    }`}
                    style={{ width: `${aggregator.success_rate}%` }}
                  ></div>
                </div>
              </div>
            </div>
          </div>

          {/* Trading Statistics */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Trading Statistics
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="bg-white dark:bg-gray-600 rounded-lg p-3">
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Total Bids
                </div>
                <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                  {aggregator.total_bids}
                </div>
              </div>
              <div className="bg-white dark:bg-gray-600 rounded-lg p-3">
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Average Bid Price
                </div>
                <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                  {aggregator.average_bid_price &&
                  !isNaN(aggregator.average_bid_price)
                    ? `${aggregator.average_bid_price.toFixed(1)}¢/kWh`
                    : "N/A"}
                </div>
              </div>
            </div>
          </div>

          {/* Strategy Description */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Strategy Description
            </h3>
            <div className="text-sm text-gray-600 dark:text-gray-400">
              {aggregator.strategy === "Random" && (
                <p>
                  Uses random bid prices within the acceptable range. Good for
                  testing and baseline performance.
                </p>
              )}
              {aggregator.strategy === "Conservative" && (
                <p>
                  Bids slightly above reserve price to ensure acceptance. Lower
                  risk, moderate returns.
                </p>
              )}
              {aggregator.strategy === "Aggressive" && (
                <p>
                  Bids close to maximum price to maximize profit. Higher risk,
                  higher potential returns.
                </p>
              )}
              {aggregator.strategy === "Intelligent" && (
                <p>
                  Uses historical data and machine learning to predict optimal
                  bid prices. Adaptive and data-driven.
                </p>
              )}
            </div>
          </div>

          {/* Performance Metrics */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Performance Metrics
            </h3>
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Bid Success Rate
                </span>
                <span
                  className={`font-semibold ${getSuccessRateColor(
                    aggregator.success_rate
                  )}`}
                >
                  {aggregator.success_rate.toFixed(1)}%
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Total Bids Placed
                </span>
                <span className="font-semibold text-gray-900 dark:text-gray-100">
                  {aggregator.total_bids}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Successful Bids
                </span>
                <span className="font-semibold text-gray-900 dark:text-gray-100">
                  {aggregator.successful_bids || 0}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Total Energy Bought
                </span>
                <span className="font-semibold text-gray-900 dark:text-gray-100">
                  {aggregator.total_energy_bought
                    ? `${aggregator.total_energy_bought.toFixed(1)} kWh`
                    : "0.0 kWh"}
                </span>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  Average Bid Price
                </span>
                <span className="font-semibold text-gray-900 dark:text-gray-100">
                  {aggregator.average_bid_price &&
                  !isNaN(aggregator.average_bid_price)
                    ? `${aggregator.average_bid_price.toFixed(1)}¢/kWh`
                    : "N/A"}
                </span>
              </div>
            </div>
          </div>

          {/* Status */}
          <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className="flex items-center space-x-2">
              <div
                className={`w-3 h-3 rounded-full ${
                  aggregator.is_online ? "bg-green-500" : "bg-red-500"
                }`}
              ></div>
              <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                {aggregator.is_online ? "Online" : "Offline"}
              </span>
            </div>
            <div className="text-sm text-gray-500 dark:text-gray-400">
              Last updated:{" "}
              {new Date(aggregator.last_updated).toLocaleTimeString()}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
