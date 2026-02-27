module Aryabhata.Consensus.Mempool where

import Aryabhata.Types.Transaction

-- Transaction mempool
-- Priority: fee per byte first, then CNS as tiebreaker
-- Replay protection: epoch-nonce validation

data MempoolEntry = MempoolEntry
    { entryTx       :: Transaction
    , entryFeeRate  :: Double
    , entrySenderCns :: Double
    } deriving (Show, Eq)

-- Fee rate = fee / tx size in bytes
feeRate :: Transaction -> Int -> Double
feeRate tx sizeBytes =
    if sizeBytes == 0
        then 0.0
        else txFee tx / fromIntegral sizeBytes

-- Higher score = higher priority
priorityScore :: MempoolEntry -> Double
priorityScore entry =
    entryFeeRate entry * 1000.0 + entrySenderCns entry

-- Reject replay: tx epoch must match current epoch
isReplayAttack :: Transaction -> Integer -> Bool
isReplayAttack tx currentEpoch =
    txEpoch tx /= currentEpoch

-- Max mempool size
maxMempoolSize :: Int
maxMempoolSize = 10000

-- Min fee to enter mempool
minFee :: Double
minFee = 0.0001
