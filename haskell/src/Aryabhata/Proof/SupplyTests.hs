module Aryabhata.Proof.SupplyTests where

import Aryabhata.Consensus.Supply
import Aryabhata.Consensus.State

-- Total supply never changes
prop_totalSupplyFixed :: Bool
prop_totalSupplyFixed =
    totalSupply == 14_000_000_000.0

-- Treasury share is exactly 2%
prop_treasuryShareExact :: Double -> Bool
prop_treasuryShareExact fee =
    treasuryShare (abs fee) == abs fee * 0.02

-- Founding node bonus is exactly 0.05
prop_foundingBonusExact :: Bool
prop_foundingBonusExact =
    foundingNodeBonus == 0.05

-- Initial state has zero circulating supply
prop_initialCirculatingZero :: Bool
prop_initialCirculatingZero =
    stateCirculating initialState == 0.0

-- Initial treasury is zero
prop_initialTreasuryZero :: Bool
prop_initialTreasuryZero =
    stateTreasury initialState == 0.0

-- Supply validity check works
prop_supplyValidityCheck :: Bool
prop_supplyValidityCheck =
    isSupplyValid initialState == True

-- Apply reward increases circulating supply
prop_rewardIncreasesCirculating :: Bool
prop_rewardIncreasesCirculating =
    let state   = initialState
        state'  = applyReward state 0.25
    in stateCirculating state' == 0.25

-- Circulating never exceeds total
prop_circulatingNeverExceedsTotal :: Double -> Bool
prop_circulatingNeverExceedsTotal reward =
    let reward' = abs reward
        state   = applyReward initialState reward'
    in isSupplyValid state || reward' > totalSupply
