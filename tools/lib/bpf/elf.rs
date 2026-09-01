// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Translated from lib/bpf/elf.c. External libelf/libbpf/libc symbols are
// declared here as dependencies supplied by the surrounding repository/build.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

pub const VERSYM_HIDDEN: c_int = 0x8000;
pub const VERSYM_VERSION: c_int = 0x7fff;

const EV_NONE: c_uint = 0;
const EV_CURRENT: c_uint = 1;
const ELF_C_READ_MMAP: c_uint = 8;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;

const SHT_SYMTAB: c_int = 2;
const SHT_DYNSYM: c_int = 11;
const SHT_GNU_verdef: c_int = 0x6ffffffd;
const SHT_GNU_versym: c_int = 0x6fffffff;

const STB_WEAK: c_int = 2;
const STT_FUNC: c_int = 2;
const ET_DYN: u16 = 3;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ESRCH: c_int = 3;

const LIBBPF_ERRNO__LIBELF: c_int = 4000;
const LIBBPF_ERRNO__FORMAT: c_int = 4001;

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: c_uint,
    pub d_version: c_uint,
    pub d_size: usize,
    pub d_off: i64,
    pub d_align: usize,
}

#[repr(C)]
pub struct elf_fd {
    pub elf: *mut Elf,
    pub fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

pub type GElf_Versym = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Verdef {
    pub vd_version: u16,
    pub vd_flags: u16,
    pub vd_ndx: u16,
    pub vd_cnt: u16,
    pub vd_hash: u32,
    pub vd_aux: u32,
    pub vd_next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GElf_Verdaux {
    pub vda_name: u32,
    pub vda_next: u32,
}

#[repr(C)]
pub struct elf_sym {
    pub name: *const c_char,
    pub sym: GElf_Sym,
    pub sh: GElf_Shdr,
    pub ver: c_int,
    pub hidden: bool,
}

#[repr(C)]
pub struct elf_sym_iter {
    pub elf: *mut Elf,
    pub syms: *mut Elf_Data,
    pub versyms: *mut Elf_Data,
    pub verdefs: *mut Elf_Data,
    pub nr_syms: usize,
    pub strtabidx: usize,
    pub verdef_strtabidx: usize,
    pub next_sym_idx: usize,
    pub sym: elf_sym,
    pub st_type: c_int,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub bind: c_int,
    pub idx: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn elf_version(version: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_uint, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_getscn(elf: *mut Elf, index: usize) -> *mut Elf_Scn;
    fn elf_strptr(elf: *mut Elf, section: usize, offset: usize) -> *const c_char;
    fn elf_errmsg(err: c_int) -> *const c_char;

    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn gelf_getsym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym) -> *mut GElf_Sym;
    fn gelf_getversym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Versym) -> *mut GElf_Versym;
    fn gelf_getverdef(data: *mut Elf_Data, offset: c_int, dst: *mut GElf_Verdef) -> *mut GElf_Verdef;
    fn gelf_getverdaux(data: *mut Elf_Data, offset: c_int, dst: *mut GElf_Verdaux) -> *mut GElf_Verdaux;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(base: *mut c_void, nmemb: usize, size: usize, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    ) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;

    fn errstr(err: c_int) -> *const c_char;
    fn glob_match(str_: *const c_char, pat: *const c_char) -> bool;
    fn libbpf_ensure_mem(
        data: *mut *mut c_void,
        cap_cnt: *mut usize,
        elem_sz: usize,
        need_cnt: usize,
    ) -> c_int;

    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

#[inline]
fn GELF_ST_TYPE(val: u8) -> c_int {
    (val & 0xf) as c_int
}

#[inline]
fn GELF_ST_BIND(val: u8) -> c_int {
    (val >> 4) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_open(binary_path: *const c_char, elf_fd: *mut elf_fd) -> c_int {
    let fd: c_int;
    let ret: c_int;
    let elf: *mut Elf;

    unsafe {
        (*elf_fd).elf = ptr::null_mut();
        (*elf_fd).fd = -1;

        if elf_version(EV_CURRENT) == EV_NONE {
            pr_warn(c"elf: failed to init libelf for %s\n".as_ptr(), binary_path);
            return -LIBBPF_ERRNO__LIBELF;
        }
        fd = open(binary_path, O_RDONLY | O_CLOEXEC);
        if fd < 0 {
            ret = -errno;
            pr_warn(
                c"elf: failed to open %s: %s\n".as_ptr(),
                binary_path,
                errstr(ret),
            );
            return ret;
        }
        elf = elf_begin(fd, ELF_C_READ_MMAP, ptr::null_mut());
        if elf.is_null() {
            pr_warn(
                c"elf: could not read elf from %s: %s\n".as_ptr(),
                binary_path,
                elf_errmsg(-1),
            );
            close(fd);
            return -LIBBPF_ERRNO__FORMAT;
        }
        (*elf_fd).fd = fd;
        (*elf_fd).elf = elf;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_close(elf_fd: *mut elf_fd) {
    unsafe {
        if elf_fd.is_null() {
            return;
        }
        elf_end((*elf_fd).elf);
        close((*elf_fd).fd);
    }
}

/* Return next ELF section of sh_type after scn, or first of that type if scn is NULL. */
unsafe fn elf_find_next_scn_by_type(
    elf: *mut Elf,
    sh_type: c_int,
    mut scn: *mut Elf_Scn,
) -> *mut Elf_Scn {
    unsafe {
        loop {
            scn = elf_nextscn(elf, scn);
            if scn.is_null() {
                break;
            }
            let mut sh: GElf_Shdr = mem::zeroed();

            if gelf_getshdr(scn, &mut sh).is_null() {
                continue;
            }
            if sh.sh_type == sh_type as u32 {
                return scn;
            }
        }
        ptr::null_mut()
    }
}

unsafe fn elf_sym_iter_new(
    iter: *mut elf_sym_iter,
    elf: *mut Elf,
    binary_path: *const c_char,
    sh_type: c_int,
    st_type: c_int,
) -> c_int {
    let mut scn: *mut Elf_Scn = ptr::null_mut();
    let mut ehdr: GElf_Ehdr = unsafe { mem::zeroed() };
    let mut sh: GElf_Shdr = unsafe { mem::zeroed() };

    unsafe {
        ptr::write_bytes(iter as *mut u8, 0, mem::size_of::<elf_sym_iter>());

        if gelf_getehdr(elf, &mut ehdr).is_null() {
            pr_warn(
                c"elf: failed to get ehdr from %s: %s\n".as_ptr(),
                binary_path,
                elf_errmsg(-1),
            );
            return -EINVAL;
        }

        scn = elf_find_next_scn_by_type(elf, sh_type, ptr::null_mut());
        if scn.is_null() {
            pr_debug(
                c"elf: failed to find symbol table ELF sections in '%s'\n".as_ptr(),
                binary_path,
            );
            return -ENOENT;
        }

        if gelf_getshdr(scn, &mut sh).is_null() {
            return -EINVAL;
        }

        (*iter).strtabidx = sh.sh_link as usize;
        (*iter).syms = elf_getdata(scn, ptr::null_mut());
        if (*iter).syms.is_null() {
            pr_warn(
                c"elf: failed to get symbols for symtab section in '%s': %s\n".as_ptr(),
                binary_path,
                elf_errmsg(-1),
            );
            return -EINVAL;
        }
        (*iter).nr_syms = (*(*iter).syms).d_size / sh.sh_entsize as usize;
        (*iter).elf = elf;
        (*iter).st_type = st_type;

        /* Version symbol table is meaningful to dynsym only */
        if sh_type != SHT_DYNSYM {
            return 0;
        }

        scn = elf_find_next_scn_by_type(elf, SHT_GNU_versym, ptr::null_mut());
        if scn.is_null() {
            return 0;
        }
        (*iter).versyms = elf_getdata(scn, ptr::null_mut());

        scn = elf_find_next_scn_by_type(elf, SHT_GNU_verdef, ptr::null_mut());
        if scn.is_null() {
            return 0;
        }

        (*iter).verdefs = elf_getdata(scn, ptr::null_mut());
        if (*iter).verdefs.is_null() || gelf_getshdr(scn, &mut sh).is_null() {
            pr_warn(
                c"elf: failed to get verdef ELF section in '%s'\n".as_ptr(),
                binary_path,
            );
            return -EINVAL;
        }
        (*iter).verdef_strtabidx = sh.sh_link as usize;

        0
    }
}

unsafe fn elf_sym_iter_next(iter: *mut elf_sym_iter) -> *mut elf_sym {
    unsafe {
        let ret: *mut elf_sym = &mut (*iter).sym;
        let sym: *mut GElf_Sym = &mut (*ret).sym;
        let mut name: *const c_char;
        let mut versym: GElf_Versym = 0;
        let mut sym_scn: *mut Elf_Scn;
        let mut idx: usize;

        idx = (*iter).next_sym_idx;
        while idx < (*iter).nr_syms {
            if gelf_getsym((*iter).syms, idx as c_int, sym).is_null() {
                idx += 1;
                continue;
            }
            if GELF_ST_TYPE((*sym).st_info) != (*iter).st_type {
                idx += 1;
                continue;
            }
            name = elf_strptr((*iter).elf, (*iter).strtabidx, (*sym).st_name as usize);
            if name.is_null() {
                idx += 1;
                continue;
            }
            sym_scn = elf_getscn((*iter).elf, (*sym).st_shndx as usize);
            if sym_scn.is_null() {
                idx += 1;
                continue;
            }
            if gelf_getshdr(sym_scn, &mut (*ret).sh).is_null() {
                idx += 1;
                continue;
            }

            (*iter).next_sym_idx = idx + 1;
            (*ret).name = name;
            (*ret).ver = 0;
            (*ret).hidden = false;

            if !(*iter).versyms.is_null() {
                if gelf_getversym((*iter).versyms, idx as c_int, &mut versym).is_null() {
                    idx += 1;
                    continue;
                }
                (*ret).ver = (versym as c_int) & VERSYM_VERSION;
                (*ret).hidden = ((versym as c_int) & VERSYM_HIDDEN) != 0;
            }
            return ret;
        }

        ptr::null_mut()
    }
}

unsafe fn elf_get_vername(iter: *mut elf_sym_iter, ver: c_int) -> *const c_char {
    unsafe {
        let mut verdaux: GElf_Verdaux = mem::zeroed();
        let mut verdef: GElf_Verdef = mem::zeroed();
        let mut offset: c_int;

        if (*iter).verdefs.is_null() {
            return ptr::null();
        }

        offset = 0;
        while !gelf_getverdef((*iter).verdefs, offset, &mut verdef).is_null() {
            if verdef.vd_ndx as c_int != ver {
                if verdef.vd_next == 0 {
                    break;
                }

                offset += verdef.vd_next as c_int;
                continue;
            }

            if gelf_getverdaux((*iter).verdefs, offset + verdef.vd_aux as c_int, &mut verdaux)
                .is_null()
            {
                break;
            }

            return elf_strptr((*iter).elf, (*iter).verdef_strtabidx, verdaux.vda_name as usize);
        }
        ptr::null()
    }
}

unsafe fn symbol_match(
    iter: *mut elf_sym_iter,
    sh_type: c_int,
    sym: *mut elf_sym,
    name: *const c_char,
    name_len: usize,
    lib_ver: *const c_char,
) -> bool {
    unsafe {
        let ver_name: *const c_char;

        /* Symbols are in forms of func, func@LIB_VER or func@@LIB_VER
         * make sure the func part matches the user specified name
         */
        if strncmp((*sym).name, name, name_len) != 0 {
            return false;
        }

        /* ...but we don't want a search for "foo" to match 'foo2" also, so any
         * additional characters in sname should be of the form "@@LIB".
         */
        if *(*sym).name.add(name_len) != 0 && *(*sym).name.add(name_len) != b'@' as c_char {
            return false;
        }

        /* If user does not specify symbol version, then we got a match */
        if lib_ver.is_null() {
            return true;
        }

        /* If user specifies symbol version, for dynamic symbols,
         * get version name from ELF verdef section for comparison.
         */
        if sh_type == SHT_DYNSYM {
            ver_name = elf_get_vername(iter, (*sym).ver);
            if ver_name.is_null() {
                return false;
            }
            return strcmp(ver_name, lib_ver) == 0;
        }

        /* For normal symbols, it is already in form of func@LIB_VER */
        strcmp((*sym).name, name) == 0
    }
}

/* Transform symbol's virtual address (absolute for binaries and relative
 * for shared libs) into file offset, which is what kernel is expecting
 * for uprobe/uretprobe attachment.
 * See Documentation/trace/uprobetracer.rst for more details. This is done
 * by looking up symbol's containing section's header and using iter's virtual
 * address (sh_addr) and corresponding file offset (sh_offset) to transform
 * sym.st_value (virtual address) into desired final file offset.
 */
unsafe fn elf_sym_offset(sym: *mut elf_sym) -> c_ulong {
    unsafe { ((*sym).sym.st_value - (*sym).sh.sh_addr + (*sym).sh.sh_offset) as c_ulong }
}

/* Find offset of function name in the provided ELF object. "binary_path" is
 * the path to the ELF binary represented by "elf", and only used for error
 * reporting matters. "name" matches symbol name or name@@LIB for library
 * functions.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_find_func_offset(
    elf: *mut Elf,
    binary_path: *const c_char,
    name: *const c_char,
) -> c_long {
    unsafe {
        let sh_types: [c_int; 2] = [SHT_DYNSYM, SHT_SYMTAB];
        let mut at_symbol: *const c_char;
        let lib_ver: *const c_char;
        let is_shared_lib: bool;
        let mut ret: c_long = -ENOENT as c_long;
        let name_len: usize;
        let mut ehdr: GElf_Ehdr = mem::zeroed();

        if gelf_getehdr(elf, &mut ehdr).is_null() {
            pr_warn(
                c"elf: failed to get ehdr from %s: %s\n".as_ptr(),
                binary_path,
                elf_errmsg(-1),
            );
            ret = -LIBBPF_ERRNO__FORMAT as c_long;
            return ret;
        }
        /* for shared lib case, we do not need to calculate relative offset */
        is_shared_lib = ehdr.e_type == ET_DYN;

        /* Does name specify "@@LIB_VER" or "@LIB_VER" ? */
        at_symbol = strchr(name, b'@' as c_int) as *const c_char;
        if !at_symbol.is_null() {
            name_len = at_symbol.offset_from(name) as usize;
            /* skip second @ if it's @@LIB_VER case */
            if *at_symbol.add(1) == b'@' as c_char {
                at_symbol = at_symbol.add(1);
            }
            lib_ver = at_symbol.add(1);
        } else {
            name_len = strlen(name);
            lib_ver = ptr::null();
        }

        /* Search SHT_DYNSYM, SHT_SYMTAB for symbol. This search order is used because if
         * a binary is stripped, it may only have SHT_DYNSYM, and a fully-statically
         * linked binary may not have SHT_DYMSYM, so absence of a section should not be
         * reported as a warning/error.
         */
        for i in 0..sh_types.len() {
            let mut iter: elf_sym_iter = mem::zeroed();
            let mut sym: *mut elf_sym;
            let mut last_bind: c_int = -1;
            let mut cur_bind: c_int;

            ret = elf_sym_iter_new(&mut iter, elf, binary_path, sh_types[i], STT_FUNC) as c_long;
            if ret == -ENOENT as c_long {
                continue;
            }
            if ret != 0 {
                return ret;
            }

            loop {
                sym = elf_sym_iter_next(&mut iter);
                if sym.is_null() {
                    break;
                }
                if !symbol_match(&mut iter, sh_types[i], sym, name, name_len, lib_ver) {
                    continue;
                }

                cur_bind = GELF_ST_BIND((*sym).sym.st_info);

                if ret > 0 {
                    /* handle multiple matches */
                    if elf_sym_offset(sym) as c_long == ret {
                        /* same offset, no problem */
                        continue;
                    } else if last_bind != STB_WEAK && cur_bind != STB_WEAK {
                        /* Only accept one non-weak bind. */
                        pr_warn(
                            c"elf: ambiguous match for '%s', '%s' in '%s'\n".as_ptr(),
                            (*sym).name,
                            name,
                            binary_path,
                        );
                        ret = -LIBBPF_ERRNO__FORMAT as c_long;
                        return ret;
                    } else if cur_bind == STB_WEAK {
                        /* already have a non-weak bind, and
                         * this is a weak bind, so ignore.
                         */
                        continue;
                    }
                }

                ret = elf_sym_offset(sym) as c_long;
                last_bind = cur_bind;
            }
            if ret > 0 {
                break;
            }
        }

        if ret > 0 {
            pr_debug(
                c"elf: symbol address match for '%s' in '%s': 0x%lx\n".as_ptr(),
                name,
                binary_path,
                ret as c_ulong,
            );
        } else if ret == 0 {
            pr_warn(
                c"elf: '%s' is 0 in symtab for '%s': %s\n".as_ptr(),
                name,
                binary_path,
                if is_shared_lib {
                    c"should not be 0 in a shared library".as_ptr()
                } else {
                    c"try using shared library path instead".as_ptr()
                },
            );
            ret = -ENOENT as c_long;
        } else {
            pr_warn(
                c"elf: failed to find symbol '%s' in '%s'\n".as_ptr(),
                name,
                binary_path,
            );
        }
        ret
    }
}

/* Find offset of function name in ELF object specified by path. "name" matches
 * symbol name or name@@LIB for library functions.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_find_func_offset_from_file(
    binary_path: *const c_char,
    name: *const c_char,
) -> c_long {
    unsafe {
        let mut elf_fd: elf_fd = mem::zeroed();
        let mut ret: c_long = -ENOENT as c_long;

        ret = elf_open(binary_path, &mut elf_fd) as c_long;
        if ret != 0 {
            return ret;
        }
        ret = elf_find_func_offset(elf_fd.elf, binary_path, name);
        elf_close(&mut elf_fd);
        ret
    }
}

unsafe extern "C" fn symbol_cmp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let sym_a: *const symbol = a as *const symbol;
        let sym_b: *const symbol = b as *const symbol;

        strcmp((*sym_a).name, (*sym_b).name)
    }
}

/*
 * Return offsets in @poffsets for symbols specified in @syms array argument.
 * On success returns 0 and offsets are returned in allocated array with @cnt
 * size, that needs to be released by the caller.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_resolve_syms_offsets(
    binary_path: *const c_char,
    cnt: c_int,
    syms: *mut *const c_char,
    poffsets: *mut *mut c_ulong,
    st_type: c_int,
) -> c_int {
    unsafe {
        let sh_types: [c_int; 2] = [SHT_DYNSYM, SHT_SYMTAB];
        let mut err: c_int = 0;
        let mut cnt_done: c_int = 0;
        let offsets: *mut c_ulong;
        let symbols: *mut symbol;
        let mut elf_fd: elf_fd = mem::zeroed();

        err = elf_open(binary_path, &mut elf_fd);
        if err != 0 {
            return err;
        }

        offsets = calloc(cnt as usize, mem::size_of::<c_ulong>()) as *mut c_ulong;
        symbols = calloc(cnt as usize, mem::size_of::<symbol>()) as *mut symbol;

        if offsets.is_null() || symbols.is_null() {
            err = -ENOMEM;
        } else {
            for i in 0..cnt {
                (*symbols.add(i as usize)).name = *syms.add(i as usize);
                (*symbols.add(i as usize)).idx = i;
            }

            qsort(
                symbols as *mut c_void,
                cnt as usize,
                mem::size_of::<symbol>(),
                symbol_cmp,
            );

            for i in 0..sh_types.len() {
                let mut iter: elf_sym_iter = mem::zeroed();
                let mut sym: *mut elf_sym;

                err = elf_sym_iter_new(&mut iter, elf_fd.elf, binary_path, sh_types[i], st_type);
                if err == -ENOENT {
                    continue;
                }
                if err != 0 {
                    break;
                }

                loop {
                    sym = elf_sym_iter_next(&mut iter);
                    if sym.is_null() {
                        break;
                    }
                    let sym_offset: c_ulong = elf_sym_offset(sym);
                    let bind: c_int = GELF_ST_BIND((*sym).sym.st_info);
                    let mut tmp = symbol {
                        name: (*sym).name,
                        bind: 0,
                        idx: 0,
                    };
                    let found: *mut symbol = bsearch(
                        &mut tmp as *mut symbol as *const c_void,
                        symbols as *const c_void,
                        cnt as usize,
                        mem::size_of::<symbol>(),
                        symbol_cmp,
                    ) as *mut symbol;
                    if found.is_null() {
                        continue;
                    }

                    let offset: *mut c_ulong = offsets.add((*found).idx as usize);
                    if *offset > 0 {
                        /* same offset, no problem */
                        if *offset == sym_offset {
                            continue;
                        }
                        /* handle multiple matches */
                        if (*found).bind != STB_WEAK && bind != STB_WEAK {
                            /* Only accept one non-weak bind. */
                            pr_warn(
                                c"elf: ambiguous match found '%s@%lu' in '%s' previous offset %lu\n"
                                    .as_ptr(),
                                (*sym).name,
                                sym_offset,
                                binary_path,
                                *offset,
                            );
                            err = -ESRCH;
                            break;
                        } else if bind == STB_WEAK {
                            /* already have a non-weak bind, and
                             * this is a weak bind, so ignore.
                             */
                            continue;
                        }
                    } else {
                        cnt_done += 1;
                    }
                    *offset = sym_offset;
                    (*found).bind = bind;
                }
                if err != 0 {
                    break;
                }
            }

            if err == 0 {
                if cnt != cnt_done {
                    err = -ENOENT;
                } else {
                    *poffsets = offsets;
                }
            }
        }

        free(symbols as *mut c_void);
        if err != 0 {
            free(offsets as *mut c_void);
        }
        elf_close(&mut elf_fd);
        err
    }
}

