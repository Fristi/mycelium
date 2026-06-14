use std::collections::HashMap;

use tracing::debug;

use crate::hci::{command_complete, command_opcode, le_connection_complete};

const OPCODE_RESET: u16 = 0x0C03;
const OPCODE_READ_BD_ADDR: u16 = 0x1009;
const OPCODE_READ_LOCAL_VERSION: u16 = 0x1001;
const OPCODE_READ_LOCAL_SUPPORTED_COMMANDS: u16 = 0x1002;
const OPCODE_READ_BUFFER_SIZE: u16 = 0x1005;
const OPCODE_SET_EVENT_MASK: u16 = 0x0C01;
const OPCODE_LE_SET_EVENT_MASK: u16 = 0x2001;
const OPCODE_LE_READ_BUFFER_SIZE: u16 = 0x2002;
const OPCODE_LE_READ_LOCAL_FEATURES: u16 = 0x2003;
const OPCODE_LE_SET_RANDOM_ADDRESS: u16 = 0x2005;
const OPCODE_LE_SET_ADV_PARAMS: u16 = 0x2006;
const OPCODE_LE_SET_ADV_DATA: u16 = 0x2008;
const OPCODE_LE_SET_SCAN_RSP: u16 = 0x2009;
const OPCODE_LE_SET_ADV_ENABLE: u16 = 0x200A;
const OPCODE_LE_READ_WHITE_LIST_SIZE: u16 = 0x200F;

#[derive(Debug, Default)]
pub struct ControllerState {
    pub random_addr: [u8; 6],
    pub adv_data: Vec<u8>,
    pub scan_rsp: Vec<u8>,
    pub advertising: bool,
    pub connection_handle: Option<u16>,
    pub next_handle: u16,
    pub esp_handles: HashMap<u16, Vec<u8>>,
}

impl ControllerState {
    pub fn new() -> Self {
        Self {
            next_handle: 0x0040,
            ..Default::default()
        }
    }

    pub fn handle_command(&mut self, packet: &[u8]) -> Vec<u8> {
        let opcode = command_opcode(packet);
        let params = &packet[3..];
        debug!(opcode = format_args!("0x{opcode:04x}"), "HCI command");

        match opcode {
            OPCODE_RESET => command_complete(opcode, 0x00, &[]),
            OPCODE_READ_BD_ADDR => {
                let mut ret = vec![0x00];
                ret.extend_from_slice(&[0x00, 0x1A, 0x7D, 0xDA, 0x71, 0x13, 0x00]);
                command_complete(opcode, 0x00, &ret)
            }
            OPCODE_READ_LOCAL_VERSION => command_complete(
                opcode,
                0x00,
                &[
                    0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0B, 0x00, 0x00, 0x00,
                ],
            ),
            OPCODE_READ_LOCAL_SUPPORTED_COMMANDS => {
                let mut mask = vec![0x00; 64];
                mask[5] = 0x20;
                mask[14] = 0x00;
                mask[25] = 0x40;
                mask[37] = 0xFF;
                mask[38] = 0xFF;
                mask[39] = 0xFF;
                let mut ret = vec![0x00];
                ret.extend_from_slice(&mask);
                command_complete(opcode, 0x00, &ret)
            }
            OPCODE_READ_BUFFER_SIZE => command_complete(
                opcode,
                0x00,
                &[0x00, 0xFF, 0xFF, 0x00, 0x00],
            ),
            OPCODE_SET_EVENT_MASK => command_complete(opcode, 0x00, &[]),
            OPCODE_LE_SET_EVENT_MASK => command_complete(opcode, 0x00, &[]),
            OPCODE_LE_READ_BUFFER_SIZE => command_complete(
                opcode,
                0x00,
                &[0x00, 0xFF, 0x00, 0xFF],
            ),
            OPCODE_LE_READ_LOCAL_FEATURES => {
                let mut ret = vec![0x00];
                ret.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
                command_complete(opcode, 0x00, &ret)
            }
            OPCODE_LE_READ_WHITE_LIST_SIZE => command_complete(opcode, 0x00, &[0x00, 0x0A]),
            OPCODE_LE_SET_RANDOM_ADDRESS => {
                if params.len() == 6 {
                    self.random_addr.copy_from_slice(params);
                }
                command_complete(opcode, 0x00, &[])
            }
            OPCODE_LE_SET_ADV_PARAMS => command_complete(opcode, 0x00, &[]),
            OPCODE_LE_SET_ADV_DATA => {
                if !params.is_empty() {
                    let len = params[0] as usize;
                    self.adv_data = params[1..1 + len.min(params.len() - 1)].to_vec();
                }
                command_complete(opcode, 0x00, &[])
            }
            OPCODE_LE_SET_SCAN_RSP => {
                if !params.is_empty() {
                    let len = params[0] as usize;
                    self.scan_rsp = params[1..1 + len.min(params.len() - 1)].to_vec();
                }
                command_complete(opcode, 0x00, &[])
            }
            OPCODE_LE_SET_ADV_ENABLE => {
                self.advertising = !params.is_empty() && params[0] == 1;
                command_complete(opcode, 0x00, &[])
            }
            _ => {
                debug!(opcode = format_args!("0x{opcode:04x}"), "unhandled opcode, returning success");
                command_complete(opcode, 0x00, &[])
            }
        }
    }

    pub fn connect_esp(&mut self, peer_addr: [u8; 6]) -> Vec<u8> {
        let handle = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1);
        self.connection_handle = Some(handle);
        le_connection_complete(
            handle,
            0x01, // peripheral
            0x01, // random address
            peer_addr,
            0x0018,
            0x0000,
            0x00C8,
        )
    }

    pub fn note_acl_from_host(&mut self, acl: &[u8]) {
        if acl.len() < 4 {
            return;
        }
        let _handle = u16::from_le_bytes([acl[0], acl[1]]) & 0x0FFF;
        let payload = &acl[4..];
        if payload.len() >= 5 && payload[4] == 0x0A {
            let handle = u16::from_le_bytes([payload[5], payload[6]]);
            if payload.len() > 7 {
                self.esp_handles.insert(handle, payload[7..].to_vec());
            }
        }
    }
}
