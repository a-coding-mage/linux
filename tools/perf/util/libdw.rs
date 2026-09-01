// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/libdw.c. C include dependencies:
// "dso.h", "libdw.h", "srcline.h", "symbol.h", "dwarf-aux.h",
// "callchain.h", <fcntl.h>, <unistd.h>, <elfutils/libdwfl.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u64 = u64;
pub type Dwarf_Addr = u64;

const O_RDONLY: c_int = 0;
const ENOMEM: c_int = 12;
const DWARF_CB_ABORT: c_int = -1;
const DW_TAG_SUBPROGRAM: c_int = 0x2e;
const ORDER_CALLEE: c_int = 1;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct inline_node {
    pub val: list_head,
}

#[repr(C)]
pub struct inline_list {
    pub list: list_head,
    pub symbol: *mut symbol,
    pub srcline: *mut c_char,
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: c_int,
}

#[repr(C)]
pub struct Dwfl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf_Die {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Callbacks {
    pub find_debuginfo: Option<unsafe extern "C" fn()>,
    pub section_address: Option<unsafe extern "C" fn()>,
    pub find_elf: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut srcline__unknown: *mut c_char;
    static mut callchain_param: callchain_param_t;

    static dwfl_standard_find_debuginfo: unsafe extern "C" fn();
    static dwfl_offline_section_address: unsafe extern "C" fn();
    static dwfl_build_id_find_elf: unsafe extern "C" fn();

    fn dso__libdw(dso: *mut dso) -> *mut Dwfl;
    fn dso__set_libdw(dso: *mut dso, dwfl: *mut Dwfl);
    fn dso__long_name(dso: *mut dso) -> *const c_char;

