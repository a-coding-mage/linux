// SPDX-License-Identifier: GPL-2.0
/*
 * Rust translation of perf/util/libbfd.c.
 *
 * Original C dependencies:
 * "libbfd.h", "annotate.h", "bpf-event.h", "bpf-utils.h", "debug.h",
 * "dso.h", "env.h", "map.h", "srcline.h", "symbol.h", "symbol_conf.h",
 * "util.h", <tools/dis-asm-compat.h>, <bfd.h>, and optional libbpf headers.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type bfd_vma = u64;
type bfd_size_type = u64;
type flagword = c_uint;
type pthread_once_t = c_int;

const PACKAGE: &[u8] = b"perf\0";
const PTHREAD_ONCE_INIT: pthread_once_t = 0;
const BFD_INIT_MAGIC: c_int = 0x0bad;
const HAS_SYMS: flagword = 0x10;
const SEC_ALLOC: flagword = 0x1;
const BSF_LOCAL: flagword = 0x01;
const BSF_GLOBAL: flagword = 0x02;
const BSF_WEAK: flagword = 0x80;
const STB_LOCAL: c_int = 0;
const STB_GLOBAL: c_int = 1;
const STB_WEAK: c_int = 2;
const STT_FUNC: c_int = 2;
const EFAULT: c_int = 14;
const EWOULDBLOCK: c_int = 11;
const O_RDONLY: c_int = 0;
const MAX_INLINE_NEST: c_int = 1024;
const PATH_MAX: usize = 4096;
const PERF_BPIL_JITED_INSNS: c_int = 0;
const PERF_BPIL_JITED_KSYMS: c_int = 1;
const DSO_BINARY_TYPE__BPF_PROG_INFO: c_int = 8;
const SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE: c_int = -1;
const SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF: c_int = -2;
const SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF: c_int = -3;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bfd_build_id {
    pub size: size_t,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct bfd {
    pub build_id: *mut bfd_build_id,
}

#[repr(C)]
pub struct asection {
    pub vma: u64,
    pub size: u64,
    pub filepos: u64,
}

#[repr(C)]
pub struct asymbol {
    pub flags: flagword,
    pub value: u64,
}

#[repr(C)]
pub struct a2l_data {
    input: *const c_char,
    addr: u64,

    found: bool_,
    filename: *const c_char,
    funcname: *const c_char,
    line: c_uint,

    abfd: *mut bfd,
    syms: *mut *mut asymbol,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inline_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct build_id {
    pub size: size_t,
    pub data: [u8; 20],
}

#[repr(C)]
pub struct annotation {
    pub src: *mut annotation_source,
}

#[repr(C)]
pub struct annotation_source {
    pub source: list_head,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotate_args {
    pub ms: *mut map_symbol,
    pub offset: i64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_bpf_prog {
    pub env: *mut perf_env,
    pub id: u32,
    pub sub_id: c_int,
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_info_node {
    pub info_linear: *mut perf_bpil,
}

#[repr(C)]
pub struct perf_bpil {
    pub arrays: c_ulong,
    pub info: bpf_prog_info,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub jited_prog_insns: u64,
    pub jited_prog_len: u32,
    pub nr_line_info: u32,
    pub btf_id: u32,
    pub jited_ksyms: u64,
}

#[repr(C)]
pub struct bpf_prog_linfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_line_info {
    pub line_off: u32,
}

#[repr(C)]
pub struct btf_node {
    pub data: *mut c_void,
    pub data_size: u32,
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disassemble_info {
    pub arch: c_int,
    pub mach: c_ulong,
    pub buffer: *mut c_void,
    pub buffer_length: u64,
}

#[repr(C)]
pub struct disasm_line {
    pub al: annotation_line,
}

#[repr(C)]
pub struct annotation_line {
    _private: [u8; 0],
}

type disassembler_ftype = Option<unsafe extern "C" fn(c_int, *mut disassemble_info) -> c_int>;
type fprintf_ftype = unsafe extern "C" fn(*mut FILE, *const c_char, ...) -> c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub addr2line_disable_warn: bool,
}

#[repr(C)]
pub struct annotate_opts_t {
    pub hide_src_code: bool,
}

extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut annotate_opts: annotate_opts_t;
    static mut errno: c_int;
    static bfd_object: c_int;
    static bfd_target_elf_flavour: c_int;

    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn mutex_init_recursive(mutex: *mut mutex);
    fn pthread_once(once: *mut pthread_once_t, init_routine: unsafe extern "C" fn());

    fn bfd_init() -> c_int;
    fn bfd_thread_init(
        lock: unsafe extern "C" fn(*mut c_void) -> bool,
        unlock: unsafe extern "C" fn(*mut c_void) -> bool,
        data: *mut c_void,
    ) -> bool;
    fn bfd_get_error() -> c_int;
    fn bfd_errmsg(error_tag: c_int) -> *const c_char;
    fn bfd_get_filename(abfd: *mut bfd) -> *const c_char;
    fn bfd_get_file_flags(abfd: *mut bfd) -> flagword;
    fn bfd_get_symtab_upper_bound(abfd: *mut bfd) -> c_long;
    fn bfd_get_dynamic_symtab_upper_bound(abfd: *mut bfd) -> c_long;
    fn bfd_canonicalize_dynamic_symtab(abfd: *mut bfd, syms: *mut *mut asymbol) -> c_long;
    fn bfd_canonicalize_symtab(abfd: *mut bfd, syms: *mut *mut asymbol) -> c_long;
    fn bfd_section_flags(section: *mut asection) -> flagword;
    fn bfd_section_vma(section: *mut asection) -> bfd_vma;
    fn bfd_section_size(section: *mut asection) -> bfd_size_type;
    fn bfd_find_nearest_line(
        abfd: *mut bfd,
        section: *mut asection,
        syms: *mut *mut asymbol,
        offset: bfd_vma,
        filename_ptr: *mut *const c_char,
        funcname_ptr: *mut *const c_char,
        line_ptr: *mut c_uint,
    ) -> bool;
    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *mut bfd;
    fn bfd_fdopenr(filename: *const c_char, target: *const c_char, fd: c_int) -> *mut bfd;
    fn bfd_check_format(abfd: *mut bfd, format: c_int) -> bool;
    fn bfd_close(abfd: *mut bfd) -> bool;
    fn bfd_map_over_sections(
        abfd: *mut bfd,
        operation: unsafe extern "C" fn(*mut bfd, *mut asection, *mut c_void),
        user_storage: *mut c_void,
    );
    fn bfd_find_inliner_info(
        abfd: *mut bfd,
        filename_ptr: *mut *const c_char,
        funcname_ptr: *mut *const c_char,
        line_ptr: *mut c_uint,
    ) -> bool;
    fn bfd_asymbol_value(symbol: *const asymbol) -> u64;
    fn bfd_asymbol_name(symbol: *const asymbol) -> *const c_char;
    fn bfd_get_flavour(abfd: *mut bfd) -> c_int;
    fn bfd_get_section_by_name(abfd: *mut bfd, name: *const c_char) -> *mut asection;
    fn bfd_asymbol_section(symbol: *mut asymbol) -> *mut asection;
    fn bfd_get_section_contents(
        abfd: *mut bfd,
        section: *mut asection,
        location: *mut c_void,
        offset: u64,
        count: u64,
    ) -> bool;

    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn abort() -> !;
    static mut stdout: *mut FILE;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn is_regular_file(filename: *const c_char) -> bool;
    fn perf_exe(path: *mut c_char, size: size_t) -> c_int;

    fn dso__a2l(dso: *mut dso) -> *mut a2l_data;
    fn dso__set_a2l(dso: *mut dso, a2l: *mut a2l_data);
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__set_text_offset(dso: *mut dso, offset: u64);
    fn dso__set_text_end(dso: *mut dso, end: u64);
    fn dso__text_offset(dso: *mut dso) -> u64;
    fn dso__symbols(dso: *mut dso) -> *mut c_void;
    fn dso__set_adjust_symbols(dso: *mut dso, adjust: bool);
    fn dso__binary_type(dso: *mut dso) -> c_int;
    fn dso__bpf_prog(dso: *mut dso) -> *mut perf_bpf_prog;
    fn map__dso(map: *mut map) -> *mut dso;

    fn new_inline_sym(dso: *mut dso, sym: *mut symbol, funcname: *const c_char) -> *mut symbol;
    fn srcline_from_fileline(filename: *const c_char, line: c_uint) -> *mut c_char;
    fn inline_list__append(sym: *mut symbol, srcline: *mut c_char, node: *mut inline_node) -> c_int;
    fn symbol__new(start: u64, len: u64, binding: c_int, typ: c_int, name: *const c_char) -> *mut symbol;
    fn symbols__insert(symbols: *mut c_void, symbol: *mut symbol);
    fn symbols__fixup_end(symbols: *mut c_void, is_kallsyms: bool);
    fn symbols__fixup_duplicate(symbols: *mut c_void);
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;

    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);

    fn init_disassemble_info_compat(info: *mut disassemble_info, stream: *mut FILE, fprintf_func: fprintf_ftype, fprintf_styled: *mut c_void);
    static fprintf_styled: *mut c_void;
    fn bfd_get_arch(abfd: *mut bfd) -> c_int;
    fn bfd_get_mach(abfd: *mut bfd) -> c_ulong;
    fn bfd_big_endian(abfd: *mut bfd) -> bool;
    fn disassemble_init_for_target(info: *mut disassemble_info);
    fn disassembler(abfd: *mut bfd) -> disassembler_ftype;
    fn disassembler_four_args(arch: c_int, big: bool, mach: c_ulong, abfd: *mut bfd) -> disassembler_ftype;
    fn perf_env__find_bpf_prog_info(env: *mut perf_env, id: u32) -> *mut bpf_prog_info_node;
    fn perf_env__find_btf(env: *mut perf_env, id: u32) -> *mut btf_node;
    fn bpf_prog_linfo__new(info: *mut bpf_prog_info) -> *mut bpf_prog_linfo;
    fn bpf_prog_linfo__free(prog_linfo: *mut bpf_prog_linfo);
    fn bpf_prog_linfo__lfind_addr_func(prog_linfo: *mut bpf_prog_linfo, addr: u64, sub_id: c_int, nr_skip: c_int) -> *const bpf_line_info;
    fn btf__new(data: *mut u8, size: u32) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__name_by_offset(btf: *mut btf, offset: u32) -> *const c_char;
    fn disasm_line__new(args: *mut annotate_args) -> *mut disasm_line;
    fn annotation_line__add(al: *mut annotation_line, source: *mut list_head);
}

unsafe extern "C" fn perf_bfd_lock(bfd_mutex: *mut c_void) -> bool {
    unsafe {
        mutex_lock(bfd_mutex);
    }
    true
}

unsafe extern "C" fn perf_bfd_unlock(bfd_mutex: *mut c_void) -> bool {
    unsafe {
        mutex_unlock(bfd_mutex);
    }
    true
}

unsafe extern "C" fn perf_bfd_init() {
    static mut BFD_MUTEX: mutex = mutex { _private: [] };

    unsafe {
        mutex_init_recursive(&raw mut BFD_MUTEX);

        if bfd_init() != BFD_INIT_MAGIC {
            pr_err(c"Error initializing libbfd\n".as_ptr());
            return;
        }
        if !bfd_thread_init(perf_bfd_lock, perf_bfd_unlock, (&raw mut BFD_MUTEX).cast()) {
            pr_err(c"Error initializing libbfd threading\n".as_ptr());
        }
    }
}

unsafe fn ensure_bfd_init() {
    static mut BFD_INIT_ONCE: pthread_once_t = PTHREAD_ONCE_INIT;

    unsafe {
        pthread_once(&raw mut BFD_INIT_ONCE, perf_bfd_init);
    }
}

unsafe fn bfd_error(string: *const c_char) -> c_int {
    let errmsg: *const c_char;

    unsafe {
        errmsg = bfd_errmsg(bfd_get_error());
        fflush(stdout);

        if !string.is_null() {
            pr_debug(c"%s: %s\n".as_ptr(), string, errmsg);
        } else {
            pr_debug(c"%s\n".as_ptr(), errmsg);
        }
    }

    -1
}

unsafe fn slurp_symtab(abfd: *mut bfd, a2l: *mut a2l_data) -> c_int {
    let mut storage: c_long;
    let symcount: c_long;
    let syms: *mut *mut asymbol;
    let mut dynamic = false;

    unsafe {
        if (bfd_get_file_flags(abfd) & HAS_SYMS) == 0 {
            return bfd_error(bfd_get_filename(abfd));
        }

        storage = bfd_get_symtab_upper_bound(abfd);
        if storage == 0 {
            storage = bfd_get_dynamic_symtab_upper_bound(abfd);
            dynamic = true;
        }
        if storage < 0 {
            return bfd_error(bfd_get_filename(abfd));
        }

        syms = malloc(storage as size_t).cast();
        if dynamic {
            symcount = bfd_canonicalize_dynamic_symtab(abfd, syms);
        } else {
            symcount = bfd_canonicalize_symtab(abfd, syms);
        }

        if symcount < 0 {
            free(syms.cast());
            return bfd_error(bfd_get_filename(abfd));
        }

        (*a2l).syms = syms;
    }
    0
}

unsafe extern "C" fn find_address_in_section(abfd: *mut bfd, section: *mut asection, data: *mut c_void) {
    let pc: bfd_vma;
    let vma: bfd_vma;
    let size: bfd_size_type;
    let a2l: *mut a2l_data = data.cast();
    let flags: flagword;

    unsafe {
        if (*a2l).found {
            return;
        }

        /* bfd_get_section_flags compatibility macro maps to bfd_section_flags here. */
        flags = bfd_section_flags(section);
        if (flags & SEC_ALLOC) == 0 {
            return;
        }

        pc = (*a2l).addr;
        /* bfd_get_section_vma compatibility macro maps to bfd_section_vma here. */
        vma = bfd_section_vma(section);
        /* bfd_get_section_size compatibility macro maps to bfd_section_size here. */
        size = bfd_section_size(section);

        if pc < vma || pc >= vma.wrapping_add(size) {
            return;
        }

        (*a2l).found = bfd_find_nearest_line(
            abfd,
            section,
            (*a2l).syms,
            pc.wrapping_sub(vma),
            &mut (*a2l).filename,
            &mut (*a2l).funcname,
            &mut (*a2l).line,
        );

        if !(*a2l).filename.is_null() && strlen((*a2l).filename) == 0 {
            (*a2l).filename = ptr::null();
        }
    }
}

