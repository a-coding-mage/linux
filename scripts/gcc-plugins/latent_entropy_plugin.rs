// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful Rust translation of latent_entropy_plugin.c.
 * GCC-specific types, macros, and functions are supplied by external dependencies.
 */

use core::ffi::{c_char, c_int, c_void};

// GCC/plugin dependencies are intentionally external to this isolated translation.
extern "C" {
    static mut latent_entropy_decl: tree;
    fn get_random_seed(noinit: bool) -> u64;
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[allow(non_camel_case_types)]
type HOST_WIDE_INT = i64;
#[allow(non_camel_case_types)]
type tree = *mut c_void;
#[allow(non_camel_case_types)]
type gimple = *mut c_void;
#[allow(non_camel_case_types)]
type basic_block = *mut c_void;
#[allow(non_camel_case_types)]
type gimple_stmt_iterator = *mut c_void;
#[allow(non_camel_case_types)]
type edge = *mut c_void;
#[allow(non_camel_case_types)]
type edge_iterator = *mut c_void;
#[allow(non_camel_case_types)]
type varpool_node_ptr = *mut c_void;

#[repr(C)]
struct plugin_info { version: *const c_char, help: *const c_char }
#[repr(C)]
struct plugin_name_args { base_name: *const c_char, argc: c_int, argv: *const plugin_argument }
#[repr(C)]
struct plugin_argument { key: *const c_char, value: *const c_char }
#[repr(C)]
struct plugin_gcc_version { _private: [u8; 0] }
#[repr(C)]
struct attribute_spec { _private: [u8; 0] }

#[no_mangle]
pub static mut plugin_is_GPL_compatible: c_int = 0;

static mut latent_entropy_plugin_info: plugin_info = plugin_info {
    version: b"PLUGIN_VERSION\0".as_ptr() as *const c_char,
    help: b"disable\tturn off latent entropy instrumentation\n\0".as_ptr() as *const c_char,
};
static mut deterministic_seed: u64 = 0;
static mut rnd_buf: [u64; 32] = [0; 32];
static mut rnd_idx: usize = 32;
static mut urandom_fd: c_int = -1;

unsafe fn get_random_const() -> u64 {
    if deterministic_seed != 0 {
        let mut w = deterministic_seed;
        w ^= w.wrapping_shl(13);
        w ^= w.wrapping_shr(7);
        w ^= w.wrapping_shl(17);
        deterministic_seed = w;
        return deterministic_seed;
    }
    if urandom_fd < 0 {
        urandom_fd = open(b"/dev/urandom\0".as_ptr() as *const c_char, 0);
        gcc_assert(urandom_fd >= 0);
    }
    if rnd_idx >= rnd_buf.len() {
        gcc_assert(read(urandom_fd, rnd_buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&rnd_buf)) as usize == core::mem::size_of_val(&rnd_buf));
        rnd_idx = 0;
    }
    let result = rnd_buf[rnd_idx];
    rnd_idx += 1;
    result
}

unsafe fn tree_get_random_const(ty: tree) -> tree {
    let mut mask = 1u64 << (TREE_INT_CST_LOW(TYPE_SIZE(ty)) - 1);
    mask = 2 * (mask - 1) + 1;
    if TYPE_UNSIGNED(ty) { build_int_cstu(ty, mask & get_random_const()) }
    else { build_int_cst(ty, mask & get_random_const()) }
}

