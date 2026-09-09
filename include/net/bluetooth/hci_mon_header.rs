/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux

   Copyright (C) 2011-2012  Intel Corporation

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

#[repr(C, packed)]
pub struct hci_mon_hdr {
    pub opcode: __le16,
    pub index: __le16,
    pub len: __le16,
}

pub const HCI_MON_HDR_SIZE: usize = 6;

pub const HCI_MON_NEW_INDEX: usize = 0;
pub const HCI_MON_DEL_INDEX: usize = 1;
pub const HCI_MON_COMMAND_PKT: usize = 2;
pub const HCI_MON_EVENT_PKT: usize = 3;
pub const HCI_MON_ACL_TX_PKT: usize = 4;
pub const HCI_MON_ACL_RX_PKT: usize = 5;
pub const HCI_MON_SCO_TX_PKT: usize = 6;
pub const HCI_MON_SCO_RX_PKT: usize = 7;
pub const HCI_MON_OPEN_INDEX: usize = 8;
pub const HCI_MON_CLOSE_INDEX: usize = 9;
pub const HCI_MON_INDEX_INFO: usize = 10;
pub const HCI_MON_VENDOR_DIAG: usize = 11;
pub const HCI_MON_SYSTEM_NOTE: usize = 12;
pub const HCI_MON_USER_LOGGING: usize = 13;
pub const HCI_MON_CTRL_OPEN: usize = 14;
pub const HCI_MON_CTRL_CLOSE: usize = 15;
pub const HCI_MON_CTRL_COMMAND: usize = 16;
pub const HCI_MON_CTRL_EVENT: usize = 17;
pub const HCI_MON_ISO_TX_PKT: usize = 18;
pub const HCI_MON_ISO_RX_PKT: usize = 19;
pub const HCI_MON_DRV_TX_PKT: usize = 20;
pub const HCI_MON_DRV_RX_PKT: usize = 21;

#[repr(C, packed)]
pub struct hci_mon_new_index {
    pub type_: __u8,
    pub bus: __u8,
    pub bdaddr: bdaddr_t,
    pub name: [core::ffi::c_char; 8],
}

pub const HCI_MON_NEW_INDEX_SIZE: usize = 16;

#[repr(C, packed)]
pub struct hci_mon_index_info {
    pub bdaddr: bdaddr_t,
    pub manufacturer: __le16,
}

pub const HCI_MON_INDEX_INFO_SIZE: usize = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
