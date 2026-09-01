/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Syscall definitions for NOLIBC (those in man(2))
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols: "nolibc.h" */
/* dependency intent: "std.h", linux syscall constants, errno, stdarg, types */

/*
 * Syscall return helper: takes the syscall value in argument and checks for an
 * error in it. This may only be used with signed returns (int or long), but
 * not with pointers. An error is any value < 0. When an error is encountered,
 * -ret is set into errno and -1 is returned. Otherwise the returned value is
 * passed as-is with its type preserved.
 */
#[inline]
pub unsafe fn __sysret<T>(arg: T) -> T
where
    T: Copy + PartialOrd + From<i8> + core::ops::Neg<Output = T>,
{
    let __sysret_arg = arg;
    if __sysret_arg < T::from(0) {
        SET_ERRNO(-__sysret_arg);
        T::from(-1)
    } else {
        __sysret_arg
    }
}

/*
 * Syscall ENOSYS helper: Avoids unused-parameter warnings, provides compile
 * time validation and a debugging hook.
 */
/* if defined(NOLIBC_COMPILE_TIME_ENOSYS) */
#[inline]
pub unsafe fn __nolibc_enosys(_syscall: *const c_char, ...) -> c_int {
    -ENOSYS
}
/* elif __nolibc_has_attribute(error): extern int __nolibc_enosys(const char *syscall, ...); */
/* else:
 * static inline returns extern int __nolibc_enosys_error after consuming syscall.
 */

/*
 * Helper for 32-bit machines where a 64-bit syscall arg needs to be split into
 * two 32-bit parts while making sure the order of the low/high parts are correct
 * for the endianness:
 * __NOLIBC_LLARGPART(x, 0), __NOLIBC_LLARGPART(x, 1)
 */
#[repr(C)]
pub union __nolibc_llargpart_union {
    pub ll: c_longlong,
    pub l: [c_long; 2],
}

#[inline]
pub unsafe fn __NOLIBC_LLARGPART(_arg: c_longlong, _part: usize) -> c_long {
    __nolibc_llargpart_union { ll: _arg }.l[_part]
}

/*
 * Functions in this file only describe syscalls. They're declared static so
 * that the compiler usually decides to inline them while still being allowed
 * to pass a pointer to one of their instances. Each syscall exists in two
 * versions:
 *   - the "internal" ones, which matches the raw syscall interface at the
 *     kernel level, which may sometimes slightly differ from the documented
 *     libc-level ones. For example most of them return either a valid value
 *     or -errno. All of these are prefixed with "_sys_". They may be called
 *     by non-portable applications if desired.
 *
 *   - the "exported" ones, whose interface must closely match the one
 *     documented in man(2), that applications are supposed to expect. These
 *     ones rely on the internal ones, and set errno.
 *
 * Each syscall will be defined with the two functions, sorted in alphabetical
 * order applied to the exported names.
 *
 * In case of doubt about the relevance of a function here, only those which
 * set errno should be defined here. Wrappers like those appearing in man(3)
 * should not be placed here.
 */

/*
 * int brk(void *addr);
 * void *sbrk(intptr_t inc)
 */
#[inline]
pub unsafe fn _sys_brk(addr: *mut c_void) -> *mut c_void {
    __nolibc_syscall1(__NR_brk, addr) as c_ulong as *mut c_void
}

#[inline]
pub unsafe fn brk(addr: *mut c_void) -> c_int {
    let ret = _sys_brk(addr);

    if ret.is_null() {
        SET_ERRNO(ENOMEM);
        return -1;
    }
    0
}

#[inline]
pub unsafe fn sbrk(inc: intptr_t) -> *mut c_void {
    /* first call to find current end */
    let ret = _sys_brk(core::ptr::null_mut());
    let wanted = (ret as *mut u8).offset(inc as isize) as *mut c_void;

    if !ret.is_null() && _sys_brk(wanted) == wanted {
        return wanted;
    }

    SET_ERRNO(ENOMEM);
    (-1isize) as *mut c_void
}

/*
 * int chdir(const char *path);
 * int fchdir(int fildes);
 */
#[inline]
pub unsafe fn _sys_chdir(path: *const c_char) -> c_int {
    __nolibc_syscall1(__NR_chdir, path) as c_int
}

#[inline]
pub unsafe fn chdir(path: *const c_char) -> c_int {
    __sysret(_sys_chdir(path))
}

#[inline]
pub unsafe fn _sys_fchdir(fildes: c_int) -> c_int {
    __nolibc_syscall1(__NR_fchdir, fildes) as c_int
}

