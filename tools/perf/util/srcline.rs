// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/srcline.c. Original C includes referenced:
// srcline.h, addr2line.h, dso.h, callchain.h, libbfd.h, llvm.h, symbol.h,
// libdw.h, debug.h, util.h, inttypes.h, string.h, linux/string.h,
// linux/zalloc.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

pub type u64 = u64;
pub type size_t = usize;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub namelen: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
    pub rb_parent_color: usize,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct inline_node {
    pub addr: u64,
    pub val: list_head,
    pub rb_node: rb_node,
}

#[repr(C)]
pub struct inline_list {
    pub list: list_head,
    pub symbol: *mut symbol,
    pub srcline: *mut c_char,
}

#[repr(C)]
pub struct srcline_node {
    pub addr: u64,
    pub srcline: *mut c_char,
    pub rb_node: rb_node,
}

pub const A2L_STYLE_UNKNOWN: c_int = 0;
pub const A2L_STYLE_LIBDW: c_int = 1;
pub const A2L_STYLE_LLVM: c_int = 2;
pub const A2L_STYLE_LIBBFD: c_int = 3;
pub const A2L_STYLE_CMD: c_int = 4;
pub const MAX_A2L_STYLE: usize = 5;

pub const ORDER_CALLEE: c_int = 1;

/*
 * Number of addr2line failures (without success) before disabling it for that
 * dso.
 */
pub const A2L_FAIL_LIMIT: c_int = 123;

#[repr(C)]
pub struct symbol_conf_t {
    pub addr2line_style: [c_int; MAX_A2L_STYLE],
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: c_int,
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;

    fn dso__symsrc_filename(dso: *mut dso) -> *const c_char;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__has_srcline(dso: *mut dso) -> bool;
    fn dso__set_has_srcline(dso: *mut dso, has_srcline: bool);
    fn dso__a2l_fails(dso: *mut dso) -> c_int;
    fn dso__set_a2l_fails(dso: *mut dso, fails: c_int);
    fn dso__free_a2l(dso: *mut dso);
    fn dso__demangle_sym(dso: *mut dso, kmodule: c_int, elf_name: *const c_char) -> *mut c_char;

