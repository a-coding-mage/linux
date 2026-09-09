/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * 440SPe's DMA engines support header file
 *
 * 2006-2009 (C) DENX Software Engineering.
 *
 * Author: Yuri Tikhonov <yur@emcraft.com>
 */

// Dependency equivalent of <linux/types.h>.

pub const MAX_STAT_DMA_CDBS: usize = 16;
pub const DMA_ENGINES_NUM: usize = 2;
pub const DMA_DEST_MAX_NUM: usize = 2;

pub const DMA0_FIFO_SIZE: u32 = 0x1000;
pub const DMA1_FIFO_SIZE: u32 = 0x1000;
pub const DMA_FIFO_ENABLE: u32 = 1 << 12;

pub const DMA_CFG_DXEPR_LP: u32 = 0 << 26;
pub const DMA_CFG_DXEPR_HP: u32 = 3 << 26;
pub const DMA_CFG_DXEPR_HHP: u32 = 2 << 26;
pub const DMA_CFG_DXEPR_HHHP: u32 = 1 << 26;

pub const DMA_CFG_DFMPP_LP: u32 = 0 << 23;
pub const DMA_CFG_DFMPP_HP: u32 = 3 << 23;
pub const DMA_CFG_DFMPP_HHP: u32 = 2 << 23;
pub const DMA_CFG_DFMPP_HHHP: u32 = 1 << 23;
pub const DMA_CFG_FALGN: u32 = 1 << 19;

pub const D0CPF_INT: u32 = 1 << 12;
pub const D0CSF_INT: u32 = 1 << 11;
pub const D1CPF_INT: u32 = 1 << 10;
pub const D1CSF_INT: u32 = 1 << 9;
pub const DMAE_INT: u32 = 1 << 9;

pub const I2O_IOPIM_P0SNE: u32 = 1 << 3;
pub const I2O_IOPIM_P0EM: u32 = 1 << 5;
pub const I2O_IOPIM_P1SNE: u32 = 1 << 6;
pub const I2O_IOPIM_P1EM: u32 = 1 << 8;

pub const DMA_CDB_MSK: u32 = 0xF;
pub const DMA_CDB_64B_ADDR: u32 = 1 << 2;
pub const DMA_CDB_NO_INT: u32 = 1 << 3;
pub const DMA_CDB_STATUS_MSK: u32 = 0x3;
pub const DMA_CDB_ADDR_MSK: u32 = 0xFFFFFFF0;

pub const DMA_CDB_OPC_NO_OP: u32 = 0x00;
pub const DMA_CDB_OPC_MV_SG1_SG2: u32 = 0x01;
pub const DMA_CDB_OPC_MULTICAST: u32 = 0x05;
pub const DMA_CDB_OPC_DFILL128: u32 = 0x24;
pub const DMA_CDB_OPC_DCHECK128: u32 = 0x23;

pub const DMA_CUED_XOR_BASE: u32 = 0x10000000;
pub const DMA_CUED_XOR_HB: u32 = 0x00000008;

// CONFIG_440SP selects the corresponding hardware layout at build time.
#[cfg(feature = "CONFIG_440SP")]
pub const DMA_CUED_MULT1_OFF: u32 = 0;
#[cfg(feature = "CONFIG_440SP")]
pub const DMA_CUED_MULT2_OFF: u32 = 8;
#[cfg(feature = "CONFIG_440SP")]
pub const DMA_CUED_MULT3_OFF: u32 = 16;
#[cfg(feature = "CONFIG_440SP")]
pub const DMA_CUED_REGION_OFF: u32 = 24;
#[cfg(feature = "CONFIG_440SP")]
pub const DMA_CUED_XOR_WIN_MSK: u32 = 0xFC000000;
#[cfg(not(feature = "CONFIG_440SP"))]
pub const DMA_CUED_MULT1_OFF: u32 = 2;
#[cfg(not(feature = "CONFIG_440SP"))]
pub const DMA_CUED_MULT2_OFF: u32 = 10;
#[cfg(not(feature = "CONFIG_440SP"))]
pub const DMA_CUED_MULT3_OFF: u32 = 18;
#[cfg(not(feature = "CONFIG_440SP"))]
pub const DMA_CUED_REGION_OFF: u32 = 26;
#[cfg(not(feature = "CONFIG_440SP"))]
pub const DMA_CUED_XOR_WIN_MSK: u32 = 0xF0000000;

