/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int};

/* Dependencies in the original header:
 * <stdbool.h>, "intlist.h", "build-id.h", "probe-event.h",
 * <linux/ctype.h>
 */

pub const MAX_PROBE_BUFFER: usize = 1024;
pub const MAX_PROBES: usize = 128;
pub const MAX_PROBE_ARGS: usize = 128;

pub const PROBE_ARG_VARS: &[u8] = b"$vars\0";
pub const PROBE_ARG_PARAMS: &[u8] = b"$params\0";

unsafe extern "C" {
    fn isalpha(c: c_int) -> c_int;
}

#[inline]
pub unsafe fn is_c_varname(name: *const c_char) -> c_int {
    /* TODO */
    let ch = unsafe { *name.add(0) };
    ((unsafe { isalpha(ch as c_int) } != 0) || ch == b'_' as c_char) as c_int
}

/* Original condition: #ifdef HAVE_LIBDW_SUPPORT
 * Dependencies in that block: "dwarf-aux.h", "debuginfo.h"
 */

#[cfg(HAVE_LIBDW_SUPPORT)]
unsafe extern "C" {
    /* Check the language code is known C */
    pub fn is_known_C_lang(lang: c_int) -> bool;

    /* Find probe_trace_events specified by perf_probe_event from debuginfo */
    pub fn debuginfo__find_trace_events(
        dbg: *mut debuginfo,
        pev: *mut perf_probe_event,
        tevs: *mut *mut probe_trace_event,
    ) -> c_int;

    /* Find a perf_probe_point from debuginfo */
    pub fn debuginfo__find_probe_point(
        dbg: *mut debuginfo,
        addr: u64,
        ppt: *mut perf_probe_point,
    ) -> c_int;

    /* Find a line range */
    pub fn debuginfo__find_line_range(dbg: *mut debuginfo, lr: *mut line_range) -> c_int;

    /* Find available variables */
    pub fn debuginfo__find_available_vars_at(
        dbg: *mut debuginfo,
        pev: *mut perf_probe_event,
        vls: *mut *mut variable_list,
    ) -> c_int;

    /* Find a src file from a DWARF tag path */
    pub fn find_source_path(
        raw_path: *const c_char,
        sbuild_id: *const c_char,
        comp_dir: *const c_char,
        new_path: *mut *mut c_char,
    ) -> c_int;
}

#[cfg(HAVE_LIBDW_SUPPORT)]
#[repr(C)]
pub struct probe_finder {
    pub pev: *mut perf_probe_event, /* Target probe event */
    pub dbg: *mut debuginfo,

    /* Callback when a probe point is found */
    pub callback: Option<unsafe extern "C" fn(sc_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int>,

    /* For function searching */
    pub lno: c_int,             /* Line number */
    pub addr: Dwarf_Addr,       /* Address */
    pub fname: *const c_char,   /* Real file name */
    pub cu_die: Dwarf_Die,      /* Current CU */
    pub sp_die: Dwarf_Die,
    pub abstrace_dieoffset: Dwarf_Off,
    pub lcache: *mut intlist,   /* Line cache for lazy match */

    /* For variable searching */
    /* Call Frame Information from .eh_frame. Owned by this struct. */
    pub cfi_eh: *mut Dwarf_CFI,
    /* Call Frame Information from .debug_frame. Not owned. */
    pub cfi_dbg: *mut Dwarf_CFI,
    pub fb_ops: *mut Dwarf_Op,  /* Frame base attribute */
    pub e_machine: u32,         /* ELF target machine arch */
    pub e_flags: u32,           /* ELF target machine flags */
    pub pvar: *mut perf_probe_arg,  /* Current target variable */
    pub tvar: *mut probe_trace_arg, /* Current result variable */
    pub skip_empty_arg: bool,   /* Skip non-exist args */
}

#[cfg(HAVE_LIBDW_SUPPORT)]
#[repr(C)]
pub struct trace_event_finder {
    pub pf: probe_finder,
    pub mod_: *mut Dwfl_Module,         /* For solving symbols */
    pub tevs: *mut probe_trace_event,   /* Found trace events */
    pub ntevs: c_int,                   /* Number of trace events */
    pub max_tevs: c_int,                /* Max number of trace events */
}

#[cfg(HAVE_LIBDW_SUPPORT)]
#[repr(C)]
pub struct available_var_finder {
    pub pf: probe_finder,
    pub mod_: *mut Dwfl_Module,       /* For solving symbols */
    pub vls: *mut variable_list,      /* Found variable lists */
    pub nvls: c_int,                  /* Number of variable lists */
    pub max_vls: c_int,               /* Max no. of variable lists */
    pub child: bool,                  /* Search child scopes */
}

#[cfg(HAVE_LIBDW_SUPPORT)]
#[repr(C)]
pub struct line_finder {
    pub lr: *mut line_range, /* Target line range */

    pub fname: *const c_char, /* File name */
    pub lno_s: c_int,         /* Start line number */
    pub lno_e: c_int,         /* End line number */
    pub cu_die: Dwarf_Die,    /* Current CU */
    pub sp_die: Dwarf_Die,
    pub found: c_int,
}

#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub fn is_known_C_lang(_lang: c_int) -> bool {
    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
