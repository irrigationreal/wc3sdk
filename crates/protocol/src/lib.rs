//! Protocol types shared between host tools and the in-process shim.
//!
//! This starts minimal on purpose. As the project grows, keep it:
//! - versioned
//! - backwards compatible where possible
//! - explicit about target build identity

/// Bump when the on-the-wire protocol changes incompatibly.
pub const PROTOCOL_VERSION: u32 = 1;

/// Identifies a specific target binary/build we support.
///
/// Keep this data *derived from the user's local file* (hash/version) and
/// never commit proprietary binaries into the repo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetIdentity {
    pub sha256: [u8; 32],
    pub file_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Ping,
    GetStatus,
    GetSnapshot(SnapshotRequest),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    Pong,
    Status(Status),
    Snapshot(Snapshot),
    Error(ProtocolError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub protocol_version: u32,
    pub connected: bool,
    pub target: Option<TargetIdentity>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotRequest {
    /// Named root configured in the SDK config.
    pub root: String,
    /// Max depth for recursive snapshotting.
    pub max_depth: u32,
    /// Whether to dereference pointer fields to nested objects.
    pub deref_pointers: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    /// Named root that was captured.
    pub root: String,
    /// JSON snapshot payload (schema defined by sdk::snapshot).
    pub json: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolError {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    Unknown = 0,
    NotConnected = 1,
    UnsupportedTarget = 2,
}
