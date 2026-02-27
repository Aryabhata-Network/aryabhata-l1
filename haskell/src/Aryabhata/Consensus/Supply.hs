module Aryabhata.Consensus.Supply where

-- Total supply: 14,000,000,000 ARY
-- Fixed, immutable, name-locked
-- No premine, no founder allocation

totalSupply :: Double
totalSupply = 14_000_000_000.0

-- Emission phases
data Phase
    = Genesis
    | Early
    | Growth
    | Mature
    | Late
    | Terminal
    deriving (Show, Eq)

blockReward :: Integer -> Double
blockReward height
    | height <= 720        = 0.25
    | height <= 14_400     = 8.00
    | height <= 72_000     = 5.00
    | height <= 216_000    = 2.50
    | height <= 1_000_000  = 0.50
    | otherwise            = 0.00

currentPhase :: Integer -> Phase
currentPhase height
    | height <= 720        = Genesis
    | height <= 14_400     = Early
    | height <= 72_000     = Growth
    | height <= 216_000    = Mature
    | height <= 1_000_000  = Late
    | otherwise            = Terminal

-- Treasury: 2% of every transaction fee
treasuryFeePercent :: Double
treasuryFeePercent = 0.02

treasuryShare :: Double -> Double
treasuryShare fee = fee * treasuryFeePercent

-- Founding node CNS bonus
foundingNodeBonus :: Double
foundingNodeBonus = 0.05