    fn dwfl_end(dwfl: *mut Dwfl);
    fn dwfl_begin(callbacks: *const Dwfl_Callbacks) -> *mut Dwfl;
    fn dwfl_report_offline(
        dwfl: *mut Dwfl,
        name: *const c_char,
        file_name: *const c_char,
        fd: c_int,
    ) -> *mut Dwfl_Module;
    fn dwfl_report_end(dwfl: *mut Dwfl, removed: *mut c_void, arg: *mut c_void) -> c_int;
    fn dwfl_addrmodule(dwfl: *mut Dwfl, address: Dwarf_Addr) -> *mut Dwfl_Module;
    fn dwfl_module_getdwarf(mod_: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Dwarf_Die;
    fn dwfl_module_getsrc(mod_: *mut Dwfl_Module, addr: Dwarf_Addr) -> *mut Dwfl_Line;
    fn dwfl_lineinfo(
        line: *mut Dwfl_Line,
        addr: *mut Dwarf_Addr,
        linep: *mut c_int,
        colp: *mut c_int,
        mtime: *mut c_void,
        length: *mut c_void,
    ) -> *const c_char;
    fn dwfl_module_addrdie(
        mod_: *mut Dwfl_Module,
        addr: Dwarf_Addr,
        bias: *mut Dwarf_Addr,
    ) -> *mut Dwarf_Die;

    fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;

    fn die_get_call_file(die: *mut Dwarf_Die) -> *const c_char;
    fn die_get_call_lineno(die: *mut Dwarf_Die) -> c_int;
    fn die_get_linkage_name(die: *mut Dwarf_Die) -> *const c_char;
    fn die_name(die: *mut Dwarf_Die) -> *const c_char;
    fn dwarf_tag(die: *mut Dwarf_Die) -> c_int;

    fn srcline_from_fileline(file: *const c_char, line: c_int) -> *mut c_char;
    fn new_inline_sym(
        dso: *mut dso,
        base_sym: *mut symbol,
        funcname: *const c_char,
    ) -> *mut symbol;
    fn inline_list__append_tail(
        sym: *mut symbol,
        srcline: *mut c_char,
        node: *mut inline_node,
    ) -> c_int;
    fn symbol__inlined(sym: *mut symbol) -> bool;
    fn symbol__delete(sym: *mut symbol);
    fn inline_node__clear_frames(node: *mut inline_node);
    fn cu_walk_functions_at(
        cudie: *mut Dwarf_Die,
        addr: u64,
        cb: Option<unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int>,
        arg: *mut c_void,
    );
}

static offline_callbacks: Dwfl_Callbacks = Dwfl_Callbacks {
    find_debuginfo: Some(dwfl_standard_find_debuginfo),
    section_address: Some(dwfl_offline_section_address),
    find_elf: Some(dwfl_build_id_find_elf),
};

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn list_entry(ptr: *mut list_head) -> *mut inline_list {
    let offset = core::mem::offset_of!(inline_list, list);
    (ptr as *mut u8).sub(offset) as *mut inline_list
}

unsafe fn list_first_entry(head: *mut list_head) -> *mut inline_list {
    list_entry((*head).next)
}

unsafe fn list_last_entry(head: *mut list_head) -> *mut inline_list {
    list_entry((*head).prev)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__free_libdw(dso: *mut dso) {
    let dwfl = dso__libdw(dso);

    if !dwfl.is_null() {
        dwfl_end(dwfl);
        dso__set_libdw(dso, core::ptr::null_mut());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__libdw_dwfl(dso: *mut dso) -> *mut Dwfl {
    let mut dwfl = dso__libdw(dso);
    let dso_name: *const c_char;
    let mod_: *mut Dwfl_Module;
    let fd: c_int;

    if !dwfl.is_null() {
        return dwfl;
    }

    dso_name = dso__long_name(dso);
    /*
     * Initialize Dwfl session.
     * We need to open the DSO file to report it to libdw.
     */
    fd = open(dso_name, O_RDONLY);
    if fd < 0 {
        return core::ptr::null_mut();
    }

    dwfl = dwfl_begin(&offline_callbacks);
    if dwfl.is_null() {
        close(fd);
        return core::ptr::null_mut();
    }

    /*
     * If the report is successful, the file descriptor fd is consumed
     * and closed by the Dwfl. If not, it is not closed.
     */
    mod_ = dwfl_report_offline(dwfl, dso_name, dso_name, fd);
    if mod_.is_null() {
        dwfl_end(dwfl);
        close(fd);
        return core::ptr::null_mut();
    }

    if dwfl_report_end(dwfl, core::ptr::null_mut(), core::ptr::null_mut()) != 0 {
        dwfl_end(dwfl);
        return core::ptr::null_mut();
    }
    dso__set_libdw(dso, dwfl);

    dwfl
}

#[repr(C)]
struct libdw_a2l_cb_args {
    dso: *mut dso,
    sym: *mut symbol,
    node: *mut inline_node,
    leaf_srcline: *mut c_char,
    leaf_srcline_used: bool,
    err: c_int,
}

unsafe extern "C" fn libdw_a2l_cb(die: *mut Dwarf_Die, _args: *mut c_void) -> c_int {
    let args = _args as *mut libdw_a2l_cb_args;
    let call_fname = die_get_call_file(die);
    let call_lineno = die_get_call_lineno(die);
    let mut call_srcline = srcline__unknown;
    let inline_sym: *mut symbol;

    if dwarf_tag(die) == DW_TAG_SUBPROGRAM && !(*args).sym.is_null() {
        /*
         * cu_walk_functions_at() opens the walk with the
         * containing DW_TAG_subprogram DIE (the non-inlined outer
         * function). That's just the base symbol -- use it
         * directly. Avoids a fragile name-vs-name compare in
         * new_inline_sym() that misfires when GCC IPA passes
         * (.isra/.constprop/.part/.cold) rename the ELF symbol
         * while DWARF keeps the pre-clone linkage name, which
         * left the outer frame spuriously tagged "(inlined)".
         */
        inline_sym = (*args).sym;
    } else {
        /*
         * Prefer DW_AT_linkage_name so C++ inline frames keep
         * their namespace/class qualification. new_inline_sym()
         * runs the name through dso__demangle_sym(), so the
         * mangled linkage name is turned back into
         * "Namespace::Class::method". Fall back to DW_AT_name
         * (unqualified) when no linkage name is present, e.g.
         * for C code or extern "C" functions.
         */
        let mut funcname = die_get_linkage_name(die);
        if funcname.is_null() {
            funcname = die_name(die);
        }

        inline_sym = new_inline_sym((*args).dso, (*args).sym, funcname);
        if inline_sym.is_null() {
            (*args).err = -ENOMEM;
            return DWARF_CB_ABORT;
        }
    }

    /* Assign caller information to the parent. */
    if !call_fname.is_null() {
        call_srcline = srcline_from_fileline(
            call_fname,
            if call_lineno >= 0 { call_lineno } else { 0 },
        );
    }

    if !list_empty(&mut (*(*args).node).val) {
        let parent: *mut inline_list;

        if callchain_param.order == ORDER_CALLEE {
            parent = list_first_entry(&mut (*(*args).node).val);
        } else {
            parent = list_last_entry(&mut (*(*args).node).val);
        }

        if (*args).leaf_srcline == (*parent).srcline {
            (*args).leaf_srcline_used = false;
        } else if (*parent).srcline != srcline__unknown {
            free((*parent).srcline as *mut c_void);
        }
        (*parent).srcline = call_srcline;
        call_srcline = core::ptr::null_mut();
    }
    if !call_srcline.is_null() && call_srcline != srcline__unknown {
        free(call_srcline as *mut c_void);
    }

    /* Add this symbol to the chain as the leaf. */
    if !(*args).leaf_srcline_used {
        if inline_list__append_tail(inline_sym, (*args).leaf_srcline, (*args).node) != 0 {
            if symbol__inlined(inline_sym) {
                symbol__delete(inline_sym);
            }
            (*args).err = -ENOMEM;
            return DWARF_CB_ABORT;
        }
        (*args).leaf_srcline_used = true;
    } else {
        let srcline = strdup((*args).leaf_srcline);

        if srcline.is_null() {
            if symbol__inlined(inline_sym) {
                symbol__delete(inline_sym);
            }
            (*args).err = -ENOMEM;
            return DWARF_CB_ABORT;
        }
        if inline_list__append_tail(inline_sym, srcline, (*args).node) != 0 {
            free(srcline as *mut c_void);
            if symbol__inlined(inline_sym) {
                symbol__delete(inline_sym);
            }
            (*args).err = -ENOMEM;
            return DWARF_CB_ABORT;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libdw__addr2line(
    addr: u64,
    file: *mut *mut c_char,
    line_nr: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    let dwfl = dso__libdw_dwfl(dso);
    let mod_: *mut Dwfl_Module;
    let dwline: *mut Dwfl_Line;
    let mut bias: Dwarf_Addr = 0;
    let src: *const c_char;
    let mut lineno: c_int = 0;

    if dwfl.is_null() {
        return 0;
    }

    mod_ = dwfl_addrmodule(dwfl, addr);
    if mod_.is_null() {
        return 0;
    }

    /*
     * Get/ignore the dwarf information. Determine the bias, difference
     * between the regular ELF addr2line addresses and those to use with
     * libdw.
     */
    if dwfl_module_getdwarf(mod_, &mut bias).is_null() {
        return 0;
    }

    /* Find source line information for the address. */
    dwline = dwfl_module_getsrc(mod_, addr.wrapping_add(bias));
    if dwline.is_null() {
        return 0;
    }

    /* Get line information. */
    src = dwfl_lineinfo(
        dwline,
        core::ptr::null_mut(),
        &mut lineno,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );

    if !file.is_null() {
        *file = if !src.is_null() {
            strdup(src)
        } else {
            core::ptr::null_mut()
        };
    }
    if !line_nr.is_null() {
        *line_nr = lineno as c_uint;
    }

    /* Optionally unwind inline function call chain. */
    if unwind_inlines && !node.is_null() {
        let mut unused_bias: Dwarf_Addr = 0;
        let cudie = dwfl_module_addrdie(mod_, addr.wrapping_add(bias), &mut unused_bias);
        let unknown = b"<unknown>\0";
        let mut args = libdw_a2l_cb_args {
            dso,
            sym,
            node,
            leaf_srcline: srcline_from_fileline(
                if !src.is_null() {
                    src
                } else {
                    unknown.as_ptr() as *const c_char
                },
                lineno,
            ),
            leaf_srcline_used: false,
            err: 0,
        };

        if args.leaf_srcline.is_null() {
            if !file.is_null() && !(*file).is_null() {
                free(*file as *mut c_void);
                *file = core::ptr::null_mut();
            }
            return 0;
        }

        /* Walk from the parent down to the leaf. */
        if !cudie.is_null() {
            cu_walk_functions_at(
                cudie,
                addr,
                Some(libdw_a2l_cb),
                &mut args as *mut libdw_a2l_cb_args as *mut c_void,
            );
        }

        if !args.leaf_srcline_used {
            free(args.leaf_srcline as *mut c_void);
        }

        if args.err != 0 {
            if !file.is_null() && !(*file).is_null() {
                free(*file as *mut c_void);
                *file = core::ptr::null_mut();
            }
            inline_node__clear_frames(node);
            return 0;
        }
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
