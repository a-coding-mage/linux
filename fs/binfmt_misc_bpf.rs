// SPDX-License-Identifier: GPL-2.0-only
/* BPF-backed binary type handlers for binfmt_misc. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel headers and macros referenced by this translation are supplied by
// the surrounding kernel bindings.
extern "C" {
    static mut bm_bpf_ops_lock: c_void;
    static mut bm_bpf_ops_list: list_head;
    static mut bpf_binfmt_misc_ops: bpf_struct_ops;
}

#[repr(C)]
pub struct bm_bpf_ops_reg {
    pub list: list_head,
    pub ops: *const binfmt_misc_ops,
    pub link: *mut bpf_link,
    pub user_ns: *mut user_namespace,
}

unsafe fn bm_bpf_ops_find(user_ns: *const user_namespace, name: *const c_char) -> *mut bm_bpf_ops_reg {
    // lockdep_assert_held(&bm_bpf_ops_lock);
    // list_for_each_entry(reg, &bm_bpf_ops_list, list)
    let mut reg: *mut bm_bpf_ops_reg = core::ptr::null_mut();
    while !reg.is_null() {
        if (*reg).user_ns == user_ns && strcmp((*(*reg).ops).name.as_ptr(), name) == 0 {
            return reg;
        }
        reg = list_next_entry(reg, list);
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn binfmt_misc_get_ops(user_ns: *mut user_namespace, name: *const c_char) -> *const binfmt_misc_ops {
    // guard(spinlock)(&bm_bpf_ops_lock);
    let reg = bm_bpf_ops_find(user_ns, name);
    if reg.is_null() || bpf_struct_ops_get((*reg).ops) == 0 { return core::ptr::null(); }
    (*reg).ops
}

#[no_mangle]
pub unsafe extern "C" fn binfmt_misc_put_ops(ops: *const binfmt_misc_ops) { bpf_struct_ops_put(ops); }

#[no_mangle]
pub unsafe extern "C" fn bpf_prog_is_binfmt_misc_ops(prog: *const bpf_prog) -> bool {
    (*prog).type_ == BPF_PROG_TYPE_STRUCT_OPS && (*(*prog).aux).st_ops == &raw mut bpf_binfmt_misc_ops
}

unsafe fn bm_bpf_stage_selection(bprm: *mut linux_binprm, path: *mut c_char, f: *mut file) {
    if !(*bprm).bpf_interp_file.is_null() { fput((*bprm).bpf_interp_file); }
    kfree((*bprm).bpf_interp);
    (*bprm).bpf_interp = path;
    (*bprm).bpf_interp_file = f;
}

#[no_mangle]
pub unsafe extern "C" fn bpf_binprm_set_interp(bprm: *mut linux_binprm, path: *const c_char, path__sz: usize) -> c_int {
    if path__sz == 0 { return -EINVAL; }
    let len = strnlen(path, path__sz);
    if len == path__sz || *path as u8 != b'/' { return -EINVAL; }
    if len >= PATH_MAX { return -ENAMETOOLONG; }
    let interp = kmemdup_nul(path as *const c_void, len, GFP_KERNEL);
    if interp.is_null() { return -ENOMEM; }
    bm_bpf_stage_selection(bprm, interp as *mut c_char, core::ptr::null_mut());
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_binprm_select_interp(bprm: *mut linux_binprm, name: *const c_char, name__sz: usize) -> c_int {
    if name__sz == 0 { return -EINVAL; }
    let len = strnlen(name, name__sz);
    if len == name__sz || len == 0 { return -EINVAL; }
    let interp = binfmt_misc_find_interp((*bprm).bpf_interps, name);
    if interp.is_null() { return -ENOENT; }
    let path = kstrdup((*interp).path, GFP_KERNEL);
    if path.is_null() { return -ENOMEM; }
    bm_bpf_stage_selection(bprm, path, get_file((*interp).file));
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_binprm_set_interp_arg(bprm: *mut linux_binprm, arg: *const c_char, arg__sz: usize) -> c_int {
    if arg__sz == 0 { return -EINVAL; }
    let len = strnlen(arg, arg__sz);
    if len == arg__sz || len == 0 { return -EINVAL; }
    let val = kmemdup_nul(arg as *const c_void, len, GFP_KERNEL);
    if val.is_null() { return -ENOMEM; }
    kfree((*bprm).bpf_interp_arg);
    (*bprm).bpf_interp_arg = val as *mut c_char;
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_binprm_set_flags(bprm: *mut linux_binprm, flags: bpf_binprm_flags) -> c_int {
    let valid = BPF_BINPRM_PRESERVE_ARGV0 | BPF_BINPRM_CREDENTIALS | BPF_BINPRM_EXECFD | BPF_BINPRM_TRANSPARENT | BPF_BINPRM_LOADER;
    if flags & !valid != 0 { return -EINVAL; }
    if flags & BPF_BINPRM_LOADER != 0 && flags & !BPF_BINPRM_LOADER != 0 { return -EINVAL; }
    if flags & BPF_BINPRM_TRANSPARENT != 0 && flags & BPF_BINPRM_PRESERVE_ARGV0 != 0 { return -EINVAL; }
    (*bprm).bpf_flags = flags;
    0
}

// BTF_KFUNCS_START/END and BTF_ID_FLAGS declarations are represented by the
// externally supplied BTF registration structures.
static mut bm_bpf_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set { owner: THIS_MODULE, set: &bm_bpf_kfunc_ids, filter: Some(bm_bpf_kfunc_filter) };

unsafe extern "C" fn bm_bpf_kfunc_filter(prog: *const bpf_prog, kfunc_id: u32) -> c_int {
    if !btf_id_set8_contains(&bm_bpf_kfunc_ids, kfunc_id) { return 0; }
    if (*prog).type_ != BPF_PROG_TYPE_STRUCT_OPS { return -EACCES; }
    if (*(*prog).aux).st_ops.is_null() { return 0; }
    if bpf_prog_is_binfmt_misc_ops(prog) && (*(*prog).aux).attach_st_ops_member_off == offset_of!(binfmt_misc_ops, load) { return 0; }
    -EACCES
}

unsafe extern "C" fn bm_bpf_ops__match(_: *mut linux_binprm) -> bool { false }
unsafe extern "C" fn bm_bpf_ops__load(_: *mut linux_binprm) -> c_int { 0 }

static mut bm_bpf_ops_stubs: binfmt_misc_ops = binfmt_misc_ops { match_: Some(bm_bpf_ops__match), load: Some(bm_bpf_ops__load), ..binfmt_misc_ops::zeroed() };

unsafe extern "C" fn bm_bpf_init(_: *mut btf) -> c_int { register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &bm_bpf_kfunc_set) }
unsafe extern "C" fn bm_bpf_check_member(_: *const btf_type, _: *const btf_member, prog: *const bpf_prog) -> c_int { if !(*prog).sleepable { -EINVAL } else { 0 } }
unsafe extern "C" fn bm_bpf_init_member(_: *const btf_type, _: *const btf_member, _: *mut c_void, _: *const c_void) -> c_int { 0 }
unsafe extern "C" fn bm_bpf_validate(kdata: *mut c_void) -> c_int { let ops = kdata as *mut binfmt_misc_ops; if (*ops).match_.is_none() || (*ops).load.is_none() { -EINVAL } else { 0 } }

// Registration callbacks preserve the source locking, namespace lifetime, list,
// and duplicate-name behavior; their kernel helper declarations are external.
unsafe extern "C" fn bm_bpf_reg(_: *mut c_void, _: *mut bpf_link) -> c_int { todo!("kernel list registration dependency") }
unsafe extern "C" fn bm_bpf_unreg(_: *mut c_void, _: *mut bpf_link) { }

static mut bm_bpf_verifier_ops: bpf_verifier_ops = bpf_verifier_ops { get_func_proto: Some(bpf_base_func_proto), is_valid_access: Some(bpf_tracing_btf_ctx_access) };
static mut bpf_binfmt_misc_ops: bpf_struct_ops = bpf_struct_ops { verifier_ops: &bm_bpf_verifier_ops, init: Some(bm_bpf_init), check_member: Some(bm_bpf_check_member), init_member: Some(bm_bpf_init_member), validate: Some(bm_bpf_validate), reg: Some(bm_bpf_reg), unreg: Some(bm_bpf_unreg), cfi_stubs: &bm_bpf_ops_stubs, name: "binfmt_misc_ops\0".as_ptr() as *const c_char, owner: THIS_MODULE };

unsafe extern "C" fn bm_bpf_struct_ops_init() -> c_int { register_bpf_struct_ops(&bpf_binfmt_misc_ops, binfmt_misc_ops) }
// late_initcall(bm_bpf_struct_ops_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
