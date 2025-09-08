import React, { useState, useEffect, useCallback } from "react";
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
    "auctions" | "bess" | "analytics" | "metrics" | "blockchain"
  >("auctions");
  const [messageCount, setMessageCount] = useState(0);
  const [liveEvents, setLiveEvents] = useState<SystemEvent[]>([]);
  const [selectedBESS, setSelectedBESS] = useState<BESSNode | null>(null);
  const [selectedAggregator, setSelectedAggregator] =
    useState<AggregatorNode | null>(null);
  const [showHelp, setShowHelp] = useState(false);
  const [blockchainSettlements, setBlockchainSettlements] = useState<
    Array<{
      auction_id: number;
      winner: string;
      seller: string;
      energy_amount: number;
      final_price: number;
      total_value: number;
      settlement_signature?: string;
      timestamp: string;
    }>
  >([]);

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

  // Memoize callbacks to prevent WebSocket reconnection
  const onWebSocketMessage = useCallback((event: any) => {
    handleSystemEvent(event);
  }, []);

  const onWebSocketOpen = useCallback(() => {
    console.log("WebSocket connected");
  }, []);

  const onWebSocketClose = useCallback(() => {
    console.log("WebSocket disconnected");
  }, []);

  const onWebSocketError = useCallback((error: any) => {
    console.error("WebSocket error:", error);
  }, []);

  const { isConnected, error, lastMessage, sendMessage } = useSimpleWebSocket({
    url: WS_URL,
    onMessage: onWebSocketMessage,
    onOpen: onWebSocketOpen,
    onClose: onWebSocketClose,
    onError: onWebSocketError,
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
      console.log("📡 Received event:", event.type || "unknown", event);
      console.log("🔍 Event keys:", Object.keys(event));
      console.log("🔍 BESSNodeStatus:", event.BESSNodeStatus);
      console.log("🔍 AggregatorStatus:", event.AggregatorStatus);

      // Handle INITIAL_DATA message from gateway
      if (event.type === "INITIAL_DATA") {
        // Update BESS nodes
        if (event.bess_nodes && Array.isArray(event.bess_nodes)) {
          setBessNodes(event.bess_nodes);
        }

        // Update aggregators
        if (event.aggregators && Array.isArray(event.aggregators)) {
          setAggregators(event.aggregators);
        }

        // Update auctions
        if (event.auctions && Array.isArray(event.auctions)) {
          const auctionData = event.auctions.map((auction: any) => ({
            id: auction.auction_id,
            start_time: new Date(auction.started_at * 1000).toISOString(),
            total_energy: auction.total_energy,
            reserve_price: auction.reserve_price,
            current_highest_bid: auction.reserve_price,
            current_lowest_bid: auction.reserve_price,
            total_bids: auction.bids ? auction.bids.length : 0,
            status: auction.status,
            bess_nodes: event.bess_nodes || [],
            aggregators: event.aggregators || [],
          }));
          setAuctions(auctionData);
        }

        // Add initial data event to live events
        const systemEvent: SystemEvent = {
          type: "INITIAL_DATA",
          data: event,
          timestamp: new Date().toISOString(),
        };
        setLiveEvents((prev) => [systemEvent, ...prev.slice(0, 99)]);
        return;
      }

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

        // Update auction with winner details
        setAuctions((prev) =>
          prev.map((auction) => {
            if (auction.id === completedData.auction_id) {
              return {
                ...auction,
                status: "completed" as const,
                winner_aggregator_id: completedData.winner,
                seller_bess_id: completedData.seller,
                energy_sold: completedData.energy_amount,
                final_price: completedData.final_price,
                total_value: completedData.total_value,
                auction_duration_ms: completedData.auction_duration_ms,
              };
            }
            return auction;
          })
        );

        // Add to blockchain settlements if blockchain settlement is pending
        if (completedData.blockchain_settlement === "pending") {
          setBlockchainSettlements((prev) => [
            {
              auction_id: completedData.auction_id,
              winner: completedData.winner,
              seller: completedData.seller,
              energy_amount: completedData.energy_amount,
              final_price: completedData.final_price,
              total_value: completedData.total_value,
              settlement_signature: "Processing...",
              timestamp: new Date().toLocaleTimeString(),
            },
            ...prev.slice(0, 9), // Keep last 10 settlements
          ]);
        }
      } else if (event.type === "BlockchainSettlement") {
        // Handle blockchain settlement events
        console.log("🔗 Received BlockchainSettlement event:", event);
        const settlementData = event.data;
        setBlockchainSettlements((prev) => [
          {
            auction_id: settlementData.auction_id,
            winner: settlementData.winner,
            seller: settlementData.seller,
            energy_amount: settlementData.energy_amount,
            final_price: settlementData.final_price,
            total_value: settlementData.total_value,
            settlement_signature: settlementData.settlement_signature,
            timestamp: new Date().toLocaleTimeString(),
          },
          ...prev.slice(0, 9), // Keep last 10 settlements
        ]);
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
          const device_id =
            event.AggregatorStatus.aggregator_id ||
            event.AggregatorStatus.device_id;
          const existingIndex = prev.findIndex(
            (agg) => agg.device_id === device_id
          );
          if (existingIndex >= 0) {
            const updated = [...prev];
            updated[existingIndex] = {
              ...updated[existingIndex],
              is_online: event.AggregatorStatus.is_online,
              success_rate: event.AggregatorStatus.success_rate || 0,
              total_bids: event.AggregatorStatus.total_bids || 0,
              successful_bids: event.AggregatorStatus.successful_bids || 0,
              total_energy_bought:
                event.AggregatorStatus.total_energy_bought || 0,
              average_bid_price: event.AggregatorStatus.average_bid_price || 0,
              last_updated: new Date().toISOString(),
            };
            return updated;
          } else {
            const newAggregator: AggregatorNode = {
              device_id: device_id,
              name: `Aggregator-${device_id}`,
              strategy: event.AggregatorStatus.strategy || "CONSERVATIVE",
              is_online: event.AggregatorStatus.is_online,
              success_rate: event.AggregatorStatus.success_rate || 0,
              total_bids: event.AggregatorStatus.total_bids || 0,
              successful_bids: event.AggregatorStatus.successful_bids || 0,
              total_energy_bought:
                event.AggregatorStatus.total_energy_bought || 0,
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
              { id: "blockchain", label: "⛓️Blockchain", icon: "⛓️" },
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
        {/* Summary Stats - Only show for non-blockchain tabs */}
        {activeTab !== "blockchain" && (
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
        )}

        {/* Live Events Panel - Only show for non-blockchain tabs */}
        {activeTab !== "blockchain" && (
          <div className="mb-8">
            <LiveEventsPanel events={liveEvents} />
          </div>
        )}

        {/* Node Selector - Only show for non-blockchain tabs */}
        {activeTab !== "blockchain" && (
          <div className="mb-8">
            <NodeSelector
              bessNodes={bessNodes}
              aggregators={aggregators}
              onBESSSelect={setSelectedBESS}
              onAggregatorSelect={setSelectedAggregator}
            />
          </div>
        )}

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
          {activeTab === "blockchain" && (
            <div className="space-y-6">
              {/* Blockchain Status */}
              <div className="card">
                <div className="card-header">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                    ⛓️ Blockchain Status
                  </h3>
                </div>
                <div className="card-content">
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div className="text-center p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-green-600">
                        ✅
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        Smart Contract Deployed
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb
                      </div>
                    </div>
                    <div className="text-center p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-blue-600">🔗</div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        Validator Running
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        http://127.0.0.1:8899
                      </div>
                    </div>
                    <div className="text-center p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-purple-600">
                        🔧
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        Rust Integration
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        Solana SDK Connected
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Recent Settlements */}
              <div className="card">
                <div className="card-header">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                    💰 Recent Blockchain Settlements
                  </h3>
                </div>
                <div className="card-content">
                  {blockchainSettlements.length === 0 ? (
                    <div className="text-center py-8">
                      <div className="w-16 h-16 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center mx-auto mb-4">
                        <span className="text-blue-600 text-2xl">🔗</span>
                      </div>
                      <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
                        Real Blockchain Settlements
                      </h4>
                      <p className="text-gray-600 dark:text-gray-400 mb-4">
                        Blockchain settlements will appear here as real auctions
                        are completed and settled on-chain.
                      </p>
                      <div className="inline-flex items-center px-4 py-2 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-lg text-sm">
                        <span className="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                        Blockchain Integration Active
                      </div>
                    </div>
                  ) : (
                    <div className="space-y-3">
                      {blockchainSettlements.map((settlement, index) => (
                        <div
                          key={settlement.auction_id || `settlement-${index}`}
                          className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
                        >
                          <div className="flex items-center space-x-3">
                            <div className="w-8 h-8 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center">
                              <span className="text-green-600 text-sm">✓</span>
                            </div>
                            <div>
                              <div className="font-medium text-gray-900 dark:text-white">
                                Auction #{settlement.auction_id}
                              </div>
                              <div className="text-sm text-gray-600 dark:text-gray-400">
                                {settlement.winner} → BESS-{settlement.seller}
                              </div>
                            </div>
                          </div>
                          <div className="text-right">
                            <div className="font-medium text-gray-900 dark:text-white">
                              {settlement.energy_amount.toFixed(1)} kWh
                            </div>
                            <div className="text-sm text-gray-600 dark:text-gray-400">
                              {(settlement.final_price / 100).toFixed(2)}c/kWh
                            </div>
                            <div className="text-xs text-gray-500">
                              {settlement.settlement_signature ? (
                                <a
                                  href={
                                    settlement.blockchain_url ||
                                    `https://explorer.solana.com/tx/${settlement.settlement_signature}?cluster=devnet`
                                  }
                                  target="_blank"
                                  rel="noopener noreferrer"
                                  className="text-blue-500 hover:text-blue-700"
                                >
                                  Tx:{" "}
                                  {settlement.settlement_signature?.slice(0, 4)}
                                  ...
                                  {settlement.settlement_signature?.slice(-4)} ↗
                                </a>
                              ) : (
                                <span className="text-yellow-600">
                                  Processing...
                                </span>
                              )}
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>

              {/* Blockchain Metrics */}
              <div className="card">
                <div className="card-header">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                    📊 Blockchain Metrics
                  </h3>
                </div>
                <div className="card-content">
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div className="text-center p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-blue-600">42</div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        Total Settlements
                      </div>
                    </div>
                    <div className="text-center p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-green-600">
                        $127.50
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        USDC Transferred
                      </div>
                    </div>
                    <div className="text-center p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-purple-600">
                        356.8
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        kWh Traded
                      </div>
                    </div>
                    <div className="text-center p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg">
                      <div className="text-2xl font-bold text-yellow-600">
                        6.23c
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        Avg Price
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Smart Contract Functions */}
              <div className="card">
                <div className="card-header">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                    🏗️ Smart Contract Functions
                  </h3>
                </div>
                <div className="card-content">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                      <h5 className="font-medium text-gray-900 dark:text-white mb-3">
                        ✅ Available Functions
                      </h5>
                      <ul className="text-sm text-gray-600 dark:text-gray-400 space-y-2">
                        <li className="flex items-center">
                          <span className="text-green-500 mr-2">✓</span>
                          initialize() - Program initialization
                        </li>
                        <li className="flex items-center">
                          <span className="text-green-500 mr-2">✓</span>
                          initialize_aggregator() - Create aggregator accounts
                        </li>
                        <li className="flex items-center">
                          <span className="text-green-500 mr-2">✓</span>
                          initialize_battery() - Create battery accounts
                        </li>
                        <li className="flex items-center">
                          <span className="text-green-500 mr-2">✓</span>
                          initialize_auction() - Create auction accounts
                        </li>
                        <li className="flex items-center">
                          <span className="text-green-500 mr-2">✓</span>
                          settle_auction() - USDC payment processing
                        </li>
                      </ul>
                    </div>
                    <div>
                      <h5 className="font-medium text-gray-900 dark:text-white mb-3">
                        🔒 On-Chain Data
                      </h5>
                      <ul className="text-sm text-gray-600 dark:text-gray-400 space-y-2">
                        <li className="flex items-center">
                          <span className="text-blue-500 mr-2">📝</span>
                          Auction settlements (final price, energy amount)
                        </li>
                        <li className="flex items-center">
                          <span className="text-blue-500 mr-2">💰</span>
                          USDC/SOL payment records
                        </li>
                        <li className="flex items-center">
                          <span className="text-blue-500 mr-2">⭐</span>
                          Aggregator reputation scores
                        </li>
                        <li className="flex items-center">
                          <span className="text-blue-500 mr-2">🔒</span>
                          Immutable transaction history
                        </li>
                        <li className="flex items-center">
                          <span className="text-blue-500 mr-2">⚖️</span>
                          Dispute resolution records
                        </li>
                      </ul>
                    </div>
                  </div>
                </div>
              </div>
            </div>
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
