const WebSocket = require("ws");

console.log("🔌 Testing WebSocket connection to ws://localhost:8080/ws");

const ws = new WebSocket("ws://localhost:8080/ws");

ws.on("open", function open() {
  console.log("✅ WebSocket connected successfully!");

  // Send a test message
  ws.send(
    JSON.stringify({
      type: "test",
      message: "Hello from Node.js test",
    })
  );
});

ws.on("message", function message(data) {
  console.log("📨 Received:", data.toString());
});

ws.on("error", function error(err) {
  console.error("❌ WebSocket error:", err);
});

ws.on("close", function close(code, reason) {
  console.log("🔌 WebSocket closed:", code, reason.toString());
});

// Close after 5 seconds
setTimeout(() => {
  console.log("🔌 Closing WebSocket connection...");
  ws.close();
}, 5000);
