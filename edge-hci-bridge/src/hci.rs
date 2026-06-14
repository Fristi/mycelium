use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};

pub const H4_CMD: u8 = 0x01;
pub const H4_ACL_HOST_TO_CTRL: u8 = 0x02;
pub const H4_EVT: u8 = 0x04;

#[derive(Debug, Clone)]
pub enum HostPacket {
    Command(Vec<u8>),
    Acl(Vec<u8>),
}

pub struct H4Reader<R> {
    reader: R,
    buf: [u8; 1],
}

impl<R: AsyncReadExt + Unpin> H4Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: [0u8; 1],
        }
    }

    pub async fn read_packet(&mut self) -> io::Result<HostPacket> {
        self.reader.read_exact(&mut self.buf).await?;
        let indicator = self.buf[0];
        match indicator {
            H4_CMD => {
                let mut header = [0u8; 3];
                self.reader.read_exact(&mut header).await?;
                let len = header[2] as usize;
                let mut payload = vec![0u8; len];
                self.reader.read_exact(&mut payload).await?;
                let mut packet = header.to_vec();
                packet.extend_from_slice(&payload);
                trace!(?packet, "HCI command from ESP host");
                Ok(HostPacket::Command(packet))
            }
            H4_ACL_HOST_TO_CTRL => {
                let mut header = [0u8; 4];
                self.reader.read_exact(&mut header).await?;
                let len = u16::from_le_bytes([header[2], header[3]]) as usize;
                let mut payload = vec![0u8; len];
                self.reader.read_exact(&mut payload).await?;
                let mut packet = header.to_vec();
                packet.extend_from_slice(&payload);
                trace!(len, "HCI ACL host->controller");
                Ok(HostPacket::Acl(packet))
            }
            other => {
                warn!(indicator = other, "unknown H4 indicator, draining");
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown H4 indicator 0x{other:02x}"),
                ))
            }
        }
    }
}

pub struct H4Writer<W> {
    writer: W,
}

impl<W: AsyncWriteExt + Unpin> H4Writer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub async fn write_event(&mut self, event: &[u8]) -> io::Result<()> {
        self.writer.write_all(&[H4_EVT]).await?;
        self.writer.write_all(event).await?;
        self.writer.flush().await?;
        debug!(len = event.len(), "HCI event -> ESP host");
        Ok(())
    }

    pub async fn write_acl_to_host(&mut self, acl: &[u8]) -> io::Result<()> {
        self.writer.write_all(&[H4_ACL_HOST_TO_CTRL]).await?;
        self.writer.write_all(acl).await?;
        self.writer.flush().await?;
        trace!(len = acl.len(), "HCI ACL controller->host");
        Ok(())
    }
}

pub fn command_opcode(packet: &[u8]) -> u16 {
    u16::from_le_bytes([packet[0], packet[1]])
}

pub fn command_complete(opcode: u16, status: u8, return_params: &[u8]) -> Vec<u8> {
    let param_len = 1 + 2 + 1 + return_params.len();
    let mut event = Vec::with_capacity(2 + param_len);
    event.push(0x0E);
    event.push(param_len as u8);
    event.push(0x01);
    event.extend_from_slice(&opcode.to_le_bytes());
    event.push(status);
    event.extend_from_slice(return_params);
    event
}

pub fn le_meta_event(subevent: u8, params: &[u8]) -> Vec<u8> {
    let param_len = 1 + params.len();
    let mut event = Vec::with_capacity(2 + param_len);
    event.push(0x3E);
    event.push(param_len as u8);
    event.push(subevent);
    event.extend_from_slice(params);
    event
}

pub fn le_connection_complete(
    handle: u16,
    role: u8,
    peer_addr_type: u8,
    peer_addr: [u8; 6],
    interval: u16,
    latency: u16,
    timeout: u16,
) -> Vec<u8> {
    let mut params = Vec::with_capacity(18);
    params.push(0x00); // status success
    params.extend_from_slice(&handle.to_le_bytes());
    params.push(role);
    params.push(peer_addr_type);
    params.extend_from_slice(&peer_addr);
    params.extend_from_slice(&interval.to_le_bytes());
    params.extend_from_slice(&latency.to_le_bytes());
    params.extend_from_slice(&timeout.to_le_bytes());
    params.push(0x00); // clock accuracy
    le_meta_event(0x01, &params)
}

pub fn split_serial(
    port: tokio_serial::SerialStream,
) -> (
    H4Reader<tokio::io::ReadHalf<tokio_serial::SerialStream>>,
    H4Writer<tokio::io::WriteHalf<tokio_serial::SerialStream>>,
    mpsc::Sender<Vec<u8>>,
) {
    let (reader, writer) = tokio::io::split(port);
    let (acl_tx, _acl_rx) = mpsc::channel(32);
    (H4Reader::new(reader), H4Writer::new(writer), acl_tx)
}
