// SPDX-License-Identifier: GPL-2.0
/*  Copyright(c) 2016-20 Intel Corporation. */

/* Translated from C. Dependencies originally provided by:
 * assert.h, elf.h, errno.h, fcntl.h, stdbool.h, stdio.h, stdint.h, stdlib.h,
 * string.h, unistd.h, sys/ioctl.h, sys/mman.h, sys/stat.h, sys/time.h,
 * sys/types.h, "defines.h", and "main.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type off_t = i64;
type size_t = usize;
type uint64_t = u64;

extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    static mut stderr: *mut FILE;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sgx_secs {
    pub size: uint64_t,
    pub base: uint64_t,
    pub ssa_frame_size: u32,
    pub miscselect: u32,
    pub attributes: uint64_t,
    pub xfrm: uint64_t,
}

#[repr(C)]
pub struct sgx_secinfo {
    pub flags: uint64_t,
    pub reserved: [uint64_t; 7],
}

#[repr(C)]
pub struct sgx_enclave_create {
    pub src: c_ulong,
}

#[repr(C)]
pub struct sgx_enclave_add_pages {
    pub src: uint64_t,
    pub offset: uint64_t,
    pub length: uint64_t,
    pub secinfo: c_ulong,
    pub flags: uint64_t,
    pub count: uint64_t,
}

#[repr(C)]
pub struct sgx_enclave_init {
    pub sigstruct: uint64_t,
}

#[repr(C)]
pub struct sgx_sigstruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct encl_segment {
    pub offset: uint64_t,
    pub size: uint64_t,
    pub src: *mut c_void,
    pub prot: c_int,
    pub flags: uint64_t,
    pub measure: bool,
}

#[repr(C)]
pub struct encl {
    pub secs: sgx_secs,
    pub sigstruct: sgx_sigstruct,
    pub encl_base: uint64_t,
    pub encl_size: uint64_t,
    pub bin: *mut c_void,
    pub bin_size: uint64_t,
    pub fd: c_int,
    pub segment_tbl: *mut encl_segment,
    pub nr_segments: c_int,
    pub src: *mut c_void,
    pub src_size: uint64_t,
}

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: uint64_t,
    pub e_phoff: uint64_t,
    pub e_shoff: uint64_t,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: uint64_t,
    pub p_vaddr: uint64_t,
    pub p_paddr: uint64_t,
    pub p_filesz: uint64_t,
    pub p_memsz: uint64_t,
    pub p_align: uint64_t,
}

#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: uint64_t,
    pub sh_addr: uint64_t,
    pub sh_offset: uint64_t,
    pub sh_size: uint64_t,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: uint64_t,
    pub sh_entsize: uint64_t,
}

#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: uint64_t,
    pub st_size: uint64_t,
}

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const PROT_NONE: c_int = 0x0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const PAGE_SIZE: uint64_t = 4096;
const PAGE_MASK: uint64_t = !(PAGE_SIZE - 1);
const PT_LOAD: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const PF_X: c_uint = 1;
const PF_W: c_uint = 2;
const PF_R: c_uint = 4;
const SGX_ATTR_MODE64BIT: uint64_t = 0x0000_0000_0000_0004;
const SGX_PAGE_TYPE_REG: uint64_t = 0x01;
const SGX_PAGE_TYPE_TCS: uint64_t = 0x05;
const SGX_PAGE_MEASURE: uint64_t = 0x01;

/* ioctl request values are supplied by Linux SGX headers in the original build. */
extern "C" {
    static SGX_IOC_ENCLAVE_CREATE: c_ulong;
    static SGX_IOC_ENCLAVE_ADD_PAGES: c_ulong;
    static SGX_IOC_ENCLAVE_INIT: c_ulong;
}

#[no_mangle]
pub unsafe extern "C" fn encl_delete(encl: *mut encl) {
    let mut heap_seg: *mut encl_segment;

    if (*encl).encl_base != 0 {
        munmap((*encl).encl_base as *mut c_void, (*encl).encl_size as size_t);
    }

    if !(*encl).bin.is_null() {
        munmap((*encl).bin, (*encl).bin_size as size_t);
    }

    if (*encl).fd != 0 {
        close((*encl).fd);
    }

    if !(*encl).segment_tbl.is_null() {
        heap_seg = (*encl).segment_tbl.offset(((*encl).nr_segments - 1) as isize);
        munmap((*heap_seg).src, (*heap_seg).size as size_t);
        free((*encl).segment_tbl as *mut c_void);
    }

    memset(
        encl as *mut c_void,
        0,
        core::mem::size_of_val(&*encl) as size_t,
    );
}

