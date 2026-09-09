/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SM4 helper macros for Crypto Extensions
 * Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 *
 * The original definitions are AArch64 assembler macros.  They are retained
 * here as Rust declarative macros whose expansions issue the same instructions.
 */

#[macro_export]
macro_rules! SM4_PREPARE {
    ($ptr:tt) => {
        core::arch::asm!(
            "ld1 {{v24.16b-v27.16b}}, [{0}], #64",
            "ld1 {{v28.16b-v31.16b}}, [{0}]",
            inout(reg) $ptr => _,
            options(nostack)
        )
    };
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK_BE {
    ($b0:tt) => {
        core::arch::asm!(
            "sm4e {0}.4s, v24.4s", "sm4e {0}.4s, v25.4s",
            "sm4e {0}.4s, v26.4s", "sm4e {0}.4s, v27.4s",
            "sm4e {0}.4s, v28.4s", "sm4e {0}.4s, v29.4s",
            "sm4e {0}.4s, v30.4s", "sm4e {0}.4s, v31.4s",
            "rev64 {0}.4s, {0}.4s", "ext {0}.16b, {0}.16b, {0}.16b, #8",
            "rev32 {0}.16b, {0}.16b",
            inout(vreg) $b0,
            options(nostack)
        )
    };
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK {
    ($b0:tt) => {{
        core::arch::asm!("rev32 {0}.16b, {0}.16b", inout(vreg) $b0, options(nostack));
        $crate::SM4_CRYPT_BLK_BE!($b0);
    }};
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK2_BE {
    ($b0:tt, $b1:tt) => {
        core::arch::asm!(
            "sm4e {0}.4s, v24.4s", "sm4e {1}.4s, v24.4s",
            "sm4e {0}.4s, v25.4s", "sm4e {1}.4s, v25.4s",
            "sm4e {0}.4s, v26.4s", "sm4e {1}.4s, v26.4s",
            "sm4e {0}.4s, v27.4s", "sm4e {1}.4s, v27.4s",
            "sm4e {0}.4s, v28.4s", "sm4e {1}.4s, v28.4s",
            "sm4e {0}.4s, v29.4s", "sm4e {1}.4s, v29.4s",
            "sm4e {0}.4s, v30.4s", "sm4e {1}.4s, v30.4s",
            "sm4e {0}.4s, v31.4s", "sm4e {1}.4s, v31.4s",
            "rev64 {0}.4s, {0}.4s", "rev64 {1}.4s, {1}.4s",
            "ext {0}.16b, {0}.16b, {0}.16b, #8", "ext {1}.16b, {1}.16b, {1}.16b, #8",
            "rev32 {0}.16b, {0}.16b", "rev32 {1}.16b, {1}.16b",
            inout(vreg) $b0, inout(vreg) $b1,
            options(nostack)
        )
    };
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK2 {
    ($b0:tt, $b1:tt) => {{
        core::arch::asm!("rev32 {0}.16b, {0}.16b", "rev32 {1}.16b, {1}.16b", inout(vreg) $b0, inout(vreg) $b1, options(nostack));
        $crate::SM4_CRYPT_BLK2_BE!($b0, $b1);
    }};
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK4_BE {
    ($b0:tt, $b1:tt, $b2:tt, $b3:tt) => {
        core::arch::asm!(
            "sm4e {0}.4s, v24.4s", "sm4e {1}.4s, v24.4s", "sm4e {2}.4s, v24.4s", "sm4e {3}.4s, v24.4s",
            "sm4e {0}.4s, v25.4s", "sm4e {1}.4s, v25.4s", "sm4e {2}.4s, v25.4s", "sm4e {3}.4s, v25.4s",
            "sm4e {0}.4s, v26.4s", "sm4e {1}.4s, v26.4s", "sm4e {2}.4s, v26.4s", "sm4e {3}.4s, v26.4s",
            "sm4e {0}.4s, v27.4s", "sm4e {1}.4s, v27.4s", "sm4e {2}.4s, v27.4s", "sm4e {3}.4s, v27.4s",
            "sm4e {0}.4s, v28.4s", "sm4e {1}.4s, v28.4s", "sm4e {2}.4s, v28.4s", "sm4e {3}.4s, v28.4s",
            "sm4e {0}.4s, v29.4s", "sm4e {1}.4s, v29.4s", "sm4e {2}.4s, v29.4s", "sm4e {3}.4s, v29.4s",
            "sm4e {0}.4s, v30.4s", "sm4e {1}.4s, v30.4s", "sm4e {2}.4s, v30.4s", "sm4e {3}.4s, v30.4s",
            "sm4e {0}.4s, v31.4s", "sm4e {1}.4s, v31.4s", "sm4e {2}.4s, v31.4s", "sm4e {3}.4s, v31.4s",
            "rev64 {0}.4s, {0}.4s", "rev64 {1}.4s, {1}.4s", "rev64 {2}.4s, {2}.4s", "rev64 {3}.4s, {3}.4s",
            "ext {0}.16b, {0}.16b, {0}.16b, #8", "ext {1}.16b, {1}.16b, {1}.16b, #8", "ext {2}.16b, {2}.16b, {2}.16b, #8", "ext {3}.16b, {3}.16b, {3}.16b, #8",
            "rev32 {0}.16b, {0}.16b", "rev32 {1}.16b, {1}.16b", "rev32 {2}.16b, {2}.16b", "rev32 {3}.16b, {3}.16b",
            inout(vreg) $b0, inout(vreg) $b1, inout(vreg) $b2, inout(vreg) $b3,
            options(nostack)
        )
    };
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK4 {
    ($b0:tt, $b1:tt, $b2:tt, $b3:tt) => {{
        core::arch::asm!(
            "rev32 {0}.16b, {0}.16b", "rev32 {1}.16b, {1}.16b",
            "rev32 {2}.16b, {2}.16b", "rev32 {3}.16b, {3}.16b",
            inout(vreg) $b0, inout(vreg) $b1, inout(vreg) $b2, inout(vreg) $b3,
            options(nostack)
        );
        $crate::SM4_CRYPT_BLK4_BE!($b0, $b1, $b2, $b3);
    }};
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK8_BE {
    ($b0:tt, $b1:tt, $b2:tt, $b3:tt, $b4:tt, $b5:tt, $b6:tt, $b7:tt) => {
        $crate::SM4_CRYPT_BLK4_BE!($b0, $b1, $b2, $b3);
        $crate::SM4_CRYPT_BLK4_BE!($b4, $b5, $b6, $b7);
    };
}

#[macro_export]
macro_rules! SM4_CRYPT_BLK8 {
    ($b0:tt, $b1:tt, $b2:tt, $b3:tt, $b4:tt, $b5:tt, $b6:tt, $b7:tt) => {{
        $crate::SM4_CRYPT_BLK8_BE!($b0, $b1, $b2, $b3, $b4, $b5, $b6, $b7);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
