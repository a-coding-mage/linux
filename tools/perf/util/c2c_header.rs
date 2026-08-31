/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of perf/util/c2c.h.
 *
 * C include dependencies preserved for the eventual integration context:
 * <stdbool.h>, <stdint.h>, <linux/types.h>, "hist.h", "mem-events.h",
 * and "stat.h".
 */

#[repr(C)]
pub struct sort_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct c2c_hists {
    pub hists: hists,
    pub list: perf_hpp_list,
    pub stats: c2c_stats,
}

#[repr(C)]
pub struct compute_stats {
    pub lcl_hitm: stats,
    pub rmt_hitm: stats,
    pub lcl_peer: stats,
    pub rmt_peer: stats,
    pub load: stats,
}

#[repr(C)]
pub struct c2c_hist_entry {
    pub hists: *mut c2c_hists,
    pub evsel: *mut evsel,
    pub stats: c2c_stats,
    pub cpuset: *mut libc::c_ulong,
    pub nodeset: *mut libc::c_ulong,
    pub node_stats: *mut c2c_stats,
    pub cacheline_idx: libc::c_uint,

    pub cstats: compute_stats,

    pub paddr: libc::c_ulong,
    pub paddr_cnt: libc::c_ulong,
    pub paddr_zero: bool,
    pub nodestr: *mut libc::c_char,

    /*
     * must be at the end,
     * because of its callchain dynamic entry
     */
    pub he: hist_entry,
}

pub const C2C_HEADER_MAX: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_header_line {
    pub text: *const libc::c_char,
    pub span: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_header {
    pub line: [c2c_header_line; C2C_HEADER_MAX],
}

#[repr(C)]
pub struct c2c_dimension {
    pub header: c2c_header,
    pub name: *const libc::c_char,
    pub width: libc::c_int,
    pub se: *mut sort_entry,

    pub cmp: Option<
        unsafe extern "C" fn(
            fmt: *mut perf_hpp_fmt,
            left: *mut hist_entry,
            right: *mut hist_entry,
        ) -> i64,
    >,
    pub entry: Option<
        unsafe extern "C" fn(
            fmt: *mut perf_hpp_fmt,
            hpp: *mut perf_hpp,
            he: *mut hist_entry,
        ) -> libc::c_int,
    >,
    pub color: Option<
        unsafe extern "C" fn(
            fmt: *mut perf_hpp_fmt,
            hpp: *mut perf_hpp,
            he: *mut hist_entry,
        ) -> libc::c_int,
    >,
}

#[repr(C)]
pub struct c2c_fmt {
    pub fmt: perf_hpp_fmt,
    pub dim: *mut c2c_dimension,
}

pub const SYMBOL_WIDTH: libc::c_int = 30;

/*
 * C macro translation:
 *
 * HEADER_LOW(__h)
 *     { .line[1] = { .text = __h } }
 *
 * HEADER_BOTH(__h0, __h1)
 *     { .line[0] = { .text = __h0 }, .line[1] = { .text = __h1 } }
 */
#[inline]
pub const fn HEADER_LOW(__h: *const libc::c_char) -> c2c_header {
    c2c_header {
        line: [
            c2c_header_line {
                text: core::ptr::null(),
                span: 0,
            },
            c2c_header_line { text: __h, span: 0 },
        ],
    }
}

#[inline]
pub const fn HEADER_BOTH(
    __h0: *const libc::c_char,
    __h1: *const libc::c_char,
) -> c2c_header {
    c2c_header {
        line: [
            c2c_header_line {
                text: __h0,
                span: 0,
            },
            c2c_header_line {
                text: __h1,
                span: 0,
            },
        ],
    }
}

unsafe extern "C" {
    pub fn c2c_fmt_free(fmt: *mut perf_hpp_fmt);
    pub fn c2c_fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool;

    /*
     * Build the function-view hierarchy. Returns -EOPNOTSUPP when @cl_sort lacks
     * iaddr. On success, *@hists remains valid until the next
     * c2c_function__build() or c2c_function__reset(). On failure, *@hists is
     * NULL.
     */
    pub fn c2c_function__build(
        cl_hists: *mut c2c_hists,
        cl_sort: *const libc::c_char,
        symbol_full: bool,
        hists: *mut *mut hists,
    ) -> libc::c_int;
    pub fn c2c_function__reset();

    /* Valid only between a successful build and c2c_function__reset(). */
    pub fn c2c_function__find_cacheline(he: *mut hist_entry) -> *mut hist_entry;
}

/* Inputs and TUI callback supplied by the c2c command. */
#[repr(C)]
pub struct c2c_function_view_args {
    /* Source cacheline histograms used by the common model. */
    pub cl_hists: *mut c2c_hists,
    /* --coalesce field list, used to require iaddr. */
    pub cl_sort: *const libc::c_char,
    /* Do not cap long symbol names. */
    pub symbol_full: bool,
    /* Open the cacheline detail view for @he. */
    pub browse_cacheline:
        Option<unsafe extern "C" fn(he: *mut hist_entry) -> libc::c_int>,
}

/*
 * C conditional translation:
 *
 * #ifdef HAVE_SLANG_SUPPORT
 * int perf_c2c__browse_function_view(struct c2c_function_view_args *args);
 * #else
 * static inline int
 * perf_c2c__browse_function_view(struct c2c_function_view_args *args __maybe_unused)
 * {
 *     return 0;
 * }
 * #endif
 */
#[cfg(HAVE_SLANG_SUPPORT)]
unsafe extern "C" {
    pub fn perf_c2c__browse_function_view(
        args: *mut c2c_function_view_args,
    ) -> libc::c_int;
}

#[cfg(not(HAVE_SLANG_SUPPORT))]
#[inline]
pub unsafe fn perf_c2c__browse_function_view(
    _args: *mut c2c_function_view_args,
) -> libc::c_int {
    0
}
