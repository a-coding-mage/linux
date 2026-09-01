// SPDX-License-Identifier: GPL-2.0
/*
 * C2C function browser - TUI front end for function-level sharing analysis
 */

use core::ffi::{c_char, c_float, c_int, c_uint, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// Dependencies from the original C includes:
// errno.h, inttypes.h, stdlib.h, sys/ttydefaults.h, linux/rbtree.h,
// linux/zalloc.h, ../browser.h, ../keysyms.h, ../libslang.h, ../ui.h,
// ../../util/c2c.h, ../../util/debug.h, ../../util/hist.h,
// ../../util/symbol.h, and hists.h.

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;

const K_TAB: c_int = 9;
const K_ESC: c_int = 27;

const fn CTRL(c: u8) -> c_int {
	(c & 0x1f) as c_int
}

#[repr(C)]
pub struct rb_node {
	_private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
	_private: [u8; 0],
}

#[repr(C)]
pub struct ui_browser {
	pub refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
	pub nr_entries: u64,
	pub extra_title_lines: c_int,
}

#[repr(C)]
pub struct hist_browser {
	pub b: ui_browser,
	pub hists: *mut hists,
	pub min_pcnt: c_float,
	pub nr_non_filtered_entries: u64,
	pub title: Option<unsafe extern "C" fn(*mut hist_browser, *mut c_char, usize) -> c_int>,
	pub c2c_filter: bool,
	pub show_headers: bool,
	pub he_selection: *mut hist_entry,
}

#[repr(C)]
pub struct hists {
	pub entries: rb_root_cached,
	pub nr_non_filtered_entries: u64,
	pub hpp_list: *mut hpp_list,
}

#[repr(C)]
pub struct hpp_list {
	pub nr_header_lines: c_int,
}

#[repr(C)]
pub struct hist_entry {
	pub rb_node: rb_node,
	pub filtered: bool,
	pub has_children: bool,
	pub unfolded: bool,
	pub hroot_out: rb_root_cached,
}

#[repr(C)]
pub struct c2c_function_view_args {
	pub cl_hists: *mut hists,
	pub cl_sort: *mut c_void,
	pub symbol_full: bool,
	pub browse_cacheline: Option<unsafe extern "C" fn(*mut hist_entry) -> c_int>,
}

#[repr(C)]
pub struct symbol_conf_t {
	pub use_callchain: bool,
}

#[repr(C)]
struct c2c_function_browser {
	hb: hist_browser,
	orig_refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
	browse_cacheline: Option<unsafe extern "C" fn(*mut hist_entry) -> c_int>,
}

unsafe extern "C" {
	static mut symbol_conf: symbol_conf_t;

	fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
	fn rb_next(node: *mut rb_node) -> *mut rb_node;
	fn hist_entry__get_percent_limit(he: *mut hist_entry) -> c_float;
	fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
	fn zalloc(size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn hist_browser__init(browser: *mut hist_browser, hists: *mut hists);
	fn c2c_function__find_cacheline(he: *mut hist_entry) -> *mut hist_entry;
	fn c2c_function__build(
		cl_hists: *mut hists,
		cl_sort: *mut c_void,
		symbol_full: bool,
		hists: *mut *mut hists,
	) -> c_int;
	fn ui__warning(fmt: *const c_char, ...);
	fn ui__error(fmt: *const c_char, ...);
	fn SLang_reset_tty();
	fn SLang_init_tty(a: c_int, b: c_int, c: c_int);
	fn SLtty_set_suspend_state(state: bool);
	fn hist_browser__run(
		browser: *mut hist_browser,
		helpline: *const c_char,
		warn_lost_event: bool,
		delay_secs: c_int,
	) -> c_int;
	fn ui_browser__help_window(browser: *mut ui_browser, text: *const c_char);
	fn c2c_function__reset();
}

unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
	(node as *mut u8).sub(core::mem::offset_of!(hist_entry, rb_node)) as *mut hist_entry
}

/*
 * Count visible entries in @root, descending only through visible, unfolded
 * parents. Match hists__filter_entries(), which drives generic browser
 * navigation, so the count cannot include rows the browser skips.
 */
unsafe fn c2c_function__nr_visible_rows(root: *mut rb_root_cached, min_pcnt: c_float) -> u64 {
	let mut nd: *mut rb_node;
	let mut rows: u64 = 0;

	nd = rb_first_cached(root);
	while !nd.is_null() {
		let he = rb_entry_hist_entry(nd);

		/*
		 * The generic refresh folds filtered parents and therefore hides
		 * their subtree. A percentage-rejected parent is merely skipped;
		 * if it is unfolded, qualifying descendants are still rendered.
		 */
		if (*he).filtered {
			nd = rb_next(nd);
			continue;
		}

		if hist_entry__get_percent_limit(he) >= min_pcnt {
			rows += 1;
		}
		if (*he).has_children && (*he).unfolded {
			rows += c2c_function__nr_visible_rows(&mut (*he).hroot_out, min_pcnt);
		}

		nd = rb_next(nd);
	}
	rows
}

unsafe fn c2c_function_browser__update_nr_entries(browser: *mut c2c_function_browser) {
	let nr_entries: u64;

	nr_entries = c2c_function__nr_visible_rows(
		&mut (*(*browser).hb.hists).entries,
		(*browser).hb.min_pcnt,
	);
	(*browser).hb.nr_non_filtered_entries = nr_entries;
	(*browser).hb.b.nr_entries = nr_entries;
}

unsafe extern "C" fn c2c_function_browser__refresh(ui_browser: *mut ui_browser) -> c_uint {
	let hist_browser = (ui_browser as *mut u8).sub(core::mem::offset_of!(hist_browser, b))
		as *mut hist_browser;
	let browser: *mut c2c_function_browser;

	browser = (hist_browser as *mut u8).sub(core::mem::offset_of!(c2c_function_browser, hb))
		as *mut c2c_function_browser;
	c2c_function_browser__update_nr_entries(browser);
	((*browser).orig_refresh.expect("orig_refresh"))(ui_browser)
}

unsafe extern "C" fn c2c_function_browser__title(
	browser: *mut hist_browser,
	bf: *mut c_char,
	size: usize,
) -> c_int {
	scnprintf(
		bf,
		size,
		c"Shared Data Functions Table     (%llu entries, sorted on Cycles %%)".as_ptr(),
		(*(*browser).hists).nr_non_filtered_entries,
	);
	0
}

unsafe fn c2c_function_browser__new(
	hists: *mut hists,
	browse_cacheline: Option<unsafe extern "C" fn(*mut hist_entry) -> c_int>,
) -> *mut c2c_function_browser {
	let browser: *mut c2c_function_browser;

	if hists.is_null() {
		return ptr::null_mut();
	}

	browser = zalloc(core::mem::size_of::<c2c_function_browser>()) as *mut c2c_function_browser;
	if browser.is_null() {
		return ptr::null_mut();
	}

	hist_browser__init(&mut (*browser).hb, hists);
	(*browser).orig_refresh = (*browser).hb.b.refresh;
	(*browser).hb.b.refresh = Some(c2c_function_browser__refresh);
	(*browser).browse_cacheline = browse_cacheline;

	(*browser).hb.title = Some(c2c_function_browser__title);
	(*browser).hb.c2c_filter = true;
	(*browser).hb.show_headers = true;
	/* Keep title line count consistent with forcing headers on. */
	(*browser).hb.b.extra_title_lines = (*(*hists).hpp_list).nr_header_lines;
	(*browser).hb.min_pcnt = 0.0;

	browser
}

unsafe fn c2c_function_browser__delete(browser: *mut c2c_function_browser) {
	free(browser as *mut c_void);
}

unsafe fn c2c_browser__browse_cacheline(
	browser: *mut c2c_function_browser,
	he_selection: *mut hist_entry,
) -> c_int {
	let he: *mut hist_entry = c2c_function__find_cacheline(he_selection);

	if !he.is_null() {
		((*browser).browse_cacheline.expect("browse_cacheline"))(he)
	} else {
		-1
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_c2c__browse_function_view(
	args: *mut c2c_function_view_args,
) -> c_int {
	let mut browser: *mut c2c_function_browser;
	let mut hists = MaybeUninit::<*mut hists>::uninit();
	let mut saved_use_callchain: bool = symbol_conf.use_callchain;
	let mut key: c_int;
	let mut ret: c_int;
	static HELP: &[u8] =
		b" d             Display details for the selected level-3 cacheline\n\
		  e/+           Expand/collapse the selected entry\n\
		  TAB/ESC/q/^C  Return to the cacheline view\n\0";

	if args.is_null() || (*args).cl_hists.is_null() || (*args).browse_cacheline.is_none() {
		return -EINVAL;
	}

	/*
	 * Function view does not display callchains; cacheline detail temporarily
	 * restores them.
	 */
	symbol_conf.use_callchain = false;

	ret = c2c_function__build(
		(*args).cl_hists,
		(*args).cl_sort,
		(*args).symbol_full,
		hists.as_mut_ptr(),
	);
	if ret != 0 {
		if ret == -EOPNOTSUPP {
			ui__warning(c"The function view requires iaddr in --coalesce.\n".as_ptr());
		} else {
			ui__error(
				c"Failed to build function view hierarchy (ret=%d)\n".as_ptr(),
				ret,
			);
		}
		goto_out(&mut saved_use_callchain);
		return ret;
	}

	let hists = hists.assume_init();
	browser = c2c_function_browser__new(hists, (*args).browse_cacheline);
	if browser.is_null() {
		ret = -ENOMEM;
		c2c_function__reset();
		goto_out(&mut saved_use_callchain);
		return ret;
	}

	/* Reset abort key so we can receive Ctrl-C as a key. */
	SLang_reset_tty();
	SLang_init_tty(0, 0, 0);
	SLtty_set_suspend_state(true);

	loop {
		c2c_function_browser__update_nr_entries(browser);
		key = hist_browser__run(&mut (*browser).hb, c"? - help".as_ptr(), true, 0);

		match key {
			x if x == b'q' as c_int || x == K_TAB || x == K_ESC || x == CTRL(b'c') => {
				break;
			}
			x if x == b'd' as c_int => {
				/* Cacheline detail honors the user's callchain setting. */
				symbol_conf.use_callchain = saved_use_callchain;
				c2c_browser__browse_cacheline(browser, (*browser).hb.he_selection);
				/*
				 * Preserve any toggle made in the detail view, then
				 * re-disable callchain for the function view.
				 */
				saved_use_callchain = symbol_conf.use_callchain;
				symbol_conf.use_callchain = false;
			}
			x if x == b'?' as c_int => {
				ui_browser__help_window(&mut (*browser).hb.b, HELP.as_ptr() as *const c_char);
			}
			_ => {}
		}
	}

	c2c_function_browser__delete(browser);
	c2c_function__reset();
	goto_out(&mut saved_use_callchain);
	ret
}

unsafe fn goto_out(saved_use_callchain: *mut bool) {
	symbol_conf.use_callchain = *saved_use_callchain;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
