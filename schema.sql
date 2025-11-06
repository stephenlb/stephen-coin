-- Schema for Ledger
-- SQL schema for storing transaction data
CREATE TABLE IF NOT EXISTS transactions (
    -- TODO Merkle Root: ***OG HASH***
    -- TODO The root hash of a Merkle tree,
    -- TODO which is a cryptographic summary of all the transactions in the block. This ensures transaction data is tamper-resistant.
    -- TODO PRIMARY KEY (fromAddress, toAddress, timestamp)
    version              INTEGER       NOT NULL, 
    difficulty           INTEGER       NOT NULL,
    nonce                INTEGER       NOT NULL,
    id                   VARCHAR(64)   NOT NULL,
    priorId              VARCHAR(64)   NOT NULL,

    previousSignature    VARCHAR(7856) NOT NULL,
    signature            VARCHAR(7856) NOT NULL,
    timestamp            INTEGER       NOT NULL,
    fromAddress          VARCHAR(64)   NOT NULL,
    toAddress            VARCHAR(64)   NOT NULL,
    amount               REAL          NOT NULL,
    note                 VARCHAR(256), -- any details about the transaction

    -- Smart Contract Part
    data                 BLOB, -- lots of data!
    code                 TEXT  -- Lua Source Code for Smart Contract
)

-- TODO: Define blocks table
-- CREATE TABLE IF NOT EXISTS blocks (
-- )
