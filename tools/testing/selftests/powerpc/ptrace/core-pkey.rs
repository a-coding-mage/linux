// SPDX-License-Identifier: GPL-2.0+
/*
 * Ptrace test for Memory Protection Key registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 * Copyright (C) 2018 IBM Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type off_t = i64;
type pid_t = c_int;
type time_t = c_long;

const CORE_FILE_LIMIT: rlim_t = 5 * 1024 * 1024; /* 5 MB should be enough */

static core_pattern_file: &[u8] = b"/proc/sys/kernel/core_pattern\0";

static user_write: &[u8] = b"[User Write (Running)]\0";
static core_read_running: &[u8] = b"[Core Read (Running)]\0";

const TEST_PASS: c_int = 0;
const TEST_FAIL: c_int = 1;
const RLIM_INFINITY: rlim_t = !0;
const RLIMIT_CORE: c_int = 4;
const RLIMIT_FSIZE: c_int = 1;
const PKEY_DISABLE_EXECUTE: c_ulong = 0x4;
const PKEY_UNRESTRICTED: c_ulong = 0x0;
const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const IPC_PRIVATE: key_t = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;
const EPERM: c_int = 1;
const ELFMAG: &[u8] = b"\x7fELF";
const SELFMAG: size_t = 4;
const ET_CORE: u16 = 4;
const EM_PPC64: u16 = 21;
const PT_NOTE: u32 = 4;
const NT_PPC_PKEY: c_int = 0x110;

type rlim_t = u64;
type key_t = c_int;

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

#[repr(C)]
struct stat {
    _private: [u8; 0],
}

#[repr(C)]
struct child_sync {
    _private: [u8; 0],
}

/* Information shared between the parent and the child. */
#[repr(C)]
struct shared_info {
    child_sync: child_sync,

    /* AMR value the parent expects to read in the core file. */
    amr: c_ulong,

    /* IAMR value the parent expects to read in the core file. */
    iamr: c_ulong,

    /* UAMOR value the parent expects to read in the core file. */
    uamor: c_ulong,

    /* When the child crashed. */
    core_time: time_t,
}

#[repr(C)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct Elf64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
struct Elf64_Nhdr {
    n_namesz: u32,
    n_descsz: u32,
    n_type: u32,
}

