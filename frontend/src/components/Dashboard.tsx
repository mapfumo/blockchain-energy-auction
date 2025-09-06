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
import { ThemeToggle } from "./ThemeToggle";
import { LiveEventsPanel } from "./LiveEventsPanel";
import { NodeSelector } from "./NodeSelector";
import { BESSNodeDetails } from "./BESSNodeDetails";
import { AggregatorDetails } from "./AggregatorDetails";
import Logo from "./Logo";
import HelpModal from "./HelpModal";
import { useKeyboardShortcuts } from "../hooks/useKeyboardShortcuts";

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
  const [liveEvents, setLiveEvents] = useState<SystemEvent[]>([]);
  const [selectedBESS, setSelectedBESS] = useState<BESSNode | null>(null);
  const [selectedAggregator, setSelectedAggregator] =
    useState<AggregatorNode | null>(null);
  const [showHelp, setShowHelp] = useState(false);

  // Keyboard shortcuts
  useKeyboardShortcuts({
    onRefresh: () => {
      // Refresh data by reconnecting WebSocket
      window.location.reload();
    },
    onToggleTheme: () => {
      // This would need to be passed from ThemeContext
      console.log("Toggle theme shortcut pressed");
    },
    onEscape: () => {
      setShowHelp(false);
      setSelectedBESS(null);
      setSelectedAggregator(null);
    },
  });

  const { isConnected, error, lastMessage, sendMessage } = useSimpleWebSocket({
    url: WS_URL,
    onMessage: (event: any) => {
      console.log("🎯 Dashboard received event:", event);
      console.log("🎯 WebSocket isConnected:", isConnected);
      handleSystemEvent(event);
    },
    onOpen: () => {
      console.log("🎯 WebSocket connected successfully!");
    },
    onClose: () => {
      console.log("🎯 WebSocket connection closed");
    },
    onError: (error) => {
      console.error("🎯 WebSocket error:", error);
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
    try {
      setMessageCount((prev) => prev + 1);

      // Add event to live events list
      const systemEvent: SystemEvent = {
        type: Object.keys(event)[0] as any,
        data: event[Object.keys(event)[0]],
        timestamp: new Date().toISOString(),
      };
      setLiveEvents((prev) => [systemEvent, ...prev.slice(0, 99)]); // Keep last 100 events

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
      } else if (
        event.auction_id &&
        event.total_energy &&
        event.reserve_price
      ) {
        // Handle flat event structure from backend
        const auctionData: AuctionData = {
          id: event.auction_id,
          start_time: new Date().toISOString(),
          total_energy: event.total_energy,
          reserve_price: event.reserve_price,
          current_highest_bid: event.reserve_price,
          current_lowest_bid: event.reserve_price,
          total_bids: 0,
          status: "active",
          bess_nodes: bessNodes,
          aggregators: aggregators,
        };
        setAuctions((prev) => [auctionData, ...prev.slice(0, 9)]); // Keep last 10 auctions
      } else if (
        event.BidPlaced ||
        (event.auction_id && event.aggregator_id && event.bid_price)
      ) {
        const bidData = event.BidPlaced || event;
        setAuctions((prev) => {
          const updated = prev.map((auction) => {
            if (auction.id === bidData.auction_id) {
              return {
                ...auction,
                current_highest_bid: Math.max(
                  auction.current_highest_bid,
                  bidData.bid_price
                ),
                current_lowest_bid: Math.min(
                  auction.current_lowest_bid,
                  bidData.bid_price
                ),
                total_bids: auction.total_bids + 1,
              };
            }
            return auction;
          });
          return updated;
        });

        // Add to price history
        setPriceHistory((prev) => [
          {
            timestamp: new Date().toISOString(),
            price: bidData.bid_price,
            energy_amount: bidData.energy_amount,
          },
          ...prev.slice(0, 99), // Keep last 100 price points
        ]);
      } else if (
        event.BidAccepted ||
        (event.auction_id && event.final_price && event.energy_amount)
      ) {
        const acceptData = event.BidAccepted || event;
        setAuctions((prev) =>
          prev.map((auction) => {
            if (auction.id === acceptData.auction_id) {
              return {
                ...auction,
                status: "completed" as const,
              };
            }
            return auction;
          })
        );
      } else if (event.AuctionCompleted) {
        // Handle detailed auction completion with winner information
        const completedData = event.AuctionCompleted;
        console.log("🏆 Auction Completed:", {
          auction_id: completedData.auction_id,
          winner: `Aggregator ${completedData.winner_aggregator_id}`,
          seller: `BESS ${completedData.seller_bess_id}`,
          energy: `${completedData.energy_sold} kWh`,
          price: `${completedData.final_price}¢/kWh`,
          total: `$${(completedData.total_value / 100).toFixed(2)}`,
          duration: `${(completedData.auction_duration_ms / 1000).toFixed(1)}s`,
        });

        // Update auction with winner details
        setAuctions((prev) =>
          prev.map((auction) => {
            if (auction.id === completedData.auction_id) {
              return {
                ...auction,
                status: "completed" as const,
                winner_aggregator_id: completedData.winner_aggregator_id,
                seller_bess_id: completedData.seller_bess_id,
                energy_sold: completedData.energy_sold,
                final_price: completedData.final_price,
                total_value: completedData.total_value,
                auction_duration_ms: completedData.auction_duration_ms,
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
              battery_health: event.BESSNodeStatus.battery_health,
              is_online: event.BESSNodeStatus.is_online,
              last_updated: new Date().toISOString(),
            };
            return updated;
          } else {
            const newBessNode: BESSNode = {
              device_id: event.BESSNodeStatus.device_id,
              name: `BESS-${event.BESSNodeStatus.device_id}`,
              capacity: 15.0, // 15kWh max capacity (realistic Australian home battery)
              current_energy_level: event.BESSNodeStatus.energy_available,
              reserve_price: 5.0 + Math.random() * 25.0, // 5-30 c/kWh (competitive pricing range)
              percentage_for_sale: 50.0 + (event.BESSNodeStatus.device_id % 30), // 50-80% available for sale
              battery_voltage: [12.0, 24.0, 48.0][
                event.BESSNodeStatus.device_id % 3
              ], // 12V, 24V, 48V (Australian residential standards)
              max_discharge_rate:
                5.0 + (event.BESSNodeStatus.device_id % 3) * 1.0, // 5-7kW discharge rate
              battery_health: event.BESSNodeStatus.battery_health,
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
              successful_bids: event.AggregatorStatus.successful_bids,
              total_energy_bought: event.AggregatorStatus.total_energy_bought,
              average_bid_price: event.AggregatorStatus.average_bid_price || 0,
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
              successful_bids: event.AggregatorStatus.successful_bids,
              total_energy_bought: event.AggregatorStatus.total_energy_bought,
              average_bid_price: event.AggregatorStatus.average_bid_price || 0,
              last_updated: new Date().toISOString(),
            };
            return [...prev, newAggregator];
          }
        });
      }
    } catch (error) {
      console.error("Error processing system event:", error, event);
    }
  };

  // Calculate summary statistics
  const totalAuctions = auctions.length;
  const activeAuctions = auctions.filter((a) => a.status === "active").length;
  const totalBids = auctions.reduce((sum, a) => sum + a.total_bids, 0);
  const totalBessNodes = bessNodes.length;
  const totalAggregators = aggregators.length;

  return (
    <div className="min-h-screen bg-white dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-soft border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center space-x-4">
              <Logo size="lg" />
              <div>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Real-time energy auction monitoring
                </p>
                <p className="text-xs text-blue-600 dark:text-blue-400">
                  Messages received: {messageCount}
                </p>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <button
                onClick={() => setShowHelp(true)}
                className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
                title="Help & Shortcuts (Ctrl+Shift+H)"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </button>
              <ThemeToggle />
              <ConnectionStatus connection={connection} />
            </div>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <nav className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
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
                className={`py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                  activeTab === tab.id
                    ? "border-blue-500 text-blue-600 dark:text-blue-400"
                    : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600"
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
          <div className="card">
            <div className="card-content">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-primary/10 rounded-md flex items-center justify-center">
                    <span className="text-primary font-semibold">⚡</span>
                  </div>
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Total Auctions
                  </p>
                  <p className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
                    {totalAuctions}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-success/10 rounded-md flex items-center justify-center">
                    <span className="text-success font-semibold">🟢</span>
                  </div>
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Active Auctions
                  </p>
                  <p className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
                    {activeAuctions}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-warning/10 rounded-md flex items-center justify-center">
                    <span className="text-warning font-semibold">💰</span>
                  </div>
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Total Bids
                  </p>
                  <p className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
                    {totalBids}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-secondary/10 rounded-md flex items-center justify-center">
                    <span className="text-secondary font-semibold">🔋</span>
                  </div>
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    BESS Nodes
                  </p>
                  <p className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
                    {totalBessNodes}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <div className="w-8 h-8 bg-accent/10 rounded-md flex items-center justify-center">
                    <span className="text-accent font-semibold">⚡</span>
                  </div>
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Aggregators
                  </p>
                  <p className="text-2xl font-semibold text-gray-900 dark:text-gray-100">
                    {totalAggregators}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Live Events Panel */}
        <div className="mb-8">
          <LiveEventsPanel events={liveEvents} />
        </div>

        {/* Node Selector */}
        <div className="mb-8">
          <NodeSelector
            bessNodes={bessNodes}
            aggregators={aggregators}
            onBESSSelect={setSelectedBESS}
            onAggregatorSelect={setSelectedAggregator}
          />
        </div>

        {/* Tab Content */}
        <div className="card">
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
            <PriceAnalytics priceHistory={priceHistory} auctions={auctions} />
          )}
          {activeTab === "metrics" && (
            <SystemMetricsComponent
              metrics={systemMetrics}
              bessNodes={bessNodes}
              aggregators={aggregators}
            />
          )}
        </div>
      </main>

      {/* Popup Components */}
      <BESSNodeDetails
        node={selectedBESS}
        onClose={() => setSelectedBESS(null)}
      />
      <AggregatorDetails
        aggregator={selectedAggregator}
        onClose={() => setSelectedAggregator(null)}
      />

      {/* Help Modal */}
      <HelpModal isOpen={showHelp} onClose={() => setShowHelp(false)} />
    </div>
  );
};
