haskell/src/Aryabhata/Proof/SecurityTests.hs
module Aryabhata.Proof.SecurityTests where

import Aryabhata.Consensus.CNS
import Aryabhata.Consensus.Penalty
import Aryabhata.Consensus.Fork

-- Double spend always results in permanent ban
prop_doubleSpendPermanentBan :: Double -> Bool
prop_doubleSpendPermanentBan cns =
    let (newCns, result) = applyPenalty DoubleSpendAttempt (abs cns)
    in newCns == 0.0 && result == PermanentBan

-- Penalty never makes CNS negative
prop_penaltyNonNegative :: Double -> Bool
prop_penaltyNonNegative cns =
    let cns' = abs cns
        (newCns, _) = applyPenalty InvalidBlockSubmit cns'
    in newCns >= 0.0

-- Expelled node has CNS below 0.20
prop_expelledBelowThreshold :: Bool
prop_expelledBelowThreshold =
    isExpelled 0.19 == True
    && isExpelled 0.20 == False

-- Reorg depth safety check
prop_reorgSafetyHolds :: Bool
prop_reorgSafetyHolds =
    isSafeReorg 1000 100 == True
    && isSafeReorg 1000 101 == False

-- Treasury slash is exactly 50%
prop_slashExactlyHalf :: Double -> Bool
prop_slashExactlyHalf stake =
    slashAmount (abs stake) == abs stake * 0.50

-- canMine requires exactly 0.60
prop_miningThresholdExact :: Bool
prop_miningThresholdExact =
    canMine 0.60 == True
    && canMine 0.599 == False
