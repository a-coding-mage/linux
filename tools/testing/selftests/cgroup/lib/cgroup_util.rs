/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of cgroup_util.c.
 *
 * Original C dependencies:
 * - cgroup_util.h provides BUF_SIZE and CG_THREADS_FILE.
 * - ../../clone3/clone3_selftests.h provides struct __clone_args,
 *   CLONE_ARGS_SIZE_VER2, CLONE_INTO_CGROUP, and sys_clone3().
 */

use libc::{
    c_char, c_int, c_long, c_void, mode_t, pid_t, pollfd, siginfo_t, size_t, ssize_t, useconds_t,
};

const PATH_MAX: usize = libc::PATH_MAX as usize;
const BUF_SIZE: usize = 4096;
const CG_THREADS_FILE: *const c_char = b"cgroup.threads\0".as_ptr() as *const c_char;

#[repr(C)]
struct __clone_args {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

const CLONE_INTO_CGROUP: u64 = 0x200000000;
const __WALL: c_int = 0x40000000;
const __WNOTHREAD: c_int = 0x20000000;

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t;
}

unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

unsafe fn get_errno() -> c_int {
    *errno_location()
}

unsafe fn set_errno(errno: c_int) {
    *errno_location() = errno;
}

#[unsafe(no_mangle)]
pub static mut cg_test_v1_named: bool = false;

