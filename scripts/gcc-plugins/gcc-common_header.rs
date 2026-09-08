/* SPDX-License-Identifier: GPL-2.0 */

// GCC headers included by the original file are external Rust dependencies.

extern "C" {
    pub fn debug_dominance_info(dir: cdi_direction);
    pub fn debug_dominance_tree(dir: cdi_direction, root: basic_block);
}

pub type cgraph_node_ptr = *mut cgraph_node;
pub type cgraph_edge_p = *mut cgraph_edge;
pub type varpool_node_ptr = *mut varpool_node;
pub type gimple_ptr = *mut gimple;
pub type const_gimple_ptr = *const gimple;

#[macro_export] macro_rules! debug_cgraph_node { ($node:expr) => { (*$node).debug() }; }
#[macro_export] macro_rules! cgraph_get_node { ($decl:expr) => { cgraph_node::get($decl) }; }
#[macro_export] macro_rules! cgraph_get_create_node { ($decl:expr) => { cgraph_node::get_create($decl) }; }
#[macro_export] macro_rules! cgraph_create_node { ($decl:expr) => { cgraph_node::create($decl) }; }
#[macro_export] macro_rules! cgraph_create_edge { ($caller:expr,$callee:expr,$stmt:expr,$count:expr,$freq:expr) => { (*$caller).create_edge($callee,$stmt,$count) }; }
#[macro_export] macro_rules! cgraph_create_edge_including_clones { ($caller:expr,$callee:expr,$old:expr,$stmt:expr,$count:expr,$freq:expr,$reason:expr) => { (*$caller).create_edge_including_clones($callee,$old,$stmt,$count,$reason) }; }
#[macro_export] macro_rules! ipa_ref_list_referring_iterate { ($l:expr,$i:expr,$p:expr) => { (*$l).referring.iterate($i,&mut $p) }; }
#[macro_export] macro_rules! ipa_ref_list_reference_iterate { ($l:expr,$i:expr,$p:expr) => { (*$l).reference.iterate($i,&mut $p) }; }

#[allow(non_camel_case_types)]
pub type __unused = (); // __attribute__((__unused__))
#[allow(non_camel_case_types)]
pub type __visible = (); // __attribute__((visibility("default")))

#[inline]
pub unsafe fn build_const_char_string(len: i32, str_: *const i8) -> tree {
    let cstr = build_string(len, str_);
    let elem = build_type_variant(char_type_node, 1, 0);
    let index = build_index_type(size_int(len.wrapping_sub(1)));
    let typ = build_array_type(elem, index);
    TREE_TYPE(cstr) = typ;
    TREE_CONSTANT(cstr) = 1;
    TREE_READONLY(cstr) = 1;
    TREE_STATIC(cstr) = 1;
    cstr
}

#[inline]
pub unsafe fn __add_type_attr(typ: tree, attr: *const i8, args: tree) {
    if typ == NULL_TREE { return; }
    let oldattr = lookup_attribute(attr, TYPE_ATTRIBUTES(typ));
    if oldattr != NULL_TREE {
        gcc_assert(TREE_VALUE(oldattr) == args || TREE_VALUE(TREE_VALUE(oldattr)) == TREE_VALUE(args));
        return;
    }
    TYPE_ATTRIBUTES(typ) = copy_list(TYPE_ATTRIBUTES(typ));
    TYPE_ATTRIBUTES(typ) = tree_cons(get_identifier(attr), args, TYPE_ATTRIBUTES(typ));
}

#[inline]
pub unsafe fn add_type_attr(mut typ: tree, attr: *const i8, args: tree) {
    let main_variant = TYPE_MAIN_VARIANT(typ);
    __add_type_attr(TYPE_CANONICAL(typ), attr, args);
    __add_type_attr(TYPE_CANONICAL(main_variant), attr, args);
    __add_type_attr(main_variant, attr, args);
    typ = TYPE_NEXT_VARIANT(main_variant);
    while !typ.is_null() {
        if !lookup_attribute(attr, TYPE_ATTRIBUTES(typ)).is_null() {
            TYPE_ATTRIBUTES(typ) = TYPE_ATTRIBUTES(main_variant);
        }
        __add_type_attr(TYPE_CANONICAL(typ), attr, args);
        typ = TYPE_NEXT_VARIANT(typ);
    }
}

