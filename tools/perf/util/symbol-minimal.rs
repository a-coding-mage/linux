// Translated from perf/util/symbol-minimal.c.
// Original includes: debug.h, dso.h, symbol.h, symsrc.h, errno.h, unistd.h,
// fcntl.h, string.h, stdlib.h, byteswap.h, sys/stat.h, linux/zalloc.h,
// internal/lib.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;

const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: c_int = 1;
const ELFDATA2MSB: c_int = 2;
const EV_CURRENT: u8 = 1;
const SELFMAG: usize = 4;
const ELFMAG: &[u8; 4] = b"\x7fELF";
const PT_NOTE: u32 = 4;
const EM_X86_64: u16 = 62;
const NT_GNU_BUILD_ID: u32 = 3;
const O_RDONLY: c_int = 0;
const SEEK_SET: c_int = 0;
const EFAULT: c_int = 14;
const EWOULDBLOCK: c_int = 11;

#[repr(C)]
pub struct build_id {
    pub data: [u8; 32],
    pub size: size_t,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kcore_extract {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symsrc {
    pub name: *mut c_char,
    pub fd: c_int,
    pub type_: dso_binary_type,
}

#[repr(C)]
pub struct dso_access {
    pub load_errno: c_int,
    pub is_64_bit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dso_binary_type {
    DSO_BINARY_TYPE__UNKNOWN = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_type {
    DSO__TYPE_UNKNOWN = 0,
    DSO__TYPE_64BIT = 1,
    DSO__TYPE_X32BIT = 2,
    DSO__TYPE_32BIT = 3,
}

pub type mapfn_t = Option<unsafe extern "C" fn()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
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
pub struct Elf64_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
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
pub struct Elf32_Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
    pub st_size: isize,
}

#[repr(C)]
struct NoteHdr {
    n_namesz: u32,
    n_descsz: u32,
    n_type: u32,
}

#[repr(C)]
union Hdrs {
    elf32: Hdrs32,
    elf64: Hdrs64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Hdrs32 {
    ehdr32: Elf32_Ehdr,
    phdr32: *mut Elf32_Phdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Hdrs64 {
    ehdr64: Elf64_Ehdr,
    phdr64: *mut Elf64_Phdr,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;

    fn is_regular_file(filename: *const c_char) -> bool;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn RC_CHK_ACCESS(dso: *mut dso) -> *mut dso_access;
    fn dso__set_build_id(dso: *mut dso, bid: *mut build_id);
    fn zfree(ptr: *mut *mut c_char);
    fn pr_warning(fmt: *const c_char, ...);
}

fn NOTE_ALIGN(sz: size_t) -> size_t {
    (sz + 3) & !3
}

unsafe fn check_need_swap(file_endian: c_int) -> bool {
    let data: c_int = 1;
    let check = &data as *const c_int as *const u8;
    let host_endian: c_int;

    if *check.add(0) == 1 {
        host_endian = ELFDATA2LSB;
    } else {
        host_endian = ELFDATA2MSB;
    }

    host_endian != file_endian
}

unsafe fn read_build_id(
    note_data: *mut c_void,
    note_len: size_t,
    bid: *mut build_id,
    need_swap: bool,
) -> c_int {
    let size = size_of_val(&(*bid).data);
    let mut nhdr: *mut NoteHdr;
    let mut ptr = note_data as *mut u8;
    let note_data_u8 = note_data as *mut u8;

    while ptr.add(size_of::<NoteHdr>()) < note_data_u8.add(note_len) {
        let name: *const c_char;
        let namesz: size_t;
        let descsz: size_t;
        let remaining: size_t;

        nhdr = ptr as *mut NoteHdr;
        if need_swap {
            (*nhdr).n_namesz = (*nhdr).n_namesz.swap_bytes();
            (*nhdr).n_descsz = (*nhdr).n_descsz.swap_bytes();
            (*nhdr).n_type = (*nhdr).n_type.swap_bytes();
        }

        namesz = NOTE_ALIGN((*nhdr).n_namesz as size_t);
        descsz = NOTE_ALIGN((*nhdr).n_descsz as size_t);

        /* validate individually to avoid size_t overflow on 32-bit */
        remaining = note_data_u8.add(note_len) as usize - ptr as usize - size_of::<NoteHdr>();
        if namesz > remaining || descsz > remaining - namesz {
            pr_warning(
                c"%s: oversized note: n_namesz=%u, n_descsz=%u\n".as_ptr(),
                c"read_build_id".as_ptr(),
                (*nhdr).n_namesz,
                (*nhdr).n_descsz,
            );
            break;
        }

        ptr = ptr.add(size_of::<NoteHdr>());
        name = ptr as *const c_char;
        ptr = ptr.add(namesz);
        if (*nhdr).n_type == NT_GNU_BUILD_ID && (*nhdr).n_namesz as usize == size_of_val(c"GNU") {
            if memcmp(name as *const c_void, c"GNU".as_ptr() as *const c_void, size_of_val(c"GNU"))
                == 0
            {
                let sz = core::cmp::min(size, descsz);
                memcpy(
                    (*bid).data.as_mut_ptr() as *mut c_void,
                    ptr as *const c_void,
                    sz,
                );
                memset(
                    (*bid).data.as_mut_ptr().add(sz) as *mut c_void,
                    0,
                    size - sz,
                );
                (*bid).size = sz;
                return 0;
            }
        }
        ptr = ptr.add(descsz);
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename__read_debuglink(
    _filename: *const c_char,
    _debuglink: *mut c_char,
    _size: size_t,
) -> c_int {
    -1
}

/*
 * Just try PT_NOTE header otherwise fails
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    let mut fd: c_int;
    let mut ret: c_int = -1;
    let mut need_swap = false;
    let elf32: bool;
    let mut hdrs: Hdrs = core::mem::zeroed();
    let mut phdr: *mut c_void;
    let mut buf: *mut c_void = ptr::null_mut();
    let phdr_size: ssize_t;
    let ehdr_size: ssize_t;
    let mut buf_size: ssize_t = 0;

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

    if read(fd, hdrs.elf32.ehdr32.e_ident.as_mut_ptr() as *mut c_void, EI_NIDENT) != EI_NIDENT as ssize_t {
        goto_out(fd, ret);
        return ret;
    }

    if memcmp(
        hdrs.elf32.ehdr32.e_ident.as_ptr() as *const c_void,
        ELFMAG.as_ptr() as *const c_void,
        SELFMAG,
    ) != 0 || hdrs.elf32.ehdr32.e_ident[EI_VERSION] != EV_CURRENT
    {
        goto_out(fd, ret);
        return ret;
    }

    need_swap = check_need_swap(hdrs.elf32.ehdr32.e_ident[EI_DATA] as c_int);
    elf32 = hdrs.elf32.ehdr32.e_ident[EI_CLASS] == ELFCLASS32;
    ehdr_size = (if elf32 {
        size_of::<Elf32_Ehdr>()
    } else {
        size_of::<Elf64_Ehdr>()
    } - EI_NIDENT) as ssize_t;

    let ehdr_ptr = if elf32 {
        &mut hdrs.elf32.ehdr32 as *mut Elf32_Ehdr as *mut u8
    } else {
        &mut hdrs.elf64.ehdr64 as *mut Elf64_Ehdr as *mut u8
    };
    if read(fd, ehdr_ptr.add(EI_NIDENT) as *mut c_void, ehdr_size as size_t) != ehdr_size {
        goto_out(fd, ret);
        return ret;
    }

    if need_swap {
        if elf32 {
            hdrs.elf32.ehdr32.e_phoff = hdrs.elf32.ehdr32.e_phoff.swap_bytes();
            hdrs.elf32.ehdr32.e_phentsize = hdrs.elf32.ehdr32.e_phentsize.swap_bytes();
            hdrs.elf32.ehdr32.e_phnum = hdrs.elf32.ehdr32.e_phnum.swap_bytes();
        } else {
            hdrs.elf64.ehdr64.e_phoff = hdrs.elf64.ehdr64.e_phoff.swap_bytes();
            hdrs.elf64.ehdr64.e_phentsize = hdrs.elf64.ehdr64.e_phentsize.swap_bytes();
            hdrs.elf64.ehdr64.e_phnum = hdrs.elf64.ehdr64.e_phnum.swap_bytes();
        }
    }
    if (elf32 && hdrs.elf32.ehdr32.e_phentsize as usize != size_of::<Elf32_Phdr>())
        || (!elf32 && hdrs.elf64.ehdr64.e_phentsize as usize != size_of::<Elf64_Phdr>())
    {
        goto_out(fd, ret);
        return ret;
    }

    phdr_size = if elf32 {
        (size_of::<Elf32_Phdr>() * hdrs.elf32.ehdr32.e_phnum as usize) as ssize_t
    } else {
        (size_of::<Elf64_Phdr>() * hdrs.elf64.ehdr64.e_phnum as usize) as ssize_t
    };
    phdr = malloc(phdr_size as size_t);
    if phdr.is_null() {
        goto_out(fd, ret);
        return ret;
    }

    lseek(
        fd,
        if elf32 {
            hdrs.elf32.ehdr32.e_phoff as off_t
        } else {
            hdrs.elf64.ehdr64.e_phoff as off_t
        },
        SEEK_SET,
    );
    if read(fd, phdr, phdr_size as size_t) != phdr_size {
        free(phdr);
        goto_out(fd, ret);
        return ret;
    }

    if elf32 {
        hdrs.elf32.phdr32 = phdr as *mut Elf32_Phdr;
    } else {
        hdrs.elf64.phdr64 = phdr as *mut Elf64_Phdr;
    }

    let phnum = if elf32 {
        hdrs.elf32.ehdr32.e_phnum as c_int
    } else {
        hdrs.elf64.ehdr64.e_phnum as c_int
    };
    for i in 0..phnum {
        let mut p_filesz: ssize_t;
        let idx = i as usize;

        if need_swap {
            if elf32 {
                (*hdrs.elf32.phdr32.add(idx)).p_type = (*hdrs.elf32.phdr32.add(idx)).p_type.swap_bytes();
                (*hdrs.elf32.phdr32.add(idx)).p_offset = (*hdrs.elf32.phdr32.add(idx)).p_offset.swap_bytes();
                (*hdrs.elf32.phdr32.add(idx)).p_filesz = (*hdrs.elf32.phdr32.add(idx)).p_filesz.swap_bytes();
            } else {
                (*hdrs.elf64.phdr64.add(idx)).p_type = (*hdrs.elf64.phdr64.add(idx)).p_type.swap_bytes();
                (*hdrs.elf64.phdr64.add(idx)).p_offset = (*hdrs.elf64.phdr64.add(idx)).p_offset.swap_bytes();
                (*hdrs.elf64.phdr64.add(idx)).p_filesz = (*hdrs.elf64.phdr64.add(idx)).p_filesz.swap_bytes();
            }
        }
        if (if elf32 {
            (*hdrs.elf32.phdr32.add(idx)).p_type
        } else {
            (*hdrs.elf64.phdr64.add(idx)).p_type
        }) != PT_NOTE
        {
            continue;
        }

        p_filesz = if elf32 {
            (*hdrs.elf32.phdr32.add(idx)).p_filesz as ssize_t
        } else {
            (*hdrs.elf64.phdr64.add(idx)).p_filesz as ssize_t
        };
        /* ssize_t can go negative with crafted ELF p_filesz values */
        if p_filesz <= 0 {
            continue;
        }
        if p_filesz > buf_size {
            let tmp: *mut c_void;

            buf_size = p_filesz;
            tmp = realloc(buf, buf_size as size_t);
            if tmp.is_null() {
                free(buf);
                free(phdr);
                goto_out(fd, ret);
                return ret;
            }
            buf = tmp;
        }
        lseek(
            fd,
            if elf32 {
                (*hdrs.elf32.phdr32.add(idx)).p_offset as off_t
            } else {
                (*hdrs.elf64.phdr64.add(idx)).p_offset as off_t
            },
            SEEK_SET,
        );
        if read(fd, buf, p_filesz as size_t) != p_filesz {
            free(buf);
            free(phdr);
            goto_out(fd, ret);
            return ret;
        }

        ret = read_build_id(buf, p_filesz as size_t, bid, need_swap);
        if ret == 0 {
            ret = (*bid).size as c_int;
            break;
        }
    }
    free(buf);
    free(phdr);
    close(fd);
    ret
}

unsafe fn goto_out(fd: c_int, _ret: c_int) {
    close(fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    let fd: c_int;
    let mut ret: c_int = -1;
    let mut stbuf: stat = core::mem::zeroed();
    let buf_size: size_t;
    let buf: *mut c_void;

    fd = open(filename, O_RDONLY);
    if fd < 0 {
        return -1;
    }

    if fstat(fd, &mut stbuf) < 0 {
        close(fd);
        return ret;
    }

    buf_size = stbuf.st_size as size_t;
    buf = malloc(buf_size);
    if buf.is_null() {
        close(fd);
        return ret;
    }

    if read(fd, buf, buf_size) != buf_size as ssize_t {
        free(buf);
        close(fd);
        return ret;
    }

    ret = read_build_id(buf, buf_size, bid, false);
    free(buf);
    close(fd);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symsrc__init(
    ss: *mut symsrc,
    dso: *mut dso,
    name: *const c_char,
    type_: dso_binary_type,
) -> c_int {
    let fd = open(name, O_RDONLY);
    if fd < 0 {
        (*RC_CHK_ACCESS(dso)).load_errno = errno;
        return -1;
    }

    (*ss).name = strdup(name);
    if (*ss).name.is_null() {
        close(fd);
        (*RC_CHK_ACCESS(dso)).load_errno = errno;
        return -1;
    }

    (*ss).fd = fd;
    (*ss).type_ = type_;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symsrc__possibly_runtime(_ss: *mut symsrc) -> bool {
    /* Assume all sym sources could be a runtime image. */
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symsrc__has_symtab(_ss: *mut symsrc) -> bool {
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symsrc__destroy(ss: *mut symsrc) {
    zfree(&mut (*ss).name);
    close((*ss).fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__synthesize_plt_symbols(_dso: *mut dso, _ss: *mut symsrc) -> c_int {
    0
}

unsafe fn fd__is_64_bit(fd: c_int) -> c_int {
    let mut e_ident: [u8; EI_NIDENT] = [0; EI_NIDENT];

    if lseek(fd, 0, SEEK_SET) != 0 {
        return -1;
    }

    if readn(fd, e_ident.as_mut_ptr() as *mut c_void, size_of_val(&e_ident)) != size_of_val(&e_ident) as ssize_t {
        return -1;
    }

    if memcmp(
        e_ident.as_ptr() as *const c_void,
        ELFMAG.as_ptr() as *const c_void,
        SELFMAG,
    ) != 0 || e_ident[EI_VERSION] != EV_CURRENT
    {
        return -1;
    }

    (e_ident[EI_CLASS] == ELFCLASS64) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__type_fd(fd: c_int) -> dso_type {
    let mut ehdr: Elf64_Ehdr = core::mem::zeroed();
    let ret: c_int;

    ret = fd__is_64_bit(fd);
    if ret < 0 {
        return dso_type::DSO__TYPE_UNKNOWN;
    }

    if ret != 0 {
        return dso_type::DSO__TYPE_64BIT;
    }

    if readn(fd, &mut ehdr as *mut Elf64_Ehdr as *mut c_void, size_of::<Elf64_Ehdr>()) != size_of::<Elf64_Ehdr>() as ssize_t {
        return dso_type::DSO__TYPE_UNKNOWN;
    }

    if ehdr.e_machine == EM_X86_64 {
        return dso_type::DSO__TYPE_X32BIT;
    }

    dso_type::DSO__TYPE_32BIT
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__load_sym(
    dso: *mut dso,
    _map: *mut map,
    ss: *mut symsrc,
    _runtime_ss: *mut symsrc,
    _kmodule: c_int,
) -> c_int {
    let mut bid = build_id {
        data: [0; 32],
        size: 0,
    };
    let ret: c_int;

    ret = fd__is_64_bit((*ss).fd);
    if ret >= 0 {
        (*RC_CHK_ACCESS(dso)).is_64_bit = ret;
    }

    if filename__read_build_id((*ss).name, &mut bid) > 0 {
        dso__set_build_id(dso, &mut bid);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file__read_maps(
    _fd: c_int,
    _exe: bool,
    _mapfn: mapfn_t,
    _data: *mut c_void,
    _is_64_bit: *mut bool,
) -> c_int {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kcore_extract__create(_kce: *mut kcore_extract) -> c_int {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kcore_extract__delete(_kce: *mut kcore_extract) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kcore_copy(_from_dir: *const c_char, _to_dir: *const c_char) -> c_int {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__elf_init() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename__has_section(_filename: *const c_char, _sec: *const c_char) -> bool {
    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
