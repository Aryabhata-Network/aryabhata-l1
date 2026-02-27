module Aryabhata.Consensus.Fork where

import Aryabhata.Types.Block

-- Fork resolution
-- Longest valid chain wins
-- Both work AND CNS must be valid
-- Post-checkpoint forks rejected

data ChainSegment = ChainSegment
    { segmentBlocks     :: [Block]
    , segmentTotalWork  :: Double
    , segmentAvgCns     :: Double
    } deriving (Show, Eq)

-- Choose winning chain segment
-- Higher total work wins
-- CNS average used as tiebreaker
selectChain
    :: ChainSegment
    -> ChainSegment
    -> ChainSegment
selectChain a b
    | segmentTotalWork a > segmentTotalWork b = a
    | segmentTotalWork b > segmentTotalWork a = b
    | segmentAvgCns a >= segmentAvgCns b      = a
    | otherwise                               = b

-- Maximum chain reorganisation depth
-- Cannot reorg past this many blocks
maxReorgDepth :: Integer
maxReorgDepth = 100

-- Reject reorg if it goes past checkpoint
isSafeReorg :: Integer -> Integer -> Bool
isSafeReorg currentHeight reorgDepth =
    reorgDepth <= maxReorgDepth
    && currentHeight - reorgDepth > 0
