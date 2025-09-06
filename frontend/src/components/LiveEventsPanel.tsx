import React, { useState, useEffect } from "react";
import {
  SystemEvent,
  AuctionStartedEvent,
  BidPlacedEvent,
  BidAcceptedEvent,
  BidRejectedEvent,
  QuerySentEvent,
  QueryResponseEvent,
  EnergyDepletedEvent,
  EnergyRechargedEvent,
  SystemMetricsEvent,
  BESSNodeStatusEvent,
  AggregatorStatusEvent,
} from "../types/energy-trading";

interface LiveEventsPanelProps {
  events: SystemEvent[];
  maxEvents?: number;
}

type EventFilter =
  | "ALL"
  | "AuctionStarted"
  | "BidPlaced"
  | "BidAccepted"
  | "BidRejected"
  | "QuerySent"
  | "QueryResponse"
  | "EnergyDepleted"
  | "EnergyRecharged"
  | "BESSNodeStatus"
  | "AggregatorStatus"
  | "SystemMetrics";

export const LiveEventsPanel: React.FC<LiveEventsPanelProps> = ({
  events,
  maxEvents = 50,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [filter, setFilter] = useState<EventFilter>("ALL");

  // Filter events based on selected filter
  const filteredEvents = events.filter((event) => {
    if (filter === "ALL") return true;
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
          description: `Auction #${
            data.auction_id
          } - ${data.total_energy.toFixed(
            1
          )} kWh available at ${data.reserve_price.toFixed(1)}¢/kWh`,
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
          details: `Bid: ${data.bid_price.toFixed(
            1
          )}¢/kWh for ${data.energy_amount.toFixed(1)} kWh`,
        };
      }
      case "BidAccepted": {
        const data = event.data as BidAcceptedEvent;
        return {
          icon: "✅",
          color: "text-emerald-600",
          bgColor: "bg-emerald-50 dark:bg-emerald-900/20",
          title: "Bid Accepted",
          description: `Auction #${data.auction_id}: Aggregator ${data.aggregator_id} → BESS ${data.bess_id}`,
          details: `Trade completed: ${data.energy_amount.toFixed(
            1
          )} kWh at ${data.final_price.toFixed(1)}¢/kWh`,
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
          description: `Aggregator ${data.aggregator_id} → BESS ${data.bess_id}`,
          details: `Requesting energy availability`,
        };
      }
      case "QueryResponse": {
        const data = event.data as QueryResponseEvent;
        return {
          icon: "📊",
          color: "text-green-600",
          bgColor: "bg-green-50 dark:bg-green-900/20",
          title: "Query Response",
          description: `BESS Node ${data.bess_id}`,
          details: `${data.energy_available.toFixed(
            1
          )} kWh available (${data.percentage_for_sale.toFixed(0)}% for sale)`,
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
          details: `Energy depleted! ${data.final_energy.toFixed(1)} kWh remaining (${data.energy_percentage.toFixed(1)}%)`,
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
          details: `Recharged to ${data.new_total.toFixed(1)} kWh (${data.energy_percentage.toFixed(1)}%)`,
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
          } bids, ${data.avg_price_improvement_percent.toFixed(
            1
          )}% avg improvement`,
        };
      }
      case "BESSNodeStatus": {
        const data = event.data as BESSNodeStatusEvent;
        const healthStatus =
          data.battery_health === 0
            ? "Excellent"
            : data.battery_health === 1
            ? "Good"
            : "Fair";
        return {
          icon: "🔋",
          color: "text-orange-600",
          bgColor: "bg-orange-50 dark:bg-orange-900/20",
          title: "BESS Status",
          description: `BESS Node ${data.device_id}: ${
            data.is_online ? "Online" : "Offline"
          }`,
          details: `${data.energy_available.toFixed(
            1
          )} kWh available, Battery Health: ${healthStatus}`,
        };
      }
      case "AggregatorStatus": {
        const data = event.data as AggregatorStatusEvent;
        return {
          icon: "⚡",
          color: "text-indigo-600",
          bgColor: "bg-indigo-50 dark:bg-indigo-900/20",
          title: "Aggregator Update",
          description: `Aggregator ${data.device_id}: ${data.strategy} strategy`,
          details: `${data.success_rate.toFixed(1)}% success rate, ${
            data.total_bids
          } total bids, Avg: ${data.average_bid_price.toFixed(1)}¢/kWh`,
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