/* Returns read len on success, or -errno on failure. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_text(
    path: *const c_char,
    buf: *mut c_char,
    max_len: size_t,
) -> ssize_t {
    let fd: c_int = libc::open(path, libc::O_RDONLY);
    if fd < 0 {
        return -(get_errno() as ssize_t);
    }

    let len: ssize_t = libc::read(fd, buf as *mut c_void, max_len.wrapping_sub(1));

    if len >= 0 {
        *buf.offset(len as isize) = 0;
    }

    libc::close(fd);
    if len < 0 {
        -(get_errno() as ssize_t)
    } else {
        len
    }
}

/* Returns written len on success, or -errno on failure. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_text(
    path: *const c_char,
    buf: *mut c_char,
    mut len: ssize_t,
) -> ssize_t {
    let fd: c_int = libc::open(path, libc::O_WRONLY | libc::O_APPEND);
    if fd < 0 {
        return -(get_errno() as ssize_t);
    }

    len = libc::write(fd, buf as *const c_void, len as size_t);
    libc::close(fd);
    if len < 0 {
        -(get_errno() as ssize_t)
    } else {
        len
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char {
    let len: size_t = libc::strlen(root) + libc::strlen(name) + 2;
    let ret: *mut c_char = libc::malloc(len) as *mut c_char;

    if !ret.is_null() {
        snprintf(ret, len, c"%s/%s".as_ptr(), root, name);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_name_indexed(
    root: *const c_char,
    name: *const c_char,
    index: c_int,
) -> *mut c_char {
    let len: size_t = libc::strlen(root) + libc::strlen(name) + 10;
    let ret: *mut c_char = libc::malloc(len) as *mut c_char;

    if !ret.is_null() {
        snprintf(ret, len, c"%s/%s_%d".as_ptr(), root, name, index);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_control(cgroup: *const c_char, control: *const c_char) -> *mut c_char {
    let len: size_t = libc::strlen(cgroup) + libc::strlen(control) + 2;
    let ret: *mut c_char = libc::malloc(len) as *mut c_char;

    if !ret.is_null() {
        snprintf(ret, len, c"%s/%s".as_ptr(), cgroup, control);
    }

    ret
}

/* Returns 0 on success, or -errno on failure. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read(
    cgroup: *const c_char,
    control: *const c_char,
    buf: *mut c_char,
    len: size_t,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];

    snprintf(path.as_mut_ptr(), path.len(), c"%s/%s".as_ptr(), cgroup, control);

    let ret: ssize_t = read_text(path.as_ptr(), buf, len);
    if ret >= 0 {
        0
    } else {
        ret as c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_strcmp(
    cgroup: *const c_char,
    control: *const c_char,
    expected: *const c_char,
) -> c_int {
    let size: size_t;
    let buf: *mut c_char;
    let ret: c_int;

    /* Handle the case of comparing against empty string */
    if expected.is_null() {
        return -1;
    }

    /* needs size > 1, otherwise cg_read() reads 0 bytes */
    size = if *expected == b'\0' as c_char {
        2
    } else {
        libc::strlen(expected) + 1
    };

    buf = libc::malloc(size) as *mut c_char;
    if buf.is_null() {
        return -1;
    }

    if cg_read(cgroup, control, buf, size) != 0 {
        libc::free(buf as *mut c_void);
        return -1;
    }

    ret = libc::strcmp(expected, buf);
    libc::free(buf as *mut c_void);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_strcmp_wait(
    cgroup: *const c_char,
    control: *const c_char,
    expected: *const c_char,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = -1;

    i = 0;
    while i < 100 {
        ret = cg_read_strcmp(cgroup, control, expected);
        if ret == 0 {
            return ret;
        }
        libc::usleep(10000);
        i += 1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_strstr(
    cgroup: *const c_char,
    control: *const c_char,
    needle: *const c_char,
) -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];

    if cg_read(cgroup, control, buf.as_mut_ptr(), buf.len()) != 0 {
        return -1;
    }

    if libc::strstr(buf.as_ptr(), needle).is_null() {
        -1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long {
    let mut buf = [0 as c_char; 128];

    if cg_read(cgroup, control, buf.as_mut_ptr(), buf.len()) != 0 {
        return -1;
    }

    libc::atol(buf.as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_long_fd(fd: c_int) -> c_long {
    let mut buf = [0 as c_char; 128];

    if libc::pread(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) <= 0 {
        return -1;
    }

    libc::atol(buf.as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_key_long(
    cgroup: *const c_char,
    control: *const c_char,
    key: *const c_char,
) -> c_long {
    let mut buf = [0 as c_char; BUF_SIZE];
    let ptr: *mut c_char;

    if cg_read(cgroup, control, buf.as_mut_ptr(), buf.len()) != 0 {
        return -1;
    }

    ptr = libc::strstr(buf.as_ptr(), key);
    if ptr.is_null() {
        return -1;
    }

    libc::atol(ptr.add(libc::strlen(key)) as *const c_char)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_key_long_poll(
    cgroup: *const c_char,
    control: *const c_char,
    key: *const c_char,
    expected: c_long,
    retries: c_int,
    wait_interval_us: useconds_t,
) -> c_long {
    let mut val: c_long = -1;
    let mut i: c_int = 0;

    while i < retries {
        val = cg_read_key_long(cgroup, control, key);
        if val < 0 {
            return val;
        }

        if val == expected {
            break;
        }

        libc::usleep(wait_interval_us);
        i += 1;
    }

    val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_read_lc(cgroup: *const c_char, control: *const c_char) -> c_long {
    let mut buf = [0 as c_char; BUF_SIZE];
    let delim = c"\n";
    let mut line: *mut c_char;
    let mut cnt: c_long = 0;

    if cg_read(cgroup, control, buf.as_mut_ptr(), buf.len()) != 0 {
        return -1;
    }

    line = libc::strtok(buf.as_mut_ptr(), delim.as_ptr());
    while !line.is_null() {
        cnt += 1;
        line = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
    }

    cnt
}

/* Returns 0 on success, or -errno on failure. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_write(
    cgroup: *const c_char,
    control: *const c_char,
    buf: *mut c_char,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let len: ssize_t = libc::strlen(buf) as ssize_t;

    snprintf(path.as_mut_ptr(), path.len(), c"%s/%s".as_ptr(), cgroup, control);
    let ret: ssize_t = write_text(path.as_ptr(), buf, len);
    if ret == len {
        0
    } else {
        ret as c_int
    }
}

/*
 * Returns fd on success, or -1 on failure.
 * (fd should be closed with close() as usual)
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_open(
    cgroup: *const c_char,
    control: *const c_char,
    flags: c_int,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];

    snprintf(path.as_mut_ptr(), path.len(), c"%s/%s".as_ptr(), cgroup, control);
    libc::open(path.as_ptr(), flags)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_write_numeric(
    cgroup: *const c_char,
    control: *const c_char,
    value: c_long,
) -> c_int {
    let mut buf = [0 as c_char; 64];
    let ret: c_int;

    ret = sprintf(buf.as_mut_ptr(), c"%lu".as_ptr(), value);
    if ret < 0 {
        return ret;
    }

    cg_write(cgroup, control, buf.as_mut_ptr())
}

unsafe extern "C" fn cg_find_root(
    root: *mut c_char,
    len: size_t,
    controller: *const c_char,
    nsdelegate: *mut bool,
) -> c_int {
    let mut buf = [0 as c_char; 10 * BUF_SIZE];
    let delim = c"\n\t ";
    let mut fs: *mut c_char;
    let mut mount: *mut c_char;
    let mut typ: *mut c_char;
    let mut options: *mut c_char;

    if read_text(c"/proc/self/mounts".as_ptr(), buf.as_mut_ptr(), buf.len()) <= 0 {
        return -1;
    }

    /*
     * Example:
     * cgroup /sys/fs/cgroup cgroup2 rw,seclabel,noexec,relatime 0 0
     */
    fs = libc::strtok(buf.as_mut_ptr(), delim.as_ptr());
    while !fs.is_null() {
        mount = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
        typ = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
        options = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
        libc::strtok(std::ptr::null_mut(), delim.as_ptr());
        libc::strtok(std::ptr::null_mut(), delim.as_ptr());
        if libc::strcmp(typ, c"cgroup".as_ptr()) == 0 {
            if controller.is_null() || libc::strstr(options, controller).is_null() {
                fs = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
                continue;
            }
        } else if libc::strcmp(typ, c"cgroup2".as_ptr()) == 0 {
            if !controller.is_null()
                && cg_read_strstr(mount, c"cgroup.controllers".as_ptr(), controller) != 0
            {
                fs = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
                continue;
            }
        } else {
            fs = libc::strtok(std::ptr::null_mut(), delim.as_ptr());
            continue;
        }
        libc::strncpy(root, mount, len);

        if !nsdelegate.is_null() {
            *nsdelegate = !libc::strstr(options, c"nsdelegate".as_ptr()).is_null();
        }
        return 0;
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_find_controller_root(
    root: *mut c_char,
    len: size_t,
    controller: *const c_char,
) -> c_int {
    cg_find_root(root, len, controller, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_find_unified_root(
    root: *mut c_char,
    len: size_t,
    nsdelegate: *mut bool,
) -> c_int {
    cg_find_root(root, len, std::ptr::null(), nsdelegate)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_create(cgroup: *const c_char) -> c_int {
    libc::mkdir(cgroup, 0o755 as mode_t)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int {
    let mut buf = [0 as c_char; 10 * BUF_SIZE];
    let mut attempts: c_int;
    let mut ptr: *mut c_char;

    attempts = 10;
    while attempts >= 0 {
        let mut nr: c_int = 0;

        if cg_read(cgroup, c"cgroup.procs".as_ptr(), buf.as_mut_ptr(), buf.len()) != 0 {
            break;
        }

        ptr = buf.as_mut_ptr();
        while *ptr != 0 {
            if *ptr == b'\n' as c_char {
                nr += 1;
            }
            ptr = ptr.add(1);
        }

        if nr >= count {
            return 0;
        }

        libc::usleep(100000);
        attempts -= 1;
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_killall(cgroup: *const c_char) -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];
    let mut ptr: *mut c_char = buf.as_mut_ptr();

    /* If cgroup.kill exists use it. */
    if cg_write(cgroup, c"cgroup.kill".as_ptr(), c"1".as_ptr() as *mut c_char) == 0 {
        return 0;
    }

    if cg_read(cgroup, c"cgroup.procs".as_ptr(), buf.as_mut_ptr(), buf.len()) != 0 {
        return -1;
    }

    while ptr < buf.as_mut_ptr().add(buf.len()) {
        let pid: c_int = libc::strtol(ptr, &mut ptr, 10) as c_int;

        if pid == 0 {
            break;
        }
        if *ptr != 0 {
            ptr = ptr.add(1);
        } else {
            break;
        }
        if libc::kill(pid, libc::SIGKILL) != 0 {
            return -1;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_destroy(cgroup: *const c_char) -> c_int {
    let mut ret: c_int;

    if cgroup.is_null() {
        return 0;
    }
    loop {
        ret = libc::rmdir(cgroup);
        if ret != 0 && get_errno() == libc::EBUSY {
            cg_killall(cgroup);
            libc::usleep(100);
            continue;
        }

        if ret != 0 && get_errno() == libc::ENOENT {
            ret = 0;
        }

        return ret;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_enter(cgroup: *const c_char, pid: c_int) -> c_int {
    let mut pidbuf = [0 as c_char; 64];

    snprintf(pidbuf.as_mut_ptr(), pidbuf.len(), c"%d".as_ptr(), pid);
    cg_write(cgroup, c"cgroup.procs".as_ptr(), pidbuf.as_mut_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_enter_current(cgroup: *const c_char) -> c_int {
    cg_write(cgroup, c"cgroup.procs".as_ptr(), c"0".as_ptr() as *mut c_char)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_enter_current_thread(cgroup: *const c_char) -> c_int {
    cg_write(cgroup, CG_THREADS_FILE, c"0".as_ptr() as *mut c_char)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_run(
    cgroup: *const c_char,
    fn_: Option<unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int>,
    arg: *mut c_void,
) -> c_int {
    let pid: c_int;
    let mut retcode: c_int = 0;

    pid = libc::fork();
    if pid < 0 {
        pid
    } else if pid == 0 {
        let mut buf = [0 as c_char; 64];

        snprintf(buf.as_mut_ptr(), buf.len(), c"%d".as_ptr(), libc::getpid());
        if cg_write(cgroup, c"cgroup.procs".as_ptr(), buf.as_mut_ptr()) != 0 {
            libc::exit(libc::EXIT_FAILURE);
        }
        libc::exit(fn_.unwrap()(cgroup, arg));
    } else {
        libc::waitpid(pid, &mut retcode, 0);
        if libc::WIFEXITED(retcode) {
            libc::WEXITSTATUS(retcode)
        } else {
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clone_into_cgroup(cgroup_fd: c_int) -> pid_t {
    let mut args = __clone_args {
        flags: CLONE_INTO_CGROUP,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: libc::SIGCHLD as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: cgroup_fd as u64,
    };

    let pid: pid_t = sys_clone3(&mut args, std::mem::size_of::<__clone_args>());
    /*
     * Verify that this is a genuine test failure:
     * ENOSYS -> clone3() not available
     * E2BIG  -> CLONE_INTO_CGROUP not available
     */
    if pid < 0 && (get_errno() == libc::ENOSYS || get_errno() == libc::E2BIG) {
        set_errno(libc::ENOSYS);
        return -libc::ENOSYS;
    }

    pid
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clone_reap(pid: pid_t, options: c_int) -> c_int {
    let mut ret: c_int;
    let mut info: siginfo_t = std::mem::zeroed();

    info.si_signo = 0;

    loop {
        ret = libc::waitid(
            libc::P_PID,
            pid as libc::id_t,
            &mut info,
            options | __WALL | __WNOTHREAD,
        );
        if ret < 0 {
            if get_errno() == libc::EINTR {
                continue;
            }
            return -1;
        }
        break;
    }

    if (options & libc::WEXITED) != 0 {
        if libc::WIFEXITED(info.si_status) {
            return libc::WEXITSTATUS(info.si_status);
        }
    }

    if (options & libc::WSTOPPED) != 0 {
        if libc::WIFSTOPPED(info.si_status) {
            return libc::WSTOPSIG(info.si_status);
        }
    }

    if (options & libc::WCONTINUED) != 0 {
        if libc::WIFCONTINUED(info.si_status) {
            return 0;
        }
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dirfd_open_opath(dir: *const c_char) -> c_int {
    libc::open(
        dir,
        libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_PATH,
    )
}

unsafe fn close_prot_errno(fd: c_int) {
    if fd >= 0 {
        let e = get_errno();
        libc::close(fd);
        set_errno(e);
    }
}

unsafe extern "C" fn clone_into_cgroup_run_nowait(
    cgroup: *const c_char,
    fn_: Option<unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int>,
    arg: *mut c_void,
) -> c_int {
    let cgroup_fd: c_int;
    let pid: pid_t;

    cgroup_fd = dirfd_open_opath(cgroup);
    if cgroup_fd < 0 {
        return -1;
    }

    pid = clone_into_cgroup(cgroup_fd);
    close_prot_errno(cgroup_fd);
    if pid == 0 {
        libc::exit(fn_.unwrap()(cgroup, arg));
    }

    pid
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_run_nowait(
    cgroup: *const c_char,
    fn_: Option<unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int>,
    arg: *mut c_void,
) -> c_int {
    let mut pid: c_int;

    pid = clone_into_cgroup_run_nowait(cgroup, fn_, arg);
    if pid > 0 {
        return pid;
    }

    /* Genuine test failure. */
    if pid < 0 && get_errno() != libc::ENOSYS {
        return -1;
    }

    pid = libc::fork();
    if pid == 0 {
        let mut buf = [0 as c_char; 64];

        snprintf(buf.as_mut_ptr(), buf.len(), c"%d".as_ptr(), libc::getpid());
        if cg_write(cgroup, c"cgroup.procs".as_ptr(), buf.as_mut_ptr()) != 0 {
            libc::exit(libc::EXIT_FAILURE);
        }
        libc::exit(fn_.unwrap()(cgroup, arg));
    }

    pid
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_mount_contains(option: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 4 * BUF_SIZE];
    let read: ssize_t;

    read = read_text(c"/proc/mounts".as_ptr(), buf.as_mut_ptr(), buf.len());
    if read < 0 {
        return read as c_int;
    }

    (!libc::strstr(buf.as_ptr(), option).is_null()) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroup_feature(feature: *const c_char) -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];
    let read: ssize_t;

    read = read_text(
        c"/sys/kernel/cgroup/features".as_ptr(),
        buf.as_mut_ptr(),
        buf.len(),
    );
    if read < 0 {
        return read as c_int;
    }

    (!libc::strstr(buf.as_ptr(), feature).is_null()) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_read_text(
    pid: c_int,
    thread: bool,
    item: *const c_char,
    buf: *mut c_char,
    size: size_t,
) -> ssize_t {
    let mut path = [0 as c_char; PATH_MAX];
    let ret: ssize_t;

    if pid == 0 {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"/proc/%s/%s".as_ptr(),
            if thread {
                c"thread-self".as_ptr()
            } else {
                c"self".as_ptr()
            },
            item,
        );
    } else {
        snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/%s".as_ptr(), pid, item);
    }

    ret = read_text(path.as_ptr(), buf, size);
    if ret < 0 {
        -1
    } else {
        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn proc_read_strstr(
    pid: c_int,
    thread: bool,
    item: *const c_char,
    needle: *const c_char,
) -> c_int {
    let mut buf = [0 as c_char; BUF_SIZE];

    if proc_read_text(pid, thread, item, buf.as_mut_ptr(), buf.len()) < 0 {
        return -1;
    }

    if libc::strstr(buf.as_ptr(), needle).is_null() {
        -1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clone_into_cgroup_run_wait(cgroup: *const c_char) -> c_int {
    let cgroup_fd: c_int;
    let pid: pid_t;

    cgroup_fd = dirfd_open_opath(cgroup);
    if cgroup_fd < 0 {
        return -1;
    }

    pid = clone_into_cgroup(cgroup_fd);
    close_prot_errno(cgroup_fd);
    if pid < 0 {
        return -1;
    }

    if pid == 0 {
        libc::exit(libc::EXIT_SUCCESS);
    }

    /*
     * We don't care whether this fails. We only care whether the initial
     * clone succeeded.
     */
    let _ = clone_reap(pid, libc::WEXITED);
    0
}

unsafe extern "C" fn __prepare_for_wait(cgroup: *const c_char, filename: *const c_char) -> c_int {
    let mut fd: c_int;
    let mut ret: c_int = -1;

    fd = libc::inotify_init1(0);
    if fd == -1 {
        return fd;
    }

    ret = libc::inotify_add_watch(fd, cg_control(cgroup, filename), libc::IN_MODIFY);
    if ret == -1 {
        libc::close(fd);
        fd = -1;
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int {
    __prepare_for_wait(cgroup, c"cgroup.events".as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcg_prepare_for_wait(cgroup: *const c_char) -> c_int {
    __prepare_for_wait(cgroup, c"memory.events".as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cg_wait_for(fd: c_int) -> c_int {
    let mut ret: c_int = -1;
    let mut fds = pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };

    loop {
        ret = libc::poll(&mut fds, 1, 10000);

        if ret == -1 {
            if get_errno() == libc::EINTR {
                continue;
            }

            break;
        }

        if ret > 0 && (fds.revents & libc::POLLIN) != 0 {
            ret = 0;
            break;
        }
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
