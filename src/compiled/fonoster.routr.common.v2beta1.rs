#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Transport {
    Udp = 0,
    Tcp = 1,
    Tls = 2,
    Sctp = 3,
    Ws = 4,
    Wss = 5,
}
impl Transport {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Transport::Udp => "UDP",
            Transport::Tcp => "TCP",
            Transport::Tls => "TLS",
            Transport::Sctp => "SCTP",
            Transport::Ws => "WS",
            Transport::Wss => "WSS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UDP" => Some(Self::Udp),
            "TCP" => Some(Self::Tcp),
            "TLS" => Some(Self::Tls),
            "SCTP" => Some(Self::Sctp),
            "WS" => Some(Self::Ws),
            "WSS" => Some(Self::Wss),
            _ => None,
        }
    }
}
