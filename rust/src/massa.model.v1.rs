/// When an address is drawn to create an endorsement it is selected for a specific index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedSlot {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index in the slot
    #[prost(fixed64, tag = "2")]
    pub index: u64,
}
/// A point in time where a block is expected
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    /// Period
    #[prost(fixed64, tag = "1")]
    pub period: u64,
    /// Thread
    #[prost(fixed32, tag = "2")]
    pub thread: u32,
}
/// An endorsement, as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endorsement {
    /// Slot in which the endorsement can be included
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index inside the including block
    #[prost(fixed32, tag = "2")]
    pub index: u32,
    /// Hash of endorsed block
    /// This is the parent in thread `self.slot.thread` of the block in which the endorsement is included
    #[prost(string, tag = "3")]
    pub endorsed_block: ::prost::alloc::string::String,
}
/// Signed endorsement
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedEndorsement {
    /// Endorsement
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Endorsement>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// BytesMapFieldEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMapFieldEntry {
    /// bytes key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// bytes key
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Packages a type such that it can be securely sent and received in a trust-free network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecureShare {
    /// Content in sharable, deserializable form. Is used in the secure verification protocols
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_data: ::prost::alloc::vec::Vec<u8>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// The operation as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The fee they have decided for this operation
    #[prost(fixed64, tag = "1")]
    pub fee: u64,
    /// After `expire_period` slot the operation won't be included in a block
    #[prost(fixed64, tag = "2")]
    pub expire_period: u64,
    /// The type specific operation part
    #[prost(message, optional, tag = "3")]
    pub op: ::core::option::Option<OperationType>,
}
/// Type specific operation content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationType {
    /// Transfer coins from sender to recipient
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    /// The sender buys `roll_count` rolls. Roll price is defined in configuration
    #[prost(message, optional, tag = "2")]
    pub roll_buy: ::core::option::Option<RollBuy>,
    /// The sender sells `roll_count` rolls. Roll price is defined in configuration
    #[prost(message, optional, tag = "3")]
    pub roll_sell: ::core::option::Option<RollSell>,
    /// Execute a smart contract
    #[prost(message, optional, tag = "4")]
    pub execut_sc: ::core::option::Option<ExecuteSc>,
    /// Calls an exported function from a stored smart contract
    #[prost(message, optional, tag = "5")]
    pub call_sc: ::core::option::Option<CallSc>,
}
/// Transfer coins from sender to recipient
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Recipient address
    #[prost(string, tag = "1")]
    pub recipient_address: ::prost::alloc::string::String,
    /// Amount
    #[prost(fixed64, tag = "2")]
    pub amount: u64,
}
/// The sender buys `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollBuy {
    /// Roll count
    #[prost(fixed64, tag = "1")]
    pub roll_count: u64,
}
/// The sender sells `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollSell {
    /// Roll count
    #[prost(fixed64, tag = "1")]
    pub roll_count: u64,
}
/// Execute a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteSc {
    /// Smart contract bytecode.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The maximum of coins that could be spent by the operation sender
    #[prost(fixed64, tag = "2")]
    pub max_coins: u64,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(fixed64, tag = "3")]
    pub max_gas: u64,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "4")]
    pub datastore: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Calls an exported function from a stored smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallSc {
    /// Target smart contract address
    #[prost(string, tag = "1")]
    pub target_addr: ::prost::alloc::string::String,
    /// Target function name. No function is called if empty
    #[prost(string, tag = "2")]
    pub target_func: ::prost::alloc::string::String,
    /// Parameter to pass to the target function
    #[prost(bytes = "vec", tag = "3")]
    pub param: ::prost::alloc::vec::Vec<u8>,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(fixed64, tag = "4")]
    pub max_gas: u64,
    /// Extra coins that are spent from the caller's balance and transferred to the target
    #[prost(fixed64, tag = "5")]
    pub coins: u64,
}
/// Signed operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedOperation {
    /// Operation
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Operation>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// A wrapper around an operation with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationWrapper {
    /// The unique ID of the operation.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The IDs of the blocks in which the operation appears
    #[prost(string, repeated, tag = "3")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The thread in which the operation can be included
    #[prost(fixed32, tag = "5")]
    pub thread: u32,
    /// The operation object itself
    #[prost(message, optional, tag = "6")]
    pub operation: ::core::option::Option<SignedOperation>,
    /// The execution statuses of the operation
    #[prost(enumeration = "OperationStatus", repeated, tag = "7")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for an operation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationStatus {
    /// Default enum value
    Unspecified = 0,
    /// The operation is still pending
    Pending = 1,
    /// The operation is final
    Final = 2,
    /// The operation was executed successfully
    Success = 3,
    /// The operation failed to execute
    Failure = 4,
    /// The status of the operation is unknown
    Unknown = 5,
}
impl OperationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationStatus::Unspecified => "OPERATION_STATUS_UNSPECIFIED",
            OperationStatus::Pending => "OPERATION_STATUS_PENDING",
            OperationStatus::Final => "OPERATION_STATUS_FINAL",
            OperationStatus::Success => "OPERATION_STATUS_SUCCESS",
            OperationStatus::Failure => "OPERATION_STATUS_FAILURE",
            OperationStatus::Unknown => "OPERATION_STATUS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATUS_PENDING" => Some(Self::Pending),
            "OPERATION_STATUS_FINAL" => Some(Self::Final),
            "OPERATION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_STATUS_FAILURE" => Some(Self::Failure),
            "OPERATION_STATUS_UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// Block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations ids
    #[prost(string, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Filled block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledBlock {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FilledOperationTuple>,
}
/// Block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// parents
    #[prost(string, repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All operations hash
    #[prost(string, tag = "3")]
    pub operation_merkle_root: ::prost::alloc::string::String,
    /// Signed endorsements
    #[prost(message, repeated, tag = "4")]
    pub endorsements: ::prost::alloc::vec::Vec<SignedEndorsement>,
}
/// Filled Operation Tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledOperationTuple {
    /// Operation id
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// Signed operation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<SignedOperation>,
}
/// Signed block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlock {
    /// Block
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Block>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// Signed block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlockHeader {
    /// BlockHeader
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<BlockHeader>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// A wrapper around a block with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWrapper {
    /// The unique ID of the block.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The block object itself
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    /// The execution statuses of the block
    #[prost(enumeration = "BlockStatus", repeated, tag = "3")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for a block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    /// Default enum value
    Unspecified = 0,
    /// The block is in the greatest clique (and not final)
    InBlockclique = 1,
    /// The block is final
    Final = 2,
    /// The block is candidate (active any clique but not final)
    Candidate = 3,
    /// The block is discarded
    Discarded = 4,
}
impl BlockStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockStatus::Unspecified => "BLOCK_STATUS_UNSPECIFIED",
            BlockStatus::InBlockclique => "BLOCK_STATUS_IN_BLOCKCLIQUE",
            BlockStatus::Final => "BLOCK_STATUS_FINAL",
            BlockStatus::Candidate => "BLOCK_STATUS_CANDIDATE",
            BlockStatus::Discarded => "BLOCK_STATUS_DISCARDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BLOCK_STATUS_IN_BLOCKCLIQUE" => Some(Self::InBlockclique),
            "BLOCK_STATUS_FINAL" => Some(Self::Final),
            "BLOCK_STATUS_CANDIDATE" => Some(Self::Candidate),
            "BLOCK_STATUS_DISCARDED" => Some(Self::Discarded),
            _ => None,
        }
    }
}
/// Selector draws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDraws {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Next block draws
    #[prost(message, repeated, tag = "2")]
    pub next_block_draws: ::prost::alloc::vec::Vec<Slot>,
    /// Next endorsements draws
    #[prost(message, repeated, tag = "3")]
    pub next_endorsement_draws: ::prost::alloc::vec::Vec<IndexedSlot>,
}
/// SlotExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotExecutionOutput {
    /// Status
    #[prost(enumeration = "ExecutionOutputStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
    /// Executed slot output
    #[prost(message, optional, tag = "2")]
    pub execution_output: ::core::option::Option<ExecutionOutput>,
}
/// FinalizedExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizedExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// ExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Block id at that slot (optional)
    #[prost(string, optional, tag = "2")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Events emitted by the execution step
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<ScExecutionEvent>,
    /// State changes caused by the execution step
    #[prost(message, optional, tag = "4")]
    pub state_changes: ::core::option::Option<StateChanges>,
}
/// ScExecutionEvent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEvent {
    /// Sc execution context
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<ScExecutionEventContext>,
    /// json data string
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// ScExecutionEvent context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventContext {
    /// base58 encoded slot(period + thread) + index_in_slot
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// When was it generated
    #[prost(message, optional, tag = "2")]
    pub origin_slot: ::core::option::Option<Slot>,
    /// Block id if there was a block at that slot (optional)
    #[prost(string, optional, tag = "3")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Index of the event in the slot
    #[prost(fixed64, tag = "4")]
    pub index_in_slot: u64,
    /// Call stack addresses. most recent at the end
    #[prost(string, repeated, tag = "5")]
    pub call_stack: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Origin operation id (optional)
    #[prost(string, optional, tag = "6")]
    pub origin_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "7")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// StateChanges
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateChanges {
    /// Ledger changes
    #[prost(message, repeated, tag = "1")]
    pub ledger_changes: ::prost::alloc::vec::Vec<LedgerChangeEntry>,
    /// Asynchronous pool changes
    #[prost(message, repeated, tag = "2")]
    pub async_pool_changes: ::prost::alloc::vec::Vec<AsyncPoolChangeEntry>,
    /// Executed operations changes
    #[prost(message, repeated, tag = "4")]
    pub executed_ops_changes: ::prost::alloc::vec::Vec<ExecutedOpsChangeEntry>,
    /// Executed denunciations changes
    #[prost(message, repeated, tag = "5")]
    pub executed_denunciations_changes: ::prost::alloc::vec::Vec<DenunciationIndex>,
}
/// ExecutedOpsChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeEntry {
    /// OperationId
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// ExecutedOpsChangeValue
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ExecutedOpsChangeValue>,
}
/// ExecutedOpsChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeValue {
    /// The status of the execution of the operation
    #[prost(enumeration = "OperationExecutionStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
    /// Slot until which the operation remains valid (included)
    #[prost(message, optional, tag = "2")]
    pub slot: ::core::option::Option<Slot>,
}
/// AsyncPoolChange Entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeEntry {
    /// Async message id
    #[prost(string, tag = "1")]
    pub async_message_id: ::prost::alloc::string::String,
    /// AsyncPool message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AsyncPoolChangeValue>,
}
/// AsyncPoolChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeValue {
    /// The type of the change
    #[prost(enumeration = "AsyncPoolChangeType", tag = "1")]
    pub r#type: i32,
    /// AsyncPool message
    #[prost(message, optional, tag = "2")]
    pub async_message: ::core::option::Option<AsyncMessage>,
}
/// Asynchronous smart contract message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessage {
    /// Slot at which the message was emitted
    #[prost(message, optional, tag = "1")]
    pub emission_slot: ::core::option::Option<Slot>,
    /// Index of the emitted message within the `emission_slot`.
    /// This is used for disambiguate the emission of multiple messages at the same slot.
    #[prost(fixed64, tag = "2")]
    pub emission_index: u64,
    /// The address that sent the message
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// The address towards which the message is being sent
    #[prost(string, tag = "4")]
    pub destination: ::prost::alloc::string::String,
    /// the handler function name within the destination address' bytecode
    #[prost(string, tag = "5")]
    pub handler: ::prost::alloc::string::String,
    /// Maximum gas to use when processing the message
    #[prost(fixed64, tag = "6")]
    pub max_gas: u64,
    /// Fee paid by the sender when the message is processed.
    #[prost(fixed64, tag = "7")]
    pub fee: u64,
    /// Coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(fixed64, tag = "8")]
    pub coins: u64,
    /// Slot at which the message starts being valid (bound included in the validity range)
    #[prost(message, optional, tag = "9")]
    pub validity_start: ::core::option::Option<Slot>,
    /// Slot at which the message stops being valid (bound not included in the validity range)
    #[prost(message, optional, tag = "10")]
    pub validity_end: ::core::option::Option<Slot>,
    /// Raw payload data of the message
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Trigger that define whenever a message can be executed
    #[prost(message, optional, tag = "12")]
    pub trigger: ::core::option::Option<AsyncMessageTrigger>,
    /// Boolean that determine if the message can be executed. For messages without filter this boolean is always true.
    /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
    #[prost(bool, tag = "13")]
    pub can_be_executed: bool,
    /// Hash of the message
    #[prost(string, tag = "14")]
    pub hash: ::prost::alloc::string::String,
}
/// Structure defining a trigger for an asynchronous message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageTrigger {
    /// Filter on the address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Filter on the datastore key (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub datastore_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// LedgerChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Ledger message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<LedgerChangeValue>,
}
/// LedgerChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeValue {
    /// The type of the change
    #[prost(enumeration = "LedgerChangeType", tag = "1")]
    pub r#type: i32,
    /// LedgerEntry or LedgerEntryUpdate
    #[prost(oneof = "ledger_change_value::Entry", tags = "2, 3")]
    pub entry: ::core::option::Option<ledger_change_value::Entry>,
}
/// Nested message and enum types in `LedgerChangeValue`.
pub mod ledger_change_value {
    /// LedgerEntry or LedgerEntryUpdate
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Created ledger entry
        #[prost(message, tag = "2")]
        CreatedEntry(super::LedgerEntry),
        /// Update ledger entry
        #[prost(message, tag = "3")]
        UpdatedEntry(super::LedgerEntryUpdate),
    }
}
/// An entry associated to an address in the `FinalLedger`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntry {
    /// The balance of that entry
    #[prost(fixed64, tag = "1")]
    pub balance: u64,
    /// Executable bytecode
    #[prost(bytes = "vec", tag = "2")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Represents an update to one or more fields of a `LedgerEntry`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntryUpdate {
    /// Change the balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<SetOrKeepBalance>,
    /// Change the executable bytecode
    #[prost(message, optional, tag = "2")]
    pub bytecode: ::core::option::Option<SetOrKeepBytecode>,
    /// / Change datastore entries
    #[prost(message, repeated, tag = "3")]
    pub datastore: ::prost::alloc::vec::Vec<SetOrDeleteDatastoreEntry>,
}
/// Set or Keep Balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBalance {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optional)
    #[prost(fixed64, optional, tag = "2")]
    pub balance: ::core::option::Option<u64>,
}
/// Set or Keep Bytecode
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytecode {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// Executable bytecode (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub bytecode: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Set or Delete DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrDeleteDatastoreEntry {
    /// The type of the change
    #[prost(enumeration = "SetOrDeleteType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optioal)
    #[prost(message, optional, tag = "2")]
    pub datastore_entry: ::core::option::Option<BytesMapFieldEntry>,
}
/// Index for Denunciations in collections (e.g. like a HashMap...)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationIndex {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[prost(oneof = "denunciation_index::Entry", tags = "1, 2")]
    pub entry: ::core::option::Option<denunciation_index::Entry>,
}
/// Nested message and enum types in `DenunciationIndex`.
pub mod denunciation_index {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Denunciation block header
        #[prost(message, tag = "1")]
        BlockHeader(super::DenunciationBlockHeader),
        /// Denunciation endorsement
        #[prost(message, tag = "2")]
        Endorsement(super::DenunciationEndorsement),
    }
}
/// Variant for Block header denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationBlockHeader {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// Variant for Endorsement denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationEndorsement {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Denounciation index
    #[prost(fixed32, tag = "2")]
    pub index: u32,
}
/// ScExecutionEventStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScExecutionEventStatus {
    /// Default enum value
    Unspecified = 0,
    /// Final status
    Final = 1,
    /// Read only status
    ReadOnly = 2,
    /// Failure status
    Failure = 3,
}
impl ScExecutionEventStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScExecutionEventStatus::Unspecified => {
                "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED"
            }
            ScExecutionEventStatus::Final => "SC_EXECUTION_EVENT_STATUS_FINAL",
            ScExecutionEventStatus::ReadOnly => "SC_EXECUTION_EVENT_STATUS_READ_ONLY",
            ScExecutionEventStatus::Failure => "SC_EXECUTION_EVENT_STATUS_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SC_EXECUTION_EVENT_STATUS_FINAL" => Some(Self::Final),
            "SC_EXECUTION_EVENT_STATUS_READ_ONLY" => Some(Self::ReadOnly),
            "SC_EXECUTION_EVENT_STATUS_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// ExecutionOutputStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionOutputStatus {
    /// Default enum value
    Unspecified = 0,
    /// Candidate status
    Candidate = 1,
    /// Final status
    Final = 2,
}
impl ExecutionOutputStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionOutputStatus::Unspecified => "EXECUTION_OUTPUT_STATUS_UNSPECIFIED",
            ExecutionOutputStatus::Candidate => "EXECUTION_OUTPUT_STATUS_CANDIDATE",
            ExecutionOutputStatus::Final => "EXECUTION_OUTPUT_STATUS_FINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_OUTPUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_OUTPUT_STATUS_CANDIDATE" => Some(Self::Candidate),
            "EXECUTION_OUTPUT_STATUS_FINAL" => Some(Self::Final),
            _ => None,
        }
    }
}
/// OperationExecutionStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationExecutionStatus {
    /// Default enum value
    Unspecified = 0,
    /// Success status
    Success = 1,
    /// Failed only status
    Failed = 2,
}
impl OperationExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationExecutionStatus::Unspecified => {
                "OPERATION_EXECUTION_STATUS_UNSPECIFIED"
            }
            OperationExecutionStatus::Success => "OPERATION_EXECUTION_STATUS_SUCCESS",
            OperationExecutionStatus::Failed => "OPERATION_EXECUTION_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_EXECUTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_EXECUTION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_EXECUTION_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// AsyncPoolChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AsyncPoolChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Add type
    Add = 1,
    /// Activate only type
    Activate = 2,
    /// Delete only type
    Delete = 3,
}
impl AsyncPoolChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AsyncPoolChangeType::Unspecified => "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED",
            AsyncPoolChangeType::Add => "ASYNC_POOL_CHANGE_TYPE_ADD",
            AsyncPoolChangeType::Activate => "ASYNC_POOL_CHANGE_TYPE_ACTIVATE",
            AsyncPoolChangeType::Delete => "ASYNC_POOL_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASYNC_POOL_CHANGE_TYPE_ADD" => Some(Self::Add),
            "ASYNC_POOL_CHANGE_TYPE_ACTIVATE" => Some(Self::Activate),
            "ASYNC_POOL_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// LedgerChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LedgerChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Set type
    Set = 1,
    /// Update type
    Update = 2,
    /// Delete type
    Delete = 3,
}
impl LedgerChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LedgerChangeType::Unspecified => "LEDGER_CHANGE_TYPE_UNSPECIFIED",
            LedgerChangeType::Set => "LEDGER_CHANGE_TYPE_SET",
            LedgerChangeType::Update => "LEDGER_CHANGE_TYPE_UPDATE",
            LedgerChangeType::Delete => "LEDGER_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEDGER_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LEDGER_CHANGE_TYPE_SET" => Some(Self::Set),
            "LEDGER_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "LEDGER_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// SetOrKeepType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrKeepType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Keeps the existing value
    Keep = 2,
}
impl SetOrKeepType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrKeepType::Unspecified => "SET_OR_KEEP_TYPE_UNSPECIFIED",
            SetOrKeepType::Set => "SET_OR_KEEP_TYPE_SET",
            SetOrKeepType::Keep => "SET_OR_KEEP_TYPE_KEEP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_KEEP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_KEEP_TYPE_SET" => Some(Self::Set),
            "SET_OR_KEEP_TYPE_KEEP" => Some(Self::Keep),
            _ => None,
        }
    }
}
/// SetOrDeleteType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrDeleteType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Deletes the existing value
    Delete = 2,
}
impl SetOrDeleteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrDeleteType::Unspecified => "SET_OR_DELETE_TYPE_UNSPECIFIED",
            SetOrDeleteType::Set => "SET_OR_DELETE_TYPE_SET",
            SetOrDeleteType::Delete => "SET_OR_DELETE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_DELETE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_DELETE_TYPE_SET" => Some(Self::Set),
            "SET_OR_DELETE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
