/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dwarf-aux.h : libdw auxiliary interfaces
 *
 * Rust translation of the C header. C include dependencies:
 * <dwarf.h>, <elfutils/libdw.h>, <elfutils/libdwfl.h>,
 * <elfutils/version.h>
 */

use std::ffi::{c_char, c_int, c_void};

pub enum strbuf {}

pub const DIE_FIND_CB_END: c_int = 0; /* End of Search */
pub const DIE_FIND_CB_CHILD: c_int = 1; /* Search only children */
pub const DIE_FIND_CB_SIBLING: c_int = 2; /* Search only siblings */
pub const DIE_FIND_CB_CONTINUE: c_int = 3; /* Search children and siblings */

pub type line_walk_callback_t = Option<
    unsafe extern "C" fn(fname: *const c_char, lineno: c_int, addr: Dwarf_Addr, data: *mut c_void) -> c_int,
>;

pub type die_callback_t =
    Option<unsafe extern "C" fn(arg1: *mut Dwarf_Die, arg2: *mut c_void) -> c_int>;

static UNKNOWN: &[u8] = b"<unknown>\0";

#[inline]
pub unsafe fn die_name(die: *mut Dwarf_Die) -> *const c_char {
    let name = unsafe { dwarf_diename(die) };

    if !name.is_null() {
        name
    } else {
        UNKNOWN.as_ptr() as *const c_char
    }
}

/* Variable type information */
#[repr(C)]
pub struct die_var_type {
    pub next: *mut die_var_type,
    pub die_off: u64,
    pub addr: u64,
    pub end: u64, /* end address of location range */
    pub reg: c_int,
    pub offset: c_int,
    /* Whether the register holds a address to the type */
    pub is_reg_var_addr: bool,
    pub has_range: bool, /* whether end is valid */
}