pub const DMA_CUED_REGION_MSK: u32 = 0x3;
pub const DMA_RXOR123: u32 = 0x0;
pub const DMA_RXOR124: u32 = 0x1;
pub const DMA_RXOR125: u32 = 0x2;
pub const DMA_RXOR12: u32 = 0x3;

pub const DMA_CDB_SG_SRC: u32 = 1;
pub const DMA_CDB_SG_DST1: u32 = 2;
pub const DMA_CDB_SG_DST2: u32 = 3;

#[repr(C)]
pub struct dma_cdb {
    pub pad0: [u8; 2],
    pub attr: u8,
    pub opc: u8,
    pub sg1u: u32,
    pub sg1l: u32,
    pub cnt: u32,
    pub sg2u: u32,
    pub sg2l: u32,
    pub sg3u: u32,
    pub sg3l: u32,
}

#[repr(C)]
pub struct dma_regs {
    pub cpfpl: u32, pub cpfph: u32, pub csfpl: u32, pub csfph: u32,
    pub dsts: u32, pub cfg: u32, pub pad0: [u8; 0x8],
    pub cpfhp: u16, pub cpftp: u16, pub csfhp: u16, pub csftp: u16,
    pub pad1: [u8; 0x8],
    pub acpl: u32, pub acph: u32, pub s1bpl: u32, pub s1bph: u32,
    pub s2bpl: u32, pub s2bph: u32, pub s3bpl: u32, pub s3bph: u32,
    pub pad2: [u8; 0x10], pub earl: u32, pub earh: u32, pub pad3: [u8; 0x8],
    pub seat: u32, pub sead: u32, pub op: u32, pub fsiz: u32,
}

#[repr(C)]
pub struct i2o_regs {
    pub ists: u32, pub iseat: u32, pub isead: u32, pub pad0: [u8; 0x14],
    pub idbel: u32, pub pad1: [u8; 0xc], pub ihis: u32, pub ihim: u32,
    pub pad2: [u8; 0x8], pub ihiq: u32, pub ihoq: u32, pub pad3: [u8; 0x8],
    pub iopis: u32, pub iopim: u32, pub iopiq: u32, pub iopoq: u8,
    pub pad4: [u8; 3], pub iiflh: u16, pub iiflt: u16, pub iiplh: u16,
    pub iiplt: u16, pub ioflh: u16, pub ioflt: u16, pub ioplh: u16,
    pub ioplt: u16, pub iidc: u32, pub ictl: u32, pub ifcpp: u32,
    pub pad5: [u8; 0x4], pub mfac0: u16, pub mfac1: u16, pub mfac2: u16,
    pub mfac3: u16, pub mfac4: u16, pub mfac5: u16, pub mfac6: u16,
    pub mfac7: u16, pub ifcfh: u16, pub ifcht: u16, pub pad6: [u8; 0x4],
    pub iifmc: u32, pub iodb: u32, pub iodbc: u32, pub ifbal: u32,
    pub ifbah: u32, pub ifsiz: u32, pub ispd0: u32, pub ispd1: u32,
    pub ispd2: u32, pub ispd3: u32, pub ihipl: u32, pub ihiph: u32,
    pub ihopl: u32, pub ihoph: u32, pub iiipl: u32, pub iiiph: u32,
    pub iiopl: u32, pub iioph: u32, pub ifcpl: u32, pub ifcph: u32,
    pub pad7: [u8; 0x8], pub iopt: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
