/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation */

// Linux headers omitted; their types and CSR accessors are external dependencies.

pub const ADF_BANK_INT_SRC_SEL_MASK: usize = 0x44;
pub const ADF_RING_CSR_RING_CONFIG: usize = 0x1000;
pub const ADF_RING_CSR_RING_LBASE: usize = 0x1040;
pub const ADF_RING_CSR_RING_UBASE: usize = 0x1080;
pub const ADF_RING_CSR_RING_HEAD: usize = 0x0C0;
pub const ADF_RING_CSR_RING_TAIL: usize = 0x100;
pub const ADF_RING_CSR_STAT: usize = 0x140;
pub const ADF_RING_CSR_UO_STAT: usize = 0x148;
pub const ADF_RING_CSR_E_STAT: usize = 0x14C;
pub const ADF_RING_CSR_NE_STAT: usize = 0x150;
pub const ADF_RING_CSR_NF_STAT: usize = 0x154;
pub const ADF_RING_CSR_F_STAT: usize = 0x158;
pub const ADF_RING_CSR_C_STAT: usize = 0x15C;
pub const ADF_RING_CSR_INT_FLAG_EN: usize = 0x16C;
pub const ADF_RING_CSR_INT_FLAG: usize = 0x170;
pub const ADF_RING_CSR_INT_SRCSEL: usize = 0x174;
pub const ADF_RING_CSR_INT_COL_EN: usize = 0x17C;
pub const ADF_RING_CSR_INT_COL_CTL: usize = 0x180;
pub const ADF_RING_CSR_INT_FLAG_AND_COL: usize = 0x184;
pub const ADF_RING_CSR_EXP_STAT: usize = 0x188;
pub const ADF_RING_CSR_EXP_INT_EN: usize = 0x18C;
pub const ADF_RING_CSR_INT_COL_CTL_ENABLE: u32 = 0x80000000;
pub const ADF_RING_CSR_ADDR_OFFSET: usize = 0x100000;
pub const ADF_RING_BUNDLE_SIZE: usize = 0x2000;
pub const ADF_RING_CSR_RING_SRV_ARB_EN: usize = 0x19C;

macro_rules! BUILD_RING_BASE_ADDR {
    ($addr:expr, $size:expr) => { ((($addr >> 6) & (u64::MAX << $size)) << 6) };
}

macro_rules! READ_CSR_RING_HEAD { ($b:expr,$k:expr,$r:expr) => { ADF_CSR_RD!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * $k + ADF_RING_CSR_RING_HEAD + ($r << 2)) }; }
macro_rules! READ_CSR_RING_TAIL { ($b:expr,$k:expr,$r:expr) => { ADF_CSR_RD!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * $k + ADF_RING_CSR_RING_TAIL + ($r << 2)) }; }

macro_rules! csr_read_bank { ($b:expr,$k:expr,$o:expr) => { ADF_CSR_RD!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * $k + $o) }; }
macro_rules! csr_write_bank { ($b:expr,$k:expr,$o:expr,$v:expr) => { ADF_CSR_WR!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * $k + $o, $v) }; }

macro_rules! READ_CSR_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_STAT) }; }
macro_rules! READ_CSR_UO_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_UO_STAT) }; }
macro_rules! READ_CSR_E_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_E_STAT) }; }
macro_rules! READ_CSR_NE_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_NE_STAT) }; }
macro_rules! READ_CSR_NF_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_NF_STAT) }; }
macro_rules! READ_CSR_F_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_F_STAT) }; }
macro_rules! READ_CSR_C_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_C_STAT) }; }
macro_rules! READ_CSR_EXP_STAT { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_EXP_STAT) }; }
macro_rules! READ_CSR_EXP_INT_EN { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_EXP_INT_EN) }; }
macro_rules! WRITE_CSR_EXP_INT_EN { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_EXP_INT_EN,$v) }; }
macro_rules! READ_CSR_RING_CONFIG { ($b:expr,$k:expr,$r:expr) => { ADF_CSR_RD!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE*$k + ADF_RING_CSR_RING_CONFIG + ($r<<2)) }; }
macro_rules! WRITE_CSR_RING_CONFIG { ($b:expr,$k:expr,$r:expr,$v:expr) => { ADF_CSR_WR!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE*$k + ADF_RING_CSR_RING_CONFIG + ($r<<2), $v) }; }