unsafe extern "C" {
    pub fn dwarf_diename(die: *mut Dwarf_Die) -> *const c_char;

    /* Find the realpath of the target file */
    pub fn cu_find_realpath(cu_die: *mut Dwarf_Die, fname: *const c_char) -> *const c_char;

    /* Get DW_AT_comp_dir (should be NULL with older gcc) */
    pub fn cu_get_comp_dir(cu_die: *mut Dwarf_Die) -> *const c_char;

    /* Get a line number and file name for given address */
    pub fn cu_find_lineinfo(
        cudie: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        fname: *mut *const c_char,
        lineno: *mut c_int,
    ) -> c_int;

    /* Walk on functions at given address */
    pub fn cu_walk_functions_at(
        cu_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        callback: die_callback_t,
        data: *mut c_void,
    ) -> c_int;

    /* Get DW_AT_linkage_name (should be NULL for C binary) */
    pub fn die_get_linkage_name(dw_die: *mut Dwarf_Die) -> *const c_char;

    /* Get the lowest PC in DIE (including range list) */
    pub fn die_entrypc(dw_die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int;

    /* Ensure that this DIE is a subprogram and definition (not declaration) */
    pub fn die_is_func_def(dw_die: *mut Dwarf_Die) -> bool;

    /* Ensure that this DIE is an instance of a subprogram */
    pub fn die_is_func_instance(dw_die: *mut Dwarf_Die) -> bool;

    /* Compare diename and tname */
    pub fn die_compare_name(dw_die: *mut Dwarf_Die, tname: *const c_char) -> bool;

    /* Matching diename with glob pattern */
    pub fn die_match_name(dw_die: *mut Dwarf_Die, glob: *const c_char) -> bool;

    /* Get callsite line number of inline-function instance */
    pub fn die_get_call_lineno(in_die: *mut Dwarf_Die) -> c_int;

    /* Get callsite file name of inlined function instance */
    pub fn die_get_call_file(in_die: *mut Dwarf_Die) -> *const c_char;

    /* Get declared file name of a DIE */
    pub fn die_get_decl_file(dw_die: *mut Dwarf_Die) -> *const c_char;

    /* Get type die */
    pub fn die_get_type(vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die;

    /* Get a type die, but skip qualifiers */
    pub fn __die_get_real_type(vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die)
        -> *mut Dwarf_Die;

    /* Get a type die, but skip qualifiers and typedef */
    pub fn die_get_real_type(vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die;

    /* Get a pointer/array type, following typedefs/qualifiers */
    pub fn die_get_pointer_type(
        type_die: *mut Dwarf_Die,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Check whether the DIE is signed or not */
    pub fn die_is_signed_type(tp_die: *mut Dwarf_Die) -> bool;

    /* Get data_member_location offset */
    pub fn die_get_data_member_location(mb_die: *mut Dwarf_Die, offs: *mut Dwarf_Word) -> c_int;

    /* Search child DIEs */
    pub fn die_find_child(
        rt_die: *mut Dwarf_Die,
        callback: die_callback_t,
        data: *mut c_void,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Search a non-inlined function including given address */
    pub fn die_find_realfunc(
        cu_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Search a non-inlined function with tail call at given address */
    pub fn die_find_tailfunc(
        cu_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Search the top inlined function including given address */
    pub fn die_find_top_inlinefunc(
        sp_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Search the deepest inlined function including given address */
    pub fn die_find_inlinefunc(
        sp_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Search a non-inlined function by name and returns its return type */
    pub fn die_find_func_rettype(
        sp_die: *mut Dwarf_Die,
        name: *const c_char,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Walk on the instances of given DIE */
    pub fn die_walk_instances(
        in_die: *mut Dwarf_Die,
        callback: die_callback_t,
        data: *mut c_void,
    ) -> c_int;

    /*
     * Walk on lines inside given DIE. If the DIE is a subprogram, walk only on
     * the lines inside the subprogram, otherwise the DIE must be a CU DIE.
     */
    pub fn die_walk_lines(
        rt_die: *mut Dwarf_Die,
        callback: line_walk_callback_t,
        data: *mut c_void,
    ) -> c_int;

    /* Find a variable called 'name' at given address */
    pub fn die_find_variable_at(
        sp_die: *mut Dwarf_Die,
        name: *const c_char,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Find a member called 'name' */
    pub fn die_find_member(
        st_die: *mut Dwarf_Die,
        name: *const c_char,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Get the name of given type DIE */
    pub fn die_get_typename_from_type(type_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;

    /* Get the name of given variable DIE */
    pub fn die_get_typename(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;

    /* Get the name and type of given variable DIE, stored as "type\tname" */
    pub fn die_get_varname(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;

    /* Check if target program is compiled with optimization */
    pub fn die_is_optimized_target(cu_die: *mut Dwarf_Die) -> bool;

    /* Use next address after prologue as probe location */
    pub fn die_skip_prologue(
        sp_die: *mut Dwarf_Die,
        cu_die: *mut Dwarf_Die,
        entrypc: *mut Dwarf_Addr,
    );

    /* Get the list of including scopes */
    pub fn die_get_scopes(
        cu_die: *mut Dwarf_Die,
        pc: Dwarf_Addr,
        scopes: *mut *mut Dwarf_Die,
    ) -> c_int;

    /* Return type info of a member at offset */
    pub fn die_get_member_type(
        type_die: *mut Dwarf_Die,
        offset: c_int,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Return type info where the pointer and offset point to */
    pub fn die_deref_ptr_type(
        ptr_die: *mut Dwarf_Die,
        offset: c_int,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Get byte offset range of given variable DIE */
    pub fn die_get_var_range(
        sp_die: *mut Dwarf_Die,
        vr_die: *mut Dwarf_Die,
        buf: *mut strbuf,
    ) -> c_int;

    /* Find a variable saved in the 'reg' at given address */
    pub fn die_find_variable_by_reg(
        sc_die: *mut Dwarf_Die,
        pc: Dwarf_Addr,
        reg: c_int,
        type_die: *mut Dwarf_Die,
        poffset: *mut c_int,
        is_fbreg: bool,
        die_mem: *mut Dwarf_Die,
    ) -> *mut Dwarf_Die;

    /* Find a (global) variable located in the 'addr' */
    pub fn die_find_variable_by_addr(
        sc_die: *mut Dwarf_Die,
        addr: Dwarf_Addr,
        die_mem: *mut Dwarf_Die,
        type_die: *mut Dwarf_Die,
        offset: *mut c_int,
    ) -> *mut Dwarf_Die;

    /* Save all variables and parameters in this scope */
    pub fn die_collect_vars(sc_die: *mut Dwarf_Die, var_types: *mut *mut die_var_type);

    /* Save all global variables in this CU */
    pub fn die_collect_global_vars(cu_die: *mut Dwarf_Die, var_types: *mut *mut die_var_type);

    /* Get the frame base information from CFA */
    pub fn die_get_cfa(dwarf: *mut Dwarf, pc: u64, preg: *mut c_int, poffset: *mut c_int)
        -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
