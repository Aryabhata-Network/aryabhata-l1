module Aryabhata.Consensus.UTXO where

import Aryabhata.Types.Transaction

-- UTXO set management
-- Pure functional — no mutable state
-- Prevents double spend by construction

data UtxoEntry = UtxoEntry
    { utxoTxHash    :: [Int]
    , utxoIndex     :: Int
    , utxoAmount    :: Double
    , utxoOwner     :: [Int]
    , utxoIsSpent   :: Bool
    } deriving (Show, Eq)

type UtxoSet = [UtxoEntry]

-- Check if UTXO exists and is unspent
isUnspent :: UtxoSet -> [Int] -> Int -> Bool
isUnspent utxos txHash index =
    any (\u ->
        utxoTxHash u == txHash &&
        utxoIndex u  == index  &&
        not (utxoIsSpent u)
    ) utxos

-- Mark UTXO as spent
markSpent :: UtxoSet -> [Int] -> Int -> UtxoSet
markSpent utxos txHash index =
    map (\u ->
        if utxoTxHash u == txHash && utxoIndex u == index
            then u { utxoIsSpent = True }
            else u
    ) utxos

-- Add new UTXO
addUtxo :: UtxoSet -> UtxoEntry -> UtxoSet
addUtxo utxos entry = entry : utxos

-- Get balance for owner
balanceOf :: UtxoSet -> [Int] -> Double
balanceOf utxos owner =
    sum
    [ utxoAmount u
    | u <- utxos
    , utxoOwner u == owner
    , not (utxoIsSpent u)
    ]
