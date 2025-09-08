use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{info, error};

mod blockchain;
mod settlement_data;

use blockchain::BlockchainClient;
use settlement_data::{SettlementStatus, AuctionSettlement, AggregatorStatus, BatteryStatus};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpResource {
    uri: String,
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResourcesResult {
    resources: Vec<McpResource>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListToolsResult {
    tools: Vec<McpTool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadResourceParams {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CallToolParams {
    name: String,
    arguments: Option<serde_json::Value>,
}

struct McpServer {
    blockchain_client: BlockchainClient,
    settlement_cache: HashMap<String, SettlementStatus>,
}

impl McpServer {
    fn new() -> Result<Self> {
        let blockchain_client = BlockchainClient::new()?;
        Ok(Self {
            blockchain_client,
            settlement_cache: HashMap::new(),
        })
    }

    async fn handle_request(&mut self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        let JsonRpcRequest { id, method, params } = request;
        match method.as_str() {
            "initialize" => {
                info!("MCP client initialized");
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "resources": {},
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "energy-trading-settlement-mcp",
                            "version": "0.1.0"
                        }
                    })),
                    error: None,
                })
            }
            "resources/list" => {
                let resources = vec![
                    McpResource {
                        uri: "settlement://auctions".to_string(),
                        name: "Auction Settlements".to_string(),
                        description: Some("All auction settlement data".to_string()),
                        mime_type: Some("application/json".to_string()),
                    },
                    McpResource {
                        uri: "settlement://aggregators".to_string(),
                        name: "Aggregator Status".to_string(),
                        description: Some("Aggregator performance and settlement data".to_string()),
                        mime_type: Some("application/json".to_string()),
                    },
                    McpResource {
                        uri: "settlement://batteries".to_string(),
                        name: "Battery Status".to_string(),
                        description: Some("Battery settlement and earnings data".to_string()),
                        mime_type: Some("application/json".to_string()),
                    },
                    McpResource {
                        uri: "settlement://recent".to_string(),
                        name: "Recent Settlements".to_string(),
                        description: Some("Recently completed settlements".to_string()),
                        mime_type: Some("application/json".to_string()),
                    },
                ];
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::to_value(ListResourcesResult { resources })?),
                    error: None,
                })
            }
            "resources/read" => {
                let params: ReadResourceParams = serde_json::from_value(
                    params.unwrap_or(serde_json::Value::Null)
                )?;
                
                let data: serde_json::Value = match params.uri.as_str() {
                    "settlement://auctions" => {
                        serde_json::to_value(self.get_all_auction_settlements().await?)?
                    }
                    "settlement://aggregators" => {
                        serde_json::to_value(self.get_all_aggregator_status().await?)?
                    }
                    "settlement://batteries" => {
                        serde_json::to_value(self.get_all_battery_status().await?)?
                    }
                    "settlement://recent" => {
                        serde_json::to_value(self.get_recent_settlements().await?)?
                    }
                    _ => {
                        return Ok(JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid resource URI".to_string(),
                                data: None,
                            }),
                        });
                    }
                };
                
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "contents": [{
                            "uri": params.uri,
                            "mimeType": "application/json",
                            "text": serde_json::to_string_pretty(&data)?
                        }]
                    })),
                    error: None,
                })
            }
            "tools/list" => {
                let tools = vec![
                    McpTool {
                        name: "query_settlement_status".to_string(),
                        description: "Query settlement status for a specific auction".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "auction_id": {
                                    "type": "integer",
                                    "description": "Auction ID to query"
                                }
                            },
                            "required": ["auction_id"]
                        }),
                    },
                    McpTool {
                        name: "verify_settlement".to_string(),
                        description: "Verify a settlement transaction on blockchain".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "transaction_signature": {
                                    "type": "string",
                                    "description": "Solana transaction signature to verify"
                                }
                            },
                            "required": ["transaction_signature"]
                        }),
                    },
                    McpTool {
                        name: "get_aggregator_performance".to_string(),
                        description: "Get performance metrics for an aggregator".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "aggregator_id": {
                                    "type": "integer",
                                    "description": "Aggregator ID to query"
                                }
                            },
                            "required": ["aggregator_id"]
                        }),
                    },
                    McpTool {
                        name: "monitor_settlements".to_string(),
                        description: "Monitor recent settlement activity".to_string(),
                        input_schema: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "limit": {
                                    "type": "integer",
                                    "description": "Number of recent settlements to return",
                                    "default": 10
                                }
                            }
                        }),
                    },
                ];
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::to_value(ListToolsResult { tools })?),
                    error: None,
                })
            }
            "tools/call" => {
                let params: CallToolParams = serde_json::from_value(
                    params.unwrap_or(serde_json::Value::Null)
                )?;
                
                let result = match params.name.as_str() {
                    "query_settlement_status" => {
                        self.query_settlement_status(params.arguments).await?
                    }
                    "verify_settlement" => {
                        self.verify_settlement(params.arguments).await?
                    }
                    "get_aggregator_performance" => {
                        self.get_aggregator_performance(params.arguments).await?
                    }
                    "monitor_settlements" => {
                        self.monitor_settlements(params.arguments).await?
                    }
                    _ => {
                        return Ok(JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32601,
                                message: "Method not found".to_string(),
                                data: None,
                            }),
                        });
                    }
                };
                
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "content": [{
                            "type": "text",
                            "text": result
                        }]
                    })),
                    error: None,
                })
            }
            _ => {
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32601,
                        message: "Method not found".to_string(),
                        data: None,
                    }),
                })
            }
        }
    }

    async fn get_all_auction_settlements(&mut self) -> Result<Vec<AuctionSettlement>> {
        // In a real implementation, this would query the database
        // For now, return cached/simulated data
        let mut settlements = Vec::new();
        
        // Simulate some auction settlements
        for i in 1..=5 {
            let settlement = AuctionSettlement {
                auction_id: i,
                energy_amount_kwh: 5.0 + (i as f64 * 2.5),
                final_price_cents: 500 + (i * 50),
                total_value_usd: (5.0 + (i as f64 * 2.5)) * (500 + (i * 50)) as f64 / 100.0,
                settled: true,
                settlement_signature: format!("settlement_sig_{:08x}", i * 12345),
                blockchain_url: format!("https://explorer.solana.com/tx/settlement_sig_{:08x}?cluster=localnet", i * 12345),
                timestamp: chrono::Utc::now().timestamp(),
                winner: format!("AGG-{:03}", (i % 3) + 1),
                seller: format!("BESS-{:03}", (i % 5) + 1),
            };
            settlements.push(settlement);
        }
        
        Ok(settlements)
    }

    async fn get_all_aggregator_status(&mut self) -> Result<Vec<AggregatorStatus>> {
        let mut aggregators = Vec::new();
        
        for i in 1..=3 {
            let status = AggregatorStatus {
                aggregator_id: i,
                reputation_score: (70 + (i * 10)) as u8,
                successful_settlements: 10 + (i * 15),
                total_energy_traded_kwh: 100.0 + (i as f64 * 50.0),
                total_usdc_paid: (5000 + (i * 2500)) as u64,
                last_settlement: chrono::Utc::now().timestamp() - (i as i64 * 3600),
            };
            aggregators.push(status);
        }
        
        Ok(aggregators)
    }

    async fn get_all_battery_status(&mut self) -> Result<Vec<BatteryStatus>> {
        let mut batteries = Vec::new();
        
        for i in 1..=5 {
            let status = BatteryStatus {
                battery_id: i,
                capacity_kwh: 15.0 + (i as f64 * 5.0),
                total_energy_sold_kwh: 50.0 + (i as f64 * 20.0),
                total_usdc_earned: (2500 + (i * 1000)) as u64,
                last_sale: chrono::Utc::now().timestamp() - (i as i64 * 1800),
                active: i % 3 != 0,
            };
            batteries.push(status);
        }
        
        Ok(batteries)
    }

    async fn get_recent_settlements(&mut self) -> Result<Vec<AuctionSettlement>> {
        // Return the 3 most recent settlements
        let mut settlements = self.get_all_auction_settlements().await?;
        settlements.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        settlements.truncate(3);
        Ok(settlements)
    }

    async fn query_settlement_status(&mut self, args: Option<serde_json::Value>) -> Result<String> {
        let args = args.unwrap_or(serde_json::Value::Null);
        let auction_id: u64 = args.get("auction_id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing auction_id parameter"))?;
        
        let settlements = self.get_all_auction_settlements().await?;
        let settlement = settlements.iter()
            .find(|s| s.auction_id == auction_id)
            .ok_or_else(|| anyhow::anyhow!("Auction {} not found", auction_id))?;
        
        Ok(serde_json::to_string_pretty(settlement)?)
    }

    async fn verify_settlement(&mut self, args: Option<serde_json::Value>) -> Result<String> {
        let args = args.unwrap_or(serde_json::Value::Null);
        let signature: String = args.get("transaction_signature")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing transaction_signature parameter"))?
            .to_string();
        
        // In a real implementation, this would verify the transaction on-chain
        // For now, simulate verification
        let verification_result = serde_json::json!({
            "signature": signature,
            "verified": true,
            "block_height": 12345,
            "confirmation_time": chrono::Utc::now().timestamp(),
            "explorer_url": format!("https://explorer.solana.com/tx/{}?cluster=localnet", signature),
            "settlement_data": {
                "auction_id": 1,
                "energy_amount": 15.5,
                "final_price": 650,
                "total_value": 10.075
            }
        });
        
        Ok(serde_json::to_string_pretty(&verification_result)?)
    }

    async fn get_aggregator_performance(&mut self, args: Option<serde_json::Value>) -> Result<String> {
        let args = args.unwrap_or(serde_json::Value::Null);
        let aggregator_id: u32 = args.get("aggregator_id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing aggregator_id parameter"))?
            .try_into()?;
        
        let aggregators = self.get_all_aggregator_status().await?;
        let aggregator = aggregators.iter()
            .find(|a| a.aggregator_id == aggregator_id)
            .ok_or_else(|| anyhow::anyhow!("Aggregator {} not found", aggregator_id))?;
        
        Ok(serde_json::to_string_pretty(aggregator)?)
    }

    async fn monitor_settlements(&mut self, args: Option<serde_json::Value>) -> Result<String> {
        let limit = args
            .as_ref()
            .and_then(|v| v.get("limit"))
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        
        let mut settlements = self.get_all_auction_settlements().await?;
        settlements.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        settlements.truncate(limit);
        
        let monitoring_data = serde_json::json!({
            "recent_settlements": settlements,
            "total_count": settlements.len(),
            "timestamp": chrono::Utc::now().timestamp(),
            "summary": {
                "total_energy_traded": settlements.iter().map(|s| s.energy_amount_kwh).sum::<f64>(),
                "total_value": settlements.iter().map(|s| s.total_value_usd).sum::<f64>(),
                "average_price": settlements.iter().map(|s| s.final_price_cents as f64).sum::<f64>() / settlements.len() as f64,
            }
        });
        
        Ok(serde_json::to_string_pretty(&monitoring_data)?)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Energy Trading MCP Server for Blockchain Settlement Status");
    
    let mut server = McpServer::new()?;
    
    info!("📡 MCP Server ready, listening on stdin/stdout");
    
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    
    while reader.read_line(&mut line).await? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(request) => {
                match server.handle_request(request).await {
                    Ok(response) => {
                        let response_json = serde_json::to_string(&response)?;
                        println!("{}", response_json);
                    }
                    Err(e) => {
                        error!("Error handling request: {}", e);
                        let error_response = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: None,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: format!("Internal error: {}", e),
                                data: None,
                            }),
                        };
                        let error_json = serde_json::to_string(&error_response)?;
                        println!("{}", error_json);
                    }
                }
            }
            Err(e) => {
                error!("Invalid JSON-RPC request: {}", e);
            }
        }
        
        line.clear();
    }
    
    Ok(())
}