unsafe fn addr2line_init(path: *const c_char) -> *mut a2l_data {
    let abfd: *mut bfd;
    let mut a2l: *mut a2l_data = ptr::null_mut();

    unsafe {
        ensure_bfd_init();
        abfd = bfd_openr(path, ptr::null());
        if abfd.is_null() {
            return ptr::null_mut();
        }

        if !bfd_check_format(abfd, bfd_object) {
            goto_out_addr2line_init(abfd, a2l);
            return ptr::null_mut();
        }

        a2l = zalloc(size_of::<a2l_data>()).cast();
        if a2l.is_null() {
            goto_out_addr2line_init(abfd, a2l);
            return ptr::null_mut();
        }

        (*a2l).abfd = abfd;
        (*a2l).input = strdup(path);
        if (*a2l).input.is_null() {
            goto_out_addr2line_init(abfd, a2l);
            return ptr::null_mut();
        }

        if slurp_symtab(abfd, a2l) != 0 {
            goto_out_addr2line_init(abfd, a2l);
            return ptr::null_mut();
        }

        a2l
    }
}

unsafe fn goto_out_addr2line_init(abfd: *mut bfd, a2l: *mut a2l_data) {
    unsafe {
        if !a2l.is_null() {
            zfree((&mut (*a2l).input as *mut *const c_char).cast());
            free(a2l.cast());
        }
        bfd_close(abfd);
    }
}

