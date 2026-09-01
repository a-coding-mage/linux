// SPDX-License-Identifier: GPL-2.0
/*
 * binfmt_misc_ops handler for the selftest's bound-interpreter case: one
 * handler, one entry, an interpreter per guest architecture - each bound to
 * a file when the entry was registered rather than to a path resolved at
 * exec time. The load program names the one it wants; a name the entry did
 * not bind fails the exec, which the harness checks too.
 */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

pub const EI_CLASS: usize = 4;
pub const ELFCLASS64: u8 = 2;
pub const E_MACHINE_OFF: usize = 18;
pub const EM_ARM: u16 = 40;
pub const EM_AARCH64: u16 = 183;
pub const EM_RISCV: u16 = 243;

unsafe extern "C" {
    pub fn bpf_binprm_select_interp(
        bprm: *mut linux_binprm,
        name: *const ::core::ffi::c_char,
        name__sz: usize,
    ) -> ::core::ffi::c_int;
}

/* The guest architecture of a 64-bit ELF, or zero if it is not one. */
unsafe fn elf_machine(bprm: *mut linux_binprm) -> u16 {
    if (*bprm).buf[0] != 0x7f
        || (*bprm).buf[1] != b'E' as _
        || (*bprm).buf[2] != b'L' as _
        || (*bprm).buf[3] != b'F' as _
        || (*bprm).buf[EI_CLASS] != ELFCLASS64 as _
    {
        return 0;
    }

    /* Little-endian 16-bit field, read byte-wise for the verifier. */
    ((*bprm).buf[E_MACHINE_OFF] as u8 as u16)
        | (((*bprm).buf[E_MACHINE_OFF + 1] as u8 as u16) << 8)
}

#[unsafe(link_section = "struct_ops.s/match")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interp_bind_match(bprm: *mut linux_binprm) -> bool {
    let machine: u16 = elf_machine(bprm);

    machine == EM_AARCH64 || machine == EM_RISCV || machine == EM_ARM
}

#[unsafe(link_section = "struct_ops.s/load")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interp_bind_load(bprm: *mut linux_binprm) -> ::core::ffi::c_int {
    /*
     * Names, not paths: each one selects a file the entry pre-opened, so
     * nothing is resolved here or later, in any namespace. The buffers
     * are on the stack because the verifier rejects .rodata for a sized
     * memory argument.
     */
    let first: [::core::ffi::c_char; 6] = [
        b'f' as ::core::ffi::c_char,
        b'i' as ::core::ffi::c_char,
        b'r' as ::core::ffi::c_char,
        b's' as ::core::ffi::c_char,
        b't' as ::core::ffi::c_char,
        0,
    ];
    let second: [::core::ffi::c_char; 7] = [
        b's' as ::core::ffi::c_char,
        b'e' as ::core::ffi::c_char,
        b'c' as ::core::ffi::c_char,
        b'o' as ::core::ffi::c_char,
        b'n' as ::core::ffi::c_char,
        b'd' as ::core::ffi::c_char,
        0,
    ];
    let unbound: [::core::ffi::c_char; 8] = [
        b'u' as ::core::ffi::c_char,
        b'n' as ::core::ffi::c_char,
        b'b' as ::core::ffi::c_char,
        b'o' as ::core::ffi::c_char,
        b'u' as ::core::ffi::c_char,
        b'n' as ::core::ffi::c_char,
        b'd' as ::core::ffi::c_char,
        0,
    ];

    match elf_machine(bprm) {
        EM_AARCH64 => bpf_binprm_select_interp(bprm, first.as_ptr(), ::core::mem::size_of_val(&first)),
        EM_RISCV => bpf_binprm_select_interp(bprm, second.as_ptr(), ::core::mem::size_of_val(&second)),
        _ => {
            /* The entry bound nothing under this name: -ENOENT fails the exec. */
            bpf_binprm_select_interp(bprm, unbound.as_ptr(), ::core::mem::size_of_val(&unbound))
        }
    }
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut interp_bind: binfmt_misc_ops = binfmt_misc_ops {
    match_: interp_bind_match as *mut ::core::ffi::c_void,
    load: interp_bind_load as *mut ::core::ffi::c_void,
    name: c"interp_bind".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