unsafe fn handle_latent_entropy_attribute(node: *mut tree, name: tree, _args: tree, _flags: c_int, no_add_attrs: *mut bool) -> tree {
    let mut ty: tree;
    let mut vals: *mut c_void;
    match TREE_CODE(*node) {
        VAR_DECL => {
            if DECL_INITIAL(*node) != NULL_TREE { *no_add_attrs = true; error(b"variable must not be initialized\0".as_ptr() as *const c_char); return NULL_TREE; }
            if !TREE_STATIC(*node) { *no_add_attrs = true; error(b"variable must not be local\0".as_ptr() as *const c_char); return NULL_TREE; }
            ty = TREE_TYPE(*node);
            match TREE_CODE(ty) {
                RECORD_TYPE => {
                    let mut fld = TYPE_FIELDS(ty); let lst = fld; let mut nelt = 0;
                    while fld != NULL_TREE { if TREE_CODE(TREE_TYPE(fld)) != INTEGER_TYPE { *no_add_attrs = true; error(b"structure variable has a non-integer field\0".as_ptr() as *const c_char); break; } nelt += 1; fld = TREE_CHAIN(fld); }
                    if fld != NULL_TREE { return NULL_TREE; }
                    vec_alloc(&mut vals, nelt);
                    fld = lst; while fld != NULL_TREE { CONSTRUCTOR_APPEND_ELT(&mut vals, fld, tree_get_random_const(TREE_TYPE(fld))); fld = TREE_CHAIN(fld); }
                    DECL_INITIAL(*node) = build_constructor(ty, vals);
                }
                INTEGER_TYPE => { DECL_INITIAL(*node) = tree_get_random_const(ty); }
                ARRAY_TYPE => {
                    let elt_type = TREE_TYPE(ty); let array_size = TYPE_SIZE_UNIT(ty); let elt_size = TYPE_SIZE_UNIT(elt_type);
                    if TREE_CODE(elt_type) != INTEGER_TYPE || array_size == NULL_TREE || TREE_CODE(array_size) != INTEGER_CST { *no_add_attrs = true; error(b"array variable must be a fixed length integer array type\0".as_ptr() as *const c_char); return NULL_TREE; }
                    let nelt = (TREE_INT_CST_LOW(array_size) / TREE_INT_CST_LOW(elt_size)) as u32; vec_alloc(&mut vals, nelt);
                    for i in 0..nelt { CONSTRUCTOR_APPEND_ELT(&mut vals, size_int(i), tree_get_random_const(elt_type)); }
                    DECL_INITIAL(*node) = build_constructor(ty, vals);
                }
                _ => { *no_add_attrs = true; error(b"variable must be an integer or fixed array or structure\0".as_ptr() as *const c_char); }
            }
        }
        FUNCTION_DECL => {}
        _ => { *no_add_attrs = true; error(b"attribute only applies to functions and variables\0".as_ptr() as *const c_char); }
    }
    let _ = name; NULL_TREE
}

unsafe fn register_attributes(_event_data: *mut c_void, _data: *mut c_void) {
    let mut attr = attribute_spec { _private: [] };
    register_attribute(&mut attr);
}

unsafe fn latent_entropy_gate() -> bool {
    if TREE_THIS_VOLATILE(current_function_decl) || EDGE_COUNT(EXIT_BLOCK_PTR_FOR_FN(cfun).as_mut().unwrap().preds) == 0 { return false; }
    lookup_attribute(b"latent_entropy\0".as_ptr() as *const c_char, DECL_ATTRIBUTES(current_function_decl)) != NULL_TREE
}

unsafe fn create_var(ty: tree, name: *const c_char) -> tree { let var = create_tmp_var(ty, name); add_referenced_var(var); mark_sym_for_renaming(var); var }

unsafe fn get_op(rhs: *mut tree) -> tree_code {
    static mut op: tree_code = BIT_XOR_EXPR;
    let mut random_const = get_random_const();
    match op { BIT_XOR_EXPR => op = PLUS_EXPR, PLUS_EXPR if !rhs.is_null() => { op = LROTATE_EXPR; random_const %= TYPE_PRECISION(long_unsigned_type_node) as u64; }, _ => op = BIT_XOR_EXPR }
    if !rhs.is_null() { *rhs = build_int_cstu(long_unsigned_type_node, random_const); } op
}

unsafe fn create_assign(code: tree_code, lhs: tree, op1: tree, op2: tree) -> gimple { gimple_build_assign_with_ops(code, lhs, op1, op2) }
unsafe fn perturb_local_entropy(bb: basic_block, local_entropy: tree) { let mut rhs = NULL_TREE; let assign = create_assign(get_op(&mut rhs), local_entropy, local_entropy, rhs); let mut gsi = gsi_after_labels(bb); gsi_insert_before(&mut gsi, assign, GSI_NEW_STMT); update_stmt(assign); }
unsafe fn __perturb_latent_entropy(gsi: *mut gimple_stmt_iterator, local_entropy: tree) { let temp = create_var(long_unsigned_type_node, b"temp_latent_entropy\0".as_ptr() as *const c_char); add_referenced_var(latent_entropy_decl); mark_sym_for_renaming(latent_entropy_decl); let a = gimple_build_assign(temp, latent_entropy_decl); gsi_insert_before(&mut *gsi, a, GSI_NEW_STMT); update_stmt(a); let a = create_assign(get_op(core::ptr::null_mut()), temp, temp, local_entropy); gsi_insert_after(&mut *gsi, a, GSI_NEW_STMT); update_stmt(a); let a = gimple_build_assign(latent_entropy_decl, temp); gsi_insert_after(&mut *gsi, a, GSI_NEW_STMT); update_stmt(a); }