#[macro_export] macro_rules! DECL_NAME_POINTER { ($node:expr) => { IDENTIFIER_POINTER(DECL_NAME($node)) }; }
#[macro_export] macro_rules! DECL_NAME_LENGTH { ($node:expr) => { IDENTIFIER_LENGTH(DECL_NAME($node)) }; }
#[macro_export] macro_rules! TYPE_NAME_POINTER { ($node:expr) => { IDENTIFIER_POINTER(TYPE_NAME($node)) }; }
#[macro_export] macro_rules! TYPE_NAME_LENGTH { ($node:expr) => { IDENTIFIER_LENGTH(TYPE_NAME($node)) }; }
#[macro_export] macro_rules! C_TYPE_FIELDS_READONLY { ($typ:expr) => { TREE_LANG_FLAG_1($typ) }; }
#[macro_export] macro_rules! PASS_INFO { ($name:ident, $reference:expr, $id:expr, $pos:expr) => {
    pub static mut $name##_pass_info: register_pass_info = register_pass_info { pass: make_$name##_pass(), reference_pass_name: $reference, ref_pass_instance_number: $id, pos_op: $pos };
}; }
#[macro_export] macro_rules! add_referenced_var { ($var:expr) => {}; }
#[macro_export] macro_rules! mark_sym_for_renaming { ($var:expr) => {}; }
#[macro_export] macro_rules! varpool_mark_needed_node { ($node:expr) => {}; }
#[macro_export] macro_rules! create_var_ann { ($var:expr) => {}; }
pub const TODO_dump_func: i32 = 0;
pub const TODO_dump_cgraph: i32 = 0;
pub const TODO_ggc_collect: i32 = 0;
#[macro_export] macro_rules! NODE_SYMBOL { ($node:expr) => { $node }; }
#[macro_export] macro_rules! NODE_DECL { ($node:expr) => { (*$node).decl }; }
#[macro_export] macro_rules! cgraph_node_name { ($node:expr) => { (*$node).name() }; }
#[macro_export] macro_rules! NODE_IMPLICIT_ALIAS { ($node:expr) => { (*$node).cpp_implicit_alias }; }

#[inline] pub unsafe fn get_pass_for_id(id: i32) -> *mut opt_pass { (*g).get_passes().get_pass_for_id(id) }
#[macro_export] macro_rules! INSN_DELETED_P { ($insn:expr) => { (*$insn).deleted() }; }
#[inline] pub unsafe fn get_decl_section_name(decl: const_tree) -> *const i8 { DECL_SECTION_NAME(decl) }

#[inline] pub unsafe fn change_decl_assembler_name(decl: tree, name: tree) { symtab->change_decl_assembler_name(decl, name); }
#[inline] pub unsafe fn varpool_finalize_decl(decl: tree) { varpool_node::finalize_decl(decl); }
#[inline] pub unsafe fn varpool_add_new_variable(decl: tree) { varpool_node::add(decl); }
#[inline] pub unsafe fn rebuild_cgraph_edges() -> u32 { cgraph_edge::rebuild_edges() }
#[inline] pub unsafe fn cgraph_function_node(node: cgraph_node_ptr, availability: *mut availability) -> cgraph_node_ptr { (*node).function_symbol(availability) }
#[inline] pub unsafe fn cgraph_function_or_thunk_node(node: cgraph_node_ptr, availability: *mut availability) -> cgraph_node_ptr { (*node).ultimate_alias_target(availability) }
#[inline] pub unsafe fn cgraph_only_called_directly_p(node: cgraph_node_ptr) -> bool { (*node).only_called_directly_p() }
#[inline] pub unsafe fn cgraph_function_body_availability(node: cgraph_node_ptr) -> availability { (*node).get_availability() }
#[inline] pub unsafe fn cgraph_alias_target(node: cgraph_node_ptr) -> cgraph_node_ptr { (*node).get_alias_target() }
#[inline] pub unsafe fn cgraph_for_node_and_aliases(node: cgraph_node_ptr, callback: Option<unsafe extern "C" fn(cgraph_node_ptr, *mut core::ffi::c_void) -> bool>, data: *mut core::ffi::c_void, include_overwritable: bool) -> bool { (*node).call_for_symbol_thunks_and_aliases(callback, data, include_overwritable) }
#[inline] pub unsafe fn cgraph_add_function_insertion_hook(hook: cgraph_node_hook, data: *mut core::ffi::c_void) -> *mut cgraph_node_hook_list { (*symtab).add_cgraph_insertion_hook(hook,data) }
#[inline] pub unsafe fn cgraph_remove_function_insertion_hook(entry: *mut cgraph_node_hook_list) { (*symtab).remove_cgraph_insertion_hook(entry) }
#[inline] pub unsafe fn cgraph_add_node_removal_hook(hook: cgraph_node_hook, data: *mut core::ffi::c_void) -> *mut cgraph_node_hook_list { (*symtab).add_cgraph_removal_hook(hook,data) }
#[inline] pub unsafe fn cgraph_remove_node_removal_hook(entry: *mut cgraph_node_hook_list) { (*symtab).remove_cgraph_removal_hook(entry) }
#[inline] pub unsafe fn cgraph_add_node_duplication_hook(hook: cgraph_2node_hook, data: *mut core::ffi::c_void) -> *mut cgraph_2node_hook_list { (*symtab).add_cgraph_duplication_hook(hook,data) }
#[inline] pub unsafe fn cgraph_remove_node_duplication_hook(entry: *mut cgraph_2node_hook_list) { (*symtab).remove_cgraph_duplication_hook(entry) }
#[inline] pub unsafe fn cgraph_call_node_duplication_hooks(node: cgraph_node_ptr, node2: cgraph_node_ptr) { (*symtab).call_cgraph_duplication_hooks(node,node2) }
#[inline] pub unsafe fn cgraph_call_edge_duplication_hooks(cs1: *mut cgraph_edge, cs2: *mut cgraph_edge) { (*symtab).call_edge_duplication_hooks(cs1,cs2) }

