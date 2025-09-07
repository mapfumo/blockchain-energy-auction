#!/usr/bin/env python3
"""
Aggregator Node
Simulates an energy aggregator that buys energy from BESS nodes
"""

import os
import json
import time
import socket
import threading
import requests
import logging
from datetime import datetime
from typing import Dict, Any, List
import random

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('/app/logs/aggregator.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

class Aggregator:
    def __init__(self):
        self.aggregator_id = os.getenv('AGGREGATOR_ID', '001')
        self.node_type = os.getenv('NODE_TYPE', 'AGGREGATOR')
        self.strategy = os.getenv('STRATEGY', 'CONSERVATIVE')
        self.max_bid_price = int(os.getenv('MAX_BID_PRICE', '800'))
        self.gateway_host = os.getenv('GATEWAY_HOST', 'gateway')
        self.gateway_port = int(os.getenv('GATEWAY_PORT', '8080'))
        self.multicast_group = os.getenv('MULTICAST_GROUP', '224.0.0.1')
        self.multicast_port = int(os.getenv('MULTICAST_PORT', '8888'))
        
        # Aggregator properties
        self.reputation_score = 50
        self.successful_settlements = 0
        self.total_energy_traded = 0
        self.total_usdc_paid = 0
        self.is_online = True
        self.last_activity = datetime.now()
        
        # Trading properties
        self.available_bess_nodes = {}  # node_id -> node_info
        self.pending_bids = {}  # auction_id -> bid_info
        self.completed_trades = []
        
        # Network properties
        self.tcp_port = 8082
        self.udp_socket = None
        self.tcp_socket = None
        
        logger.info(f"Aggregator {self.aggregator_id} initialized:")
        logger.info(f"  Strategy: {self.strategy}")
        logger.info(f"  Max Bid Price: {self.max_bid_price}¢/kWh")
        logger.info(f"  Gateway: {self.gateway_host}:{self.gateway_port}")

    def start_multicast_discovery(self):
        """Start multicast discovery to find BESS nodes"""
        try:
            self.udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            self.udp_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            
            # Join multicast group
            mreq = socket.inet_aton(self.multicast_group) + socket.inet_aton('0.0.0.0')
            self.udp_socket.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, mreq)
            
            # Bind to multicast port
            self.udp_socket.bind(('', self.multicast_port))
            
            logger.info(f"Joined multicast group {self.multicast_group}:{self.multicast_port}")
            
            # Start discovery thread
            discovery_thread = threading.Thread(target=self._multicast_discovery_loop)
            discovery_thread.daemon = True
            discovery_thread.start()
            
        except Exception as e:
            logger.error(f"Failed to start multicast discovery: {e}")

    def _multicast_discovery_loop(self):
        """Multicast discovery loop"""
        while True:
            try:
                # Listen for BESS node announcements
                data, address = self.udp_socket.recvfrom(1024)
                
                try:
                    message = json.loads(data.decode('utf-8'))
                    
                    if message.get('type') == 'BESS_DISCOVERY':
                        self._handle_bess_discovery(message, address)
                        
                except json.JSONDecodeError:
                    logger.debug(f"Invalid JSON from {address}")
                    
            except Exception as e:
                logger.error(f"Multicast discovery error: {e}")
                time.sleep(1)

    def _handle_bess_discovery(self, message: Dict[str, Any], address):
        """Handle BESS node discovery"""
        node_id = message.get('node_id')
        
        if node_id:
            self.available_bess_nodes[node_id] = {
                'node_id': node_id,
                'node_type': message.get('node_type'),
                'capacity_kwh': message.get('capacity_kwh'),
                'energy_level': message.get('energy_level'),
                'reserve_price': message.get('reserve_price'),
                'address': address[0],
                'last_seen': datetime.now(),
                'timestamp': message.get('timestamp')
            }
            
            logger.info(f"Discovered BESS node {node_id}: {message.get('energy_level')} kWh available")
            
            # Send discovery response
            response = {
                'type': 'AGGREGATOR_DISCOVERY',
                'aggregator_id': self.aggregator_id,
                'strategy': self.strategy,
                'max_bid_price': self.max_bid_price,
                'reputation_score': self.reputation_score,
                'timestamp': datetime.now().isoformat()
            }
            
            try:
                self.udp_socket.sendto(
                    json.dumps(response).encode('utf-8'),
                    address
                )
            except Exception as e:
                logger.error(f"Failed to send discovery response: {e}")

    def start_tcp_server(self):
        """Start TCP server for direct communication"""
        try:
            self.tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.tcp_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.tcp_socket.bind(('0.0.0.0', self.tcp_port))
            self.tcp_socket.listen(5)
            
            logger.info(f"TCP server started on port {self.tcp_port}")
            
            # Start TCP server thread
            tcp_thread = threading.Thread(target=self._tcp_server_loop)
            tcp_thread.daemon = True
            tcp_thread.start()
            
        except Exception as e:
            logger.error(f"Failed to start TCP server: {e}")

    def _tcp_server_loop(self):
        """TCP server loop"""
        while True:
            try:
                client_socket, address = self.tcp_socket.accept()
                logger.info(f"TCP connection from {address}")
                
                # Handle client in separate thread
                client_thread = threading.Thread(
                    target=self._handle_tcp_client,
                    args=(client_socket, address)
                )
                client_thread.daemon = True
                client_thread.start()
                
            except Exception as e:
                logger.error(f"TCP server error: {e}")
                time.sleep(1)

    def _handle_tcp_client(self, client_socket, address):
        """Handle TCP client connection"""
        try:
            while True:
                data = client_socket.recv(1024)
                if not data:
                    break
                
                try:
                    message = json.loads(data.decode('utf-8'))
                    response = self._process_message(message)
                    
                    if response:
                        client_socket.send(json.dumps(response).encode('utf-8'))
                        
                except json.JSONDecodeError:
                    logger.error(f"Invalid JSON from {address}")
                    
        except Exception as e:
            logger.error(f"TCP client error: {e}")
        finally:
            client_socket.close()
            logger.info(f"TCP connection closed: {address}")

    def _process_message(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Process incoming message"""
        msg_type = message.get('type')
        
        if msg_type == 'AUCTION_STARTED':
            return self._handle_auction_started(message)
        elif msg_type == 'BID_ACCEPTED':
            return self._handle_bid_accepted(message)
        elif msg_type == 'BID_REJECTED':
            return self._handle_bid_rejected(message)
        elif msg_type == 'PING':
            return {'type': 'PONG', 'aggregator_id': self.aggregator_id}
        else:
            logger.warning(f"Unknown message type: {msg_type}")
            return None

    def _handle_auction_started(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle auction started message"""
        auction_id = message.get('auction_id')
        total_energy = message.get('total_energy')
        reserve_price = message.get('reserve_price')
        
        logger.info(f"Auction {auction_id} started: {total_energy} kWh at {reserve_price}¢/kWh")
        
        # Update last activity
        self.last_activity = datetime.now()
        
        # Decide whether to bid based on strategy
        bid_price = self._calculate_bid_price(reserve_price)
        
        if bid_price > 0:
            # Place bid
            bid_response = self._place_bid(auction_id, bid_price, total_energy)
            return bid_response
        else:
            logger.info(f"Not bidding on auction {auction_id} (strategy: {self.strategy})")
            return None

    def _calculate_bid_price(self, reserve_price: float) -> int:
        """Calculate bid price based on strategy"""
        if self.strategy == 'CONSERVATIVE':
            # Conservative: bid close to reserve price
            return int(reserve_price * random.uniform(1.05, 1.15))
        elif self.strategy == 'AGGRESSIVE':
            # Aggressive: bid higher to win
            return int(reserve_price * random.uniform(1.20, 1.40))
        elif self.strategy == 'OPPORTUNISTIC':
            # Opportunistic: bid only if price is very low
            if reserve_price < 600:  # Less than 6¢/kWh
                return int(reserve_price * random.uniform(1.10, 1.25))
            else:
                return 0
        else:
            return 0

    def _place_bid(self, auction_id: str, bid_price: int, total_energy: float) -> Dict[str, Any]:
        """Place a bid on an auction"""
        bid_amount = int(total_energy * bid_price)  # Total bid amount in cents
        
        # Check if we can afford this bid
        if bid_amount > self.max_bid_price * total_energy:
            logger.info(f"Bid too expensive: {bid_amount} > {self.max_bid_price * total_energy}")
            return None
        
        bid_info = {
            'auction_id': auction_id,
            'bid_price': bid_price,
            'bid_amount': bid_amount,
            'total_energy': total_energy,
            'timestamp': datetime.now().isoformat()
        }
        
        self.pending_bids[auction_id] = bid_info
        
        logger.info(f"Placing bid on auction {auction_id}: {bid_price}¢/kWh (${bid_amount/100:.2f})")
        
        return {
            'type': 'BID_PLACED',
            'aggregator_id': self.aggregator_id,
            'auction_id': auction_id,
            'bid_price': bid_price,
            'bid_amount': bid_amount,
            'total_energy': total_energy,
            'timestamp': datetime.now().isoformat()
        }

    def _handle_bid_accepted(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle accepted bid"""
        auction_id = message.get('auction_id')
        energy_amount = message.get('energy_amount', 0)
        final_price = message.get('final_price', 0)
        
        logger.info(f"Bid accepted for auction {auction_id}: {energy_amount} kWh at {final_price}¢/kWh")
        
        # Update aggregator stats
        self.successful_settlements += 1
        self.total_energy_traded += energy_amount
        self.total_usdc_paid += int(energy_amount * final_price)
        self.reputation_score = min(100, self.reputation_score + 1)
        
        # Record completed trade
        trade = {
            'auction_id': auction_id,
            'energy_amount': energy_amount,
            'price': final_price,
            'total_cost': int(energy_amount * final_price),
            'timestamp': datetime.now().isoformat()
        }
        self.completed_trades.append(trade)
        
        # Remove from pending bids
        if auction_id in self.pending_bids:
            del self.pending_bids[auction_id]
        
        # Update last activity
        self.last_activity = datetime.now()
        
        return {
            'type': 'BID_ACCEPTED_ACK',
            'aggregator_id': self.aggregator_id,
            'auction_id': auction_id,
            'energy_amount': energy_amount,
            'final_price': final_price,
            'timestamp': datetime.now().isoformat()
        }

    def _handle_bid_rejected(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle rejected bid"""
        auction_id = message.get('auction_id')
        reason = message.get('reason', 'Unknown')
        
        logger.info(f"Bid rejected for auction {auction_id}: {reason}")
        
        # Remove from pending bids
        if auction_id in self.pending_bids:
            del self.pending_bids[auction_id]
        
        return {
            'type': 'BID_REJECTED_ACK',
            'aggregator_id': self.aggregator_id,
            'auction_id': auction_id,
            'reason': reason,
            'timestamp': datetime.now().isoformat()
        }

    def start_health_monitoring(self):
        """Start health monitoring and reporting"""
        def health_loop():
            while True:
                try:
                    # Send status to gateway
                    status = {
                        'type': 'AGGREGATOR_STATUS',
                        'aggregator_id': self.aggregator_id,
                        'strategy': self.strategy,
                        'reputation_score': self.reputation_score,
                        'successful_settlements': self.successful_settlements,
                        'total_energy_traded': self.total_energy_traded,
                        'total_usdc_paid': self.total_usdc_paid,
                        'available_bess_nodes': len(self.available_bess_nodes),
                        'pending_bids': len(self.pending_bids),
                        'is_online': self.is_online,
                        'last_activity': self.last_activity.isoformat(),
                        'timestamp': datetime.now().isoformat()
                    }
                    
                    # Send to gateway via HTTP
                    try:
                        response = requests.post(
                            f'http://{self.gateway_host}:3001/api/aggregator-status',
                            json=status,
                            timeout=5
                        )
                        if response.status_code == 200:
                            logger.debug("Status sent to gateway")
                        else:
                            logger.warning(f"Gateway response: {response.status_code}")
                    except Exception as e:
                        logger.debug(f"Gateway communication error: {e}")
                    
                    time.sleep(15)  # Send every 15 seconds
                    
                except Exception as e:
                    logger.error(f"Health monitoring error: {e}")
                    time.sleep(5)
        
        health_thread = threading.Thread(target=health_loop)
        health_thread.daemon = True
        health_thread.start()

    def start_http_health_endpoint(self):
        """Start HTTP health check endpoint"""
        from http.server import HTTPServer, BaseHTTPRequestHandler
        import json
        
        class HealthHandler(BaseHTTPRequestHandler):
            def do_GET(self):
                if self.path == '/health':
                    health_data = {
                        'status': 'healthy',
                        'aggregator_id': self.aggregator_id,
                        'strategy': self.strategy,
                        'reputation_score': self.reputation_score,
                        'successful_settlements': self.successful_settlements,
                        'is_online': self.is_online
                    }
                    
                    self.send_response(200)
                    self.send_header('Content-type', 'application/json')
                    self.end_headers()
                    self.wfile.write(json.dumps(health_data).encode())
                else:
                    self.send_response(404)
                    self.end_headers()
        
        try:
            server = HTTPServer(('0.0.0.0', 8082), HealthHandler)
            logger.info("HTTP health endpoint started on port 8082")
            
            # Start HTTP server in separate thread
            http_thread = threading.Thread(target=server.serve_forever)
            http_thread.daemon = True
            http_thread.start()
            
        except Exception as e:
            logger.error(f"Failed to start HTTP health endpoint: {e}")

    def run(self):
        """Main run loop"""
        logger.info(f"Starting Aggregator {self.aggregator_id}")
        
        # Start all services
        self.start_multicast_discovery()
        self.start_tcp_server()
        self.start_health_monitoring()
        self.start_http_health_endpoint()
        
        logger.info("Aggregator started successfully")
        
        # Keep main thread alive
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            logger.info("Shutting down Aggregator")
            self.is_online = False

if __name__ == '__main__':
    aggregator = Aggregator()
    aggregator.run()
