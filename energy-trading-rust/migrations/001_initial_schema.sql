-- Create batteries table
CREATE TABLE batteries (
    id SERIAL PRIMARY KEY,
    device_id INTEGER UNIQUE NOT NULL,
    owner_pubkey VARCHAR(44) NOT NULL,
    energy_total DECIMAL(10,2) NOT NULL,
    percentage_for_sale DECIMAL(5,2) NOT NULL,
    reserve_price DECIMAL(8,2) NOT NULL,
    health_status INTEGER NOT NULL,
    voltage DECIMAL(6,2) NOT NULL,
    discharge_rate DECIMAL(6,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create aggregators table
CREATE TABLE aggregators (
    id SERIAL PRIMARY KEY,
    device_id INTEGER UNIQUE NOT NULL,
    owner_pubkey VARCHAR(44) NOT NULL,
    max_bid_price DECIMAL(8,2) NOT NULL,
    energy_requirements DECIMAL(10,2) NOT NULL,
    reputation_score INTEGER DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'offline',
    last_seen TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create auctions table
CREATE TABLE auctions (
    id BIGSERIAL PRIMARY KEY,
    battery_id INTEGER NOT NULL REFERENCES batteries(id),
    aggregator_id INTEGER REFERENCES aggregators(id),
    energy_amount DECIMAL(10,2) NOT NULL,
    reserve_price DECIMAL(8,2) NOT NULL,
    final_price DECIMAL(8,2),
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    blockchain_tx_hash VARCHAR(88),
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settled_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create bids table
CREATE TABLE bids (
    id BIGSERIAL PRIMARY KEY,
    auction_id BIGINT NOT NULL REFERENCES auctions(id),
    aggregator_id INTEGER NOT NULL REFERENCES aggregators(id),
    bid_price DECIMAL(8,2) NOT NULL,
    energy_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_batteries_device_id ON batteries(device_id);
CREATE INDEX idx_batteries_status ON batteries(status);
CREATE INDEX idx_aggregators_device_id ON aggregators(device_id);
CREATE INDEX idx_aggregators_status ON aggregators(status);
CREATE INDEX idx_auctions_battery_id ON auctions(battery_id);
CREATE INDEX idx_auctions_status ON auctions(status);
CREATE INDEX idx_auctions_started_at ON auctions(started_at);
CREATE INDEX idx_bids_auction_id ON bids(auction_id);
CREATE INDEX idx_bids_aggregator_id ON bids(aggregator_id);
CREATE INDEX idx_bids_status ON bids(status);

-- Insert some initial test data
INSERT INTO batteries (device_id, owner_pubkey, energy_total, percentage_for_sale, reserve_price, health_status, voltage, discharge_rate, status) VALUES
(101, 'BESS101_OWNER_PUBKEY', 10.0, 80.0, 8.3, 0, 48.0, 5.0, 'online'),
(102, 'BESS102_OWNER_PUBKEY', 12.0, 75.0, 7.8, 1, 24.0, 4.5, 'online'),
(103, 'BESS103_OWNER_PUBKEY', 14.0, 85.0, 9.1, 2, 48.0, 6.0, 'online');

INSERT INTO aggregators (device_id, owner_pubkey, max_bid_price, energy_requirements, reputation_score, status) VALUES
(201, 'AGG201_OWNER_PUBKEY', 30.0, 50.0, 100, 'online'),
(202, 'AGG202_OWNER_PUBKEY', 25.0, 40.0, 95, 'online'),
(203, 'AGG203_OWNER_PUBKEY', 28.0, 45.0, 98, 'online'),
(204, 'AGG204_OWNER_PUBKEY', 32.0, 55.0, 100, 'online'),
(205, 'AGG205_OWNER_PUBKEY', 26.0, 35.0, 92, 'online');
