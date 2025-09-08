import React from "react";
import { BESSNode, AggregatorNode } from "../types/energy-trading";

interface BESSNodeMapProps {
  bessNodes: BESSNode[];
  aggregators: AggregatorNode[];
}

export const BESSNodeMap: React.FC<BESSNodeMapProps> = ({
  bessNodes,
  aggregators,
}) => {
  const formatPrice = (price: number | undefined) =>
    price !== undefined ? `${(price / 100).toFixed(2)}c/kWh` : "N/A";
  const formatEnergy = (energy: number | undefined) =>
    energy !== undefined ? `${energy.toFixed(1)} kWh` : "N/A";
  const formatVoltage = (voltage: number | undefined) =>
    voltage !== undefined ? `${voltage.toFixed(1)}V` : "N/A";
  const formatTime = (timestamp: string | undefined) =>
    timestamp ? new Date(timestamp).toLocaleTimeString() : "N/A";

  const getEnergyLevelColor = (node: BESSNode) => {
    const percentage = (node.energy_level / node.capacity_kwh) * 100;
    if (percentage >= 80) return "bg-green-500";
    if (percentage >= 50) return "bg-yellow-500";
    if (percentage >= 20) return "bg-orange-500";
    return "bg-red-500";
  };

  const getBatteryHealthColor = (node: BESSNode) => {
    // Simplified health based on energy level
    const percentage = (node.energy_level / node.capacity_kwh) * 100;
    if (percentage >= 80) return "text-green-600";
    if (percentage >= 50) return "text-yellow-600";
    return "text-red-600";
  };

  const getOnlineStatus = (node: BESSNode) => {
    return node.is_online ? "🟢 Online" : "🔴 Offline";
  };

  const totalEnergy = bessNodes.reduce(
    (sum, node) => sum + node.energy_level,
    0
  );
  const totalCapacity = bessNodes.reduce(
    (sum, node) => sum + node.capacity_kwh,
    0
  );
  const onlineNodes = bessNodes.filter((node) => node.is_online).length;

  return (
    <div className="space-y-6">
      {/* Summary Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-gray-900">
            {bessNodes.length}
          </div>
          <div className="text-sm text-gray-500">Total BESS Nodes</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-green-600">{onlineNodes}</div>
          <div className="text-sm text-gray-500">Online Nodes</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-blue-600">
            {formatEnergy(totalEnergy)}
          </div>
          <div className="text-sm text-gray-500">Total Available Energy</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-purple-600">
            {totalCapacity > 0
              ? ((totalEnergy / totalCapacity) * 100).toFixed(1)
              : 0}
            %
          </div>
          <div className="text-sm text-gray-500">Average Charge Level</div>
        </div>
      </div>

      {/* BESS Nodes Grid */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            BESS Node Status
          </h2>
        </div>
        <div className="p-6">
          {bessNodes.length === 0 ? (
            <div className="text-center text-gray-500 py-8">
              No BESS nodes available. Waiting for node status events...
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {bessNodes.map((node) => (
                <div
                  key={node.node_id}
                  className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
                >
                  {/* Node Header */}
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-medium text-gray-900">
                      BESS-{node.node_id}
                    </h3>
                    <span
                      className={`text-sm font-medium ${
                        node.is_online ? "text-green-600" : "text-red-600"
                      }`}
                    >
                      {getOnlineStatus(node)}
                    </span>
                  </div>

                  {/* Energy Level */}
                  <div className="mb-4">
                    <div className="flex items-center justify-between text-sm text-gray-500 mb-2">
                      <span>Energy Level</span>
                      <span>
                        {formatEnergy(node.energy_level)} /{" "}
                        {formatEnergy(node.capacity_kwh)}
                      </span>
                    </div>
                    <div className="relative">
                      <div className="bg-gray-200 rounded-full h-3">
                        <div
                          className={`h-3 rounded-full ${getEnergyLevelColor(
                            node
                          )}`}
                          style={{
                            width: `${Math.min(
                              100,
                              (node.energy_level / node.capacity_kwh) * 100
                            )}%`,
                          }}
                        />
                      </div>
                    </div>
                  </div>

                  {/* Node Details */}
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-500">Reserve Price:</span>
                      <span className="font-medium text-blue-600 dark:text-blue-400">
                        {formatPrice(node.reserve_price)}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Capacity:</span>
                      <span className="font-medium text-green-600 dark:text-green-400">
                        {formatEnergy(node.capacity_kwh)}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Status:</span>
                      <span className="font-medium text-purple-600 dark:text-purple-400">
                        {node.is_online ? "Online" : "Offline"}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Last Seen:</span>
                      <span className="font-medium text-orange-600 dark:text-orange-400">
                        {new Date(node.last_seen * 1000).toLocaleTimeString()}
                      </span>
                    </div>
                  </div>

                  {/* Energy Available */}
                  <div className="mt-4 p-3 bg-gray-50 rounded-lg">
                    <div className="text-sm text-gray-600 mb-1">
                      Energy Available
                    </div>
                    <div className="text-lg font-semibold text-blue-600">
                      {formatEnergy(node.energy_level)}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Aggregators */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            Connected Aggregators
          </h2>
        </div>
        <div className="p-6">
          {aggregators.length === 0 ? (
            <div className="text-center text-gray-500 py-8">
              No aggregators connected. Waiting for aggregator status events...
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {aggregators.map((aggregator) => (
                <div
                  key={aggregator.device_id}
                  className="border border-gray-200 rounded-lg p-4"
                >
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="text-lg font-medium text-gray-900">
                      AGG-{aggregator.device_id}
                    </h3>
                    <span
                      className={`px-2 py-1 text-xs font-medium rounded-full ${
                        aggregator.strategy === "Intelligent"
                          ? "bg-purple-100 text-purple-800"
                          : aggregator.strategy === "Aggressive"
                          ? "bg-red-100 text-red-800"
                          : aggregator.strategy === "Conservative"
                          ? "bg-blue-100 text-blue-800"
                          : "bg-gray-100 text-gray-800"
                      }`}
                    >
                      {aggregator.strategy}
                    </span>
                  </div>
                  <div className="space-y-1 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-500">Success Rate:</span>
                      <span className="font-medium text-green-600 dark:text-green-400">
                        {aggregator.reputation_score}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Max Bid Price:</span>
                      <span className="font-medium text-blue-600 dark:text-blue-400">
                        {formatPrice(aggregator.max_bid_price)}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Status:</span>
                      <span className="font-medium text-purple-600 dark:text-purple-400">
                        {aggregator.is_online ? "Online" : "Offline"}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">Last Seen:</span>
                      <span className="font-medium text-orange-600 dark:text-orange-400">
                        {new Date(
                          aggregator.last_seen * 1000
                        ).toLocaleTimeString()}
                      </span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
