// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

// C dependencies: fcntl.h, getopt.h, errno.h, stdarg.h, string.h, unistd.h,
// and gendwarfksyms.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
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
pub struct Dwarf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Dwarf_CU {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Dwarf_Die {
    _private: [u8; 0],
}
pub type Dwarf_Addr = u64;

#[repr(C)]
pub struct Dwfl_Callbacks {
    pub section_address: Option<unsafe extern "C" fn(*mut Dwfl_Module, *mut Dwarf_Addr) -> Dwarf_Addr>,
    pub find_debuginfo: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, shortopts: *const c_char,
                   longopts: *const Option, longindex: *mut c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;

    fn process_cu(cudie: *mut Dwarf_Die);
    fn generate_symtypes_and_versions(symfile: *mut FILE);
    fn die_map_free();
    fn symbol_read_exports(file: *mut FILE) -> c_int;
    fn symbol_read_symtab(fd: c_int);
    fn kabi_read_rules(fd: c_int);
    fn kabi_free();
    fn symbol_print_versions();
    fn symbol_free();
    fn error(format: *const c_char, ...);
    fn debug(format: *const c_char, ...);
    fn check(value: c_int);

    fn dwfl_offline_section_address() -> Dwarf_Addr;
    fn dwfl_standard_find_debuginfo();
    fn dwfl_module_getdwarf(mod: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Dwarf;
    fn dwarf_get_units(dbg: *mut Dwarf, cu: *mut Dwarf_CU, nextcu: *mut *mut Dwarf_CU,
                       a: *mut c_void, b: *mut c_void, cudie: *mut Dwarf_Die,
                       c: *mut c_void) -> c_int;
    fn dwfl_begin(callbacks: *const Dwfl_Callbacks) -> *mut Dwfl;
    fn dwfl_report_offline(dwfl: *mut Dwfl, name: *const c_char,
                           file_name: *const c_char, fd: c_int) -> *mut Dwfl_Module;
    fn dwfl_report_end(dwfl: *mut Dwfl, a: *mut c_void, b: *mut c_void);
    fn dwfl_getmodules(dwfl: *mut Dwfl,
                       callback: unsafe extern "C" fn(*mut Dwfl_Module, *mut *mut c_void,
                                                       *const c_char, Dwarf_Addr, *mut c_void) -> c_int,
                       arg: *mut c_void, offset: c_int) -> c_int;
    fn dwfl_end(dwfl: *mut Dwfl);
    fn dwarf_errmsg(error: c_int) -> *const c_char;
}

/* Options */

/* Print debugging information to stderr */
pub static mut debug: c_int = 0;
/* Dump DIE contents */
pub static mut dump_dies: c_int = 0;
/* Print debugging information about die_map changes */
pub static mut dump_die_map: c_int = 0;
/* Print out type strings (i.e. type_map) */
pub static mut dump_types: c_int = 0;
/* Print out expanded type strings used for symbol versions */
pub static mut dump_versions: c_int = 0;
/* Support kABI stability features */
pub static mut stable: c_int = 0;
/* Write a symtypes file */
pub static mut symtypes: c_int = 0;
static mut symtypes_file: *const c_char = core::ptr::null();

unsafe fn usage() {
    fputs(b"Usage: gendwarfksyms [options] elf-object-file ... < symbol-list\n\nOptions:\n  -d, --debug          Print debugging information\n      --dump-dies      Dump DWARF DIE contents\n      --dump-die-map   Print debugging information about die_map changes\n      --dump-types     Dump type strings\n      --dump-versions  Dump expanded type strings used for symbol versions\n  -s, --stable         Support kABI stability features\n  -T, --symtypes file  Write a symtypes file\n  -h, --help           Print this message\n\n\0".as_ptr() as *const c_char, stderr);
}

unsafe extern "C" fn process_module(mod_: *mut Dwfl_Module, _userdata: *mut *mut c_void,
                                     name: *const c_char, _base: Dwarf_Addr,
                                     arg: *mut c_void) -> c_int {
    let mut dwbias: Dwarf_Addr = 0;
    let mut cudie = Dwarf_Die { _private: [] };
    let mut cu: *mut Dwarf_CU = core::ptr::null_mut();
    let dbg: *mut Dwarf;
    let symfile = arg as *mut FILE;
    let mut res: c_int;

    debug(b"%s\0".as_ptr() as *const c_char, name);
    dbg = dwfl_module_getdwarf(mod_, &mut dwbias);

    /*
     * Look for exported symbols in each CU, follow the DIE tree, and add
     * the entries to die_map.
     */
    loop {
        res = dwarf_get_units(dbg, cu, &mut cu, core::ptr::null_mut(), core::ptr::null_mut(),
                              &mut cudie, core::ptr::null_mut());
        if res < 0 {
            error(b"dwarf_get_units failed: no debugging information?\0".as_ptr() as *const c_char);
        }
        if res == 1 {
            break; /* No more units */
        }

        process_cu(&mut cudie);
        if cu.is_null() {
            break;
        }
    }

    /*
     * Use die_map to expand type strings, write them to `symfile`, and
     * calculate symbol versions.
     */
    generate_symtypes_and_versions(symfile);
    die_map_free();

    0
}

static callbacks: Dwfl_Callbacks = Dwfl_Callbacks {
    section_address: Some(dwfl_offline_section_address),
    find_debuginfo: Some(dwfl_standard_find_debuginfo),
};

#[repr(C)]
struct Option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut symfile: *mut FILE = core::ptr::null_mut();
    let mut n: c_int;
    let mut opt: c_int;

