# Socken5

[![GitHub license](https://img.shields.io/github/license/FreeziFtw/Socken5)](./LICENSE.md)

An incomplete Rust library for the [socks5](https://tools.ietf.org/html/rfc1928) protocol.
- IPv4, IPv6 and Domain name
- No authentication
- Username and password authentication
- Connect command

#### Supported Address types:
- [IPv4](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
- [IPv6](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html)
- Domain name

## Method

#### Supported:
- No authentication
- Username/Password authentication
- Custom methods

#### Unsupported:
- [Gssapi](https://tools.ietf.org/html/rfc1961)

## Command

#### Supported:
- Connect

#### Unsupported:
- Bind
- Udp associate