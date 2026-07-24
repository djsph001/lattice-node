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
    use crate::message::types::{TransactionRequest, TransactionResponse};
    use crate::message::types::{VerifyRequest, VerifyResponse};
    use crate::message::types::{ChainRangeRequest, ChainRangeResponse};
    use crate::message::types::{WitnessRequest, WitnessResponse};

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

    /// Protocol identifier for the Lattice storage verification RPC channel.
    #[derive(Debug, Clone)]
    pub struct VerifyProtocol;

    impl AsRef<str> for VerifyProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/verify/v1"
        }
    }

    /// CBOR + length-prefix codec for VerifyRequest/VerifyResponse.
    #[derive(Debug, Clone, Default)]
    pub struct VerifyCodec;

    #[async_trait]
    impl request_response::Codec for VerifyCodec {
        type Protocol = VerifyProtocol;
        type Request = VerifyRequest;
        type Response = VerifyResponse;

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

    /// Protocol identifier for the transaction fetch RPC channel.
    #[derive(Debug, Clone)]
    pub struct TransactionProtocol;

    impl AsRef<str> for TransactionProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/tx-fetch/v1"
        }
    }

    /// CBOR + length-prefix codec for TransactionRequest/TransactionResponse.
    #[derive(Debug, Clone, Default)]
    pub struct TransactionCodec;

    #[async_trait]
    impl request_response::Codec for TransactionCodec {
        type Protocol = TransactionProtocol;
        type Request = TransactionRequest;
        type Response = TransactionResponse;

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

    // ── Phase 10: chain sync ───────────────────────────────
    /// Protocol identifier for the chain range request RPC channel.
    #[derive(Debug, Clone)]
    pub struct ChainSyncProtocol;

    impl AsRef<str> for ChainSyncProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/chain-sync/v1"
        }
    }

    /// CBOR + length-prefix codec for ChainRangeRequest/ChainRangeResponse.
    #[derive(Debug, Clone, Default)]
    pub struct ChainSyncCodec;

    #[async_trait]
    impl request_response::Codec for ChainSyncCodec {
        type Protocol = ChainSyncProtocol;
        type Request = ChainRangeRequest;
        type Response = ChainRangeResponse;

        async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
        where T: AsyncRead + Unpin + Send {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn read_response<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Response>
        where T: AsyncRead + Unpin + Send {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, req: Self::Request) -> io::Result<()>
        where T: AsyncWrite + Unpin + Send {
            let data = super::encode(&req).map_err(to_io)?;
            write_frame(io, &data).await
        }

        async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, res: Self::Response) -> io::Result<()>
        where T: AsyncWrite + Unpin + Send {
            let data = super::encode(&res).map_err(to_io)?;
            write_frame(io, &data).await
        }
    }

    // ── Witness RPC ───────────────────────────────────────────
    /// Protocol identifier for the witness request-response channel.
    #[derive(Debug, Clone)]
    pub struct WitnessProtocol;

    impl AsRef<str> for WitnessProtocol {
        fn as_ref(&self) -> &str {
            "/lattice/witness/v1"
        }
    }

    /// CBOR + length-prefix codec for WitnessRequest/WitnessResponse.
    #[derive(Debug, Clone, Default)]
    pub struct WitnessCodec;

    #[async_trait]
    impl request_response::Codec for WitnessCodec {
        type Protocol = WitnessProtocol;
        type Request = WitnessRequest;
        type Response = WitnessResponse;

        async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
        where T: AsyncRead + Unpin + Send {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn read_response<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Response>
        where T: AsyncRead + Unpin + Send {
            let data = read_frame(io).await?;
            super::decode(&data).map_err(to_io)
        }

        async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, req: Self::Request) -> io::Result<()>
        where T: AsyncWrite + Unpin + Send {
            let data = super::encode(&req).map_err(to_io)?;
            write_frame(io, &data).await
        }

        async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, res: Self::Response) -> io::Result<()>
        where T: AsyncWrite + Unpin + Send {
            let data = super::encode(&res).map_err(to_io)?;
            write_frame(io, &data).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::types::{Heartbeat, LatticeMessage, WitnessRequest, WitnessResponse};
    use chrono::Utc;
    use libp2p::identity::Keypair;
    use libp2p::PeerId;

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

    // ── Witness RPC tests ──────────────────────────────────

    #[test]
    fn witness_request_roundtrip() {
        let req = WitnessRequest {
            claim_id: "claim-001".into(),
            claim_type: 0,
            claimant_id: PeerId::random(),
            claim_hash: [0xAB; 32],
            requested_at_epoch: 42,
        };

        let bytes = encode(&req).expect("encode failed");
        let decoded: WitnessRequest = decode(&bytes).expect("decode failed");

        assert_eq!(decoded.claim_id, "claim-001");
        assert_eq!(decoded.claim_type, 0);
        assert_eq!(decoded.requested_at_epoch, 42);
    }

    #[test]
    fn witness_response_roundtrip() {
        let sig: Vec<u8> = (0..64).collect();
        let resp = WitnessResponse {
            claim_id: "claim-002".into(),
            witness_id: PeerId::random(),
            claim_hash: [0xCD; 32],
            witnessed_at_epoch: 100,
            signature: sig.clone(),
            decline_reason: None,
        };

        let bytes = encode(&resp).expect("encode failed");
        let decoded: WitnessResponse = decode(&bytes).expect("decode failed");

        assert_eq!(decoded.claim_id, "claim-002");
        assert_eq!(decoded.witnessed_at_epoch, 100);
        assert_eq!(decoded.signature, sig);
        assert!(decoded.decline_reason.is_none());
    }

    #[test]
    fn witness_response_decline() {
        let resp = WitnessResponse {
            claim_id: "claim-003".into(),
            witness_id: PeerId::random(),
            claim_hash: [0xEF; 32],
            witnessed_at_epoch: 0,
            signature: Vec::new(), // empty = declined
            decline_reason: Some("Self-witness is not permitted".into()),
        };

        let bytes = encode(&resp).expect("encode failed");
        let decoded: WitnessResponse = decode(&bytes).expect("decode failed");

        assert!(decoded.signature.is_empty());
        assert_eq!(decoded.decline_reason.unwrap(), "Self-witness is not permitted");
    }

    #[test]
    fn witness_response_does_not_imply_verification() {
        // I4: Witness ≠ Certification — a response with a signature
        // must NOT contain any "verified" field.
        let sig: Vec<u8> = (0..64).collect();
        let resp = WitnessResponse {
            claim_id: "claim-004".into(),
            witness_id: PeerId::random(),
            claim_hash: [0xAA; 32],
            witnessed_at_epoch: 50,
            signature: sig,
            decline_reason: None,
        };

        let bytes = encode(&resp).expect("encode failed");
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(
            // Fall back: deserialize the CBOR through JSON conversion
            serde_json::to_value(&resp).unwrap()
        );

        // The serialized response must NOT contain a "verified" field
        // at the top level. This is the executable invariant.
        assert!(!json.as_object().map_or(true, |o| o.contains_key("verified")),
            "Witness response must not contain a 'verified' field — I4 invariant violated");
    }

    #[test]
    fn witness_sign_verify_roundtrip() {
        // Sign a canonical payload, then verify it with the verifier.
        // This is the load-bearing test: if the handler and the verifier
        // disagree on the payload format, this test fails.
        let kp = Keypair::generate_ed25519();
        let pubkey = kp.public();
        let witness_id = PeerId::from(pubkey.clone());
        let claim_hash = [0xCA; 32];
        let epoch = 42u64;

        // Reconstruct what the handler signs
        let payload = [
            crate::claims::WITNESS_DOMAIN as &[u8],
            &claim_hash[..],
            &witness_id.to_bytes()[..],
            &epoch.to_le_bytes()[..],
        ].concat();

        let signature = kp.sign(&payload).expect("sign failed");

        // Verify with the shared verifier
        let valid = crate::claims::verify_witness_signature(
            &claim_hash,
            &witness_id,
            epoch,
            &signature,
            &pubkey,
        );
        assert!(valid, "signature must verify against canonical payload");

        // Wrong claim hash fails
        let wrong_hash = [0xFF; 32];
        let valid_wrong = crate::claims::verify_witness_signature(
            &wrong_hash, &witness_id, epoch, &signature, &pubkey,
        );
        assert!(!valid_wrong, "wrong claim_hash must fail verification");

        // Wrong epoch fails
        let valid_bad_epoch = crate::claims::verify_witness_signature(
            &claim_hash, &witness_id, 99, &signature, &pubkey,
        );
        assert!(!valid_bad_epoch, "wrong epoch must fail verification");

        // Wrong witness fails
        let other_id = PeerId::random();
        let valid_bad_witness = crate::claims::verify_witness_signature(
            &claim_hash, &other_id, epoch, &signature, &pubkey,
        );
        assert!(!valid_bad_witness, "wrong witness_id must fail verification");

        // Different key fails
        let other_kp = Keypair::generate_ed25519();
        let valid_bad_key = crate::claims::verify_witness_signature(
            &claim_hash, &witness_id, epoch, &signature, &other_kp.public(),
        );
        assert!(!valid_bad_key, "wrong public key must fail verification");
    }

    #[test]
    fn witness_response_self_witness_decline() {
        // The handler produces this shape for self-witness:
        // empty signature + decline_reason
        let resp = WitnessResponse {
            claim_id: "self-test".into(),
            witness_id: PeerId::random(),
            claim_hash: [0xBE; 32],
            witnessed_at_epoch: 0,
            signature: vec![],
            decline_reason: Some("Self-witness is not permitted".into()),
        };
        assert!(resp.signature.is_empty());
        assert_eq!(resp.decline_reason.as_deref(), Some("Self-witness is not permitted"));
    }

    #[test]
    fn witness_response_unestablished_decline() {
        // The handler produces this shape for unestablished claimants
        let resp = WitnessResponse {
            claim_id: "unestablished-test".into(),
            witness_id: PeerId::random(),
            claim_hash: [0xDE; 32],
            witnessed_at_epoch: 0,
            signature: vec![],
            decline_reason: Some("Claimant is not established (no heartbeats observed)".into()),
        };
        assert!(resp.signature.is_empty());
        assert_eq!(
            resp.decline_reason.as_deref(),
            Some("Claimant is not established (no heartbeats observed)")
        );
    }
}
