/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of the AArch64 user-access assembly header.
// The original includes provide assembler macros and symbols from other headers.

/*
 * User access enabling/disabling macros.
 *
 * CONFIG_ARM64_SW_TTBR0_PAN is a build-time configuration condition from the
 * original header; retain both configurations here as declarative assembler
 * macro definitions.
 */
#[cfg(feature = "CONFIG_ARM64_SW_TTBR0_PAN")]
macro_rules! __uaccess_ttbr0_disable {
    ($tmp1:ident) => {
        // mrs $tmp1, ttbr1_el1                 // swapper_pg_dir
        // bic $tmp1, $tmp1, #TTBRx_EL1_ASID_MASK
        // sub $tmp1, $tmp1, #RESERVED_SWAPPER_OFFSET // reserved_pg_dir
        // msr ttbr0_el1, $tmp1                 // set reserved TTBR0_EL1
        // add $tmp1, $tmp1, #RESERVED_SWAPPER_OFFSET
        // msr ttbr1_el1, $tmp1                 // set reserved ASID
        // isb
    };
}

#[cfg(feature = "CONFIG_ARM64_SW_TTBR0_PAN")]
macro_rules! __uaccess_ttbr0_enable {
    ($tmp1:ident, $tmp2:ident) => {
        // get_current_task $tmp1
        // ldr $tmp1, [$tmp1, #TSK_TI_TTBR0] // load saved TTBR0_EL1
        // mrs $tmp2, ttbr1_el1
        // extr $tmp2, $tmp2, $tmp1, #48
        // ror $tmp2, $tmp2, #16
        // msr ttbr1_el1, $tmp2              // set the active ASID
        // msr ttbr0_el1, $tmp1              // set the non-PAN TTBR0_EL1
        // isb
    };
}

#[cfg(feature = "CONFIG_ARM64_SW_TTBR0_PAN")]
macro_rules! uaccess_ttbr0_disable {
    ($tmp1:ident, $tmp2:ident) => {
        // alternative_if_not ARM64_HAS_PAN
        // save_and_disable_irq $tmp2       // avoid preemption
        // __uaccess_ttbr0_disable $tmp1
        // restore_irq $tmp2
        // alternative_else_nop_endif
    };
}

#[cfg(feature = "CONFIG_ARM64_SW_TTBR0_PAN")]
macro_rules! uaccess_ttbr0_enable {
    ($tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        // alternative_if_not ARM64_HAS_PAN
        // save_and_disable_irq $tmp3       // avoid preemption
        // __uaccess_ttbr0_enable $tmp1, $tmp2
        // restore_irq $tmp3
        // alternative_else_nop_endif
    };
}

#[cfg(not(feature = "CONFIG_ARM64_SW_TTBR0_PAN"))]
macro_rules! uaccess_ttbr0_disable {
    ($tmp1:ident, $tmp2:ident) => {};
}

#[cfg(not(feature = "CONFIG_ARM64_SW_TTBR0_PAN"))]
macro_rules! uaccess_ttbr0_enable {
    ($tmp1:ident, $tmp2:ident, $tmp3:ident) => {};
}

macro_rules! USER {
    ($l:tt, $($x:tt)*) => {
        // 9999: $($x)*;
        // _asm_extable_uaccess 9999b, $l
    };
}

macro_rules! USER_CPY {
    ($l:tt, $uaccess_is_write:tt, $($x:tt)*) => {
        // 9999: $($x)*;
        // _asm_extable_uaccess_cpy 9999b, $l, $uaccess_is_write
    };
}

/*
 * Generate the assembly for LDTR/STTR with exception table entries.
 * This is complicated as there is no post-increment or pair versions of the
 * unprivileged instructions, and USER() only works for single instructions.
 */
macro_rules! user_ldp {
    ($l:tt, $reg1:ident, $reg2:ident, $addr:ident, $post_inc:tt) => {
        // 8888: ldtr $reg1, [$addr];
        // 8889: ldtr $reg2, [$addr, #8];
        // add $addr, $addr, $post_inc;
        // _asm_extable_uaccess 8888b, $l;
        // _asm_extable_uaccess 8889b, $l;
    };
}

macro_rules! user_stp {
    ($l:tt, $reg1:ident, $reg2:ident, $addr:ident, $post_inc:tt) => {
        // 8888: sttr $reg1, [$addr];
        // 8889: sttr $reg2, [$addr, #8];
        // add $addr, $addr, $post_inc;
        // _asm_extable_uaccess 8888b, $l;
        // _asm_extable_uaccess 8889b, $l;
    };
}

macro_rules! user_ldst {
    ($l:tt, $inst:ident, $reg:ident, $addr:ident, $post_inc:tt) => {
        // 8888: $inst $reg, [$addr];
        // add $addr, $addr, $post_inc;
        // _asm_extable_uaccess 8888b, $l;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
