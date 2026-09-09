/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation */

// C dependency: <linux/bitops.h>
// C dependency: "adf_accel_devices.h"

pub const ADF_BANK_INT_SRC_SEL_MASK_0: u32 = 0x4444444C;
pub const ADF_BANK_INT_SRC_SEL_MASK_X: u32 = 0x44444444;
pub const ADF_RING_CSR_RING_CONFIG: u32 = 0x000;
pub const ADF_RING_CSR_RING_LBASE: u32 = 0x040;
pub const ADF_RING_CSR_RING_UBASE: u32 = 0x080;
pub const ADF_RING_CSR_RING_HEAD: u32 = 0x0C0;
pub const ADF_RING_CSR_RING_TAIL: u32 = 0x100;
pub const ADF_RING_CSR_E_STAT: u32 = 0x14C;
pub const ADF_RING_CSR_INT_FLAG: u32 = 0x170;
pub const ADF_RING_CSR_INT_SRCSEL: u32 = 0x174;
pub const ADF_RING_CSR_INT_SRCSEL_2: u32 = 0x178;
pub const ADF_RING_CSR_INT_COL_EN: u32 = 0x17C;
pub const ADF_RING_CSR_INT_COL_CTL: u32 = 0x180;
pub const ADF_RING_CSR_INT_FLAG_AND_COL: u32 = 0x184;
pub const ADF_RING_CSR_INT_COL_CTL_ENABLE: u32 = 0x80000000;
pub const ADF_RING_BUNDLE_SIZE: u32 = 0x1000;
pub const ADF_ARB_REG_SLOT: u32 = 0x1000;
pub const ADF_ARB_RINGSRVARBEN_OFFSET: u32 = 0x19C;

#[macro_export]
macro_rules! BUILD_RING_BASE_ADDR {
    ($addr:expr, $size:expr) => {
        (($addr >> 6) & (u64::MAX << $size))
    };
}

#[macro_export]
macro_rules! READ_CSR_RING_HEAD {
    ($csr_base_addr:expr, $bank:expr, $ring:expr) => {
        ADF_CSR_RD($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_HEAD + ($ring << 2))
    };
}

#[macro_export]
macro_rules! READ_CSR_RING_TAIL {
    ($csr_base_addr:expr, $bank:expr, $ring:expr) => {
        ADF_CSR_RD($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_TAIL + ($ring << 2))
    };
}

#[macro_export]
macro_rules! READ_CSR_E_STAT {
    ($csr_base_addr:expr, $bank:expr) => {
        ADF_CSR_RD($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_E_STAT)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_RING_CONFIG {
    ($csr_base_addr:expr, $bank:expr, $ring:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_CONFIG + ($ring << 2), $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_RING_BASE {
    ($csr_base_addr:expr, $bank:expr, $ring:expr, $value:expr) => {{
        let l_base: u32 = ($value & 0xFFFFFFFF) as u32;
        let u_base: u32 = (($value & 0xFFFFFFFF00000000u64) >> 32) as u32;
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_LBASE + ($ring << 2), l_base);
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_UBASE + ($ring << 2), u_base);
    }};
}

#[macro_export]
macro_rules! WRITE_CSR_RING_HEAD {
    ($csr_base_addr:expr, $bank:expr, $ring:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_HEAD + ($ring << 2), $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_RING_TAIL {
    ($csr_base_addr:expr, $bank:expr, $ring:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_RING_TAIL + ($ring << 2), $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_INT_FLAG {
    ($csr_base_addr:expr, $bank:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_FLAG, $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_INT_SRCSEL {
    ($csr_base_addr:expr, $bank:expr) => {{
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_SRCSEL, ADF_BANK_INT_SRC_SEL_MASK_0);
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_SRCSEL_2, ADF_BANK_INT_SRC_SEL_MASK_X);
    }};
}

#[macro_export]
macro_rules! WRITE_CSR_INT_COL_EN {
    ($csr_base_addr:expr, $bank:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_COL_EN, $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_INT_COL_CTL {
    ($csr_base_addr:expr, $bank:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_COL_CTL, ADF_RING_CSR_INT_COL_CTL_ENABLE | $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_INT_FLAG_AND_COL {
    ($csr_base_addr:expr, $bank:expr, $value:expr) => {
        ADF_CSR_WR($csr_base_addr, (ADF_RING_BUNDLE_SIZE * $bank) + ADF_RING_CSR_INT_FLAG_AND_COL, $value)
    };
}

#[macro_export]
macro_rules! WRITE_CSR_RING_SRV_ARB_EN {
    ($csr_addr:expr, $index:expr, $value:expr) => {
        ADF_CSR_WR($csr_addr, ADF_ARB_RINGSRVARBEN_OFFSET + (ADF_ARB_REG_SLOT * $index), $value)
    };
}

extern "C" {
    pub fn adf_gen2_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops);
}

// Supplied by the translated dependency header.
#[allow(non_camel_case_types)]
pub enum adf_hw_csr_ops {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
