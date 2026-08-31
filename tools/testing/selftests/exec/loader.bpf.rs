// SPDX-License-Identifier: GPL-2.0
/*
 * binfmt_misc_ops handler for the loader-substitution case: match the
 * marker the harness poked into the payload's e_ident padding and ask for
 * the selected interpreter to be substituted for the binary's PT_INTERP,
 * so the binary itself runs as a fully native exec.
 */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

const EI_CLASS: usize = 4;
const EI_PAD: usize = 9;
const ELFCLASS64: u8 = 2;

extern "C" {
    fn bpf_binprm_set_interp(
        bprm: *mut linux_binprm,
        path: *const u8,
        path__sz: usize,
    ) -> i32;
    fn bpf_binprm_set_flags(bprm: *mut linux_binprm, flags: bpf_binprm_flags) -> i32;
}

#[link_section = "struct_ops.s/match"]
#[no_mangle]
pub unsafe extern "C" fn loader_match(bprm: *mut linux_binprm) -> bool {
    if (*bprm).buf[0] != 0x7f
        || (*bprm).buf[1] != b'E'
        || (*bprm).buf[2] != b'L'
        || (*bprm).buf[3] != b'F'
        || (*bprm).buf[EI_CLASS] != ELFCLASS64
    {
        return false;
    }

    /* The harness marks the payload with "LDRTST" at EI_PAD. */
    (*bprm).buf[EI_PAD + 0] == b'L'
        && (*bprm).buf[EI_PAD + 1] == b'D'
        && (*bprm).buf[EI_PAD + 2] == b'R'
        && (*bprm).buf[EI_PAD + 3] == b'T'
        && (*bprm).buf[EI_PAD + 4] == b'S'
        && (*bprm).buf[EI_PAD + 5] == b'T'
}

#[link_section = "struct_ops.s/load"]
#[no_mangle]
pub unsafe extern "C" fn loader_load(bprm: *mut linux_binprm) -> i32 {
    let interp = *b"/tmp/binfmt_loader_interp\0";
    let mut err: i32;

    err = bpf_binprm_set_flags(bprm, BPF_BINPRM_LOADER);
    if err != 0 {
        return err;
    }

    /* @path__sz includes the terminating NUL; 0 commits the selection. */
    bpf_binprm_set_interp(bprm, interp.as_ptr(), core::mem::size_of_val(&interp))
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut loader: binfmt_misc_ops = binfmt_misc_ops {
    match_: loader_match as *mut core::ffi::c_void,
    load: loader_load as *mut core::ffi::c_void,
    name: b"loader\0".as_ptr(),
};
