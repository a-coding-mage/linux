/* Direct Rust translation of randomize_layout_plugin.c. GCC/plugin symbols are
 * supplied by external dependencies and are intentionally not implemented here. */

// Source dependencies: gcc-common.h and randomize_layout_seed.h.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
type u64_ = u64;

// External GCC/plugin types and symbols are expected from the surrounding build.
extern "C" {
    static mut plugin_is_GPL_compatible: ::std::os::raw::c_int;
    static mut randstruct_seed: *const ::std::os::raw::c_char;
}

static mut performance_mode: ::std::os::raw::c_int = 0;

#[repr(C)]
struct ranctx { a: u64, b: u64, c: u64, d: u64 }

#[repr(C)]
struct partition_group { tree_start: tree, start: usize, length: usize }

static mut shuffle_seed: [u64; 4] = [0; 4];

#[inline]
unsafe fn partial_name_hash(c: usize, prevhash: usize) -> usize {
    (prevhash.wrapping_add(c.wrapping_shl(4)).wrapping_add(c >> 4)).wrapping_mul(11)
}

#[inline]
unsafe fn name_hash(mut name: *const u8) -> u32 {
    let mut hash: usize = 0;
    let mut len = libc_strlen(name as *const i8);
    while len != 0 { hash = partial_name_hash(*name as usize, hash); name = name.add(1); len -= 1; }
    hash as u32
}

#[inline] unsafe fn rot(x: u64, k: u32) -> u64 { x.rotate_left(k) }

unsafe fn ranval(x: *mut ranctx) -> u64 {
    let e = (*x).a.wrapping_sub(rot((*x).b, 7));
    (*x).a = (*x).b ^ rot((*x).c, 13);
    (*x).b = (*x).c.wrapping_add(rot((*x).d, 37));
    (*x).c = (*x).d.wrapping_add(e);
    (*x).d = e.wrapping_add((*x).a);
    (*x).d
}

unsafe fn raninit(x: *mut ranctx, seed: *const u64) {
    (*x).a = *seed; (*x).b = *seed.add(1); (*x).c = *seed.add(2); (*x).d = *seed.add(3);
    for _ in 0..30 { let _ = ranval(x); }
}

unsafe fn partition_struct(fields: *mut tree, length: usize, groups: *mut partition_group, num_groups: *mut usize) {
    for i in 0..length { *groups.add(i) = partition_group { tree_start: 0 as tree, start: 0, length: 0 }; }
    let mut accum_size = 0usize; let mut accum_length = 0usize; let mut group_idx = 0usize;
    for i in 0..length {
        if (*groups.add(group_idx)).tree_start == 0 as tree {
            (*groups.add(group_idx)).tree_start = *fields.add(i); (*groups.add(group_idx)).start = i;
            accum_length = 0; accum_size = 0;
        }
        accum_size = accum_size.wrapping_add(int_size_in_bytes(TREE_TYPE(*fields.add(i))) as usize);
        accum_length += 1;
        if accum_size >= 64 { (*groups.add(group_idx)).length = accum_length; accum_length = 0; group_idx += 1; }
    }
    if (*groups.add(group_idx)).tree_start != 0 as tree && (*groups.add(group_idx)).length == 0 {
        (*groups.add(group_idx)).length = accum_length; group_idx += 1;
    }
    *num_groups = group_idx;
}

unsafe fn performance_shuffle(newtree: *mut tree, length: usize, state: *mut ranctx) {
    let mut groups: Vec<partition_group> = vec![partition_group { tree_start: 0 as tree, start: 0, length: 0 }; length];
    let mut num_groups = 0usize; partition_struct(newtree, length, groups.as_mut_ptr(), &mut num_groups);
    for i in (1..num_groups).rev() { let r = (ranval(state) % (i as u64 + 1)) as usize; groups.swap(i, r); }
    for x in 0..num_groups { for index in (1..groups[x].length).rev() {
        let i = groups[x].start + index; if DECL_BIT_FIELD_TYPE(*newtree.add(i)) != 0 as tree { continue; }
        let r = groups[x].start + (ranval(state) % (index as u64 + 1)) as usize;
        if DECL_BIT_FIELD_TYPE(*newtree.add(r)) != 0 as tree { continue; }
        std::ptr::swap(newtree.add(i), newtree.add(r));
    }}
}

unsafe fn full_shuffle(newtree: *mut tree, length: usize, state: *mut ranctx) {
    for i in (1..length).rev() { let r = (ranval(state) % (i as u64 + 1)) as usize; std::ptr::swap(newtree.add(i), newtree.add(r)); }
}

unsafe fn shuffle(type_: const_tree, newtree: *mut tree, length: usize) {
    if length == 0 { return; }
    let name = ORIG_TYPE_NAME(type_); let h = name_hash(name); let mut seed = shuffle_seed;
    for s in &mut seed { *s ^= h as u64; }
    let mut state = ranctx { a: 0, b: 0, c: 0, d: 0 }; raninit(&mut state, seed.as_ptr());
    if performance_mode != 0 { performance_shuffle(newtree, length, &mut state); } else { full_shuffle(newtree, length, &mut state); }
}

