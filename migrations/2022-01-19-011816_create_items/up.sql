CREATE TABLE items (
  id VARCHAR NOT NULL PRIMARY KEY,
  description VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  done BOOLEAN NOT NULL DEFAULT 'f'
)
