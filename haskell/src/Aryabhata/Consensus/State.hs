module Aryabhata.Consensus.State where

import Aryabhata.Types.Block
import Aryabhata.Consensus.CNS
import Aryabhata.Consensus.Supply

-- Protocol state
-- Pure — no IO, no side effects
-- Haskell is the only source of truth

data ProtocolState = ProtocolState
    { stateHeight      :: Integer
    , stateDifficulty  :: Double
    , stateTotalSupply :: Double
    , stateCirculating :: Double
    , stateTreasury    :: Double
    , stateEpoch       :: Integer
    } deriving (Show, Eq)

initialState :: ProtocolState
initialState = ProtocolState
    { stateHeight      = 0
    , stateDifficulty  = 1.0
    , stateTotalSupply = totalSupply
    , stateCirculating = 0.0
    , stateTreasury    = 0.0
    , stateEpoch       = 0
    }

-- Apply block reward to state
applyReward :: ProtocolState -> Double -> ProtocolState
applyReward state reward =
    state
        { stateCirculating = stateCirculating state + reward
        , stateHeight      = stateHeight state + 1
        }

-- Apply treasury fee
applyTreasury :: ProtocolState -> Double -> ProtocolState
applyTreasury state fee =
    state
        { stateTreasury = stateTreasury state + treasuryShare fee
        }

-- Check supply not exceeded
isSupplyValid :: ProtocolState -> Bool
isSupplyValid state =
    stateCirculating state <= stateTotalSupply state

-- Epoch increments every 720 blocks
updateEpoch :: ProtocolState -> ProtocolState
updateEpoch state =
    if stateHeight state `mod` 720 == 0
        then state { stateEpoch = stateEpoch state + 1 }
        else state
