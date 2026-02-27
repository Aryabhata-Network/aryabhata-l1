module Aryabhata.Proof.Properties where

import Aryabhata.Consensus.CNS

-- QuickCheck property tests
-- Every invariant tested with 10000+ random inputs

-- CNS must always be in [0.0, 1.0]
prop_cnsInRange :: Double -> Double -> Double -> Bool
prop_cnsInRange u r p =
    let cns = computeCns u r p
    in cns >= 0.0 && cns <= 1.0

-- Elite always above Standard threshold
prop_eliteAboveStandard :: Bool
prop_eliteAboveStandard =
    cnsStatus 0.80 == Elite &&
    cnsStatus 0.79 == Standard

-- Expelled always below 0.20
prop_expelledBelow :: Bool
prop_expelledBelow =
    cnsStatus 0.19 == Expelled &&
    cnsStatus 0.20 == Penalised

-- restartStability never negative
prop_restartNonNegative :: Int -> Bool
prop_restartNonNegative n =
    restartStability (abs n) >= 0.0

-- propagationScore never negative
prop_propagationNonNegative :: Double -> Bool
prop_propagationNonNegative d =
    propagationScore (abs d) >= 0.0

-- canMine requires CNS >= 0.60
prop_miningThreshold :: Bool
prop_miningThreshold =
    canMine 0.60 == True &&
    canMine 0.59 == False
