module Aryabhata.Consensus.Penalty where

import Aryabhata.Consensus.CNS

-- Automatic penalty system
-- No human intervention — ever
-- Escalating, irreversible enforcement

data Violation
    = InvalidBlockSubmit
    | SubmitWhileWarned
    | TimestampManipulation
    | DoubleSpendAttempt
    | PeerProtocolViolation
    deriving (Show, Eq)

data PenaltyResult
    = CnsPenalty Double
    | SubmissionBan Int
    | PermanentBan
    deriving (Show, Eq)

-- Apply penalty based on violation type
applyPenalty :: Violation -> Double -> (Double, PenaltyResult)
applyPenalty violation currentCns =
    case violation of
        InvalidBlockSubmit ->
            let newCns = max 0.0 (currentCns - 0.05)
            in (newCns, CnsPenalty 0.05)

        SubmitWhileWarned ->
            let newCns = max 0.0 (currentCns - 0.10)
            in (newCns, SubmissionBan 12)

        TimestampManipulation ->
            let newCns = max 0.0 (currentCns - 0.20)
            in (newCns, SubmissionBan 48)

        DoubleSpendAttempt ->
            (0.0, PermanentBan)

        PeerProtocolViolation ->
            let newCns = max 0.0 (currentCns - 0.05)
            in (newCns, CnsPenalty 0.05)

-- Check if node should be expelled
shouldExpel :: Double -> Int -> Bool
shouldExpel cns epochsBelow =
    cns < 0.20 && epochsBelow >= 10

-- Treasury slash on expulsion
slashAmount :: Double -> Double
slashAmount stake = stake * 0.50
