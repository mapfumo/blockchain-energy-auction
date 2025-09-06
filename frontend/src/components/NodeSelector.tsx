import React, { useState } from "react";
import { BESSNode, AggregatorNode } from "../types/energy-trading";

interface NodeSelectorProps {
  bessNodes: BESSNode[];
  aggregators: AggregatorNode[];
  onBESSSelect: (node: BESSNode | null) => void;
  onAggregatorSelect: (aggregator: AggregatorNode | null) => void;
}

export const NodeSelector: React.FC<NodeSelectorProps> = ({
  bessNodes,
  aggregators,
  onBESSSelect,
  onAggregatorSelect,
}) => {
  const [selectedBESS, setSelectedBESS] = useState<BESSNode | null>(null);
  const [selectedAggregator, setSelectedAggregator] =
    useState<AggregatorNode | null>(null);

  const handleBESSChange = (nodeId: string) => {
    const node = nodeId
      ? bessNodes.find((n) => n.device_id.toString() === nodeId) || null
      : null;
    setSelectedBESS(node);
    onBESSSelect(node);
  };

  const handleAggregatorChange = (aggregatorId: string) => {
    const aggregator = aggregatorId
      ? aggregators.find((a) => a.device_id.toString() === aggregatorId) || null
      : null;
    setSelectedAggregator(aggregator);
    onAggregatorSelect(aggregator);
  };

  return (
    <div className="card">
      <div className="card-header">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
          Node Details
        </h3>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          Select a BESS node or Aggregator to view detailed information
        </p>
      </div>
      <div className="card-content">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* BESS Node Selector */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              BESS Nodes
            </label>
            <select
              value={selectedBESS?.device_id.toString() || ""}
              onChange={(e) => handleBESSChange(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary dark:bg-gray-700 dark:text-gray-100"
            >
              <option value="">Select a BESS node...</option>
              {bessNodes.map((node) => (
                <option key={node.device_id} value={node.device_id.toString()}>
                  {node.name} - {node.current_energy_level.toFixed(1)} kWh
                  available
                </option>
              ))}
            </select>
            {selectedBESS && (
              <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
                Selected: {selectedBESS.name} (ID: {selectedBESS.device_id})
              </div>
            )}
          </div>

          {/* Aggregator Selector */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Aggregators
            </label>
            <select
              value={selectedAggregator?.device_id.toString() || ""}
              onChange={(e) => handleAggregatorChange(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary dark:bg-gray-700 dark:text-gray-100"
            >
              <option value="">Select an aggregator...</option>
              {aggregators.map((aggregator) => (
                <option
                  key={aggregator.device_id}
                  value={aggregator.device_id.toString()}
                >
                  {aggregator.name} - {aggregator.strategy} (
                  {aggregator.success_rate.toFixed(1)}% success)
                </option>
              ))}
            </select>
            {selectedAggregator && (
              <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
                Selected: {selectedAggregator.name} (ID:{" "}
                {selectedAggregator.device_id})
              </div>
            )}
          </div>
        </div>

        {/* Quick Stats */}
        <div className="mt-4 grid grid-cols-2 gap-4">
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-3">
            <div className="text-sm text-gray-500 dark:text-gray-400">
              Total BESS Nodes
            </div>
            <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
              {bessNodes.length}
            </div>
          </div>
          <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-3">
            <div className="text-sm text-gray-500 dark:text-gray-400">
              Total Aggregators
            </div>
            <div className="text-lg font-semibold text-gray-900 dark:text-gray-100">
              {aggregators.length}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
