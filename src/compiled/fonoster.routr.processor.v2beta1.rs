#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetInterface {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub port: i32,
    #[prost(enumeration = "super::super::common::v2beta1::Transport", tag = "3")]
    pub transport: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRequest {
    /// Same as the Call-Id header
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub edge_port_ref: ::prost::alloc::string::String,
    #[prost(enumeration = "Method", tag = "3")]
    pub method: i32,
    #[prost(message, optional, tag = "4")]
    pub sender: ::core::option::Option<NetInterface>,
    #[prost(message, repeated, tag = "5")]
    pub listening_points: ::prost::alloc::vec::Vec<NetInterface>,
    #[prost(string, repeated, tag = "6")]
    pub external_addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub localnets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub message: ::core::option::Option<super::super::sipmessage::v2beta1::SipMessage>,
    #[prost(map = "string, string", tag = "9")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageResponse {
    #[prost(message, optional, tag = "1")]
    pub sender: ::core::option::Option<NetInterface>,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<super::super::sipmessage::v2beta1::SipMessage>,
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Method {
    Unknown = 0,
    /// Communicates user location (host name, IP)
    Register = 1,
    /// Establishes a session
    Invite = 2,
    /// Transports Instant Messages
    Message = 3,
    /// Publishes an event to the Server
    Publish = 4,
    /// Notifies the subscriber of a new event
    Notify = 5,
    /// Subscribes for Notification from the notifier
    Subscribe = 6,
    /// Confirms an INVITE request
    Ack = 7,
    /// Terminates a session
    Bye = 8,
    /// Cancels establishing of a session
    Cancel = 9,
    /// Communicates information about the capabilities of the calling and receiving SIP phones
    Options = 10,
    /// Provisional Acknowledgement
    Prack = 11,
    /// Sends mid session information
    Info = 12,
    /// Asks the recipient to issue call transfer
    Refer = 13,
    /// Modifies the state of a session
    Update = 14,
}
impl Method {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Method::Unknown => "UNKNOWN",
            Method::Register => "REGISTER",
            Method::Invite => "INVITE",
            Method::Message => "MESSAGE",
            Method::Publish => "PUBLISH",
            Method::Notify => "NOTIFY",
            Method::Subscribe => "SUBSCRIBE",
            Method::Ack => "ACK",
            Method::Bye => "BYE",
            Method::Cancel => "CANCEL",
            Method::Options => "OPTIONS",
            Method::Prack => "PRACK",
            Method::Info => "INFO",
            Method::Refer => "REFER",
            Method::Update => "UPDATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "REGISTER" => Some(Self::Register),
            "INVITE" => Some(Self::Invite),
            "MESSAGE" => Some(Self::Message),
            "PUBLISH" => Some(Self::Publish),
            "NOTIFY" => Some(Self::Notify),
            "SUBSCRIBE" => Some(Self::Subscribe),
            "ACK" => Some(Self::Ack),
            "BYE" => Some(Self::Bye),
            "CANCEL" => Some(Self::Cancel),
            "OPTIONS" => Some(Self::Options),
            "PRACK" => Some(Self::Prack),
            "INFO" => Some(Self::Info),
            "REFER" => Some(Self::Refer),
            "UPDATE" => Some(Self::Update),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod processor_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Processor service
    #[derive(Debug, Clone)]
    pub struct ProcessorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProcessorClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProcessorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProcessorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ProcessorClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Process Message Request
        pub async fn process_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fonoster.routr.processor.v2beta1.Processor/ProcessMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "fonoster.routr.processor.v2beta1.Processor",
                        "ProcessMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod processor_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProcessorServer.
    #[async_trait]
    pub trait Processor: Send + Sync + 'static {
        /// Process Message Request
        async fn process_message(
            &self,
            request: tonic::Request<super::MessageRequest>,
        ) -> std::result::Result<tonic::Response<super::MessageResponse>, tonic::Status>;
    }
    /// Processor service
    #[derive(Debug)]
    pub struct ProcessorServer<T: Processor> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Processor> ProcessorServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProcessorServer<T>
    where
        T: Processor,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/fonoster.routr.processor.v2beta1.Processor/ProcessMessage" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessMessageSvc<T: Processor>(pub Arc<T>);
                    impl<T: Processor> tonic::server::UnaryService<super::MessageRequest>
                    for ProcessMessageSvc<T> {
                        type Response = super::MessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Processor>::process_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProcessMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Processor> Clone for ProcessorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Processor> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Processor> tonic::server::NamedService for ProcessorServer<T> {
        const NAME: &'static str = "fonoster.routr.processor.v2beta1.Processor";
    }
}