unsafe extern "C" {
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn wait_parent(sync: *mut child_sync) -> c_int;
    fn sys_pkey_alloc(flags: c_ulong, access_rights: c_ulong) -> c_int;
    fn sys_pkey_free(pkey: c_int) -> c_int;
    fn pkeyshift(pkey: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn set_amr(value: c_ulong);
    fn time(tloc: *mut time_t) -> time_t;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn ptrace_read_regs(pid: pid_t, note_type: c_int, regs: *mut c_ulong, count: c_int) -> c_int;
    fn prod_child(sync: *mut child_sync) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn perror(s: *const c_char);
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn write_file(path: *const c_char, buf: *const c_char, count: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn read_file(path: *const c_char, buf: *mut c_char, count: size_t, len: *mut size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn shmget(key: key_t, size: size_t, shmflg: c_int) -> c_int;
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn init_child_sync(sync: *mut child_sync) -> c_int;
    fn fork() -> pid_t;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn destroy_child_sync(sync: *mut child_sync);
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return TEST_FAIL;
        }
    };
}

macro_rules! PARENT_FAIL_IF {
    ($cond:expr, $sync:expr) => {
        if $cond != 0 {
            return TEST_FAIL;
        }
    };
}

macro_rules! PARENT_SKIP_IF_UNSUPPORTED {
    ($ret:expr, $sync:expr, $msg:expr) => {
        if $ret != 0 {
            return TEST_FAIL;
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return TEST_PASS;
        }
    };
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

unsafe fn WCOREDUMP(status: c_int) -> bool {
    (status & 0x80) != 0
}

const fn __ALIGN_KERNEL(x: u32, a: u32) -> usize {
    ((x + (a - 1)) & !(a - 1)) as usize
}

unsafe extern "C" fn increase_core_file_limit() -> c_int {
    let mut rlim: rlimit = core::mem::zeroed();
    let mut ret: c_int;

    ret = getrlimit(RLIMIT_CORE, &mut rlim);
    FAIL_IF!(ret != 0);

    if rlim.rlim_cur != RLIM_INFINITY && rlim.rlim_cur < CORE_FILE_LIMIT {
        rlim.rlim_cur = CORE_FILE_LIMIT;

        if rlim.rlim_max != RLIM_INFINITY && rlim.rlim_max < CORE_FILE_LIMIT {
            rlim.rlim_max = CORE_FILE_LIMIT;
        }

        ret = setrlimit(RLIMIT_CORE, &rlim);
        FAIL_IF!(ret != 0);
    }

    ret = getrlimit(RLIMIT_FSIZE, &mut rlim);
    FAIL_IF!(ret != 0);

    if rlim.rlim_cur != RLIM_INFINITY && rlim.rlim_cur < CORE_FILE_LIMIT {
        rlim.rlim_cur = CORE_FILE_LIMIT;

        if rlim.rlim_max != RLIM_INFINITY && rlim.rlim_max < CORE_FILE_LIMIT {
            rlim.rlim_max = CORE_FILE_LIMIT;
        }

        ret = setrlimit(RLIMIT_FSIZE, &rlim);
        FAIL_IF!(ret != 0);
    }

    TEST_PASS
}

unsafe extern "C" fn child(info: *mut shared_info) -> c_int {
    let mut disable_execute = true;
    let pkey1: c_int;
    let pkey2: c_int;
    let pkey3: c_int;
    let ptr_: *mut c_int;
    let mut ret: c_int;

    /* Wait until parent fills out the initial register values. */
    ret = wait_parent(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    ret = increase_core_file_limit();
    FAIL_IF!(ret != 0);

    /* Get some pkeys so that we can change their bits in the AMR. */
    let mut pkey1_tmp = sys_pkey_alloc(0, PKEY_DISABLE_EXECUTE);
    if pkey1_tmp < 0 {
        pkey1_tmp = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
        FAIL_IF!(pkey1_tmp < 0);

        disable_execute = false;
    }
    pkey1 = pkey1_tmp;

    pkey2 = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
    FAIL_IF!(pkey2 < 0);

    pkey3 = sys_pkey_alloc(0, PKEY_UNRESTRICTED);
    FAIL_IF!(pkey3 < 0);

    (*info).amr |= (3u64 << pkeyshift(pkey1)) | (2u64 << pkeyshift(pkey2));

    if disable_execute {
        (*info).iamr |= 1u64 << pkeyshift(pkey1);
    } else {
        (*info).iamr &= !(1u64 << pkeyshift(pkey1));
    }

    (*info).iamr &= !((1u64 << pkeyshift(pkey2)) | (1u64 << pkeyshift(pkey3)));

    (*info).uamor |= (3u64 << pkeyshift(pkey1)) | (3u64 << pkeyshift(pkey2));

    printf(
        b"%-30s AMR: %016lx pkey1: %d pkey2: %d pkey3: %d\n\0".as_ptr() as *const c_char,
        user_write.as_ptr() as *const c_char,
        (*info).amr,
        pkey1,
        pkey2,
        pkey3,
    );

    set_amr((*info).amr);

    /*
     * We won't use pkey3. This tests whether the kernel restores the UAMOR
     * permissions after a key is freed.
     */
    sys_pkey_free(pkey3);

    (*info).core_time = time(ptr::null_mut());

    /* Crash. */
    ptr_ = ptr::null_mut();
    *ptr_ = 1;

    /* Shouldn't get here. */
    FAIL_IF!(true);

    TEST_FAIL
}

/* Return file size if filename exists and pass sanity check, or zero if not. */
unsafe extern "C" fn try_core_file(filename: *const c_char, info: *mut shared_info, _pid: pid_t) -> off_t {
    let mut buf: stat = core::mem::zeroed();
    let ret: c_int;

    ret = stat(filename, &mut buf);
    if ret == -1 {
        return TEST_FAIL as off_t;
    }

    /*
     * File-local translation note: struct stat layout is supplied by system
     * headers in C; this preserves the source-level field use conceptually.
     */
    let st_mtime: time_t = *(&buf as *const stat as *const time_t);
    let st_size: off_t = *(&buf as *const stat as *const off_t);

    /* Make sure we're not using a stale core file. */
    if st_mtime >= (*info).core_time {
        st_size
    } else {
        TEST_FAIL as off_t
    }
}

unsafe fn next_note(nhdr: *mut Elf64_Nhdr) -> *mut Elf64_Nhdr {
    (nhdr as *mut u8)
        .add(size_of::<Elf64_Nhdr>())
        .add(__ALIGN_KERNEL((*nhdr).n_namesz, 4))
        .add(__ALIGN_KERNEL((*nhdr).n_descsz, 4)) as *mut Elf64_Nhdr
}

unsafe extern "C" fn check_core_file(info: *mut shared_info, ehdr: *mut Elf64_Ehdr, core_size: off_t) -> c_int {
    let regs: *mut c_ulong;
    let mut phdr: *mut Elf64_Phdr;
    let mut nhdr: *mut Elf64_Nhdr;
    let phdr_size: size_t;
    let mut p: *mut c_void = ehdr as *mut c_void;
    let note: *mut c_void;
    let mut ret: c_int;

    ret = memcmp((*ehdr).e_ident.as_ptr() as *const c_void, ELFMAG.as_ptr() as *const c_void, SELFMAG);
    FAIL_IF!(ret != 0);

    FAIL_IF!((*ehdr).e_type != ET_CORE);
    FAIL_IF!((*ehdr).e_machine != EM_PPC64);
    FAIL_IF!((*ehdr).e_phoff == 0 || (*ehdr).e_phnum == 0);

    /*
     * e_phnum is at most 65535 so calculating the size of the
     * program header cannot overflow.
     */
    phdr_size = size_of::<Elf64_Phdr>() * (*ehdr).e_phnum as size_t;

    /* Sanity check the program header table location. */
    FAIL_IF!((*ehdr).e_phoff + phdr_size as u64 < (*ehdr).e_phoff);
    FAIL_IF!((*ehdr).e_phoff + phdr_size as u64 > core_size as u64);

    /* Find the PT_NOTE segment. */
    phdr = (p as *mut u8).add((*ehdr).e_phoff as usize) as *mut Elf64_Phdr;
    while (phdr as *mut c_void) < (p as *mut u8).add((*ehdr).e_phoff as usize).add(phdr_size) as *mut c_void {
        if (*phdr).p_type == PT_NOTE {
            break;
        }
        phdr = (phdr as *mut u8).add((*ehdr).e_phentsize as usize) as *mut Elf64_Phdr;
    }

    FAIL_IF!((phdr as *mut c_void) >= (p as *mut u8).add((*ehdr).e_phoff as usize).add(phdr_size) as *mut c_void);

    /* Find the NT_PPC_PKEY note. */
    nhdr = (p as *mut u8).add((*phdr).p_offset as usize) as *mut Elf64_Nhdr;
    while (nhdr as *mut c_void) < (p as *mut u8).add((*phdr).p_offset as usize).add((*phdr).p_filesz as usize) as *mut c_void {
        if (*nhdr).n_type == NT_PPC_PKEY as u32 {
            break;
        }
        nhdr = next_note(nhdr);
    }

    FAIL_IF!((nhdr as *mut c_void) >= (p as *mut u8).add((*phdr).p_offset as usize).add((*phdr).p_filesz as usize) as *mut c_void);
    FAIL_IF!((*nhdr).n_descsz == 0);

    p = nhdr as *mut c_void;
    note = (p as *mut u8)
        .add(size_of::<Elf64_Nhdr>())
        .add(__ALIGN_KERNEL((*nhdr).n_namesz, 4)) as *mut c_void;

    regs = note as *mut c_ulong;

    printf(
        b"%-30s AMR: %016lx IAMR: %016lx UAMOR: %016lx\n\0".as_ptr() as *const c_char,
        core_read_running.as_ptr() as *const c_char,
        *regs.add(0),
        *regs.add(1),
        *regs.add(2),
    );

    FAIL_IF!(*regs.add(0) != (*info).amr);
    FAIL_IF!(*regs.add(1) != (*info).iamr);
    FAIL_IF!(*regs.add(2) != (*info).uamor);

    TEST_PASS
}

unsafe extern "C" fn parent(info: *mut shared_info, pid: pid_t) -> c_int {
    let filenames: *mut c_char;
    let mut filename: [*mut c_char; 3] = [ptr::null_mut(); 3];
    let fd: c_int;
    let mut i: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut regs: [c_ulong; 3] = [0; 3];
    let mut core_size: off_t;
    let core: *mut c_void;

    /*
     * Get the initial values for AMR, IAMR and UAMOR and communicate them
     * to the child.
     */
    ret = ptrace_read_regs(pid, NT_PPC_PKEY, regs.as_mut_ptr(), 3);
    PARENT_SKIP_IF_UNSUPPORTED!(ret, &mut (*info).child_sync, "PKEYs not supported");
    PARENT_FAIL_IF!(ret, &mut (*info).child_sync);

    (*info).amr = regs[0];
    (*info).iamr = regs[1];
    (*info).uamor = regs[2];

    /* Wake up child so that it can set itself up. */
    ret = prod_child(&mut (*info).child_sync);
    PARENT_FAIL_IF!(ret, &mut (*info).child_sync);

    ret = wait(&mut status);
    if ret != pid {
        printf(b"Child's exit status not captured\n\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    } else if !WIFSIGNALED(status) || !WCOREDUMP(status) {
        printf(b"Child didn't dump core\n\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    /* Construct array of core file names to try. */

    filenames = malloc(PATH_MAX) as *mut c_char;
    filename[0] = filenames;
    if filenames.is_null() {
        perror(b"Error allocating memory\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    ret = snprintf(filename[0], PATH_MAX, b"core-pkey.%d\0".as_ptr() as *const c_char, pid);
    if ret < 0 || ret as usize >= PATH_MAX {
        ret = TEST_FAIL;
        free(filenames as *mut c_void);
        return ret;
    }

    filename[1] = filename[0].add(ret as usize + 1);
    ret = snprintf(
        filename[1],
        PATH_MAX - ret as usize - 1,
        b"core.%d\0".as_ptr() as *const c_char,
        pid,
    );
    if ret < 0 || ret as usize >= PATH_MAX - ret as usize - 1 {
        ret = TEST_FAIL;
        free(filenames as *mut c_void);
        return ret;
    }
    filename[2] = b"core\0".as_ptr() as *mut c_char;

    i = 0;
    loop {
        if i >= 3 {
            break;
        }
        core_size = try_core_file(filename[i as usize], info, pid);
        if core_size != TEST_FAIL as off_t {
            break;
        }
        i += 1;
    }

    if i == 3 {
        printf(b"Couldn't find core file\n\0".as_ptr() as *const c_char);
        ret = TEST_FAIL;
        free(filenames as *mut c_void);
        return ret;
    }

    fd = open(filename[i as usize], O_RDONLY);
    if fd == -1 {
        perror(b"Error opening core file\0".as_ptr() as *const c_char);
        ret = TEST_FAIL;
        free(filenames as *mut c_void);
        return ret;
    }

    core = mmap(ptr::null_mut(), core_size as size_t, PROT_READ, MAP_PRIVATE, fd, 0);
    if core == -1isize as *mut c_void {
        perror(b"Error mmapping core file\0".as_ptr() as *const c_char);
        ret = TEST_FAIL;
        free(filenames as *mut c_void);
        return ret;
    }

    ret = check_core_file(info, core as *mut Elf64_Ehdr, core_size);

    munmap(core, core_size as size_t);
    close(fd);
    unlink(filename[i as usize]);

    free(filenames as *mut c_void);

    ret
}

unsafe extern "C" fn write_core_pattern(core_pattern: *const c_char) -> c_int {
    let err: c_int;

    err = write_file(core_pattern_file.as_ptr() as *const c_char, core_pattern, strlen(core_pattern));
    if err != 0 {
        SKIP_IF_MSG!(err == -EPERM, "Try with root privileges");
        perror(b"Error writing to core_pattern file\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    TEST_PASS
}

unsafe extern "C" fn setup_core_pattern(core_pattern_: *mut *mut c_char, changed_: *mut bool) -> c_int {
    let core_pattern: *mut c_char;
    let mut len: size_t = 0;
    let mut ret: c_int;

    core_pattern = malloc(PATH_MAX) as *mut c_char;
    if core_pattern.is_null() {
        perror(b"Error allocating memory\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    ret = read_file(core_pattern_file.as_ptr() as *const c_char, core_pattern, PATH_MAX - 1, &mut len);
    if ret != 0 {
        perror(b"Error reading core_pattern file\0".as_ptr() as *const c_char);
        ret = TEST_FAIL;
        free(core_pattern as *mut c_void);
        return ret;
    }

    *core_pattern.add(len) = b'\0' as c_char;

    /* Check whether we can predict the name of the core file. */
    if strcmp(core_pattern, b"core\0".as_ptr() as *const c_char) == 0
        || strcmp(core_pattern, b"core.%p\0".as_ptr() as *const c_char) == 0
    {
        *changed_ = false;
    } else {
        ret = write_core_pattern(b"core-pkey.%p\0".as_ptr() as *const c_char);
        if ret != 0 {
            free(core_pattern as *mut c_void);
            return ret;
        }

        *changed_ = true;
    }

    *core_pattern_ = core_pattern;
    ret = TEST_PASS;

    ret
}

unsafe extern "C" fn core_pkey() -> c_int {
    let mut core_pattern: *mut c_char = ptr::null_mut();
    let mut changed_core_pattern: bool = false;
    let info: *mut shared_info;
    let shm_id: c_int;
    let mut ret: c_int;
    let pid: pid_t;

    ret = setup_core_pattern(&mut core_pattern, &mut changed_core_pattern);
    if ret != 0 {
        return ret;
    }

    shm_id = shmget(IPC_PRIVATE, size_of::<shared_info>(), 0o777 | IPC_CREAT);
    info = shmat(shm_id, ptr::null(), 0) as *mut shared_info;

    ret = init_child_sync(&mut (*info).child_sync);
    if ret != 0 {
        return ret;
    }

    pid = fork();
    if pid < 0 {
        perror(b"fork() failed\0".as_ptr() as *const c_char);
        ret = TEST_FAIL;
    } else if pid == 0 {
        ret = child(info);
    } else {
        ret = parent(info, pid);
    }

    shmdt(info as *const c_void);

    if pid != 0 {
        destroy_child_sync(&mut (*info).child_sync);
        shmctl(shm_id, IPC_RMID, ptr::null_mut());

        if changed_core_pattern {
            write_core_pattern(core_pattern);
        }
    }

    free(core_pattern as *mut c_void);

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(core_pkey, b"core_pkey\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