unsafe fn addr2line_cleanup(a2l: *mut a2l_data) {
    unsafe {
        if !(*a2l).abfd.is_null() {
            bfd_close((*a2l).abfd);
        }
        zfree((&mut (*a2l).input as *mut *const c_char).cast());
        zfree((&mut (*a2l).syms as *mut *mut *mut asymbol).cast());
        free(a2l.cast());
    }
}

unsafe fn inline_list__append_dso_a2l(dso: *mut dso, node: *mut inline_node, sym: *mut symbol) -> c_int {
    unsafe {
        let a2l = dso__a2l(dso);
        let inline_sym = new_inline_sym(dso, sym, (*a2l).funcname);
        let mut srcline: *mut c_char = ptr::null_mut();

        if !(*a2l).filename.is_null() {
            srcline = srcline_from_fileline((*a2l).filename, (*a2l).line);
        }

        inline_list__append(inline_sym, srcline, node)
    }
}

#[no_mangle]
pub unsafe extern "C" fn libbfd__addr2line(
    dso_name: *const c_char,
    addr: u64,
    file: *mut *mut c_char,
    line: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    unsafe {
        let mut ret = 0;
        let mut a2l = dso__a2l(dso);

        if a2l.is_null() {
            a2l = addr2line_init(dso_name);
            dso__set_a2l(dso, a2l);
        }

        if a2l.is_null() {
            if !symbol_conf.addr2line_disable_warn {
                pr_warning(c"addr2line_init failed for %s\n".as_ptr(), dso_name);
            }
            return 0;
        }

        (*a2l).addr = addr;
        (*a2l).found = false;

        bfd_map_over_sections((*a2l).abfd, find_address_in_section, a2l.cast());

        if !(*a2l).found {
            return 0;
        }

        if unwind_inlines {
            let mut cnt = 0;

            if !node.is_null() && inline_list__append_dso_a2l(dso, node, sym) != 0 {
                return 0;
            }

            while bfd_find_inliner_info((*a2l).abfd, &mut (*a2l).filename, &mut (*a2l).funcname, &mut (*a2l).line)
                && {
                    let old = cnt;
                    cnt += 1;
                    old < MAX_INLINE_NEST
                }
            {
                if !(*a2l).filename.is_null() && strlen((*a2l).filename) == 0 {
                    (*a2l).filename = ptr::null();
                }

                if !node.is_null() {
                    if inline_list__append_dso_a2l(dso, node, sym) != 0 {
                        return 0;
                    }
                    // found at least one inline frame
                    ret = 1;
                }
            }
        }

        if !file.is_null() {
            *file = if !(*a2l).filename.is_null() { strdup((*a2l).filename) } else { ptr::null_mut() };
            ret = if !(*file).is_null() { 1 } else { 0 };
        }

        if !line.is_null() {
            *line = (*a2l).line;
        }

        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn dso__free_a2l_libbfd(dso: *mut dso) {
    unsafe {
        let a2l = dso__a2l(dso);

        if a2l.is_null() {
            return;
        }

        addr2line_cleanup(a2l);

        dso__set_a2l(dso, ptr::null_mut());
    }
}

unsafe extern "C" fn bfd_symbols__cmpvalue(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let as_ = *(a as *const *const asymbol);
        let bs = *(b as *const *const asymbol);

        if bfd_asymbol_value(as_) != bfd_asymbol_value(bs) {
            return bfd_asymbol_value(as_).wrapping_sub(bfd_asymbol_value(bs)) as c_int;
        }

        *bfd_asymbol_name(as_) as c_int - *bfd_asymbol_name(bs) as c_int
    }
}

unsafe fn bfd2elf_binding(symbol: *mut asymbol) -> c_int {
    unsafe {
        if ((*symbol).flags & BSF_WEAK) != 0 {
            return STB_WEAK;
        }
        if ((*symbol).flags & BSF_GLOBAL) != 0 {
            return STB_GLOBAL;
        }
        if ((*symbol).flags & BSF_LOCAL) != 0 {
            return STB_LOCAL;
        }
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn dso__load_bfd_symbols(dso: *mut dso, debugfile: *const c_char) -> c_int {
    unsafe {
        let mut err = -1;
        let symbols_size: c_long;
        let symbols_count: c_long;
        let mut i: c_long;
        let mut section: *mut asection;
        let symbols: *mut *mut asymbol;
        let mut sym: *mut asymbol;
        let mut symbol: *mut symbol;
        let abfd: *mut bfd;
        let start: u64;
        let len: u64;

        ensure_bfd_init();
        abfd = bfd_openr(debugfile, ptr::null());
        if abfd.is_null() {
            return -1;
        }

        if !bfd_check_format(abfd, bfd_object) {
            pr_debug2(c"%s: cannot read %s bfd file.\n".as_ptr(), c"dso__load_bfd_symbols".as_ptr(), dso__long_name(dso));
            bfd_close(abfd);
            return err;
        }

        if bfd_get_flavour(abfd) == bfd_target_elf_flavour {
            bfd_close(abfd);
            return err;
        }

        symbols_size = bfd_get_symtab_upper_bound(abfd);
        if symbols_size == 0 {
            bfd_close(abfd);
            return 0;
        }

        if symbols_size < 0 {
            bfd_close(abfd);
            return err;
        }

        symbols = malloc(symbols_size as size_t).cast();
        if symbols.is_null() {
            bfd_close(abfd);
            return err;
        }

        symbols_count = bfd_canonicalize_symtab(abfd, symbols);
        if symbols_count < 0 {
            free(symbols.cast());
            bfd_close(abfd);
            return err;
        }

        section = bfd_get_section_by_name(abfd, c".text".as_ptr());
        if !section.is_null() {
            i = 0;
            while i < symbols_count {
                if strcmp(bfd_asymbol_name(*symbols.offset(i as isize)), c"__ImageBase".as_ptr()) == 0
                    || strcmp(bfd_asymbol_name(*symbols.offset(i as isize)), c"__image_base__".as_ptr()) == 0
                {
                    break;
                }
                i += 1;
            }
            if i < symbols_count {
                /* PE symbols can only have 4 bytes, so use .text high bits */
                let text_offset: u64 = ((*section).vma - ((*section).vma as u32 as u64))
                    + (bfd_asymbol_value(*symbols.offset(i as isize)) as u32 as u64);
                dso__set_text_offset(dso, text_offset);
                dso__set_text_end(dso, ((*section).vma - text_offset) + (*section).size);
            } else {
                dso__set_text_offset(dso, (*section).vma - (*section).filepos);
                dso__set_text_end(dso, (*section).filepos + (*section).size);
            }
        }

        qsort(symbols.cast(), symbols_count as size_t, size_of::<*mut asymbol>(), bfd_symbols__cmpvalue);

        /* bfd_get_section compatibility macro maps bfd_asymbol_section to bfd_get_section in C when needed. */
        i = 0;
        while i < symbols_count {
            sym = *symbols.offset(i as isize);
            section = bfd_asymbol_section(sym);
            if bfd2elf_binding(sym) < 0 {
                i += 1;
                continue;
            }

            while i + 1 < symbols_count
                && bfd_asymbol_section(*symbols.offset((i + 1) as isize)) == section
                && bfd2elf_binding(*symbols.offset((i + 1) as isize)) < 0
            {
                i += 1;
            }

            if i + 1 < symbols_count && bfd_asymbol_section(*symbols.offset((i + 1) as isize)) == section {
                len = (**symbols.offset((i + 1) as isize)).value - (*sym).value;
            } else {
                len = (*section).size - (*sym).value;
            }

            start = bfd_asymbol_value(sym) - dso__text_offset(dso);
            symbol = symbol__new(start, len, bfd2elf_binding(sym), STT_FUNC, bfd_asymbol_name(sym));
            if symbol.is_null() {
                free(symbols.cast());
                bfd_close(abfd);
                return err;
            }

            symbols__insert(dso__symbols(dso), symbol);
            i += 1;
        }

        symbols__fixup_end(dso__symbols(dso), false);
        symbols__fixup_duplicate(dso__symbols(dso));
        dso__set_adjust_symbols(dso, true);

        err = 0;
        free(symbols.cast());
        bfd_close(abfd);
        err
    }
}

#[no_mangle]
pub unsafe extern "C" fn libbfd__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    unsafe {
        let size = size_of_val(&(*bid).data);
        let mut err = -1;
        let fd: c_int;
        let abfd: *mut bfd;

        if filename.is_null() {
            return -EFAULT;
        }

        errno = 0;
        if !is_regular_file(filename) {
            return if errno == 0 { -EWOULDBLOCK } else { -errno };
        }

        fd = open(filename, O_RDONLY);
        if fd < 0 {
            return -1;
        }

        ensure_bfd_init();
        abfd = bfd_fdopenr(filename, ptr::null(), fd);
        if abfd.is_null() {
            return -1;
        }

        if !bfd_check_format(abfd, bfd_object) {
            pr_debug2(c"%s: cannot read %s bfd file.\n".as_ptr(), c"libbfd__read_build_id".as_ptr(), filename);
            bfd_close(abfd);
            return err;
        }

        if (*abfd).build_id.is_null() || (*(*abfd).build_id).size > size {
            bfd_close(abfd);
            return err;
        }

        memcpy((*bid).data.as_mut_ptr().cast(), (*(*abfd).build_id).data, (*(*abfd).build_id).size);
        memset(
            (*bid).data.as_mut_ptr().add((*(*abfd).build_id).size).cast(),
            0,
            size - (*(*abfd).build_id).size,
        );
        (*bid).size = (*(*abfd).build_id).size;
        err = (*bid).size as c_int;

        bfd_close(abfd);
        err
    }
}

#[no_mangle]
pub unsafe extern "C" fn libbfd_filename__read_debuglink(filename: *const c_char, debuglink: *mut c_char, size: size_t) -> c_int {
    unsafe {
        let mut err = -1;
        let section: *mut asection;
        let abfd: *mut bfd;

        ensure_bfd_init();
        abfd = bfd_openr(filename, ptr::null());
        if abfd.is_null() {
            return -1;
        }

        if !bfd_check_format(abfd, bfd_object) {
            pr_debug2(c"%s: cannot read %s bfd file.\n".as_ptr(), c"libbfd_filename__read_debuglink".as_ptr(), filename);
            bfd_close(abfd);
            return err;
        }

        section = bfd_get_section_by_name(abfd, c".gnu_debuglink".as_ptr());
        if section.is_null() {
            bfd_close(abfd);
            return err;
        }

        if (*section).size > size as u64 {
            bfd_close(abfd);
            return err;
        }

        if !bfd_get_section_contents(abfd, section, debuglink.cast(), 0, (*section).size) {
            bfd_close(abfd);
            return err;
        }

        err = 0;

        bfd_close(abfd);
        err
    }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__disassemble_bpf_libbfd(sym: *mut symbol, args: *mut annotate_args) -> c_int {
    /*
     * C condition: #ifdef HAVE_LIBBPF_SUPPORT.
     * The translated body below preserves the libbpf-supported path. When that
     * build-time condition is not enabled, the C function returns
     * SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF.
     */
    const HAVE_LIBBPF_SUPPORT: bool = true;

    unsafe {
        if !HAVE_LIBBPF_SUPPORT {
            return SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF;
        }

        let notes = symbol__annotation(sym);
        let mut prog_linfo: *mut bpf_prog_linfo = ptr::null_mut();
        let info_node: *mut bpf_prog_info_node;
        let len = ((*sym).end - (*sym).start) as c_int;
        let disassemble: disassembler_ftype;
        let map = (*(*args).ms).map;
        let info_linear: *mut perf_bpil;
        let mut info: disassemble_info = core::mem::zeroed();
        let dso = map__dso(map);
        let mut pc = 0;
        let mut count: c_int;
        let sub_id: c_int;
        let mut btf: *mut btf = ptr::null_mut();
        let mut tpath = [0 as c_char; PATH_MAX];
        let mut buf_size: size_t = 0;
        let mut nr_skip = 0;
        let mut buf: *mut c_char = ptr::null_mut();
        let bfdf: *mut bfd;
        let mut ret: c_int;
        let mut s: *mut FILE = ptr::null_mut();

        if dso__binary_type(dso) != DSO_BINARY_TYPE__BPF_PROG_INFO {
            return SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE;
        }

        pr_debug(
            c"%s: handling sym %s addr %llx len %llx\n".as_ptr(),
            c"symbol__disassemble_bpf_libbfd".as_ptr(),
            (*sym).name,
            (*sym).start,
            (*sym).end - (*sym).start,
        );

        memset(tpath.as_mut_ptr().cast(), 0, tpath.len());
        perf_exe(tpath.as_mut_ptr(), tpath.len());

        ensure_bfd_init();
        bfdf = bfd_openr(tpath.as_ptr(), ptr::null());
        if bfdf.is_null() {
            abort();
        }

        if !bfd_check_format(bfdf, bfd_object) {
            abort();
        }

        s = open_memstream(&mut buf, &mut buf_size);
        if s.is_null() {
            ret = errno;
            bpf_prog_linfo__free(prog_linfo);
            btf__free(btf);
            bfd_close(bfdf);
            return ret;
        }
        init_disassemble_info_compat(&mut info, s, fprintf, fprintf_styled);
        info.arch = bfd_get_arch(bfdf);
        info.mach = bfd_get_mach(bfdf);

        info_node = perf_env__find_bpf_prog_info((*dso__bpf_prog(dso)).env, (*dso__bpf_prog(dso)).id);
        if info_node.is_null() {
            ret = SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF;
            fclose(s);
            free(buf.cast());
            bfd_close(bfdf);
            return ret;
        }
        info_linear = (*info_node).info_linear;
        sub_id = (*dso__bpf_prog(dso)).sub_id;

        /* jited_prog_insns is only valid if bpil_offs_to_addr() converted it */
        if ((*info_linear).arrays & (1_u64 << PERF_BPIL_JITED_INSNS) as c_ulong) == 0 {
            ret = SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF;
            fclose(s);
            free(buf.cast());
            bfd_close(bfdf);
            return ret;
        }
        info.buffer = ((*info_linear).info.jited_prog_insns as usize) as *mut c_void;
        info.buffer_length = (*info_linear).info.jited_prog_len as u64;

        if (*info_linear).info.nr_line_info != 0 {
            prog_linfo = bpf_prog_linfo__new(&mut (*info_linear).info);
        }

        if (*info_linear).info.btf_id != 0 {
            let node = perf_env__find_btf((*dso__bpf_prog(dso)).env, (*info_linear).info.btf_id);
            if !node.is_null() {
                btf = btf__new((*node).data.cast(), (*node).data_size);
            }
        }

        disassemble_init_for_target(&mut info);

        /* C condition DISASM_FOUR_ARGS_SIGNATURE selects this alternate call. */
        let use_disasm_four_args_signature = false;
        disassemble = if use_disasm_four_args_signature {
            disassembler_four_args(info.arch, bfd_big_endian(bfdf), info.mach, bfdf)
        } else {
            disassembler(bfdf)
        };
        if disassemble.is_none() {
            abort();
        }

        /* jited_ksyms is only valid if bpil_offs_to_addr() converted it */
        if ((*info_linear).arrays & (1_u64 << PERF_BPIL_JITED_KSYMS) as c_ulong) == 0 {
            ret = SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF;
            bpf_prog_linfo__free(prog_linfo);
            btf__free(btf);
            fclose(s);
            free(buf.cast());
            bfd_close(bfdf);
            return ret;
        }

        fflush(s);
        loop {
            let mut linfo: *const bpf_line_info = ptr::null();
            let dl: *mut disasm_line;
            let prev_buf_size: size_t;
            let srcline: *const c_char;
            let addr: u64;

            addr = pc as u64 + *(((*info_linear).info.jited_ksyms as usize) as *mut u64).offset(sub_id as isize);
            count = disassemble.unwrap()(pc, &mut info);

            if !prog_linfo.is_null() {
                linfo = bpf_prog_linfo__lfind_addr_func(prog_linfo, addr, sub_id, nr_skip);
            }

            if !linfo.is_null() && !btf.is_null() {
                srcline = btf__name_by_offset(btf, (*linfo).line_off);
                nr_skip += 1;
            } else {
                srcline = ptr::null();
            }

            fprintf(s, c"\n".as_ptr());
            prev_buf_size = buf_size;
            fflush(s);

            if !annotate_opts.hide_src_code && !srcline.is_null() {
                (*args).offset = -1;
                (*args).line = srcline as *mut c_char;
                (*args).line_nr = 0;
                (*args).fileloc = ptr::null_mut();
                (*(*args).ms).sym = sym;
                let dl_src = disasm_line__new(args);
                if !dl_src.is_null() {
                    annotation_line__add(&mut (*dl_src).al, &mut (*(*notes).src).source);
                }
            }

            (*args).offset = pc as i64;
            (*args).line = buf.add(prev_buf_size);
            (*args).line_nr = 0;
            (*args).fileloc = ptr::null_mut();
            (*(*args).ms).sym = sym;
            dl = disasm_line__new(args);
            if !dl.is_null() {
                annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);
            }

            pc += count;
            if !(count > 0 && pc < len) {
                break;
            }
        }

        ret = 0;
        bpf_prog_linfo__free(prog_linfo);
        btf__free(btf);
        if !s.is_null() {
            fclose(s);
            free(buf.cast());
        }
        bfd_close(bfdf);
        ret
    }
}