    fn is_perf_pid_map_name(name: *const c_char) -> bool;
    fn perf_basename(path: *const c_char) -> *const c_char;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn pr_warning(fmt: *const c_char, ...);

    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);

    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool);
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);

    fn symbol__new(
        start: u64,
        len: u64,
        binding: c_int,
        type_: c_int,
        name: *const c_char,
    ) -> *mut symbol;
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn symbol__type(sym: *mut symbol) -> c_int;
    fn symbol__set_inlined(sym: *mut symbol, inlined: bool);
    fn symbol__inlined(sym: *mut symbol) -> bool;
    fn symbol__delete(sym: *mut symbol);

    fn libdw__addr2line(
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;
    fn llvm__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;
    fn libbfd__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;
    fn cmd__addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line_nr: *mut c_uint,
        dso: *mut dso,
        unwind_inlines: bool,
        node: *mut inline_node,
        sym: *mut symbol,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub static mut srcline_full_filename: bool = false;

static SRCLINE_UNKNOWN_BYTES: &[u8] = b"??:0\0";

#[unsafe(no_mangle)]
pub static mut srcline__unknown: *mut c_char = SRCLINE_UNKNOWN_BYTES.as_ptr() as *mut c_char;

const ADDR2LINE_STYLE_BYTES: &[u8] = b"addr2line.style\0";
const UNKNOWN_FUNC_BYTES: &[u8] = b"??\0";
const COLON_FMT_BYTES: &[u8] = b"%s:%u\0";
const PLUS_FMT_BYTES: &[u8] = b"%s+%llu\0";
const ADDR_FMT_BYTES: &[u8] = b"%s[%llx]\0";
const COMMA_BYTES: &[u8] = b",\0";
const LIBDW_BYTES: &[u8] = b"libdw\0";
const LLVM_BYTES: &[u8] = b"llvm\0";
const LIBBFD_BYTES: &[u8] = b"libbfd\0";
const CMD_BYTES: &[u8] = b"addr2line\0";
const UNKNOWN_A2L_STYLE_FMT_BYTES: &[u8] = b"Unknown addr2line style: %s\n\0";
const INLINE_NODE_MEM_BYTES: &[u8] = b"not enough memory for the inline node\0";
const SRCLINE_NODE_MEM_BYTES: &[u8] = b"not enough memory for the srcline node\0";

unsafe fn srcline_unknown() -> *mut c_char {
    srcline__unknown
}

unsafe fn rb_entry_srcline_node(ptr: *mut rb_node) -> *mut srcline_node {
    (ptr as *mut u8).sub(mem::offset_of!(srcline_node, rb_node)) as *mut srcline_node
}

unsafe fn rb_entry_inline_node(ptr: *mut rb_node) -> *mut inline_node {
    (ptr as *mut u8).sub(mem::offset_of!(inline_node, rb_node)) as *mut inline_node
}

unsafe fn list_entry_inline_list(ptr: *mut list_head) -> *mut inline_list {
    (ptr as *mut u8).sub(mem::offset_of!(inline_list, list)) as *mut inline_list
}

unsafe fn srcline_dso_name(dso: *mut dso) -> *const c_char {
    let dso_name: *const c_char;

    if !dso__symsrc_filename(dso).is_null() {
        dso_name = dso__symsrc_filename(dso);
    } else {
        dso_name = dso__long_name(dso);
    }

    if *dso_name == b'[' as c_char {
        return ptr::null();
    }

    if is_perf_pid_map_name(dso_name) {
        return ptr::null();
    }

    dso_name
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_list__append(
    symbol: *mut symbol,
    srcline: *mut c_char,
    node: *mut inline_node,
) -> c_int {
    let ilist = zalloc(mem::size_of::<inline_list>()) as *mut inline_list;
    if ilist.is_null() {
        return -1;
    }

    (*ilist).symbol = symbol;
    (*ilist).srcline = srcline;

    if callchain_param.order == ORDER_CALLEE {
        list_add_tail(&mut (*ilist).list, &mut (*node).val);
    } else {
        list_add(&mut (*ilist).list, &mut (*node).val);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_list__append_tail(
    symbol: *mut symbol,
    srcline: *mut c_char,
    node: *mut inline_node,
) -> c_int {
    let ilist = zalloc(mem::size_of::<inline_list>()) as *mut inline_list;
    if ilist.is_null() {
        return -1;
    }

    (*ilist).symbol = symbol;
    (*ilist).srcline = srcline;

    if callchain_param.order == ORDER_CALLEE {
        list_add(&mut (*ilist).list, &mut (*node).val);
    } else {
        list_add_tail(&mut (*ilist).list, &mut (*node).val);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn srcline_from_fileline(file: *const c_char, line: c_uint) -> *mut c_char {
    let mut srcline: *mut c_char = ptr::null_mut();
    let mut file = file;

    if file.is_null() {
        return ptr::null_mut();
    }

    if !srcline_full_filename {
        file = perf_basename(file);
    }

    if asprintf(&mut srcline, COLON_FMT_BYTES.as_ptr() as *const c_char, file, line) < 0 {
        return ptr::null_mut();
    }

    srcline
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_inline_sym(
    dso: *mut dso,
    base_sym: *mut symbol,
    funcname: *const c_char,
) -> *mut symbol {
    let mut funcname = funcname;
    let mut demangled: *mut c_char = ptr::null_mut();
    let inline_sym: *mut symbol;

    if funcname.is_null() {
        funcname = UNKNOWN_FUNC_BYTES.as_ptr() as *const c_char;
    }

    if !dso.is_null() {
        demangled = dso__demangle_sym(dso, 0, funcname);
        if !demangled.is_null() {
            funcname = demangled;
        }
    }

    if !base_sym.is_null() && strcmp(funcname, (*base_sym).name) == 0 {
        /* reuse the real, existing symbol */
        inline_sym = base_sym;
        /* ensure that we don't alias an inlined symbol, which could
         * lead to double frees in inline_node__delete
         */
        assert!(!symbol__inlined(base_sym));
    } else {
        /* create a fake symbol for the inline frame */
        inline_sym = symbol__new(
            if !base_sym.is_null() { (*base_sym).start } else { 0 },
            if !base_sym.is_null() {
                (*base_sym).end.wrapping_sub((*base_sym).start)
            } else {
                0
            },
            if !base_sym.is_null() {
                symbol__binding(base_sym)
            } else {
                0
            },
            if !base_sym.is_null() {
                symbol__type(base_sym)
            } else {
                0
            },
            funcname,
        );
        if !inline_sym.is_null() {
            symbol__set_inlined(inline_sym, true);
        }
    }

    free(demangled as *mut c_void);

    inline_sym
}

unsafe fn addr2line(
    dso_name: *const c_char,
    addr: u64,
    file: *mut *mut c_char,
    line_nr: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    let mut ret: c_int = 0;

    if symbol_conf.addr2line_style[0] == A2L_STYLE_UNKNOWN {
        let mut i: usize = 0;

        /* Default addr2line fallback order.
         * Original C conditionally adds LIBDW, LLVM, and LIBBFD under
         * HAVE_LIBDW_SUPPORT, HAVE_LIBLLVM_SUPPORT, and HAVE_LIBBFD_SUPPORT.
         */
        symbol_conf.addr2line_style[i] = A2L_STYLE_LIBDW;
        i += 1;
        symbol_conf.addr2line_style[i] = A2L_STYLE_LLVM;
        i += 1;
        symbol_conf.addr2line_style[i] = A2L_STYLE_LIBBFD;
        i += 1;
        symbol_conf.addr2line_style[i] = A2L_STYLE_CMD;
    }

    for i in 0..symbol_conf.addr2line_style.len() {
        match symbol_conf.addr2line_style[i] {
            A2L_STYLE_LIBDW => {
                ret = libdw__addr2line(addr, file, line_nr, dso, unwind_inlines, node, sym);
            }
            A2L_STYLE_LLVM => {
                ret = llvm__addr2line(dso_name, addr, file, line_nr, dso, unwind_inlines, node, sym);
            }
            A2L_STYLE_LIBBFD => {
                ret = libbfd__addr2line(dso_name, addr, file, line_nr, dso, unwind_inlines, node, sym);
            }
            A2L_STYLE_CMD => {
                ret = cmd__addr2line(dso_name, addr, file, line_nr, dso, unwind_inlines, node, sym);
            }
            A2L_STYLE_UNKNOWN | _ => {}
        }
        if ret > 0 {
            return ret;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn addr2line_configure(
    var: *const c_char,
    value: *const c_char,
    _cb: *mut c_void,
) -> c_int {
    let a2l_style_names: [*const c_char; MAX_A2L_STYLE + 1] = [
        ptr::null(),
        LIBDW_BYTES.as_ptr() as *const c_char,
        LLVM_BYTES.as_ptr() as *const c_char,
        LIBBFD_BYTES.as_ptr() as *const c_char,
        CMD_BYTES.as_ptr() as *const c_char,
        ptr::null(),
    ];

    let mut i: size_t = 0;

    if strcmp(var, ADDR2LINE_STYLE_BYTES.as_ptr() as *const c_char) != 0 {
        return 0;
    }

    if value.is_null() {
        return -1;
    }

    let s = strdup(value);
    if s.is_null() {
        return -1;
    }

    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut p = strtok_r(s, COMMA_BYTES.as_ptr() as *const c_char, &mut saveptr);
    while !p.is_null() && i < symbol_conf.addr2line_style.len() {
        let mut found = false;
        let q = strim(p);

        for j in A2L_STYLE_LIBDW as usize..MAX_A2L_STYLE {
            if strcasecmp(q, a2l_style_names[j]) == 0 {
                symbol_conf.addr2line_style[i] = j as c_int;
                i += 1;
                found = true;
                break;
            }
        }
        if !found {
            pr_warning(UNKNOWN_A2L_STYLE_FMT_BYTES.as_ptr() as *const c_char, q);
        }
        p = strtok_r(ptr::null_mut(), COMMA_BYTES.as_ptr() as *const c_char, &mut saveptr);
    }

    free(s as *mut c_void);
    0
}

unsafe fn addr2inlines(
    dso_name: *const c_char,
    addr: u64,
    dso: *mut dso,
    sym: *mut symbol,
) -> *mut inline_node {
    let node = zalloc(mem::size_of::<inline_node>()) as *mut inline_node;
    if node.is_null() {
        perror(INLINE_NODE_MEM_BYTES.as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*node).val);
    (*node).addr = addr;

    addr2line(
        dso_name,
        addr,
        ptr::null_mut(),
        ptr::null_mut(),
        dso,
        true,
        node,
        sym,
    );

    node
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __get_srcline(
    dso: *mut dso,
    addr: u64,
    sym: *mut symbol,
    show_sym: bool,
    show_addr: bool,
    unwind_inlines: bool,
    ip: u64,
) -> *mut c_char {
    let mut file: *mut c_char = ptr::null_mut();
    let mut line: c_uint = 0;
    let mut srcline: *mut c_char = ptr::null_mut();
    let dso_name: *const c_char;

    if !dso__has_srcline(dso) {
        return out(dso, addr, sym, show_sym, show_addr, ip);
    }

    dso_name = srcline_dso_name(dso);
    if dso_name.is_null() {
        return out_err(dso, addr, sym, show_sym, show_addr, ip);
    }

    if addr2line(
        dso_name,
        addr,
        &mut file,
        &mut line,
        dso,
        unwind_inlines,
        ptr::null_mut(),
        sym,
    ) == 0
    {
        return out_err(dso, addr, sym, show_sym, show_addr, ip);
    }

    srcline = srcline_from_fileline(file, line);
    free(file as *mut c_void);

    if srcline.is_null() {
        return out_err(dso, addr, sym, show_sym, show_addr, ip);
    }

    dso__set_a2l_fails(dso, 0);

    srcline
}

unsafe fn out_err(
    dso: *mut dso,
    addr: u64,
    sym: *mut symbol,
    show_sym: bool,
    show_addr: bool,
    ip: u64,
) -> *mut c_char {
    dso__set_a2l_fails(dso, dso__a2l_fails(dso) + 1);
    if dso__a2l_fails(dso) > A2L_FAIL_LIMIT {
        dso__set_has_srcline(dso, false);
        dso__free_a2l(dso);
    }
    out(dso, addr, sym, show_sym, show_addr, ip)
}

unsafe fn out(
    dso: *mut dso,
    addr: u64,
    sym: *mut symbol,
    show_sym: bool,
    show_addr: bool,
    ip: u64,
) -> *mut c_char {
    let mut srcline: *mut c_char = ptr::null_mut();

    if !show_addr {
        return if show_sym && !sym.is_null() {
            strndup((*sym).name, (*sym).namelen as size_t)
        } else {
            srcline_unknown()
        };
    }

    if !sym.is_null() {
        if asprintf(
            &mut srcline,
            PLUS_FMT_BYTES.as_ptr() as *const c_char,
            if show_sym { (*sym).name } else { b"\0".as_ptr() as *const c_char },
            ip.wrapping_sub((*sym).start),
        ) < 0
        {
            return srcline_unknown();
        }
    } else if asprintf(
        &mut srcline,
        ADDR_FMT_BYTES.as_ptr() as *const c_char,
        dso__short_name(dso),
        addr,
    ) < 0
    {
        return srcline_unknown();
    }
    srcline
}

/* Returns filename and fills in line number in line */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_srcline_split(
    dso: *mut dso,
    addr: u64,
    line: *mut c_uint,
) -> *mut c_char {
    let mut file: *mut c_char = ptr::null_mut();
    let dso_name: *const c_char;

    if !dso__has_srcline(dso) {
        return ptr::null_mut();
    }

    dso_name = srcline_dso_name(dso);
    if dso_name.is_null() {
        return get_srcline_split_out_err(dso);
    }

    if addr2line(
        dso_name,
        addr,
        &mut file,
        line,
        dso,
        true,
        ptr::null_mut(),
        ptr::null_mut(),
    ) == 0
    {
        return get_srcline_split_out_err(dso);
    }

    dso__set_a2l_fails(dso, 0);
    file
}

unsafe fn get_srcline_split_out_err(dso: *mut dso) -> *mut c_char {
    dso__set_a2l_fails(dso, dso__a2l_fails(dso) + 1);
    if dso__a2l_fails(dso) > A2L_FAIL_LIMIT {
        dso__set_has_srcline(dso, false);
        dso__free_a2l(dso);
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zfree_srcline(srcline: *mut *mut c_char) {
    if (*srcline).is_null() {
        return;
    }

    if *srcline != srcline_unknown() {
        free(*srcline as *mut c_void);
    }

    *srcline = ptr::null_mut();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_srcline(
    dso: *mut dso,
    addr: u64,
    sym: *mut symbol,
    show_sym: bool,
    show_addr: bool,
    ip: u64,
) -> *mut c_char {
    __get_srcline(dso, addr, sym, show_sym, show_addr, false, ip)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn srcline__tree_insert(
    tree: *mut rb_root_cached,
    addr: u64,
    srcline: *mut c_char,
) {
    let mut p: *mut *mut rb_node = &mut (*tree).rb_root.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut leftmost = true;

    let node = zalloc(mem::size_of::<srcline_node>()) as *mut srcline_node;
    if node.is_null() {
        perror(SRCLINE_NODE_MEM_BYTES.as_ptr() as *const c_char);
        return;
    }

    (*node).addr = addr;
    (*node).srcline = srcline;

    while !(*p).is_null() {
        parent = *p;
        let i = rb_entry_srcline_node(parent);
        if addr < (*i).addr {
            p = &mut (**p).rb_left;
        } else {
            p = &mut (**p).rb_right;
            leftmost = false;
        }
    }
    rb_link_node(&mut (*node).rb_node, parent, p);
    rb_insert_color_cached(&mut (*node).rb_node, tree, leftmost);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn srcline__tree_find(
    tree: *mut rb_root_cached,
    addr: u64,
) -> *mut c_char {
    let mut n = (*tree).rb_root.rb_node;

    while !n.is_null() {
        let i = rb_entry_srcline_node(n);

        if addr < (*i).addr {
            n = (*n).rb_left;
        } else if addr > (*i).addr {
            n = (*n).rb_right;
        } else {
            return (*i).srcline;
        }
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn srcline__tree_delete(tree: *mut rb_root_cached) {
    let mut next = rb_first_cached(tree);

    while !next.is_null() {
        let pos = rb_entry_srcline_node(next);
        next = rb_next(&mut (*pos).rb_node);
        rb_erase_cached(&mut (*pos).rb_node, tree);
        zfree_srcline(&mut (*pos).srcline);
        let mut pos_void = pos as *mut c_void;
        zfree(&mut pos_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__parse_addr_inlines(
    dso: *mut dso,
    addr: u64,
    sym: *mut symbol,
) -> *mut inline_node {
    let dso_name = srcline_dso_name(dso);
    if dso_name.is_null() {
        return ptr::null_mut();
    }

    addr2inlines(dso_name, addr, dso, sym)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_node__clear_frames(node: *mut inline_node) {
    if node.is_null() {
        return;
    }

    let head = &mut (*node).val as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let ilist = list_entry_inline_list(pos);
        let tmp = (*pos).next;
        list_del_init(&mut (*ilist).list);
        zfree_srcline(&mut (*ilist).srcline);
        /* only the inlined symbols are owned by the list */
        if !(*ilist).symbol.is_null() && symbol__inlined((*ilist).symbol) {
            symbol__delete((*ilist).symbol);
        }
        free(ilist as *mut c_void);
        pos = tmp;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_node__delete(node: *mut inline_node) {
    inline_node__clear_frames(node);
    free(node as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inlines__tree_insert(
    tree: *mut rb_root_cached,
    inlines: *mut inline_node,
) {
    let mut p: *mut *mut rb_node = &mut (*tree).rb_root.rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let addr = (*inlines).addr;
    let mut leftmost = true;

    while !(*p).is_null() {
        parent = *p;
        let i = rb_entry_inline_node(parent);
        if addr < (*i).addr {
            p = &mut (**p).rb_left;
        } else {
            p = &mut (**p).rb_right;
            leftmost = false;
        }
    }
    rb_link_node(&mut (*inlines).rb_node, parent, p);
    rb_insert_color_cached(&mut (*inlines).rb_node, tree, leftmost);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inlines__tree_find(
    tree: *mut rb_root_cached,
    addr: u64,
) -> *mut inline_node {
    let mut n = (*tree).rb_root.rb_node;

    while !n.is_null() {
        let i = rb_entry_inline_node(n);

        if addr < (*i).addr {
            n = (*n).rb_left;
        } else if addr > (*i).addr {
            n = (*n).rb_right;
        } else {
            return i;
        }
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inlines__tree_delete(tree: *mut rb_root_cached) {
    let mut next = rb_first_cached(tree);

    while !next.is_null() {
        let pos = rb_entry_inline_node(next);
        next = rb_next(&mut (*pos).rb_node);
        rb_erase_cached(&mut (*pos).rb_node, tree);
        inline_node__delete(pos);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
