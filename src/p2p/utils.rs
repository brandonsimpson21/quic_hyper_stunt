use libp2p::{core::multiaddr::Protocol, core::Multiaddr};
use std::net::{Ipv4Addr, Ipv6Addr};

#[allow(unused)]
fn get_tcp_listen_all_addr(port: u16, use_ipv6: bool) -> Multiaddr {
    let listen_addr_tcp = Multiaddr::empty()
        .with(match use_ipv6 {
            true => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Tcp(port));
    listen_addr_tcp
}

#[allow(unused)]
fn get_quic_listen_all_addr(port: u16, use_ipv6: bool) -> Multiaddr {
    let listen_addr_quic = Multiaddr::empty()
        .with(match use_ipv6 {
            true => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Udp(port))
        .with(Protocol::QuicV1);
    listen_addr_quic
}
