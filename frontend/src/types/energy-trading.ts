// Energy Trading System Types
// Based on the Rust backend ETP message protocol and system events

export interface ETPMessage {
  message_id: number;
  device_id: number;
  message_type: number;
  source_address: string;
  destination_address: string;
  multicast_address: string;
  bid_price: number;
  required_energy_amount: number;
  total_energy_available: number;
  percentage_for_sale: number;
  battery_voltage: number;
  timeout: number;
  ttl: number;
  timestamp: string;
}

export interface BESSNode {
  device_id: number;
  name: string;
  capacity: number;
  current_energy_level: number;
  reserve_price: number;
  percentage_for_sale: number;
  battery_voltage: number;
  max_discharge_rate: number;
  is_online: boolean;
  last_updated: string;
}

export interface AggregatorNode {
  device_id: number;
  name: string;
  strategy: BiddingStrategy;
  is_online: boolean;
  success_rate: number;
  total_bids: number;
  average_bid_price: number;
  last_updated: string;
}

export type BiddingStrategy =
  | "Random"
  | "Conservative"
  | "Aggressive"
  | "Intelligent";

export interface SystemEvent {
  type:
    | "AuctionStarted"
    | "BidPlaced"
    | "BidAccepted"
    | "BidRejected"
    | "SystemMetrics"
    | "BESSNodeStatus"
    | "AggregatorStatus";
  data:
    | AuctionStartedEvent
    | BidPlacedEvent
    | BidAcceptedEvent
    | BidRejectedEvent
    | SystemMetricsEvent
    | BESSNodeStatusEvent
    | AggregatorStatusEvent;
  timestamp: string;
}

export interface AuctionStartedEvent {
  auction_id: number;
  total_energy: number;
  reserve_price: number;
}

export interface BidPlacedEvent {
  auction_id: number;
  aggregator_id: number;
  bess_id: number;
  bid_price: number;
  energy_amount: number;
}

export interface BidAcceptedEvent {
  auction_id: number;
  aggregator_id: number;
  bess_id: number;
  final_price: number;
  energy_amount: number;
}

export interface BidRejectedEvent {
  aggregator_id: number;
  bess_id: number;
  reason: string;
}

export interface SystemMetricsEvent {
  total_auctions: number;
  total_bids: number;
  avg_price_improvement_percent: number;
  active_bess_nodes: number;
  active_aggregators: number;
}

export interface BESSNodeStatusEvent {
  device_id: number;
  energy_available: number;
  battery_health: number;
  is_online: boolean;
}

export interface AggregatorStatusEvent {
  device_id: number;
  strategy: string;
  success_rate: number;
  total_bids: number;
}

export interface SystemMetrics {
  total_events_broadcast: number;
  connected_clients: number;
  average_events_per_second: number;
  total_auctions: number;
  total_bids: number;
  avg_price_improvement_percent: number;
  active_bess_nodes: number;
  active_aggregators: number;
}

export interface AuctionData {
  id: number;
  start_time: string;
  total_energy: number;
  reserve_price: number;
  current_highest_bid: number;
  current_lowest_bid: number;
  total_bids: number;
  status: "active" | "completed" | "cancelled";
  bess_nodes: BESSNode[];
  aggregators: AggregatorNode[];
}

export interface PriceHistory {
  timestamp: string;
  price: number;
  energy_amount: number;
  aggregator_id: number;
  bess_id: number;
}

export interface WebSocketConnection {
  isConnected: boolean;
  lastMessage: SystemEvent | null;
  error: string | null;
  reconnectAttempts: number;
}