unsafe fn encl_map_bin(path: *const c_char, encl: *mut encl) -> bool {
    let mut sb: stat = core::mem::zeroed();
    let bin: *mut c_void;
    let mut ret: c_int;
    let fd: c_int;

    fd = open(path, O_RDONLY);
    if fd == -1 {
        perror(c"enclave executable open()".as_ptr());
        return false;
    }

    ret = stat(path, &mut sb);
    if ret != 0 {
        perror(c"enclave executable stat()".as_ptr());
        close(fd);
        return false;
    }

    let st_size = *(&sb as *const stat as *const i64).offset(6) as size_t;
    bin = mmap(core::ptr::null_mut(), st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if bin == MAP_FAILED {
        perror(c"enclave executable mmap()".as_ptr());
        close(fd);
        return false;
    }

    (*encl).bin = bin;
    (*encl).bin_size = st_size as uint64_t;

    close(fd);
    true
}

unsafe fn encl_ioc_create(encl: *mut encl) -> bool {
    let secs: *mut sgx_secs = &mut (*encl).secs;
    let mut ioc: sgx_enclave_create = core::mem::zeroed();
    let rc: c_int;

    assert!((*encl).encl_base != 0);

    memset(
        secs as *mut c_void,
        0,
        core::mem::size_of::<sgx_secs>() as size_t,
    );
    (*secs).ssa_frame_size = 1;
    (*secs).attributes = SGX_ATTR_MODE64BIT;
    (*secs).xfrm = 3;
    (*secs).base = (*encl).encl_base;
    (*secs).size = (*encl).encl_size;

    ioc.src = secs as c_ulong;
    rc = ioctl((*encl).fd, SGX_IOC_ENCLAVE_CREATE, &mut ioc);
    if rc != 0 {
        perror(c"SGX_IOC_ENCLAVE_CREATE failed".as_ptr());
        munmap((*secs).base as *mut c_void, (*encl).encl_size as size_t);
        return false;
    }

    true
}

unsafe fn encl_ioc_add_pages(encl: *mut encl, seg: *mut encl_segment) -> bool {
    let mut ioc: sgx_enclave_add_pages = core::mem::zeroed();
    let mut secinfo: sgx_secinfo = core::mem::zeroed();
    let rc: c_int;

    memset(
        &mut secinfo as *mut sgx_secinfo as *mut c_void,
        0,
        core::mem::size_of::<sgx_secinfo>() as size_t,
    );
    secinfo.flags = (*seg).flags;

    ioc.src = (*seg).src as uint64_t;
    ioc.offset = (*seg).offset;
    ioc.length = (*seg).size;
    ioc.secinfo = &mut secinfo as *mut sgx_secinfo as c_ulong;
    if (*seg).measure {
        ioc.flags = SGX_PAGE_MEASURE;
    } else {
        ioc.flags = 0;
    }

    rc = ioctl((*encl).fd, SGX_IOC_ENCLAVE_ADD_PAGES, &mut ioc);
    if rc < 0 {
        perror(c"SGX_IOC_ENCLAVE_ADD_PAGES failed".as_ptr());
        return false;
    }

    true
}

/*
 * Parse the enclave code's symbol table to locate and return address of
 * the provided symbol
 */
#[no_mangle]
pub unsafe extern "C" fn encl_get_entry(encl: *mut encl, symbol: *const c_char) -> uint64_t {
    let mut symtab: *mut Elf64_Sym = core::ptr::null_mut();
    let mut sym_names: *mut c_char = core::ptr::null_mut();
    let sections: *mut Elf64_Shdr;
    let ehdr: *mut Elf64_Ehdr;
    let mut num_sym: c_int = 0;
    let mut i: c_int;

    ehdr = (*encl).bin as *mut Elf64_Ehdr;
    sections = ((*encl).bin as *mut u8).offset((*ehdr).e_shoff as isize) as *mut Elf64_Shdr;

    i = 0;
    while i < (*ehdr).e_shnum as c_int {
        if (*sections.offset(i as isize)).sh_type == SHT_SYMTAB {
            symtab = ((*encl).bin as *mut u8)
                .offset((*sections.offset(i as isize)).sh_offset as isize)
                as *mut Elf64_Sym;
            num_sym = ((*sections.offset(i as isize)).sh_size
                / (*sections.offset(i as isize)).sh_entsize) as c_int;
            break;
        }
        i += 1;
    }

    i = 0;
    while i < (*ehdr).e_shnum as c_int {
        if (*sections.offset(i as isize)).sh_type == SHT_STRTAB {
            sym_names = ((*encl).bin as *mut u8)
                .offset((*sections.offset(i as isize)).sh_offset as isize)
                as *mut c_char;
            break;
        }
        i += 1;
    }

    if symtab.is_null() || sym_names.is_null() {
        return 0;
    }

    i = 0;
    while i < num_sym {
        let sym: *mut Elf64_Sym = symtab.offset(i as isize);

        if strcmp(symbol, sym_names.offset((*sym).st_name as isize)) == 0 {
            return (*sym).st_value as uint64_t;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn encl_load(
    path: *const c_char,
    encl: *mut encl,
    heap_size: c_ulong,
) -> bool {
    let device_path = c"/dev/sgx_enclave";
    let mut seg: *mut encl_segment;
    let phdr_tbl: *mut Elf64_Phdr;
    let mut src_offset: off_t = 0;
    let ehdr: *mut Elf64_Ehdr;
    let mut sb: stat = core::mem::zeroed();
    let mut ptr: *mut c_void;
    let mut i: c_int;
    let mut j: c_int;
    let mut ret: c_int;
    let mut fd: c_int = -1;

    memset(
        encl as *mut c_void,
        0,
        core::mem::size_of_val(&*encl) as size_t,
    );

    fd = open(device_path.as_ptr(), O_RDWR);
    if fd < 0 {
        perror(c"Unable to open /dev/sgx_enclave".as_ptr());
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }

    ret = stat(device_path.as_ptr(), &mut sb);
    if ret != 0 {
        perror(c"device file stat()".as_ptr());
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }

    ptr = mmap(
        core::ptr::null_mut(),
        PAGE_SIZE as size_t,
        PROT_READ,
        MAP_SHARED,
        fd,
        0,
    );
    if ptr == -1isize as *mut c_void {
        perror(c"mmap for read".as_ptr());
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }
    munmap(ptr, PAGE_SIZE as size_t);

    let err_msg = c"mmap() succeeded for PROT_READ, but failed for PROT_EXEC.\n Check that /dev does not have noexec set:\n \tmount | grep \"/dev .*noexec\"\n If so, remount it executable: mount -o remount,exec /dev\n\n";

    ptr = mmap(
        core::ptr::null_mut(),
        PAGE_SIZE as size_t,
        PROT_EXEC,
        MAP_SHARED,
        fd,
        0,
    );
    if ptr == -1isize as *mut c_void {
        fprintf(stderr, err_msg.as_ptr());
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }
    munmap(ptr, PAGE_SIZE as size_t);

    (*encl).fd = fd;

    if !encl_map_bin(path, encl) {
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }

    ehdr = (*encl).bin as *mut Elf64_Ehdr;
    phdr_tbl = ((*encl).bin as *mut u8).offset((*ehdr).e_phoff as isize) as *mut Elf64_Phdr;

    (*encl).nr_segments = 1; /* one for the heap */

    i = 0;
    while i < (*ehdr).e_phnum as c_int {
        let phdr: *mut Elf64_Phdr = phdr_tbl.offset(i as isize);

        if (*phdr).p_type == PT_LOAD {
            (*encl).nr_segments += 1;
        }
        i += 1;
    }

    (*encl).segment_tbl = calloc(
        (*encl).nr_segments as size_t,
        core::mem::size_of::<encl_segment>() as size_t,
    ) as *mut encl_segment;
    if (*encl).segment_tbl.is_null() {
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }

    i = 0;
    j = 0;
    while i < (*ehdr).e_phnum as c_int {
        let phdr: *mut Elf64_Phdr = phdr_tbl.offset(i as isize);
        let flags: c_uint = (*phdr).p_flags;

        if (*phdr).p_type != PT_LOAD {
            i += 1;
            continue;
        }

        seg = (*encl).segment_tbl.offset(j as isize);

        if (flags & !(PF_R | PF_W | PF_X)) != 0 {
            fprintf(
                stderr,
                c"%d has invalid segment flags 0x%02x.\n".as_ptr(),
                i,
                (*phdr).p_flags,
            );
            if fd != -1 {
                close(fd);
            }
            encl_delete(encl);
            return false;
        }

        if j == 0 && flags != (PF_R | PF_W) {
            fprintf(
                stderr,
                c"TCS has invalid segment flags 0x%02x.\n".as_ptr(),
                (*phdr).p_flags,
            );
            if fd != -1 {
                close(fd);
            }
            encl_delete(encl);
            return false;
        }

        if j == 0 {
            src_offset = ((*phdr).p_offset & PAGE_MASK) as off_t;
            (*encl).src = ((*encl).bin as *mut u8).offset(src_offset as isize) as *mut c_void;

            (*seg).prot = PROT_READ | PROT_WRITE;
            (*seg).flags = SGX_PAGE_TYPE_TCS << 8;
        } else {
            (*seg).prot = if ((*phdr).p_flags & PF_R) != 0 {
                PROT_READ
            } else {
                0
            };
            (*seg).prot |= if ((*phdr).p_flags & PF_W) != 0 {
                PROT_WRITE
            } else {
                0
            };
            (*seg).prot |= if ((*phdr).p_flags & PF_X) != 0 {
                PROT_EXEC
            } else {
                0
            };
            (*seg).flags = (SGX_PAGE_TYPE_REG << 8) | (*seg).prot as uint64_t;
        }

        (*seg).offset = ((*phdr).p_offset & PAGE_MASK).wrapping_sub(src_offset as uint64_t);
        (*seg).size = ((*phdr).p_filesz + PAGE_SIZE - 1) & PAGE_MASK;
        (*seg).src = ((*encl).src as *mut u8).offset((*seg).offset as isize) as *mut c_void;
        (*seg).measure = true;

        j += 1;
        i += 1;
    }

    assert!(j == (*encl).nr_segments - 1);

    seg = (*encl).segment_tbl.offset(j as isize);
    (*seg).offset = (*(*encl).segment_tbl.offset((j - 1) as isize)).offset
        + (*(*encl).segment_tbl.offset((j - 1) as isize)).size;
    (*seg).size = heap_size as uint64_t;
    (*seg).src = mmap(
        core::ptr::null_mut(),
        heap_size as size_t,
        PROT_READ | PROT_WRITE,
        MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
    );
    (*seg).prot = PROT_READ | PROT_WRITE;
    (*seg).flags = (SGX_PAGE_TYPE_REG << 8) | (*seg).prot as uint64_t;
    (*seg).measure = false;

    if (*seg).src == MAP_FAILED {
        if fd != -1 {
            close(fd);
        }
        encl_delete(encl);
        return false;
    }

    (*encl).src_size = (*(*encl).segment_tbl.offset(j as isize)).offset
        + (*(*encl).segment_tbl.offset(j as isize)).size;

    (*encl).encl_size = 4096;
    while (*encl).encl_size < (*encl).src_size {
        (*encl).encl_size <<= 1;
    }

    true
}

unsafe fn encl_map_area(encl: *mut encl) -> bool {
    let encl_size: size_t = (*encl).encl_size as size_t;
    let area: *mut c_void;

    area = mmap(
        core::ptr::null_mut(),
        encl_size * 2,
        PROT_NONE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if area == MAP_FAILED {
        perror(c"reservation mmap()".as_ptr());
        return false;
    }

    (*encl).encl_base =
        ((area as uint64_t) + encl_size as uint64_t - 1) & !((encl_size as uint64_t) - 1);

    munmap(
        area,
        ((*encl).encl_base - area as uint64_t) as size_t,
    );
    munmap(
        ((*encl).encl_base + encl_size as uint64_t) as *mut c_void,
        ((area as uint64_t) + encl_size as uint64_t - (*encl).encl_base) as size_t,
    );

    true
}

#[no_mangle]
pub unsafe extern "C" fn encl_build(encl: *mut encl) -> bool {
    let mut ioc: sgx_enclave_init = core::mem::zeroed();
    let ret: c_int;
    let mut i: c_int;

    if !encl_map_area(encl) {
        return false;
    }

    if !encl_ioc_create(encl) {
        return false;
    }

    /*
     * Pages must be added before mapping VMAs because their permissions
     * cap the VMA permissions.
     */
    i = 0;
    while i < (*encl).nr_segments {
        let seg: *mut encl_segment = (*encl).segment_tbl.offset(i as isize);

        if !encl_ioc_add_pages(encl, seg) {
            return false;
        }
        i += 1;
    }

    ioc.sigstruct = &mut (*encl).sigstruct as *mut sgx_sigstruct as uint64_t;
    ret = ioctl((*encl).fd, SGX_IOC_ENCLAVE_INIT, &mut ioc);
    if ret != 0 {
        perror(c"SGX_IOC_ENCLAVE_INIT failed".as_ptr());
        return false;
    }

    true
}
