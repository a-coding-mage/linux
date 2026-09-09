/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header: linux/types.h
// External declarations supplied by other translation units:
pub struct dsa_switch;
pub struct sk_buff;

pub const QCA_HDR_LEN: u32 = 2;
pub const QCA_HDR_VERSION: u32 = 0x2;

pub const QCA_HDR_RECV_VERSION: u16 = 0xc000;
pub const QCA_HDR_RECV_PRIORITY: u16 = 0x3800;
pub const QCA_HDR_RECV_TYPE: u16 = 0x07c0;
pub const QCA_HDR_RECV_FRAME_IS_TAGGED: u16 = 0x0008;
pub const QCA_HDR_RECV_SOURCE_PORT: u16 = 0x0007;

/* Packet type for recv */
pub const QCA_HDR_RECV_TYPE_NORMAL: u32 = 0x0;
pub const QCA_HDR_RECV_TYPE_MIB: u32 = 0x1;
pub const QCA_HDR_RECV_TYPE_RW_REG_ACK: u32 = 0x2;

pub const QCA_HDR_XMIT_VERSION: u16 = 0xc000;
pub const QCA_HDR_XMIT_PRIORITY: u16 = 0x3800;
pub const QCA_HDR_XMIT_CONTROL: u16 = 0x0700;
pub const QCA_HDR_XMIT_FROM_CPU: u16 = 0x0080;
pub const QCA_HDR_XMIT_DP_BIT: u16 = 0x007f;

/* Packet type for xmit */
pub const QCA_HDR_XMIT_TYPE_NORMAL: u32 = 0x0;
pub const QCA_HDR_XMIT_TYPE_RW_REG: u32 = 0x1;

/* Check code for a valid mgmt packet. Switch will ignore the packet
 * with this wrong.
 */
pub const QCA_HDR_MGMT_CHECK_CODE_VAL: u32 = 0x5;

/* Specific define for in-band MDIO read/write with Ethernet packet */
pub const QCA_HDR_MGMT_SEQ_LEN: u32 = 4; /* 4 byte for the seq */
pub const QCA_HDR_MGMT_COMMAND_LEN: u32 = 4; /* 4 byte for the command */
pub const QCA_HDR_MGMT_DATA1_LEN: u32 = 4; /* First 4 byte for the mdio data */
pub const QCA_HDR_MGMT_HEADER_LEN: u32 =
    QCA_HDR_MGMT_SEQ_LEN + QCA_HDR_MGMT_COMMAND_LEN + QCA_HDR_MGMT_DATA1_LEN;

pub const QCA_HDR_MGMT_DATA2_LEN: u32 = 28; /* Other 28 byte for the mdio data */
pub const QCA_HDR_MGMT_PADDING_LEN: u32 = 18; /* Padding to reach the min Ethernet packet */

pub const QCA_HDR_MGMT_PKT_LEN: u32 = QCA_HDR_MGMT_HEADER_LEN
    + QCA_HDR_LEN
    + QCA_HDR_MGMT_DATA2_LEN
    + QCA_HDR_MGMT_PADDING_LEN;

pub const QCA_HDR_MGMT_SEQ_NUM: u32 = 0xffff_ffff; /* 63, 32 */
pub const QCA_HDR_MGMT_CHECK_CODE: u32 = 0xe000_0000; /* 31, 29 */
pub const QCA_HDR_MGMT_CMD: u32 = 1 << 28; /* 28 */
pub const QCA_HDR_MGMT_LENGTH: u32 = 0x00f0_0000; /* 23, 20 */
pub const QCA_HDR_MGMT_ADDR: u32 = 0x0007_ffff; /* 18, 0 */

/* Special struct emulating a Ethernet header */
#[repr(C, packed)]
pub struct qca_mgmt_ethhdr {
    pub command: u32,   /* __le32, command bit 31:0 */
    pub seq: u32,       /* __le32, seq 63:32 */
    pub mdio_data: u32, /* __le32, first 4byte mdio */
    pub hdr: u16,       /* __be16, qca hdr */
}

#[repr(u32)]
pub enum mdio_cmd {
    MDIO_WRITE = 0x0,
    MDIO_READ,
}

#[repr(C, packed)]
pub struct mib_ethhdr {
    pub data: [u32; 3], /* __le32, first 3 mib counter */
    pub hdr: u16,       /* __be16, qca hdr */
}

pub struct qca_tagger_data {
    pub rw_reg_ack_handler:
        Option<unsafe extern "C" fn(ds: *mut dsa_switch, skb: *mut sk_buff)>,
    pub mib_autocast_handler:
        Option<unsafe extern "C" fn(ds: *mut dsa_switch, skb: *mut sk_buff)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
