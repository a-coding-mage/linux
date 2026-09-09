// SPDX-License-Identifier: GPL-2.0

// Declarations supplied by the corresponding Linux headers are represented
// here only as needed by this translation.

#[repr(C)]
pub struct pt_regs {
    pub ip: usize,
}

/// Assembly implementation of a function which immediately returns.
///
/// Original C declaration:
/// `asmlinkage void just_return_func(void);`
///
/// The original definition is emitted through architecture-specific assembly
/// macros (`ASM_FUNC_ALIGN`, `ANNOTATE_NOENDBR`, and `ASM_RET`) and is supplied
/// by the target build environment.
extern "C" {
    pub fn just_return_func();
}

#[no_mangle]
pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    (*regs).ip = just_return_func as usize;
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
