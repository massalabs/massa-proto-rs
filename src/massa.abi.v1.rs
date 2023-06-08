/// Error message used in abi Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// a string representing the error
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Create SC request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScRequest {
    /// Bytecode is the compiled code of the smart contract
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
}
/// Create SC result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScResult {
    /// Address of the just created smart contract
    #[prost(message, optional, tag = "1")]
    pub sc_address: ::core::option::Option<super::super::model::v1::NativeAddress>,
}
/// Call SC request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallRequest {
    /// Address of the smart contract to call
    #[prost(message, optional, tag = "1")]
    pub target_sc_address: ::core::option::Option<
        super::super::model::v1::NativeAddress,
    >,
    /// Function to call in the targeted smart contract
    #[prost(string, tag = "2")]
    pub target_function_name: ::prost::alloc::string::String,
    /// Argument to the function serialized in a byte array.
    #[prost(bytes = "vec", tag = "3")]
    pub function_arg: ::prost::alloc::vec::Vec<u8>,
    /// call_coins is the amount of coins to pay for the call
    #[prost(message, optional, tag = "4")]
    pub call_coins: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Call SC response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallResponse {
    /// Return_data is the return value of the function
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// LocalCall
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalCallRequest {
    /// Address of the smart contract to call
    #[prost(message, optional, tag = "1")]
    pub target_sc_address: ::core::option::Option<
        super::super::model::v1::NativeAddress,
    >,
    /// Function to call in the targeted smart contract
    #[prost(string, tag = "2")]
    pub target_function_name: ::prost::alloc::string::String,
    /// Argument to the function serialized in a byte array.
    #[prost(bytes = "vec", tag = "3")]
    pub function_arg: ::prost::alloc::vec::Vec<u8>,
}
/// LocalCall response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalCallResponse {
    /// Return_data is the return value of the function
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Generate event request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEventRequest {
    /// Event
    #[prost(string, tag = "1")]
    pub event: ::prost::alloc::string::String,
}
/// Generate event resulst
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEventResult {}
/// Transfer coins request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCoinsRequest {
    /// The address of the recipient
    #[prost(message, optional, tag = "1")]
    pub target_address: ::core::option::Option<super::super::model::v1::NativeAddress>,
    /// The amount of coins to transfer
    #[prost(message, optional, tag = "2")]
    pub amount_to_transfer: ::core::option::Option<
        super::super::model::v1::NativeAmount,
    >,
}
/// Transfer coins result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCoinsResult {}
/// Function exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExistsRequest {
    /// Address of the smart contract to call
    #[prost(message, optional, tag = "1")]
    pub target_sc_address: ::core::option::Option<
        super::super::model::v1::NativeAddress,
    >,
    /// The name of the function to check the existance of
    #[prost(string, tag = "2")]
    pub function_name: ::prost::alloc::string::String,
}
/// Function exists result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExistsResult {
    /// Exists is true if the function exists
    #[prost(bool, tag = "1")]
    pub exists: bool,
}
/// Message returned in case of success of an ABI call.
/// See the documentation of the specific result for the meaning of the fields.
/// Call RespResult because Result will conflict with the Result type in Rust
/// data MUST have the same name as they type in Rust they are converted to type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespResult {
    #[prost(
        oneof = "resp_result::Res",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39"
    )]
    pub res: ::core::option::Option<resp_result::Res>,
}
/// Nested message and enum types in `RespResult`.
pub mod resp_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Res {
        #[prost(message, tag = "1")]
        NativeAddressToStringResult(super::NativeAddressToStringResult),
        #[prost(message, tag = "2")]
        NativePubKeyToStringResult(super::NativePubKeyToStringResult),
        #[prost(message, tag = "3")]
        NativeSigToStringResult(super::NativeSigToStringResult),
        #[prost(message, tag = "4")]
        NativeHashToStringResult(super::NativeHashToStringResult),
        #[prost(message, tag = "5")]
        NativeAmountToStringResult(super::NativeAmountToStringResult),
        #[prost(message, tag = "6")]
        NativeAddressFromStringResult(super::NativeAddressFromStringResult),
        #[prost(message, tag = "7")]
        NativePubKeyFromStringResult(super::NativePubKeyFromStringResult),
        #[prost(message, tag = "8")]
        NativeSigFromStringResult(super::NativeSigFromStringResult),
        #[prost(message, tag = "9")]
        NativeHashFromStringResult(super::NativeHashFromStringResult),
        #[prost(message, tag = "10")]
        NativeAmountFromStringResult(super::NativeAmountFromStringResult),
        #[prost(message, tag = "11")]
        CheckNativeAddressResult(super::CheckNativeAddressResult),
        #[prost(message, tag = "12")]
        CheckNativePubKeyResult(super::CheckNativePubKeyResult),
        #[prost(message, tag = "13")]
        CheckNativeSigResult(super::CheckNativeSigResult),
        #[prost(message, tag = "14")]
        CheckNativeHashResult(super::CheckNativeHashResult),
        #[prost(message, tag = "15")]
        CheckNativeAmountResult(super::CheckNativeAmountResult),
        #[prost(message, tag = "16")]
        AddNativeAmountsResult(super::AddNativeAmountsResult),
        #[prost(message, tag = "17")]
        SubNativeAmountsResult(super::SubNativeAmountsResult),
        #[prost(message, tag = "18")]
        MulNativeAmountResult(super::MulNativeAmountResult),
        #[prost(message, tag = "19")]
        ScalarDivRemNativeAmountResult(super::ScalarDivRemNativeAmountResult),
        #[prost(message, tag = "20")]
        DivRemNativeAmountResult(super::DivRemNativeAmountResult),
        #[prost(message, tag = "21")]
        CheckedAddNativeTimeResult(super::CheckedAddNativeTimeResult),
        #[prost(message, tag = "22")]
        CheckedSubNativeTimeResult(super::CheckedSubNativeTimeResult),
        #[prost(message, tag = "23")]
        CheckedMulNativeTimeResult(super::CheckedMulNativeTimeResult),
        #[prost(message, tag = "24")]
        CheckedScalarDivRemNativeTimeResult(super::CheckedScalarDivRemNativeTimeResult),
        #[prost(message, tag = "25")]
        CheckedDivRemNativeTimeResult(super::CheckedDivRemNativeTimeResult),
        #[prost(message, tag = "26")]
        CompareNativeTimeResult(super::CompareNativeTimeResult),
        #[prost(message, tag = "27")]
        CompareNativeAddressResult(super::CompareNativeAddressResult),
        #[prost(message, tag = "28")]
        CompareNativePubKeyResult(super::CompareNativePubKeyResult),
        #[prost(message, tag = "29")]
        CompareNativeSigResult(super::CompareNativeSigResult),
        #[prost(message, tag = "30")]
        VerifyNativeSigResult(super::VerifyNativeSigResult),
        #[prost(message, tag = "31")]
        CompareNativeAmountResult(super::CompareNativeAmountResult),
        #[prost(message, tag = "32")]
        Keccak256Result(super::Keccak256Result),
        #[prost(message, tag = "33")]
        VerifyEvmSigResult(super::VerifyEvmSigResult),
        #[prost(message, tag = "34")]
        VerifyBlsSingleSigResult(super::VerifyBlsSingleSigResult),
        #[prost(message, tag = "35")]
        VerifyBlsMultiSigResult(super::VerifyBlsMultiSigResult),
        #[prost(message, tag = "36")]
        TransferCoinsResult(super::TransferCoinsResult),
        #[prost(message, tag = "37")]
        GenerateEventResult(super::GenerateEventResult),
        #[prost(message, tag = "38")]
        CreateScResult(super::CreateScResult),
        #[prost(message, tag = "39")]
        FunctionExistsResult(super::FunctionExistsResult),
    }
}
/// Generic message that encapsulate response from ABI calls.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbiResponse {
    #[prost(oneof = "abi_response::Resp", tags = "1, 2")]
    pub resp: ::core::option::Option<abi_response::Resp>,
}
/// Nested message and enum types in `AbiResponse`.
pub mod abi_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resp {
        /// variant for success
        #[prost(message, tag = "1")]
        Res(super::RespResult),
        /// variant for error
        #[prost(message, tag = "2")]
        Error(super::Error),
    }
}
/// Address to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAddressToStringRequest {
    /// Address to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativeAddress>,
}
/// Address to string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAddressToStringResult {
    /// Converted address
    #[prost(string, tag = "1")]
    pub converted_address: ::prost::alloc::string::String,
}
/// PubKey to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePubKeyToStringRequest {
    /// PubKey to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativePubKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePubKeyToStringResult {
    /// Converted PubKey
    #[prost(string, tag = "1")]
    pub converted_pubkey: ::prost::alloc::string::String,
}
/// Sig to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeSigToStringRequest {
    /// Sig to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativeSig>,
}
/// Sig to string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeSigToStringResult {
    /// Converted Sig
    #[prost(string, tag = "1")]
    pub converted_sig: ::prost::alloc::string::String,
}
/// Hash to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeHashToStringRequest {
    /// Hash to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativeHash>,
}
/// Hash to string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeHashToStringResult {
    /// Converted Hash
    #[prost(string, tag = "1")]
    pub converted_hash: ::prost::alloc::string::String,
}
/// Amount to string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountToStringRequest {
    /// Amount to convert to string
    #[prost(message, optional, tag = "1")]
    pub to_convert: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount to string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountToStringResult {
    /// Converted Amount
    #[prost(string, tag = "1")]
    pub converted_amount: ::prost::alloc::string::String,
}
/// Address from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAddressFromStringRequest {
    /// String to convert to address
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// Address from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAddressFromStringResult {
    /// Converted address
    #[prost(message, optional, tag = "1")]
    pub converted_address: ::core::option::Option<
        super::super::model::v1::NativeAddress,
    >,
}
/// PubKey from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePubKeyFromStringRequest {
    /// String to convert to PubKey
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// PubKey from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePubKeyFromStringResult {
    /// Converted PubKey
    #[prost(message, optional, tag = "1")]
    pub converted_pubkey: ::core::option::Option<super::super::model::v1::NativePubKey>,
}
/// Sig from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeSigFromStringRequest {
    /// String to convert to Sig
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// Sig from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeSigFromStringResult {
    /// Converted Sig
    #[prost(message, optional, tag = "1")]
    pub converted_sig: ::core::option::Option<super::super::model::v1::NativeSig>,
}
/// Hash from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeHashFromStringRequest {
    /// String to convert to Hash
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// Hash from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeHashFromStringResult {
    /// Converted Hash
    #[prost(message, optional, tag = "1")]
    pub converted_hash: ::core::option::Option<super::super::model::v1::NativeHash>,
}
/// Amount from string request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountFromStringRequest {
    /// String to convert to Amount
    #[prost(string, tag = "1")]
    pub to_convert: ::prost::alloc::string::String,
}
/// Amount from string result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeAmountFromStringResult {
    /// Converted Amount
    #[prost(message, optional, tag = "1")]
    pub converted_amount: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Check address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAddressRequest {
    /// Address to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativeAddress>,
}
/// Check address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAddressResult {
    /// Is address valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check PubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativePubKeyRequest {
    /// PubKey to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativePubKey>,
}
/// Check PubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativePubKeyResult {
    /// Is PubKey valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeSigRequest {
    /// Sig to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativeSig>,
}
/// Check Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeSigResult {
    /// Is Sig valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeHashRequest {
    /// Hash to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativeHash>,
}
/// Check Hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeHashResult {
    /// Is Hash valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Amount request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAmountRequest {
    /// Amount to check
    #[prost(message, optional, tag = "1")]
    pub to_check: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Check Amount result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNativeAmountResult {
    /// Is Amount valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Amount addition request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNativeAmountsRequest {
    /// First amount to add
    #[prost(message, optional, tag = "1")]
    pub amount1: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to add
    #[prost(message, optional, tag = "2")]
    pub amount2: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount addition result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNativeAmountsResult {
    /// Sum of amounts
    #[prost(message, optional, tag = "1")]
    pub sum: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount subtraction request
/// try to compute difference = left - right
/// fails if right > left
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubNativeAmountsRequest {
    /// First amount to subtract from
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to subtract
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount subtraction result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubNativeAmountsResult {
    /// Difference of amounts (left - right)
    #[prost(message, optional, tag = "1")]
    pub difference: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount multiplication request
/// Try to compute product = amount * coefficient (fail if overflow)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MulNativeAmountRequest {
    /// Amount to multiply
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Coefficient to multiply by
    #[prost(int64, tag = "2")]
    pub coefficient: i64,
}
/// Amount multiplication result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MulNativeAmountResult {
    /// Product of amount and coefficient
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division by scalar request
/// Try to compute quotient = dividend / divisor
/// Fails if divisor == 0
/// Fails if underflow
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDivRemNativeAmountRequest {
    /// Amount to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Divisor to divide by
    #[prost(int64, tag = "2")]
    pub divisor: i64,
}
/// Amount division by scalar result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDivRemNativeAmountResult {
    /// Quotient of amount and divisor
    #[prost(message, optional, tag = "1")]
    pub quotient: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Remainder of amount and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DivRemNativeAmountRequest {
    /// Amount to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Divisor to divide by
    #[prost(message, optional, tag = "2")]
    pub divisor: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Amount division result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DivRemNativeAmountResult {
    /// Quotient of amount and divisor
    #[prost(int64, tag = "1")]
    pub quotient: i64,
    /// Remainder of amount and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Comparison result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparisonResult {
    /// Status
    #[prost(oneof = "comparison_result::Restult", tags = "1, 2, 3")]
    pub restult: ::core::option::Option<comparison_result::Restult>,
}
/// Nested message and enum types in `ComparisonResult`.
pub mod comparison_result {
    /// Left is lower
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeftLower {}
    /// Left is equal to right
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Equal {}
    /// Left is greater
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeftGreater {}
    /// Status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Restult {
        #[prost(message, tag = "1")]
        LeftLower(LeftLower),
        #[prost(message, tag = "2")]
        Equal(Equal),
        #[prost(message, tag = "3")]
        LeftGreater(LeftGreater),
    }
}
/// Time addition checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedAddNativeTimeRequest {
    /// First time to add
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to add
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time addition checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedAddNativeTimeResult {
    /// Sum of times
    #[prost(message, optional, tag = "1")]
    pub sum: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time subtraction checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedSubNativeTimeRequest {
    /// First time to subtract from
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to subtract
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time subtraction checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedSubNativeTimeResult {
    /// Difference of times (left - right)
    #[prost(message, optional, tag = "1")]
    pub difference: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time scalar mult checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedMulNativeTimeRequest {
    /// Time to multiply
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Coefficient to multiply by
    #[prost(int64, tag = "2")]
    pub coefficient: i64,
}
/// Time scalar mult checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedMulNativeTimeResult {
    /// Product of time and coefficient
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time scalar divrem checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarDivRemNativeTimeRequest {
    /// Time to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Divisor to divide by
    #[prost(int64, tag = "2")]
    pub divisor: i64,
}
/// Time scalar divrem checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedScalarDivRemNativeTimeResult {
    /// Quotient of time and divisor
    #[prost(message, optional, tag = "1")]
    pub quotient: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Remainder of time and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time division checked request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedDivRemNativeTimeRequest {
    /// Time to divide
    #[prost(message, optional, tag = "1")]
    pub dividend: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Divisor to divide by
    #[prost(message, optional, tag = "2")]
    pub divisor: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time division checked result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedDivRemNativeTimeResult {
    /// Quotient of time and divisor
    #[prost(int64, tag = "1")]
    pub quotient: i64,
    /// Remainder of time and divisor
    #[prost(message, optional, tag = "2")]
    pub remainder: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time comparison request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeTimeRequest {
    /// First time to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeTime>,
    /// Second time to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Time comparison result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeTimeResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Compare NativeAddress request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAddressRequest {
    /// First address to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeAddress>,
    /// Second address to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeAddress>,
}
/// Compare NativeAddress result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAddressResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Compare NativePubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativePubKeyRequest {
    /// First public key to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativePubKey>,
    /// Second public key to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativePubKey>,
}
/// Compare NativePubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativePubKeyResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Compare NativeSig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeSigRequest {
    /// First signature to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeSig>,
    /// Second signature to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeSig>,
}
/// Compare NativeSig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeSigResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Verify NativeSig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNativeSigRequest {
    /// Signature to verify
    #[prost(message, optional, tag = "1")]
    pub sig: ::core::option::Option<super::super::model::v1::NativeSig>,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(message, optional, tag = "3")]
    pub pub_key: ::core::option::Option<super::super::model::v1::NativePubKey>,
}
/// Verify NativeSig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNativeSigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
/// Compare NativeAmount request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAmountRequest {
    /// First amount to compare
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<super::super::model::v1::NativeAmount>,
    /// Second amount to compare
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Compare NativeAmount result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareNativeAmountResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Keccak256 hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keccak256Request {
    /// Data to hash
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Keccak256 hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keccak256Result {
    /// Hash of data
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// EVM signature verification request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyEvmSigRequest {
    /// Signature to verify
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(bytes = "vec", tag = "3")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
/// EVM signature verification result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyEvmSigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
/// BLS signature single verification request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyBlsSingleSigRequest {
    /// Signature to verify
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(bytes = "vec", tag = "3")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
}
/// BLS signature single verification result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyBlsSingleSigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
/// BLS signature multiple verification request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyBlsMultiSigRequest {
    /// Signature to verify
    #[prost(bytes = "vec", tag = "1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public keys to verify with
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub pub_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// BLS signature multiple verification result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyBlsMultiSigResult {
    /// Verification result
    #[prost(bool, tag = "1")]
    pub is_verified: bool,
}
