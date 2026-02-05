use anyhow::{bail, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use bytes::{Buf, BufMut, BytesMut};
use kexa_proto::{Block, Hash32};

pub const MAX_MESSAGE_SIZE: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum Message {
    Version { height: u64, tip: Hash32 },
    GetBlock { hash: Hash32 },
    GetBlocks { start_height: u64 },
    Block { block: Block },
    GetTip,
    Tip { height: u64, tip: Hash32 },
}

pub fn encode_message(message: &Message) -> Result<Vec<u8>> {
    let payload = borsh::to_vec(message)?;
    if payload.len() > MAX_MESSAGE_SIZE {
        bail!("message too large");
    }
    let mut buf = Vec::with_capacity(4 + payload.len());
    buf.put_u32(payload.len() as u32);
    buf.extend_from_slice(&payload);
    Ok(buf)
}

pub fn decode_message(buf: &mut BytesMut) -> Result<Option<Message>> {
    if buf.len() < 4 {
        return Ok(None);
    }
    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(&buf[..4]);
    let len = u32::from_be_bytes(len_bytes) as usize;
    if len > MAX_MESSAGE_SIZE {
        bail!("message too large");
    }
    if buf.len() < 4 + len {
        return Ok(None);
    }
    buf.advance(4);
    let payload = buf.split_to(len);
    let message = Message::try_from_slice(&payload)?;
    Ok(Some(message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_round_trip() {
        let msg = Message::GetTip;
        let data = encode_message(&msg).expect("encode");
        let mut buf = BytesMut::from(&data[..]);
        let decoded = decode_message(&mut buf).expect("decode").expect("msg");
        matches!(decoded, Message::GetTip);
    }

    #[test]
    fn reject_large_message() {
        let mut buf = BytesMut::new();
        buf.put_u32((MAX_MESSAGE_SIZE as u32) + 1);
        buf.extend_from_slice(&[0u8; 4]);
        let err = decode_message(&mut buf).unwrap_err();
        assert!(err.to_string().contains("too large"));
    }
}
