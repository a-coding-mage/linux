// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <linux/btf.h>, <linux/module.h>, <linux/init.h>

// __bpf_kfunc_start_defs();

#[no_mangle]
pub unsafe extern "C" fn bpf_test_modorder_retx() -> ::core::ffi::c_int {
    b'x' as ::core::ffi::c_int
}

// __bpf_kfunc_end_defs();

// BTF_KFUNCS_START(bpf_test_modorder_kfunc_x_ids)
// BTF_ID_FLAGS(func, bpf_test_modorder_retx);
// BTF_KFUNCS_END(bpf_test_modorder_kfunc_x_ids)
extern "C" {
    static bpf_test_modorder_kfunc_x_ids: btf_id_set8;
    static mut THIS_MODULE: module;

    fn register_btf_kfunc_id_set(
        prog_type: bpf_prog_type,
        kset: *const btf_kfunc_id_set,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_id_set8 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut module,
    pub set: *const btf_id_set8,
}

pub type bpf_prog_type = ::core::ffi::c_uint;

pub const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;

static bpf_test_modorder_x_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: unsafe { &raw mut THIS_MODULE },
    set: unsafe { &raw const bpf_test_modorder_kfunc_x_ids },
};

unsafe extern "C" fn bpf_test_modorder_x_init() -> ::core::ffi::c_int {
    unsafe {
        register_btf_kfunc_id_set(
            BPF_PROG_TYPE_SCHED_CLS,
            &raw const bpf_test_modorder_x_set,
        )
    }
}

unsafe extern "C" fn bpf_test_modorder_x_exit() {}

// module_init(bpf_test_modorder_x_init);
// module_exit(bpf_test_modorder_x_exit);

// MODULE_DESCRIPTION("BPF selftest ordertest module X");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
