/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/annotate-data.h. */
/* C includes:
 * <errno.h>, <linux/compiler.h>, <linux/rbtree.h>, <linux/types.h>,
 * "dwarf-regs.h", "annotate.h", and, with HAVE_LIBDW_SUPPORT, "debuginfo.h".
 */

use core::ffi::{c_char, c_int};

/* External C types supplied by other perf/Linux headers. */
pub enum arch {}
pub enum annotated_op_loc {}
pub enum debuginfo {}
pub enum evsel {}
pub enum hist_browser_timer {}
pub enum hist_entry {}
pub enum map_symbol {}
pub enum thread {}

/* External C types supplied by included headers. */
pub enum Dwarf_Die {}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub __rb_parent_color: usize,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type s32 = i32;
pub type size_t = usize;

/* pr_debug_dtp(fmt, ...):
 * if (debug_type_profile)
 *     pr_info(fmt, ...);
 * else
 *     pr_debug3(fmt, ...);
 *
 * Variadic debug-printing macro depends on external C preprocessor symbols and
 * logging macros, so it is preserved here as intent rather than executable Rust.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum type_state_kind {
    TSR_KIND_INVALID = 0,
    TSR_KIND_TYPE,
    TSR_KIND_PERCPU_BASE,
    TSR_KIND_CONST,
    TSR_KIND_PERCPU_POINTER,
    TSR_KIND_POINTER,
    TSR_KIND_CANARY,
}

/**
 * struct annotated_member - Type of member field
 * @node: List entry in the parent list
 * @children: List head for child nodes
 * @type_name: Name of the member type
 * @var_name: Name of the member variable
 * @offset: Offset from the outer data type
 * @size: Size of the member field
 *
 * This represents a member type in a data type.
 */
#[repr(C)]
pub struct annotated_member {
    pub node: list_head,
    pub children: list_head,
    pub type_name: *mut c_char,
    pub var_name: *mut c_char,
    pub offset: c_int,
    pub size: c_int,
}

/**
 * struct type_hist_entry - Histogram entry per offset
 * @nr_samples: Number of samples
 * @period: Count of event
 */
#[repr(C)]
pub struct type_hist_entry {
    pub nr_samples: c_int,
    pub period: u64,
}

/**
 * struct type_hist - Type histogram for each event
 * @nr_samples: Total number of samples in this data type
 * @period: Total count of the event in this data type
 * @offset: Array of histogram entry
 */
#[repr(C)]
pub struct type_hist {
    pub nr_samples: u64,
    pub period: u64,
    /* C flexible array member: struct type_hist_entry addr[]; */
    pub addr: [type_hist_entry; 0],
}

/**
 * struct annotated_data_type - Data type to profile
 * @node: RB-tree node for dso->type_tree
 * @self: Actual type information
 * @nr_histogram: Number of histogram entries
 * @histograms: An array of pointers to histograms
 *
 * This represents a data type accessed by samples in the profile data.
 */
#[repr(C)]
pub struct annotated_data_type {
    pub node: rb_node,
    pub self_: annotated_member,
    pub nr_histograms: c_int,
    pub histograms: *mut *mut type_hist,
}

unsafe extern "C" {
    pub static mut unknown_type: annotated_data_type;
    pub static mut stackop_type: annotated_data_type;
    pub static mut canary_type: annotated_data_type;
}

/**
 * struct data_loc_info - Data location information
 * @arch: CPU architecture info
 * @thread: Thread info
 * @ms: Map and Symbol info
 * @ip: Instruction address
 * @var_addr: Data address (for global variables)
 * @cpumode: CPU execution mode
 * @op: Instruction operand location (regs and offset)
 * @di: Debug info
 * @fbreg: Frame base register
 * @fb_cfa: Whether the frame needs to check CFA
 * @type_offset: Final offset in the type
 */
#[repr(C)]
pub struct data_loc_info {
    /* These are input field, should be filled by caller */
    pub arch: *const arch,
    pub thread: *mut thread,
    pub ms: *mut map_symbol,
    pub ip: u64,
    pub var_addr: u64,
    pub cpumode: u8,
    pub op: *mut annotated_op_loc,
    pub di: *mut debuginfo,

    /* These are used internally */
    pub fbreg: c_int,
    pub fb_cfa: bool,

    /* This is for the result */
    pub type_offset: c_int,
}

/**
 * struct annotated_data_stat - Debug statistics
 * @total: Total number of entry
 * @no_sym: No symbol or map found
 * @no_insn: Failed to get disasm line
 * @no_insn_ops: The instruction has no operands
 * @no_mem_ops: The instruction has no memory operands
 * @no_reg: Failed to extract a register from the operand
 * @no_dbginfo: The binary has no debug information
 * @no_cuinfo: Failed to find a compile_unit
 * @no_var: Failed to find a matching variable
 * @no_typeinfo: Failed to get a type info for the variable
 * @invalid_size: Failed to get a size info of the type
 * @bad_offset: The access offset is out of the type
 */
#[repr(C)]
pub struct annotated_data_stat {
    pub total: c_int,
    pub no_sym: c_int,
    pub no_insn: c_int,
    pub no_insn_ops: c_int,
    pub no_mem_ops: c_int,
    pub no_reg: c_int,
    pub no_dbginfo: c_int,
    pub no_cuinfo: c_int,
    pub no_var: c_int,
    pub no_typeinfo: c_int,
    pub invalid_size: c_int,
    pub bad_offset: c_int,
    pub insn_track: c_int,
}

