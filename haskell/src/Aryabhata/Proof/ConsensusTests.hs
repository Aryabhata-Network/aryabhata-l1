module Aryabhata.Proof.ConsensusTests where

import Aryabhata.Consensus.Core
import Aryabhata.Consensus.CNS
import Aryabhata.Consensus.Supply
import Aryabhata.Consensus.Difficulty
import Aryabhata.Types.Block

-- Block reward never exceeds total supply
prop_rewardBelowSupply :: Integer -> Bool
prop_rewardBelowSupply height =
    blockReward (abs height) <= totalSupply

-- Block reward never negative
prop_rewardNonNegative :: Integer -> Bool
prop_rewardNonNegative height =
    blockReward (abs height) >= 0.0

-- Terminal phase reward is zero
prop_terminalZeroReward :: Bool
prop_terminalZeroReward =
    blockReward 2_000_000 == 0.0

-- Genesis reward is 0.25
prop_genesisRewardCorrect :: Bool
prop_genesisRewardCorrect =
    blockReward 0 == 0.25

-- Difficulty ratio always capped at 25%
prop_difficultyCapHolds :: Double -> Double -> Bool
prop_difficultyCapHolds current actual =
    let current' = abs current + 0.01
        actual'  = abs actual  + 0.01
        newDiff  = adjustDifficulty current' actual'
        ratio    = newDiff / current'
    in ratio >= 0.75 && ratio <= 1.25

-- Moving average of single value equals itself
prop_movingAverageSingle :: Double -> Bool
prop_movingAverageSingle x =
    let x' = abs x + 0.01
    in movingAverage [x'] == x'

-- CNS weighted sum correct
prop_cnsWeightSum :: Bool
prop_cnsWeightSum =
    let result = computeCns 1.0 1.0 1.0
    in result == 1.0

-- Time validation rejects out of range
prop_timeValidRange :: Bool
prop_timeValidRange =
    let block = Block 1 [] [] [] 0.70 100 [] 0 0.25
    in timeValid block 0  == True
    && timeValid block 54 == False
    && timeValid block 55 == True