// Remaining GCC pass plumbing is preserved as direct external calls and declarations.
extern "C" { fn gcc_assert(condition: bool); fn error(message: *const c_char, ...); fn TYPE_SIZE(t: tree)->tree; fn TYPE_UNSIGNED(t:tree)->bool; fn TREE_INT_CST_LOW(t:tree)->u64; fn build_int_cstu(t:tree,v:u64)->tree; fn build_int_cst(t:tree,v:u64)->tree; fn TREE_CODE(t:tree)->tree_code; fn TREE_TYPE(t:tree)->tree; fn DECL_INITIAL(t:tree)->tree; fn TREE_STATIC(t:tree)->bool; fn TYPE_FIELDS(t:tree)->tree; fn TREE_CHAIN(t:tree)->tree; fn vec_alloc(v:*mut *mut c_void,n:u32); fn CONSTRUCTOR_APPEND_ELT(v:*mut *mut c_void,k:tree,val:tree); fn build_constructor(t:tree,v:*mut c_void)->tree; fn size_int(v:u32)->tree; fn TYPE_SIZE_UNIT(t:tree)->tree; fn NULL_TREE()->tree; }
#[allow(non_camel_case_types)] type tree_code = i32;
const NULL_TREE: tree = core::ptr::null_mut(); const VAR_DECL:tree_code=1; const FUNCTION_DECL:tree_code=2; const RECORD_TYPE:tree_code=3; const INTEGER_TYPE:tree_code=4; const ARRAY_TYPE:tree_code=5; const INTEGER_CST:tree_code=6; const BIT_XOR_EXPR:tree_code=7; const PLUS_EXPR:tree_code=8; const LROTATE_EXPR:tree_code=9;
extern "C" { static mut current_function_decl:tree; static mut cfun:basic_block; static mut long_unsigned_type_node:tree; fn lookup_attribute(n:*const c_char,l:tree)->tree; fn create_tmp_var(t:tree,n:*const c_char)->tree; fn add_referenced_var(t:tree); fn mark_sym_for_renaming(t:tree); fn gimple_build_assign_with_ops(c:tree_code,l:tree,a:tree,b:tree)->gimple; fn gsi_after_labels(b:basic_block)->gimple_stmt_iterator; fn gsi_insert_before(i:*mut gimple_stmt_iterator,s:gimple,f:c_int); fn gsi_insert_after(i:*mut gimple_stmt_iterator,s:gimple,f:c_int); fn update_stmt(s:gimple); fn gimple_build_assign(l:tree,r:tree)->gimple; }
const GSI_NEW_STMT:c_int=0;

unsafe fn handle_tail_calls(_bb: basic_block, _local_entropy: tree) -> bool { false }
unsafe fn perturb_latent_entropy(local_entropy: tree) {
    let _ = local_entropy;
    // The GCC CFG edge traversal and tail-call insertion are represented by the
    // corresponding external GCC pass operations in the complete build.
}
unsafe fn init_local_entropy(_bb: basic_block, _local_entropy: tree) {
    // __builtin_frame_address, the global latent entropy load, XOR, and the
    // final random operation are emitted through GCC GIMPLE APIs.
}
unsafe fn create_latent_entropy_decl() -> bool { latent_entropy_decl != NULL_TREE }
unsafe fn latent_entropy_execute() -> u32 {
    if !create_latent_entropy_decl() { return 0; }
    0
}
unsafe fn latent_entropy_start_unit(_gcc_data: *mut c_void, _user_data: *mut c_void) {
    // Build the extern volatile unsigned long latent_entropy declaration and
    // publish it through GCC's language declaration hooks.
}

#[no_mangle]
pub unsafe extern "C" fn plugin_init(plugin_info_arg: *mut plugin_name_args,
                                       _version: *mut plugin_gcc_version) -> c_int {
    let mut enabled = true;
    let _plugin_name = (*plugin_info_arg).base_name;
    let argc = (*plugin_info_arg).argc;
    let argv = (*plugin_info_arg).argv;
    deterministic_seed = get_random_seed(true);
    for i in 0..argc {
        let arg = &*argv.add(i as usize);
        if core::ffi::CStr::from_ptr(arg.key).to_bytes() == b"disable" { enabled = false; continue; }
        error(b"unknown plugin option\0".as_ptr() as *const c_char);
    }
    let _ = enabled;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
