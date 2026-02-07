{-# LANGUAGE DeriveAnyClass     #-}
{-# LANGUAGE DeriveGeneric      #-}
{-# LANGUAGE DerivingStrategies #-}
{-# LANGUAGE StrictData         #-}

-- |
-- Module: Aryabhata.Consensus.Types
--
-- Optimized and safe domain types for Aryabhata Layer-1 consensus.
--
-- GUARANTEES:
-- • Fixed-size (32-byte) invariants enforced
-- • Opaque, non-interchangeable types
-- • Canonical binary serialization
-- • Safe construction only
--
-- This module is IMMUTABLE after Genesis.

module Aryabhata.Consensus.Types
  ( Hash32
  , Address
  , mkHash32
  , mkAddress
  , hashToBytes     -- FFI / transport only
  , addressToBytes  -- FFI / transport only
  ) where

import           Codec.Serialise           (Serialise)
import           Control.DeepSeq           (NFData)
import           Data.ByteString           (ByteString)
import qualified Data.ByteString           as BS
import           Data.ByteString.Short     (ShortByteString)
import qualified Data.ByteString.Short     as SBS
import           GHC.Generics              (Generic)

--------------------------------------------------------------------------------
-- Validation Logic
--------------------------------------------------------------------------------

expectedLength :: Int
expectedLength = 32

validateSize :: ByteString -> Maybe ShortByteString
validateSize bs
  | BS.length bs == expectedLength = Just (SBS.toShort bs)
  | otherwise                      = Nothing

--------------------------------------------------------------------------------
-- Hash32
--------------------------------------------------------------------------------

newtype Hash32 = Hash32 ShortByteString
  deriving stock (Eq, Ord, Show, Generic)
  deriving anyclass (Serialise, NFData)

mkHash32 :: ByteString -> Maybe Hash32
mkHash32 = fmap Hash32 . validateSize

-- | Extract raw bytes (transport / FFI only).
hashToBytes :: Hash32 -> ByteString
hashToBytes (Hash32 sbs) = SBS.fromShort sbs

--------------------------------------------------------------------------------
-- Address
--------------------------------------------------------------------------------

newtype Address = Address ShortByteString
  deriving stock (Eq, Ord, Show, Generic)
  deriving anyclass (Serialise, NFData)

mkAddress :: ByteString -> Maybe Address
mkAddress = fmap Address . validateSize

-- | Extract raw bytes (transport / FFI only).
addressToBytes :: Address -> ByteString
addressToBytes (Address sbs) = SBS.fromShort sbs