unsafe extern "C" {
    pub static mut ann_data_stat: annotated_data_stat;
}

/* The following block is present in C only when HAVE_LIBDW_SUPPORT is enabled. */

/*
 * Type information in a register, valid when @ok is true.
 * The @caller_saved registers are invalidated after a function call.
 */
#[repr(C)]
pub struct type_state_reg {
    pub type_: Dwarf_Die,
    pub imm_value: u32,
    /*
     * The offset within the struct that the register points to.
     * A value of 0 means the register points to the beginning.
     * type_offset = op->offset + reg->offset
     */
    pub offset: s32,
    pub ok: bool,
    pub caller_saved: bool,
    /* DWARF location range tracking for register lifetime */
    pub lifetime_active: bool,
    pub lifetime_end: u64,
    pub kind: u8,
    pub copied_from: u8,
}

/* Type information in a stack location, dynamically allocated */
#[repr(C)]
pub struct type_state_stack {
    pub list: list_head,
    pub type_: Dwarf_Die,
    pub offset: c_int,
    /* pointer offset, saves tsr->offset on the stack state */
    pub ptr_offset: c_int,
    pub size: c_int,
    pub compound: bool,
    pub kind: u8,
}

/*
 * Maximum number of registers tracked in type_state.
 *
 * This limit must cover all supported architectures, since perf
 * may analyze perf.data files generated on systems with a different
 * register set. Use 32 as a safe upper bound instead of relying on
 * build-arch specific values.
 */
pub const TYPE_STATE_MAX_REGS: usize = 32;

/*
 * State table to maintain type info in each register and stack location.
 * It'll be updated when new variable is allocated or type info is moved
 * to a new location (register or stack).  As it'd be used with the
 * shortest path of basic blocks, it only maintains a single table.
 */
#[repr(C)]
pub struct type_state {
    /* state of general purpose registers */
    pub regs: [type_state_reg; TYPE_STATE_MAX_REGS],
    /* state of stack location */
    pub stack_vars: list_head,
    /* return value register */
    pub ret_reg: c_int,
    /* stack pointer register */
    pub stack_reg: c_int,
}

unsafe extern "C" {
    /* Returns data type at the location (ip, reg, offset) */
    pub fn find_data_type(dloc: *mut data_loc_info) -> *mut annotated_data_type;

    /* Update type access histogram at the given offset */
    pub fn annotated_data_type__update_samples(
        adt: *mut annotated_data_type,
        evsel: *mut evsel,
        offset: c_int,
        nr_samples: c_int,
        period: u64,
    ) -> c_int;

    /* Release all data type information in the tree */
    pub fn annotated_data_type__tree_delete(root: *mut rb_root);

    /* Release all global variable information in the tree */
    pub fn global_var_type__tree_delete(root: *mut rb_root);

    /* Print data type annotation (including members) on stdout */
    pub fn hist_entry__annotate_data_tty(he: *mut hist_entry, evsel: *mut evsel) -> c_int;

    /* Get name of member field at the given offset in the data type */
    pub fn annotated_data_type__get_member_name(
        adt: *mut annotated_data_type,
        buf: *mut c_char,
        sz: size_t,
        member_offset: c_int,
    ) -> c_int;

    pub fn has_reg_type(state: *mut type_state, reg: c_int) -> bool;
    pub fn findnew_stack_state(
        state: *mut type_state,
        offset: c_int,
        kind: u8,
        type_die: *mut Dwarf_Die,
        ptr_offset: c_int,
    ) -> *mut type_state_stack;
    pub fn set_stack_state(
        stack: *mut type_state_stack,
        offset: c_int,
        kind: u8,
        type_die: *mut Dwarf_Die,
        ptr_offset: c_int,
    );
    pub fn find_stack_state(state: *mut type_state, offset: c_int) -> *mut type_state_stack;
    pub fn get_global_var_type(
        cu_die: *mut Dwarf_Die,
        dloc: *mut data_loc_info,
        ip: u64,
        var_addr: u64,
        var_offset: *mut c_int,
        type_die: *mut Dwarf_Die,
    ) -> bool;
    pub fn get_global_var_info(
        dloc: *mut data_loc_info,
        addr: u64,
        var_name: *mut *const c_char,
        var_offset: *mut c_int,
    ) -> bool;
    pub fn pr_debug_type_name(die: *mut Dwarf_Die, kind: type_state_kind);
}

/* Without HAVE_LIBDW_SUPPORT, the C header provides these inline fallbacks:
 *
 * find_data_type(...) -> NULL
 * annotated_data_type__update_samples(...) -> -1
 * annotated_data_type__tree_delete(...) -> ()
 * global_var_type__tree_delete(...) -> ()
 * hist_entry__annotate_data_tty(...) -> -1
 * annotated_data_type__get_member_name(...) -> -1
 *
 * They are conditional in C and are documented here to preserve that branch
 * without inventing a Rust build configuration for HAVE_LIBDW_SUPPORT.
 */

/* With HAVE_SLANG_SUPPORT, this function is an external declaration. */
unsafe extern "C" {
    pub fn hist_entry__annotate_data_tui(
        he: *mut hist_entry,
        evsel: *mut evsel,
        hbt: *mut hist_browser_timer,
    ) -> c_int;
}

/* Without HAVE_SLANG_SUPPORT, the C inline fallback returns -1. */
#[inline]
pub unsafe fn hist_entry__annotate_data_tui_no_slang(
    _he: *mut hist_entry,
    _evsel: *mut evsel,
    _hbt: *mut hist_browser_timer,
) -> c_int {
    -1
}