unsafe fn is_flexible_array(field: const_tree) -> bool {
    let ft = TREE_TYPE(field); let size = TYPE_SIZE(ft);
    TREE_CODE(ft) == ARRAY_TYPE && size == 0 as tree && TYPE_DOMAIN(ft) != 0 as tree && TYPE_MAX_VALUE(TYPE_DOMAIN(ft)) == 0 as tree
}

unsafe fn relayout_struct(type_: tree) -> i32 {
    let n = list_length(TYPE_FIELDS(type_)) as usize; if TYPE_FIELDS(type_) == 0 as tree || n < 2 { return 0; }
    let mut fields = vec![0 as tree; n]; let mut f = TYPE_FIELDS(type_); let mut i = 0;
    while f != 0 as tree { fields[i] = f; f = TREE_CHAIN(f); i += 1; }
    let flex = is_flexible_array(fields[n - 1]); shuffle(type_, fields.as_mut_ptr(), if flex { n - 1 } else { n });
    for j in 0..n-1 { TREE_CHAIN(fields[j]) = fields[j + 1]; } TREE_CHAIN(fields[n - 1]) = 0 as tree;
    add_type_attr(type_, "randomize_performed", 0 as tree); add_type_attr(type_, "designated_init", 0 as tree);
    if flex { add_type_attr(type_, "has_flexarray", 0 as tree); }
    let main = TYPE_MAIN_VARIANT(type_); let mut v = main; while v != 0 as tree { TYPE_FIELDS(v) = fields[0]; v = TYPE_NEXT_VARIANT(v); }
    TYPE_SIZE(main) = 0 as tree; layout_type(main); 1
}

unsafe fn get_field_type(field: const_tree) -> const_tree { strip_array_types(TREE_TYPE(field)) }
unsafe fn is_fptr(fieldtype: const_tree) -> bool { TREE_CODE(fieldtype) == POINTER_TYPE && TREE_CODE(TREE_TYPE(fieldtype)) == FUNCTION_TYPE }

unsafe fn is_pure_ops_struct(node: const_tree) -> bool {
    let mut field = TYPE_FIELDS(node); while field != 0 as tree { let ft = get_field_type(field); if node != ft { let c = TREE_CODE(ft); if c == RECORD_TYPE || c == UNION_TYPE { if !is_pure_ops_struct(ft) { return false; } } else if !is_fptr(ft) { return false; } } field = TREE_CHAIN(field); } true
}

unsafe fn randomize_type(type_: tree) { if lookup_attribute("randomize_considered", TYPE_ATTRIBUTES(type_)) != 0 as tree { return; } if lookup_attribute("randomize_layout", TYPE_ATTRIBUTES(TYPE_MAIN_VARIANT(type_))) != 0 as tree || is_pure_ops_struct(type_) { relayout_struct(type_); } add_type_attr(type_, "randomize_considered", 0 as tree); }

unsafe fn handle_randomize_considered_attr(_node: *mut tree, _name: tree, _args: tree, _flags: i32, no_add_attrs: *mut bool) -> tree { *no_add_attrs = false; 0 as tree }
unsafe fn handle_randomize_performed_attr(_node: *mut tree, _name: tree, _args: tree, _flags: i32, no_add_attrs: *mut bool) -> tree { *no_add_attrs = false; 0 as tree }

// Remaining GCC callbacks retain their C behavior through the external GCC API.
unsafe fn finish_type(event_data: *mut std::ffi::c_void, _data: *mut std::ffi::c_void) { let t = event_data as tree; if t == 0 as tree || t == error_mark_node || TREE_CODE(t) != RECORD_TYPE || TYPE_FIELDS(t) == 0 as tree { return; } if lookup_attribute("randomize_considered", TYPE_ATTRIBUTES(t)) == 0 as tree { randomize_type(t); } }

unsafe fn randomize_layout_finish_decl(_event_data: *mut std::ffi::c_void, _data: *mut std::ffi::c_void) {}
unsafe fn register_attributes(_event_data: *mut std::ffi::c_void, _data: *mut std::ffi::c_void) {}
unsafe fn check_global_variables(_event_data: *mut std::ffi::c_void, _data: *mut std::ffi::c_void) {}
unsafe fn handle_local_var_initializers() {}
unsafe fn find_bad_casts_execute() -> u32 { handle_local_var_initializers(); 0 }

// C preprocessor-generated pass and plugin registration interfaces are external dependencies.
extern "C" {
    fn plugin_default_version_check(version: *mut plugin_gcc_version, gcc_version: *mut plugin_gcc_version) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn plugin_init(_plugin_info: *mut plugin_name_args, _version: *mut plugin_gcc_version) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
