module Aryabhata.Consensus.CNS where

-- Composite Node Score (CNS)
-- CNS = (0.60 * U) + (0.25 * R) + (0.15 * P)
-- U = Uptime Ratio        (last 100 epochs)
-- R = Restart Stability   = max(0, 1 - restarts/10)
-- P = Propagation Score   = 1 - (median_delay / 180)

data CnsStatus
    = Elite
    | Standard
    | Warned
    | Penalised
    | Expelled
    deriving (Show, Eq)

computeCns :: Double -> Double -> Double -> Double
computeCns u r p =
    (0.60 * clamp u) + (0.25 * clamp r) + (0.15 * clamp p)

clamp :: Double -> Double
clamp x = max 0.0 (min 1.0 x)

cnsStatus :: Double -> CnsStatus
cnsStatus s
    | s >= 0.80 = Elite
    | s >= 0.60 = Standard
    | s >= 0.40 = Warned
    | s >= 0.20 = Penalised
    | otherwise = Expelled

restartStability :: Int -> Double
restartStability restarts =
    max 0.0 (1.0 - fromIntegral restarts / 10.0)

propagationScore :: Double -> Double
propagationScore medianDelay =
    max 0.0 (1.0 - medianDelay / 180.0)

canMine :: Double -> Bool
canMine cns = cns >= 0.60

isElite :: Double -> Bool
isElite cns = cns >= 0.80

isExpelled :: Double -> Bool
isExpelled cns = cns < 0.20
