// SPDX-License-Identifier: GPL-2.0
//
// Generic support for BUG(). Translated directly from bug.c.
// Kernel includes and configuration macros are supplied by the surrounding build.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct bug_entry {
    pub bug_addr: usize,
    pub bug_addr_disp: i32,
    pub file: *const c_char,
    pub file_disp: i32,
    pub line: u32,
    pub format: *const c_char,
    pub format_disp: i32,
    pub flags: u32,
}

#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct module { pub bug_table: *mut bug_entry, pub num_bugs: usize, pub bug_list: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct Elf_Ehdr { pub e_shstrndx: u16, pub e_shnum: u16 }
#[repr(C)] pub struct Elf_Shdr { pub sh_offset: usize, pub sh_name: u32, pub sh_addr: usize, pub sh_size: usize }
#[repr(C)] pub struct arch_va_list { _private: [u8; 0] }

extern "C" {
    static __start___bug_table: *mut bug_entry;
    static __stop___bug_table: *mut bug_entry;
    fn is_valid_bugaddr(bugaddr: usize) -> bool;
    fn kunit_is_suppressed_warning(value: bool) -> bool;
    fn disable_trace_on_warning();
    fn warn_rcu_enter() -> bool;
    fn warn_rcu_exit(rcu: bool);
    fn __warn(file: *const c_char, line: u32, addr: *mut c_void, taint: u32,
              regs: *mut pt_regs, arg: *mut c_void);
    fn vprintk(fmt: *const c_char, args: *mut c_void);
    fn __warn_args(args: *mut arch_va_list, regs: *mut pt_regs) -> *mut c_void;
}

pub const BUGFLAG_WARNING: u32 = 1 << 0;
pub const BUGFLAG_ONCE: u32 = 1 << 1;
pub const BUGFLAG_DONE: u32 = 1 << 2;
pub const BUGFLAG_NO_CUT_HERE: u32 = 1 << 3;
pub const BUGFLAG_ARGS: u32 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bug_trap_type { None, Warn, Bug }

unsafe fn bug_addr(bug: *const bug_entry) -> usize {
    #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)]
    { (core::ptr::addr_of!((*bug).bug_addr_disp) as usize).wrapping_add((*bug).bug_addr_disp as usize) }
    #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))]
    { (*bug).bug_addr }
}

#[cfg(CONFIG_MODULES)]
static mut module_bug_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[cfg(CONFIG_MODULES)]
unsafe fn module_find_bug(bugaddr: usize) -> *mut bug_entry {
    // list_for_each_entry_rcu and guard(rcu) are provided by the kernel environment.
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_MODULES))]
unsafe fn module_find_bug(_bugaddr: usize) -> *mut bug_entry { core::ptr::null_mut() }

#[cfg(CONFIG_MODULES)]
pub unsafe fn module_bug_finalize(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr, mod_: *mut module) {
    (*mod_).bug_table = core::ptr::null_mut();
    (*mod_).num_bugs = 0;
    let secstrings = (hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset) as *mut c_char;
    for i in 1..(*hdr).e_shnum as usize {
        let section = &*sechdrs.add(i);
        if libc_strcmp(secstrings.add(section.sh_name as usize), b"__bug_table\0".as_ptr() as *const c_char) != 0 { continue; }
        (*mod_).bug_table = section.sh_addr as *mut bug_entry;
        (*mod_).num_bugs = section.sh_size / core::mem::size_of::<bug_entry>();
        break;
    }
}

#[cfg(CONFIG_MODULES)] pub unsafe fn module_bug_cleanup(_mod_: *mut module) { }

extern "C" { fn libc_strcmp(a: *const c_char, b: *const c_char) -> i32; }

