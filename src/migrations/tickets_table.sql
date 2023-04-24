DROP TABLE IF EXISTS tickets;
CREATE TABLE tickets (
    id INTEGER PRIMARY KEY,
    created_by_uid INTEGER NOT NULL,
    title TEXT NOT NULL,
    done BOOL NOT NULL DEFAULT false
);
-- Indexes
DROP INDEX IF EXISTS no_dups;
CREATE UNIQUE INDEX no_dups ON tickets(created_by_uid, title);