import React from "react";
import {
  SystemMetrics as SystemMetricsType,
  BESSNode,
  AggregatorNode,
} from "../types/energy-trading";

interface SystemMetricsProps {
  metrics: SystemMetricsType | null;
  bessNodes: BESSNode[];
  aggregators: AggregatorNode[];
}

export const SystemMetrics: React.FC<SystemMetricsProps> = ({
  metrics,
  bessNodes,
  aggregators,
}) => {
  const formatNumber = (num: number) => num.toLocaleString();
  const formatPercentage = (num: number) => `${num.toFixed(1)}%`;
  const formatTime = (timestamp: string) =>
    new Date(timestamp).toLocaleTimeString();

  const onlineBessNodes = bessNodes.filter((node) => node.is_online).length;
  const onlineAggregators = aggregators.filter((agg) => agg.is_online).length;
  const totalEnergyAvailable = bessNodes.reduce(
    (sum, node) => sum + node.current_energy_level,
    0
  );
  const averageBatteryHealth =
    bessNodes.length > 0
      ? bessNodes.reduce((sum, node) => sum + node.battery_voltage, 0) /
        bessNodes.length
      : 0;

  const getHealthStatus = (
    value: number,
    thresholds: { good: number; warning: number }
  ) => {
    if (value >= thresholds.good)
      return { status: "Good", color: "text-green-600" };
    if (value >= thresholds.warning)
      return { status: "Warning", color: "text-yellow-600" };
    return { status: "Critical", color: "text-red-600" };
  };

  const systemHealth = getHealthStatus(
    onlineBessNodes / Math.max(bessNodes.length, 1),
    {
      good: 0.8,
      warning: 0.6,
    }
  );

  const batteryHealth = getHealthStatus(averageBatteryHealth, {
    good: 400,
    warning: 350,
  });

  return (
    <div className="space-y-6">
      {/* System Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-gray-900">
            {metrics ? formatNumber(metrics.total_events_broadcast) : "0"}
          </div>
          <div className="text-sm text-gray-500">Events Broadcast</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-blue-600">
            {metrics ? formatNumber(metrics.connected_clients) : "0"}
          </div>
          <div className="text-sm text-gray-500">Connected Clients</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-green-600">
            {metrics ? formatNumber(metrics.average_events_per_second) : "0"}
          </div>
          <div className="text-sm text-gray-500">Events/sec</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-purple-600">
            {metrics
              ? formatPercentage(metrics.avg_price_improvement_percent)
              : "0%"}
          </div>
          <div className="text-sm text-gray-500">Avg Price Improvement</div>
        </div>
      </div>

      {/* System Health */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">System Health</h2>
        </div>
        <div className="p-6">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="text-center">
              <div className="text-3xl font-bold text-gray-900 mb-2">
                {onlineBessNodes}/{bessNodes.length}
              </div>
              <div className="text-sm text-gray-500 mb-1">
                BESS Nodes Online
              </div>
              <div className={`text-sm font-medium ${systemHealth.color}`}>
                {systemHealth.status}
              </div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-gray-900 mb-2">
                {onlineAggregators}/{aggregators.length}
              </div>
              <div className="text-sm text-gray-500 mb-1">
                Aggregators Online
              </div>
              <div className="text-sm font-medium text-green-600">
                {onlineAggregators > 0 ? "Active" : "Inactive"}
              </div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-gray-900 mb-2">
                {formatNumber(totalEnergyAvailable)} kWh
              </div>
              <div className="text-sm text-gray-500 mb-1">
                Total Energy Available
              </div>
              <div className="text-sm font-medium text-blue-600">
                {totalEnergyAvailable > 0 ? "Available" : "None"}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Performance Metrics */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            Performance Metrics
          </h2>
        </div>
        <div className="p-6">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div>
              <div className="text-sm text-gray-500 mb-1">Total Auctions</div>
              <div className="text-2xl font-bold text-gray-900">
                {metrics ? formatNumber(metrics.total_auctions) : "0"}
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 mb-1">Total Bids</div>
              <div className="text-2xl font-bold text-blue-600">
                {metrics ? formatNumber(metrics.total_bids) : "0"}
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 mb-1">
                Active BESS Nodes
              </div>
              <div className="text-2xl font-bold text-green-600">
                {metrics ? formatNumber(metrics.active_bess_nodes) : "0"}
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 mb-1">
                Active Aggregators
              </div>
              <div className="text-2xl font-bold text-purple-600">
                {metrics ? formatNumber(metrics.active_aggregators) : "0"}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Battery Health Overview */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">
            Battery Health Overview
          </h2>
        </div>
        <div className="p-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <div className="text-sm text-gray-500 mb-2">
                Average Battery Voltage
              </div>
              <div className="flex items-center">
                <div className="text-3xl font-bold text-gray-900 mr-4">
                  {averageBatteryHealth.toFixed(1)}V
                </div>
                <div className={`text-sm font-medium ${batteryHealth.color}`}>
                  {batteryHealth.status}
                </div>
              </div>
            </div>
            <div>
              <div className="text-sm text-gray-500 mb-2">
                Battery Health Distribution
              </div>
              <div className="space-y-2">
                {bessNodes.slice(0, 5).map((node) => {
                  const health = getHealthStatus(node.battery_voltage, {
                    good: 400,
                    warning: 350,
                  });
                  return (
                    <div
                      key={node.device_id}
                      className="flex items-center justify-between"
                    >
                      <span className="text-sm text-gray-600">{node.name}</span>
                      <div className="flex items-center">
                        <span className="text-sm text-gray-500 mr-2">
                          {node.battery_voltage.toFixed(1)}V
                        </span>
                        <span className={`text-xs font-medium ${health.color}`}>
                          {health.status}
                        </span>
                      </div>
                    </div>
                  );
                })}
                {bessNodes.length > 5 && (
                  <div className="text-xs text-gray-400">
                    +{bessNodes.length - 5} more nodes
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">Recent Activity</h2>
        </div>
        <div className="p-6">
          {bessNodes.length === 0 && aggregators.length === 0 ? (
            <div className="text-center text-gray-500 py-8">
              No activity data available. Waiting for system events...
            </div>
          ) : (
            <div className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-3">
                    Recent BESS Updates
                  </h3>
                  <div className="space-y-2">
                    {bessNodes
                      .sort(
                        (a, b) =>
                          new Date(b.last_updated).getTime() -
                          new Date(a.last_updated).getTime()
                      )
                      .slice(0, 5)
                      .map((node) => (
                        <div
                          key={node.device_id}
                          className="flex items-center justify-between text-sm"
                        >
                          <span className="text-gray-600">{node.name}</span>
                          <div className="flex items-center space-x-2">
                            <span className="text-gray-500">
                              {formatTime(node.last_updated)}
                            </span>
                            <span
                              className={`px-2 py-1 text-xs rounded-full ${
                                node.is_online
                                  ? "bg-green-100 text-green-800"
                                  : "bg-red-100 text-red-800"
                              }`}
                            >
                              {node.is_online ? "Online" : "Offline"}
                            </span>
                          </div>
                        </div>
                      ))}
                  </div>
                </div>
                <div>
                  <h3 className="text-sm font-medium text-gray-700 mb-3">
                    Recent Aggregator Updates
                  </h3>
                  <div className="space-y-2">
                    {aggregators
                      .sort(
                        (a, b) =>
                          new Date(b.last_updated).getTime() -
                          new Date(a.last_updated).getTime()
                      )
                      .slice(0, 5)
                      .map((agg) => (
                        <div
                          key={agg.device_id}
                          className="flex items-center justify-between text-sm"
                        >
                          <span className="text-gray-600">{agg.name}</span>
                          <div className="flex items-center space-x-2">
                            <span className="text-gray-500">
                              {formatTime(agg.last_updated)}
                            </span>
                            <span className="text-xs text-gray-500">
                              {agg.strategy}
                            </span>
                          </div>
                        </div>
                      ))}
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
