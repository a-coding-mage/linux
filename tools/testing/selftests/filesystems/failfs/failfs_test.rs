// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included Linux/glibc headers plus
// "../../kselftest_harness.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const __NR_fchroot: c_long = 472;

const FD_PIDFS_ROOT: c_int = -10002;
const FD_NSFS_ROOT: c_int = -10003;
const FD_FAILFS_ROOT: c_int = -10004;

const NOBODY_UID: libc::uid_t = 65534;

/* Child sentinel exit code: the exec was blocked as expected. */
const FAILFS_EXEC_BLOCKED: c_int = 99;

/* Stack for the CLONE_FS helper in fchroot_sentinel_shared_fs_struct. */
const FAILFS_CLONE_STACK: usize = 64 * 1024;

const EI_NIDENT: usize = 16;
const ELFMAG: &[u8; 4] = b"\x7fELF";
const SELFMAG: usize = 4;
const PT_INTERP: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
struct Elf64_Ehdr {
    e_ident: [u8; EI_NIDENT],
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
#[derive(Copy, Clone)]
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
struct failfs_file_handle {
    handle: libc::file_handle,
    f_handle: [u8; libc::MAX_HANDLE_SZ as usize],
}

unsafe fn errno_value() -> c_int {
    *libc::__errno_location()
}

unsafe fn sys_fchroot(fd: c_int, flags: c_uint) -> c_int {
    libc::syscall(__NR_fchroot, fd, flags) as c_int
}

/*
 * Raw syscall: glibc's getcwd() rejects the kernel's "(unreachable)"
 * result and falls back to a generic implementation.
 */
unsafe fn sys_getcwd(buf: *mut c_char, size: libc::size_t) -> c_long {
    libc::syscall(libc::SYS_getcwd, buf, size) as c_long
}

unsafe fn drop_to_nobody() -> c_int {
    libc::setresuid(NOBODY_UID, NOBODY_UID, NOBODY_UID)
}

/* Parked CLONE_FS child; dies with its parent so it never leaks. */
unsafe extern "C" fn failfs_park(arg: *mut c_void) -> c_int {
    let parent: libc::pid_t = arg as c_long as libc::pid_t;

    libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL);
    /* The parent may have died before the death signal was armed. */
    if libc::getppid() != parent {
        libc::_exit(0);
    }
    libc::pause();
    0
}

/* Is fd a dynamically linked ELF with an absolute PT_INTERP interpreter? */
unsafe fn elf_has_absolute_interp(fd: c_int) -> c_int {
    let mut ehdr: Elf64_Ehdr = zeroed();
    let mut phdr: Elf64_Phdr = zeroed();
    let mut interp: c_char = 0;
    let mut i: c_int;

    if libc::pread(
        fd,
        &mut ehdr as *mut _ as *mut c_void,
        size_of::<Elf64_Ehdr>(),
        0,
    ) != size_of::<Elf64_Ehdr>() as isize
    {
        return 0;
    }
    if libc::memcmp(
        ehdr.e_ident.as_ptr() as *const c_void,
        ELFMAG.as_ptr() as *const c_void,
        SELFMAG,
    ) != 0
    {
        return 0;
    }

    i = 0;
    while i < ehdr.e_phnum as c_int {
        if libc::pread(
            fd,
            &mut phdr as *mut _ as *mut c_void,
            size_of::<Elf64_Phdr>(),
            (ehdr.e_phoff + i as u64 * size_of::<Elf64_Phdr>() as u64) as libc::off_t,
        ) != size_of::<Elf64_Phdr>() as isize
        {
            return 0;
        }
        if phdr.p_type != PT_INTERP {
            i += 1;
            continue;
        }
        if libc::pread(
            fd,
            &mut interp as *mut _ as *mut c_void,
            1,
            phdr.p_offset as libc::off_t,
        ) != 1
        {
            return 0;
        }
        return (interp == b'/' as c_char) as c_int;
    }

    0
}

