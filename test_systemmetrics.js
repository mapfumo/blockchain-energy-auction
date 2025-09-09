const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8080/ws');

ws.on('open', function open() {
  console.log('WebSocket connected - waiting for SystemMetrics...');
});

ws.on('message', function message(data) {
  try {
    const event = JSON.parse(data);
    
    if (event.data && event.type === 'SystemMetrics') {
      console.log('\n=== SystemMetrics Event ===');
      console.log(JSON.stringify(event.data, null, 2));
      console.log('==========================\n');
      ws.close();
      process.exit(0);
    }
  } catch (e) {
    // Ignore parsing errors
  }
});

// Timeout after 15 seconds
setTimeout(() => {
  console.log('Timeout waiting for SystemMetrics event');
  ws.close();
  process.exit(1);
}, 15000);
