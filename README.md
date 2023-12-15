# MediaRustle

A cloud-native media server, built with Rust.

## NOTES

Prior to building, you may need to set the following environment variables:

```bash
# Update the following to match your OpenSSL installation
export LDFLAGS="-L/opt/homebrew/opt/openssl@3.1/lib/"
export CPPFLAGS="-I/opt/homebrew/opt/openssl@3.1/include"
``
