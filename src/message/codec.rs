use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

/// Encode any serializable Lattice message to CBOR bytes.
///
/// CBOR (RFC 8949) chosen over protobuf/msgpack because:
///   - IETF standard, no corporate ownership
///   - Self-describing, schema-evolvable
///   - Compact binary format suitable for constrained Pi hardware
///   - First-class serde support in Rust
pub fn encode<T: Serialize>(msg: &T) -> Result<Vec<u8>> {
    let bytes = serde_cbor::to_vec(msg)?;
    Ok(bytes)
}

/// Decode CBOR bytes back into a typed Lattice message.
pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    let msg = serde_cbor::from_slice(bytes)?;
    Ok(msg)
}

/// Request-response codec for the Lattice RPC protocol (`/lattice/rpc/v1`).
///
/// Wraps the CBOR `encode`/`decode` helpers with a 4-byte big-endian length
/// prefix so the stream reader knows exactly where each framed message ends —
/// without framing, consecutive messages on a Yamux stream would run together.
pub mod rpc {
    use std::io;

    use async_trait::async_trait;
    use libp2p::futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
    use libp2p::request_response;

    use crate::message::types::{BalanceRequest, BalanceResponse, StatusRequest, StatusResponse};

    /// Maximum frame size (1 MiB) — guards against a malicious or buggy peer
    /// announcing a huge length prefix and exhausting memory.
    const MAX_FRAME_BYTES: u32 = 1024 * 1024;

    /// Protocol identifier for the Lattice direct-query RPC channel.
    #[derive(Debug, Clone)]
    pub struct LatticeProtocol;

    impl AsRef<str> for LatticeProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/rpc/v1"
        }
    }

    /// Protocol identifier for the Lattice balance query RPC channel.
    #[derive(Debug, Clone)]
    pub struct BalanceProtocol;

    impl AsRef<str> for BalanceProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/balance/v1"
        }
    }

    /// CBOR + length-prefix codec for StatusRequest/StatusResponse.
    #[derive(Debug, Clone, Default)]
    pub struct LatticeCodec;

    /// CBOR + length-prefix codec for BalanceRequest/BalanceResponse.
    #[derive(Debug, Clone, Default)]
    pub struct BalanceCodec;

    /// Read a length-prefixed CBOR frame from the stream.
    async fn read_frame<T>(io: &mut T) -> io::Result<Vec<u8>>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut len_buf = [0u8; 4];
        io.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf);
        if len > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("frame length {len} exceeds max {MAX_FRAME_BYTES}"),
            ));
        }
        let mut data = vec![0u8; len as usize];
        io.read_exact(&mut data).await?;
        Ok(data)
    }

    /// Write a length-prefixed CBOR frame to the stream.
    async fn write_frame<T>(io: &mut T, data: &[u8]) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let len = data.len() as u32;
        io.write_all(&len.to_be_bytes()).await?;
        io.write_all(data).await?;
        Ok(())
    }

    fn to_io<E: std::fmt::Display>(e: E) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
    }

    #[async_trait]
    impl request_response::Codec for LatticeCodec {
        type Protocol = LatticeProtocol;
        type Request = StatusRequest;
        type Response = StatusResponse;

        async fn read_request<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
        ) -> io::Result<Self::Request>
        where
            T: AsyncRead + Unpin + Send,
        {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn read_response<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
        ) -> io::Result<Self::Response>
        where
            T: AsyncRead + Unpin + Send,
        {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn write_request<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
            req: Self::Request,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            let data = super::encode(&req).map_err(to_io)?;
            write_frame(io, &data).await
        }

        async fn write_response<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
            res: Self::Response,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            let data = super::encode(&res).map_err(to_io)?;
            write_frame(io, &data).await
        }
    }

    #[async_trait]
    impl request_response::Codec for BalanceCodec {
        type Protocol = BalanceProtocol;
        type Request = BalanceRequest;
        type Response = BalanceResponse;

        async fn read_request<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
        ) -> io::Result<Self::Request>
        where
            T: AsyncRead + Unpin + Send,
        {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn read_response<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
        ) -> io::Result<Self::Response>
        where
            T: AsyncRead + Unpin + Send,
        {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn write_request<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
            req: Self::Request,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            let data = super::encode(&req).map_err(to_io)?;
            write_frame(io, &data).await
        }

        async fn write_response<T>(
            &mut self,
            _: &Self::Protocol,
            io: &mut T,
            res: Self::Response,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            let data = super::encode(&res).map_err(to_io)?;
            write_frame(io, &data).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::types::{Heartbeat, LatticeMessage};
    use chrono::Utc;

    #[test]
    fn roundtrip_heartbeat() {
        let msg = LatticeMessage::Heartbeat(Heartbeat {
            node_name: "test-node".to_string(),
            peer_id: "12D3KooWFake".to_string(),
            timestamp: Utc::now(),
            peer_count: 3,
        });

        let bytes = encode(&msg).expect("encode failed");
        let decoded: LatticeMessage = decode(&bytes).expect("decode failed");

        match decoded {
            LatticeMessage::Heartbeat(hb) => {
                assert_eq!(hb.node_name, "test-node");
                assert_eq!(hb.peer_count, 3);
            }
            _ => panic!("wrong message variant"),
        }
    }

    #[test]
    fn cbor_is_compact() {
        let msg = LatticeMessage::Heartbeat(Heartbeat {
            node_name: "lattice-alpha".to_string(),
            peer_id: "12D3KooWExamplePeerId".to_string(),
            timestamp: Utc::now(),
            peer_count: 5,
        });

        let cbor_bytes = encode(&msg).unwrap();
        let json_bytes = serde_json::to_vec(&msg).unwrap();

        // CBOR should be meaningfully smaller than JSON
        assert!(
            cbor_bytes.len() < json_bytes.len(),
            "CBOR ({} bytes) should be smaller than JSON ({} bytes)",
            cbor_bytes.len(),
            json_bytes.len()
        );
    }
}
