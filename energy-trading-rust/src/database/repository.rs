use crate::database::models::*;
use crate::database::connection::DatabaseConnection;
use crate::error::Result;
use bigdecimal::BigDecimal;
use chrono::Utc;
use sqlx::Row;

pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Battery operations
    pub async fn create_battery(&self, battery: NewBattery) -> Result<Battery> {
        let result = sqlx::query_as!(
            Battery,
            r#"
            INSERT INTO batteries (device_id, owner_pubkey, energy_total, percentage_for_sale, 
                                 reserve_price, health_status, voltage, discharge_rate, status, last_seen)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            battery.device_id,
            battery.owner_pubkey,
            battery.energy_total,
            battery.percentage_for_sale,
            battery.reserve_price,
            battery.health_status,
            battery.voltage,
            battery.discharge_rate,
            battery.status,
            Utc::now()
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_battery(&self, device_id: i32) -> Result<Option<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            "SELECT * FROM batteries WHERE device_id = $1",
            device_id
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_battery_status(&self, device_id: i32, status: &str, energy_total: BigDecimal) -> Result<()> {
        sqlx::query!(
            "UPDATE batteries SET status = $1, energy_total = $2, last_seen = $3 WHERE device_id = $4",
            status,
            energy_total,
            Utc::now(),
            device_id
        )
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    pub async fn get_online_batteries(&self) -> Result<Vec<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            "SELECT * FROM batteries WHERE status = 'online' ORDER BY created_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(result)
    }

    // Aggregator operations
    pub async fn create_aggregator(&self, aggregator: NewAggregator) -> Result<Aggregator> {
        let result = sqlx::query_as!(
            Aggregator,
            r#"
            INSERT INTO aggregators (device_id, owner_pubkey, max_bid_price, energy_requirements, 
                                   reputation_score, status, last_seen)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            aggregator.device_id,
            aggregator.owner_pubkey,
            aggregator.max_bid_price,
            aggregator.energy_requirements,
            aggregator.reputation_score,
            aggregator.status,
            Utc::now()
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_aggregator(&self, device_id: i32) -> Result<Option<Aggregator>> {
        let result = sqlx::query_as!(
            Aggregator,
            "SELECT * FROM aggregators WHERE device_id = $1",
            device_id
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_online_aggregators(&self) -> Result<Vec<Aggregator>> {
        let result = sqlx::query_as!(
            Aggregator,
            "SELECT * FROM aggregators WHERE status = 'online' ORDER BY created_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(result)
    }

    // Auction operations
    pub async fn create_auction(&self, auction: NewAuction) -> Result<Auction> {
        let result = sqlx::query_as!(
            Auction,
            r#"
            INSERT INTO auctions (battery_id, energy_amount, reserve_price, status, started_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            auction.battery_id,
            auction.energy_amount,
            auction.reserve_price,
            auction.status,
            Utc::now()
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_auction(&self, id: i64) -> Result<Option<Auction>> {
        let result = sqlx::query_as!(
            Auction,
            "SELECT * FROM auctions WHERE id = $1",
            id
        )
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_auction_status(&self, id: i64, status: &str, final_price: Option<BigDecimal>) -> Result<()> {
        sqlx::query!(
            "UPDATE auctions SET status = $1, final_price = $2, settled_at = $3 WHERE id = $4",
            status,
            final_price,
            Utc::now(),
            id
        )
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    pub async fn get_active_auctions(&self) -> Result<Vec<Auction>> {
        let result = sqlx::query_as!(
            Auction,
            "SELECT * FROM auctions WHERE status = 'active' ORDER BY started_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(result)
    }

    // Bid operations
    pub async fn create_bid(&self, bid: NewBid) -> Result<Bid> {
        let result = sqlx::query_as!(
            Bid,
            r#"
            INSERT INTO bids (auction_id, aggregator_id, bid_price, energy_amount, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            bid.auction_id,
            bid.aggregator_id,
            bid.bid_price,
            bid.energy_amount,
            bid.status,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_auction_bids(&self, auction_id: i64) -> Result<Vec<Bid>> {
        let result = sqlx::query_as!(
            Bid,
            "SELECT * FROM bids WHERE auction_id = $1 ORDER BY bid_price DESC, created_at ASC",
            auction_id
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(result)
    }

    // System metrics
    pub async fn get_system_metrics(&self) -> Result<SystemMetrics> {
        let auction_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM auctions")
            .fetch_one(&self.db.pool)
            .await?;

        let bid_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM bids")
            .fetch_one(&self.db.pool)
            .await?;

        let active_batteries: i32 = sqlx::query_scalar!("SELECT COUNT(*) FROM batteries WHERE status = 'online'")
            .fetch_one(&self.db.pool)
            .await?;

        let active_aggregators: i32 = sqlx::query_scalar!("SELECT COUNT(*) FROM aggregators WHERE status = 'online'")
            .fetch_one(&self.db.pool)
            .await?;

        Ok(SystemMetrics {
            total_auctions: auction_count,
            total_bids: bid_count,
            avg_price_improvement_percent: 0.0, // TODO: Calculate actual improvement
            active_bess_nodes: active_batteries,
            active_aggregators,
        })
    }
}