{-# LANGUAGE DeriveGeneric      #-}
{-# LANGUAGE DerivingStrategies #-}
{-# LANGUAGE StrictData         #-}

-- |
-- Module: Aryabhata.Consensus.State
--
-- This module defines the canonical protocol state for Aryabhata Layer-1.
--
-- CONSTITUTIONAL GUARANTEES:
-- 1. Pure data only (no IO, no time, no randomness)
-- 2. Deterministic and total
-- 3. Strongly typed to prevent invalid states
-- 4. Serialization is canonical (binary only)
--
-- This file is IMMUTABLE after Genesis (2024-12-31).

module Aryabhata.Consensus.State
  ( -- * Fundamental Types
    BlockHeight(..)
  , StateRoot(..)

    -- * Protocol State
  , ProtocolState(..)

    -- * Genesis
  , initialState
  ) where

import           Codec.Serialise (Serialise)
import           Data.ByteString (ByteString)
import           Data.Word       (Word64)
import           GHC.Generics    (Generic)

--------------------------------------------------------------------------------
-- Fundamental Types
--------------------------------------------------------------------------------

-- | Block height.
--
-- Represents the number of finalized blocks since Genesis.
--
-- FORMAL INVARIANT:
--   BlockHeight ≥ 0
--
-- NOTE:
-- - No Num instance on purpose
-- - Arithmetic on heights must be explicit and proven elsewhere
newtype BlockHeight = BlockHeight
  { unBlockHeight :: Word64
  }
  deriving stock (Eq, Ord, Show, Generic)
  deriving newtype (Enum, Serialise)

-- | Opaque cryptographic commitment to global protocol state.
--
-- The internal structure is:
-- - Defined in the consensus rules
-- - Proven correct in Agda
-- - NEVER interpreted here
newtype StateRoot = StateRoot
  { unStateRoot :: ByteString
  }
  deriving stock (Eq, Ord, Show, Generic)
  deriving newtype (Serialise)

--------------------------------------------------------------------------------
-- Protocol State
--------------------------------------------------------------------------------

-- | Canonical protocol state of the Aryabhata Layer-1.
--
-- This is the SINGLE source of truth for consensus state.
--
-- FORMAL INVARIANTS:
-- 1. psBlockHeight == number of finalized blocks
-- 2. psStateRoot commits to full protocol state
-- 3. State transitions are deterministic and total
data ProtocolState = ProtocolState
  { psBlockHeight :: !BlockHeight
    -- ^ Height of the latest finalized block
  , psStateRoot   :: !StateRoot
    -- ^ Commitment to global state
  }
  deriving stock (Eq, Show, Generic)
  deriving anyclass (Serialise)

--------------------------------------------------------------------------------
-- Genesis State
--------------------------------------------------------------------------------

-- | The unique protocol state at Genesis.
--
-- FORMAL INVARIANTS:
-- 1. psBlockHeight == 0
-- 2. psStateRoot is the empty commitment
-- 3. This value is unique and immutable
initialState :: ProtocolState
initialState = ProtocolState
  { psBlockHeight = BlockHeight 0
  , psStateRoot   = StateRoot mempty
  }