macro_rules! WRITE_CSR_RING_BASE {
    ($base:expr, $bank:expr, $ring:expr, $value:expr) => {{
        let _csr_base_addr = $base;
        let _bank: u32 = $bank;
        let _ring: u32 = $ring;
        let _value: u64 = $value;
        let l_base: u32 = _value as u32;
        let u_base: u32 = (_value >> 32) as u32;
        ADF_CSR_WR!(_csr_base_addr + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * (_bank as usize) + ADF_RING_CSR_RING_LBASE + ((_ring as usize)<<2), l_base);
        ADF_CSR_WR!(_csr_base_addr + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE * (_bank as usize) + ADF_RING_CSR_RING_UBASE + ((_ring as usize)<<2), u_base);
    }};
}

pub unsafe fn read_base(csr_base_addr: *mut core::ffi::c_void, bank: u32, ring: u32) -> u64 {
    /* Use special IO wrapper for ring base as LBASE and UBASE are not physically contigious. */
    let l_base: u32 = ADF_CSR_RD!(csr_base_addr, ADF_RING_BUNDLE_SIZE * bank as usize + ADF_RING_CSR_RING_LBASE + (ring as usize << 2));
    let u_base: u32 = ADF_CSR_RD!(csr_base_addr, ADF_RING_BUNDLE_SIZE * bank as usize + ADF_RING_CSR_RING_UBASE + (ring as usize << 2));
    (u_base as u64) << 32 | l_base as u64
}

macro_rules! READ_CSR_RING_BASE { ($b:expr,$k:expr,$r:expr) => { read_base($b + ADF_RING_CSR_ADDR_OFFSET, $k, $r) }; }
macro_rules! WRITE_CSR_RING_HEAD { ($b:expr,$k:expr,$r:expr,$v:expr) => { ADF_CSR_WR!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE*$k + ADF_RING_CSR_RING_HEAD + ($r<<2), $v) }; }
macro_rules! WRITE_CSR_RING_TAIL { ($b:expr,$k:expr,$r:expr,$v:expr) => { ADF_CSR_WR!($b + ADF_RING_CSR_ADDR_OFFSET, ADF_RING_BUNDLE_SIZE*$k + ADF_RING_CSR_RING_TAIL + ($r<<2), $v) }; }

macro_rules! READ_CSR_INT_EN { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_FLAG_EN) }; }
macro_rules! WRITE_CSR_INT_EN { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_FLAG_EN,$v) }; }
macro_rules! READ_CSR_INT_FLAG { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_FLAG) }; }
macro_rules! WRITE_CSR_INT_FLAG { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_FLAG,$v) }; }
macro_rules! READ_CSR_INT_SRCSEL { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_SRCSEL) }; }
macro_rules! WRITE_CSR_INT_SRCSEL { ($b:expr,$k:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_SRCSEL,ADF_BANK_INT_SRC_SEL_MASK) }; }
macro_rules! WRITE_CSR_INT_SRCSEL_W_VAL { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_SRCSEL,$v) }; }
macro_rules! READ_CSR_INT_COL_EN { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_COL_EN) }; }
macro_rules! WRITE_CSR_INT_COL_EN { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_COL_EN,$v) }; }
macro_rules! READ_CSR_INT_COL_CTL { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_COL_CTL) }; }
macro_rules! WRITE_CSR_INT_COL_CTL { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_COL_CTL,ADF_RING_CSR_INT_COL_CTL_ENABLE | $v) }; }
macro_rules! READ_CSR_INT_FLAG_AND_COL { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_INT_FLAG_AND_COL) }; }
macro_rules! WRITE_CSR_INT_FLAG_AND_COL { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_INT_FLAG_AND_COL,$v) }; }
macro_rules! READ_CSR_RING_SRV_ARB_EN { ($b:expr,$k:expr) => { csr_read_bank!($b,$k,ADF_RING_CSR_RING_SRV_ARB_EN) }; }
macro_rules! WRITE_CSR_RING_SRV_ARB_EN { ($b:expr,$k:expr,$v:expr) => { csr_write_bank!($b,$k,ADF_RING_CSR_RING_SRV_ARB_EN,$v) }; }

extern "C" {
    pub fn adf_gen4_init_hw_csr_ops(csr_ops: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
