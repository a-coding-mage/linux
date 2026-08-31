// SPDX-License-Identifier: GPL-2.0
/*
 * binfmt_misc_ops handler for the selftest's fixed-interpreter case: match a
 * 64-bit aarch64 ELF header from the prefetched buffer and route it to a fixed
 * interpreter chosen by the program. This is the portable, self-contained
 * equivalent of routing a foreign binary to an emulator: it matches
 * programmatically and computes the interpreter, but points at a test binary
 * the harness installs rather than a system emulator.
 */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const EI_CLASS: usize = 4;
pub const ELFCLASS64: u8 = 2;
pub const EM_AARCH64: u16 = 183;

unsafe extern "C" {
    pub type linux_binprm;
    pub type binfmt_misc_ops;

    pub fn bpf_binprm_set_interp(
        bprm: *mut linux_binprm,
        path: *const core::ffi::c_char,
        path__sz: usize,
    ) -> core::ffi::c_int;
}

/*
 * A magic-style decision needs nothing beyond the prefetched bprm->buf,
 * even though the match program could read the file.
 */
// SEC("struct_ops.s/match")
// bool BPF_PROG(bpf_interp_match, struct linux_binprm *bprm)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_interp_match(bprm: *mut linux_binprm) -> bool {
    let machine: u16;

    if (*bprm).buf[0] != 0x7f
        || (*bprm).buf[1] != b'E' as _
        || (*bprm).buf[2] != b'L' as _
        || (*bprm).buf[3] != b'F' as _
        || (*bprm).buf[EI_CLASS] != ELFCLASS64 as _
    {
        return false;
    }

    /* e_machine is a 16-bit little-endian field at offset 18. */
    machine = ((*bprm).buf[18] as u8 as u16) | (((*bprm).buf[19] as u8 as u16) << 8);
    machine == EM_AARCH64
}

// SEC("struct_ops.s/load")
// int BPF_PROG(bpf_interp_load, struct linux_binprm *bprm)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_interp_load(bprm: *mut linux_binprm) -> core::ffi::c_int {
    /*
     * Keep the path on the (writable) stack: bpf_binprm_set_interp() takes
     * a sized memory arg and the verifier rejects a read-only .rodata
     * buffer for it. The harness installs the interpreter at this path.
     */
    let mut interp: [core::ffi::c_char; 24] = *b"/tmp/binfmt_bpf_interp\0";

    /* @path__sz includes the terminating NUL; 0 commits the selection. */
    bpf_binprm_set_interp(bprm, interp.as_mut_ptr(), core::mem::size_of_val(&interp))
}

// SEC(".struct_ops.link")
// struct binfmt_misc_ops bpf_interp = {
//     .match = (void *)bpf_interp_match,
//     .load = (void *)bpf_interp_load,
//     .name = "bpf_interp",
// };
#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut bpf_interp: binfmt_misc_ops = binfmt_misc_ops {
    match_: bpf_interp_match as *mut core::ffi::c_void,
    load: bpf_interp_load as *mut core::ffi::c_void,
    name: b"bpf_interp\0".as_ptr() as *const core::ffi::c_char,
};
