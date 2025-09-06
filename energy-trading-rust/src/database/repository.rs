use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use crate::database::models::*;

pub struct BatteryRepository {
    pool: PgPool,
}

impl BatteryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, battery: NewBattery) -> Result<Battery> {
        let result = sqlx::query_as::<_, Battery>(
            r#"
            INSERT INTO batteries (device_id, owner_pubkey, energy_total, percentage_for_sale, 
                                 reserve_price, health_status, voltage, discharge_rate, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(battery.device_id)
        .bind(battery.owner_pubkey)
        .bind(battery.energy_total)
        .bind(battery.percentage_for_sale)
        .bind(battery.reserve_price)
        .bind(battery.health_status)
        .bind(battery.voltage)
        .bind(battery.discharge_rate)
        .bind(battery.status)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_device_id(&self, device_id: i32) -> Result<Option<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            "SELECT * FROM batteries WHERE device_id = $1",
            device_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_status(&self, device_id: i32, status: &str, energy_total: f64) -> Result<()> {
        sqlx::query!(
            "UPDATE batteries SET status = $1, energy_total = $2, last_seen = $3 WHERE device_id = $4",
            status,
            energy_total,
            Utc::now(),
            device_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_active(&self) -> Result<Vec<Battery>> {
        let result = sqlx::query_as!(
            Battery,
            "SELECT * FROM batteries WHERE status = 'online' ORDER BY device_id"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

pub struct AggregatorRepository {
    pool: PgPool,
}

impl AggregatorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, aggregator: NewAggregator) -> Result<Aggregator> {
        let result = sqlx::query_as!(
            Aggregator,
            r#"
            INSERT INTO aggregators (device_id, owner_pubkey, max_bid_price, 
                                   energy_requirements, reputation_score, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            aggregator.device_id,
            aggregator.owner_pubkey,
            aggregator.max_bid_price,
            aggregator.energy_requirements,
            aggregator.reputation_score,
            aggregator.status
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_device_id(&self, device_id: i32) -> Result<Option<Aggregator>> {
        let result = sqlx::query_as!(
            Aggregator,
            "SELECT * FROM aggregators WHERE device_id = $1",
            device_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_status(&self, device_id: i32, status: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE aggregators SET status = $1, last_seen = $2 WHERE device_id = $3",
            status,
            Utc::now(),
            device_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_active(&self) -> Result<Vec<Aggregator>> {
        let result = sqlx::query_as!(
            Aggregator,
            "SELECT * FROM aggregators WHERE status = 'online' ORDER BY device_id"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

pub struct AuctionRepository {
    pool: PgPool,
}

impl AuctionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, auction: NewAuction) -> Result<Auction> {
        let result = sqlx::query_as!(
            Auction,
            r#"
            INSERT INTO auctions (battery_id, energy_amount, reserve_price, status)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            auction.battery_id,
            auction.energy_amount,
            auction.reserve_price,
            auction.status
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Auction>> {
        let result = sqlx::query_as!(
            Auction,
            "SELECT * FROM auctions WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_status(&self, id: i64, status: &str, final_price: Option<f64>, aggregator_id: Option<i32>) -> Result<()> {
        sqlx::query!(
            "UPDATE auctions SET status = $1, final_price = $2, aggregator_id = $3, settled_at = $4 WHERE id = $5",
            status,
            final_price,
            aggregator_id,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_active_auctions(&self) -> Result<Vec<Auction>> {
        let result = sqlx::query_as!(
            Auction,
            "SELECT * FROM auctions WHERE status = 'active' ORDER BY started_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_total_bids(&self) -> Result<i64> {
        let result = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM bids WHERE status = 'accepted'"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(0))
    }
}

pub struct BidRepository {
    pool: PgPool,
}

impl BidRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, bid: NewBid) -> Result<Bid> {
        let result = sqlx::query_as!(
            Bid,
            r#"
            INSERT INTO bids (auction_id, aggregator_id, bid_price, energy_amount, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            bid.auction_id,
            bid.aggregator_id,
            bid.bid_price,
            bid.energy_amount,
            bid.status
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_auction(&self, auction_id: i64) -> Result<Vec<Bid>> {
        let result = sqlx::query_as!(
            Bid,
            "SELECT * FROM bids WHERE auction_id = $1 ORDER BY bid_price DESC, created_at ASC",
            auction_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE bids SET status = $1, updated_at = $2 WHERE id = $3",
            status,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
