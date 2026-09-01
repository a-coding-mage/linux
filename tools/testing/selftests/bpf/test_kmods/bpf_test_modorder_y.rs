// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <linux/btf.h>, <linux/module.h>, <linux/init.h>

unsafe extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
    static bpf_test_modorder_kfunc_y_ids: btf_id_set8;

    fn register_btf_kfunc_id_set(
        prog_type: bpf_prog_type,
        kset: *const btf_kfunc_id_set,
    ) -> core::ffi::c_int;
}

#[allow(non_camel_case_types)]
type bpf_prog_type = core::ffi::c_uint;

const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;

#[repr(C)]
struct btf_id_set8 {
    cnt: core::ffi::c_uint,
    flags: core::ffi::c_uint,
    pairs: [btf_id_set8_pair; 0],
}

#[repr(C)]
struct btf_id_set8_pair {
    id: core::ffi::c_uint,
    flags: core::ffi::c_uint,
}

#[repr(C)]
struct btf_kfunc_id_set {
    owner: *mut core::ffi::c_void,
    set: *const btf_id_set8,
}

// __bpf_kfunc_start_defs();

#[no_mangle]
pub unsafe extern "C" fn bpf_test_modorder_rety() -> core::ffi::c_int {
    'y' as core::ffi::c_int
}

// __bpf_kfunc_end_defs();

// BTF_KFUNCS_START(bpf_test_modorder_kfunc_y_ids)
// BTF_ID_FLAGS(func, bpf_test_modorder_rety);
// BTF_KFUNCS_END(bpf_test_modorder_kfunc_y_ids)

static bpf_test_modorder_y_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: unsafe { THIS_MODULE },
    set: unsafe { &bpf_test_modorder_kfunc_y_ids },
};

unsafe extern "C" fn bpf_test_modorder_y_init() -> core::ffi::c_int {
    unsafe {
        register_btf_kfunc_id_set(
            BPF_PROG_TYPE_SCHED_CLS,
            &bpf_test_modorder_y_set,
        )
    }
}

unsafe extern "C" fn bpf_test_modorder_y_exit() {}

// module_init(bpf_test_modorder_y_init);
// module_exit(bpf_test_modorder_y_exit);

// MODULE_DESCRIPTION("BPF selftest ordertest module Y");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