test!(fchdir_sentinel, {
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];
    let mut fd: c_int;

    assert_eq!(unsafe { libc::fchdir(FD_FAILFS_ROOT) }, 0);

    /* The working directory is unreachable from the process root. */
    assert_gt!(unsafe { sys_getcwd(buf.as_mut_ptr(), buf.len()) }, 0);
    assert_eq!(unsafe { libc::strncmp(buf.as_ptr(), c"(unreachable)".as_ptr(), 13) }, 0);

    /* Every AT_FDCWD-relative lookup fails. */
    assert_eq!(unsafe { libc::openat(libc::AT_FDCWD, c"foo".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::openat(libc::AT_FDCWD, c".".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::openat(libc::AT_FDCWD, c"..".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::openat(libc::AT_FDCWD, c"foo".as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o600) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* The cwd cannot be pinned by following /proc/self/cwd into it. */
    assert_eq!(unsafe { libc::open(c"/proc/self/cwd".as_ptr(), libc::O_PATH) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* The root is untouched so absolute lookups keep working... */
    fd = unsafe { libc::open(c"/".as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);

    /* ... and the working directory can be recovered. */
    assert_eq!(unsafe { libc::chdir(c"/".as_ptr()) }, 0);
    assert_gt!(unsafe { sys_getcwd(buf.as_mut_ptr(), buf.len()) }, 0);
    assert_eq!(unsafe { libc::strcmp(buf.as_ptr(), c"/".as_ptr()) }, 0);
});

test!(fchdir_rejects_other_sentinels, {
    assert_eq!(unsafe { libc::fchdir(FD_PIDFS_ROOT) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);
    assert_eq!(unsafe { libc::fchdir(FD_NSFS_ROOT) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);
    assert_eq!(unsafe { libc::fchdir(-10009) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);
});

test!(fchroot_flags, {
    let fd: c_int;

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 1) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EINVAL);

    fd = unsafe { libc::open(c"/".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { sys_fchroot(fd, 1) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EINVAL);
    assert_eq!(unsafe { libc::close(fd) }, 0);
});

test!(fchroot_bad_fd, {
    assert_eq!(unsafe { sys_fchroot(-1, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);

    /* Only FD_FAILFS_ROOT is a valid sentinel. */
    assert_eq!(unsafe { sys_fchroot(FD_PIDFS_ROOT, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);
    assert_eq!(unsafe { sys_fchroot(FD_NSFS_ROOT, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EBADF);
});

test!(fchroot_notdir, {
    let fd: c_int;

    fd = unsafe { libc::open(c"/proc/self/status".as_ptr(), libc::O_RDONLY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { sys_fchroot(fd, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::ENOTDIR);
    assert_eq!(unsafe { libc::close(fd) }, 0);
});

test!(fchroot_realfd_requires_cap, {
    let fd: c_int;

    if unsafe { libc::geteuid() } == 0 {
        assert_eq!(unsafe { drop_to_nobody() }, 0);
    }

    fd = unsafe { libc::open(c"/".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { sys_fchroot(fd, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EPERM);
    assert_eq!(unsafe { libc::close(fd) }, 0);
});

test!(fchroot_realfd, {
    let mut template = *b"/tmp/failfs_test.XXXXXX\0";
    let mut path = [0 as c_char; libc::PATH_MAX as usize];
    let mut st: libc::stat = unsafe { zeroed() };
    let tmpfd: c_int;
    let dfd: c_int;
    let fd: c_int;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "fchroot() with a regular fd requires CAP_SYS_CHROOT");
    }

    tmpfd = unsafe { libc::open(c"/tmp".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(tmpfd, 0);

    assert_ne!(unsafe { libc::mkdtemp(template.as_mut_ptr() as *mut c_char) }, ptr::null_mut());
    unsafe { libc::snprintf(path.as_mut_ptr(), path.len(), c"%s/canary".as_ptr(), template.as_ptr() as *const c_char) };
    fd = unsafe { libc::open(path.as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o600) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);

    dfd = unsafe { libc::open(template.as_ptr() as *const c_char, libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(dfd, 0);
    assert_eq!(unsafe { sys_fchroot(dfd, 0) }, 0);
    assert_eq!(unsafe { libc::close(dfd) }, 0);

    assert_eq!(unsafe { libc::stat(c"/canary".as_ptr(), &mut st) }, 0);

    /* Best-effort cleanup: dirfd-anchored I/O works with the new root. */
    unsafe { libc::snprintf(path.as_mut_ptr(), path.len(), c"%s/canary".as_ptr(), template.as_ptr().add(c"/tmp/".to_bytes().len()) as *const c_char) };
    unsafe { libc::unlinkat(tmpfd, path.as_ptr(), 0) };
    unsafe { libc::unlinkat(tmpfd, template.as_ptr().add(c"/tmp/".to_bytes().len()) as *const c_char, libc::AT_REMOVEDIR) };
});

test!(fchroot_sentinel, {
    let mut template = *b"/tmp/failfs_test.XXXXXX\0";
    let mut realroot: libc::stat = unsafe { zeroed() };
    let mut st: libc::stat = unsafe { zeroed() };
    let mut sfs: libc::statfs = unsafe { zeroed() };
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];
    let procfd: c_int;
    let tmpfd: c_int;
    let dfd: c_int;
    let mut fd: c_int;
    let mut fh: failfs_file_handle = unsafe { zeroed() };
    let mut mntid: c_int = 0;
    let ret: isize;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    }

    assert_eq!(unsafe { libc::stat(c"/".as_ptr(), &mut realroot) }, 0);
    procfd = unsafe { libc::open(c"/proc".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(procfd, 0);
    tmpfd = unsafe { libc::open(c"/tmp".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(tmpfd, 0);
    assert_ne!(unsafe { libc::mkdtemp(template.as_mut_ptr() as *mut c_char) }, ptr::null_mut());
    dfd = unsafe { libc::open(template.as_ptr() as *const c_char, libc::O_RDONLY | libc::O_DIRECTORY) };
    assert_ge!(dfd, 0);

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    /* Absolute lookups fail. */
    assert_eq!(unsafe { libc::open(c"/etc/passwd".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::mkdir(c"/foo".as_ptr(), 0o700) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /*
     * The root cannot be referenced at all - not even an O_PATH open,
     * which skips ->permission(), because it lands on the root as a
     * jumped walk terminal that ->d_weak_revalidate() refuses.
     */
    assert_eq!(unsafe { libc::open(c"/".as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::open(c"/".as_ptr(), libc::O_PATH) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
    assert_eq!(unsafe { libc::statfs(c"/".as_ptr(), &mut sfs) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /*
     * It cannot be pinned by following /proc/self/root into it either
     * (only the root is in failfs here, so self/cwd is still real).
     */
    assert_eq!(unsafe { libc::openat(procfd, c"self/root".as_ptr(), libc::O_PATH) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* Nor encoded into a file handle. */
    fh.handle.handle_bytes = libc::MAX_HANDLE_SZ as u32;
    assert_eq!(unsafe { libc::name_to_handle_at(libc::AT_FDCWD, c"/".as_ptr(), &mut fh.handle, &mut mntid, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* The working directory is now unreachable from the root. */
    assert_gt!(unsafe { sys_getcwd(buf.as_mut_ptr(), buf.len()) }, 0);
    assert_eq!(unsafe { libc::strncmp(buf.as_ptr(), c"(unreachable)".as_ptr(), 13) }, 0);

    /* Lookups anchored at real directories keep working. */
    fd = unsafe { libc::openat(libc::AT_FDCWD, c".".as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);
    fd = unsafe { libc::openat(dfd, c"canary".as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o600) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::write(fd, c"x".as_ptr() as *const c_void, 1) }, 1);
    assert_eq!(unsafe { libc::close(fd) }, 0);
    fd = unsafe { libc::openat(dfd, c"canary".as_ptr(), libc::O_RDONLY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);

    /* ".." walks clamp at the top of the mount tree, not at failfs. */
    fd = unsafe { libc::openat(libc::AT_FDCWD, c"../../../../../../../../../..".as_ptr(), libc::O_PATH) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::fstat(fd, &mut st) }, 0);
    assert_eq!(st.st_dev, realroot.st_dev);
    assert_eq!(st.st_ino, realroot.st_ino);
    assert_eq!(unsafe { libc::close(fd) }, 0);

    /* readlink of the magic link still works: it does not follow. */
    ret = unsafe { libc::readlinkat(procfd, c"self/root".as_ptr(), buf.as_mut_ptr(), buf.len() - 1) };
    assert_gt!(ret, 0);
    buf[ret as usize] = 0;
    th_log!("/proc/self/root points to '%s'", buf.as_ptr());
    /* d_path() names the failfs root synthetically, never as a real path. */
    assert_eq!(unsafe { libc::strcmp(buf.as_ptr(), c"failfs:/".as_ptr()) }, 0);

    /* But following it into failfs is refused. */
    assert_eq!(unsafe { libc::fstatat(procfd, c"self/root".as_ptr(), &mut st, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* Best-effort cleanup via the pre-opened dirfds. */
    unsafe { libc::unlinkat(dfd, c"canary".as_ptr(), 0) };
    unsafe { libc::unlinkat(tmpfd, template.as_ptr().add(c"/tmp/".to_bytes().len()) as *const c_char, libc::AT_REMOVEDIR) };
});

test!(fchroot_sentinel_absolute_symlink, {
    let mut template = *b"/tmp/failfs_test.XXXXXX\0";
    let tmpfd: c_int;
    let dfd: c_int;
    let fd: c_int;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    }

    tmpfd = unsafe { libc::open(c"/tmp".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(tmpfd, 0);
    assert_ne!(unsafe { libc::mkdtemp(template.as_mut_ptr() as *mut c_char) }, ptr::null_mut());
    dfd = unsafe { libc::open(template.as_ptr() as *const c_char, libc::O_RDONLY | libc::O_DIRECTORY) };
    assert_ge!(dfd, 0);

    fd = unsafe { libc::openat(dfd, c"target".as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o600) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);
    assert_eq!(unsafe { libc::symlinkat(c"target".as_ptr(), dfd, c"rel".as_ptr()) }, 0);
    assert_eq!(unsafe { libc::symlinkat(c"/etc".as_ptr(), dfd, c"abs".as_ptr()) }, 0);

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    /* Relative symlinks keep resolving within the dirfd-anchored walk... */
    fd = unsafe { libc::openat(dfd, c"rel".as_ptr(), libc::O_RDONLY) };
    assert_ge!(fd, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);

    /* ... absolute symlinks restart the walk at the failfs root. */
    assert_eq!(unsafe { libc::openat(dfd, c"abs".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* Best-effort cleanup via the pre-opened dirfds. */
    unsafe { libc::unlinkat(dfd, c"abs".as_ptr(), 0) };
    unsafe { libc::unlinkat(dfd, c"rel".as_ptr(), 0) };
    unsafe { libc::unlinkat(dfd, c"target".as_ptr(), 0) };
    unsafe { libc::unlinkat(tmpfd, template.as_ptr().add(c"/tmp/".to_bytes().len()) as *const c_char, libc::AT_REMOVEDIR) };
});

test!(fchroot_sentinel_unprivileged, {
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];

    if unsafe { libc::geteuid() } == 0 {
        assert_eq!(unsafe { drop_to_nobody() }, 0);
    }

    /* Without no_new_privs entering failfs is not allowed... */
    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EPERM);

    /* ... with no_new_privs set it is allowed. */
    assert_eq!(unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) }, 0);
    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    assert_eq!(unsafe { libc::open(c"/etc/passwd".as_ptr(), libc::O_RDONLY) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* The task counts as chrooted: no user namespaces anymore. */
    assert_eq!(unsafe { libc::unshare(libc::CLONE_NEWUSER) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EPERM);

    /* With both root and cwd in failfs getcwd() reports "/". */
    assert_eq!(unsafe { libc::fchdir(FD_FAILFS_ROOT) }, 0);
    assert_gt!(unsafe { sys_getcwd(buf.as_mut_ptr(), buf.len()) }, 0);
    assert_eq!(unsafe { libc::strcmp(buf.as_ptr(), c"/".as_ptr()) }, 0);
});

test!(fchroot_sentinel_rejected_when_chrooted, {
    let mut template = *b"/tmp/failfs_test.XXXXXX\0";
    let tmpfd: c_int;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "chroot() requires CAP_SYS_CHROOT");
    }

    tmpfd = unsafe { libc::open(c"/tmp".as_ptr(), libc::O_PATH | libc::O_DIRECTORY) };
    assert_ge!(tmpfd, 0);
    assert_ne!(unsafe { libc::mkdtemp(template.as_mut_ptr() as *mut c_char) }, ptr::null_mut());
    assert_eq!(unsafe { libc::chroot(template.as_ptr() as *const c_char) }, 0);
    assert_eq!(unsafe { libc::chdir(c"/".as_ptr()) }, 0);

    /* Remove the jail while still privileged; sticky /tmp blocks nobody. */
    unsafe { libc::unlinkat(tmpfd, template.as_ptr().add(c"/tmp/".to_bytes().len()) as *const c_char, libc::AT_REMOVEDIR) };

    assert_eq!(unsafe { drop_to_nobody() }, 0);
    assert_eq!(unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) }, 0);

    /* An unprivileged chrooted task must not lift its ".." barrier. */
    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EPERM);
});

test!(fchroot_sentinel_shared_fs_struct, {
    let mut stack = [0 as c_char; FAILFS_CLONE_STACK];
    let pid: libc::pid_t;

    if unsafe { libc::geteuid() } == 0 {
        assert_eq!(unsafe { drop_to_nobody() }, 0);
    }

    /* A CLONE_FS sibling shares the fs_struct: bump fs->users to 2. */
    pid = unsafe {
        libc::clone(
            failfs_park,
            stack.as_mut_ptr().add(stack.len()) as *mut c_void,
            libc::CLONE_FS | libc::SIGCHLD,
            libc::getpid() as c_long as *mut c_void,
        )
    };
    assert_ge!(pid, 0);

    assert_eq!(unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) }, 0);

    /*
     * A sibling without no_new_privs could exec a setuid binary with
     * the failfs root, so a shared fs_struct is refused even with
     * no_new_privs set.
     */
    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EINVAL);

    assert_eq!(unsafe { libc::kill(pid, libc::SIGKILL) }, 0);
    assert_eq!(unsafe { libc::waitpid(pid, ptr::null_mut(), 0) }, pid);
});

test!(fchroot_sentinel_no_overmount, {
    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "mounting requires privileges");
    }

    /*
     * Contain the blast radius: if failfs ever regressed and "/"
     * resolved to the real root, the tmpfs mount below must not touch
     * the host. A private mount namespace keeps it local to this child.
     */
    assert_eq!(unsafe { libc::unshare(libc::CLONE_NEWNS) }, 0);
    assert_eq!(unsafe { libc::mount(ptr::null(), c"/".as_ptr(), ptr::null(), libc::MS_REC | libc::MS_PRIVATE, ptr::null()) }, 0);

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    /*
     * Nothing can be mounted on top of the failfs root. It cannot even
     * be named as a mount target: resolving "/" is refused before the
     * mount machinery (which, failfs being in no mount namespace, would
     * reject it anyway) is ever reached. open_tree(OPEN_TREE_CLONE) is
     * likewise moot since no fd to the root can be obtained.
     */
    assert_eq!(unsafe { libc::mount(c"none".as_ptr(), c"/".as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);
});

test!(fchroot_sentinel_setns_escape, {
    let mut realroot: libc::stat = unsafe { zeroed() };
    let mut st: libc::stat = unsafe { zeroed() };
    let nsfd: c_int;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "setns() to a mount namespace requires privileges");
    }

    assert_eq!(unsafe { libc::stat(c"/".as_ptr(), &mut realroot) }, 0);
    nsfd = unsafe { libc::open(c"/proc/self/ns/mnt".as_ptr(), libc::O_RDONLY) };
    assert_ge!(nsfd, 0);

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);
    assert_eq!(unsafe { libc::open(c"/etc".as_ptr(), libc::O_PATH) }, -1);
    assert_eq!(unsafe { errno_value() }, libc::EOPNOTSUPP);

    /* A mount namespace fd is the key out: it resets root and cwd. */
    assert_eq!(unsafe { libc::setns(nsfd, libc::CLONE_NEWNS) }, 0);
    assert_eq!(unsafe { libc::close(nsfd) }, 0);

    assert_eq!(unsafe { libc::stat(c"/".as_ptr(), &mut st) }, 0);
    assert_eq!(st.st_dev, realroot.st_dev);
    assert_eq!(st.st_ino, realroot.st_ino);
});

test!(fchroot_sentinel_exec, {
    let pid: libc::pid_t;
    let mut status: c_int = 0;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    }

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    /*
     * Exec in a child: a wrongly successful exec would replace the test
     * image and its exit code would not match the sentinel below.
     */
    pid = unsafe { libc::fork() };
    assert_ge!(pid, 0);
    if pid == 0 {
        unsafe {
            libc::execl(c"/bin/true".as_ptr(), c"true".as_ptr(), ptr::null::<c_char>());
            libc::_exit(if errno_value() == libc::EOPNOTSUPP { FAILFS_EXEC_BLOCKED } else { 1 });
        }
    }
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    assert_true!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), FAILFS_EXEC_BLOCKED);
});

test!(fchroot_sentinel_exec_interpreter, {
    static ARGV0: &[u8] = b"failfs_test\0";
    static ARGV: [*const c_char; 2] = [ARGV0.as_ptr() as *const c_char, ptr::null()];
    static ENVP: [*const c_char; 1] = [ptr::null()];
    let pid: libc::pid_t;
    let mut status: c_int = 0;
    let exefd: c_int;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    }

    /* Exec ourselves: the one binary guaranteed to be around. */
    exefd = unsafe { libc::open(c"/proc/self/exe".as_ptr(), libc::O_RDONLY) };
    assert_ge!(exefd, 0);
    if unsafe { elf_has_absolute_interp(exefd) } == 0 {
        skip!(return, "test binary has no absolute PT_INTERP interpreter");
    }

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    /*
     * The binary itself needs no path lookup - it is executed by fd -
     * but loading it fails on opening the absolute PT_INTERP
     * interpreter. Run it in a child so a wrongly successful exec does
     * not replace the test image and masquerade as a pass.
     */
    pid = unsafe { libc::fork() };
    assert_ge!(pid, 0);
    if pid == 0 {
        unsafe {
            libc::syscall(libc::SYS_execveat, exefd, c"".as_ptr(), ARGV.as_ptr(), ENVP.as_ptr(), libc::AT_EMPTY_PATH);
            libc::_exit(if errno_value() == libc::EOPNOTSUPP { FAILFS_EXEC_BLOCKED } else { 1 });
        }
    }
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    assert_true!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), FAILFS_EXEC_BLOCKED);
});

test!(fchroot_sentinel_inherited, {
    let pid: libc::pid_t;
    let mut status: c_int = 0;

    if unsafe { libc::geteuid() } != 0 {
        skip!(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    }

    assert_eq!(unsafe { sys_fchroot(FD_FAILFS_ROOT, 0) }, 0);

    pid = unsafe { libc::fork() };
    assert_ge!(pid, 0);
    if pid == 0 {
        unsafe {
            if libc::open(c"/etc".as_ptr(), libc::O_PATH) != -1 || errno_value() != libc::EOPNOTSUPP {
                libc::_exit(1);
            }
            libc::_exit(0);
        }
    }
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    assert_true!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
});

test_harness_main!();