#[inline]
pub unsafe fn fchdir(fildes: c_int) -> c_int {
    __sysret(_sys_fchdir(fildes))
}

/*
 * int chmod(const char *path, mode_t mode);
 */
#[inline]
pub unsafe fn _sys_chmod(path: *const c_char, mode: mode_t) -> c_int {
    /* if defined(__NR_fchmodat) */
    __nolibc_syscall4(__NR_fchmodat, AT_FDCWD, path, mode, 0) as c_int
    /* else: __nolibc_syscall2(__NR_chmod, path, mode) */
}

#[inline]
pub unsafe fn chmod(path: *const c_char, mode: mode_t) -> c_int {
    __sysret(_sys_chmod(path, mode))
}

/*
 * int chown(const char *path, uid_t owner, gid_t group);
 */
#[inline]
pub unsafe fn _sys_chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    /* if defined(__NR_fchownat) */
    __nolibc_syscall5(__NR_fchownat, AT_FDCWD, path, owner, group, 0) as c_int
    /* else: __nolibc_syscall3(__NR_chown, path, owner, group) */
}

#[inline]
pub unsafe fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    __sysret(_sys_chown(path, owner, group))
}

/*
 * int chroot(const char *path);
 */
#[inline]
pub unsafe fn _sys_chroot(path: *const c_char) -> c_int {
    __nolibc_syscall1(__NR_chroot, path) as c_int
}

#[inline]
pub unsafe fn chroot(path: *const c_char) -> c_int {
    __sysret(_sys_chroot(path))
}

/*
 * int close(int fd);
 */
#[inline]
pub unsafe fn _sys_close(fd: c_int) -> c_int {
    __nolibc_syscall1(__NR_close, fd) as c_int
}

#[inline]
pub unsafe fn close(fd: c_int) -> c_int {
    __sysret(_sys_close(fd))
}

/*
 * int dup(int fd);
 */
#[inline]
pub unsafe fn _sys_dup(fd: c_int) -> c_int {
    __nolibc_syscall1(__NR_dup, fd) as c_int
}

#[inline]
pub unsafe fn dup(fd: c_int) -> c_int {
    __sysret(_sys_dup(fd))
}

/*
 * int dup2(int old, int new);
 */
#[inline]
pub unsafe fn _sys_dup2(old: c_int, new: c_int) -> c_int {
    /* if defined(__NR_dup3) */
    let ret: c_int;
    let nr_fcntl: c_int;

    /* ifdef __NR_fcntl64 */
    nr_fcntl = __NR_fcntl64;
    /* else: nr_fcntl = __NR_fcntl; */

    if old == new {
        ret = __nolibc_syscall2(nr_fcntl, old, F_GETFD) as c_int;
        return if ret < 0 { ret } else { old };
    }

    __nolibc_syscall3(__NR_dup3, old, new, 0) as c_int
    /* else: __nolibc_syscall2(__NR_dup2, old, new) */
}

#[inline]
pub unsafe fn dup2(old: c_int, new: c_int) -> c_int {
    __sysret(_sys_dup2(old, new))
}

/*
 * int dup3(int old, int new, int flags);
 */
/* if defined(__NR_dup3) */
#[inline]
pub unsafe fn _sys_dup3(old: c_int, new: c_int, flags: c_int) -> c_int {
    __nolibc_syscall3(__NR_dup3, old, new, flags) as c_int
}

#[inline]
pub unsafe fn dup3(old: c_int, new: c_int, flags: c_int) -> c_int {
    __sysret(_sys_dup3(old, new, flags))
}

/*
 * int execve(const char *filename, char *const argv[], char *const envp[]);
 */
#[inline]
pub unsafe fn _sys_execve(
    filename: *const c_char,
    argv: *const *mut c_char,
    envp: *const *mut c_char,
) -> c_int {
    __nolibc_syscall3(__NR_execve, filename, argv, envp) as c_int
}

#[inline]
pub unsafe fn execve(
    filename: *const c_char,
    argv: *const *mut c_char,
    envp: *const *mut c_char,
) -> c_int {
    __sysret(_sys_execve(filename, argv, envp))
}

/*
 * void exit(int status);
 */
#[inline]
pub unsafe fn _sys_exit(status: c_int) -> ! {
    __nolibc_syscall1(__NR_exit, status & 255);
    loop {}
}

#[inline]
pub unsafe fn _exit(status: c_int) -> ! {
    _sys_exit(status)
}