    static mut opts: [Option; 9] = [
        Option { name: b"debug\0".as_ptr() as *const c_char, has_arg: 0, flag: core::ptr::null_mut(), val: 'd' as c_int },
        Option { name: b"dump-dies\0".as_ptr() as *const c_char, has_arg: 0, flag: &raw mut dump_dies, val: 1 },
        Option { name: b"dump-die-map\0".as_ptr() as *const c_char, has_arg: 0, flag: &raw mut dump_die_map, val: 1 },
        Option { name: b"dump-types\0".as_ptr() as *const c_char, has_arg: 0, flag: &raw mut dump_types, val: 1 },
        Option { name: b"dump-versions\0".as_ptr() as *const c_char, has_arg: 0, flag: &raw mut dump_versions, val: 1 },
        Option { name: b"stable\0".as_ptr() as *const c_char, has_arg: 0, flag: core::ptr::null_mut(), val: 's' as c_int },
        Option { name: b"symtypes\0".as_ptr() as *const c_char, has_arg: 1, flag: core::ptr::null_mut(), val: 'T' as c_int },
        Option { name: b"help\0".as_ptr() as *const c_char, has_arg: 0, flag: core::ptr::null_mut(), val: 'h' as c_int },
        Option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
    ];

    loop {
        opt = getopt_long(argc, argv, b"dsT:h\0".as_ptr() as *const c_char, opts.as_ptr(), core::ptr::null_mut());
        if opt == -1 { break; }
        match opt {
            0 => (),
            x if x == 'd' as c_int => debug = 1,
            x if x == 's' as c_int => stable = 1,
            x if x == 'T' as c_int => { symtypes = 1; symtypes_file = optarg; },
            x if x == 'h' as c_int => { usage(); return 0; },
            _ => { usage(); return 1; },
        }
    }

    if dump_die_map != 0 { dump_dies = 1; }
    if optind >= argc { usage(); error(b"no input files?\0".as_ptr() as *const c_char); }
    if symbol_read_exports(stdin) == 0 { return 0; }

    if !symtypes_file.is_null() {
        symfile = fopen(symtypes_file, b"w\0".as_ptr() as *const c_char);
        if symfile.is_null() { error(b"fopen failed for '%s': %s\0".as_ptr() as *const c_char, symtypes_file, strerror(0)); }
    }

    n = optind;
    while n < argc {
        let path = *argv.add(n as usize);
        let fd = open(path, 0, 0);
        if fd == -1 { error(b"open failed for '%s': %s\0".as_ptr() as *const c_char, path, strerror(0)); }
        symbol_read_symtab(fd);
        kabi_read_rules(fd);
        let dwfl = dwfl_begin(&callbacks);
        if dwfl.is_null() { error(b"dwfl_begin failed for '%s': %s\0".as_ptr() as *const c_char, path, dwarf_errmsg(-1)); }
        if dwfl_report_offline(dwfl, path, path, fd).is_null() { error(b"dwfl_report_offline failed for '%s': %s\0".as_ptr() as *const c_char, path, dwarf_errmsg(-1)); }
        dwfl_report_end(dwfl, core::ptr::null_mut(), core::ptr::null_mut());
        if dwfl_getmodules(dwfl, process_module, symfile as *mut c_void, 0) != 0 { error(b"dwfl_getmodules failed for '%s'\0".as_ptr() as *const c_char, path); }
        dwfl_end(dwfl);
        kabi_free();
        n += 1;
    }
    if !symfile.is_null() { check(fclose(symfile)); }
    symbol_print_versions();
    symbol_free();
    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int { main_impl(argc, argv) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
