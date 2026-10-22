use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    PlayerNotFound = 3,
    NotFound = 4,
    InvalidInput = 5,
    AlreadyVerified = 6,
    InsufficientFee = 7,
    /// Scout has no active subscription (or the subscription has already been
    /// cancelled).  Used by `cancel_subscription` and any access-guard that
    /// requires a live subscription.
    NotSubscribed = 8,
    Unauthorized = 9,
    ContractPaused = 10,
    Overflow = 11,
    /// Not enough unsold tokens remain to fulfil the requested purchase.
    /// Used by `player_token::buy_token` when `amount` exceeds the player's
    /// remaining supply (`total_supply - sold`).
    InsufficientSupply = 12,

    // ── Granular errors (#1530) ────────────────────────────────────────────
    //
    // APPEND ONLY. These discriminants are part of the on-chain ABI: existing
    // values must never be renumbered or reused, and each backend mapping in
    // src/utils/contractErrors.ts must stay in sync.

    /// The caller is not a registered validator (progress contract).
    ValidatorNotRegistered = 13,
    /// The validator was revoked and may no longer submit or approve milestones.
    ValidatorRevoked = 14,
    /// No milestone exists with the requested id.
    MilestoneNotFound = 15,
    /// The milestone has already been approved and cannot be approved twice.
    MilestoneAlreadyApproved = 16,
    /// A validator attempted to approve a milestone they submitted themselves.
    SelfApprovalForbidden = 17,
    /// The scout's subscription expired before the requested operation.
    SubscriptionExpired = 18,
    /// The scout has already paid to contact this player.
    ContactAlreadyPaid = 19,
    /// The supplied platform fee is outside the allowed 0–10 000 bps range.
    FeeBpsOutOfRange = 20,
    /// No connection exists for the given scout and player.
    ConnectionNotFound = 21,
    /// The connection is already closed and cannot be closed again.
    ConnectionAlreadyClosed = 22,
}
