module Aryabhata.Consensus.Core where

-- Pure state transitions only
-- No IO, no networking, no system time
-- Same input always produces identical output

data BlockValidity
    = Valid
    | InvalidWork
    | InvalidTime
    | InvalidSilence
    deriving (Show, Eq)

data Block = Block
    { blockHeight  :: Integer
    , blockHash    :: [Word8]
    , parentHash   :: [Word8]
    , minerPubKey  :: [Word8]
    , minerCns     :: Double
    , timestamp    :: Integer
    , nonce        :: Integer
    } deriving (Show, Eq)

data Word8 = Word8 Int
    deriving (Show, Eq)

-- valid(B) = WORK(B) AND TIME(B) AND SILENCE(miner)
validateBlock
    :: Block
    -> Bool
    -> Integer
    -> BlockValidity
validateBlock block workValid parentTime
    | not workValid
    = InvalidWork
    | not (timeValid block parentTime)
    = InvalidTime
    | minerCns block < minCns
    = InvalidSilence
    | otherwise
    = Valid

-- TIME condition
-- timestamp must be in [parent+45, parent+180]
timeValid :: Block -> Integer -> Bool
timeValid block parentTime =
    let t = timestamp block
        tMin = parentTime + 45
        tMax = parentTime + 180
    in t >= tMin && t <= tMax

-- Minimum CNS to submit a block
minCns :: Double
minCns = 0.60
