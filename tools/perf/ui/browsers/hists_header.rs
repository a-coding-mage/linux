/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/ui/browsers/hists.h.
// C include dependency: "ui/browser.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct hist_browser {
    pub b: ui_browser,
    pub hists: *mut hists,
    pub he_selection: *mut hist_entry,
    pub selection: *mut map_symbol,
    pub hbt: *mut hist_browser_timer,
    pub pstack: *mut pstack,
    pub env: *mut perf_env,
    pub block_evsel: *mut evsel,
    pub print_seq: c_int,
    pub show_dso: bool,
    pub show_headers: bool,
    pub min_pcnt: f32,
    pub nr_non_filtered_entries: u64,
    pub nr_hierarchy_entries: u64,
    pub nr_callchain_rows: u64,
    pub c2c_filter: bool,

    /* Get title string. */
    pub title: Option<
        unsafe extern "C" fn(
            browser: *mut hist_browser,
            bf: *mut c_char,
            size: usize,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub fn hist_browser__new(hists: *mut hists) -> *mut hist_browser;
    pub fn hist_browser__delete(browser: *mut hist_browser);
    pub fn hist_browser__run(
        browser: *mut hist_browser,
        help: *const c_char,
        warn_lost_event: bool,
        key: c_int,
    ) -> c_int;
    pub fn hist_browser__init(browser: *mut hist_browser, hists: *mut hists);
}
