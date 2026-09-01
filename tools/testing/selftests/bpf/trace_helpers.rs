// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/trace_helpers.c.
// C includes are represented by extern declarations and libc-compatible types.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_ulonglong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type uintptr_t = usize;
type Elf32_Word = u32;
type __u32 = u32;
type __u64 = u64;

const TRACEFS_PIPE: &[u8] = b"/sys/kernel/tracing/trace_pipe\0";
const DEBUGFS_PIPE: &[u8] = b"/sys/kernel/debug/tracing/trace_pipe\0";
const KALLSYMS: &[u8] = b"/proc/kallsyms\0";
const SELF_MAPS: &[u8] = b"/proc/self/maps\0";
const TRACE_FILE: &[u8] = b"/sys/kernel/tracing/trace\0";
const AVAILABLE_FILTER_FUNCTIONS: &[u8] =
    b"/sys/kernel/tracing/available_filter_functions\0";
const DEBUG_AVAILABLE_FILTER_FUNCTIONS: &[u8] =
    b"/sys/kernel/debug/tracing/available_filter_functions\0";
const AVAILABLE_FILTER_FUNCTIONS_ADDRS: &[u8] =
    b"/sys/kernel/tracing/available_filter_functions_addrs\0";
const DEBUG_AVAILABLE_FILTER_FUNCTIONS_ADDRS: &[u8] =
    b"/sys/kernel/debug/tracing/available_filter_functions_addrs\0";

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EAGAIN: c_int = 11;
const EOPNOTSUPP: c_int = 95;
const ESRCH: c_int = 3;
const ENOTTY: c_int = 25;
const EEXIST: c_int = 17;
const F_OK: c_int = 0;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o4000;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const BPF_BUILD_ID_SIZE: usize = 20;
const PATH_MAX: usize = 4096;
const PROCMAP_QUERY_VMA_EXECUTABLE: __u32 = 0x04;
const PT_NOTE: u32 = 4;
const EV_CURRENT: c_int = 1;
const ELF_C_READ_MMAP: c_int = 6;
const ELF_K_ELF: c_int = 3;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_mutex_t {
    __data: [u8; 40],
}

#[repr(C)]
pub struct ksym {
    pub addr: c_ulong,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct ksyms {
    pub syms: *mut ksym,
    pub sym_cnt: c_int,
    pub sym_cap: size_t,
    pub filtered_syms: *mut *mut c_char,
    pub filtered_cnt: size_t,
}

type ksym_cmp_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>;
type ksym_search_cmp_t = Option<unsafe extern "C" fn(*const c_void, *const ksym) -> c_int>;

#[repr(C)]
pub struct Elf32_Nhdr {
    pub n_namesz: Elf32_Word,
    pub n_descsz: Elf32_Word,
    pub n_type: Elf32_Word,
}

#[repr(C)]
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
pub struct GElf_Phdr {
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
pub struct procmap_query {
    pub size: __u64,
    pub query_flags: __u64,
    pub query_addr: __u64,
    pub vma_start: __u64,
    pub vma_end: __u64,
    pub vma_flags: __u64,
    pub vma_offset: __u64,
    pub inode: __u64,
    pub dev_major: __u32,
    pub dev_minor: __u32,
    pub vma_name_addr: __u64,
    pub vma_name_size: __u32,
    pub build_id_addr: __u64,
    pub build_id_size: __u32,
}

extern "C" {
    static mut errno: c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: ksym_cmp_t);
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;

    fn elf_version(ver: c_int) -> c_int;
    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut Elf) -> *mut Elf;
    fn elf_kind(elf: *mut Elf) -> c_int;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_rawfile(elf: *mut Elf, ptr: *mut size_t) -> *mut c_char;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn gelf_getphdr(elf: *mut Elf, ndx: size_t, dst: *mut GElf_Phdr) -> *mut GElf_Phdr;

