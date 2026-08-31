/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

// C header dependency: <stdlib.h> for size_t.

#[repr(C)]
pub struct bpf_insn {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn disasm_insn(
        insn: *mut bpf_insn,
        buf: *mut ::std::os::raw::c_char,
        buf_sz: usize,
    ) -> *mut bpf_insn;
}