pub unsafe fn bug_get_file_line(bug: *mut bug_entry, file: *mut *const c_char, line: *mut u32) {
    #[cfg(CONFIG_DEBUG_BUGVERBOSE)] {
        #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)] { *file = (core::ptr::addr_of!((*bug).file_disp) as usize).wrapping_add((*bug).file_disp as usize) as *const c_char; }
        #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))] { *file = (*bug).file; }
        *line = (*bug).line;
    }
    #[cfg(not(CONFIG_DEBUG_BUGVERBOSE))] { *file = core::ptr::null(); *line = 0; }
}

unsafe fn bug_get_format(bug: *mut bug_entry) -> *const c_char {
    #[cfg(HAVE_ARCH_BUG_FORMAT)] {
        #[cfg(CONFIG_GENERIC_BUG_RELATIVE_POINTERS)] {
            if (*bug).format_disp != 0 { let p = (core::ptr::addr_of!((*bug).format_disp) as usize).wrapping_add((*bug).format_disp as usize) as *const c_char; if *p != 0 { return p; } }
            return core::ptr::null();
        }
        #[cfg(not(CONFIG_GENERIC_BUG_RELATIVE_POINTERS))] { return (*bug).format; }
    }
    core::ptr::null()
}

pub unsafe fn find_bug(bugaddr: usize) -> *mut bug_entry {
    let mut bug = __start___bug_table;
    while bug < __stop___bug_table { if bugaddr == bug_addr(bug) { return bug; } bug = bug.add(1); }
    module_find_bug(bugaddr)
}

unsafe fn __warn_printf(fmt: *const c_char, regs: *mut pt_regs) {
    if fmt.is_null() { return; }
    #[cfg(HAVE_ARCH_BUG_FORMAT_ARGS)] if !regs.is_null() { let mut args = arch_va_list { _private: [] }; let p = __warn_args(&mut args, regs); if !p.is_null() { vprintk(fmt, p); return; } }
    // pr_warn("%s", fmt)
}

unsafe fn __report_bug(mut bug: *mut bug_entry, bugaddr: usize, regs: *mut pt_regs) -> bug_trap_type {
    if bug.is_null() { if !is_valid_bugaddr(bugaddr) { return bug_trap_type::None; } bug = find_bug(bugaddr); if bug.is_null() { return bug_trap_type::None; } }
    let mut file = core::ptr::null(); let mut line = 0; bug_get_file_line(bug, &mut file, &mut line); let fmt = bug_get_format(bug);
    let warning = (*bug).flags & BUGFLAG_WARNING != 0; let once = (*bug).flags & BUGFLAG_ONCE != 0; let done = (*bug).flags & BUGFLAG_DONE != 0; let no_cut = (*bug).flags & BUGFLAG_NO_CUT_HERE != 0; let has_args = (*bug).flags & BUGFLAG_ARGS != 0;
    if warning && kunit_is_suppressed_warning(true) { return bug_trap_type::Warn; }
    disable_trace_on_warning();
    if warning && once { if done { return bug_trap_type::Warn; } (*bug).flags |= BUGFLAG_DONE; }
    if !no_cut { __warn_printf(fmt, if has_args { regs } else { core::ptr::null_mut() }); }
    if warning { __warn(file, line, bugaddr as *mut c_void, 0, regs, core::ptr::null_mut()); return bug_trap_type::Warn; }
    bug_trap_type::Bug
}

pub unsafe fn report_bug_entry(bug: *mut bug_entry, regs: *mut pt_regs) -> bug_trap_type { let r = warn_rcu_enter(); let ret = __report_bug(bug, bug_addr(bug), regs); warn_rcu_exit(r); ret }
pub unsafe fn report_bug(bugaddr: usize, regs: *mut pt_regs) -> bug_trap_type { let r = warn_rcu_enter(); let ret = __report_bug(core::ptr::null_mut(), bugaddr, regs); warn_rcu_exit(r); ret }

unsafe fn clear_once_table(mut start: *mut bug_entry, end: *mut bug_entry) { while start < end { (*start).flags &= !BUGFLAG_DONE; start = start.add(1); } }
pub unsafe fn generic_bug_clear_once() { clear_once_table(__start___bug_table, __stop___bug_table); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
