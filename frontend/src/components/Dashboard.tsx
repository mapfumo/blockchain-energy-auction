import React, { useState, useEffect } from "react";
import { useSimpleWebSocket } from "../hooks/useSimpleWebSocket";
import {
  SystemEvent,
  BESSNode,
  AggregatorNode,
  AuctionData,
  SystemMetrics,
  AuctionStartedEvent,
  BidPlacedEvent,
  BidAcceptedEvent,
  BESSNodeStatusEvent,
  AggregatorStatusEvent,
  SystemMetricsEvent,
} from "../types/energy-trading";
import { AuctionView } from "./AuctionView";
import { BESSNodeMap } from "./BESSNodeMap";
import { PriceAnalytics } from "./PriceAnalytics";
import { SystemMetrics as SystemMetricsComponent } from "./SystemMetrics";
import { ConnectionStatus } from "./ConnectionStatus";

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8080/ws";

export const Dashboard: React.FC = () => {
  const [bessNodes, setBessNodes] = useState<BESSNode[]>([]);
  const [aggregators, setAggregators] = useState<AggregatorNode[]>([]);
  const [auctions, setAuctions] = useState<AuctionData[]>([]);
  const [systemMetrics, setSystemMetrics] = useState<SystemMetrics | null>(
    null
  );
  const [priceHistory, setPriceHistory] = useState<
    Array<{ timestamp: string; price: number; energy_amount: number }>
  >([]);
  const [activeTab, setActiveTab] = useState<
    "auctions" | "bess" | "analytics" | "metrics"
  >("auctions");
  const [messageCount, setMessageCount] = useState(0);

  const { isConnected, error, lastMessage, sendMessage } = useSimpleWebSocket({
    url: WS_URL,
    onMessage: (event: any) => {
      handleSystemEvent(event);
    },
    onError: (error) => {
      console.error("WebSocket error:", error);
    },
  });

  // Create connection object for compatibility
  const connection = {
    isConnected,
    lastMessage,
    error,
    reconnectAttempts: 0,
  };

  const handleSystemEvent = (event: any) => {
    console.log("🎯 Dashboard received event:", event);
    console.log("🎯 Event type:", typeof event);
    console.log("🎯 Event keys:", Object.keys(event || {}));
    setMessageCount((prev) => prev + 1);

    // Handle the actual WebSocket message format from Rust backend
    if (event.AuctionStarted) {
      const auctionData: AuctionData = {
        id: event.AuctionStarted.auction_id,
        start_time: new Date().toISOString(),
        total_energy: event.AuctionStarted.total_energy,
        reserve_price: event.AuctionStarted.reserve_price,
        current_highest_bid: event.AuctionStarted.reserve_price,
        current_lowest_bid: event.AuctionStarted.reserve_price,
        total_bids: 0,
        status: "active",
        bess_nodes: bessNodes,
        aggregators: aggregators,
      };
      setAuctions((prev) => [auctionData, ...prev.slice(0, 9)]); // Keep last 10 auctions
    } else if (event.BidPlaced) {
      setAuctions((prev) =>
        prev.map((auction) => {
          if (auction.id === event.BidPlaced.auction_id) {
            return {
              ...auction,
              current_highest_bid: Math.max(
                auction.current_highest_bid,
                event.BidPlaced.bid_price
              ),
              current_lowest_bid: Math.min(
                auction.current_lowest_bid,
                event.BidPlaced.bid_price
              ),
              total_bids: auction.total_bids + 1,
            };
          }
          return auction;
        })
      );

      // Add to price history
      setPriceHistory((prev) => [
        {
          timestamp: new Date().toISOString(),
          price: event.BidPlaced.bid_price,
          energy_amount: event.BidPlaced.energy_amount,
        },
        ...prev.slice(0, 99), // Keep last 100 price points
      ]);
    } else if (event.BidAccepted) {
      setAuctions((prev) =>
        prev.map((auction) => {
          if (auction.id === event.BidAccepted.auction_id) {
            return {
              ...auction,
              status: "completed" as const,
            };
          }
          return auction;
        })
      );
    } else if (event.SystemMetrics) {
      setSystemMetrics(event.SystemMetrics);
    } else if (event.BESSNodeStatus) {
      setBessNodes((prev) => {
        const existingIndex = prev.findIndex(
          (node) => node.device_id === event.BESSNodeStatus.device_id
        );
        if (existingIndex >= 0) {
          const updated = [...prev];
          updated[existingIndex] = {
            ...updated[existingIndex],
            current_energy_level: event.BESSNodeStatus.energy_available,
            is_online: event.BESSNodeStatus.is_online,
            last_updated: new Date().toISOString(),
          };
          return updated;
        } else {
          const newBessNode: BESSNode = {
            device_id: event.BESSNodeStatus.device_id,
            name: `BESS-${event.BESSNodeStatus.device_id}`,
            capacity: 100.0,
            current_energy_level: event.BESSNodeStatus.energy_available,
            reserve_price: 15.0,
            percentage_for_sale: 50.0,
            battery_voltage: 12.0,
            max_discharge_rate: 10.0,
            is_online: event.BESSNodeStatus.is_online,
            last_updated: new Date().toISOString(),
          };
          return [...prev, newBessNode];
        }
      });
    } else if (event.AggregatorStatus) {
      setAggregators((prev) => {
        const existingIndex = prev.findIndex(
          (agg) => agg.device_id === event.AggregatorStatus.device_id
        );
        if (existingIndex >= 0) {
          const updated = [...prev];
          updated[existingIndex] = {
            ...updated[existingIndex],
            is_online: event.AggregatorStatus.is_online,
            success_rate: event.AggregatorStatus.success_rate,
            total_bids: event.AggregatorStatus.total_bids,
            average_bid_price: event.AggregatorStatus.average_bid_price,
            last_updated: new Date().toISOString(),
          };
          return updated;
        } else {
          const newAggregator: AggregatorNode = {
            device_id: event.AggregatorStatus.device_id,
            name: `Aggregator-${event.AggregatorStatus.device_id}`,
            strategy: event.AggregatorStatus.strategy,
            is_online: event.AggregatorStatus.is_online,
            success_rate: event.AggregatorStatus.success_rate,
            total_bids: event.AggregatorStatus.total_bids,
            average_bid_price: event.AggregatorStatus.average_bid_price,
            last_updated: new Date().toISOString(),
          };
          return [...prev, newAggregator];
        }
      });
    }
  };

  // Calculate summary statistics
  const totalAuctions = auctions.length;
  const activeAuctions = auctions.filter((a) => a.status === "active").length;
  const totalBids = auctions.reduce((sum, a) => sum + a.total_bids, 0);
  const totalBessNodes = bessNodes.length;

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                Energy Trading Dashboard
              </h1>
              <p className="text-sm text-gray-600">
                Real-time energy auction monitoring
              </p>
              <p className="text-xs text-blue-600">
                Messages received: {messageCount}
              </p>
            </div>
            <ConnectionStatus connection={connection} />
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="bg-white border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex space-x-8">
            {[
              { id: "auctions", label: "⚡Live Auctions", icon: "⚡" },
              { id: "bess", label: "🔋BESS Nodes", icon: "🔋" },
              { id: "analytics", label: "📊Price Analytics", icon: "📊" },
              { id: "metrics", label: "📈System Metrics", icon: "📈" },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === tab.id
                    ? "border-blue-500 text-blue-600"
                    : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Summary Stats */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-blue-100 rounded-md flex items-center justify-center">
                  <span className="text-blue-600 font-semibold">⚡</span>
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  Total Auctions
                </p>
                <p className="text-2xl font-semibold text-gray-900">
                  {totalAuctions}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-green-100 rounded-md flex items-center justify-center">
                  <span className="text-green-600 font-semibold">🟢</span>
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">
                  Active Auctions
                </p>
                <p className="text-2xl font-semibold text-gray-900">
                  {activeAuctions}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-yellow-100 rounded-md flex items-center justify-center">
                  <span className="text-yellow-600 font-semibold">💰</span>
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">Total Bids</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {totalBids}
                </p>
              </div>
            </div>
          </div>

          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <div className="w-8 h-8 bg-purple-100 rounded-md flex items-center justify-center">
                  <span className="text-purple-600 font-semibold">🔋</span>
                </div>
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-500">BESS Nodes</p>
                <p className="text-2xl font-semibold text-gray-900">
                  {totalBessNodes}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Tab Content */}
        <div className="bg-white rounded-lg shadow">
          {activeTab === "auctions" && (
            <AuctionView
              auctions={auctions}
              bessNodes={bessNodes}
              aggregators={aggregators}
            />
          )}
          {activeTab === "bess" && (
            <BESSNodeMap bessNodes={bessNodes} aggregators={aggregators} />
          )}
          {activeTab === "analytics" && (
            <PriceAnalytics
              priceHistory={priceHistory}
              auctions={auctions}
              bessNodes={bessNodes}
              aggregators={aggregators}
            />
          )}
          {activeTab === "metrics" && (
            <SystemMetricsComponent metrics={systemMetrics} />
          )}
        </div>
      </main>
    </div>
  );
};
