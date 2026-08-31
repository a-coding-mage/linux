/* SPDX-License-Identifier: GPL-2.0 */
/* Helpers shared by the binfmt_misc selftests. */

/* C header dependencies: elf.h, errno.h, fcntl.h, libgen.h, limits.h, link.h,
 * stdbool.h, stdio.h, stdlib.h, string.h, sys/mount.h, sys/types.h,
 * sys/wait.h, unistd.h.
 */

pub const BINFMT_DIR: &[u8] = b"/proc/sys/fs/binfmt_misc\0";
pub const BINFMT_REG: &[u8] = b"/proc/sys/fs/binfmt_misc/register\0";

/* comm holds 15 usable chars; a read of /proc/self/comm appends a newline. */
pub const TASK_COMM_LEN: usize = 16;

/* The canonical payload argv: run_payload() passes it, the payloads assert it. */
pub const PAYLOAD_ARGV0: &[u8] = b"payload-argv0\0";
pub const PAYLOAD_ARG1: &[u8] = b"argone\0";
pub const PAYLOAD_ARG2: &[u8] = b"argtwo\0";

/* Marker the loader tests poke into the payload's e_ident padding. */
pub const LOADER_MARKER: &[u8] = b"LDRTST\0";

/* Exit status run_payload() reports when the exec was refused as unhandled. */
pub const RUN_ENOEXEC: i32 = 42;

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_CREAT: i32 = 0o100;
pub const O_EXCL: i32 = 0o200;
pub const O_CLOEXEC: i32 = 0o2000000;
pub const F_OK: i32 = 0;
pub const ENOEXEC: i32 = 8;
pub const ETXTBSY: i32 = 26;
pub const PT_INTERP: u32 = 3;
pub const PATH_MAX: usize = 4096;

pub type c_char = i8;
pub type c_int = i32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_void = core::ffi::c_void;
pub type mode_t = u32;
pub type off_t = i64;
pub type pid_t = i32;
pub type size_t = usize;
pub type ssize_t = isize;

#[cfg(target_pointer_width = "64")]
pub type Elf_Ehdr = Elf64_Ehdr;
#[cfg(target_pointer_width = "64")]
pub type Elf_Phdr = Elf64_Phdr;

#[cfg(target_pointer_width = "32")]
pub type Elf_Ehdr = Elf32_Ehdr;
#[cfg(target_pointer_width = "32")]
pub type Elf_Phdr = Elf32_Phdr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Ehdr {
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
#[derive(Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; 16],
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

unsafe extern "C" {
    static mut errno: c_int;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn _exit(status: c_int) -> !;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fork() -> pid_t;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    fn strlen(s: *const c_char) -> size_t;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn unlink(pathname: *const c_char) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

#[inline]
pub unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
pub unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

pub unsafe fn copy_file(src: *const c_char, dst: *const c_char) -> c_int {
    let mut buf = [0u8; 4096];
    let mut n: ssize_t;

    let in_fd = unsafe { open(src, O_RDONLY) };
    if in_fd < 0 {
        return -1;
    }
    /* The tests share /tmp, so never write through a name they don't own. */
    unsafe { unlink(dst) };
    let out = unsafe { open(dst, O_WRONLY | O_CREAT | O_EXCL, 0o755 as mode_t) };
    if out < 0 {
        unsafe { close(in_fd) };
        return -1;
    }
    loop {
        n = unsafe { read(in_fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if n <= 0 {
            break;
        }
        if unsafe { write(out, buf.as_ptr() as *const c_void, n as size_t) } != n {
            unsafe { close(in_fd) };
            unsafe { close(out) };
            return -1;
        }
    }
    unsafe { close(in_fd) };
    unsafe { close(out) };
    if n < 0 { -1 } else { 0 }
}

/* Write @rule to the register file, preserving the write's errno. */
pub unsafe fn write_reg(rule: *const c_char) -> c_int {
    let saved: c_int;

    let fd = unsafe { open(BINFMT_REG.as_ptr() as *const c_char, O_WRONLY) };
    if fd < 0 {
        return -1;
    }
    let n = unsafe { write(fd, rule as *const c_void, strlen(rule)) };
    saved = unsafe { errno };
    unsafe { close(fd) };
    unsafe { errno = saved };
    if n < 0 { -1 } else { 0 }
}

pub unsafe fn unregister(name: *const c_char) {
    let mut path = [0 as c_char; PATH_MAX];

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"/proc/sys/fs/binfmt_misc/%s\0".as_ptr() as *const c_char,
            name,
        )
    };
    let fd = unsafe { open(path.as_ptr(), O_WRONLY) };
    if fd >= 0 {
        if unsafe { write(fd, b"-1\0".as_ptr() as *const c_void, 2) } < 0 {
            /* best effort */
        }
        unsafe { close(fd) };
    }
}

/* Write @line to @entry's file, reporting the errno it was refused with. */
pub unsafe fn entry_command(entry: *const c_char, line: *const c_char) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut retval: c_int = 0;
    let len = unsafe { strlen(line) };

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"/proc/sys/fs/binfmt_misc/%s\0".as_ptr() as *const c_char,
            entry,
        )
    };
    let fd = unsafe { open(path.as_ptr(), O_WRONLY | O_CLOEXEC) };
    if fd < 0 {
        return unsafe { -errno };
    }
    if unsafe { write(fd, line as *const c_void, len) } != len as ssize_t {
        retval = unsafe { -errno };
    }
    unsafe { close(fd) };
    retval
}

