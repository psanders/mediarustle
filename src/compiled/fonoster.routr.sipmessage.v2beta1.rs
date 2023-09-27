#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WwwAuthenticate {
    #[prost(string, tag = "1")]
    pub realm: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scheme: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub algorithm: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub qop: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub opaque: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub stale: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    #[prost(string, tag = "1")]
    pub realm: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub scheme: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub c_nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub algorithm: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub qop: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub opaque: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub response: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub method: ::prost::alloc::string::String,
    #[prost(int32, tag = "12")]
    pub nonce_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallId {
    #[prost(string, tag = "1")]
    pub call_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLength {
    #[prost(int32, tag = "1")]
    pub content_length: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Via {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub port: i32,
    #[prost(string, tag = "3")]
    pub branch: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transport: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub r_port_flag: bool,
    #[prost(int32, tag = "6")]
    pub r_port: i32,
    #[prost(string, tag = "7")]
    pub received: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub m_addr: ::prost::alloc::string::String,
    #[prost(int32, tag = "9")]
    pub ttl: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SipUri {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_password: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transport_param: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub m_addr_param: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub method_param: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub user_param: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub ttl_param: i32,
    #[prost(int32, tag = "9")]
    pub port: i32,
    #[prost(bool, tag = "10")]
    pub lr_param: bool,
    #[prost(bool, tag = "11")]
    pub bnc_param: bool,
    #[prost(bool, tag = "12")]
    pub secure: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(message, optional, tag = "1")]
    pub uri: ::core::option::Option<SipUri>,
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub wildcard: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxForwards {
    #[prost(int32, tag = "1")]
    pub max_forwards: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(map = "string, string", tag = "2")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordRoute {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(map = "string, string", tag = "2")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct From {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct To {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<Address>,
    #[prost(int32, tag = "2")]
    pub expires: i32,
    #[prost(float, tag = "3")]
    pub q_value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expires {
    #[prost(int32, tag = "1")]
    pub expires: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SipMessage {
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<From>,
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<To>,
    #[prost(message, optional, tag = "5")]
    pub contact: ::core::option::Option<Contact>,
    #[prost(message, optional, tag = "6")]
    pub call_id: ::core::option::Option<CallId>,
    #[prost(message, optional, tag = "7")]
    pub content_length: ::core::option::Option<ContentLength>,
    #[prost(message, optional, tag = "8")]
    pub expires: ::core::option::Option<Expires>,
    #[prost(message, optional, tag = "9")]
    pub www_authenticate: ::core::option::Option<WwwAuthenticate>,
    #[prost(message, optional, tag = "10")]
    pub max_forwards: ::core::option::Option<MaxForwards>,
    #[prost(message, optional, tag = "11")]
    pub authorization: ::core::option::Option<Authorization>,
    #[prost(message, repeated, tag = "12")]
    pub extensions: ::prost::alloc::vec::Vec<Extension>,
    #[prost(message, repeated, tag = "13")]
    pub via: ::prost::alloc::vec::Vec<Via>,
    #[prost(message, repeated, tag = "14")]
    pub route: ::prost::alloc::vec::Vec<Route>,
    #[prost(message, repeated, tag = "15")]
    pub record_route: ::prost::alloc::vec::Vec<RecordRoute>,
    #[prost(string, tag = "16")]
    pub reason_phrase: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub body: ::prost::alloc::string::String,
    #[prost(oneof = "sip_message::MessageType", tags = "1, 2")]
    pub message_type: ::core::option::Option<sip_message::MessageType>,
}
/// Nested message and enum types in `SIPMessage`.
pub mod sip_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessageType {
        #[prost(enumeration = "super::ResponseType", tag = "1")]
        ResponseType(i32),
        #[prost(message, tag = "2")]
        RequestUri(super::SipUri),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseType {
    Unknown = 0,
    Trying = 1,
    Ringing = 2,
    CallIsBeingForwarded = 3,
    Queued = 4,
    SessionProgress = 5,
    Success = 6,
    Ok = 7,
    Accepted = 8,
    Redirection = 9,
    MultipleChoices = 10,
    MovedPermanently = 11,
    MovedTemporarily = 12,
    UseProxy = 13,
    AlternativeService = 14,
    ClientError = 15,
    BadRequest = 16,
    Unauthorized = 17,
    PaymentRequired = 18,
    Forbidden = 19,
    NotFound = 20,
    MethodNotAllowed = 21,
    NotAcceptable = 22,
    ProxyAuthenticationRequired = 23,
    RequestTimeout = 24,
    Gone = 25,
    RequestEntityTooLarge = 26,
    RequestUriTooLong = 27,
    UnsupportedMediaType = 28,
    UnsupportedUriScheme = 29,
    BadExtension = 30,
    ExtensionRequired = 31,
    IntervalTooBrief = 32,
    TemporarilyUnavailable = 33,
    CallOrTransactionDoesNotExist = 34,
    LoopDetected = 35,
    TooManyHops = 36,
    AddressIncomplete = 37,
    Ambiguous = 38,
    BusyHere = 39,
    RequestTerminated = 40,
    NotAcceptableHere = 41,
    BadEvent = 42,
    RequestPending = 43,
    Undecipherable = 44,
    ServerError = 45,
    ServerInternalError = 46,
    NotImplemented = 47,
    BadGateway = 48,
    ServiceUnavailable = 49,
    ServerTimeout = 50,
    VersionNotSupported = 51,
    MessageTooLarge = 52,
    GlobalError = 53,
    BusyEverywhere = 54,
    Decline = 55,
    DoesNotExistAnywhere = 56,
    SessionNotAcceptable = 57,
}
impl ResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseType::Unknown => "UNKNOWN",
            ResponseType::Trying => "TRYING",
            ResponseType::Ringing => "RINGING",
            ResponseType::CallIsBeingForwarded => "CALL_IS_BEING_FORWARDED",
            ResponseType::Queued => "QUEUED",
            ResponseType::SessionProgress => "SESSION_PROGRESS",
            ResponseType::Success => "SUCCESS",
            ResponseType::Ok => "OK",
            ResponseType::Accepted => "ACCEPTED",
            ResponseType::Redirection => "REDIRECTION",
            ResponseType::MultipleChoices => "MULTIPLE_CHOICES",
            ResponseType::MovedPermanently => "MOVED_PERMANENTLY",
            ResponseType::MovedTemporarily => "MOVED_TEMPORARILY",
            ResponseType::UseProxy => "USE_PROXY",
            ResponseType::AlternativeService => "ALTERNATIVE_SERVICE",
            ResponseType::ClientError => "CLIENT_ERROR",
            ResponseType::BadRequest => "BAD_REQUEST",
            ResponseType::Unauthorized => "UNAUTHORIZED",
            ResponseType::PaymentRequired => "PAYMENT_REQUIRED",
            ResponseType::Forbidden => "FORBIDDEN",
            ResponseType::NotFound => "NOT_FOUND",
            ResponseType::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            ResponseType::NotAcceptable => "NOT_ACCEPTABLE",
            ResponseType::ProxyAuthenticationRequired => "PROXY_AUTHENTICATION_REQUIRED",
            ResponseType::RequestTimeout => "REQUEST_TIMEOUT",
            ResponseType::Gone => "GONE",
            ResponseType::RequestEntityTooLarge => "REQUEST_ENTITY_TOO_LARGE",
            ResponseType::RequestUriTooLong => "REQUEST_URI_TOO_LONG",
            ResponseType::UnsupportedMediaType => "UNSUPPORTED_MEDIA_TYPE",
            ResponseType::UnsupportedUriScheme => "UNSUPPORTED_URI_SCHEME",
            ResponseType::BadExtension => "BAD_EXTENSION",
            ResponseType::ExtensionRequired => "EXTENSION_REQUIRED",
            ResponseType::IntervalTooBrief => "INTERVAL_TOO_BRIEF",
            ResponseType::TemporarilyUnavailable => "TEMPORARILY_UNAVAILABLE",
            ResponseType::CallOrTransactionDoesNotExist => {
                "CALL_OR_TRANSACTION_DOES_NOT_EXIST"
            }
            ResponseType::LoopDetected => "LOOP_DETECTED",
            ResponseType::TooManyHops => "TOO_MANY_HOPS",
            ResponseType::AddressIncomplete => "ADDRESS_INCOMPLETE",
            ResponseType::Ambiguous => "AMBIGUOUS",
            ResponseType::BusyHere => "BUSY_HERE",
            ResponseType::RequestTerminated => "REQUEST_TERMINATED",
            ResponseType::NotAcceptableHere => "NOT_ACCEPTABLE_HERE",
            ResponseType::BadEvent => "BAD_EVENT",
            ResponseType::RequestPending => "REQUEST_PENDING",
            ResponseType::Undecipherable => "UNDECIPHERABLE",
            ResponseType::ServerError => "SERVER_ERROR",
            ResponseType::ServerInternalError => "SERVER_INTERNAL_ERROR",
            ResponseType::NotImplemented => "NOT_IMPLEMENTED",
            ResponseType::BadGateway => "BAD_GATEWAY",
            ResponseType::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            ResponseType::ServerTimeout => "SERVER_TIMEOUT",
            ResponseType::VersionNotSupported => "VERSION_NOT_SUPPORTED",
            ResponseType::MessageTooLarge => "MESSAGE_TOO_LARGE",
            ResponseType::GlobalError => "GLOBAL_ERROR",
            ResponseType::BusyEverywhere => "BUSY_EVERYWHERE",
            ResponseType::Decline => "DECLINE",
            ResponseType::DoesNotExistAnywhere => "DOES_NOT_EXIST_ANYWHERE",
            ResponseType::SessionNotAcceptable => "SESSION_NOT_ACCEPTABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "TRYING" => Some(Self::Trying),
            "RINGING" => Some(Self::Ringing),
            "CALL_IS_BEING_FORWARDED" => Some(Self::CallIsBeingForwarded),
            "QUEUED" => Some(Self::Queued),
            "SESSION_PROGRESS" => Some(Self::SessionProgress),
            "SUCCESS" => Some(Self::Success),
            "OK" => Some(Self::Ok),
            "ACCEPTED" => Some(Self::Accepted),
            "REDIRECTION" => Some(Self::Redirection),
            "MULTIPLE_CHOICES" => Some(Self::MultipleChoices),
            "MOVED_PERMANENTLY" => Some(Self::MovedPermanently),
            "MOVED_TEMPORARILY" => Some(Self::MovedTemporarily),
            "USE_PROXY" => Some(Self::UseProxy),
            "ALTERNATIVE_SERVICE" => Some(Self::AlternativeService),
            "CLIENT_ERROR" => Some(Self::ClientError),
            "BAD_REQUEST" => Some(Self::BadRequest),
            "UNAUTHORIZED" => Some(Self::Unauthorized),
            "PAYMENT_REQUIRED" => Some(Self::PaymentRequired),
            "FORBIDDEN" => Some(Self::Forbidden),
            "NOT_FOUND" => Some(Self::NotFound),
            "METHOD_NOT_ALLOWED" => Some(Self::MethodNotAllowed),
            "NOT_ACCEPTABLE" => Some(Self::NotAcceptable),
            "PROXY_AUTHENTICATION_REQUIRED" => Some(Self::ProxyAuthenticationRequired),
            "REQUEST_TIMEOUT" => Some(Self::RequestTimeout),
            "GONE" => Some(Self::Gone),
            "REQUEST_ENTITY_TOO_LARGE" => Some(Self::RequestEntityTooLarge),
            "REQUEST_URI_TOO_LONG" => Some(Self::RequestUriTooLong),
            "UNSUPPORTED_MEDIA_TYPE" => Some(Self::UnsupportedMediaType),
            "UNSUPPORTED_URI_SCHEME" => Some(Self::UnsupportedUriScheme),
            "BAD_EXTENSION" => Some(Self::BadExtension),
            "EXTENSION_REQUIRED" => Some(Self::ExtensionRequired),
            "INTERVAL_TOO_BRIEF" => Some(Self::IntervalTooBrief),
            "TEMPORARILY_UNAVAILABLE" => Some(Self::TemporarilyUnavailable),
            "CALL_OR_TRANSACTION_DOES_NOT_EXIST" => {
                Some(Self::CallOrTransactionDoesNotExist)
            }
            "LOOP_DETECTED" => Some(Self::LoopDetected),
            "TOO_MANY_HOPS" => Some(Self::TooManyHops),
            "ADDRESS_INCOMPLETE" => Some(Self::AddressIncomplete),
            "AMBIGUOUS" => Some(Self::Ambiguous),
            "BUSY_HERE" => Some(Self::BusyHere),
            "REQUEST_TERMINATED" => Some(Self::RequestTerminated),
            "NOT_ACCEPTABLE_HERE" => Some(Self::NotAcceptableHere),
            "BAD_EVENT" => Some(Self::BadEvent),
            "REQUEST_PENDING" => Some(Self::RequestPending),
            "UNDECIPHERABLE" => Some(Self::Undecipherable),
            "SERVER_ERROR" => Some(Self::ServerError),
            "SERVER_INTERNAL_ERROR" => Some(Self::ServerInternalError),
            "NOT_IMPLEMENTED" => Some(Self::NotImplemented),
            "BAD_GATEWAY" => Some(Self::BadGateway),
            "SERVICE_UNAVAILABLE" => Some(Self::ServiceUnavailable),
            "SERVER_TIMEOUT" => Some(Self::ServerTimeout),
            "VERSION_NOT_SUPPORTED" => Some(Self::VersionNotSupported),
            "MESSAGE_TOO_LARGE" => Some(Self::MessageTooLarge),
            "GLOBAL_ERROR" => Some(Self::GlobalError),
            "BUSY_EVERYWHERE" => Some(Self::BusyEverywhere),
            "DECLINE" => Some(Self::Decline),
            "DOES_NOT_EXIST_ANYWHERE" => Some(Self::DoesNotExistAnywhere),
            "SESSION_NOT_ACCEPTABLE" => Some(Self::SessionNotAcceptable),
            _ => None,
        }
    }
}
