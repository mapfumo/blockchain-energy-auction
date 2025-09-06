import React from "react";
import { BESSNode } from "../types/energy-trading";

interface BESSNodeDetailsProps {
  node: BESSNode | null;
  onClose: () => void;
}

export const BESSNodeDetails: React.FC<BESSNodeDetailsProps> = ({
  node,
  onClose,
}) => {
  if (!node) return null;

  const energyPercentage = (node.current_energy_level / node.capacity) * 100;
  const availableForSale =
    node.current_energy_level * (node.percentage_for_sale / 100.0);
  const soldEnergy = node.capacity - node.current_energy_level;

  const getBatteryHealthColor = (health: number) => {
    switch (health) {
      case 0:
        return "text-green-600 bg-green-100";
      case 1:
        return "text-blue-600 bg-blue-100";
      case 2:
        return "text-yellow-600 bg-yellow-100";
      case 3:
        return "text-red-600 bg-red-100";
      default:
        return "text-gray-600 bg-gray-100";
    }
  };

  const getBatteryHealthText = (health: number) => {
    switch (health) {
      case 0:
        return "Excellent";
      case 1:
        return "Good";
      case 2:
        return "Fair";
      case 3:
        return "Poor";
      default:
        return "Unknown";
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center space-x-3">
            <div className="w-12 h-12 bg-secondary/10 rounded-lg flex items-center justify-center">
              <span className="text-2xl">🔋</span>
            </div>
            <div>
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
                {node.name}
              </h2>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Device ID: {node.device_id}
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
          {/* Energy Status */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Energy Status
            </h3>
            <div className="space-y-4">
              {/* Energy Level Bar */}
              <div>
                <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-2">
                  <span>Current Level</span>
                  <span>
                    {node.current_energy_level.toFixed(1)} /{" "}
                    {node.capacity.toFixed(1)} kWh
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-3">
                  <div
                    className="bg-gradient-to-r from-green-400 to-green-600 h-3 rounded-full transition-all duration-300"
                    style={{ width: `${energyPercentage}%` }}
                  ></div>
                </div>
                <div className="text-right text-sm text-gray-500 dark:text-gray-400 mt-1">
                  {energyPercentage.toFixed(1)}% charged
                </div>
              </div>

              {/* Energy Stats Grid */}
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-white dark:bg-gray-600 rounded-lg p-3">
                  <div className="text-sm text-gray-500 dark:text-gray-400">
                    Available for Sale
                  </div>
                  <div className="text-lg font-semibold text-green-600 dark:text-green-400">
                    {availableForSale.toFixed(1)} kWh
                  </div>
                </div>
                <div className="bg-white dark:bg-gray-600 rounded-lg p-3">
                  <div className="text-sm text-gray-500 dark:text-gray-400">
                    Sold Energy
                  </div>
                  <div className="text-lg font-semibold text-blue-600 dark:text-blue-400">
                    {soldEnergy.toFixed(1)} kWh
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Trading Info */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Trading Information
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Reserve Price
                </div>
                <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                  {node.reserve_price.toFixed(1)}¢/kWh
                </div>
              </div>
              <div>
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Available for Sale
                </div>
                <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                  {node.percentage_for_sale.toFixed(0)}%
                </div>
              </div>
              <div>
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Max Discharge Rate
                </div>
                <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                  {node.max_discharge_rate.toFixed(1)} kW
                </div>
              </div>
              <div>
                <div className="text-sm text-gray-500 dark:text-gray-400">
                  Battery Voltage
                </div>
                <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                  {node.battery_voltage.toFixed(0)}V
                  <span className="text-sm text-gray-500 dark:text-gray-400 ml-1">
                    {node.battery_voltage === 12
                      ? "(Portable)"
                      : node.battery_voltage === 24
                      ? "(Off-grid)"
                      : node.battery_voltage === 48
                      ? "(Residential)"
                      : ""}
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* Battery Health */}
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
            <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100 mb-4">
              Battery Health
            </h3>
            <div className="flex items-center space-x-4">
              <div
                className={`px-3 py-1 rounded-full text-sm font-medium ${getBatteryHealthColor(
                  node.battery_health
                )}`}
              >
                {getBatteryHealthText(node.battery_health)}
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">
                Last updated: {new Date(node.last_updated).toLocaleTimeString()}
              </div>
            </div>
          </div>

          {/* Status */}
          <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <div className="flex items-center space-x-2">
              <div
                className={`w-3 h-3 rounded-full ${
                  node.is_online ? "bg-green-500" : "bg-red-500"
                }`}
              ></div>
              <span className="text-sm font-medium text-gray-900 dark:text-gray-100">
                {node.is_online ? "Online" : "Offline"}
              </span>
            </div>
            <div className="text-sm text-gray-500 dark:text-gray-400">
              {node.is_online ? "Connected and ready" : "Disconnected"}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
