module Aryabhata.Proof.UTXOTests where

import Aryabhata.Consensus.UTXO
import Aryabhata.Consensus.Reward

-- Spent UTXO cannot be spent again
prop_noDoubleSpend :: Bool
prop_noDoubleSpend =
    let entry = UtxoEntry [1,2,3] 0 10.0 [4,5,6] False
        utxos  = [entry]
        spent  = markSpent utxos [1,2,3] 0
    in not (isUnspent spent [1,2,3] 0)

-- Balance never negative
prop_balanceNonNegative :: Bool
prop_balanceNonNegative =
    let entry = UtxoEntry [1,2,3] 0 5.0 [4,5,6] False
        utxos  = [entry]
    in balanceOf utxos [4,5,6] >= 0.0

-- Spent UTXO excluded from balance
prop_spentExcludedFromBalance :: Bool
prop_spentExcludedFromBalance =
    let entry = UtxoEntry [1,2,3] 0 5.0 [4,5,6] False
        utxos  = [entry]
        spent  = markSpent utxos [1,2,3] 0
    in balanceOf spent [4,5,6] == 0.0

-- Miner and treasury shares sum to total fee
prop_feeSharesSumToTotal :: Double -> Bool
prop_feeSharesSumToTotal fee =
    sharesValid (abs fee)

-- Miner gets 98% of fee
prop_minerShareCorrect :: Bool
prop_minerShareCorrect =
    minerFeeShare 100.0 == 98.0

-- Treasury gets 2% of fee
prop_treasuryShareCorrect :: Bool
prop_treasuryShareCorrect =
    treasuryFeeShare 100.0 == 2.0
