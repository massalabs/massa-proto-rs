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
    #[prost(string, tag = "1")]
    pub sc_address: ::prost::alloc::string::String,
}
/// Call SC request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallRequest {
    /// Address of the smart contract to call
    #[prost(string, tag = "1")]
    pub target_sc_address: ::prost::alloc::string::String,
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
/// Local execution request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalExecutionRequest {
    /// Bytecode is the compiled code of the smart contract
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// Function to call in the targeted smart contract
    #[prost(string, tag = "2")]
    pub target_function_name: ::prost::alloc::string::String,
    /// Argument to the function serialized in a byte array.
    #[prost(bytes = "vec", tag = "3")]
    pub function_arg: ::prost::alloc::vec::Vec<u8>,
}
/// Local call response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalExecutionResponse {
    /// Return_data is the return value of the function
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Generate event request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateEventRequest {
    /// Event
    #[prost(bytes = "vec", tag = "1")]
    pub event: ::prost::alloc::vec::Vec<u8>,
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
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// The amount of coins to transfer
    #[prost(message, optional, tag = "2")]
    pub amount_to_transfer: ::core::option::Option<
        super::super::model::v1::NativeAmount,
    >,
    /// The address of the sender, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub optional_sender_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Transfer coin result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCoinsResult {}
/// Function exists request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionExistsRequest {
    /// Address of the smart contract to call
    #[prost(string, tag = "1")]
    pub target_sc_address: ::prost::alloc::string::String,
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
/// Tips to check for completeness exec:
/// `rg message | rg "\{" | rg "Result" | wc -l`
/// the given count should be equal to the number of messages in RespResult + 2
/// the +2 comes from ResResult itself which is counted above and from ComparisonResult which is not in the same category
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespResult {
    #[prost(
        oneof = "resp_result::Res",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65"
    )]
    pub res: ::core::option::Option<resp_result::Res>,
}
/// Nested message and enum types in `RespResult`.
pub mod resp_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Res {
        #[prost(message, tag = "1")]
        AddNativeAmountsResult(super::AddNativeAmountsResult),
        #[prost(message, tag = "2")]
        AddressFromPubKeyResult(super::AddressFromPubKeyResult),
        #[prost(message, tag = "3")]
        AppendDataResult(super::AppendDataResult),
        #[prost(message, tag = "4")]
        Base58CheckToBytesResult(super::Base58CheckToBytesResult),
        #[prost(message, tag = "5")]
        Blake3HashResult(super::Blake3HashResult),
        #[prost(message, tag = "6")]
        BytesToBase58CheckResult(super::BytesToBase58CheckResult),
        #[prost(message, tag = "7")]
        CallerHasWriteAccessResult(super::CallerHasWriteAccessResult),
        #[prost(message, tag = "8")]
        CheckAddressResult(super::CheckAddressResult),
        #[prost(message, tag = "9")]
        CheckBlake3HashResult(super::CheckBlake3HashResult),
        #[prost(message, tag = "10")]
        CheckNativeAmountResult(super::CheckNativeAmountResult),
        #[prost(message, tag = "11")]
        CheckPubKeyResult(super::CheckPubKeyResult),
        #[prost(message, tag = "12")]
        CheckSigResult(super::CheckSigResult),
        #[prost(message, tag = "13")]
        CheckedAddNativeTimeResult(super::CheckedAddNativeTimeResult),
        #[prost(message, tag = "14")]
        CheckedDivRemNativeTimeResult(super::CheckedDivRemNativeTimeResult),
        #[prost(message, tag = "15")]
        CheckedMulNativeTimeResult(super::CheckedMulNativeTimeResult),
        #[prost(message, tag = "16")]
        CheckedScalarDivRemNativeTimeResult(super::CheckedScalarDivRemNativeTimeResult),
        #[prost(message, tag = "17")]
        CheckedSubNativeTimeResult(super::CheckedSubNativeTimeResult),
        #[prost(message, tag = "18")]
        CompareAddressResult(super::CompareAddressResult),
        #[prost(message, tag = "19")]
        CompareNativeAmountResult(super::CompareNativeAmountResult),
        #[prost(message, tag = "20")]
        CompareNativeTimeResult(super::CompareNativeTimeResult),
        #[prost(message, tag = "21")]
        ComparePubKeyResult(super::ComparePubKeyResult),
        #[prost(message, tag = "22")]
        CompareSigResult(super::CompareSigResult),
        #[prost(message, tag = "23")]
        CreateScResult(super::CreateScResult),
        #[prost(message, tag = "24")]
        DateNowResult(super::DateNowResult),
        #[prost(message, tag = "25")]
        DeleteDataResult(super::DeleteDataResult),
        #[prost(message, tag = "26")]
        DivRemNativeAmountsResult(super::DivRemNativeAmountsResult),
        #[prost(message, tag = "27")]
        FunctionExistsResult(super::FunctionExistsResult),
        #[prost(message, tag = "28")]
        GenerateEventResult(super::GenerateEventResult),
        #[prost(message, tag = "29")]
        GetAddressCategoryResult(super::GetAddressCategoryResult),
        #[prost(message, tag = "30")]
        GetAddressVersionResult(super::GetAddressVersionResult),
        #[prost(message, tag = "31")]
        GetBalanceResult(super::GetBalanceResult),
        #[prost(message, tag = "32")]
        GetBytecodeResult(super::GetBytecodeResult),
        #[prost(message, tag = "33")]
        GetCallCoinsResult(super::GetCallCoinsResult),
        #[prost(message, tag = "34")]
        GetCallStackResult(super::GetCallStackResult),
        #[prost(message, tag = "35")]
        GetCurrentSlotResult(super::GetCurrentSlotResult),
        #[prost(message, tag = "36")]
        GetDataResult(super::GetDataResult),
        #[prost(message, tag = "37")]
        GetKeysResult(super::GetKeysResult),
        #[prost(message, tag = "38")]
        GetNativeTimeResult(super::GetNativeTimeResult),
        #[prost(message, tag = "39")]
        GetOpDataResult(super::GetOpDataResult),
        #[prost(message, tag = "40")]
        GetOpKeysResult(super::GetOpKeysResult),
        #[prost(message, tag = "41")]
        GetOwnedAddressesResult(super::GetOwnedAddressesResult),
        #[prost(message, tag = "42")]
        GetPubKeyVersionResult(super::GetPubKeyVersionResult),
        #[prost(message, tag = "43")]
        GetRemainingGasResult(super::GetRemainingGasResult),
        #[prost(message, tag = "44")]
        GetSignatureVersionResult(super::GetSignatureVersionResult),
        #[prost(message, tag = "45")]
        HashSha256Result(super::HashSha256Result),
        #[prost(message, tag = "46")]
        HasDataResult(super::HasDataResult),
        #[prost(message, tag = "47")]
        HasOpKeyResult(super::HasOpKeyResult),
        #[prost(message, tag = "48")]
        Keccak256Result(super::Keccak256Result),
        #[prost(message, tag = "49")]
        MulNativeAmountResult(super::MulNativeAmountResult),
        #[prost(message, tag = "50")]
        NativeAmountFromStringResult(super::NativeAmountFromStringResult),
        #[prost(message, tag = "51")]
        NativeAmountToStringResult(super::NativeAmountToStringResult),
        #[prost(message, tag = "52")]
        ProcessExitResult(super::ProcessExitResult),
        #[prost(message, tag = "53")]
        ScalarDivRemNativeAmountResult(super::ScalarDivRemNativeAmountResult),
        #[prost(message, tag = "54")]
        SeedResult(super::SeedResult),
        #[prost(message, tag = "55")]
        SetBytecodeResult(super::SetBytecodeResult),
        #[prost(message, tag = "56")]
        SetDataResult(super::SetDataResult),
        #[prost(message, tag = "57")]
        SubNativeAmountsResult(super::SubNativeAmountsResult),
        #[prost(message, tag = "58")]
        TransferCoinsResult(super::TransferCoinsResult),
        #[prost(message, tag = "59")]
        UnsafeRandomResult(super::UnsafeRandomResult),
        #[prost(message, tag = "60")]
        VerifyBlsMultiSigResult(super::VerifyBlsMultiSigResult),
        #[prost(message, tag = "61")]
        VerifyBlsSingleSigResult(super::VerifyBlsSingleSigResult),
        #[prost(message, tag = "62")]
        VerifyEvmSigResult(super::VerifyEvmSigResult),
        #[prost(message, tag = "63")]
        VerifySigResult(super::VerifySigResult),
        #[prost(message, tag = "64")]
        SendAsyncMessageResult(super::SendAsyncMessageResult),
        #[prost(message, tag = "65")]
        LocalExecutionResponse(super::LocalExecutionResponse),
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
pub struct CheckAddressRequest {
    /// Address to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAddressResult {
    /// Is address valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check PubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPubKeyRequest {
    /// PubKey to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check PubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPubKeyResult {
    /// Is PubKey valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSigRequest {
    /// Sig to check
    #[prost(string, tag = "1")]
    pub to_check: ::prost::alloc::string::String,
}
/// Check Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSigResult {
    /// Is Sig valid
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
}
/// Check Hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBlake3HashRequest {
    /// Hash to check
    #[prost(bytes = "vec", tag = "1")]
    pub to_check: ::prost::alloc::vec::Vec<u8>,
}
/// Check Hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBlake3HashResult {
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
    #[prost(message, optional, tag = "2")]
    pub mandatory_coefficient: ::core::option::Option<u64>,
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
    #[prost(message, optional, tag = "2")]
    pub mandatory_divisor: ::core::option::Option<u64>,
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
pub struct DivRemNativeAmountsRequest {
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
pub struct DivRemNativeAmountsResult {
    /// Quotient of amount and divisor
    #[prost(message, optional, tag = "1")]
    pub mandatory_quotient: ::core::option::Option<u64>,
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
    #[prost(message, optional, tag = "2")]
    pub mandatory_coefficient: ::core::option::Option<u64>,
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
    #[prost(message, optional, tag = "2")]
    pub mandatory_divisor: ::core::option::Option<u64>,
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
    #[prost(message, optional, tag = "1")]
    pub mandatory_quotient: ::core::option::Option<u64>,
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
/// Compare Address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareAddressRequest {
    /// First address to compare
    #[prost(string, tag = "1")]
    pub left: ::prost::alloc::string::String,
    /// Second address to compare
    #[prost(string, tag = "2")]
    pub right: ::prost::alloc::string::String,
}
/// Compare Address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareAddressResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Compare PubKey request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparePubKeyRequest {
    /// First public key to compare
    #[prost(string, tag = "1")]
    pub left: ::prost::alloc::string::String,
    /// Second public key to compare
    #[prost(string, tag = "2")]
    pub right: ::prost::alloc::string::String,
}
/// Compare PubKey result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparePubKeyResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Compare Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareSigRequest {
    /// First signature to compare
    #[prost(string, tag = "1")]
    pub left: ::prost::alloc::string::String,
    /// Second signature to compare
    #[prost(string, tag = "2")]
    pub right: ::prost::alloc::string::String,
}
/// Compare Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompareSigResult {
    /// Comparison result
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ComparisonResult>,
}
/// Verify Sig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySigRequest {
    /// Signature to verify
    #[prost(string, tag = "1")]
    pub sig: ::prost::alloc::string::String,
    /// Message to verify
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    /// Public key to verify with
    #[prost(string, tag = "3")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Verify Sig result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySigResult {
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
/// Blake3 hash request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blake3HashRequest {
    /// Data to hash
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Blake3 hash result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blake3HashResult {
    /// Hash of data
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Get keys request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeysRequest {
    /// keys prefix
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
    /// Address to get keys for
    #[prost(message, optional, tag = "2")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get keys result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeysResult {
    /// keys
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Set data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// Address to set data for, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataResult {}
/// Append data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// Address to append data for, if none, use current address
    #[prost(message, optional, tag = "3")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Append data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendDataResult {}
/// Get data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to get data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataResult {
    /// value
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Delete data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to delete data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Delete data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataResult {}
/// Has data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// Address to check data for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Has data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasDataResult {
    /// has data
    #[prost(bool, tag = "1")]
    pub has_data: bool,
}
/// Get owned addresses request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOwnedAddressesRequest {}
/// Get owned addresses result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOwnedAddressesResult {
    /// owned addresses
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Get call stack request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallStackRequest {}
/// Get call stack result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallStackResult {
    /// call stack
    #[prost(string, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Address from public key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressFromPubKeyRequest {
    /// Native public key
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Address from public key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressFromPubKeyResult {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Unsafe random request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeRandomRequest {
    #[prost(message, optional, tag = "2")]
    pub mandatory_num_bytes: ::core::option::Option<u32>,
}
/// Unsafe random result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeRandomResult {
    /// Random bytes generated
    #[prost(bytes = "vec", tag = "1")]
    pub random_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Send async message request filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageFilter {
    /// Target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target key
    #[prost(message, optional, tag = "2")]
    pub target_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Send async message request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageRequest {
    /// Target address
    #[prost(string, tag = "1")]
    pub target_address: ::prost::alloc::string::String,
    /// Target handler (function name)
    #[prost(string, tag = "2")]
    pub target_handler: ::prost::alloc::string::String,
    /// Start slot for the message execution
    #[prost(message, optional, tag = "3")]
    pub validity_start: ::core::option::Option<super::super::model::v1::Slot>,
    /// End slot for the message execution
    #[prost(message, optional, tag = "4")]
    pub validity_end: ::core::option::Option<super::super::model::v1::Slot>,
    /// Gas given for the execution
    #[prost(uint64, tag = "5")]
    pub execution_gas: u64,
    /// Message fee
    #[prost(uint64, tag = "6")]
    pub raw_fee: u64,
    /// Coins sent to the execution context
    #[prost(uint64, tag = "7")]
    pub raw_coins: u64,
    /// Message data
    #[prost(bytes = "vec", tag = "8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Filter for the message
    #[prost(message, optional, tag = "9")]
    pub filter: ::core::option::Option<SendAsyncMessageFilter>,
}
/// Send async message result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendAsyncMessageResult {}
/// Get native time request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNativeTimeRequest {}
/// Get native time result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNativeTimeResult {
    /// Native time
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::super::model::v1::NativeTime>,
}
/// Get current slot request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentSlotRequest {}
/// Get current slot result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentSlotResult {
    /// Current slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<super::super::model::v1::Slot>,
}
/// Set bytecode request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBytecodeRequest {
    /// Bytecode
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// Address to set bytecode for, if none, use current address
    #[prost(message, optional, tag = "2")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set bytecode result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBytecodeResult {}
/// Get bytecode request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBytecodeRequest {
    /// Address to get bytecode for, if none, use current address
    #[prost(message, optional, tag = "1")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get bytecode result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBytecodeResult {
    /// Bytecode
    #[prost(bytes = "vec", tag = "1")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
}
/// Caller has write access request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallerHasWriteAccessRequest {}
/// Caller has write access result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallerHasWriteAccessResult {
    /// Caller has write access
    #[prost(bool, tag = "1")]
    pub has_write_access: bool,
}
/// AS builtin `seed` function
/// Seed request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedRequest {}
/// Seed result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedResult {
    /// Seed
    #[prost(double, tag = "1")]
    pub seed: f64,
}
/// AS builtin `Date.now` function
/// date now request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateNowRequest {}
/// date now result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateNowResult {
    /// date now
    #[prost(double, tag = "1")]
    pub date_now: f64,
}
/// AS builtin `process.exit()` function
/// Process exit request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessExitRequest {
    /// exit code
    #[prost(message, optional, tag = "1")]
    pub mandatory_code: ::core::option::Option<u32>,
}
/// Process exit result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessExitResult {}
/// Hash sha256 request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashSha256Request {
    /// data
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Hash sha256 result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashSha256Result {
    /// hash
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Get op data request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpDataRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Get op data result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpDataResult {
    /// value
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Has op key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasOpKeyRequest {
    /// key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// Has op key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasOpKeyResult {
    /// has key
    #[prost(bool, tag = "1")]
    pub has_key: bool,
}
/// Get op keys request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpKeysRequest {
    /// keys prefix
    #[prost(bytes = "vec", tag = "1")]
    pub prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Get op keys result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpKeysResult {
    /// keys
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Get remaining gas request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemainingGasRequest {}
/// Get remaining gas result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemainingGasResult {
    /// remaining gas
    #[prost(message, optional, tag = "1")]
    pub mandatory_remaining_gas: ::core::option::Option<u64>,
}
/// Get balance request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceRequest {
    /// Address to get balance for, if none, use current address
    #[prost(message, optional, tag = "1")]
    pub optional_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Get balance result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceResult {
    /// balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Get call coins request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallCoinsRequest {}
/// Get call coins result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCallCoinsResult {
    /// coins
    #[prost(message, optional, tag = "1")]
    pub coins: ::core::option::Option<super::super::model::v1::NativeAmount>,
}
/// Get the version of the address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressVersionRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Get the version of the address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressVersionResult {
    #[prost(message, optional, tag = "1")]
    pub mandatory_version: ::core::option::Option<u64>,
}
/// Get the category of the address request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressCategoryRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Get the category of the address result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressCategoryResult {
    #[prost(enumeration = "super::super::model::v1::AddressCategory", tag = "1")]
    pub category: i32,
}
/// Get the version of the public key request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubKeyVersionRequest {
    #[prost(string, tag = "1")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Get the version of the public key result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubKeyVersionResult {
    #[prost(message, optional, tag = "1")]
    pub mandatory_version: ::core::option::Option<u64>,
}
/// Get the version of the signature request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureVersionRequest {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
}
/// Get the version of the signature result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignatureVersionResult {
    #[prost(message, optional, tag = "1")]
    pub mandatory_version: ::core::option::Option<u64>,
}
/// bytes to base58 request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesToBase58CheckRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// bytes to base58 result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesToBase58CheckResult {
    #[prost(string, tag = "1")]
    pub base58_check: ::prost::alloc::string::String,
}
/// base58 to bytes request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base58CheckToBytesRequest {
    #[prost(string, tag = "1")]
    pub base58_check: ::prost::alloc::string::String,
}
/// base58 to bytes result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base58CheckToBytesResult {
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
