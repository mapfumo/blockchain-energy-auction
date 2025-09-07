#!/usr/bin/env python3
"""
BESS (Battery Energy Storage System) Node
Simulates a real battery storage device with energy trading capabilities
"""

import os
import json
import time
import socket
import threading
import requests
import logging
from datetime import datetime
from typing import Dict, Any
import random

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('/app/logs/bess-node.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

class BESSNode:
    def __init__(self):
        self.node_id = os.getenv('NODE_ID', '001')
        self.node_type = os.getenv('NODE_TYPE', 'BESS')
        self.capacity_kwh = float(os.getenv('CAPACITY_KWH', '15'))
        self.energy_level = float(os.getenv('INITIAL_ENERGY', '12.5'))
        self.gateway_host = os.getenv('GATEWAY_HOST', 'gateway')
        self.gateway_port = int(os.getenv('GATEWAY_PORT', '8080'))
        self.multicast_group = os.getenv('MULTICAST_GROUP', '224.0.0.1')
        self.multicast_port = int(os.getenv('MULTICAST_PORT', '8888'))
        
        # BESS specific properties
        self.battery_health = 95.0
        self.voltage = 48.0  # 48V system
        self.temperature = 25.0
        self.charge_rate = 5.0  # kW
        self.discharge_rate = 5.0  # kW
        self.efficiency = 0.95
        
        # Trading properties
        self.reserve_price = random.randint(500, 800)  # cents/kWh
        self.is_online = True
        self.last_activity = datetime.now()
        
        # Network properties
        self.tcp_port = 8081
        self.udp_socket = None
        self.tcp_socket = None
        
        logger.info(f"BESS Node {self.node_id} initialized:")
        logger.info(f"  Capacity: {self.capacity_kwh} kWh")
        logger.info(f"  Energy Level: {self.energy_level} kWh")
        logger.info(f"  Reserve Price: {self.reserve_price}¢/kWh")
        logger.info(f"  Gateway: {self.gateway_host}:{self.gateway_port}")

    def start_multicast_discovery(self):
        """Start multicast discovery to announce BESS node"""
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
                # Send discovery message
                discovery_msg = {
                    'type': 'BESS_DISCOVERY',
                    'node_id': self.node_id,
                    'node_type': self.node_type,
                    'capacity_kwh': self.capacity_kwh,
                    'energy_level': self.energy_level,
                    'reserve_price': self.reserve_price,
                    'timestamp': datetime.now().isoformat()
                }
                
                message = json.dumps(discovery_msg).encode('utf-8')
                self.udp_socket.sendto(message, (self.multicast_group, self.multicast_port))
                
                logger.debug(f"Sent discovery message: {discovery_msg}")
                time.sleep(30)  # Send every 30 seconds
                
            except Exception as e:
                logger.error(f"Multicast discovery error: {e}")
                time.sleep(5)

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
        
        if msg_type == 'QUERY':
            return self._handle_query(message)
        elif msg_type == 'BID_ACCEPTED':
            return self._handle_bid_accepted(message)
        elif msg_type == 'BID_REJECTED':
            return self._handle_bid_rejected(message)
        elif msg_type == 'PING':
            return {'type': 'PONG', 'node_id': self.node_id}
        else:
            logger.warning(f"Unknown message type: {msg_type}")
            return None

    def _handle_query(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle energy query"""
        logger.info(f"Received energy query: {message}")
        
        # Update last activity
        self.last_activity = datetime.now()
        
        # Check if we have energy to sell
        if self.energy_level > 0.1:  # At least 0.1 kWh available
            return {
                'type': 'QUERY_RESPONSE',
                'node_id': self.node_id,
                'available_energy': self.energy_level,
                'reserve_price': self.reserve_price,
                'battery_health': self.battery_health,
                'voltage': self.voltage,
                'temperature': self.temperature,
                'timestamp': datetime.now().isoformat()
            }
        else:
            return {
                'type': 'QUERY_RESPONSE',
                'node_id': self.node_id,
                'available_energy': 0,
                'reason': 'Insufficient energy',
                'timestamp': datetime.now().isoformat()
            }

    def _handle_bid_accepted(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle accepted bid"""
        energy_sold = message.get('energy_amount', 0)
        price = message.get('price', 0)
        
        logger.info(f"Bid accepted: {energy_sold} kWh at {price}¢/kWh")
        
        # Update energy level
        self.energy_level = max(0, self.energy_level - energy_sold)
        
        # Update last activity
        self.last_activity = datetime.now()
        
        return {
            'type': 'BID_CONFIRMED',
            'node_id': self.node_id,
            'energy_sold': energy_sold,
            'price': price,
            'remaining_energy': self.energy_level,
            'timestamp': datetime.now().isoformat()
        }

    def _handle_bid_rejected(self, message: Dict[str, Any]) -> Dict[str, Any]:
        """Handle rejected bid"""
        reason = message.get('reason', 'Unknown')
        logger.info(f"Bid rejected: {reason}")
        
        return {
            'type': 'BID_REJECTED_ACK',
            'node_id': self.node_id,
            'reason': reason,
            'timestamp': datetime.now().isoformat()
        }

    def start_energy_simulation(self):
        """Start energy level simulation"""
        def energy_loop():
            while True:
                try:
                    # Simulate energy changes
                    if self.energy_level < self.capacity_kwh * 0.1:  # Below 10%
                        # Critical recharge - faster rate
                        self.energy_level = min(
                            self.capacity_kwh,
                            self.energy_level + 0.1  # 0.1 kWh per second
                        )
                        logger.info(f"Critical recharge: {self.energy_level:.2f} kWh")
                    elif self.energy_level < self.capacity_kwh * 0.8:  # Below 80%
                        # Normal recharge
                        self.energy_level = min(
                            self.capacity_kwh,
                            self.energy_level + 0.05  # 0.05 kWh per second
                        )
                    
                    # Update battery health (slowly degrade)
                    self.battery_health = max(80, self.battery_health - 0.001)
                    
                    # Update temperature (simulate)
                    self.temperature = 20 + random.uniform(-2, 5)
                    
                    # Update reserve price (slight variation)
                    self.reserve_price = max(400, min(1000, 
                        self.reserve_price + random.randint(-10, 10)
                    ))
                    
                    time.sleep(1)  # Update every second
                    
                except Exception as e:
                    logger.error(f"Energy simulation error: {e}")
                    time.sleep(1)
        
        energy_thread = threading.Thread(target=energy_loop)
        energy_thread.daemon = True
        energy_thread.start()

    def start_health_monitoring(self):
        """Start health monitoring and reporting"""
        def health_loop():
            while True:
                try:
                    # Send status to gateway
                    status = {
                        'type': 'BESS_STATUS',
                        'node_id': self.node_id,
                        'energy_level': self.energy_level,
                        'capacity_kwh': self.capacity_kwh,
                        'battery_health': self.battery_health,
                        'voltage': self.voltage,
                        'temperature': self.temperature,
                        'reserve_price': self.reserve_price,
                        'is_online': self.is_online,
                        'last_activity': self.last_activity.isoformat(),
                        'timestamp': datetime.now().isoformat()
                    }
                    
                    # Send to gateway via HTTP
                    try:
                        response = requests.post(
                            f'http://{self.gateway_host}:3001/api/bess-status',
                            json=status,
                            timeout=5
                        )
                        if response.status_code == 200:
                            logger.debug("Status sent to gateway")
                        else:
                            logger.warning(f"Gateway response: {response.status_code}")
                    except Exception as e:
                        logger.debug(f"Gateway communication error: {e}")
                    
                    time.sleep(10)  # Send every 10 seconds
                    
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
                        'node_id': self.node_id,
                        'energy_level': self.energy_level,
                        'battery_health': self.battery_health,
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
            server = HTTPServer(('0.0.0.0', 8081), HealthHandler)
            logger.info("HTTP health endpoint started on port 8081")
            
            # Start HTTP server in separate thread
            http_thread = threading.Thread(target=server.serve_forever)
            http_thread.daemon = True
            http_thread.start()
            
        except Exception as e:
            logger.error(f"Failed to start HTTP health endpoint: {e}")

    def run(self):
        """Main run loop"""
        logger.info(f"Starting BESS Node {self.node_id}")
        
        # Start all services
        self.start_multicast_discovery()
        self.start_tcp_server()
        self.start_energy_simulation()
        self.start_health_monitoring()
        self.start_http_health_endpoint()
        
        logger.info("BESS Node started successfully")
        
        # Keep main thread alive
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            logger.info("Shutting down BESS Node")
            self.is_online = False

if __name__ == '__main__':
    bess_node = BESSNode()
    bess_node.run()