#[inline]
pub unsafe fn exit(status: c_int) -> ! {
    _exit(status)
}

/*
 * pid_t fork(void);
 */
/* ifndef _sys_fork */
#[inline]
pub unsafe fn _sys_fork() -> pid_t {
    /* if defined(__NR_clone) */
    /*
     * note: some archs only have clone() and not fork(). Different archs
     * have a different API, but most archs have the flags on first arg and
     * will not use the rest with no other flag.
     */
    __nolibc_syscall5(__NR_clone, SIGCHLD, 0, 0, 0, 0) as pid_t
    /* else: __nolibc_syscall0(__NR_fork) */
}

#[inline]
pub unsafe fn fork() -> pid_t {
    __sysret(_sys_fork())
}

/* ifndef _sys_vfork */
#[inline]
pub unsafe fn _sys_vfork() -> pid_t {
    /* if defined(__NR_clone) */
    /* See the note in _sys_fork(). */
    __nolibc_syscall5(__NR_clone, CLONE_VM | CLONE_VFORK | SIGCHLD, 0, 0, 0, 0) as pid_t
    /* elif defined(__NR_vfork): __nolibc_syscall0(__NR_vfork) */
}

#[inline]
pub unsafe fn vfork() -> pid_t {
    __sysret(_sys_vfork())
}

/*
 * int fsync(int fd);
 */
#[inline]
pub unsafe fn _sys_fsync(fd: c_int) -> c_int {
    __nolibc_syscall1(__NR_fsync, fd) as c_int
}

#[inline]
pub unsafe fn fsync(fd: c_int) -> c_int {
    __sysret(_sys_fsync(fd))
}

/*
 * int getdents64(int fd, struct linux_dirent64 *dirp, int count);
 */
#[inline]
pub unsafe fn _sys_getdents64(fd: c_int, dirp: *mut linux_dirent64, count: c_int) -> c_int {
    __nolibc_syscall3(__NR_getdents64, fd, dirp, count) as c_int
}

#[inline]
pub unsafe fn getdents64(fd: c_int, dirp: *mut linux_dirent64, count: c_int) -> c_int {
    __sysret(_sys_getdents64(fd, dirp, count))
}

/*
 * uid_t geteuid(void);
 */
#[inline]
pub unsafe fn _sys_geteuid() -> uid_t {
    /* if defined(__NR_geteuid32) */
    __nolibc_syscall0(__NR_geteuid32) as uid_t
    /* else: __nolibc_syscall0(__NR_geteuid) */
}

#[inline]
pub unsafe fn geteuid() -> uid_t {
    _sys_geteuid()
}

/*
 * pid_t getpgid(pid_t pid);
 */
#[inline]
pub unsafe fn _sys_getpgid(pid: pid_t) -> pid_t {
    __nolibc_syscall1(__NR_getpgid, pid) as pid_t
}

#[inline]
pub unsafe fn getpgid(pid: pid_t) -> pid_t {
    __sysret(_sys_getpgid(pid))
}

/*
 * pid_t getpgrp(void);
 */
#[inline]
pub unsafe fn _sys_getpgrp() -> pid_t {
    _sys_getpgid(0)
}

#[inline]
pub unsafe fn getpgrp() -> pid_t {
    _sys_getpgrp()
}

/*
 * pid_t getpid(void);
 */
#[inline]
pub unsafe fn _sys_getpid() -> pid_t {
    __nolibc_syscall0(__NR_getpid) as pid_t
}

#[inline]
pub unsafe fn getpid() -> pid_t {
    _sys_getpid()
}

/*
 * pid_t getppid(void);
 */
#[inline]
pub unsafe fn _sys_getppid() -> pid_t {
    __nolibc_syscall0(__NR_getppid) as pid_t
}

#[inline]
pub unsafe fn getppid() -> pid_t {
    _sys_getppid()
}

/*
 * pid_t gettid(void);
 */
#[inline]
pub unsafe fn _sys_gettid() -> pid_t {
    __nolibc_syscall0(__NR_gettid) as pid_t
}

#[inline]
pub unsafe fn gettid() -> pid_t {
    _sys_gettid()
}

/* ifndef NOLIBC_NO_RUNTIME */
unsafe extern "C" {
    fn getauxval(key: c_ulong) -> c_ulong;
}

/*
 * int getpagesize(void);
 */
#[inline]
pub unsafe fn getpagesize() -> c_int {
    let val = getauxval(AT_PAGESZ) as c_int;
    __sysret(if val != 0 { val } else { -ENOENT })
}
/* endif NOLIBC_NO_RUNTIME */

