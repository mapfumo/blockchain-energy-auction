const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8080/ws');

ws.on('open', function open() {
  console.log('WebSocket connected');
});

ws.on('message', function message(data) {
  try {
    const event = JSON.parse(data);
    console.log('Received event:', JSON.stringify(event, null, 2));
    
    if (event.BESSNodeStatus) {
      console.log('BESS Status data:', event.BESSNodeStatus);
    }
    if (event.AggregatorStatus) {
      console.log('Aggregator Status data:', event.AggregatorStatus);
    }
  } catch (e) {
    console.log('Raw message:', data.toString());
  }
});

ws.on('error', function error(err) {
  console.error('WebSocket error:', err);
});

// Keep the script running for 30 seconds
setTimeout(() => {
  ws.close();
  process.exit(0);
}, 30000);
