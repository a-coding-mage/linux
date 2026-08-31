/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */

/* Dependencies from the original header:
 * #include <stdio.h>
 * #include <dis-asm.h>
 */

use core::ffi::{c_char, c_int, c_void, VaListImpl};

/*
 * define types for older binutils version, to centralize ifdef'ery a bit
 *
 * Original condition: #ifndef DISASM_INIT_STYLED
 */
#[cfg(not(DISASM_INIT_STYLED))]
#[repr(C)]
pub enum disassembler_style {
    DISASSEMBLER_STYLE_NOT_EMPTY,
}

#[cfg(not(DISASM_INIT_STYLED))]
pub type fprintf_styled_ftype = Option<
    unsafe extern "C" fn(
        *mut c_void,
        disassembler_style,
        *const c_char,
        ...
    ) -> c_int,
>;

extern "C" {
    pub type disassemble_info;

    pub fn vfprintf(
        stream: *mut c_void,
        format: *const c_char,
        arg: VaListImpl,
    ) -> c_int;

    #[cfg(DISASM_INIT_STYLED)]
    pub fn init_disassemble_info(
        info: *mut disassemble_info,
        stream: *mut c_void,
        unstyled_func: fprintf_ftype,
        styled_func: fprintf_styled_ftype,
    );

    #[cfg(not(DISASM_INIT_STYLED))]
    pub fn init_disassemble_info(
        info: *mut disassemble_info,
        stream: *mut c_void,
        unstyled_func: fprintf_ftype,
    );
}

pub type fprintf_ftype = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int>;

/*
 * Trivial fprintf wrapper to be used as the fprintf_styled_func argument to
 * init_disassemble_info_compat() when normal fprintf suffices.
 */
pub unsafe extern "C" fn fprintf_styled(
    out: *mut c_void,
    style: disassembler_style,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let r: c_int;

    let _ = style;

    args.as_va_list();
    r = vfprintf(out, fmt, args.as_va_list());

    r
}

/*
 * Wrapper for init_disassemble_info() that hides version
 * differences. Depending on binutils version and architecture either
 * fprintf_func or fprintf_styled_func will be called.
 */
#[cfg(DISASM_INIT_STYLED)]
pub unsafe fn init_disassemble_info_compat(
    info: *mut disassemble_info,
    stream: *mut c_void,
    unstyled_func: fprintf_ftype,
    styled_func: fprintf_styled_ftype,
) {
    init_disassemble_info(info, stream, unstyled_func, styled_func);
}

#[cfg(not(DISASM_INIT_STYLED))]
pub unsafe fn init_disassemble_info_compat(
    info: *mut disassemble_info,
    stream: *mut c_void,
    unstyled_func: fprintf_ftype,
    styled_func: fprintf_styled_ftype,
) {
    let _ = styled_func;
    init_disassemble_info(info, stream, unstyled_func);
}
