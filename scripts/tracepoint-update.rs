// SPDX-License-Identifier: GPL-2.0-only

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem;
use std::ptr;

// Supplied by elf-parse.h in the C implementation.
#[repr(C)]
pub struct Elf_Ehdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Shdr {
    _private: [u8; 0],
}

const SHN_UNDEF: u32 = 0;
const SHN_XINDEX: u32 = 0xffff;
const ET_REL: u32 = 1;

extern "C" {
    fn shdr_offset(sec: *const Elf_Shdr) -> usize;
    fn shdr_size(sec: *const Elf_Shdr) -> usize;
    fn shdr_link(sec: *const Elf_Shdr) -> u32;
    fn shdr_name(sec: *const Elf_Shdr) -> u32;
    fn ehdr_shoff(ehdr: *const Elf_Ehdr) -> usize;
    fn ehdr_shentsize(ehdr: *const Elf_Ehdr) -> c_int;
    fn ehdr_shstrndx(ehdr: *const Elf_Ehdr) -> u32;
    fn ehdr_shnum(ehdr: *const Elf_Ehdr) -> u32;
    fn elf_map(fname: *const c_char, size: *mut usize, etype: u32) -> *mut c_void;
    fn elf_unmap(addr: *mut c_void, size: usize);
}

static mut CHECK_DATA_SEC: *mut Elf_Shdr = ptr::null_mut();
static mut TRACEPOINT_DATA_SEC: *mut Elf_Shdr = ptr::null_mut();

unsafe fn get_index(start: *mut c_void, entsize: c_int, index: c_int) -> *mut Elf_Shdr {
    (start as *mut u8).add((entsize * index) as usize) as *mut Elf_Shdr
}

unsafe extern "C" fn compare_strings(a: *const c_void, b: *const c_void) -> c_int {
    let av = *(a as *const *const c_char);
    let bv = *(b as *const *const c_char);
    libc::strcmp(av, bv)
}

struct ElfTracepoint {
    ehdr: *mut Elf_Ehdr,
    array: *mut *const c_char,
    count: c_int,
}

const REALLOC_SIZE: usize = 1 << 10;
const REALLOC_MASK: c_int = (REALLOC_SIZE - 1) as c_int;

unsafe fn add_string(str_: *const c_char, vals: *mut *mut *const c_char, count: *mut c_int) -> c_int {
    let mut array = *vals;

    if (*count & REALLOC_MASK) == 0 {
        let size = (*count as usize) + REALLOC_SIZE;
        let new_array = libc::realloc(array as *mut c_void, mem::size_of::<*const c_char>() * size)
            as *mut *const c_char;
        if new_array.is_null() {
            libc::fprintf(libc::stderr, b"Failed memory allocation\n\0".as_ptr() as *const c_char);
            libc::free(*vals as *mut c_void);
            *vals = ptr::null_mut();
            return -1;
        }
        array = new_array;
        *vals = array;
    }

    *array.add(*count as usize) = str_;
    *count += 1;
    0
}

unsafe fn make_trace_array(etrace: *mut ElfTracepoint) {
    let ehdr = (*etrace).ehdr;
    let mut vals: *mut *const c_char = ptr::null_mut();
    let mut count: c_int = 0;
    (*etrace).array = ptr::null_mut();

    // The __tracepoint_check section is filled with strings of the names of
    // tracepoints (in tracepoint_strings). Create an array that points to
    // each string and then sort the array.
    let sec = CHECK_DATA_SEC;
    let mut str_ = (ehdr as *mut u8).add(shdr_offset(sec)) as *const c_char;
    let end = str_.add(shdr_size(sec));
    while str_ < end {
        let len = libc::strlen(str_);
        if len != 0 && add_string(str_, &mut vals, &mut count) < 0 {
            return;
        }
        str_ = str_.add(len + 1);
    }

    // If CONFIG_TRACEPOINT_VERIFY_USED is not set, there's nothing to do
    if count == 0 {
        return;
    }

    libc::qsort(vals as *mut c_void, count as usize, mem::size_of::<*const c_char>(), Some(compare_strings));
    (*etrace).array = vals;
    (*etrace).count = count;
}

unsafe fn find_event(str_: *const c_char, array: *mut *const c_char, size: c_int) -> bool {
    libc::bsearch(
        &str_ as *const *const c_char as *const c_void,
        array as *const c_void,
        size as usize,
        mem::size_of::<*const c_char>(),
        Some(compare_strings),
    ) != ptr::null_mut()
}

