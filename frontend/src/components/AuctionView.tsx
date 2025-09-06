import React from "react";
import { AuctionData, BESSNode, AggregatorNode } from "../types/energy-trading";

interface AuctionViewProps {
  auctions: AuctionData[];
  bessNodes: BESSNode[];
  aggregators: AggregatorNode[];
}

export const AuctionView: React.FC<AuctionViewProps> = ({
  auctions,
  bessNodes,
  aggregators,
}) => {
  const formatPrice = (price: number) => `${price.toFixed(1)}¢/kWh`;
  const formatEnergy = (energy: number) => `${energy.toFixed(1)} kWh`;
  const formatTime = (timestamp: string) =>
    new Date(timestamp).toLocaleTimeString();

  const getStatusColor = (status: string) => {
    switch (status) {
      case "active":
        return "bg-green-100 text-green-800";
      case "completed":
        return "bg-blue-100 text-blue-800";
      case "cancelled":
        return "bg-red-100 text-red-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const calculatePriceImprovement = (auction: AuctionData) => {
    if (auction.current_highest_bid <= auction.reserve_price) return 0;
    return (
      ((auction.current_highest_bid - auction.reserve_price) /
        auction.reserve_price) *
      100
    );
  };

  return (
    <div className="space-y-6">
      {/* Header Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-gray-900">
            {auctions.length}
          </div>
          <div className="text-sm text-gray-500">Total Auctions</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-green-600">
            {auctions.filter((a) => a.status === "active").length}
          </div>
          <div className="text-sm text-gray-500">Active Auctions</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-blue-600">
            {auctions.reduce((sum, a) => sum + a.total_bids, 0)}
          </div>
          <div className="text-sm text-gray-500">Total Bids</div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="text-2xl font-bold text-purple-600">
            {bessNodes?.length || 0}
          </div>
          <div className="text-sm text-gray-500">BESS Nodes</div>
        </div>
      </div>

      {/* Live Auctions */}
      <div className="bg-white shadow rounded-lg">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-lg font-medium text-gray-900">Live Auctions</h2>
        </div>
        <div className="divide-y divide-gray-200">
          {auctions.length === 0 ? (
            <div className="px-6 py-8 text-center text-gray-500">
              No auctions available. Waiting for auction events...
            </div>
          ) : (
            auctions.map((auction) => (
              <div key={auction.id} className="px-6 py-4">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-4">
                      <h3 className="text-lg font-medium text-gray-900">
                        Auction #{auction.id}
                      </h3>
                      <span
                        className={`px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(
                          auction.status
                        )}`}
                      >
                        {auction.status.toUpperCase()}
                      </span>
                    </div>
                    <div className="mt-2 grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                      <div>
                        <span className="text-gray-600 dark:text-gray-400 font-medium">
                          Start Time:
                        </span>
                        <div className="font-semibold text-blue-600 dark:text-blue-400">
                          {formatTime(auction.start_time)}
                        </div>
                      </div>
                      <div>
                        <span className="text-gray-600 dark:text-gray-400 font-medium">
                          Total Energy:
                        </span>
                        <div className="font-semibold text-green-600 dark:text-green-400">
                          {formatEnergy(auction.total_energy)}
                        </div>
                      </div>
                      <div>
                        <span className="text-gray-600 dark:text-gray-400 font-medium">
                          Reserve Price:
                        </span>
                        <div className="font-semibold text-purple-600 dark:text-purple-400">
                          {formatPrice(auction.reserve_price)}
                        </div>
                      </div>
                      <div>
                        <span className="text-gray-600 dark:text-gray-400 font-medium">
                          Total Bids:
                        </span>
                        <div className="font-semibold text-orange-600 dark:text-orange-400">
                          {auction.total_bids}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Winner Details (for completed auctions) */}
                {auction.status === "completed" &&
                  auction.winner_aggregator_id && (
                    <div className="mt-4 p-4 bg-blue-50 dark:bg-gray-800 rounded-lg border border-blue-200 dark:border-gray-600">
                      <div className="flex items-center space-x-2 mb-3">
                        <span className="text-lg">🏆</span>
                        <h4 className="text-lg font-semibold text-blue-900 dark:text-white">
                          Auction Winner
                        </h4>
                      </div>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Winner:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            Aggregator {auction.winner_aggregator_id}
                          </div>
                        </div>
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Seller:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            BESS Node {auction.seller_bess_id}
                          </div>
                        </div>
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Energy Sold:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            {auction.energy_sold?.toFixed(1)} kWh
                          </div>
                        </div>
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Final Price:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            {auction.final_price?.toFixed(1)}¢/kWh
                          </div>
                        </div>
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Total Value:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            ${((auction.total_value || 0) / 100).toFixed(2)}
                          </div>
                        </div>
                        <div>
                          <span className="text-blue-700 dark:text-gray-300 font-medium">
                            Duration:
                          </span>
                          <div className="font-semibold text-blue-900 dark:text-white">
                            {(
                              (auction.auction_duration_ms || 0) / 1000
                            ).toFixed(1)}
                            s
                          </div>
                        </div>
                      </div>
                    </div>
                  )}

                {/* Bidding Range */}
                <div className="mt-4">
                  <div className="flex items-center justify-between text-sm text-gray-500 mb-2">
                    <span>Bidding Range</span>
                    <span>
                      Price Improvement: +
                      {calculatePriceImprovement(auction).toFixed(1)}%
                    </span>
                  </div>
                  <div className="relative">
                    <div className="flex items-center justify-between text-xs text-gray-500">
                      <span>{formatPrice(auction.current_lowest_bid)}</span>
                      <span>{formatPrice(auction.current_highest_bid)}</span>
                    </div>
                    <div className="mt-1 bg-gray-200 rounded-full h-2">
                      <div
                        className="bg-gradient-to-r from-green-400 to-blue-500 h-2 rounded-full"
                        style={{
                          width: `${Math.min(
                            100,
                            (auction.current_highest_bid /
                              (auction.reserve_price * 1.5)) *
                              100
                          )}%`,
                        }}
                      />
                    </div>
                  </div>
                </div>

                {/* Participants */}
                <div className="mt-4 grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-2">
                      BESS Nodes ({bessNodes?.length || 0})
                    </h4>
                    <div className="space-y-1">
                      {(bessNodes || []).slice(0, 3).map((node) => (
                        <div
                          key={node.device_id}
                          className="flex items-center justify-between text-xs"
                        >
                          <span className="text-gray-600">{node.name}</span>
                          <span className="text-gray-500">
                            {formatEnergy(node.current_energy_level)} @{" "}
                            {formatPrice(node.reserve_price)}
                          </span>
                        </div>
                      ))}
                      {(bessNodes?.length || 0) > 3 && (
                        <div className="text-xs text-gray-400">
                          +{(bessNodes?.length || 0) - 3} more nodes
                        </div>
                      )}
                    </div>
                  </div>
                  <div>
                    <h4 className="text-sm font-medium text-gray-700 mb-2">
                      Aggregators ({aggregators?.length || 0})
                    </h4>
                    <div className="space-y-1">
                      {(aggregators || []).slice(0, 3).map((agg) => (
                        <div
                          key={agg.device_id}
                          className="flex items-center justify-between text-xs"
                        >
                          <span className="text-gray-600">{agg.name}</span>
                          <span className="text-gray-500">
                            {agg.strategy} ({agg.success_rate.toFixed(1)}%)
                          </span>
                        </div>
                      ))}
                      {(aggregators?.length || 0) > 3 && (
                        <div className="text-xs text-gray-400">
                          +{(aggregators?.length || 0) - 3} more aggregators
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
