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
