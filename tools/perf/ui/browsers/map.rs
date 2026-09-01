// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/ui/browsers/map.c.
// C includes are represented here as external declarations for the symbols
// this implementation uses.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type bool_ = bool;

const STB_LOCAL: c_int = 0;
const STB_GLOBAL: c_int = 1;
const K_ENTER: c_int = 13;
const K_ESC: c_int = 27;
const K_LEFT: c_int = 260;
const fn CTRL(c: u8) -> c_int {
    (c & 0x1f) as c_int
}

#[repr(C)]
pub struct rb_node {
    __rb_parent_color: c_ulong,
    rb_right: *mut rb_node,
    rb_left: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
pub struct symbol {
    rb_node: rb_node,
    start: u64,
    end: u64,
    name: *const c_char,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ui_browser {
    entries: *mut rb_root,
    refresh: Option<unsafe extern "C" fn(*mut ui_browser) -> c_uint>,
    seek: Option<unsafe extern "C" fn(*mut ui_browser, c_long, c_int)>,
    write: Option<unsafe extern "C" fn(*mut ui_browser, *mut c_void, c_int)>,
    top: *mut rb_node,
    index: u32,
    top_idx: u32,
    nr_entries: u32,
    width: c_int,
}

type c_long = isize;

#[repr(C)]
struct map_browser {
    b: ui_browser,
    map: *mut map,
    addrlen: u8,
}

unsafe extern "C" {
    static mut verbose: c_int;

    fn ui_browser__is_current_entry(browser: *mut ui_browser, row: c_int) -> bool_;
    fn ui_browser__set_percent_color(browser: *mut ui_browser, percent: c_int, current: bool_);
    fn ui_browser__printf(browser: *mut ui_browser, fmt: *const c_char, ...) -> c_int;
    fn ui_browser__write_nstring(browser: *mut ui_browser, s: *const c_char, width: c_int);
    fn ui_browser__input_window(
        title: *const c_char,
        text: *const c_char,
        input: *mut c_char,
        exit_msg: *const c_char,
        delay_secs: c_int,
    ) -> c_int;
    fn ui_helpline__fpush(fmt: *const c_char, ...) -> c_int;
    fn ui_browser__show(browser: *mut ui_browser, title: *const c_char, helpline: *const c_char, ...)
        -> c_int;
    fn ui_browser__run(browser: *mut ui_browser, delay_secs: c_int) -> c_int;
    fn ui_browser__warn_unhandled_hotkey(
        browser: *mut ui_browser,
        key: c_int,
        delay_secs: c_int,
        title: *mut c_void,
    );
    fn ui_browser__hide(browser: *mut ui_browser);
    fn ui_browser__rb_tree_refresh(browser: *mut ui_browser) -> c_uint;
    fn ui_browser__rb_tree_seek(browser: *mut ui_browser, offset: c_long, whence: c_int);

    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__symbols(dso: *mut dso) -> *mut rb_root;
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map__find_symbol_by_name(map: *mut map, name: *const c_char) -> *mut symbol;
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn tolower(c: c_int) -> c_int;
}

unsafe fn rb_entry_symbol(node: *mut rb_node) -> *mut symbol {
    (node as *mut u8).sub(offset_of!(symbol, rb_node)) as *mut symbol
}

unsafe fn container_of_map_browser(browser: *mut ui_browser) -> *mut map_browser {
    (browser as *mut u8).sub(offset_of!(map_browser, b)) as *mut map_browser
}

unsafe extern "C" fn map_browser__write(browser: *mut ui_browser, nd: *mut c_void, row: c_int) {
    let sym = rb_entry_symbol(nd as *mut rb_node);
    let mb = container_of_map_browser(browser);
    let current_entry = ui_browser__is_current_entry(browser, row);
    let width: c_int;

    ui_browser__set_percent_color(browser, 0, current_entry);
    ui_browser__printf(
        browser,
        b"%*llx %*llx %c \0".as_ptr() as *const c_char,
        (*mb).addrlen as c_int,
        (*sym).start,
        (*mb).addrlen as c_int,
        (*sym).end,
        if symbol__binding(sym) == STB_GLOBAL {
            b'g' as c_int
        } else if symbol__binding(sym) == STB_LOCAL {
            b'l' as c_int
        } else {
            b'w' as c_int
        },
    );
    width = (*browser).width - (((*mb).addrlen as c_int * 2) + 4);
    if width > 0 {
        ui_browser__write_nstring(browser, (*sym).name, width);
    }
}

/* FIXME uber-kludgy, see comment on cmd_report... */
unsafe fn symbol__browser_index(browser: *mut symbol) -> *mut u32 {
    (browser as *mut c_void as *mut u8)
        .sub(size_of::<rb_node>())
        .sub(size_of::<u32>()) as *mut u32
}

unsafe fn map_browser__search(browser: *mut map_browser) -> c_int {
    let mut target = [0 as c_char; 512];
    let sym: *mut symbol;
    let err = ui_browser__input_window(
        b"Search by name/addr\0".as_ptr() as *const c_char,
        b"Prefix with 0x to search by address\0".as_ptr() as *const c_char,
        target.as_mut_ptr(),
        b"ENTER: OK, ESC: Cancel\0".as_ptr() as *const c_char,
        0,
    );
    if err != K_ENTER {
        return -1;
    }

    if target[0] == b'0' as c_char && tolower(target[1] as c_int) == b'x' as c_int {
        let addr = strtoull(target.as_ptr(), ptr::null_mut(), 16);
        sym = map__find_symbol((*browser).map, addr);
    } else {
        sym = map__find_symbol_by_name((*browser).map, target.as_ptr());
    }

    if !sym.is_null() {
        let idx = symbol__browser_index(sym);

        (*browser).b.top = ptr::addr_of_mut!((*sym).rb_node);
        (*browser).b.top_idx = *idx;
        (*browser).b.index = (*browser).b.top_idx;
    } else {
        ui_helpline__fpush(
            b"%s not found!\0".as_ptr() as *const c_char,
            target.as_ptr(),
        );
    }

    0
}

unsafe fn map_browser__run(browser: *mut map_browser) -> c_int {
    let mut key: c_int;

    if ui_browser__show(
        ptr::addr_of_mut!((*browser).b),
        dso__long_name(map__dso((*browser).map)),
        b"Press ESC to exit, %s / to search\0".as_ptr() as *const c_char,
        if verbose > 0 {
            b"\0".as_ptr() as *const c_char
        } else {
            b"restart with -v to use\0".as_ptr() as *const c_char
        },
    ) < 0
    {
        return -1;
    }

    loop {
        key = ui_browser__run(ptr::addr_of_mut!((*browser).b), 0);

        match key {
            x if x == b'/' as c_int => {
                if verbose > 0 {
                    map_browser__search(browser);
                }
                /* fall thru */
                ui_browser__warn_unhandled_hotkey(
                    ptr::addr_of_mut!((*browser).b),
                    key,
                    0,
                    ptr::null_mut(),
                );
                continue;
            }
            x if x == K_LEFT || x == K_ESC || x == b'q' as c_int || x == CTRL(b'c') => {
                break;
            }
            _ => {
                ui_browser__warn_unhandled_hotkey(
                    ptr::addr_of_mut!((*browser).b),
                    key,
                    0,
                    ptr::null_mut(),
                );
                continue;
            }
        }
    }

    ui_browser__hide(ptr::addr_of_mut!((*browser).b));
    key
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn map__browse(map: *mut map) -> c_int {
    let mut mb = map_browser {
        b: ui_browser {
            entries: dso__symbols(map__dso(map)),
            refresh: Some(ui_browser__rb_tree_refresh),
            seek: Some(ui_browser__rb_tree_seek),
            write: Some(map_browser__write),
            top: ptr::null_mut(),
            index: 0,
            top_idx: 0,
            nr_entries: 0,
            width: 0,
        },
        map,
        addrlen: 0,
    };
    let mut nd: *mut rb_node;
    let mut tmp = [0 as c_char; (size_of::<c_ulong>() * 8) / 4];
    let mut maxaddr: u64 = 0;

    nd = rb_first(mb.b.entries);
    while !nd.is_null() {
        let pos = rb_entry_symbol(nd);

        if maxaddr < (*pos).end {
            maxaddr = (*pos).end;
        }
        if verbose > 0 {
            let idx = symbol__browser_index(pos);
            *idx = mb.b.nr_entries;
        }
        mb.b.nr_entries += 1;

        nd = rb_next(nd);
    }

    mb.addrlen = snprintf(
        tmp.as_mut_ptr(),
        tmp.len(),
        b"%llx\0".as_ptr() as *const c_char,
        maxaddr,
    ) as u8;
    map_browser__run(ptr::addr_of_mut!(mb))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
