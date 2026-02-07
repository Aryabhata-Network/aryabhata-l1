{-# LANGUAGE DeriveAnyClass #-}
{-# LANGUAGE DeriveGeneric  #-}
{-# LANGUAGE StrictData     #-}

module Aryabhata.Consensus.ZeroPoW
  ( TransitionError(..)
  , Header(..)
  , applyHeader
  ) where

import           Aryabhata.Consensus.State
import           GHC.Generics (Generic)
import           Control.Monad (when)

--------------------------------------------------------------------------------
-- | Block Header (ZeroPoW "Proof")
--------------------------------------------------------------------------------

-- | In ZeroPoW, the header commits to the next state deterministically.
-- No hashing, no randomness, no energy expenditure.
data Header = Header
  { hNewStateRoot :: !StateRoot
  } deriving (Eq, Show, Generic)

--------------------------------------------------------------------------------
-- | Consensus Errors
--------------------------------------------------------------------------------

data TransitionError
  = ParentHeightMismatch
      { expected :: BlockHeight
      , actual   :: BlockHeight
      }
  | HeightMaxReached
  deriving (Eq, Show, Generic)

--------------------------------------------------------------------------------
-- | ZeroPoW Transition Law
--------------------------------------------------------------------------------

-- | Apply a header to the current protocol state.
--
-- FORMAL LAWS:
-- 1. Height increases by exactly one
-- 2. Transition is deterministic
-- 3. No time, randomness, or work involved
--
-- NOTE:
-- StateRoot semantic validity is checked elsewhere.
applyHeader
  :: ProtocolState
  -> Header
  -> Either TransitionError ProtocolState
applyHeader oldState header = do
    newHeight <- incrementHeight (psBlockHeight oldState)

    pure ProtocolState
      { psBlockHeight = newHeight
      , psStateRoot   = hNewStateRoot header
      }

--------------------------------------------------------------------------------
-- | Internal helper
--------------------------------------------------------------------------------

incrementHeight :: BlockHeight -> Either TransitionError BlockHeight
incrementHeight (BlockHeight h)
  | h == maxBound = Left HeightMaxReached
  | otherwise    = Right (BlockHeight (h + 1))
