/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 * Linker script variables to be set after section resolution, as
 * ld.lld does not like variables assigned before SECTIONS is processed.
 * Based on arch/arm64/kernel/image-vars.h
 */

// This header is intended for inclusion by vmlinux.lds.S with LINKER_SCRIPT
// defined. The following declarations preserve the linker-script symbols and
// aliases represented by the original C header.

#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    // __efistub__start = _start;
    #[link_name = "_start"]
    pub static __efistub__start: u8;

    // __efistub__start_kernel = _start_kernel;
    #[link_name = "_start_kernel"]
    pub static __efistub__start_kernel: u8;

    // __efistub__end = _end;
    #[link_name = "_end"]
    pub static __efistub__end: u8;

    // __efistub__edata = _edata;
    #[link_name = "_edata"]
    pub static __efistub__edata: u8;

    // __efistub___init_text_end = __init_text_end;
    #[link_name = "__init_text_end"]
    pub static __efistub___init_text_end: u8;
}

#[cfg(all(feature = "CONFIG_EFI", any(feature = "CONFIG_EFI_EARLYCON", feature = "CONFIG_SYSFB")))]
extern "C" {
    // __efistub_sysfb_primary_display = sysfb_primary_display;
    #[link_name = "sysfb_primary_display"]
    pub static __efistub_sysfb_primary_display: u8;
}

#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    /*
     * These double-word integer shifts are used by the library code, and
     * the first two of them are required to link EFI stub. Note __ashrdi3()
     * is not actually used by the stub but this may change in the future.
     * The PROVIDE assignments below are linker aliases.
     */
    // PROVIDE(__efistub___lshrdi3 = __lshrdi3);
    #[link_name = "__lshrdi3"]
    pub static __efistub___lshrdi3: u8;

    // PROVIDE(__efistub___ashldi3 = __ashldi3);
    #[link_name = "__ashldi3"]
    pub static __efistub___ashldi3: u8;

    // PROVIDE(__efistub___ashrdi3 = __ashrdi3);
    #[link_name = "__ashrdi3"]
    pub static __efistub___ashrdi3: u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
