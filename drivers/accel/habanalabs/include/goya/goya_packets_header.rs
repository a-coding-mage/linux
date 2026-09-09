/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2017-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Dependency supplied by the surrounding translation unit: linux/types.h

pub const PACKET_HEADER_PACKET_ID_SHIFT: u32 = 56;
pub const PACKET_HEADER_PACKET_ID_MASK: u64 = 0x1F00000000000000u64;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum packet_id {
    PACKET_WREG_32 = 0x1,
    PACKET_WREG_BULK = 0x2,
    PACKET_MSG_LONG = 0x3,
    PACKET_MSG_SHORT = 0x4,
    PACKET_CP_DMA = 0x5,
    PACKET_MSG_PROT = 0x7,
    PACKET_FENCE = 0x8,
    PACKET_LIN_DMA = 0x9,
    PACKET_NOP = 0xA,
    PACKET_STOP = 0xB,
    MAX_PACKET_ID = ((PACKET_HEADER_PACKET_ID_MASK >> PACKET_HEADER_PACKET_ID_SHIFT) + 1) as u32,
}

pub const GOYA_PKT_CTL_OPCODE_SHIFT: u32 = 24;
pub const GOYA_PKT_CTL_OPCODE_MASK: u32 = 0x1F000000;

pub const GOYA_PKT_CTL_EB_SHIFT: u32 = 29;
pub const GOYA_PKT_CTL_EB_MASK: u32 = 0x20000000;

pub const GOYA_PKT_CTL_RB_SHIFT: u32 = 30;
pub const GOYA_PKT_CTL_RB_MASK: u32 = 0x40000000;

pub const GOYA_PKT_CTL_MB_SHIFT: u32 = 31;
pub const GOYA_PKT_CTL_MB_MASK: u32 = 0x80000000;

/* All packets have, at least, an 8-byte header, which contains
 * the packet type. The kernel driver uses the packet header for packet
 * validation and to perform any necessary required preparation before
 * sending them off to the hardware.
 */
#[repr(C)]
pub struct goya_packet {
    pub header: __le64,
    /* The rest of the packet data follows. Use the corresponding
     * packet_XXX struct to deference the data, based on packet type
     */
    pub contents: [u8; 0],
}

#[repr(C)]
pub struct packet_nop {
    pub reserved: __le32,
    pub ctl: __le32,
}

#[repr(C)]
pub struct packet_stop {
    pub reserved: __le32,
    pub ctl: __le32,
}

pub const GOYA_PKT_WREG32_CTL_REG_OFFSET_SHIFT: u32 = 0;
pub const GOYA_PKT_WREG32_CTL_REG_OFFSET_MASK: u32 = 0x0000FFFF;

#[repr(C)]
pub struct packet_wreg32 {
    pub value: __le32,
    pub ctl: __le32,
}

#[repr(C)]
pub struct packet_wreg_bulk {
    pub size64: __le32,
    pub ctl: __le32,
    pub values: [__le64; 0], /* data starts here */
}

#[repr(C)]
pub struct packet_msg_long {
    pub value: __le32,
    pub ctl: __le32,
    pub addr: __le64,
}

#[repr(C)]
pub struct packet_msg_short {
    pub value: __le32,
    pub ctl: __le32,
}

#[repr(C)]
pub struct packet_msg_prot {
    pub value: __le32,
    pub ctl: __le32,
    pub addr: __le64,
}

#[repr(C)]
pub struct packet_fence {
    pub cfg: __le32,
    pub ctl: __le32,
}

pub const GOYA_PKT_LIN_DMA_CTL_WO_SHIFT: u32 = 0;
pub const GOYA_PKT_LIN_DMA_CTL_WO_MASK: u32 = 0x00000001;

pub const GOYA_PKT_LIN_DMA_CTL_RDCOMP_SHIFT: u32 = 1;
pub const GOYA_PKT_LIN_DMA_CTL_RDCOMP_MASK: u32 = 0x00000002;

pub const GOYA_PKT_LIN_DMA_CTL_WRCOMP_SHIFT: u32 = 2;
pub const GOYA_PKT_LIN_DMA_CTL_WRCOMP_MASK: u32 = 0x00000004;

pub const GOYA_PKT_LIN_DMA_CTL_MEMSET_SHIFT: u32 = 6;
pub const GOYA_PKT_LIN_DMA_CTL_MEMSET_MASK: u32 = 0x00000040;

pub const GOYA_PKT_LIN_DMA_CTL_DMA_DIR_SHIFT: u32 = 20;
pub const GOYA_PKT_LIN_DMA_CTL_DMA_DIR_MASK: u32 = 0x00700000;

#[repr(C)]
pub struct packet_lin_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
    pub dst_addr: __le64,
}

#[repr(C)]
pub struct packet_cp_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