/* Does @entry's file report @line? */
pub unsafe fn entry_shows(entry: *const c_char, line: *const c_char) -> bool {
    let mut path = [0 as c_char; PATH_MAX];
    let mut buf = [0 as c_char; PATH_MAX];
    let mut found = false;

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"/proc/sys/fs/binfmt_misc/%s\0".as_ptr() as *const c_char,
            entry,
        )
    };
    let fp = unsafe { fopen(path.as_ptr(), b"r\0".as_ptr() as *const c_char) };
    if fp.is_null() {
        return false;
    }
    while !unsafe { fgets(buf.as_mut_ptr(), buf.len() as c_int, fp) }.is_null() {
        let nl = unsafe { strcspn(buf.as_ptr(), b"\n\0".as_ptr() as *const c_char) };
        buf[nl] = b'\0' as c_char;
        if unsafe { strcmp(buf.as_ptr(), line) } == 0 {
            found = true;
            break;
        }
    }
    unsafe { fclose(fp) };
    found
}

/* Mount binfmt_misc unless it already is, and report whether it is usable. */
pub unsafe fn binfmt_misc_available() -> bool {
    if unsafe { access(BINFMT_REG.as_ptr() as *const c_char, F_OK) } < 0 {
        unsafe {
            mount(
                b"binfmt_misc\0".as_ptr() as *const c_char,
                BINFMT_DIR.as_ptr() as *const c_char,
                b"binfmt_misc\0".as_ptr() as *const c_char,
                0,
                core::ptr::null(),
            )
        };
    }
    unsafe { access(BINFMT_REG.as_ptr() as *const c_char, F_OK) } == 0
}

/* Absolute path of @name in the directory this test was built into. */
pub unsafe fn artifact_path(out: *mut c_char, sz: size_t, name: *const c_char) -> c_int {
    let mut exe = [0 as c_char; PATH_MAX];

    let n = unsafe {
        readlink(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            exe.as_mut_ptr(),
            exe.len() - 1,
        )
    };
    if n < 0 {
        return -1;
    }
    exe[n as usize] = b'\0' as c_char;
    if unsafe {
        snprintf(
            out,
            sz,
            b"%s/%s\0".as_ptr() as *const c_char,
            dirname(exe.as_mut_ptr()),
            name,
        )
    } as size_t
        >= sz
    {
        return -1;
    }
    0
}

/* Probe kernel support for a registration flag with a throwaway entry. */
pub unsafe fn binfmt_flag_supported(flag: c_char) -> bool {
    let mut rule = [0 as c_char; 64];

    unsafe {
        snprintf(
            rule.as_mut_ptr(),
            rule.len(),
            b":bm_flag_probe:E::bmprobe::/bin/true:%c\0".as_ptr() as *const c_char,
            flag as c_int,
        )
    };
    if unsafe { write_reg(rule.as_ptr()) } != 0 {
        return false;
    }
    unsafe { unregister(b"bm_flag_probe\0".as_ptr() as *const c_char) };
    true
}

/*
 * Run @path with the canonical payload argv and return its exit status, or
 * RUN_ENOEXEC when the exec itself was refused as unhandled.
 */
pub unsafe fn run_payload(path: *const c_char) -> c_int {
    let mut status: c_int = 0;

    let pid = unsafe { fork() };
    if pid == 0 {
        unsafe {
            execl(
                path,
                PAYLOAD_ARGV0.as_ptr() as *const c_char,
                PAYLOAD_ARG1.as_ptr() as *const c_char,
                PAYLOAD_ARG2.as_ptr() as *const c_char,
                core::ptr::null::<c_char>(),
            )
        };
        unsafe { _exit(if errno == ENOEXEC { RUN_ENOEXEC } else { 126 }) };
    }
    if pid < 0
        || unsafe { waitpid(pid, &mut status, 0) } != pid
        || !unsafe { WIFEXITED(status) }
    {
        return -1;
    }
    unsafe { WEXITSTATUS(status) }
}

/* Does the exe link name @path? */
pub unsafe fn exe_is(path: *const c_char) -> bool {
    let mut exe = [0 as c_char; PATH_MAX];
    let mut real = [0 as c_char; PATH_MAX];

    let n = unsafe {
        readlink(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            exe.as_mut_ptr(),
            exe.len() - 1,
        )
    };
    if n <= 0 || unsafe { realpath(path, real.as_mut_ptr()) }.is_null() {
        return false;
    }
    exe[n as usize] = b'\0' as c_char;
    unsafe { strcmp(exe.as_ptr(), real.as_ptr()) } == 0
}

