// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type pid_t = c_int;
type __u8 = u8;
type __u16 = u16;
type __u64 = u64;

const USDT_BASE_SEC: &[u8] = b".stapsdt.base\0";
const USDT_SEMA_SEC: &[u8] = b".probes\0";
const USDT_NOTE_SEC: &[u8] = b".note.stapsdt\0";
const USDT_NOTE_TYPE: c_uint = 3;
const USDT_NOTE_NAME: &[u8] = b"stapsdt\0";
const USDT_MAX_ARG_CNT: usize = 12;

const ESRCH: c_int = 3;
const EBADF: c_int = 9;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOTSUP: c_int = 95;
const E2BIG: c_int = 7;
const EEXIST: c_int = 17;

const ELF_K_ELF: c_int = 3;
const ELFCLASS32: c_int = 1;
const ELFCLASS64: c_int = 2;
const ELFDATA2LSB: c_int = 1;
const ELFDATA2MSB: c_int = 2;
const EI_DATA: usize = 5;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const SHT_NOTE: u32 = 7;
const PATH_MAX: usize = 4096;
const AT_FDCWD: c_int = -100;
const F_OK: c_int = 0;
const AT_EACCESS: c_int = 0x200;
const BPF_ANY: __u64 = 0;
const BPF_NOEXIST: __u64 = 1;
const FEAT_BPF_COOKIE: c_int = 0;
const FEAT_UPROBE_MULTI_LINK: c_int = 1;
const FEAT_UPROBE_SYSCALL: c_int = 2;

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
}
#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    pub detach: Option<unsafe extern "C" fn(*mut bpf_link) -> c_int>,
    pub dealloc: Option<unsafe extern "C" fn(*mut bpf_link)>,
}

#[repr(C)]
pub struct bpf_uprobe_opts {
    pub sz: size_t,
    pub ref_ctr_offset: c_ulong,
    pub bpf_cookie: __u64,
}

#[repr(C)]
pub struct bpf_uprobe_multi_opts {
    pub sz: size_t,
    pub ref_ctr_offsets: *mut c_ulong,
    pub offsets: *mut c_ulong,
    pub cookies: *mut __u64,
    pub cnt: size_t,
}

#[repr(C)]
pub struct elf_fd {
    pub elf: *mut Elf,
    pub fd: c_int,
}

#[repr(C)]
pub struct GElf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
}

#[repr(C)]
pub struct GElf_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_addr: c_ulong,
}

#[repr(C)]
pub struct GElf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: c_ulong,
    pub p_vaddr: c_ulong,
    pub p_memsz: c_ulong,
}

#[repr(C)]
pub struct GElf_Nhdr {
    pub n_namesz: u32,
    pub n_descsz: u32,
    pub n_type: u32,
}

#[repr(C)]
pub struct pt_regs {
    pub r15: c_ulong, pub r14: c_ulong, pub r13: c_ulong, pub r12: c_ulong,
    pub rbp: c_ulong, pub rbx: c_ulong, pub r11: c_ulong, pub r10: c_ulong,
    pub r9: c_ulong, pub r8: c_ulong, pub rax: c_ulong, pub rcx: c_ulong,
    pub rdx: c_ulong, pub rsi: c_ulong, pub rdi: c_ulong, pub orig_rax: c_ulong,
    pub rip: c_ulong, pub cs: c_ulong, pub eflags: c_ulong, pub rsp: c_ulong, pub ss: c_ulong,
    pub eax: c_ulong, pub ebx: c_ulong, pub ecx: c_ulong, pub edx: c_ulong,
    pub esi: c_ulong, pub edi: c_ulong, pub ebp: c_ulong, pub esp: c_ulong, pub eip: c_ulong,
    pub uregs: [c_ulong; 18],
}

#[repr(C)]
pub struct user_pt_regs {
    pub regs: [c_ulong; 31],
    pub sp: c_ulong,
    pub pc: c_ulong,
    pub pstate: c_ulong,
    pub gprs: [c_ulong; 16],
}

#[repr(C)]
pub struct user_regs_struct {
    pub ra: c_ulong, pub sp: c_ulong, pub gp: c_ulong, pub tp: c_ulong,
    pub t0: c_ulong, pub t1: c_ulong, pub t2: c_ulong,
    pub s0: c_ulong, pub s1: c_ulong,
    pub a0: c_ulong, pub a1: c_ulong, pub a2: c_ulong, pub a3: c_ulong,
    pub a4: c_ulong, pub a5: c_ulong, pub a6: c_ulong, pub a7: c_ulong,
    pub s2: c_ulong, pub s3: c_ulong, pub s4: c_ulong, pub s5: c_ulong,
    pub s6: c_ulong, pub s7: c_ulong, pub rv_s8: c_ulong, pub s9: c_ulong,
    pub s10: c_ulong, pub s11: c_ulong, pub t3: c_ulong, pub t4: c_ulong,
    pub t5: c_ulong, pub t6: c_ulong,
}

unsafe extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn isblank(c: c_int) -> c_int;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: c_long) -> isize;
    fn getpid() -> pid_t;
    fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    static mut errno: c_int;

    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> __u64;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__attach_uprobe_opts(prog: *const bpf_program, pid: pid_t, path: *const c_char, offset: c_ulong, opts: *const bpf_uprobe_opts) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_multi(prog: *const bpf_program, pid: pid_t, path: *const c_char, func_name: *const c_char, opts: *const bpf_uprobe_multi_opts) -> *mut bpf_link;
    fn kernel_supports(obj: *mut bpf_object, feat: c_int) -> bool;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_err_ptr(err: c_int) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn errstr(err: c_int) -> *const c_char;
    fn libbpf_strlcpy(dst: *mut c_char, src: *const c_char, sz: size_t) -> size_t;
    fn elf_open(path: *const c_char, elf_fd: *mut elf_fd) -> c_int;
    fn elf_close(elf_fd: *mut elf_fd);
    fn elf_kind(elf: *mut Elf) -> c_int;
    fn gelf_getclass(elf: *mut Elf) -> c_int;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getscn(elf: *mut Elf, index: size_t) -> *mut Elf_Scn;
    fn elf_rawdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *mut c_char;
    fn elf_getphdrnum(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn gelf_getphdr(elf: *mut Elf, ndx: c_int, dst: *mut GElf_Phdr) -> *mut GElf_Phdr;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn gelf_getnote(data: *mut Elf_Data, offset: size_t, nhdr: *mut GElf_Nhdr, name_off: *mut size_t, desc_off: *mut size_t) -> size_t;
    fn hashmap__new(hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>, equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut c_long) -> bool;
    fn hashmap__add(map: *mut hashmap, key: *const c_char, value: c_long) -> c_int;
    fn str_hash(s: *mut c_char) -> size_t;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn IS_ERR_OR_NULL<T>(ptr: *mut T) -> bool {
    ptr.is_null() || (ptr as isize) < 0 && (ptr as isize) > -4096
}
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}
unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

/* libbpf's USDT support consists of BPF-side state/code and user-space
 * state/code working together in concert. BPF-side parts are defined in
 * usdt.bpf.h header library. User-space state is encapsulated by struct
 * usdt_manager and all the supporting code centered around usdt_manager.
 */

/* should match exactly enum __bpf_usdt_arg_type from usdt.bpf.h */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usdt_arg_type {
    USDT_ARG_CONST,
    USDT_ARG_REG,
    USDT_ARG_REG_DEREF,
    USDT_ARG_SIB,
}

