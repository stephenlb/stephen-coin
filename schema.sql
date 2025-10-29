-- SQL schema for storing transaction data
CREATE TABLE transaction (
    -- TODO Merkle Root: ***OG HASH***
    -- TODO The root hash of a Merkle tree,
    -- TODO which is a cryptographic summary of all the transactions in the block. This ensures transaction data is tamper-resistant.
    version      INTEGER       NOT NULL,
    signature    VARCHAR(7856) NOT NULL,
    difficulty   INTEGER       NOT NULL,
    nonce        INTEGER       NOT NULL,

    timestamp    INTEGER       NOT NULL,
    fromAddress  VARCHAR(64)   NOT NULL,
    toAddress    VARCHAR(64)   NOT NULL,
    amount       REAL          NOT NULL,
    note         VARCHAR(256), -- any details about the transaction

    -- Smart Contract Part
    data         BLOB, -- lots of data!
    code         TEXT  -- Lua Source Code for Smart Contract

    PRIMARY KEY (fromAddress, toAddress, timestamp)

    Difficulty Target: The encoded representation of the target threshold the block's hash must be below.
    Nonce: A 4-byte number that miners adjust to find a valid block hash that meets the difficulty target.
)
