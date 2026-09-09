/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2017-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// #include <linux/types.h>

pub const PACKET_HEADER_PACKET_ID_SHIFT: u32 = 56;
pub const PACKET_HEADER_PACKET_ID_MASK: u64 = 0x1F00000000000000;

#[repr(i32)]
pub enum packet_id {
    PACKET_WREG_32 = 0x1,
    PACKET_WREG_BULK = 0x2,
    PACKET_MSG_LONG = 0x3,
    PACKET_MSG_SHORT = 0x4,
    PACKET_CP_DMA = 0x5,
    PACKET_REPEAT = 0x6,
    PACKET_MSG_PROT = 0x7,
    PACKET_FENCE = 0x8,
    PACKET_LIN_DMA = 0x9,
    PACKET_NOP = 0xA,
    PACKET_STOP = 0xB,
    PACKET_ARB_POINT = 0xC,
    PACKET_WAIT = 0xD,
    PACKET_LOAD_AND_EXE = 0xF,
    MAX_PACKET_ID = ((PACKET_HEADER_PACKET_ID_MASK >> PACKET_HEADER_PACKET_ID_SHIFT) + 1) as i32,
}

pub const GAUDI_PKT_CTL_OPCODE_SHIFT: u32 = 24;
pub const GAUDI_PKT_CTL_OPCODE_MASK: u32 = 0x1F000000;
pub const GAUDI_PKT_CTL_EB_SHIFT: u32 = 29;
pub const GAUDI_PKT_CTL_EB_MASK: u32 = 0x20000000;
pub const GAUDI_PKT_CTL_RB_SHIFT: u32 = 30;
pub const GAUDI_PKT_CTL_RB_MASK: u32 = 0x40000000;
pub const GAUDI_PKT_CTL_MB_SHIFT: u32 = 31;
pub const GAUDI_PKT_CTL_MB_MASK: u32 = 0x80000000;

/* All packets have, at least, an 8-byte header, which contains
 * the packet type. The kernel driver uses the packet header for packet
 * validation and to perform any necessary required preparation before
 * sending them off to the hardware.
 */
#[repr(C)]
pub struct gaudi_packet {
    pub header: __le64,
    /* The rest of the packet data follows. Use the corresponding
     * packet_XXX struct to deference the data, based on packet type
     */
    pub contents: [u8; 0],
}

