CREATE TABLE vaults (
  owner TEXT PRIMARY KEY,
  total_balance BIGINT,
  locked_balance BIGINT,
  available_balance BIGINT,
  created_at TIMESTAMP
);

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  owner TEXT,
  type TEXT,
  amount BIGINT,
  timestamp TIMESTAMP
);

CREATE TABLE balance_snapshots (
  owner TEXT,
  balance BIGINT,
  snapshot_time TIMESTAMP
);
