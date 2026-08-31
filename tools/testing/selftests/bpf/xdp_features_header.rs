/* SPDX-License-Identifier: GPL-2.0 */

/* test commands */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum test_commands {
    CMD_STOP = 0,        /* CMD */
    CMD_START = 1,       /* CMD */
    CMD_ECHO = 2,        /* CMD */
    CMD_ACK = 3,         /* CMD + data */
    CMD_GET_XDP_CAP = 4, /* CMD */
    CMD_GET_STATS = 5,   /* CMD */
}

pub const DUT_CTRL_PORT: i32 = 12345;
pub const DUT_ECHO_PORT: i32 = 12346;

#[repr(C)]
pub struct tlv_hdr {
    pub type_: __be16,
    pub len: __be16,
    /* Flexible array member: __u8 data[] follows this header in memory. */
}