/* should match exactly struct __bpf_usdt_arg_spec from usdt.bpf.h */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usdt_arg_spec {
    pub val_off: __u64,
    /* C source uses endian-dependent bitfields here:
     * arg_type:8, idx_reg_off:12, scale_bitshift:4, __reserved:8.
     * Kept as explicit storage fields for source-level translation.
     */
    pub arg_type: usdt_arg_type,
    pub idx_reg_off: __u16,
    pub scale_bitshift: __u16,
    pub __reserved: __u8,
    pub reg_off: i16,
    pub arg_signed: bool,
    pub arg_bitshift: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usdt_spec {
    pub args: [usdt_arg_spec; USDT_MAX_ARG_CNT],
    pub usdt_cookie: __u64,
    pub arg_cnt: i16,
}

#[repr(C)]
pub struct usdt_note {
    pub provider: *const c_char,
    pub name: *const c_char,
    /* USDT args specification string, e.g.:
     * "-4@%esi -4@-24(%rbp) -4@%ecx 2@%ax 8@%rdx"
     */
    pub args: *const c_char,
    pub loc_addr: c_long,
    pub base_addr: c_long,
    pub sema_addr: c_long,
}

#[repr(C)]
pub struct usdt_target {
    pub abs_ip: c_long,
    pub rel_ip: c_long,
    pub sema_off: c_long,
    pub spec: usdt_spec,
    pub spec_str: *const c_char,
}

#[repr(C)]
pub struct usdt_manager {
    pub specs_map: *mut bpf_map,
    pub ip_to_spec_id_map: *mut bpf_map,
    pub free_spec_ids: *mut c_int,
    pub free_spec_cnt: size_t,
    pub next_free_spec_id: size_t,
    pub has_bpf_cookie: bool,
    pub has_sema_refcnt: bool,
    pub has_uprobe_multi: bool,
    pub has_uprobe_syscall: bool,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_manager_new(obj: *mut bpf_object) -> *mut usdt_manager {
    static ref_ctr_sysfs_path: &[u8] = b"/sys/bus/event_source/devices/uprobe/format/ref_ctr_offset\0";
    let specs_map = unsafe { bpf_object__find_map_by_name(obj, c"__bpf_usdt_specs".as_ptr()) };
    let ip_to_spec_id_map = unsafe { bpf_object__find_map_by_name(obj, c"__bpf_usdt_ip_to_spec_id".as_ptr()) };
    if specs_map.is_null() || ip_to_spec_id_map.is_null() {
        unsafe { pr_warn(c"usdt: failed to find USDT support BPF maps, did you forget to include bpf/usdt.bpf.h?\n".as_ptr()) };
        return unsafe { ERR_PTR(-ESRCH) };
    }

    let man = unsafe { calloc(1, size_of::<usdt_manager>()) as *mut usdt_manager };
    if man.is_null() {
        return unsafe { ERR_PTR(-ENOMEM) };
    }

    unsafe {
        (*man).specs_map = specs_map;
        (*man).ip_to_spec_id_map = ip_to_spec_id_map;
        (*man).has_bpf_cookie = kernel_supports(obj, FEAT_BPF_COOKIE);
        (*man).has_sema_refcnt = faccessat(AT_FDCWD, ref_ctr_sysfs_path.as_ptr() as *const c_char, F_OK, AT_EACCESS) == 0;
        (*man).has_uprobe_multi = kernel_supports(obj, FEAT_UPROBE_MULTI_LINK);
        (*man).has_uprobe_syscall = kernel_supports(obj, FEAT_UPROBE_SYSCALL);
    }
    man
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_manager_free(man: *mut usdt_manager) {
    if unsafe { IS_ERR_OR_NULL(man) } {
        return;
    }
    unsafe {
        free((*man).free_spec_ids as *mut c_void);
        free(man as *mut c_void);
    }
}

unsafe fn sanity_check_usdt_elf(elf: *mut Elf, path: *const c_char) -> c_int {
    let mut ehdr: GElf_Ehdr = unsafe { core::mem::zeroed() };
    let endianness: c_int;

    if unsafe { elf_kind(elf) } != ELF_K_ELF {
        unsafe { pr_warn(c"usdt: unrecognized ELF kind %u for '%s'\n".as_ptr(), elf_kind(elf), path) };
        return -EBADF;
    }

    match unsafe { gelf_getclass(elf) } {
        ELFCLASS64 => {
            if size_of::<*mut c_void>() != 8 {
                unsafe { pr_warn(c"usdt: attaching to 64-bit ELF binary '%s' is not supported\n".as_ptr(), path) };
                return -EBADF;
            }
        }
        ELFCLASS32 => {
            if size_of::<*mut c_void>() != 4 {
                unsafe { pr_warn(c"usdt: attaching to 32-bit ELF binary '%s' is not supported\n".as_ptr(), path) };
                return -EBADF;
            }
        }
        _ => {
            unsafe { pr_warn(c"usdt: unsupported ELF class for '%s'\n".as_ptr(), path) };
            return -EBADF;
        }
    }

    if unsafe { gelf_getehdr(elf, &mut ehdr) }.is_null() {
        return -EINVAL;
    }
    if ehdr.e_type != ET_EXEC && ehdr.e_type != ET_DYN {
        unsafe { pr_warn(c"usdt: unsupported type of ELF binary '%s' (%d), only ET_EXEC and ET_DYN are supported\n".as_ptr(), path, ehdr.e_type as c_int) };
        return -EBADF;
    }

    #[cfg(target_endian = "little")]
    { endianness = ELFDATA2LSB; }
    #[cfg(target_endian = "big")]
    { endianness = ELFDATA2MSB; }
    if endianness != ehdr.e_ident[EI_DATA] as c_int {
        unsafe { pr_warn(c"usdt: ELF endianness mismatch for '%s'\n".as_ptr(), path) };
        return -EBADF;
    }
    0
}

unsafe fn find_elf_sec_by_name(elf: *mut Elf, sec_name: *const c_char, shdr: *mut GElf_Shdr, scn: *mut *mut Elf_Scn) -> c_int {
    let mut sec: *mut Elf_Scn = ptr::null_mut();
    let mut shstrndx: size_t = 0;
    if unsafe { elf_getshdrstrndx(elf, &mut shstrndx) } != 0 {
        return -EINVAL;
    }
    if unsafe { elf_rawdata(elf_getscn(elf, shstrndx), ptr::null_mut()) }.is_null() {
        return -EINVAL;
    }
    loop {
        sec = unsafe { elf_nextscn(elf, sec) };
        if sec.is_null() {
            break;
        }
        if unsafe { gelf_getshdr(sec, shdr) }.is_null() {
            return -EINVAL;
        }
        let name = unsafe { elf_strptr(elf, shstrndx, (*shdr).sh_name as size_t) };
        if !name.is_null() && unsafe { strcmp(sec_name, name) } == 0 {
            unsafe { *scn = sec };
            return 0;
        }
    }
    -ENOENT
}

#[repr(C)]
pub struct elf_seg {
    pub start: c_long,
    pub end: c_long,
    pub offset: c_long,
    pub is_exec: bool,
}

unsafe extern "C" fn cmp_elf_segs(_a: *const c_void, _b: *const c_void) -> c_int {
    let a = _a as *const elf_seg;
    let b = _b as *const elf_seg;
    if unsafe { (*a).start < (*b).start } { -1 } else { 1 }
}

unsafe fn parse_elf_segs(elf: *mut Elf, path: *const c_char, segs: *mut *mut elf_seg, seg_cnt: *mut size_t) -> c_int {
    let mut phdr: GElf_Phdr = unsafe { core::mem::zeroed() };
    let mut n: size_t = 0;
    let mut err: c_int;
    unsafe { *seg_cnt = 0 };
    if unsafe { elf_getphdrnum(elf, &mut n) } != 0 {
        return unsafe { -errno };
    }
    for i in 0..n {
        if unsafe { gelf_getphdr(elf, i as c_int, &mut phdr) }.is_null() {
            return unsafe { -errno };
        }
        unsafe {
            pr_debug(c"usdt: discovered PHDR #%d in '%s': vaddr 0x%lx memsz 0x%lx offset 0x%lx type 0x%lx flags 0x%lx\n".as_ptr(),
                     i as c_int, path, phdr.p_vaddr, phdr.p_memsz, phdr.p_offset, phdr.p_type as c_ulong, phdr.p_flags as c_ulong);
        }
        if phdr.p_type != PT_LOAD {
            continue;
        }
        let tmp = unsafe { libbpf_reallocarray(*segs as *mut c_void, *seg_cnt + 1, size_of::<elf_seg>()) as *mut elf_seg };
        if tmp.is_null() {
            return -ENOMEM;
        }
        unsafe {
            *segs = tmp;
            let seg = (*segs).add(*seg_cnt);
            *seg_cnt += 1;
            (*seg).start = phdr.p_vaddr as c_long;
            (*seg).end = phdr.p_vaddr.wrapping_add(phdr.p_memsz) as c_long;
            (*seg).offset = phdr.p_offset as c_long;
            (*seg).is_exec = (phdr.p_flags & PF_X) != 0;
        }
    }
    if unsafe { *seg_cnt } == 0 {
        unsafe { pr_warn(c"usdt: failed to find PT_LOAD program headers in '%s'\n".as_ptr(), path) };
        return -ESRCH;
    }
    unsafe { qsort(*segs as *mut c_void, *seg_cnt, size_of::<elf_seg>(), cmp_elf_segs) };
    err = 0;
    err
}

unsafe fn parse_vma_segs(pid: pid_t, lib_path: *const c_char, segs: *mut *mut elf_seg, seg_cnt: *mut size_t) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut line = [0 as c_char; 4096];
    let mut mode = [0 as c_char; 16];
    let mut seg_start: size_t = 0;
    let mut seg_end: size_t = 0;
    let mut seg_off: size_t = 0;
    let mut tmp_pid: c_int = 0;
    let mut n: c_int = 0;
    let mut err: c_int;
    unsafe { *seg_cnt = 0 };

    if unsafe { sscanf(lib_path, c"/proc/%d/root%n".as_ptr(), &mut tmp_pid, &mut n) } == 1
        && n > 0 && pid == tmp_pid && unsafe { *lib_path.add(n as usize) } == b'/' as c_char {
        unsafe { libbpf_strlcpy(path.as_mut_ptr(), lib_path.add(n as usize), path.len()) };
    } else if unsafe { realpath(lib_path, path.as_mut_ptr()) }.is_null() {
        unsafe {
            pr_warn(c"usdt: failed to get absolute path of '%s' (err %s), using path as is...\n".as_ptr(), lib_path, errstr(-errno));
            libbpf_strlcpy(path.as_mut_ptr(), lib_path, path.len());
        }
    }

    unsafe { sprintf(line.as_mut_ptr(), c"/proc/%d/maps".as_ptr(), pid) };
    let f = unsafe { fopen(line.as_ptr(), c"re".as_ptr()) };
    if f.is_null() {
        err = unsafe { -errno };
        unsafe { pr_warn(c"usdt: failed to open '%s' to get base addr of '%s': %s\n".as_ptr(), line.as_ptr(), lib_path, errstr(err)) };
        return err;
    }

    while unsafe {
        fscanf(f, c"%zx-%zx %15s %zx %*s %*d%4095[^\n]%*[^\n]\n".as_ptr(),
               &mut seg_start, &mut seg_end, mode.as_mut_ptr(), &mut seg_off, line.as_mut_ptr())
    } == 5 {
        let mut i = 0usize;
        while unsafe { isblank(line[i] as c_int) } != 0 {
            i += 1;
        }
        if unsafe { strcmp(line.as_ptr().add(i), path.as_ptr()) } != 0 {
            continue;
        }
        unsafe { pr_debug(c"usdt: discovered segment for lib '%s': addrs %zx-%zx mode %s offset %zx\n".as_ptr(), path.as_ptr(), seg_start, seg_end, mode.as_ptr(), seg_off) };
        if mode[2] != b'x' as c_char {
            continue;
        }
        let tmp = unsafe { libbpf_reallocarray(*segs as *mut c_void, *seg_cnt + 1, size_of::<elf_seg>()) as *mut elf_seg };
        if tmp.is_null() {
            err = -ENOMEM;
            unsafe { fclose(f) };
            return err;
        }
        unsafe {
            *segs = tmp;
            let seg = (*segs).add(*seg_cnt);
            *seg_cnt += 1;
            (*seg).start = seg_start as c_long;
            (*seg).end = seg_end as c_long;
            (*seg).offset = seg_off as c_long;
            (*seg).is_exec = true;
        }
    }

    if unsafe { *seg_cnt } == 0 {
        unsafe { pr_warn(c"usdt: failed to find '%s' (resolved to '%s') within PID %d memory mappings\n".as_ptr(), lib_path, path.as_ptr(), pid) };
        err = -ESRCH;
    } else {
        unsafe { qsort(*segs as *mut c_void, *seg_cnt, size_of::<elf_seg>(), cmp_elf_segs) };
        err = 0;
    }
    unsafe { fclose(f) };
    err
}

unsafe fn find_elf_seg(segs: *mut elf_seg, seg_cnt: size_t, virtaddr: c_long) -> *mut elf_seg {
    for i in 0..seg_cnt {
        let seg = unsafe { segs.add(i) };
        if unsafe { (*seg).start <= virtaddr && virtaddr < (*seg).end } {
            return seg;
        }
    }
    ptr::null_mut()
}

unsafe fn find_vma_seg(segs: *mut elf_seg, seg_cnt: size_t, offset: c_long) -> *mut elf_seg {
    for i in 0..seg_cnt {
        let seg = unsafe { segs.add(i) };
        if unsafe { (*seg).offset <= offset && offset < (*seg).offset + ((*seg).end - (*seg).start) } {
            return seg;
        }
    }
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
unsafe fn has_nop_combo(fd: c_int, off: c_long) -> bool {
    let nop_combo: [u8; 11] = [0x90, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut buf = [0u8; 11];
    if unsafe { pread(fd, buf.as_mut_ptr() as *mut c_void, 11, off) } != 11 {
        return false;
    }
    unsafe { memcmp(buf.as_ptr() as *const c_void, nop_combo.as_ptr() as *const c_void, 11) == 0 }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn has_nop_combo(_fd: c_int, _off: c_long) -> bool {
    false
}

unsafe fn collect_usdt_targets(
    man: *mut usdt_manager,
    elf_fd: *mut elf_fd,
    path: *const c_char,
    pid: pid_t,
    usdt_provider: *const c_char,
    usdt_name: *const c_char,
    usdt_cookie: __u64,
    out_targets: *mut *mut usdt_target,
    out_target_cnt: *mut size_t,
) -> c_int {
    let mut off: size_t;
    let mut name_off: size_t = 0;
    let mut desc_off: size_t = 0;
    let mut seg_cnt: size_t = 0;
    let mut vma_seg_cnt: size_t = 0;
    let mut target_cnt: size_t = 0;
    let mut segs: *mut elf_seg = ptr::null_mut();
    let mut vma_segs: *mut elf_seg = ptr::null_mut();
    let mut targets: *mut usdt_target = ptr::null_mut();
    let elf = unsafe { (*elf_fd).elf };
    let mut base_addr: c_long = 0;
    let mut notes_scn: *mut Elf_Scn = ptr::null_mut();
    let mut base_scn: *mut Elf_Scn = ptr::null_mut();
    let mut base_shdr: GElf_Shdr = unsafe { core::mem::zeroed() };
    let mut notes_shdr: GElf_Shdr = unsafe { core::mem::zeroed() };
    let mut ehdr: GElf_Ehdr = unsafe { core::mem::zeroed() };
    let mut nhdr: GElf_Nhdr = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    unsafe {
        *out_targets = ptr::null_mut();
        *out_target_cnt = 0;
    }

    err = unsafe { find_elf_sec_by_name(elf, USDT_NOTE_SEC.as_ptr() as *const c_char, &mut notes_shdr, &mut notes_scn) };
    if err != 0 {
        unsafe { pr_warn(c"usdt: no USDT notes section (%s) found in '%s'\n".as_ptr(), USDT_NOTE_SEC.as_ptr(), path) };
        return err;
    }
    if notes_shdr.sh_type != SHT_NOTE || unsafe { gelf_getehdr(elf, &mut ehdr) }.is_null() {
        unsafe { pr_warn(c"usdt: invalid USDT notes section (%s) in '%s'\n".as_ptr(), USDT_NOTE_SEC.as_ptr(), path) };
        return -EINVAL;
    }
    err = unsafe { parse_elf_segs(elf, path, &mut segs, &mut seg_cnt) };
    if err != 0 {
        unsafe { pr_warn(c"usdt: failed to process ELF program segments for '%s': %s\n".as_ptr(), path, errstr(err)) };
        goto_err_out!(err, segs, vma_segs, targets);
    }
    if unsafe { find_elf_sec_by_name(elf, USDT_BASE_SEC.as_ptr() as *const c_char, &mut base_shdr, &mut base_scn) } == 0 {
        base_addr = base_shdr.sh_addr as c_long;
    }

    let data = unsafe { elf_getdata(notes_scn, ptr::null_mut()) };
    off = 0;
    loop {
        off = unsafe { gelf_getnote(data, off, &mut nhdr, &mut name_off, &mut desc_off) };
        if off == 0 {
            break;
        }
        let mut usdt_abs_ip: c_long;
        let mut usdt_rel_ip: c_long;
        let mut usdt_sema_off: c_long = 0;
        let mut note: usdt_note = unsafe { core::mem::zeroed() };
        let mut seg: *mut elf_seg = ptr::null_mut();
        err = unsafe { parse_usdt_note(&mut nhdr, (*data).d_buf as *const c_char, name_off, desc_off, &mut note) };
        if err != 0 { unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); if err < 0 { free(targets as *mut c_void); } } return err; }
        if unsafe { strcmp(note.provider, usdt_provider) } != 0 || unsafe { strcmp(note.name, usdt_name) } != 0 {
            continue;
        }
        usdt_abs_ip = note.loc_addr;
        if base_addr != 0 && note.base_addr != 0 {
            usdt_abs_ip += base_addr - note.base_addr;
        }
        seg = unsafe { find_elf_seg(segs, seg_cnt, usdt_abs_ip) };
        if seg.is_null() {
            err = -ESRCH;
            unsafe { pr_warn(c"usdt: failed to find ELF program segment for '%s:%s' in '%s' at IP 0x%lx\n".as_ptr(), usdt_provider, usdt_name, path, usdt_abs_ip as c_ulong) };
            unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
            return err;
        }
        if unsafe { !(*seg).is_exec } {
            err = -ESRCH;
            unsafe { pr_warn(c"usdt: matched ELF binary '%s' segment [0x%lx, 0x%lx) for '%s:%s' at IP 0x%lx is not executable\n".as_ptr(), path, (*seg).start as c_ulong, (*seg).end as c_ulong, usdt_provider, usdt_name, usdt_abs_ip as c_ulong) };
            unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
            return err;
        }
        usdt_rel_ip = unsafe { usdt_abs_ip - (*seg).start + (*seg).offset };

        if ehdr.e_type == ET_DYN && unsafe { !(*man).has_bpf_cookie } {
            if pid < 0 {
                unsafe { pr_warn(c"usdt: attaching to shared libraries without specific PID is not supported on current kernel\n".as_ptr()) };
                err = -ENOTSUP;
                unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                return err;
            }
            if vma_seg_cnt == 0 {
                err = unsafe { parse_vma_segs(pid, path, &mut vma_segs, &mut vma_seg_cnt) };
                if err != 0 {
                    unsafe { pr_warn(c"usdt: failed to get memory segments in PID %d for shared library '%s': %s\n".as_ptr(), pid, path, errstr(err)) };
                    unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                    return err;
                }
            }
            seg = unsafe { find_vma_seg(vma_segs, vma_seg_cnt, usdt_rel_ip) };
            if seg.is_null() {
                err = -ESRCH;
                unsafe { pr_warn(c"usdt: failed to find shared lib memory segment for '%s:%s' in '%s' at relative IP 0x%lx\n".as_ptr(), usdt_provider, usdt_name, path, usdt_rel_ip as c_ulong) };
                unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                return err;
            }
            usdt_abs_ip = unsafe { (*seg).start - (*seg).offset + usdt_rel_ip };
        }

        unsafe {
            pr_debug(c"usdt: probe for '%s:%s' in %s '%s': addr 0x%lx base 0x%lx (resolved abs_ip 0x%lx rel_ip 0x%lx) args '%s' in segment [0x%lx, 0x%lx) at offset 0x%lx\n".as_ptr(),
                     usdt_provider, usdt_name, if ehdr.e_type == ET_EXEC { c"exec".as_ptr() } else { c"lib ".as_ptr() }, path,
                     note.loc_addr as c_ulong, note.base_addr as c_ulong, usdt_abs_ip as c_ulong, usdt_rel_ip as c_ulong, note.args,
                     if !seg.is_null() { (*seg).start as c_ulong } else { 0 },
                     if !seg.is_null() { (*seg).end as c_ulong } else { 0 },
                     if !seg.is_null() { (*seg).offset as c_ulong } else { 0 });
        }

        if note.sema_addr != 0 {
            if unsafe { !(*man).has_sema_refcnt } {
                unsafe { pr_warn(c"usdt: kernel doesn't support USDT semaphore refcounting for '%s:%s' in '%s'\n".as_ptr(), usdt_provider, usdt_name, path) };
                err = -ENOTSUP;
                unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                return err;
            }
            seg = unsafe { find_elf_seg(segs, seg_cnt, note.sema_addr) };
            if seg.is_null() {
                err = -ESRCH;
                unsafe { pr_warn(c"usdt: failed to find ELF loadable segment with semaphore of '%s:%s' in '%s' at 0x%lx\n".as_ptr(), usdt_provider, usdt_name, path, note.sema_addr as c_ulong) };
                unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                return err;
            }
            if unsafe { (*seg).is_exec } {
                err = -ESRCH;
                unsafe { pr_warn(c"usdt: matched ELF binary '%s' segment [0x%lx, 0x%lx] for semaphore of '%s:%s' at 0x%lx is executable\n".as_ptr(), path, (*seg).start as c_ulong, (*seg).end as c_ulong, usdt_provider, usdt_name, note.sema_addr as c_ulong) };
                unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
                return err;
            }
            usdt_sema_off = unsafe { note.sema_addr - (*seg).start + (*seg).offset };
            unsafe { pr_debug(c"usdt: sema  for '%s:%s' in %s '%s': addr 0x%lx base 0x%lx (resolved 0x%lx) in segment [0x%lx, 0x%lx] at offset 0x%lx\n".as_ptr(), usdt_provider, usdt_name, if ehdr.e_type == ET_EXEC { c"exec".as_ptr() } else { c"lib ".as_ptr() }, path, note.sema_addr as c_ulong, note.base_addr as c_ulong, usdt_sema_off as c_ulong, (*seg).start as c_ulong, (*seg).end as c_ulong, (*seg).offset as c_ulong) };
        }

        let tmp = unsafe { libbpf_reallocarray(targets as *mut c_void, target_cnt + 1, size_of::<usdt_target>()) as *mut usdt_target };
        if tmp.is_null() {
            err = -ENOMEM;
            unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
            return err;
        }
        targets = tmp;
        let target = unsafe { targets.add(target_cnt) };
        unsafe { memset(target as *mut c_void, 0, size_of::<usdt_target>()) };
        if unsafe { (*man).has_uprobe_syscall && has_nop_combo((*elf_fd).fd, usdt_rel_ip) } {
            usdt_abs_ip += 1;
            usdt_rel_ip += 1;
        }
        unsafe {
            (*target).abs_ip = usdt_abs_ip;
            (*target).rel_ip = usdt_rel_ip;
            (*target).sema_off = usdt_sema_off;
            (*target).spec_str = note.args;
        }
        err = unsafe { parse_usdt_spec(&mut (*target).spec, &note, usdt_cookie) };
        if err != 0 {
            unsafe { free(segs as *mut c_void); free(vma_segs as *mut c_void); free(targets as *mut c_void); }
            return err;
        }
        target_cnt += 1;
    }

    unsafe {
        *out_targets = targets;
        *out_target_cnt = target_cnt;
        free(segs as *mut c_void);
        free(vma_segs as *mut c_void);
    }
    target_cnt as c_int
}

macro_rules! goto_err_out {
    ($err:expr, $segs:expr, $vma_segs:expr, $targets:expr) => {{
        unsafe {
            free($segs as *mut c_void);
            free($vma_segs as *mut c_void);
            if $err < 0 { free($targets as *mut c_void); }
        }
        return $err;
    }};
}
use goto_err_out;

#[repr(C)]
pub struct bpf_link_usdt_uprobe {
    pub abs_ip: c_long,
    pub link: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_link_usdt {
    pub link: bpf_link,
    pub usdt_man: *mut usdt_manager,
    pub spec_cnt: size_t,
    pub spec_ids: *mut c_int,
    pub uprobe_cnt: size_t,
    pub uprobes: *mut bpf_link_usdt_uprobe,
    pub multi_link: *mut bpf_link,
}

unsafe extern "C" fn bpf_link_usdt_detach(link: *mut bpf_link) -> c_int {
    let usdt_link = link as *mut bpf_link_usdt;
    let man = unsafe { (*usdt_link).usdt_man };
    unsafe { bpf_link__destroy((*usdt_link).multi_link) };
    for i in 0..unsafe { (*usdt_link).uprobe_cnt } {
        unsafe {
            bpf_link__destroy((*(*usdt_link).uprobes.add(i)).link);
            if !(*man).has_bpf_cookie {
                bpf_map_delete_elem(bpf_map__fd((*man).ip_to_spec_id_map), &(*(*usdt_link).uprobes.add(i)).abs_ip as *const _ as *const c_void);
            }
        }
    }
    unsafe {
        if (*man).free_spec_ids.is_null() {
            (*man).free_spec_ids = (*usdt_link).spec_ids;
            (*man).free_spec_cnt = (*usdt_link).spec_cnt;
            (*usdt_link).spec_ids = ptr::null_mut();
        } else {
            let new_cnt = (*man).free_spec_cnt + (*usdt_link).spec_cnt;
            let new_free_ids = libbpf_reallocarray((*man).free_spec_ids as *mut c_void, new_cnt, size_of::<c_int>()) as *mut c_int;
            if !new_free_ids.is_null() || new_cnt == 0 {
                memcpy(new_free_ids.add((*man).free_spec_cnt) as *mut c_void,
                       (*usdt_link).spec_ids as *const c_void,
                       (*usdt_link).spec_cnt * size_of::<c_int>());
                (*man).free_spec_ids = new_free_ids;
                (*man).free_spec_cnt = new_cnt;
            }
        }
    }
    0
}

unsafe extern "C" fn bpf_link_usdt_dealloc(link: *mut bpf_link) {
    let usdt_link = link as *mut bpf_link_usdt;
    unsafe {
        free((*usdt_link).spec_ids as *mut c_void);
        free((*usdt_link).uprobes as *mut c_void);
        free(usdt_link as *mut c_void);
    }
}

unsafe extern "C" fn specs_hash_fn(key: c_long, _ctx: *mut c_void) -> size_t {
    unsafe { str_hash(key as *mut c_char) }
}

unsafe extern "C" fn specs_equal_fn(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    unsafe { strcmp(key1 as *const c_char, key2 as *const c_char) == 0 }
}

unsafe fn allocate_spec_id(
    man: *mut usdt_manager,
    specs_hash: *mut hashmap,
    link: *mut bpf_link_usdt,
    target: *mut usdt_target,
    spec_id: *mut c_int,
    is_new: *mut bool,
) -> c_int {
    let mut tmp: c_long = 0;
    if unsafe { hashmap__find(specs_hash, (*target).spec_str, &mut tmp) } {
        unsafe { *spec_id = tmp as c_int; *is_new = false; }
        return 0;
    }
    let new_ids = unsafe { libbpf_reallocarray((*link).spec_ids as *mut c_void, (*link).spec_cnt + 1, size_of::<c_int>()) as *mut c_int };
    if new_ids.is_null() {
        return -ENOMEM;
    }
    unsafe { (*link).spec_ids = new_ids };
    let err: c_int;
    unsafe {
        if (*man).free_spec_cnt != 0 {
            *spec_id = *(*man).free_spec_ids.add((*man).free_spec_cnt - 1);
            err = hashmap__add(specs_hash, (*target).spec_str, *spec_id as c_long);
            if err != 0 { return err; }
            (*man).free_spec_cnt -= 1;
        } else {
            if (*man).next_free_spec_id >= bpf_map__max_entries((*man).specs_map) as usize {
                return -E2BIG;
            }
            *spec_id = (*man).next_free_spec_id as c_int;
            err = hashmap__add(specs_hash, (*target).spec_str, *spec_id as c_long);
            if err != 0 { return err; }
            (*man).next_free_spec_id += 1;
        }
        *(*link).spec_ids.add((*link).spec_cnt) = *spec_id;
        (*link).spec_cnt += 1;
        *is_new = true;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_manager_attach_usdt(
    man: *mut usdt_manager,
    prog: *const bpf_program,
    mut pid: pid_t,
    path: *const c_char,
    usdt_provider: *const c_char,
    usdt_name: *const c_char,
    usdt_cookie: __u64,
) -> *mut bpf_link {
    let mut offsets: *mut c_ulong = ptr::null_mut();
    let mut ref_ctr_offsets: *mut c_ulong = ptr::null_mut();
    let mut cookies: *mut __u64 = ptr::null_mut();
    let mut specs_hash: *mut hashmap = ptr::null_mut();
    let mut link: *mut bpf_link_usdt = ptr::null_mut();
    let mut targets: *mut usdt_target = ptr::null_mut();
    let mut elf_fd: elf_fd = unsafe { core::mem::zeroed() };
    let mut target_cnt: size_t = 0;
    let mut err: c_int;
    let spec_map_fd = unsafe { bpf_map__fd((*man).specs_map) };
    let ip_map_fd = unsafe { bpf_map__fd((*man).ip_to_spec_id_map) };

    err = unsafe { elf_open(path, &mut elf_fd) };
    if err != 0 {
        return unsafe { libbpf_err_ptr(err) };
    }
    err = unsafe { sanity_check_usdt_elf(elf_fd.elf, path) };
    if err != 0 { unsafe { elf_close(&mut elf_fd); } return unsafe { libbpf_err_ptr(err) }; }
    if pid < 0 { pid = -1; } else if pid == 0 { pid = unsafe { getpid() }; }
    err = unsafe { collect_usdt_targets(man, &mut elf_fd, path, pid, usdt_provider, usdt_name, usdt_cookie, &mut targets, &mut target_cnt) };
    if err <= 0 {
        err = if err == 0 { -ENOENT } else { err };
        unsafe { elf_close(&mut elf_fd); }
        return unsafe { libbpf_err_ptr(err) };
    }
    specs_hash = unsafe { hashmap__new(Some(specs_hash_fn), Some(specs_equal_fn), ptr::null_mut()) };
    if unsafe { IS_ERR(specs_hash) } {
        err = unsafe { PTR_ERR(specs_hash) };
        unsafe { free(targets as *mut c_void); elf_close(&mut elf_fd); }
        return unsafe { libbpf_err_ptr(err) };
    }
    link = unsafe { calloc(1, size_of::<bpf_link_usdt>()) as *mut bpf_link_usdt };
    if link.is_null() {
        err = -ENOMEM;
        unsafe { free(targets as *mut c_void); hashmap__free(specs_hash); elf_close(&mut elf_fd); }
        return unsafe { libbpf_err_ptr(err) };
    }
    unsafe {
        (*link).usdt_man = man;
        (*link).link.detach = Some(bpf_link_usdt_detach);
        (*link).link.dealloc = Some(bpf_link_usdt_dealloc);
    }
    if unsafe { (*man).has_uprobe_multi } {
        unsafe {
            offsets = calloc(target_cnt, size_of::<c_ulong>()) as *mut c_ulong;
            cookies = calloc(target_cnt, size_of::<__u64>()) as *mut __u64;
            ref_ctr_offsets = calloc(target_cnt, size_of::<c_ulong>()) as *mut c_ulong;
        }
        if offsets.is_null() || ref_ctr_offsets.is_null() || cookies.is_null() {
            err = -ENOMEM;
            unsafe { free(offsets as *mut c_void); free(ref_ctr_offsets as *mut c_void); free(cookies as *mut c_void); bpf_link__destroy(&mut (*link).link); free(targets as *mut c_void); hashmap__free(specs_hash); elf_close(&mut elf_fd); }
            return unsafe { libbpf_err_ptr(err) };
        }
    } else {
        unsafe { (*link).uprobes = calloc(target_cnt, size_of::<bpf_link_usdt_uprobe>()) as *mut bpf_link_usdt_uprobe; }
        if unsafe { (*link).uprobes.is_null() } {
            err = -ENOMEM;
            unsafe { bpf_link__destroy(&mut (*link).link); free(targets as *mut c_void); hashmap__free(specs_hash); elf_close(&mut elf_fd); }
            return unsafe { libbpf_err_ptr(err) };
        }
    }

    for i in 0..target_cnt {
        let target = unsafe { targets.add(i) };
        let mut is_new = false;
        let mut spec_id: c_int = 0;
        err = unsafe { allocate_spec_id(man, specs_hash, link, target, &mut spec_id, &mut is_new) };
        if err != 0 { break; }
        if is_new && unsafe { bpf_map_update_elem(spec_map_fd, &spec_id as *const _ as *const c_void, &(*target).spec as *const _ as *const c_void, BPF_ANY) } != 0 {
            err = unsafe { -errno };
            unsafe { pr_warn(c"usdt: failed to set USDT spec #%d for '%s:%s' in '%s': %s\n".as_ptr(), spec_id, usdt_provider, usdt_name, path, errstr(err)) };
            break;
        }
        if unsafe { !(*man).has_bpf_cookie } &&
            unsafe { bpf_map_update_elem(ip_map_fd, &(*target).abs_ip as *const _ as *const c_void, &spec_id as *const _ as *const c_void, BPF_NOEXIST) } != 0 {
            err = unsafe { -errno };
            if err == -EEXIST {
                unsafe { pr_warn(c"usdt: IP collision detected for spec #%d for '%s:%s' in '%s'\n".as_ptr(), spec_id, usdt_provider, usdt_name, path) };
            } else {
                unsafe { pr_warn(c"usdt: failed to map IP 0x%lx to spec #%d for '%s:%s' in '%s': %s\n".as_ptr(), (*target).abs_ip as c_ulong, spec_id, usdt_provider, usdt_name, path, errstr(err)) };
            }
            break;
        }
        if unsafe { (*man).has_uprobe_multi } {
            unsafe {
                *offsets.add(i) = (*target).rel_ip as c_ulong;
                *ref_ctr_offsets.add(i) = (*target).sema_off as c_ulong;
                *cookies.add(i) = spec_id as __u64;
            }
        } else {
            let mut opts = bpf_uprobe_opts { sz: size_of::<bpf_uprobe_opts>(), ref_ctr_offset: unsafe { (*target).sema_off as c_ulong }, bpf_cookie: unsafe { if (*man).has_bpf_cookie { spec_id as __u64 } else { 0 } } };
            let uprobe_link = unsafe { bpf_program__attach_uprobe_opts(prog, pid, path, (*target).rel_ip as c_ulong, &mut opts) };
            err = unsafe { libbpf_get_error(uprobe_link as *const c_void) };
            if err != 0 {
                unsafe { pr_warn(c"usdt: failed to attach uprobe #%d for '%s:%s' in '%s': %s\n".as_ptr(), i as c_int, usdt_provider, usdt_name, path, errstr(err)) };
                break;
            }
            unsafe {
                (*(*link).uprobes.add(i)).link = uprobe_link;
                (*(*link).uprobes.add(i)).abs_ip = (*target).abs_ip;
                (*link).uprobe_cnt += 1;
            }
        }
        err = 0;
    }
    if err != 0 {
        unsafe { free(offsets as *mut c_void); free(ref_ctr_offsets as *mut c_void); free(cookies as *mut c_void); bpf_link__destroy(&mut (*link).link); free(targets as *mut c_void); hashmap__free(specs_hash); elf_close(&mut elf_fd); }
        return unsafe { libbpf_err_ptr(err) };
    }
    if unsafe { (*man).has_uprobe_multi } {
        let mut opts_multi = bpf_uprobe_multi_opts { sz: size_of::<bpf_uprobe_multi_opts>(), ref_ctr_offsets, offsets, cookies, cnt: target_cnt };
        unsafe { (*link).multi_link = bpf_program__attach_uprobe_multi(prog, pid, path, ptr::null(), &mut opts_multi) };
        if unsafe { (*link).multi_link.is_null() } {
            err = unsafe { -errno };
            unsafe {
                pr_warn(c"usdt: failed to attach uprobe multi for '%s:%s' in '%s': %s\n".as_ptr(), usdt_provider, usdt_name, path, errstr(err));
                free(offsets as *mut c_void); free(ref_ctr_offsets as *mut c_void); free(cookies as *mut c_void);
                bpf_link__destroy(&mut (*link).link); free(targets as *mut c_void); hashmap__free(specs_hash); elf_close(&mut elf_fd);
            }
            return unsafe { libbpf_err_ptr(err) };
        }
        unsafe { free(offsets as *mut c_void); free(ref_ctr_offsets as *mut c_void); free(cookies as *mut c_void); }
    }
    unsafe {
        free(targets as *mut c_void);
        hashmap__free(specs_hash);
        elf_close(&mut elf_fd);
        &mut (*link).link
    }
}

/* Parse out USDT ELF note from '.note.stapsdt' section.
 * Logic inspired by perf's code.
 */
unsafe fn parse_usdt_note(nhdr: *mut GElf_Nhdr, data: *const c_char, name_off: size_t, desc_off: size_t, note: *mut usdt_note) -> c_int {
    let mut addrs = [0 as c_long; 3];
    if unsafe { strncmp(data.add(name_off), USDT_NOTE_NAME.as_ptr() as *const c_char, (*nhdr).n_namesz as size_t) } != 0 {
        return -EINVAL;
    }
    if unsafe { (*nhdr).n_type } != USDT_NOTE_TYPE {
        return -EINVAL;
    }
    let len = unsafe { (*nhdr).n_descsz as size_t };
    let data = unsafe { data.add(desc_off) };
    if len < size_of::<[c_long; 3]>() + 3 {
        return -EINVAL;
    }
    unsafe { memcpy(addrs.as_mut_ptr() as *mut c_void, data as *const c_void, size_of::<[c_long; 3]>()) };
    let provider = unsafe { data.add(size_of::<[c_long; 3]>()) };
    let mut name = unsafe { memchr(provider as *const c_void, 0, data.add(len).offset_from(provider) as size_t) as *const c_char };
    if name.is_null() { return -EINVAL; }
    name = unsafe { name.add(1) };
    if name >= unsafe { data.add(len) } || unsafe { *name } == 0 { return -EINVAL; }
    let mut args = unsafe { memchr(name as *const c_void, 0, data.add(len).offset_from(name) as size_t) as *const c_char };
    if args.is_null() { return -EINVAL; }
    args = unsafe { args.add(1) };
    if args >= unsafe { data.add(len) } { return -EINVAL; }
    unsafe {
        (*note).provider = provider;
        (*note).name = name;
        (*note).args = if *args == 0 || *args == b':' as c_char { c"".as_ptr() } else { args };
        (*note).loc_addr = addrs[0];
        (*note).base_addr = addrs[1];
        (*note).sema_addr = addrs[2];
    }
    0
}

unsafe fn parse_usdt_spec(spec: *mut usdt_spec, note: *const usdt_note, usdt_cookie: __u64) -> c_int {
    unsafe {
        (*spec).usdt_cookie = usdt_cookie;
        (*spec).arg_cnt = 0;
    }
    let mut s = unsafe { (*note).args };
    while unsafe { *s } != 0 {
        if unsafe { (*spec).arg_cnt as usize >= USDT_MAX_ARG_CNT } {
            unsafe { pr_warn(c"usdt: too many USDT arguments (> %d) for '%s:%s' with args spec '%s'\n".as_ptr(), USDT_MAX_ARG_CNT as c_int, (*note).provider, (*note).name, (*note).args) };
            return -E2BIG;
        }
        let idx = unsafe { (*spec).arg_cnt as usize };
        let arg = unsafe { &mut (*spec).args[idx] as *mut usdt_arg_spec };
        let mut arg_sz: c_int = 0;
        let len = unsafe { parse_usdt_arg(s, (*spec).arg_cnt as c_int, arg, &mut arg_sz) };
        if len < 0 { return len; }
        unsafe { (*arg).arg_signed = arg_sz < 0 };
        if arg_sz < 0 { arg_sz = -arg_sz; }
        match arg_sz {
            1 | 2 | 4 | 8 => unsafe { (*arg).arg_bitshift = (64 - arg_sz * 8) as c_char },
            _ => {
                unsafe { pr_warn(c"usdt: unsupported arg #%d (spec '%s') size: %d\n".as_ptr(), (*spec).arg_cnt as c_int, s, arg_sz) };
                return -EINVAL;
            }
        }
        s = unsafe { s.add(len as usize) };
        unsafe { (*spec).arg_cnt += 1 };
    }
    0
}

/* Architecture-specific logic for parsing USDT argument location specs */

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn calc_pt_regs_off(reg_name: *const c_char) -> c_int {
    #[repr(C)]
    struct RegMap { names: [*const c_char; 4], pt_regs_off: size_t }
    let reg_map = [
        RegMap { names: [c"rip".as_ptr(), c"eip".as_ptr(), c"".as_ptr(), c"".as_ptr()], pt_regs_off: offset_of!(pt_regs, rip) },
        RegMap { names: [c"rax".as_ptr(), c"eax".as_ptr(), c"ax".as_ptr(), c"al".as_ptr()], pt_regs_off: offset_of!(pt_regs, rax) },
        RegMap { names: [c"rbx".as_ptr(), c"ebx".as_ptr(), c"bx".as_ptr(), c"bl".as_ptr()], pt_regs_off: offset_of!(pt_regs, rbx) },
        RegMap { names: [c"rcx".as_ptr(), c"ecx".as_ptr(), c"cx".as_ptr(), c"cl".as_ptr()], pt_regs_off: offset_of!(pt_regs, rcx) },
        RegMap { names: [c"rdx".as_ptr(), c"edx".as_ptr(), c"dx".as_ptr(), c"dl".as_ptr()], pt_regs_off: offset_of!(pt_regs, rdx) },
        RegMap { names: [c"rsi".as_ptr(), c"esi".as_ptr(), c"si".as_ptr(), c"sil".as_ptr()], pt_regs_off: offset_of!(pt_regs, rsi) },
        RegMap { names: [c"rdi".as_ptr(), c"edi".as_ptr(), c"di".as_ptr(), c"dil".as_ptr()], pt_regs_off: offset_of!(pt_regs, rdi) },
        RegMap { names: [c"rbp".as_ptr(), c"ebp".as_ptr(), c"bp".as_ptr(), c"bpl".as_ptr()], pt_regs_off: offset_of!(pt_regs, rbp) },
        RegMap { names: [c"rsp".as_ptr(), c"esp".as_ptr(), c"sp".as_ptr(), c"spl".as_ptr()], pt_regs_off: offset_of!(pt_regs, rsp) },
        RegMap { names: [c"r8".as_ptr(), c"r8d".as_ptr(), c"r8w".as_ptr(), c"r8b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r8) },
        RegMap { names: [c"r9".as_ptr(), c"r9d".as_ptr(), c"r9w".as_ptr(), c"r9b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r9) },
        RegMap { names: [c"r10".as_ptr(), c"r10d".as_ptr(), c"r10w".as_ptr(), c"r10b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r10) },
        RegMap { names: [c"r11".as_ptr(), c"r11d".as_ptr(), c"r11w".as_ptr(), c"r11b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r11) },
        RegMap { names: [c"r12".as_ptr(), c"r12d".as_ptr(), c"r12w".as_ptr(), c"r12b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r12) },
        RegMap { names: [c"r13".as_ptr(), c"r13d".as_ptr(), c"r13w".as_ptr(), c"r13b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r13) },
        RegMap { names: [c"r14".as_ptr(), c"r14d".as_ptr(), c"r14w".as_ptr(), c"r14b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r14) },
        RegMap { names: [c"r15".as_ptr(), c"r15d".as_ptr(), c"r15w".as_ptr(), c"r15b".as_ptr()], pt_regs_off: offset_of!(pt_regs, r15) },
    ];
    for item in reg_map.iter() {
        for name in item.names.iter() {
            if unsafe { strcmp(reg_name, *name) } == 0 {
                return item.pt_regs_off as c_int;
            }
        }
    }
    unsafe { pr_warn(c"usdt: unrecognized register '%s'\n".as_ptr(), reg_name) };
    -ENOENT
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn parse_usdt_arg(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int) -> c_int {
    let mut reg_name = [0 as c_char; 16];
    let mut idx_reg_name = [0 as c_char; 16];
    let mut len: c_int = 0;
    let mut scale: c_int = 1;
    let mut off: c_long = 0;
    if unsafe { sscanf(arg_str, c" %d @ %ld ( %%%15[^,] , %%%15[^,] , %d ) %n".as_ptr(), arg_sz, &mut off, reg_name.as_mut_ptr(), idx_reg_name.as_mut_ptr(), &mut scale, &mut len) } == 5 ||
       unsafe { sscanf(arg_str, c" %d @ ( %%%15[^,] , %%%15[^,] , %d ) %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), idx_reg_name.as_mut_ptr(), &mut scale, &mut len) } == 4 ||
       unsafe { sscanf(arg_str, c" %d @ %ld ( %%%15[^,] , %%%15[^)] ) %n".as_ptr(), arg_sz, &mut off, reg_name.as_mut_ptr(), idx_reg_name.as_mut_ptr(), &mut len) } == 4 ||
       unsafe { sscanf(arg_str, c" %d @ ( %%%15[^,] , %%%15[^)] ) %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), idx_reg_name.as_mut_ptr(), &mut len) } == 3 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_SIB; (*arg).val_off = off as __u64; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
        let idx_reg_off = unsafe { calc_pt_regs_off(idx_reg_name.as_ptr()) }; if idx_reg_off < 0 { return idx_reg_off; }
        unsafe { (*arg).idx_reg_off = idx_reg_off as __u16; }
        unsafe { (*arg).scale_bitshift = match scale { 1 => 0, 2 => 1, 4 => 2, 8 => 3, _ => { pr_warn(c"usdt: invalid SIB scale %d, expected 1, 2, 4, 8\n".as_ptr(), scale); return -EINVAL; } }; }
    } else if unsafe { sscanf(arg_str, c" %d @ %ld ( %%%15[^)] ) %n".as_ptr(), arg_sz, &mut off, reg_name.as_mut_ptr(), &mut len) } == 3 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG_DEREF; (*arg).val_off = off as __u64; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ ( %%%15[^)] ) %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG_DEREF; (*arg).val_off = 0; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ %%%15s %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG; (*arg).val_off = 0; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ $%ld %n".as_ptr(), arg_sz, &mut off, &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_CONST; (*arg).val_off = off as __u64; (*arg).reg_off = 0; }
    } else {
        unsafe { pr_warn(c"usdt: unrecognized arg #%d spec '%s'\n".as_ptr(), arg_num, arg_str) };
        return -EINVAL;
    }
    len
}

#[cfg(target_arch = "s390x")]
unsafe fn parse_usdt_arg(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int) -> c_int {
    let mut reg: c_uint = 0; let mut len: c_int = 0; let mut off: c_long = 0;
    if unsafe { sscanf(arg_str, c" %d @ %ld ( %%r%u ) %n".as_ptr(), arg_sz, &mut off, &mut reg, &mut len) } == 3 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG_DEREF; (*arg).val_off = off as __u64; }
        if reg > 15 { unsafe { pr_warn(c"usdt: unrecognized register '%%r%u'\n".as_ptr(), reg) }; return -EINVAL; }
        unsafe { (*arg).reg_off = (offset_of!(user_pt_regs, gprs) + reg as usize * size_of::<c_ulong>()) as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ %%r%u %n".as_ptr(), arg_sz, &mut reg, &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG; (*arg).val_off = 0; }
        if reg > 15 { unsafe { pr_warn(c"usdt: unrecognized register '%%r%u'\n".as_ptr(), reg) }; return -EINVAL; }
        unsafe { (*arg).reg_off = (offset_of!(user_pt_regs, gprs) + reg as usize * size_of::<c_ulong>()) as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ %ld %n".as_ptr(), arg_sz, &mut off, &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_CONST; (*arg).val_off = off as __u64; (*arg).reg_off = 0; }
    } else { unsafe { pr_warn(c"usdt: unrecognized arg #%d spec '%s'\n".as_ptr(), arg_num, arg_str) }; return -EINVAL; }
    len
}

#[cfg(target_arch = "aarch64")]
unsafe fn calc_pt_regs_off(reg_name: *const c_char) -> c_int {
    let mut reg_num: c_int = 0;
    if unsafe { sscanf(reg_name, c"x%d".as_ptr(), &mut reg_num) } == 1 {
        if reg_num >= 0 && reg_num < 31 { return (offset_of!(user_pt_regs, regs) + reg_num as usize * size_of::<c_ulong>()) as c_int; }
    } else if unsafe { strcmp(reg_name, c"sp".as_ptr()) } == 0 { return offset_of!(user_pt_regs, sp) as c_int; }
    unsafe { pr_warn(c"usdt: unrecognized register '%s'\n".as_ptr(), reg_name) };
    -ENOENT
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "arm"))]
unsafe fn parse_usdt_arg_common(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int, arm_hash: bool) -> c_int {
    let mut reg_name = [0 as c_char; 16]; let mut len: c_int = 0; let mut off: c_long = 0;
    if arm_hash && unsafe { sscanf(arg_str, c" %d @ [ %15[a-z0-9] , #%ld ] %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut off, &mut len) } == 3 ||
       !arm_hash && unsafe { sscanf(arg_str, c" %d @ [ %15[a-z0-9] , %ld ] %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut off, &mut len) } == 3 ||
       unsafe { sscanf(arg_str, c" %d @ %ld ( %15[a-z0-9] ) %n".as_ptr(), arg_sz, &mut off, reg_name.as_mut_ptr(), &mut len) } == 3 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG_DEREF; (*arg).val_off = off as __u64; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else if unsafe { sscanf(arg_str, c" %d @ [ %15[a-z0-9] ] %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG_DEREF; (*arg).val_off = 0; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else if arm_hash && unsafe { sscanf(arg_str, c" %d @ #%ld %n".as_ptr(), arg_sz, &mut off, &mut len) } == 2 ||
              !arm_hash && unsafe { sscanf(arg_str, c" %d @ %ld %n".as_ptr(), arg_sz, &mut off, &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_CONST; (*arg).val_off = off as __u64; (*arg).reg_off = 0; }
    } else if unsafe { sscanf(arg_str, c" %d @ %15[a-z0-9] %n".as_ptr(), arg_sz, reg_name.as_mut_ptr(), &mut len) } == 2 {
        unsafe { (*arg).arg_type = usdt_arg_type::USDT_ARG_REG; (*arg).val_off = 0; }
        let reg_off = unsafe { calc_pt_regs_off(reg_name.as_ptr()) }; if reg_off < 0 { return reg_off; }
        unsafe { (*arg).reg_off = reg_off as i16; }
    } else { unsafe { pr_warn(c"usdt: unrecognized arg #%d spec '%s'\n".as_ptr(), arg_num, arg_str) }; return -EINVAL; }
    len
}

#[cfg(target_arch = "aarch64")]
unsafe fn parse_usdt_arg(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int) -> c_int {
    unsafe { parse_usdt_arg_common(arg_str, arg_num, arg, arg_sz, false) }
}

#[cfg(target_arch = "riscv64")]
unsafe fn calc_pt_regs_off(reg_name: *const c_char) -> c_int {
    let reg_map = [
        (c"ra".as_ptr(), offset_of!(user_regs_struct, ra)), (c"sp".as_ptr(), offset_of!(user_regs_struct, sp)),
        (c"gp".as_ptr(), offset_of!(user_regs_struct, gp)), (c"tp".as_ptr(), offset_of!(user_regs_struct, tp)),
        (c"a0".as_ptr(), offset_of!(user_regs_struct, a0)), (c"a1".as_ptr(), offset_of!(user_regs_struct, a1)),
        (c"a2".as_ptr(), offset_of!(user_regs_struct, a2)), (c"a3".as_ptr(), offset_of!(user_regs_struct, a3)),
        (c"a4".as_ptr(), offset_of!(user_regs_struct, a4)), (c"a5".as_ptr(), offset_of!(user_regs_struct, a5)),
        (c"a6".as_ptr(), offset_of!(user_regs_struct, a6)), (c"a7".as_ptr(), offset_of!(user_regs_struct, a7)),
        (c"s0".as_ptr(), offset_of!(user_regs_struct, s0)), (c"s1".as_ptr(), offset_of!(user_regs_struct, s1)),
        (c"s2".as_ptr(), offset_of!(user_regs_struct, s2)), (c"s3".as_ptr(), offset_of!(user_regs_struct, s3)),
        (c"s4".as_ptr(), offset_of!(user_regs_struct, s4)), (c"s5".as_ptr(), offset_of!(user_regs_struct, s5)),
        (c"s6".as_ptr(), offset_of!(user_regs_struct, s6)), (c"s7".as_ptr(), offset_of!(user_regs_struct, s7)),
        (c"s8".as_ptr(), offset_of!(user_regs_struct, rv_s8)), (c"s9".as_ptr(), offset_of!(user_regs_struct, s9)),
        (c"s10".as_ptr(), offset_of!(user_regs_struct, s10)), (c"s11".as_ptr(), offset_of!(user_regs_struct, s11)),
        (c"t0".as_ptr(), offset_of!(user_regs_struct, t0)), (c"t1".as_ptr(), offset_of!(user_regs_struct, t1)),
        (c"t2".as_ptr(), offset_of!(user_regs_struct, t2)), (c"t3".as_ptr(), offset_of!(user_regs_struct, t3)),
        (c"t4".as_ptr(), offset_of!(user_regs_struct, t4)), (c"t5".as_ptr(), offset_of!(user_regs_struct, t5)),
        (c"t6".as_ptr(), offset_of!(user_regs_struct, t6)),
    ];
    for (name, off) in reg_map { if unsafe { strcmp(reg_name, name) } == 0 { return off as c_int; } }
    unsafe { pr_warn(c"usdt: unrecognized register '%s'\n".as_ptr(), reg_name) };
    -ENOENT
}

#[cfg(target_arch = "riscv64")]
unsafe fn parse_usdt_arg(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int) -> c_int {
    unsafe { parse_usdt_arg_common(arg_str, arg_num, arg, arg_sz, false) }
}

#[cfg(target_arch = "arm")]
unsafe fn calc_pt_regs_off(reg_name: *const c_char) -> c_int {
    let reg_map = [
        (c"r0".as_ptr(), 0usize), (c"r1".as_ptr(), 1), (c"r2".as_ptr(), 2), (c"r3".as_ptr(), 3),
        (c"r4".as_ptr(), 4), (c"r5".as_ptr(), 5), (c"r6".as_ptr(), 6), (c"r7".as_ptr(), 7),
        (c"r8".as_ptr(), 8), (c"r9".as_ptr(), 9), (c"r10".as_ptr(), 10), (c"fp".as_ptr(), 11),
        (c"ip".as_ptr(), 12), (c"sp".as_ptr(), 13), (c"lr".as_ptr(), 14), (c"pc".as_ptr(), 15),
    ];
    for (name, idx) in reg_map {
        if unsafe { strcmp(reg_name, name) } == 0 {
            return (offset_of!(pt_regs, uregs) + idx * size_of::<c_ulong>()) as c_int;
        }
    }
    unsafe { pr_warn(c"usdt: unrecognized register '%s'\n".as_ptr(), reg_name) };
    -ENOENT
}

#[cfg(target_arch = "arm")]
unsafe fn parse_usdt_arg(arg_str: *const c_char, arg_num: c_int, arg: *mut usdt_arg_spec, arg_sz: *mut c_int) -> c_int {
    unsafe { parse_usdt_arg_common(arg_str, arg_num, arg, arg_sz, true) }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "s390x", target_arch = "aarch64", target_arch = "riscv64", target_arch = "arm")))]
unsafe fn parse_usdt_arg(_arg_str: *const c_char, _arg_num: c_int, _arg: *mut usdt_arg_spec, _arg_sz: *mut c_int) -> c_int {
    unsafe { pr_warn(c"usdt: libbpf doesn't support USDTs on current architecture\n".as_ptr()) };
    -ENOTSUP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
