// Energy Trading System Types
// Based on the Rust backend ETP message protocol and system events

export interface ETPMessage {
  message_id: number;
  device_id: number;
  message_type: number;
  source_address: string;
  destination_address: string;
  multicast_address: string;
  bid_price: number; // Price in cents/kWh
  required_energy_amount: number; // Energy in kWh
  total_energy_available: number; // Energy in kWh
  percentage_for_sale: number; // Percentage (0-100)
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
  reserve_price: number; // Price in cents/kWh
  percentage_for_sale: number; // Percentage (0-100)
  battery_voltage: number;
  max_discharge_rate: number;
  battery_health: number; // 0=Excellent, 1=Good, 2=Fair, 3=Poor
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
  successful_bids: number;
  total_energy_bought: number; // Total energy bought in kWh
  average_bid_price: number; // Price in cents/kWh
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
    | "QuerySent"
    | "QueryResponse"
    | "EnergyDepleted"
    | "EnergyRecharged"
    | "SystemMetrics"
    | "BESSNodeStatus"
    | "AggregatorStatus";
  data:
    | AuctionStartedEvent
    | BidPlacedEvent
    | BidAcceptedEvent
    | BidRejectedEvent
    | QuerySentEvent
    | QueryResponseEvent
    | EnergyDepletedEvent
    | EnergyRechargedEvent
    | SystemMetricsEvent
    | BESSNodeStatusEvent
    | AggregatorStatusEvent;
  timestamp: string;
}

export interface AuctionStartedEvent {
  auction_id: number;
  total_energy: number; // Energy in kWh
  reserve_price: number; // Price in cents/kWh
}

export interface BidPlacedEvent {
  auction_id: number;
  aggregator_id: number;
  bess_id: number;
  bid_price: number; // Price in cents/kWh
  energy_amount: number; // Energy in kWh
}

export interface BidAcceptedEvent {
  auction_id: number;
  aggregator_id: number;
  bess_id: number;
  final_price: number; // Price in cents/kWh
  energy_amount: number; // Energy in kWh
}

export interface BidRejectedEvent {
  aggregator_id: number;
  bess_id: number;
  reason: string;
}

export interface QuerySentEvent {
  aggregator_id: number;
  bess_id: number;
}

export interface QueryResponseEvent {
  bess_id: number;
  energy_available: number; // Energy in kWh
  percentage_for_sale: number; // Percentage (0-100)
}

export interface EnergyDepletedEvent {
  bess_id: number;
  final_energy: number; // Final energy level in kWh
  energy_percentage: number; // Energy percentage (0-100)
}

export interface EnergyRechargedEvent {
  bess_id: number;
  energy_added: number; // Energy added in kWh
  new_total: number; // New total energy in kWh
  energy_percentage: number; // Energy percentage (0-100)
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
  successful_bids: number;
  total_energy_bought: number; // Total energy bought in kWh
  average_bid_price: number; // Price in cents/kWh
  is_online: boolean; // Online status
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
  total_energy: number; // Energy in kWh
  reserve_price: number; // Price in cents/kWh
  current_highest_bid: number; // Price in cents/kWh
  current_lowest_bid: number; // Price in cents/kWh
  total_bids: number;
  status: "active" | "completed" | "cancelled";
  bess_nodes: BESSNode[];
  aggregators: AggregatorNode[];
}

export interface PriceHistory {
  timestamp: string;
  price: number; // Price in cents/kWh
  energy_amount: number; // Energy in kWh
  aggregator_id: number;
  bess_id: number;
}

export interface WebSocketConnection {
  isConnected: boolean;
  lastMessage: SystemEvent | null;
  error: string | null;
  reconnectAttempts: number;
}