/*
 * uid_t getuid(void);
 */
#[inline]
pub unsafe fn _sys_getuid() -> uid_t {
    /* if defined(__NR_getuid32) */
    __nolibc_syscall0(__NR_getuid32) as uid_t
    /* else: __nolibc_syscall0(__NR_getuid) */
}

#[inline]
pub unsafe fn getuid() -> uid_t {
    _sys_getuid()
}

/*
 * int kill(pid_t pid, int signal);
 */
#[inline]
pub unsafe fn _sys_kill(pid: pid_t, signal: c_int) -> c_int {
    __nolibc_syscall2(__NR_kill, pid, signal) as c_int
}

#[inline]
pub unsafe fn kill(pid: pid_t, signal: c_int) -> c_int {
    __sysret(_sys_kill(pid, signal))
}

/*
 * int link(const char *old, const char *new);
 */
#[inline]
pub unsafe fn _sys_link(old: *const c_char, new: *const c_char) -> c_int {
    /* if defined(__NR_linkat) */
    __nolibc_syscall5(__NR_linkat, AT_FDCWD, old, AT_FDCWD, new, 0) as c_int
    /* else: __nolibc_syscall2(__NR_link, old, new) */
}

#[inline]
pub unsafe fn link(old: *const c_char, new: *const c_char) -> c_int {
    __sysret(_sys_link(old, new))
}

/*
 * off_t lseek(int fd, off_t offset, int whence);
 */
#[inline]
pub unsafe fn _sys_lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    /* if defined(__NR_llseek) || defined(__NR__llseek) */
    let mut loff: __kernel_loff_t = 0;
    let ret: c_int;
    let nr_llseek: c_int;
    let result: off_t;

    /* if defined(__NR_llseek) */
    nr_llseek = __NR_llseek;
    /* else: nr_llseek = __NR__llseek; */

    ret = __nolibc_syscall5(
        nr_llseek,
        fd,
        offset >> 32,
        offset as uint32_t,
        &mut loff as *mut __kernel_loff_t,
        whence,
    ) as c_int;
    if ret < 0 {
        result = ret as off_t;
    } else {
        result = loff as off_t;
    }

    result
    /* else: __nolibc_syscall3(__NR_lseek, fd, offset, whence) */
}

#[inline]
pub unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    __sysret(_sys_lseek(fd, offset, whence))
}

/*
 * int mkdir(const char *path, mode_t mode);
 */
#[inline]
pub unsafe fn _sys_mkdir(path: *const c_char, mode: mode_t) -> c_int {
    /* if defined(__NR_mkdirat) */
    __nolibc_syscall3(__NR_mkdirat, AT_FDCWD, path, mode) as c_int
    /* else: __nolibc_syscall2(__NR_mkdir, path, mode) */
}

#[inline]
pub unsafe fn mkdir(path: *const c_char, mode: mode_t) -> c_int {
    __sysret(_sys_mkdir(path, mode))
}

/*
 * int rmdir(const char *path);
 */
#[inline]
pub unsafe fn _sys_rmdir(path: *const c_char) -> c_int {
    /* if defined(__NR_rmdir) */
    __nolibc_syscall1(__NR_rmdir, path) as c_int
    /* else: __nolibc_syscall3(__NR_unlinkat, AT_FDCWD, path, AT_REMOVEDIR) */
}

#[inline]
pub unsafe fn rmdir(path: *const c_char) -> c_int {
    __sysret(_sys_rmdir(path))
}

/*
 * int mknod(const char *path, mode_t mode, dev_t dev);
 */
#[inline]
pub unsafe fn _sys_mknod(path: *const c_char, mode: mode_t, dev: dev_t) -> c_long {
    /* if defined(__NR_mknodat) */
    __nolibc_syscall4(__NR_mknodat, AT_FDCWD, path, mode, dev) as c_long
    /* else: __nolibc_syscall3(__NR_mknod, path, mode, dev) */
}

#[inline]
pub unsafe fn mknod(path: *const c_char, mode: mode_t, dev: dev_t) -> c_int {
    __sysret(_sys_mknod(path, mode, dev)) as c_int
}

/*
 * int pipe2(int pipefd[2], int flags);
 * int pipe(int pipefd[2]);
 */
#[inline]
pub unsafe fn _sys_pipe2(pipefd: *mut c_int, flags: c_int) -> c_int {
    __nolibc_syscall2(__NR_pipe2, pipefd, flags) as c_int
}

