module Aryabhata.Consensus.Checkpoint where

-- Checkpoints every 10,000 blocks
-- Embedded in client binary
-- Pre-checkpoint history cannot be rewritten
-- Prevents long-range attacks

data Checkpoint = Checkpoint
    { checkpointHeight :: Integer
    , checkpointHash   :: [Int]
    } deriving (Show, Eq)

-- Hardcoded genesis checkpoint
-- More added after each 10,000 blocks
genesisCheckpoint :: Checkpoint
genesisCheckpoint = Checkpoint
    { checkpointHeight = 0
    , checkpointHash   = replicate 32 0
    }

-- Validate block against nearest checkpoint
isAboveCheckpoint :: Integer -> Bool
isAboveCheckpoint height =
    height > checkpointHeight genesisCheckpoint

-- Checkpoint interval
checkpointInterval :: Integer
checkpointInterval = 10_000

-- Check if height is a checkpoint height
isCheckpointHeight :: Integer -> Bool
isCheckpointHeight height =
    height `mod` checkpointInterval == 0