/*
 * Return offsets in @poffsets for symbols specified by @pattern argument.
 * On success returns 0 and offsets are returned in allocated @poffsets
 * array with the @pctn size, that needs to be released by the caller.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn elf_resolve_pattern_offsets(
    binary_path: *const c_char,
    pattern: *const c_char,
    poffsets: *mut *mut c_ulong,
    pcnt: *mut usize,
) -> c_int {
    unsafe {
        let sh_types: [c_int; 2] = [SHT_SYMTAB, SHT_DYNSYM];
        let mut offsets: *mut c_ulong = ptr::null_mut();
        let mut cap: usize = 0;
        let mut cnt: usize = 0;
        let mut elf_fd: elf_fd = mem::zeroed();
        let mut err: c_int;

        err = elf_open(binary_path, &mut elf_fd);
        if err != 0 {
            return err;
        }

        for i in 0..sh_types.len() {
            let mut iter: elf_sym_iter = mem::zeroed();
            let mut sym: *mut elf_sym;

            err = elf_sym_iter_new(&mut iter, elf_fd.elf, binary_path, sh_types[i], STT_FUNC);
            if err == -ENOENT {
                continue;
            }
            if err != 0 {
                break;
            }

            loop {
                sym = elf_sym_iter_next(&mut iter);
                if sym.is_null() {
                    break;
                }
                if !glob_match((*sym).name, pattern) {
                    continue;
                }

                err = libbpf_ensure_mem(
                    &mut offsets as *mut *mut c_ulong as *mut *mut c_void,
                    &mut cap,
                    mem::size_of::<c_ulong>(),
                    cnt + 1,
                );
                if err != 0 {
                    break;
                }

                *offsets.add(cnt) = elf_sym_offset(sym);
                cnt += 1;
            }
            if err != 0 {
                break;
            }

            /* If we found anything in the first symbol section,
             * do not search others to avoid duplicates.
             */
            if cnt != 0 {
                break;
            }
        }

        if err == 0 {
            if cnt != 0 {
                *poffsets = offsets;
                *pcnt = cnt;
            } else {
                err = -ENOENT;
            }
        }

        if err != 0 {
            free(offsets as *mut c_void);
        }
        elf_close(&mut elf_fd);
        err
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
