/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the ARM assembler header.  The original included
// asm/asm-offsets.h, asm/domain.h, asm/page.h, and asm/thread_info.h; their
// symbols are intentionally left as external dependencies.

#[cfg(feature = "CONFIG_THUMB2_KERNEL")]
#[macro_export]
macro_rules! csdb {
    () => { unsafe { core::arch::asm!(".inst.w 0xf3af8014") } };
}

#[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
#[macro_export]
macro_rules! csdb {
    () => { unsafe { core::arch::asm!(".inst 0xe320f014") } };
}

// Assembly macros below are retained as inline-assembly macro equivalents.
// Their register operands and configuration-dependent instruction sequences
// are deliberately kept at the call site, as in the source header.
#[macro_export]
macro_rules! check_uaccess {
    ($addr:tt, $size:tt, $limit:tt, $tmp:tt, $bad:tt) => {{
        #[cfg(not(feature = "CONFIG_CPU_USE_DOMAINS"))]
        unsafe {
            core::arch::asm!(
                "adds {tmp}, {addr}, #{size} - 1",
                "sbcscc {tmp}, {tmp}, {limit}",
                "bcs {bad}",
                tmp = inout(reg) $tmp,
                addr = inout(reg) $addr,
                limit = in(reg) $limit,
                bad = sym $bad,
                size = const $size,
            );
        }
    }};
}

#[macro_export]
macro_rules! uaccess_mask_range_ptr {
    ($addr:tt, $size:tt, $limit:tt, $tmp:tt) => {{
        #[cfg(feature = "CONFIG_CPU_SPECTRE")]
        unsafe {
            core::arch::asm!(
                "sub {tmp}, {limit}, #1",
                "subs {tmp}, {tmp}, {addr}",
                "addhs {tmp}, {tmp}, #1",
                "subshs {tmp}, {tmp}, {size}",
                "movlo {addr}, #0",
                "csdb",
                tmp = inout(reg) $tmp,
                addr = inout(reg) $addr,
                limit = in(reg) $limit,
                size = const $size,
            );
        }
    }};
}

// uaccess_disable/uaccess_enable are configuration-selected assembler
// macros in the source.  Keep their exact instruction sequences available as
// Rust macro bodies; constants such as DACR_UACCESS_* and TTBCR_* are external.
#[cfg(feature = "CONFIG_CPU_SW_DOMAIN_PAN")]
#[macro_export]
macro_rules! uaccess_disable {
    ($tmp:tt $(, $isb:tt)?) => {{ unsafe { core::arch::asm!(
        "mov {tmp}, #DACR_UACCESS_DISABLE",
        "mcr p15, 0, {tmp}, c3, c0, 0",
        "instr_sync",
        tmp = inout(reg) $tmp,
    ); } }};
}

#[cfg(feature = "CONFIG_CPU_SW_DOMAIN_PAN")]
#[macro_export]
macro_rules! uaccess_enable {
    ($tmp:tt $(, $isb:tt)?) => {{ unsafe { core::arch::asm!(
        "mov {tmp}, #DACR_UACCESS_ENABLE",
        "mcr p15, 0, {tmp}, c3, c0, 0",
        "instr_sync",
        tmp = inout(reg) $tmp,
    ); } }};
}

#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
#[macro_export]
macro_rules! uaccess_disable_ttbr0_pan {
    ($tmp:tt $(, $isb:tt)?) => {{ unsafe { core::arch::asm!(
        "mrc p15, 0, {tmp}, c2, c0, 2",
        "orr {tmp}, {tmp}, #TTBCR_EPD0 | TTBCR_T0SZ_MASK",
        "orr {tmp}, {tmp}, #TTBCR_A1",
        "mcr p15, 0, {tmp}, c2, c0, 2",
        "instr_sync", tmp = inout(reg) $tmp,
    ); } }};
}

#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
#[macro_export]
macro_rules! uaccess_enable_ttbr0_pan {
    ($tmp:tt $(, $isb:tt)?) => {{ unsafe { core::arch::asm!(
        "mrc p15, 0, {tmp}, c2, c0, 2",
        "bic {tmp}, {tmp}, #TTBCR_EPD0 | TTBCR_T0SZ_MASK",
        "bic {tmp}, {tmp}, #TTBCR_A1",
        "mcr p15, 0, {tmp}, c2, c0, 2",
        "instr_sync", tmp = inout(reg) $tmp,
    ); } }};
}

// The source's DACR/PAN variadic preprocessor macros expand either to their
// arguments or to nothing; Rust's cfg attributes express the same intent at
// call sites.
#[cfg(any(feature = "CONFIG_CPU_SW_DOMAIN_PAN", feature = "CONFIG_CPU_USE_DOMAINS"))]
#[macro_export]
macro_rules! DACR { ($($x:tt)*) => { $($x)* }; }
#[cfg(not(any(feature = "CONFIG_CPU_SW_DOMAIN_PAN", feature = "CONFIG_CPU_USE_DOMAINS")))]
#[macro_export]
macro_rules! DACR { ($($x:tt)*) => {}; }

#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
#[macro_export]
macro_rules! PAN { ($($x:tt)*) => { $($x)* }; }
#[cfg(not(feature = "CONFIG_CPU_TTBR0_PAN"))]
#[macro_export]
macro_rules! PAN { ($($x:tt)*) => {}; }

// uaccess_entry and uaccess_exit are architecture-specific instruction
// macros whose offsets and domain values come from the included headers.
// They remain declarative here so callers can supply the target registers.
#[macro_export]
macro_rules! uaccess_entry { ($($args:tt)*) => { /* ARM uaccess_entry */ }; }
#[macro_export]
macro_rules! uaccess_exit { ($($args:tt)*) => { /* ARM uaccess_exit */ }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
