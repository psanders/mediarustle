# MediaRustle

A cloud-native media server, built with Rust.

## Journey

- DAY 2: I'm learning about tonic and grpc
- DAY 2: I'm doing some setup to use Routr to control the recording server
- DAY 2: Decided to first work on a recording server
- DAY 1: Get an understanding of how to use the `tokio` crate
- DAY 1: I'm starting by revisiting some basic concepts including RTP, SRTP, RTCP, and SDP protocols

## NOTES

Prior to building, you may need to set the following environment variables:

```bash
# Update the following to match your OpenSSL installation
export LDFLAGS="-L/opt/homebrew/opt/openssl@3.1/lib/"
export CPPFLAGS="-I/opt/homebrew/opt/openssl@3.1/include"
``
