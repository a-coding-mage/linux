// SPDX-License-Identifier: GPL-2.0
// Dependency equivalent of <asm/lowcore.h>.
// Dependency equivalent of <linux/btf.h>.

// __bpf_kfunc_start_defs();

// Opaque type supplied by the lowcore dependency.
#[repr(C)]
pub struct lowcore {
    _private: [u8; 0],
}

extern "C" {
    fn get_lowcore() -> *mut lowcore;
}

// __bpf_kfunc
pub unsafe extern "C" fn bpf_get_lowcore() -> *mut lowcore {
    unsafe { get_lowcore() }
}

// __bpf_kfunc_end_defs();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
