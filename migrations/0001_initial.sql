CREATE TABLE IF NOT EXISTS devices (
 id uuid PRIMARY KEY, manufacturer text NOT NULL CHECK (length(manufacturer) BETWEEN 1 AND 100),
 model text NOT NULL CHECK (length(model) BETWEEN 1 AND 150),
 notes text NOT NULL DEFAULT '',
 created_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS research_entries (
 id uuid PRIMARY KEY, device_id uuid NOT NULL REFERENCES devices(id),
 title text NOT NULL CHECK (length(title) BETWEEN 1 AND 200),
 body text NOT NULL CHECK (length(body) BETWEEN 1 AND 10000),
 created_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS experiments (
 id uuid PRIMARY KEY, device_id uuid NOT NULL REFERENCES devices(id),
 title text NOT NULL CHECK (length(title) BETWEEN 1 AND 200),
 hypothesis text NOT NULL CHECK (length(hypothesis) BETWEEN 1 AND 2000),
 status text NOT NULL DEFAULT 'proposed' CHECK (status IN ('proposed','in_progress','blocked','verified')),
 created_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS artifacts (
 id uuid PRIMARY KEY, device_id uuid NOT NULL REFERENCES devices(id),
 name text NOT NULL CHECK (length(name) BETWEEN 1 AND 200),
 sha256 text NOT NULL CHECK (sha256 ~ '^[a-f0-9]{64}$'),
 provenance text NOT NULL CHECK (length(provenance) BETWEEN 1 AND 2000),
 created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS research_device_idx ON research_entries(device_id);
CREATE INDEX IF NOT EXISTS experiments_device_idx ON experiments(device_id);
CREATE INDEX IF NOT EXISTS artifacts_device_idx ON artifacts(device_id);
