module Aryabhata.Types.Transaction where

-- Transaction type
-- Signed with CRYSTALS-Dilithium5
-- Epoch-nonce prevents replay attacks

data Transaction = Transaction
    { txId        :: [Int]
    , txSender    :: [Int]
    , txReceiver  :: [Int]
    , txAmount    :: Double
    , txFee       :: Double
    , txNonce     :: Integer
    , txEpoch     :: Integer
    , txSignature :: [Int]
    } deriving (Show, Eq)

data TxPriority
    = HighPriority
    | NormalPriority
    | LowPriority
    deriving (Show, Eq)

-- Fee multiplier based on sender CNS
feeMultiplier :: Double -> Double
feeMultiplier cns
    | cns >= 0.80 = 0.80
    | cns >= 0.60 = 1.00
    | cns >= 0.40 = 1.50
    | cns >= 0.20 = 3.00
    | otherwise   = 2.00

-- Treasury takes 2% of every fee
treasuryShare :: Double -> Double
treasuryShare fee = fee * 0.02
