/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Arm Ltd.
 */

// Dependency intent: symbols from <asm/sysreg.h> are supplied externally.

pub const POR_EL0_INIT: u64 = POR_ELx_PERM_PREP(0, POE_RWX);

#[inline]
pub fn por_elx_allows_read(por: u64, pkey: u8) -> bool {
    let perm: u8 = POR_ELx_PERM_GET(pkey, por);

    (perm & POE_R) != 0
}

#[inline]
pub fn por_elx_allows_write(por: u64, pkey: u8) -> bool {
    let perm: u8 = POR_ELx_PERM_GET(pkey, por);

    (perm & POE_W) != 0
}

#[inline]
pub fn por_elx_allows_exec(por: u64, pkey: u8) -> bool {
    let perm: u8 = POR_ELx_PERM_GET(pkey, por);

    (perm & POE_X) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