    fn libbpf_ensure_mem(
        data: *mut *mut c_void,
        cap_cnt: *mut size_t,
        elem_sz: size_t,
        need_cnt: size_t,
    ) -> c_int;
    fn str_hash(s: *const c_char) -> size_t;
    fn hashmap__new(
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__add(map: *mut hashmap, key: *const c_char, value: c_long) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
}

static mut ksyms: *mut ksyms = ptr::null_mut();
static mut ksyms_mutex: pthread_mutex_t = pthread_mutex_t { __data: [0; 40] };

unsafe fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

unsafe fn is_err(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe extern "C" fn ksyms__add_symbol(ksyms: *mut ksyms, name: *const c_char, addr: c_ulong) -> c_int {
    let tmp = strdup(name);
    if tmp.is_null() {
        return -ENOMEM;
    }
    (*(*ksyms).syms.add((*ksyms).sym_cnt as usize)).addr = addr;
    (*(*ksyms).syms.add((*ksyms).sym_cnt as usize)).name = tmp;
    (*ksyms).sym_cnt += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn free_kallsyms_local(ksyms: *mut ksyms) {
    if ksyms.is_null() {
        return;
    }
    free((*ksyms).filtered_syms as *mut c_void);
    if (*ksyms).syms.is_null() {
        free(ksyms as *mut c_void);
        return;
    }
    let mut i: c_int = 0;
    while i < (*ksyms).sym_cnt {
        free((*(*ksyms).syms.add(i as usize)).name as *mut c_void);
        i += 1;
    }
    free((*ksyms).syms as *mut c_void);
    free(ksyms as *mut c_void);
}

unsafe extern "C" fn load_kallsyms_local_common(cmp_cb: ksym_cmp_t) -> *mut ksyms {
    let f = fopen(KALLSYMS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return ptr::null_mut();
    }
    let ksyms = calloc(1, mem::size_of::<ksyms>()) as *mut ksyms;
    if ksyms.is_null() {
        fclose(f);
        return ptr::null_mut();
    }
    let mut func = [0 as c_char; 256];
    let mut buf = [0 as c_char; 256];
    let mut symbol: c_char = 0;
    let mut addr: *mut c_void = ptr::null_mut();
    while !fgets(buf.as_mut_ptr(), buf.len() as c_int, f).is_null() {
        if sscanf(
            buf.as_ptr(),
            b"%p %c %s\0".as_ptr() as *const c_char,
            &mut addr,
            &mut symbol,
            func.as_mut_ptr(),
        ) != 3
        {
            break;
        }
        if addr.is_null() {
            continue;
        }
        let mut syms_ptr = (*ksyms).syms as *mut c_void;
        let ret = libbpf_ensure_mem(
            &mut syms_ptr,
            &mut (*ksyms).sym_cap,
            mem::size_of::<ksym>(),
            (*ksyms).sym_cnt as usize + 1,
        );
        (*ksyms).syms = syms_ptr as *mut ksym;
        if ret != 0 {
            fclose(f);
            free_kallsyms_local(ksyms);
            return ptr::null_mut();
        }
        let ret = ksyms__add_symbol(ksyms, func.as_ptr(), addr as c_ulong);
        if ret != 0 {
            fclose(f);
            free_kallsyms_local(ksyms);
            return ptr::null_mut();
        }
    }
    fclose(f);
    qsort((*ksyms).syms as *mut c_void, (*ksyms).sym_cnt as usize, mem::size_of::<ksym>(), cmp_cb);
    ksyms
}

unsafe extern "C" fn ksym_cmp(p1: *const c_void, p2: *const c_void) -> c_int {
    ((*p1.cast::<ksym>()).addr).wrapping_sub((*p2.cast::<ksym>()).addr) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn load_kallsyms_local() -> *mut ksyms {
    load_kallsyms_local_common(Some(ksym_cmp))
}

#[no_mangle]
pub unsafe extern "C" fn load_kallsyms_custom_local(cmp_cb: ksym_cmp_t) -> *mut ksyms {
    load_kallsyms_local_common(cmp_cb)
}

#[no_mangle]
pub unsafe extern "C" fn load_kallsyms() -> c_int {
    pthread_mutex_lock(&mut ksyms_mutex);
    if ksyms.is_null() {
        ksyms = load_kallsyms_local();
    }
    pthread_mutex_unlock(&mut ksyms_mutex);
    if !ksyms.is_null() { 0 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn ksym_search_local(ksyms: *mut ksyms, key: c_long) -> *mut ksym {
    let mut start: c_int = 0;
    let mut end: c_int = (*ksyms).sym_cnt;
    if (*ksyms).sym_cnt <= 0 {
        return ptr::null_mut();
    }
    while start < end {
        let mid = start + (end - start) / 2;
        let result = key.wrapping_sub((*(*ksyms).syms.add(mid as usize)).addr as c_long);
        if result < 0 {
            end = mid;
        } else if result > 0 {
            start = mid + 1;
        } else {
            return (*ksyms).syms.add(mid as usize);
        }
    }
    if start >= 1
        && ((*(*ksyms).syms.add((start - 1) as usize)).addr as c_long) < key
        && key < ((*(*ksyms).syms.add(start as usize)).addr as c_long)
    {
        return (*ksyms).syms.add((start - 1) as usize);
    }
    (*ksyms).syms
}

#[no_mangle]
pub unsafe extern "C" fn search_kallsyms_custom_local(
    ksyms: *mut ksyms,
    p: *const c_void,
    cmp_cb: ksym_search_cmp_t,
) -> *mut ksym {
    let mut start: c_int = 0;
    let mut end: c_int = (*ksyms).sym_cnt;
    while start < end {
        let mid = start + (end - start) / 2;
        let ks = (*ksyms).syms.add(mid as usize);
        let result = cmp_cb.unwrap()(p, ks);
        if result < 0 {
            end = mid;
        } else if result > 0 {
            start = mid + 1;
        } else {
            return ks;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ksym_search(key: c_long) -> *mut ksym {
    if ksyms.is_null() {
        return ptr::null_mut();
    }
    ksym_search_local(ksyms, key)
}

#[no_mangle]
pub unsafe extern "C" fn ksym_get_addr_local(ksyms: *mut ksyms, name: *const c_char) -> c_long {
    let mut i: c_int = 0;
    while i < (*ksyms).sym_cnt {
        if strcmp((*(*ksyms).syms.add(i as usize)).name, name) == 0 {
            return (*(*ksyms).syms.add(i as usize)).addr as c_long;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksym_get_addr(name: *const c_char) -> c_long {
    if ksyms.is_null() {
        return 0;
    }
    ksym_get_addr_local(ksyms, name)
}

#[no_mangle]
pub unsafe extern "C" fn kallsyms_find(sym: *const c_char, addr: *mut c_ulonglong) -> c_int {
    let mut name = [0 as c_char; 500];
    let mut type_: c_char = 0;
    let mut value: c_ulonglong = 0;
    let mut err = 0;
    let f = fopen(KALLSYMS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -EINVAL;
    }
    while fscanf(
        f,
        b"%llx %c %499s%*[^\n]\n\0".as_ptr() as *const c_char,
        &mut value,
        &mut type_,
        name.as_mut_ptr(),
    ) > 0
    {
        if type_ == b'd' as c_char {
            let mat = strstr(name.as_ptr(), b".llvm.\0".as_ptr() as *const c_char);
            if !mat.is_null() {
                *mat = 0;
            }
        }
        if strcmp(name.as_ptr(), sym) == 0 {
            *addr = value;
            fclose(f);
            return err;
        }
    }
    err = -ENOENT;
    fclose(f);
    err
}

// If PROCMAP_QUERY is available in the target libc/kernel headers, the C source
// uses ioctl(PROCMAP_QUERY). Otherwise this fallback is compiled.
unsafe extern "C" fn procmap_query(
    _fd: c_int,
    _addr: *const c_void,
    _query_flags: __u32,
    _start: *mut size_t,
    _offset: *mut size_t,
    _flags: *mut c_int,
) -> c_int {
    -EOPNOTSUPP
}

#[no_mangle]
pub unsafe extern "C" fn get_uprobe_offset(addr: *const c_void) -> ssize_t {
    let mut start: size_t = 0;
    let mut base: size_t = 0;
    let mut end: size_t = 0;
    let mut buf = [0 as c_char; 256];
    let mut flags: c_int = 0;
    let f = fopen(SELF_MAPS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -errno as ssize_t;
    }
    let err = procmap_query(fileno(f), addr, PROCMAP_QUERY_VMA_EXECUTABLE, &mut start, &mut base, &mut flags);
    if err == -EOPNOTSUPP {
        let mut found = false;
        while fscanf(
            f,
            b"%zx-%zx %s %zx %*[^\n]\n\0".as_ptr() as *const c_char,
            &mut start,
            &mut end,
            buf.as_mut_ptr(),
            &mut base,
        ) == 4
        {
            if buf[2] == b'x' as c_char && (addr as uintptr_t) >= start && (addr as uintptr_t) < end {
                found = true;
                break;
            }
        }
        if !found {
            fclose(f);
            return -ESRCH as ssize_t;
        }
    } else if err != 0 {
        fclose(f);
        return err as ssize_t;
    }
    fclose(f);
    // PPC64 ABIv2 global-entry adjustment from the C source is target-conditional.
    (addr as uintptr_t).wrapping_sub(start).wrapping_add(base) as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn get_rel_offset(addr: uintptr_t) -> ssize_t {
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut offset: size_t = 0;
    let mut buf = [0 as c_char; 256];
    let mut flags: c_int = 0;
    let f = fopen(SELF_MAPS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -errno as ssize_t;
    }
    let err = procmap_query(fileno(f), addr as *const c_void, 0, &mut start, &mut offset, &mut flags);
    if err == 0 {
        fclose(f);
        return (addr as size_t).wrapping_sub(start).wrapping_add(offset) as ssize_t;
    } else if err != -EOPNOTSUPP {
        fclose(f);
        return err as ssize_t;
    } else if err != 0 {
        while fscanf(
            f,
            b"%zx-%zx %s %zx %*[^\n]\n\0".as_ptr() as *const c_char,
            &mut start,
            &mut end,
            buf.as_mut_ptr(),
            &mut offset,
        ) == 4
        {
            if addr >= start && addr < end {
                fclose(f);
                return (addr as size_t).wrapping_sub(start).wrapping_add(offset) as ssize_t;
            }
        }
    }
    fclose(f);
    -EINVAL as ssize_t
}

unsafe extern "C" fn parse_build_id_buf(
    note_start: *const c_void,
    note_size: Elf32_Word,
    build_id: *mut c_char,
) -> c_int {
    let mut note_offs: Elf32_Word = 0;
    while (note_offs as usize) + mem::size_of::<Elf32_Nhdr>() < note_size as usize {
        let nhdr = (note_start as *const u8).add(note_offs as usize) as *mut Elf32_Nhdr;
        if (*nhdr).n_type == 3
            && (*nhdr).n_namesz as usize == mem::size_of_val(b"GNU\0")
            && strcmp(nhdr.add(1) as *const c_char, b"GNU\0".as_ptr() as *const c_char) == 0
            && (*nhdr).n_descsz > 0
            && (*nhdr).n_descsz as usize <= BPF_BUILD_ID_SIZE
        {
            memcpy(
                build_id as *mut c_void,
                (note_start as *const u8)
                    .add(note_offs as usize + align(mem::size_of_val(b"GNU\0"), 4) + mem::size_of::<Elf32_Nhdr>())
                    as *const c_void,
                (*nhdr).n_descsz as usize,
            );
            memset(
                build_id.add((*nhdr).n_descsz as usize) as *mut c_void,
                0,
                BPF_BUILD_ID_SIZE - (*nhdr).n_descsz as usize,
            );
            return (*nhdr).n_descsz as c_int;
        }
        note_offs = note_offs
            .wrapping_add(mem::size_of::<Elf32_Nhdr>() as Elf32_Word)
            .wrapping_add(align((*nhdr).n_namesz as usize, 4) as Elf32_Word)
            .wrapping_add(align((*nhdr).n_descsz as usize, 4) as Elf32_Word);
    }
    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn read_build_id(path: *const c_char, build_id: *mut c_char, size: size_t) -> c_int {
    let mut err = -EINVAL;
    let mut elf: *mut Elf = ptr::null_mut();
    let mut ehdr: GElf_Ehdr = mem::zeroed();
    let mut max: size_t = 0;
    if size < BPF_BUILD_ID_SIZE {
        return -EINVAL;
    }
    let fd = open(path, O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        return -errno;
    }
    elf_version(EV_CURRENT);
    elf = elf_begin(fd, ELF_C_READ_MMAP, ptr::null_mut());
    if !elf.is_null() && elf_kind(elf) == ELF_K_ELF && !gelf_getehdr(elf, &mut ehdr).is_null() {
        let mut i: size_t = 0;
        while i < ehdr.e_phnum as size_t {
            let mut mem_phdr: GElf_Phdr = mem::zeroed();
            let phdr = gelf_getphdr(elf, i, &mut mem_phdr);
            if phdr.is_null() {
                break;
            }
            if (*phdr).p_type == PT_NOTE {
                let data = elf_rawfile(elf, &mut max);
                if data.is_null() {
                    break;
                }
                if ((*phdr).p_offset + (*phdr).p_memsz) as size_t > max {
                    break;
                }
                err = parse_build_id_buf(data.add((*phdr).p_offset as usize) as *const c_void, (*phdr).p_memsz as Elf32_Word, build_id);
                if err > 0 {
                    break;
                }
            }
            i += 1;
        }
    }
    if !elf.is_null() {
        elf_end(elf);
    }
    close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn read_trace_pipe_iter(
    cb: Option<unsafe extern "C" fn(*const c_char, *mut c_void)>,
    data: *mut c_void,
    mut iter: c_int,
) -> c_int {
    let mut buflen: size_t = 0;
    let mut buf: *mut c_char = ptr::null_mut();
    let fp = if access(TRACEFS_PIPE.as_ptr() as *const c_char, F_OK) == 0 {
        fopen(TRACEFS_PIPE.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    } else {
        fopen(DEBUGFS_PIPE.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    };
    if fp.is_null() {
        return -1;
    }
    if iter != 0 {
        fcntl(fileno(fp), F_SETFL, O_NONBLOCK);
    }
    loop {
        let n = getline(&mut buf, &mut buflen, fp);
        if !(n >= 0 || errno == EAGAIN) {
            break;
        }
        if n > 0 {
            cb.unwrap()(buf, data);
        }
        if iter != 0 {
            iter -= 1;
            if iter == 0 {
                break;
            }
        }
    }
    free(buf as *mut c_void);
    fclose(fp);
    0
}

unsafe extern "C" fn trace_pipe_cb(str_: *const c_char, _data: *mut c_void) {
    printf(b"%s\0".as_ptr() as *const c_char, str_);
}

#[no_mangle]
pub unsafe extern "C" fn read_trace_pipe() {
    read_trace_pipe_iter(Some(trace_pipe_cb), ptr::null_mut(), 0);
}

unsafe extern "C" fn symbol_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    str_hash(key as *const c_char)
}

unsafe extern "C" fn symbol_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    strcmp(key1 as *const c_char, key2 as *const c_char) == 0
}

unsafe extern "C" fn is_invalid_entry(buf: *mut c_char, kernel: bool) -> bool {
    if kernel && !strchr(buf, b'[' as c_int).is_null() {
        return true;
    }
    if !kernel && strchr(buf, b'[' as c_int).is_null() {
        return true;
    }
    false
}

static trace_blacklist: [*const c_char; 9] = [
    b"migrate_disable\0".as_ptr() as *const c_char,
    b"migrate_enable\0".as_ptr() as *const c_char,
    b"rcu_read_unlock_strict\0".as_ptr() as *const c_char,
    b"preempt_count_add\0".as_ptr() as *const c_char,
    b"preempt_count_sub\0".as_ptr() as *const c_char,
    b"__rcu_read_lock\0".as_ptr() as *const c_char,
    b"__rcu_read_unlock\0".as_ptr() as *const c_char,
    b"bpf_get_numa_node_id\0".as_ptr() as *const c_char,
    b"___migrate_enable\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn is_unsafe_function(name: *const c_char) -> bool {
    if strcmp(name, b"arch_cpu_idle\0".as_ptr() as *const c_char) == 0 {
        return true;
    }
    if strcmp(name, b"default_idle\0".as_ptr() as *const c_char) == 0 {
        return true;
    }
    if strncmp(name, b"rcu_\0".as_ptr() as *const c_char, 4) == 0 {
        return true;
    }
    if strcmp(name, b"bpf_dispatcher_xdp_func\0".as_ptr() as *const c_char) == 0 {
        return true;
    }
    if strncmp(
        name,
        b"__ftrace_invalid_address__\0".as_ptr() as *const c_char,
        mem::size_of_val(b"__ftrace_invalid_address__\0") - 1,
    ) == 0
    {
        return true;
    }
    let mut i = 0usize;
    while i < trace_blacklist.len() {
        if strcmp(name, trace_blacklist[i]) == 0 {
            return true;
        }
        i += 1;
    }
    false
}

unsafe extern "C" fn compare_name(name1: *const c_char, name2: *const c_char) -> c_int {
    let res1 = strstr(name1, b".llvm.\0".as_ptr() as *const c_char);
    let res2 = strstr(name2, b".llvm.\0".as_ptr() as *const c_char);
    let len1 = if !res1.is_null() { res1.offset_from(name1) as c_int } else { strlen(name1) as c_int };
    let len2 = if !res2.is_null() { res2.offset_from(name2) as c_int } else { strlen(name2) as c_int };
    if len1 == len2 {
        return strncmp(name1, name2, len1 as size_t);
    }
    if len1 < len2 {
        return if strncmp(name1, name2, len1 as size_t) <= 0 { -1 } else { 1 };
    }
    if strncmp(name1, name2, len2 as size_t) >= 0 { 1 } else { -1 }
}

unsafe extern "C" fn load_kallsyms_compare(p1: *const c_void, p2: *const c_void) -> c_int {
    compare_name((*p1.cast::<ksym>()).name, (*p2.cast::<ksym>()).name)
}

unsafe extern "C" fn search_kallsyms_compare(p1: *const c_void, p2: *const ksym) -> c_int {
    compare_name(p1 as *const c_char, (*p2).name)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_get_ksyms(ksymsp: *mut *mut ksyms, kernel: bool) -> c_int {
    let mut cap: size_t = 0;
    let mut cnt: size_t = 0;
    let mut name: *mut c_char = ptr::null_mut();
    let mut syms: *mut *mut c_char = ptr::null_mut();
    let ksyms = load_kallsyms_custom_local(Some(load_kallsyms_compare));
    if ksyms.is_null() {
        return -EINVAL;
    }
    let f = if access(TRACE_FILE.as_ptr() as *const c_char, F_OK) == 0 {
        fopen(AVAILABLE_FILTER_FUNCTIONS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    } else {
        fopen(DEBUG_AVAILABLE_FILTER_FUNCTIONS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    };
    if f.is_null() {
        free_kallsyms_local(ksyms);
        return -EINVAL;
    }
    let map = hashmap__new(Some(symbol_hash), Some(symbol_equal), ptr::null_mut());
    let mut err = 0;
    if is_err(map as *const c_void) {
        err = libbpf_get_error(map as *const c_void);
    } else {
        let mut buf = [0 as c_char; 256];
        while err == 0 && !fgets(buf.as_mut_ptr(), buf.len() as c_int, f).is_null() {
            if is_invalid_entry(buf.as_mut_ptr(), kernel) {
                continue;
            }
            free(name as *mut c_void);
            if sscanf(buf.as_ptr(), b"%ms%*[^\n]\n\0".as_ptr() as *const c_char, &mut name) != 1 {
                continue;
            }
            if is_unsafe_function(name) {
                continue;
            }
            let ks = search_kallsyms_custom_local(ksyms, name as *const c_void, Some(search_kallsyms_compare));
            if ks.is_null() {
                err = -EINVAL;
                break;
            }
            let ksym_name = (*ks).name;
            err = hashmap__add(map, ksym_name, 0);
            if err == -EEXIST {
                err = 0;
                continue;
            }
            if err != 0 {
                break;
            }
            let mut syms_ptr = syms as *mut c_void;
            err = libbpf_ensure_mem(&mut syms_ptr, &mut cap, mem::size_of::<*mut c_char>(), cnt + 1);
            syms = syms_ptr as *mut *mut c_char;
            if err != 0 {
                break;
            }
            *syms.add(cnt) = ksym_name;
            cnt += 1;
        }
        if err == 0 {
            (*ksyms).filtered_syms = syms;
            (*ksyms).filtered_cnt = cnt;
            *ksymsp = ksyms;
        }
    }
    free(name as *mut c_void);
    fclose(f);
    hashmap__free(map);
    if err != 0 {
        free(syms as *mut c_void);
        free_kallsyms_local(ksyms);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn bpf_get_addrs(
    addrsp: *mut *mut c_ulong,
    cntp: *mut size_t,
    kernel: bool,
) -> c_int {
    let mut name: *mut c_char = ptr::null_mut();
    let mut cnt: size_t = 0;
    let f = if access(TRACE_FILE.as_ptr() as *const c_char, F_OK) == 0 {
        fopen(AVAILABLE_FILTER_FUNCTIONS_ADDRS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    } else {
        fopen(DEBUG_AVAILABLE_FILTER_FUNCTIONS_ADDRS.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char)
    };
    if f.is_null() {
        return -ENOENT;
    }
    let mut max_cnt: c_int = 65536;
    let inc_cnt: c_int = 1024;
    let mut addrs = malloc(max_cnt as usize * mem::size_of::<c_long>()) as *mut c_ulong;
    let mut err = 0;
    if addrs.is_null() {
        err = -ENOMEM;
    } else {
        let mut buf = [0 as c_char; 256];
        while err == 0 && !fgets(buf.as_mut_ptr(), buf.len() as c_int, f).is_null() {
            if is_invalid_entry(buf.as_mut_ptr(), kernel) {
                continue;
            }
            free(name as *mut c_void);
            let mut addr: *mut c_ulong = ptr::null_mut();
            if sscanf(
                buf.as_ptr(),
                b"%p %ms%*[^\n]\n\0".as_ptr() as *const c_char,
                &mut addr,
                &mut name,
            ) != 2
            {
                continue;
            }
            if is_unsafe_function(name) {
                continue;
            }
            if cnt == max_cnt as usize {
                max_cnt += inc_cnt;
                let tmp_addrs = realloc(addrs as *mut c_void, max_cnt as usize * mem::size_of::<c_long>()) as *mut c_ulong;
                if tmp_addrs.is_null() {
                    err = -ENOMEM;
                    break;
                }
                addrs = tmp_addrs;
            }
            *addrs.add(cnt) = addr as c_ulong;
            cnt += 1;
        }
    }
    if err == 0 {
        *addrsp = addrs;
        *cntp = cnt;
    }
    free(name as *mut c_void);
    fclose(f);
    if err != 0 {
        free(addrs as *mut c_void);
    }
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
