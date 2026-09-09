/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_JUMP_LABEL_H
// HAVE_JUMP_LABEL_BATCH
// Dependencies: asm/asm.h, asm/nops.h, linux/stringify.h, linux/types.h

// The following macros emit inline assembly and are also expanded on the Rust
// side.  Their assembly fragments are preserved as Rust macro expansions.
macro_rules! JUMP_TABLE_ENTRY {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection __jump_table,  \"aw\" \\n\\t",
            /* _ASM_ALIGN */ "\\n\\t",
            /* ANNOTATE_DATA_SPECIAL */ "\\n",
            ".long 1b - . \\n\\t",
            $label, " - . \\n\\t",
            /* _ASM_PTR */ " ", $key, " - . \\n\\t",
            ".popsection \\n\\t"
        )
    };
}

#[cfg(CONFIG_HAVE_JUMP_LABEL_HACK)]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            "1: jmp ", $label, " # objtool NOPs this \\n\\t",
            JUMP_TABLE_ENTRY!(concat!($key, " + 2"), $label)
        )
    };
}

#[cfg(not(CONFIG_HAVE_JUMP_LABEL_HACK))]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            "1: .byte ", /* __stringify(BYTES_NOP5) */ "BYTES_NOP5", "\\n\\t",
            JUMP_TABLE_ENTRY!($key, $label)
        )
    };
}

// The C implementations use asm goto. Rust has no direct equivalent for this
// compiler extension; preserve the control-flow contract and assembly intent.
#[inline(always)]
pub unsafe fn arch_static_branch(
    key: *const static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // asm goto(ARCH_STATIC_BRANCH_ASM("%c0 + %c1", "%l[l_yes]") ...)
    // TODO: provide the target/toolchain-specific asm-goto lowering.
    false
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(
    key: *const static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // asm goto("1:" "jmp %l[l_yes]\\n\\t"
    //     JUMP_TABLE_ENTRY("%c0 + %c1", "%l[l_yes]") ...)
    // TODO: provide the target/toolchain-specific asm-goto lowering.
    false
}

#[repr(C)]
pub struct static_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

extern "C" {
    pub fn arch_jump_entry_size(entry: *mut jump_entry) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
