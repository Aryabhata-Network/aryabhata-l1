module Aryabhata.Consensus.Difficulty where

-- Difficulty adjustment every 720 blocks
-- Target block time: 60 seconds
-- Max adjustment per period: +/- 25%
-- Uses exponential moving average

targetBlockTime :: Double
targetBlockTime = 60.0

adjustmentInterval :: Integer
adjustmentInterval = 720

maxAdjustmentPercent :: Double
maxAdjustmentPercent = 0.25

adjustDifficulty :: Double -> Double -> Double
adjustDifficulty currentDifficulty actualAvgTime =
    let ratio    = targetBlockTime / actualAvgTime
        capped   = capRatio ratio
    in currentDifficulty * capped

capRatio :: Double -> Double
capRatio ratio
    | ratio > (1.0 + maxAdjustmentPercent) = 1.0 + maxAdjustmentPercent
    | ratio < (1.0 - maxAdjustmentPercent) = 1.0 - maxAdjustmentPercent
    | otherwise                             = ratio

movingAverage :: [Double] -> Double
movingAverage [] = targetBlockTime
movingAverage xs = sum xs / fromIntegral (length xs)