#[inline]
pub unsafe fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int {
    __sysret(_sys_pipe2(pipefd, flags))
}

#[inline]
pub unsafe fn pipe(pipefd: *mut c_int) -> c_int {
    pipe2(pipefd, 0)
}

/*
 * int pivot_root(const char *new, const char *old);
 */
#[inline]
pub unsafe fn _sys_pivot_root(new: *const c_char, old: *const c_char) -> c_int {
    __nolibc_syscall2(__NR_pivot_root, new, old) as c_int
}

#[inline]
pub unsafe fn pivot_root(new: *const c_char, old: *const c_char) -> c_int {
    __sysret(_sys_pivot_root(new, old))
}

/*
 * ssize_t read(int fd, void *buf, size_t count);
 */
#[inline]
pub unsafe fn _sys_read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    __nolibc_syscall3(__NR_read, fd, buf, count) as ssize_t
}

#[inline]
pub unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    __sysret(_sys_read(fd, buf, count))
}

/*
 * int sched_yield(void);
 */
#[inline]
pub unsafe fn _sys_sched_yield() -> c_int {
    __nolibc_syscall0(__NR_sched_yield) as c_int
}

#[inline]
pub unsafe fn sched_yield() -> c_int {
    __sysret(_sys_sched_yield())
}

/*
 * int setpgid(pid_t pid, pid_t pgid);
 */
#[inline]
pub unsafe fn _sys_setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    __nolibc_syscall2(__NR_setpgid, pid, pgid) as c_int
}

#[inline]
pub unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    __sysret(_sys_setpgid(pid, pgid))
}

/*
 * pid_t setpgrp(void)
 */
#[inline]
pub unsafe fn setpgrp() -> pid_t {
    setpgid(0, 0) as pid_t
}

/*
 * pid_t setsid(void);
 */
#[inline]
pub unsafe fn _sys_setsid() -> pid_t {
    __nolibc_syscall0(__NR_setsid) as pid_t
}

#[inline]
pub unsafe fn setsid() -> pid_t {
    __sysret(_sys_setsid())
}

/*
 * int symlink(const char *old, const char *new);
 */
#[inline]
pub unsafe fn _sys_symlink(old: *const c_char, new: *const c_char) -> c_int {
    /* if defined(__NR_symlinkat) */
    __nolibc_syscall3(__NR_symlinkat, old, AT_FDCWD, new) as c_int
    /* else: __nolibc_syscall2(__NR_symlink, old, new) */
}

#[inline]
pub unsafe fn symlink(old: *const c_char, new: *const c_char) -> c_int {
    __sysret(_sys_symlink(old, new))
}

/*
 * mode_t umask(mode_t mode);
 */
#[inline]
pub unsafe fn _sys_umask(mode: mode_t) -> mode_t {
    __nolibc_syscall1(__NR_umask, mode) as mode_t
}

#[inline]
pub unsafe fn umask(mode: mode_t) -> mode_t {
    _sys_umask(mode)
}

/*
 * int umount2(const char *path, int flags);
 */
#[inline]
pub unsafe fn _sys_umount2(path: *const c_char, flags: c_int) -> c_int {
    __nolibc_syscall2(__NR_umount2, path, flags) as c_int
}

#[inline]
pub unsafe fn umount2(path: *const c_char, flags: c_int) -> c_int {
    __sysret(_sys_umount2(path, flags))
}

/*
 * int unlink(const char *path);
 */
#[inline]
pub unsafe fn _sys_unlink(path: *const c_char) -> c_int {
    /* if defined(__NR_unlinkat) */
    __nolibc_syscall3(__NR_unlinkat, AT_FDCWD, path, 0) as c_int
    /* else: __nolibc_syscall1(__NR_unlink, path) */
}

#[inline]
pub unsafe fn unlink(path: *const c_char) -> c_int {
    __sysret(_sys_unlink(path))
}

/*
 * ssize_t write(int fd, const void *buf, size_t count);
 */
#[inline]
pub unsafe fn _sys_write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    __nolibc_syscall3(__NR_write, fd, buf, count) as ssize_t
}

#[inline]
pub unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    __sysret(_sys_write(fd, buf, count))
}

/*
 * int memfd_create(const char *name, unsigned int flags);
 */
#[inline]
pub unsafe fn _sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int {
    __nolibc_syscall2(__NR_memfd_create, name, flags) as c_int
}

#[inline]
pub unsafe fn memfd_create(name: *const c_char, flags: c_uint) -> c_int {
    __sysret(_sys_memfd_create(name, flags))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
