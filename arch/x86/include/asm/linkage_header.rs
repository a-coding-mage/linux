/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the x86 linkage header. The included kernel symbols and
// configuration options are supplied by the surrounding translation unit.

#[allow(unused_macros)]
macro_rules! notrace {
    ($item:item) => { $item };
}

// CONFIG_64BIT: the generic version can create spurious ENDBR instructions.
#[cfg(target_pointer_width = "64")]
#[allow(unused_macros)]
macro_rules! _THIS_IP_ {
    () => {{
        let mut __here: usize;
        unsafe {
            core::arch::asm!("lea 0({rip}), {here}", rip = const 0, here = lateout(reg) __here);
        }
        __here
    }};
}

// CONFIG_X86_32: CPP_ASMLINKAGE with __attribute__((regparm(0))).

// __ALIGN is `.balign CONFIG_FUNCTION_ALIGNMENT, 0x90;`.
#[allow(unused_macros)]
macro_rules! __ALIGN {
    () => { ".balign CONFIG_FUNCTION_ALIGNMENT, 0x90;" };
}

#[allow(unused_macros)]
macro_rules! __ALIGN_STR {
    () => { ".balign CONFIG_FUNCTION_ALIGNMENT, 0x90;" };
}

// FUNCTION_PADDING is `.skip CONFIG_FUNCTION_ALIGNMENT, 0x90;` when
// CONFIG_CALL_PADDING is enabled and exports/VDSO are not being built.
#[allow(unused_macros)]
macro_rules! FUNCTION_PADDING {
    () => { ".skip CONFIG_FUNCTION_ALIGNMENT, 0x90;" };
}

#[allow(unused_macros)]
macro_rules! __FUNC_ALIGN {
    () => { __ALIGN!() };
}

#[allow(unused_macros)]
macro_rules! ASM_FUNC_ALIGN {
    () => { __FUNC_ALIGN!() };
}

#[allow(unused_macros)]
macro_rules! SYM_F_ALIGN {
    () => { __FUNC_ALIGN!() };
}

// Assembly-side RET expands according to CONFIG_MITIGATION_RETHUNK and
// CONFIG_MITIGATION_SLS. The corresponding inline-assembly spelling is below.
#[allow(unused_macros)]
macro_rules! RET {
    () => { "ret" };
}

#[allow(unused_macros)]
macro_rules! ASM_RET {
    () => { "ret\n\t" };
}

/*
 * Depending on -fpatchable-function-entry=N,N usage (CONFIG_CALL_PADDING) the
 * CFI symbol layout changes.
 */
#[allow(unused_macros)]
macro_rules! CFI_PRE_PADDING {
    () => { ".skip CONFIG_FUNCTION_PADDING_BYTES, 0x90;" };
}

#[allow(unused_macros)]
macro_rules! CFI_POST_PADDING {
    () => { "" };
}

#[allow(unused_macros)]
macro_rules! __CFI_TYPE {
    ($name:ident) => {
        SYM_START!(__cfi_$name, SYM_L_LOCAL, SYM_A_NONE);
        CFI_PRE_PADDING!();
        ".byte 0xb8";
        ".long __kcfi_typeid_";
        CFI_POST_PADDING!();
        SYM_END!(__cfi_$name, SYM_T_FUNC);
    };
}

/* UML needs to be able to override memcpy() and friends for KASAN. */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_ALIAS_MEMFUNC {
    () => { SYM_FUNC_ALIAS! };
}

/* SYM_TYPED_FUNC_START -- use for indirectly called globals, w/ CFI type */
#[allow(unused_macros)]
macro_rules! SYM_TYPED_FUNC_START {
    ($name:ident) => {
        SYM_TYPED_START!($name, SYM_L_GLOBAL, SYM_F_ALIGN!());
        ENDBR!();
    };
}

/* SYM_FUNC_START -- use for global functions */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START {
    ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_F_ALIGN!()); };
}

/* SYM_FUNC_START_NOALIGN -- use for global functions, w/o alignment */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_NOALIGN {
    ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE); };
}

/* SYM_FUNC_START_LOCAL -- use for local functions */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_LOCAL {
    ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_F_ALIGN!()); };
}

/* SYM_FUNC_START_LOCAL_NOALIGN -- use for local functions, w/o alignment */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_LOCAL_NOALIGN {
    ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE); };
}

/* SYM_FUNC_START_WEAK -- use for weak functions */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_WEAK {
    ($name:ident) => { SYM_START!($name, SYM_L_WEAK, SYM_F_ALIGN!()); };
}

/* SYM_FUNC_START_WEAK_NOALIGN -- use for weak functions, w/o alignment */
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_WEAK_NOALIGN {
    ($name:ident) => { SYM_START!($name, SYM_L_WEAK, SYM_A_NONE); };
}

/* Expose `sym` to startup code by emitting an alias prefixed with `__pi_`. */
#[allow(unused_macros)]
macro_rules! SYM_PIC_ALIAS {
    ($sym:ident) => { SYM_ALIAS!(__pi_$sym, $sym, SYM_L_GLOBAL); };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
