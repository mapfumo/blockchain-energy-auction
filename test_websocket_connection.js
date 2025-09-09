const WebSocket = require("ws");

console.log("Testing WebSocket connection to ws://localhost:8080/ws");

const ws = new WebSocket("ws://localhost:8080/ws");

ws.on("open", function open() {
  console.log("✅ WebSocket connected successfully!");
  console.log("Waiting for events...");
});

ws.on("message", function message(data) {
  try {
    const event = JSON.parse(data);
    console.log("📡 Received event:", Object.keys(event)[0] || "unknown");
  } catch (e) {
    console.log(
      "📡 Received raw data:",
      data.toString().substring(0, 100) + "..."
    );
  }
});

ws.on("error", function error(err) {
  console.error("❌ WebSocket error:", err.message);
});

ws.on("close", function close() {
  console.log("🔌 WebSocket connection closed");
});

// Keep the connection alive for 10 seconds
setTimeout(() => {
  console.log("🔄 Closing connection after 10 seconds...");
  ws.close();
  process.exit(0);
}, 10000);