#[repr(C)]
pub struct packet_nop { pub reserved: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_stop { pub reserved: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_wreg32 { pub value: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_wreg_bulk {
    pub size64: __le32,
    pub ctl: __le32,
    pub values: [__le64; 0], /* data starts here */
}

pub const GAUDI_PKT_LONG_CTL_OP_SHIFT: u32 = 20;
pub const GAUDI_PKT_LONG_CTL_OP_MASK: u32 = 0x00300000;

#[repr(C)]
pub struct packet_msg_long { pub value: __le32, pub ctl: __le32, pub addr: __le64 }

pub const GAUDI_PKT_SHORT_VAL_SOB_SYNC_VAL_SHIFT: u32 = 0;
pub const GAUDI_PKT_SHORT_VAL_SOB_SYNC_VAL_MASK: u32 = 0x00007FFF;
pub const GAUDI_PKT_SHORT_VAL_SOB_MOD_SHIFT: u32 = 31;
pub const GAUDI_PKT_SHORT_VAL_SOB_MOD_MASK: u32 = 0x80000000;
pub const GAUDI_PKT_SHORT_VAL_MON_SYNC_GID_SHIFT: u32 = 0;
pub const GAUDI_PKT_SHORT_VAL_MON_SYNC_GID_MASK: u32 = 0x000000FF;
pub const GAUDI_PKT_SHORT_VAL_MON_MASK_SHIFT: u32 = 8;
pub const GAUDI_PKT_SHORT_VAL_MON_MASK_MASK: u32 = 0x0000FF00;
pub const GAUDI_PKT_SHORT_VAL_MON_MODE_SHIFT: u32 = 16;
pub const GAUDI_PKT_SHORT_VAL_MON_MODE_MASK: u32 = 0x00010000;
pub const GAUDI_PKT_SHORT_VAL_MON_SYNC_VAL_SHIFT: u32 = 17;
pub const GAUDI_PKT_SHORT_VAL_MON_SYNC_VAL_MASK: u32 = 0xFFFE0000;
pub const GAUDI_PKT_SHORT_CTL_ADDR_SHIFT: u32 = 0;
pub const GAUDI_PKT_SHORT_CTL_ADDR_MASK: u32 = 0x0000FFFF;
pub const GAUDI_PKT_SHORT_CTL_OP_SHIFT: u32 = 20;
pub const GAUDI_PKT_SHORT_CTL_OP_MASK: u32 = 0x00300000;
pub const GAUDI_PKT_SHORT_CTL_BASE_SHIFT: u32 = 22;
pub const GAUDI_PKT_SHORT_CTL_BASE_MASK: u32 = 0x00C00000;

#[repr(C)]
pub struct packet_msg_short { pub value: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_msg_prot { pub value: __le32, pub ctl: __le32, pub addr: __le64 }

pub const GAUDI_PKT_FENCE_CFG_DEC_VAL_SHIFT: u32 = 0;
pub const GAUDI_PKT_FENCE_CFG_DEC_VAL_MASK: u32 = 0x0000000F;
pub const GAUDI_PKT_FENCE_CFG_TARGET_VAL_SHIFT: u32 = 16;
pub const GAUDI_PKT_FENCE_CFG_TARGET_VAL_MASK: u32 = 0x00FF0000;
pub const GAUDI_PKT_FENCE_CFG_ID_SHIFT: u32 = 30;
pub const GAUDI_PKT_FENCE_CFG_ID_MASK: u32 = 0xC0000000;
pub const GAUDI_PKT_FENCE_CTL_PRED_SHIFT: u32 = 0;
pub const GAUDI_PKT_FENCE_CTL_PRED_MASK: u32 = 0x0000001F;

#[repr(C)]
pub struct packet_fence { pub cfg: __le32, pub ctl: __le32 }

pub const GAUDI_PKT_LIN_DMA_CTL_WRCOMP_EN_SHIFT: u32 = 0;
pub const GAUDI_PKT_LIN_DMA_CTL_WRCOMP_EN_MASK: u32 = 0x00000001;
pub const GAUDI_PKT_LIN_DMA_CTL_LIN_SHIFT: u32 = 3;
pub const GAUDI_PKT_LIN_DMA_CTL_LIN_MASK: u32 = 0x00000008;
pub const GAUDI_PKT_LIN_DMA_CTL_MEMSET_SHIFT: u32 = 4;
pub const GAUDI_PKT_LIN_DMA_CTL_MEMSET_MASK: u32 = 0x00000010;
pub const GAUDI_PKT_LIN_DMA_DST_ADDR_SHIFT: u32 = 0;
pub const GAUDI_PKT_LIN_DMA_DST_ADDR_MASK: u64 = 0x00FFFFFFFFFFFFFF;

#[repr(C)]
pub struct packet_lin_dma {
    pub tsize: __le32,
    pub ctl: __le32,
    pub src_addr: __le64,
    pub dst_addr: __le64,
}

#[repr(C)]
pub struct packet_arb_point { pub cfg: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_repeat { pub cfg: __le32, pub ctl: __le32 }
#[repr(C)]
pub struct packet_wait { pub cfg: __le32, pub ctl: __le32 }

pub const GAUDI_PKT_LOAD_AND_EXE_CFG_DST_SHIFT: u32 = 0;
pub const GAUDI_PKT_LOAD_AND_EXE_CFG_DST_MASK: u32 = 0x00000001;

#[repr(C)]
pub struct packet_load_and_exe { pub cfg: __le32, pub ctl: __le32, pub src_addr: __le64 }
#[repr(C)]
pub struct packet_cp_dma { pub tsize: __le32, pub ctl: __le32, pub src_addr: __le64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
