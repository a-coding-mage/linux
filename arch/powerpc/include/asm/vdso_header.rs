/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_POWERPC_VDSO_H

// #define VDSO_VERSION_STRING LINUX_2.6.15
pub const VDSO_VERSION_STRING: &str = "LINUX_2.6.15";

pub const __VDSO_PAGES: usize = 4;

// The generated offset headers are supplied by the surrounding build.
// #ifdef CONFIG_PPC64
// #include <generated/vdso64-offsets.h>
// #endif
//
// #ifdef CONFIG_VDSO32
// #include <generated/vdso32-offsets.h>
// #endif

// #define VDSO64_SYMBOL(base, name) ((unsigned long)(base) + (vdso64_offset_##name))
#[macro_export]
macro_rules! VDSO64_SYMBOL {
    ($base:expr, $offset:expr) => {
        ($base as ::core::ffi::c_ulong).wrapping_add($offset as ::core::ffi::c_ulong)
    };
}

// #define VDSO32_SYMBOL(base, name) ((unsigned long)(base) + (vdso32_offset_##name))
#[macro_export]
macro_rules! VDSO32_SYMBOL {
    ($base:expr, $offset:expr) => {
        ($base as ::core::ffi::c_ulong).wrapping_add($offset as ::core::ffi::c_ulong)
    };
}

unsafe extern "C" {
    pub fn vdso_getcpu_init() -> ::core::ffi::c_int;
}

// Assembler-only declarations retained as conditional intent:
//
// #ifdef __VDSO64__
// #define V_FUNCTION_BEGIN(name) .globl name; .type name,@function; name:
// #define V_FUNCTION_END(name) .size name,.-name;
// #define V_LOCAL_FUNC(name) (name)
// #endif
//
// #ifdef __VDSO32__
// #define V_FUNCTION_BEGIN(name) .globl name; .type name,@function; name:
// #define V_FUNCTION_END(name) .size name,.-name;
// #define V_LOCAL_FUNC(name) (name)
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
