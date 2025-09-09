import React, { useState, useEffect } from "react";
import {
  SystemEvent,
  AuctionStartedEvent,
  BidPlacedEvent,
  BidAcceptedEvent,
  BidRejectedEvent,
  AuctionCompletedEvent,
  QuerySentEvent,
  QueryResponseEvent,
  EnergyDepletedEvent,
  EnergyRechargedEvent,
  SystemMetricsEvent,
  BESSNodeStatusEvent,
  AggregatorStatusEvent,
  BESSNodeDiscoveredEvent,
  AggregatorDiscoveredEvent,
  HeartbeatReceivedEvent,
  BESSNodeRegisteredEvent,
  AggregatorRegisteredEvent,
  DirectQuerySentEvent,
  DirectQueryResponseEvent,
} from "../types/energy-trading";

interface LiveEventsPanelProps {
  events: SystemEvent[];
  maxEvents?: number;
}

// Helper function to safely format numbers
const safeToFixed = (
  value: number | undefined | null,
  decimals: number = 1
): string => {
  return value !== undefined && value !== null
    ? value.toFixed(decimals)
    : "N/A";
};

// Helper function to safely format prices (convert cents to c/kWh)
const safePrice = (
  value: number | undefined | null,
  decimals: number = 2
): string => {
  return value !== undefined && value !== null
    ? `${(value / 100).toFixed(decimals)}c/kWh`
    : "N/A";
};

type EventFilter =
  | "ALL"
  | "AuctionStarted"
  | "BidPlaced"
  | "BidAccepted"
  | "BidRejected"
  | "AuctionCompleted"
  | "QuerySent"
  | "QueryResponse"
  | "EnergyDepleted"
  | "EnergyRecharged"
  | "BESSNodeStatus"
  | "AggregatorStatus"
  | "SystemMetrics"
  | "BESSNodeDiscovered"
  | "AggregatorDiscovered"
  | "HeartbeatReceived"
  | "BESSNodeRegistered"
  | "AggregatorRegistered"
  | "DirectQuerySent"
  | "DirectQueryResponse"
  | "MULTICAST"
  | "QUERIES"
  | "REGISTRATIONS"
  | "DIRECT_QUERIES";