#[inline] pub unsafe fn gimple_build_assign_with_ops(subcode: tree_code, lhs: tree, op1: tree, op2: tree) -> gimple_ptr { gimple_build_assign(lhs, subcode, op1, op2) }
#[inline] pub unsafe fn as_a_gasm(stmt: gimple_ptr) -> *mut gasm { as_a::<*mut gasm>(stmt) }
#[inline] pub unsafe fn as_a_const_gasm(stmt: const_gimple_ptr) -> *const gasm { as_a::<*const gasm>(stmt) }
#[inline] pub unsafe fn as_a_gassign(stmt: gimple_ptr) -> *mut gassign { as_a::<*mut gassign>(stmt) }
#[inline] pub unsafe fn as_a_const_gassign(stmt: const_gimple_ptr) -> *const gassign { as_a::<*const gassign>(stmt) }
#[inline] pub unsafe fn as_a_gcall(stmt: gimple_ptr) -> *mut gcall { as_a::<*mut gcall>(stmt) }
#[inline] pub unsafe fn as_a_const_gcall(stmt: const_gimple_ptr) -> *const gcall { as_a::<*const gcall>(stmt) }
#[inline] pub unsafe fn as_a_ggoto(stmt: gimple_ptr) -> *mut ggoto { as_a::<*mut ggoto>(stmt) }
#[inline] pub unsafe fn as_a_const_ggoto(stmt: const_gimple_ptr) -> *const ggoto { as_a::<*const ggoto>(stmt) }
#[inline] pub unsafe fn as_a_gphi(stmt: gimple_ptr) -> *mut gphi { as_a::<*mut gphi>(stmt) }
#[inline] pub unsafe fn as_a_const_gphi(stmt: const_gimple_ptr) -> *const gphi { as_a::<*const gphi>(stmt) }
#[inline] pub unsafe fn as_a_greturn(stmt: gimple_ptr) -> *mut greturn { as_a::<*mut greturn>(stmt) }
#[inline] pub unsafe fn as_a_const_greturn(stmt: const_gimple_ptr) -> *const greturn { as_a::<*const greturn>(stmt) }

#[inline] pub unsafe fn ipa_ref_referring_node(reference: *mut ipa_ref) -> cgraph_node_ptr { dyn_cast((*reference).referring) }
#[inline] pub unsafe fn ipa_remove_stmt_references(referring_node: *mut symtab_node, stmt: gimple_ptr) { (*referring_node).remove_stmt_references(stmt); }

#[macro_export] macro_rules! gen_rtx_set { ($a:expr, $b:expr) => { gen_rtx_SET($a, $b) }; }
#[macro_export] macro_rules! get_inner_reference { ($($arg:expr),*) => { get_inner_reference($($arg),*) }; }
#[macro_export] macro_rules! last_stmt { ($x:expr) => { last_nondebug_stmt($x) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