unsafe fn check_tracepoints(etrace: *mut ElfTracepoint, fname: *const c_char) {
    let ehdr = (*etrace).ehdr;
    if (*etrace).array.is_null() {
        return;
    }

    // The __tracepoints_strings section holds all the names of the defined
    // tracepoints. If any are not in __tracepoint_check_section they are unused.
    let sec = TRACEPOINT_DATA_SEC;
    let mut str_ = (ehdr as *mut u8).add(shdr_offset(sec)) as *const c_char;
    let end = str_.add(shdr_size(sec));
    while str_ < end {
        let len = libc::strlen(str_);
        if len != 0 && !find_event(str_, (*etrace).array, (*etrace).count) {
            libc::fprintf(libc::stderr, b"warning: tracepoint '%s' is unused\0".as_ptr() as *const c_char, str_);
            if !fname.is_null() {
                libc::fprintf(libc::stderr, b" in module %s\n\0".as_ptr() as *const c_char, fname);
            } else {
                libc::fprintf(libc::stderr, b"\n\0".as_ptr() as *const c_char);
            }
        }
        str_ = str_.add(len + 1);
    }
    libc::free((*etrace).array as *mut c_void);
}

unsafe fn tracepoint_check(etrace: *mut ElfTracepoint, fname: *const c_char) -> *mut c_void {
    make_trace_array(etrace);
    check_tracepoints(etrace, fname);
    ptr::null_mut()
}

unsafe fn process_tracepoints(mod_: bool, addr: *mut c_void, mut fname: *const c_char) -> c_int {
    let mut etrace = ElfTracepoint { ehdr: ptr::null_mut(), array: ptr::null_mut(), count: 0 };
    let ehdr = addr as *mut Elf_Ehdr;
    let shdr_start = (ehdr as *mut u8).add(ehdr_shoff(ehdr)) as *mut Elf_Shdr;
    let shentsize = ehdr_shentsize(ehdr);
    let mut shstrndx = ehdr_shstrndx(ehdr);
    if shstrndx == SHN_XINDEX {
        shstrndx = shdr_link(shdr_start);
    }
    let string_sec = get_index(shdr_start as *mut c_void, shentsize, shstrndx as c_int);
    let secstrings = (ehdr as *mut u8).add(shdr_offset(string_sec)) as *const c_char;
    let mut shnum = ehdr_shnum(ehdr);
    if shnum == SHN_UNDEF {
        shnum = shdr_size(shdr_start) as u32;
    }

    let mut done = 2;
    let mut i = 0;
    while done != 0 && i < shnum {
        let shdr = get_index(shdr_start as *mut c_void, shentsize, i as c_int);
        let idx = shdr_name(shdr) as usize;
        if CStr::from_ptr(secstrings.add(idx)).to_bytes() == b"__tracepoint_check" {
            CHECK_DATA_SEC = shdr;
            done -= 1;
        }
        if CStr::from_ptr(secstrings.add(idx)).to_bytes() == b"__tracepoints_strings" {
            TRACEPOINT_DATA_SEC = shdr;
            done -= 1;
        }
        i += 1;
    }

    if mod_ && CHECK_DATA_SEC.is_null() && TRACEPOINT_DATA_SEC.is_null() { return 0; }
    if CHECK_DATA_SEC.is_null() {
        if mod_ {
            libc::fprintf(libc::stderr, b"warning: Module %s has only unused tracepoints\n\0".as_ptr() as *const c_char, fname);
            return 0;
        }
        libc::fprintf(libc::stderr, b"no __tracepoint_check in file: %s\n\0".as_ptr() as *const c_char, fname);
        return -1;
    }
    if TRACEPOINT_DATA_SEC.is_null() {
        if mod_ { return 0; }
        libc::fprintf(libc::stderr, b"no __tracepoint_strings in file: %s\n\0".as_ptr() as *const c_char, fname);
        return -1;
    }
    if !mod_ { fname = ptr::null(); }
    etrace.ehdr = ehdr;
    tracepoint_check(&mut etrace, fname);
    0
}

pub unsafe fn main(argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut n_error = 0;
    let mut size = 0usize;
    let mut mod_ = false;
    if argc > 1 && libc::strcmp(*argv.add(1), b"--module\0".as_ptr() as *const c_char) == 0 {
        mod_ = true;
        argv = argv.add(1);
    }
    let argc = if mod_ { argc - 1 } else { argc };
    if argc < 2 {
        if mod_ { libc::fprintf(libc::stderr, b"usage: tracepoint-update --module module...\n\0".as_ptr() as *const c_char); }
        else { libc::fprintf(libc::stderr, b"usage: tracepoint-update vmlinux...\n\0".as_ptr() as *const c_char); }
        return 0;
    }
    for i in 1..argc {
        let fname = *argv.add(i as usize);
        let addr = elf_map(fname, &mut size, 1 << ET_REL);
        if addr.is_null() { n_error += 1; continue; }
        if process_tracepoints(mod_, addr, fname) != 0 { n_error += 1; }
        elf_unmap(addr, size);
    }
    if n_error != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
