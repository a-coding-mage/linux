/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: symbols and macros supplied by <asm/text-patching.h> and the
// surrounding static-call implementation remain external to this header.

/*
 * For CONFIG_HAVE_STATIC_CALL_INLINE, this is a temporary trampoline which
 * uses the current value of the key->func pointer to do an indirect jump to
 * the function.  This trampoline is only used during boot, before the call
 * sites get patched by static_call_update().  The name of this trampoline has
 * a magical aspect: objtool uses it to find static call sites so it can create
 * the .static_call_sites section.
 *
 * For CONFIG_HAVE_STATIC_CALL, this is a permanent trampoline which
 * does a direct jump to the function.  The direct jump gets patched by
 * static_call_update().
 *
 * Having the trampoline in a special section forces GCC to emit a JMP.d32 when
 * it does tail-call optimization on the call; since you cannot compute the
 * relative displacement across sections.
 */

/*
 * The trampoline is 8 bytes and of the general form:
 *
 *   jmp.d32 \\func
 *   ud1 %esp, %ecx
 *
 * That trailing #UD provides both a speculation stop and serves as a unique
 * 3 byte signature identifying static call trampolines. Also see tramp_ud[]
 * and __static_call_fixup().
 */

// C inline-assembly trampoline definitions are retained as declarative Rust
// macros; the emitted assembly is supplied by the target integration.
macro_rules! __ARCH_DEFINE_STATIC_CALL_TRAMP {
    ($name:ident, $insns:expr) => {{
        // .static_call.text, alignment 4, global symbol, $insns, trailing
        // 0x0f 0xb9 0xcc, function type/size, and section restoration.
        let _ = stringify!($name);
        let _ = $insns;
    }};
}

macro_rules! ARCH_DEFINE_STATIC_CALL_TRAMP {
    ($name:ident, $func:ident) => {
        __ARCH_DEFINE_STATIC_CALL_TRAMP!($name, concat!(
            ".byte 0xe9; .long ", stringify!($func), " - (. + 4)"
        ));
    };
}

#[cfg(feature = "CONFIG_MITIGATION_RETHUNK")]
macro_rules! ARCH_DEFINE_STATIC_CALL_NULL_TRAMP {
    ($name:ident) => {
        __ARCH_DEFINE_STATIC_CALL_TRAMP!($name, "jmp __x86_return_thunk");
    };
}

#[cfg(not(feature = "CONFIG_MITIGATION_RETHUNK"))]
macro_rules! ARCH_DEFINE_STATIC_CALL_NULL_TRAMP {
    ($name:ident) => {
        __ARCH_DEFINE_STATIC_CALL_TRAMP!($name, "ret; int3; nop; nop; nop");
    };
}

macro_rules! ARCH_DEFINE_STATIC_CALL_RET0_TRAMP {
    ($name:ident) => {
        ARCH_DEFINE_STATIC_CALL_TRAMP!($name, __static_call_return0);
    };
}

macro_rules! ARCH_ADD_TRAMP_KEY {
    ($name:ident) => {{
        // Emit .long STATIC_CALL_TRAMP_STR($name) - . and
        // .long STATIC_CALL_KEY_STR($name) - . in .static_call_tramp_key.
        let _ = stringify!($name);
    }};
}

unsafe extern "C" {
    pub fn __static_call_fixup(tramp: *mut core::ffi::c_void, op: u8,
                               dest: *mut core::ffi::c_void) -> bool;
    pub fn __static_call_update_early(tramp: *mut core::ffi::c_void,
                                      func: *mut core::ffi::c_void);
}

macro_rules! static_call_update_early {
    ($name:ident, $func:expr) => {{
        // `typeof(&STATIC_CALL_TRAMP(name))` is preserved by the function
        // pointer cast; key/trampoline address macros are external.
        let __f = $func;
        unsafe {
            if static_call_initialized {
                __static_call_update(
                    &STATIC_CALL_KEY!($name),
                    STATIC_CALL_TRAMP_ADDR!($name),
                    __f,
                );
            } else {
                WRITE_ONCE!(STATIC_CALL_KEY!($name).func, $func);
                __static_call_update_early(STATIC_CALL_TRAMP_ADDR!($name), __f);
            }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
