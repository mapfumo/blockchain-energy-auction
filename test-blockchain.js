#!/usr/bin/env node

// Test script to demonstrate blockchain settlements
const WebSocket = require("ws");

console.log("🔗 Testing Blockchain Settlements...");

const ws = new WebSocket("ws://localhost:8080/ws");

ws.on("open", function open() {
  console.log("✅ Connected to WebSocket");
});

ws.on("message", function message(data) {
  try {
    const event = JSON.parse(data.toString());

    if (event.type === "BlockchainSettlement") {
      console.log("🎉 BLOCKCHAIN SETTLEMENT DETECTED!");
      console.log("📊 Auction ID:", event.data.auction_id);
      console.log("🏆 Winner:", event.data.winner);
      console.log("🔋 Seller:", event.data.seller);
      console.log("⚡ Energy:", event.data.energy_amount, "kWh");
      console.log("💰 Price:", event.data.final_price, "cents/kWh");
      console.log("🔗 Signature:", event.data.settlement_signature);
      console.log("🌐 Explorer URL:", event.data.blockchain_url);
      console.log("---");
    }

    if (event.type === "AuctionCompleted") {
      console.log("⚡ Auction Completed:", event.data.auction_id);
    }
  } catch (e) {
    // Ignore non-JSON messages
  }
});

ws.on("error", function error(err) {
  console.error("❌ WebSocket error:", err.message);
});

ws.on("close", function close() {
  console.log("🔌 WebSocket connection closed");
});

// Keep the script running
setTimeout(() => {
  console.log("⏰ Test completed after 30 seconds");
  process.exit(0);
}, 30000);