export const LiveEventsPanel: React.FC<LiveEventsPanelProps> = ({
  events,
  maxEvents = 100,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [filter, setFilter] = useState<EventFilter>("ALL");

  // Filter events based on selected filter
  const filteredEvents = events.filter((event) => {
    if (filter === "ALL") return true;
    if (filter === "MULTICAST") {
      return (
        event.type === "BESSNodeDiscovered" ||
        event.type === "AggregatorDiscovered" ||
        event.type === "HeartbeatReceived"
      );
    }
    if (filter === "QUERIES") {
      return event.type === "QuerySent" || event.type === "QueryResponse";
    }
    if (filter === "REGISTRATIONS") {
      return (
        event.type === "BESSNodeRegistered" ||
        event.type === "AggregatorRegistered"
      );
    }
    if (filter === "DIRECT_QUERIES") {
      return (
        event.type === "DirectQuerySent" || event.type === "DirectQueryResponse"
      );
    }
    return event.type === filter;
  });

  // Get the most recent filtered events
  const recentEvents = filteredEvents.slice(-maxEvents).reverse();

  const formatEvent = (event: SystemEvent) => {
    const timestamp = new Date().toLocaleTimeString();

    switch (event.type) {
      case "AuctionStarted": {
        const data = event.data as AuctionStartedEvent;
        return {
          icon: "🎯",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "Auction Started",
          description: `Auction #${data.auction_id} - ${safeToFixed(
            data.total_energy
          )} kWh available at ${safePrice(data.reserve_price)}`,
        };
      }
      case "BidPlaced": {
        const data = event.data as BidPlacedEvent;
        return {
          icon: "💰",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Bid Placed",
          description: `Auction #${data.auction_id}: Aggregator ${data.aggregator_id} → BESS ${data.bess_id}`,
          details: `Bid: ${safePrice(data.bid_price)} for ${safeToFixed(
            data.energy_amount
          )} kWh`,
        };
      }
      case "BidAccepted": {
        const data = event.data as BidAcceptedEvent;
        return {
          icon: "✅",
          color: "text-emerald-600",
          bgColor: "bg-emerald-50 dark:bg-emerald-900/20",
          title: "Bid Accepted",
          description: `Auction #${data.auction_id}: Aggregator ${data.aggregator_id} → BESS ${data.bess_node_id}`,
          details: `Trade completed: ${safeToFixed(
            data.energy_amount
          )} kWh at ${safePrice(data.price)}`,
        };
      }
      case "BidRejected": {
        const data = event.data as BidRejectedEvent;
        return {
          icon: "❌",
          color: "text-red-600",
          bgColor: "bg-red-50 dark:bg-red-900/20",
          title: "Bid Rejected",
          description: `Aggregator ${data.aggregator_id} → BESS ${data.bess_id}`,
          details: `Rejected: ${data.reason}`,
        };
      }
      case "QuerySent": {
        const data = event.data as QuerySentEvent;
        return {
          icon: "❓",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "Query Sent",
          description: `Aggregator ${data.aggregator_id} → BESS-${data.bess_node_id}`,
          details: `Requesting energy availability`,
        };
      }
      case "QueryResponse": {
        const data = event.data as QueryResponseEvent;
        const totalCapacity = data.capacity_kwh;
        const currentEnergy = data.energy_available;
        const reserveEnergy = totalCapacity * 0.1; // Keep 10% reserve
        const availableForSale = Math.max(0, currentEnergy - reserveEnergy);
        const availablePercentage =
          totalCapacity > 0 ? (availableForSale / totalCapacity) * 100 : 0;

        return {
          icon: "📊",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Query Response",
          description: `BESS Node ${data.bess_node_id}`,
          details: `${safeToFixed(
            availableForSale
          )} kWh available for sale (${safeToFixed(
            availablePercentage,
            1
          )}% of capacity, ${safeToFixed(
            reserveEnergy,
            1
          )} kWh reserved) at ${safePrice(data.reserve_price)}`,
        };
      }
      case "AuctionCompleted": {
        const data = event.data as AuctionCompletedEvent;
        return {
          icon: "🏆",
          color: "text-purple-600",
          bgColor: "bg-purple-50 dark:bg-purple-900/20",
          title: "Auction Completed",
          description: `${data.winner} → BESS-${data.seller}`,
          details: `${safeToFixed(data.energy_amount)} kWh at ${safePrice(
            data.final_price
          )} (${(data.auction_duration_ms / 1000).toFixed(1)}s)`,
        };
      }
      case "EnergyDepleted": {
        const data = event.data as EnergyDepletedEvent;
        return {
          icon: "🔋",
          color: "text-red-600",
          bgColor: "bg-red-50 dark:bg-red-900/20",
          title: "Energy Depleted",
          description: `BESS Node ${data.bess_id}`,
          details: `Energy depleted! ${safeToFixed(
            data.final_energy
          )} kWh remaining (${safeToFixed(data.energy_percentage)}%)`,
        };
      }
      case "EnergyRecharged": {
        const data = event.data as EnergyRechargedEvent;
        return {
          icon: "⚡",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Energy Recharged",
          description: `BESS Node ${data.bess_id}`,
          details: `Recharged to ${safeToFixed(
            data.new_total
          )} kWh (${safeToFixed(data.energy_percentage)}%)`,
        };
      }
      case "SystemMetrics": {
        const data = event.data as SystemMetricsEvent;
        return {
          icon: "📊",
          color: "text-purple-600",
          bgColor: "bg-purple-50 dark:bg-purple-900/20",
          title: "System Update",
          description: `${data.total_auctions} auctions, ${
            data.total_bids
          } bids, ${safeToFixed(
            data.avg_price_improvement_percent
          )}% avg improvement`,
        };
      }
      case "BESSNodeStatus": {
        const data = event.data as any; // Use any to handle different data formats

        // Handle battery health - support both 0-3 and 80-100 ranges
        let healthStatus = "Unknown";
        if (data.battery_health !== undefined && data.battery_health !== null) {
          if (data.battery_health <= 3) {
            // 0-3 range from gateway
            healthStatus =
              data.battery_health === 0
                ? "Excellent"
                : data.battery_health === 1
                ? "Good"
                : data.battery_health === 2
                ? "Fair"
                : "Poor";
          } else {
            // 80-100 range from containers
            healthStatus =
              data.battery_health >= 95
                ? "Excellent"
                : data.battery_health >= 85
                ? "Good"
                : data.battery_health >= 75
                ? "Fair"
                : "Poor";
          }
        }

        // Extract device ID - handle various formats
        const deviceId = data.device_id || data.node_id || "Unknown";
        const nodeId = deviceId.toString().startsWith("BESS-")
          ? deviceId
          : `BESS-${deviceId}`;

        // Handle energy field - support both field names
        const energyAvailable = data.energy_available || data.energy_level || 0;

        return {
          icon: "🔋",
          color: "text-orange-600",
          bgColor: "bg-orange-50 dark:bg-orange-900/20",
          title: "BESS Status",
          description: `${nodeId}: ${data.is_online ? "Online" : "Offline"}`,
          details: `${safeToFixed(
            energyAvailable
          )} kWh available, Battery Health: ${healthStatus}`,
        };
      }
      case "AggregatorStatus": {
        const data = event.data as any; // Use any to handle different data formats

        // Extract device ID - handle various formats
        let deviceId = data.device_id || data.aggregator_id || "Unknown";
        if (!deviceId.toString().startsWith("AGG-") && deviceId !== "Unknown") {
          deviceId = `AGG-${deviceId}`;
        }

        // Handle different field names and missing data - support container vs gateway formats
        const successRate = data.success_rate ?? data.reputation_score ?? null;
        const totalBids =
          data.total_bids ??
          data.successful_bids ??
          data.pending_bids ??
          data.successful_settlements ??
          null;
        const avgPrice = data.average_bid_price ?? null;

        return {
          icon: "⚡",
          color: "text-indigo-600",
          bgColor: "bg-indigo-50 dark:bg-indigo-900/20",
          title: "Aggregator Update",
          description: `${deviceId}: ${data.strategy || "Unknown"} strategy`,
          details: `${safeToFixed(successRate)}% success rate, ${
            totalBids !== null ? totalBids : "N/A"
          } total bids, Avg: ${safePrice(avgPrice)}`,
        };
      }
      case "BESSNodeDiscovered": {
        const data = event.data as BESSNodeDiscoveredEvent;
        return {
          icon: "🔍",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "BESS Node Discovered",
          description: `BESS-${data.node_id} discovered via multicast`,
          details: `${safeToFixed(
            data.energy_level
          )} kWh available at ${safePrice(data.reserve_price)} from ${
            data.discovery_address
          }`,
        };
      }
      case "AggregatorDiscovered": {
        const data = event.data as AggregatorDiscoveredEvent;
        return {
          icon: "🔍",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "Aggregator Discovered",
          description: `AGG-${data.aggregator_id} discovered via multicast`,
          details: `${data.strategy} strategy, max bid ${safePrice(
            data.max_bid_price
          )} from ${data.discovery_address}`,
        };
      }
      case "HeartbeatReceived": {
        const data = event.data as HeartbeatReceivedEvent;
        return {
          icon: "💓",
          color: "text-purple-600",
          bgColor: "bg-purple-50 dark:bg-purple-900/20",
          title: "Heartbeat Received",
          description: `${data.node_type} ${data.node_id} heartbeat`,
          details: `From ${data.heartbeat_address} at ${new Date(
            data.timestamp * 1000
          ).toLocaleTimeString()}`,
        };
      }
      case "BESSNodeRegistered": {
        const data = event.data as BESSNodeRegisteredEvent;
        return {
          icon: "📝",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "BESS Node Registered",
          description: `BESS-${data.node_id} registered with gateway`,
          details: `${safeToFixed(
            data.energy_level
          )} kWh available at ${safePrice(data.reserve_price)}`,
        };
      }
      case "AggregatorRegistered": {
        const data = event.data as AggregatorRegisteredEvent;
        return {
          icon: "📝",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "Aggregator Registered",
          description: `AGG-${data.aggregator_id} registered with gateway`,
          details: `${data.strategy} strategy, max bid ${safePrice(
            data.max_bid_price
          )}`,
        };
      }
      case "DirectQuerySent": {
        const data = event.data as DirectQuerySentEvent;
        return {
          icon: "🔍",
          color: "text-purple-600",
          bgColor: "bg-purple-50 dark:bg-purple-900/20",
          title: "Direct Query Sent",
          description: `AGG-${data.aggregator_id} → BESS-${data.bess_node_id}`,
          details: `Query type: ${data.query_type}`,
        };
      }
      case "DirectQueryResponse": {
        const data = event.data as DirectQueryResponseEvent;
        return {
          icon: "📡",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Direct Query Response",
          description: `BESS-${data.bess_node_id} → AGG-${data.aggregator_id}`,
          details: `${safeToFixed(
            data.energy_available
          )} kWh available at ${safePrice(data.reserve_price)} (${
            data.response_time_ms
          }ms)`,
        };
      }
      case "BESSNodeDiscovered": {
        const data = event.data as BESSNodeDiscoveredEvent;
        return {
          icon: "🔍",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "BESS Node Discovered",
          description: `BESS-${data.node_id} discovered via multicast`,
          details: `${safeToFixed(
            data.energy_level
          )} kWh available at ${safePrice(data.reserve_price)} from ${
            data.discovery_address
          }`,
        };
      }
      case "AggregatorDiscovered": {
        const data = event.data as AggregatorDiscoveredEvent;
        return {
          icon: "⚡",
          color: "text-orange-600",
          bgColor: "bg-orange-50 dark:bg-orange-900/20",
          title: "Aggregator Discovered",
          description: `${data.aggregator_id} discovered via multicast`,
          details: `Strategy: ${data.strategy}, Max bid: ${safePrice(
            data.max_bid_price
          )}`,
        };
      }
      case "HeartbeatReceived": {
        const data = event.data as HeartbeatReceivedEvent;
        return {
          icon: "💓",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Heartbeat Received",
          description: `${data.node_type} ${data.node_id} is alive`,
          details: `Last seen: ${new Date(
            data.timestamp * 1000
          ).toLocaleTimeString()}`,
        };
      }
      case "INITIAL_DATA": {
        return {
          icon: "🔄",
          color: "text-blue-600",
          bgColor: "bg-blue-50 dark:bg-blue-900/20",
          title: "System Initialized",
          description: "Dashboard data refreshed",
          details: "All system components loaded successfully",
        };
      }
      default:
        return {
          icon: "📝",
          color: "text-gray-600",
          bgColor: "bg-gray-50 dark:bg-gray-900/20",
          title: "Event",
          description: "Unknown event type",
        };
    }
  };

  return (
    <div className="card">
      <div className="card-header">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <span className="text-lg">📡</span>
            <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
              Live Events
            </h3>
            <span className="px-2 py-1 text-xs font-medium bg-primary/10 text-primary rounded-full">
              {filteredEvents.length}
            </span>
          </div>
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          >
            {isExpanded ? (
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            ) : (
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            )}
          </button>
        </div>
      </div>

      {isExpanded && (
        <div className="card-content">
          {/* Filter Dropdown */}
          <div className="mb-4">
            <label
              htmlFor="event-filter"
              className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Filter Events
            </label>
            <select
              id="event-filter"
              value={filter}
              onChange={(e) => setFilter(e.target.value as EventFilter)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary dark:bg-gray-700 dark:text-white"
            >
              <option value="ALL">All Events ({events.length})</option>
              <option value="AuctionStarted">
                Auction Started (
                {events.filter((e) => e.type === "AuctionStarted").length})
              </option>
              <option value="BidPlaced">
                Bid Placed (
                {events.filter((e) => e.type === "BidPlaced").length})
              </option>
              <option value="BidAccepted">
                Bid Accepted (
                {events.filter((e) => e.type === "BidAccepted").length})
              </option>
              <option value="BidRejected">
                Bid Rejected (
                {events.filter((e) => e.type === "BidRejected").length})
              </option>
              <option value="QuerySent">
                Query Sent (
                {events.filter((e) => e.type === "QuerySent").length})
              </option>
              <option value="QueryResponse">
                Query Response (
                {events.filter((e) => e.type === "QueryResponse").length})
              </option>
              <option value="EnergyDepleted">
                Energy Depleted (
                {events.filter((e) => e.type === "EnergyDepleted").length})
              </option>
              <option value="EnergyRecharged">
                Energy Recharged (
                {events.filter((e) => e.type === "EnergyRecharged").length})
              </option>
              <option value="BESSNodeStatus">
                BESS Status (
                {events.filter((e) => e.type === "BESSNodeStatus").length})
              </option>
              <option value="AggregatorStatus">
                Aggregator Status (
                {events.filter((e) => e.type === "AggregatorStatus").length})
              </option>
              <option value="SystemMetrics">
                System Metrics (
                {events.filter((e) => e.type === "SystemMetrics").length})
              </option>
              <option value="BESSNodeDiscovered">
                BESS Discovered (
                {events.filter((e) => e.type === "BESSNodeDiscovered").length})
              </option>
              <option value="AggregatorDiscovered">
                Aggregator Discovered (
                {events.filter((e) => e.type === "AggregatorDiscovered").length}
                )
              </option>
              <option value="HeartbeatReceived">
                Heartbeats (
                {events.filter((e) => e.type === "HeartbeatReceived").length})
              </option>
              <option value="BESSNodeRegistered">
                BESS Registered (
                {events.filter((e) => e.type === "BESSNodeRegistered").length})
              </option>
              <option value="AggregatorRegistered">
                Aggregator Registered (
                {events.filter((e) => e.type === "AggregatorRegistered").length}
                )
              </option>
              <option value="REGISTRATIONS">
                All Registrations (
                {
                  events.filter(
                    (e) =>
                      e.type === "BESSNodeRegistered" ||
                      e.type === "AggregatorRegistered"
                  ).length
                }
                )
              </option>
              <option value="MULTICAST">
                Multicast Events (
                {
                  events.filter(
                    (e) =>
                      e.type === "BESSNodeDiscovered" ||
                      e.type === "AggregatorDiscovered" ||
                      e.type === "HeartbeatReceived"
                  ).length
                }
                )
              </option>
              <option value="QUERIES">
                Query Events (
                {
                  events.filter(
                    (e) => e.type === "QuerySent" || e.type === "QueryResponse"
                  ).length
                }
                )
              </option>
              <option value="DirectQuerySent">
                Direct Query Sent (
                {events.filter((e) => e.type === "DirectQuerySent").length})
              </option>
              <option value="DirectQueryResponse">
                Direct Query Response (
                {events.filter((e) => e.type === "DirectQueryResponse").length})
              </option>
              <option value="DIRECT_QUERIES">
                All Direct Queries (
                {
                  events.filter(
                    (e) =>
                      e.type === "DirectQuerySent" ||
                      e.type === "DirectQueryResponse"
                  ).length
                }
                )
              </option>
            </select>
          </div>

          <div className="space-y-3 max-h-96 overflow-y-auto">
            {recentEvents.length === 0 ? (
              <div className="text-center py-8 text-gray-500 dark:text-gray-400">
                <div className="text-4xl mb-2">📡</div>
                <p>No events yet. Waiting for live data...</p>
              </div>
            ) : (
              recentEvents.map((event, index) => {
                const eventInfo = formatEvent(event);
                return (
                  <div
                    key={index}
                    className={`p-3 rounded-lg border-l-4 ${
                      eventInfo.bgColor
                    } border-l-${
                      eventInfo.color.split("-")[1]
                    }-500 animate-fade-in`}
                  >
                    <div className="flex items-start space-x-3">
                      <span className="text-lg flex-shrink-0">
                        {eventInfo.icon}
                      </span>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between">
                          <h4
                            className={`text-sm font-medium ${eventInfo.color}`}
                          >
                            {eventInfo.title}
                          </h4>
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            {new Date().toLocaleTimeString()}
                          </span>
                        </div>
                        <p className="text-sm text-gray-700 dark:text-gray-300 mt-1">
                          {eventInfo.description}
                        </p>
                        {eventInfo.details && (
                          <p className="text-xs text-gray-600 dark:text-gray-400 mt-1 font-mono">
                            {eventInfo.details}
                          </p>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })
            )}
          </div>
        </div>
      )}
    </div>
  );
};
