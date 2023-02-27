use serde_derive::{Serialize, Deserialize};

/// A message passed between Raft nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// The current term of the sender.
    pub term: u64,
    /// The sender address.
    pub src_addr: Address,
    /// The recipient address.
    pub dst_addr: Address,
    /// The message event.
    pub event: Event,
}

/// A message address.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Address {
    /// Broadcast to all peers.
    Peers,
    /// A remote peer.
    Peer(String),
    /// The local node.
    Local,
    /// A local client.
    Client,
}

/// An event contained within messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Leaders send periodic heartbeats to its followers.
    Heartbeat {
        /// The index of the leader's last committed log entry.
        commit_index: u64,
        /// The term of the leader's last committed log entry.
        commit_term: u64,
    },

    /// Candidates solicit votes from all peers.
    SolicitVote {
        // The index of the candidate's last stored log entry
        last_log_index: u64,
        // The term of the candidate's last stored log entry
        last_log_term: u64,
    },
    
    /// Followers may grant votes to candidates.
    GrantVote,
}

/// A client request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Request {
    
}

/// A client response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Response {
    
}
