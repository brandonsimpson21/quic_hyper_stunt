use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quic_hyper_stunt::quic::{
    self, client, common::read_recv_stream, error::NetworkError, handlers::handle_accept, server,
};
use quinn::Endpoint;
use std::{net::ToSocketAddrs, time::Duration};

async fn run_server(server: Endpoint, msg: Bytes) -> Result<(), NetworkError> {
    while let Some(conn) = server.accept().await {
        let conn = conn.await?;
        let msg = msg.clone();
        let _ = tokio::spawn(async move {
            let (mut send, recv) = conn.accept_bi().await?;
            let _ = read_recv_stream(recv, None).await?;
            send.write_all(&*msg).await?;
            send.finish().await?;
            Ok::<_, NetworkError>(())
        });
    }
    Ok(())
}

async fn run_client(client: Endpoint, msg: Bytes) -> Result<(), NetworkError> {
    let server_addr = "127.0.0.1:4000".to_socket_addrs()?.next().unwrap();

    let client_conn = client
        .connect(server_addr, "127.0.0.1")
        .map_err(|e| NetworkError::InternalError(e.to_string()))?
        .await?;
    let (mut client_send, client_recv) = client_conn.open_bi().await?;
    client_send.write_all(&*msg).await?;
    client_send.finish().await?;
    let _ = read_recv_stream(client_recv, None).await?;
    client.close(0u32.into(), b"done");
    let _ = tokio::time::timeout(Duration::from_secs(60), client.wait_idle()).await;
    Ok(())
}

fn test_client_server(n_clients: usize) -> Result<(), NetworkError> {
    let server_addr = "127.0.0.1:4000".to_socket_addrs()?.next().unwrap();

    let (client, server) = quic::get_self_signed_client_server(server_addr)?;
    tokio::spawn(async move {
        let _ = run_server(server, b"world".to_vec().into()).await;
    });
    for _ in (0..n_clients).into_iter() {
        let client = client.clone();
        tokio::spawn(async move {
            let _ = run_client(client, b"world".to_vec().into()).await;
        });
    }
    Ok(())
}

fn benchmark(n: usize) -> Result<(), NetworkError> {
    for _ in (0..n).into_iter() {
        test_client_server(20)?;
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench 10_000", |b| b.iter(|| benchmark(black_box(10000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
