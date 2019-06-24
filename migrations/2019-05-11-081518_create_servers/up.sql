CREATE TABLE servers
(
    id       VARCHAR NOT NULL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    address  VARCHAR NOT NULL,
    rank     INTEGER NOT NULL DEFAULT 0,
    record   INTEGER NOT NULL DEFAULT 0,
    versions TEXT    NOT NULL DEFAULT ''
)