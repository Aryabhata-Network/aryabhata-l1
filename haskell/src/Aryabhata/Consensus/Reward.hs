module Aryabhata.Consensus.Reward where

import Aryabhata.Consensus.Supply
import Aryabhata.Consensus.CNS

-- Block reward distribution
-- Miner gets full reward if ELITE
-- ELITE miner gets 10% bonus

minerReward :: Integer -> Double -> Double
minerReward height cns =
    let base = blockReward height
    in if isElite cns
        then base * 1.10
        else base

-- Founding node certificate bonus
-- Permanent +0.05 CNS for genesis miners
applyFoundingBonus :: Double -> Bool -> Double
applyFoundingBonus cns isFounding =
    if isFounding
        then min 1.0 (cns + foundingNodeBonus)
        else cns

-- Fee distribution
-- 98% to miner, 2% to treasury
minerFeeShare :: Double -> Double
minerFeeShare fee = fee * 0.98

treasuryFeeShare :: Double -> Double
treasuryFeeShare fee = fee * 0.02

-- Verify shares sum to 100%
sharesValid :: Double -> Bool
sharesValid fee =
    abs (minerFeeShare fee + treasuryFeeShare fee - fee) < 0.000001