/* Is comm @name truncated to what a comm can hold? */
pub unsafe fn comm_is(name: *const c_char) -> bool {
    let mut comm = [0 as c_char; TASK_COMM_LEN + 2];
    let mut expect = [0 as c_char; TASK_COMM_LEN];

    let fd = unsafe { open(b"/proc/self/comm\0".as_ptr() as *const c_char, O_RDONLY) };
    if fd < 0 {
        return false;
    }
    let mut n = unsafe { read(fd, comm.as_mut_ptr() as *mut c_void, comm.len() - 1) };
    unsafe { close(fd) };
    if n <= 0 {
        return false;
    }
    if comm[(n - 1) as usize] == b'\n' as c_char {
        n -= 1;
    }
    comm[n as usize] = b'\0' as c_char;
    unsafe {
        snprintf(
            expect.as_mut_ptr(),
            expect.len(),
            b"%s\0".as_ptr() as *const c_char,
            name,
        )
    };
    unsafe { strcmp(comm.as_ptr(), expect.as_ptr()) } == 0
}

/* Opening @path for writing has to fail with ETXTBSY. */
pub unsafe fn write_denied(path: *const c_char) -> bool {
    let fd = unsafe { open(path, O_WRONLY) };

    if fd >= 0 {
        unsafe { close(fd) };
        return false;
    }
    unsafe { errno == ETXTBSY }
}

pub unsafe fn patch_file(
    path: *const c_char,
    off: off_t,
    data: *const c_void,
    len: size_t,
) -> c_int {
    let fd = unsafe { open(path, O_WRONLY) };
    if fd < 0 {
        return -1;
    }
    let n = unsafe { pwrite(fd, data, len, off) };
    unsafe { close(fd) };
    if n == len as ssize_t { 0 } else { -1 }
}

/* start_code and end_code are the 26th and 27th fields of /proc/pid/stat. */
pub unsafe fn stat_codes(
    pid: pid_t,
    start_code: *mut c_ulong,
    end_code: *mut c_ulong,
) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut path = [0 as c_char; 64];

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"/proc/%d/stat\0".as_ptr() as *const c_char,
            pid,
        )
    };
    let fd = unsafe { open(path.as_ptr(), O_RDONLY) };
    if fd < 0 {
        return -1;
    }
    let n = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1) };
    unsafe { close(fd) };
    if n <= 0 {
        return -1;
    }
    buf[n as usize] = b'\0' as c_char;

    /* Skip "pid (comm)", then start_code is the 24th field after it. */
    let mut p = unsafe { strrchr(buf.as_ptr(), b')' as c_int) };
    if p.is_null() {
        return -1;
    }
    p = unsafe { p.add(1) };
    for _i in 0..23 {
        p = unsafe { strchr(p.add(1), b' ' as c_int) };
        if p.is_null() {
            return -1;
        }
    }
    if unsafe {
        sscanf(
            p,
            b" %lu %lu\0".as_ptr() as *const c_char,
            start_code,
            end_code,
        )
    } != 2
    {
        return -1;
    }
    0
}

/* Find the system loader through our own PT_INTERP. */
pub unsafe fn find_loader(out: *mut c_char, sz: size_t) -> c_int {
    let mut eh = core::mem::MaybeUninit::<Elf_Ehdr>::uninit();
    let mut ph = core::mem::MaybeUninit::<Elf_Phdr>::uninit();
    let mut ret = -1;

    let fd = unsafe { open(b"/proc/self/exe\0".as_ptr() as *const c_char, O_RDONLY) };
    if fd < 0 {
        return -1;
    }
    if unsafe {
        pread(
            fd,
            eh.as_mut_ptr() as *mut c_void,
            core::mem::size_of::<Elf_Ehdr>(),
            0,
        )
    } != core::mem::size_of::<Elf_Ehdr>() as ssize_t
    {
        unsafe { close(fd) };
        return ret;
    }
    let eh = unsafe { eh.assume_init() };
    for i in 0..eh.e_phnum as c_int {
        if unsafe {
            pread(
                fd,
                ph.as_mut_ptr() as *mut c_void,
                core::mem::size_of::<Elf_Phdr>(),
                (eh.e_phoff as off_t) + (i as off_t) * (eh.e_phentsize as off_t),
            )
        } != core::mem::size_of::<Elf_Phdr>() as ssize_t
        {
            unsafe { close(fd) };
            return ret;
        }
        let ph_val = unsafe { ph.assume_init() };
        if ph_val.p_type != PT_INTERP {
            continue;
        }
        if ph_val.p_filesz == 0 || (ph_val.p_filesz as size_t) > sz {
            unsafe { close(fd) };
            return ret;
        }
        if unsafe {
            pread(
                fd,
                out as *mut c_void,
                ph_val.p_filesz as size_t,
                ph_val.p_offset as off_t,
            )
        } != ph_val.p_filesz as ssize_t
        {
            unsafe { close(fd) };
            return ret;
        }
        unsafe { *out.add(ph_val.p_filesz as usize - 1) = b'\0' as c_char };
        ret = 0;
        break;
    }
    unsafe { close(fd) };
    ret
}
