module Aryabhata.Types.Block where

-- Block type definition
-- Immutable after genesis
-- Any modification defines a different protocol

data Block = Block
    { blockHeight    :: Integer
    , blockHash      :: [Int]
    , parentHash     :: [Int]
    , minerPubKey    :: [Int]
    , minerCns       :: Double
    , timestamp      :: Integer
    , transactions   :: [[Int]]
    , nonce          :: Integer
    , blockReward    :: Double
    } deriving (Show, Eq)

data BlockHeader = BlockHeader
    { headerHeight   :: Integer
    , headerHash     :: [Int]
    , headerParent   :: [Int]
    , headerTime     :: Integer
    , headerNonce    :: Integer
    } deriving (Show, Eq)

genesisReward :: Double
genesisReward = 0.25

isGenesisBlock :: Block -> Bool
isGenesisBlock block = blockHeight block == 0
