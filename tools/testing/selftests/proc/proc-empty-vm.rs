#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
mod proc_empty_vm {
    /*
     * Copyright (c) 2022 Alexey Dobriyan <adobriyan@gmail.com>
     *
     * Permission to use, copy, modify, and distribute this software for any
     * purpose with or without fee is hereby granted, provided that the above
     * copyright notice and this permission notice appear in all copies.
     *
     * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
     * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
     * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
     * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
     * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
     * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
     * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
     */
    /*
     * Create a process without mappings by unmapping everything at once and
     * holding it with ptrace(2). See what happens to
     *
     *	/proc/${pid}/maps
     *	/proc/${pid}/numa_maps
     *	/proc/${pid}/smaps
     *	/proc/${pid}/smaps_rollup
     */

    use core::arch::asm;
    use core::ffi::{c_char, c_int, c_long, c_void};
    use core::ptr;

    #[cfg(target_arch = "x86_64")]
    const SYS_PKEY_ALLOC: c_long = 330;
    #[cfg(target_arch = "x86_64")]
    const SYS_PKEY_FREE: c_long = 331;

    #[cfg(target_arch = "x86")]
    const SYS_PKEY_ALLOC: c_long = 381;
    #[cfg(target_arch = "x86")]
    const SYS_PKEY_FREE: c_long = 382;

    static mut G_PROTECTION_KEY_SUPPORT: c_int = 0;

    fn errno() -> c_int {
        unsafe { *libc::__errno_location() }
    }

    unsafe fn protection_key_support() -> c_int {
        let rv = libc::syscall(SYS_PKEY_ALLOC, 0, 0);
        if rv > 0 {
            libc::syscall(SYS_PKEY_FREE, rv as c_int);
            1
        } else if rv == -1 && errno() == libc::ENOSYS {
            0
        } else if rv == -1 && errno() == libc::EINVAL {
            // ospke=n
            0
        } else {
            libc::fprintf(
                libc::stderr,
                b"%s: error: rv %ld, errno %d\n\0".as_ptr() as *const c_char,
                b"protection_key_support\0".as_ptr() as *const c_char,
                rv,
                errno(),
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    /*
     * 0: vsyscall VMA doesn't exist	vsyscall=none
     * 1: vsyscall VMA is --xp		vsyscall=xonly
     * 2: vsyscall VMA is r-xp		vsyscall=emulate
     */
    static mut G_VSYSCALL: c_int = 0;
    static mut G_PROC_PID_MAPS_VSYSCALL: *const c_char = ptr::null();
    static mut G_PROC_PID_SMAPS_VSYSCALL: *const c_char = ptr::null();

    static PROC_PID_MAPS_VSYSCALL_0: &[u8] = b"\0";
    static PROC_PID_MAPS_VSYSCALL_1: &[u8] =
        b"ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n\0";
    static PROC_PID_MAPS_VSYSCALL_2: &[u8] =
        b"ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n\0";

    static PROC_PID_SMAPS_VSYSCALL_0: &[u8] = b"\0";

    static PROC_PID_SMAPS_VSYSCALL_1: &[u8] =
        b"ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n\
Size:                  4 kB\n\
KernelPageSize:        4 kB\n\
MMUPageSize:           4 kB\n\
Rss:                   0 kB\n\
Pss:                   0 kB\n\
Pss_Dirty:             0 kB\n\
Shared_Clean:          0 kB\n\
Shared_Dirty:          0 kB\n\
Private_Clean:         0 kB\n\
Private_Dirty:         0 kB\n\
Referenced:            0 kB\n\
Anonymous:             0 kB\n\
KSM:                   0 kB\n\
LazyFree:              0 kB\n\
AnonHugePages:         0 kB\n\
ShmemPmdMapped:        0 kB\n\
FilePmdMapped:         0 kB\n\
Shared_Hugetlb:        0 kB\n\
Private_Hugetlb:       0 kB\n\
Swap:                  0 kB\n\
SwapPss:               0 kB\n\
Locked:                0 kB\n\
THPeligible:           0\n\0";

    static PROC_PID_SMAPS_VSYSCALL_2: &[u8] =
        b"ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n\
Size:                  4 kB\n\
KernelPageSize:        4 kB\n\
MMUPageSize:           4 kB\n\
Rss:                   0 kB\n\
Pss:                   0 kB\n\
Pss_Dirty:             0 kB\n\
Shared_Clean:          0 kB\n\
Shared_Dirty:          0 kB\n\
Private_Clean:         0 kB\n\
Private_Dirty:         0 kB\n\
Referenced:            0 kB\n\
Anonymous:             0 kB\n\
KSM:                   0 kB\n\
LazyFree:              0 kB\n\
AnonHugePages:         0 kB\n\
ShmemPmdMapped:        0 kB\n\
FilePmdMapped:         0 kB\n\
Shared_Hugetlb:        0 kB\n\
Private_Hugetlb:       0 kB\n\
Swap:                  0 kB\n\
SwapPss:               0 kB\n\
Locked:                0 kB\n\
THPeligible:           0\n\0";

    unsafe extern "C" fn sigaction_SIGSEGV(_: c_int, _: *mut libc::siginfo_t, _: *mut c_void) {
        libc::_exit(libc::EXIT_FAILURE);
    }

    #[cfg(target_arch = "x86_64")]
    unsafe extern "C" fn sigaction_SIGSEGV_vsyscall(
        _: c_int,
        _: *mut libc::siginfo_t,
        _: *mut c_void,
    ) {
        libc::_exit(G_VSYSCALL);
    }

    /*
     * vsyscall page can't be unmapped, probe it directly.
     */
    #[cfg(target_arch = "x86_64")]
    unsafe fn vsyscall() {
        let pid: libc::pid_t;
        let mut wstatus: c_int = 0;

        pid = libc::fork();
        if pid < 0 {
            libc::fprintf(libc::stderr, b"fork, errno %d\n\0".as_ptr() as *const c_char, errno());
            libc::exit(1);
        }
        if pid == 0 {
            let rlim: libc::rlimit = core::mem::zeroed();
            libc::setrlimit(libc::RLIMIT_CORE, &rlim);

            /* Hide "segfault at ffffffffff600000" messages. */
            let mut act: libc::sigaction = core::mem::zeroed();
            act.sa_flags = libc::SA_SIGINFO;
            act.sa_sigaction = sigaction_SIGSEGV_vsyscall as usize;
            libc::sigaction(libc::SIGSEGV, &act, ptr::null_mut());

            G_VSYSCALL = 0;
            /* gettimeofday(NULL, NULL); */
            let mut rax: u64 = 0xffffffffff600000;
            asm!(
                "call *{rax}",
                rax = inout(reg) rax,
                in("rdi") ptr::null::<c_void>(),
                in("rsi") ptr::null::<c_void>(),
                lateout("rcx") _,
                lateout("r11") _,
            );

            G_VSYSCALL = 1;
            ptr::read_volatile(0xffffffffff600000usize as *const c_int);

            G_VSYSCALL = 2;
            libc::exit(G_VSYSCALL);
        }
        libc::waitpid(pid, &mut wstatus, 0);
        if libc::WIFEXITED(wstatus) {
            G_VSYSCALL = libc::WEXITSTATUS(wstatus);
        } else {
            libc::fprintf(
                libc::stderr,
                b"error: vsyscall wstatus %08x\n\0".as_ptr() as *const c_char,
                wstatus,
            );
            libc::exit(1);
        }
    }

    unsafe fn test_proc_pid_maps(pid: libc::pid_t) -> c_int {
        let mut buf = [0u8; 4096];
        libc::snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            b"/proc/%u/maps\0".as_ptr() as *const c_char,
            pid,
        );
        let fd = libc::open(buf.as_ptr() as *const c_char, libc::O_RDONLY);
        if fd == -1 {
            libc::perror(b"open /proc/${pid}/maps\0".as_ptr() as *const c_char);
            libc::EXIT_FAILURE
        } else {
            let rv = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
            libc::close(fd);
            if G_VSYSCALL == 0 {
                assert!(rv == 0);
            } else {
                let len = libc::strlen(G_PROC_PID_MAPS_VSYSCALL);
                assert!(rv == len as isize);
                assert!(libc::memcmp(buf.as_ptr() as *const c_void, G_PROC_PID_MAPS_VSYSCALL as *const c_void, len) == 0);
            }
            libc::EXIT_SUCCESS
        }
    }

    unsafe fn test_proc_pid_numa_maps(pid: libc::pid_t) -> c_int {
        let mut buf = [0u8; 4096];
        libc::snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            b"/proc/%u/numa_maps\0".as_ptr() as *const c_char,
            pid,
        );
        let fd = libc::open(buf.as_ptr() as *const c_char, libc::O_RDONLY);
        if fd == -1 {
            if errno() == libc::ENOENT {
                /*
                 * /proc/${pid}/numa_maps is under CONFIG_NUMA,
                 * it doesn't necessarily exist.
                 */
                return libc::EXIT_SUCCESS;
            }
            libc::perror(b"open /proc/${pid}/numa_maps\0".as_ptr() as *const c_char);
            libc::EXIT_FAILURE
        } else {
            let rv = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
            libc::close(fd);
            assert!(rv == 0);
            libc::EXIT_SUCCESS
        }
    }

    unsafe fn test_proc_pid_smaps(pid: libc::pid_t) -> c_int {
        let mut buf = [0u8; 4096];
        libc::snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            b"/proc/%u/smaps\0".as_ptr() as *const c_char,
            pid,
        );
        let fd = libc::open(buf.as_ptr() as *const c_char, libc::O_RDONLY);
        if fd == -1 {
            if errno() == libc::ENOENT {
                /*
                 * /proc/${pid}/smaps is under CONFIG_PROC_PAGE_MONITOR,
                 * it doesn't necessarily exist.
                 */
                return libc::EXIT_SUCCESS;
            }
            libc::perror(b"open /proc/${pid}/smaps\0".as_ptr() as *const c_char);
            return libc::EXIT_FAILURE;
        }
        let rv = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        libc::close(fd);

        assert!(0 <= rv);
        assert!(rv <= buf.len() as isize);

        if G_VSYSCALL == 0 {
            assert!(rv == 0);
        } else {
            let len = libc::strlen(G_PROC_PID_SMAPS_VSYSCALL);
            assert!(rv > len as isize);
            assert!(libc::memcmp(buf.as_ptr() as *const c_void, G_PROC_PID_SMAPS_VSYSCALL as *const c_void, len) == 0);

            if G_PROTECTION_KEY_SUPPORT != 0 {
                const PROTECTION_KEY: &[u8] = b"ProtectionKey:         0\n\0";
                assert!(!libc::memmem(
                    buf.as_ptr() as *const c_void,
                    rv as usize,
                    PROTECTION_KEY.as_ptr() as *const c_void,
                    libc::strlen(PROTECTION_KEY.as_ptr() as *const c_char),
                )
                .is_null());
            }
        }

        libc::EXIT_SUCCESS
    }

    static G_SMAPS_ROLLUP: &[u8] =
        b"00000000-00000000 ---p 00000000 00:00 0                                  [rollup]\n\
Rss:                   0 kB\n\
Pss:                   0 kB\n\
Pss_Dirty:             0 kB\n\
Pss_Anon:              0 kB\n\
Pss_File:              0 kB\n\
Pss_Shmem:             0 kB\n\
Shared_Clean:          0 kB\n\
Shared_Dirty:          0 kB\n\
Private_Clean:         0 kB\n\
Private_Dirty:         0 kB\n\
Referenced:            0 kB\n\
Anonymous:             0 kB\n\
KSM:                   0 kB\n\
LazyFree:              0 kB\n\
AnonHugePages:         0 kB\n\
ShmemPmdMapped:        0 kB\n\
FilePmdMapped:         0 kB\n\
Shared_Hugetlb:        0 kB\n\
Private_Hugetlb:       0 kB\n\
Swap:                  0 kB\n\
SwapPss:               0 kB\n\
Locked:                0 kB\n\0";

    unsafe fn test_proc_pid_smaps_rollup(pid: libc::pid_t) -> c_int {
        let mut buf = [0u8; 4096];
        libc::snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            b"/proc/%u/smaps_rollup\0".as_ptr() as *const c_char,
            pid,
        );
        let fd = libc::open(buf.as_ptr() as *const c_char, libc::O_RDONLY);
        if fd == -1 {
            if errno() == libc::ENOENT {
                /*
                 * /proc/${pid}/smaps_rollup is under CONFIG_PROC_PAGE_MONITOR,
                 * it doesn't necessarily exist.
                 */
                return libc::EXIT_SUCCESS;
            }
            libc::perror(b"open /proc/${pid}/smaps_rollup\0".as_ptr() as *const c_char);
            libc::EXIT_FAILURE
        } else {
            let rv = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
            libc::close(fd);
            assert!(rv == (G_SMAPS_ROLLUP.len() - 1) as isize);
            assert!(libc::memcmp(
                buf.as_ptr() as *const c_void,
                G_SMAPS_ROLLUP.as_ptr() as *const c_void,
                G_SMAPS_ROLLUP.len() - 1,
            ) == 0);
            libc::EXIT_SUCCESS
        }
    }

    unsafe fn parse_u64(mut p: *const c_char, end: *const c_char, rv: *mut u64) -> *const c_char {
        *rv = 0;
        while p != end {
            if b'0' as c_char <= *p && *p <= b'9' as c_char {
                let (mul, mul_overflow) = (*rv).overflowing_mul(10);
                assert!(!mul_overflow);
                *rv = mul;
                let (add, add_overflow) = (*rv).overflowing_add((*p - b'0' as c_char) as u64);
                assert!(!add_overflow);
                *rv = add;
            } else {
                break;
            }
            p = p.add(1);
        }
        assert!(p != end);
        p
    }

    /*
     * There seems to be 2 types of valid output:
     * "0 A A B 0 0 0\n" for dynamic exeuctables,
     * "0 0 0 B 0 0 0\n" for static executables.
     */
    unsafe fn test_proc_pid_statm(pid: libc::pid_t) -> c_int {
        let mut buf = [0u8; 4096];
        libc::snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            b"/proc/%u/statm\0".as_ptr() as *const c_char,
            pid,
        );
        let fd = libc::open(buf.as_ptr() as *const c_char, libc::O_RDONLY);
        if fd == -1 {
            libc::perror(b"open /proc/${pid}/statm\0".as_ptr() as *const c_char);
            return libc::EXIT_FAILURE;
        }

        let read_rv = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        libc::close(fd);

        assert!(read_rv >= 0);
        assert!(read_rv <= buf.len() as isize);

        let mut p = buf.as_ptr() as *const c_char;
        let end = p.add(read_rv as usize);

        /* size */
        assert!(p != end && *p == b'0' as c_char);
        p = p.add(1);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        let mut resident: u64 = 0;
        p = parse_u64(p, end, &mut resident);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        let mut shared: u64 = 0;
        p = parse_u64(p, end, &mut shared);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        let mut text: u64 = 0;
        p = parse_u64(p, end, &mut text);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        assert!(p != end && *p == b'0' as c_char);
        p = p.add(1);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        /* data */
        assert!(p != end && *p == b'0' as c_char);
        p = p.add(1);
        assert!(p != end && *p == b' ' as c_char);
        p = p.add(1);

        assert!(p != end && *p == b'0' as c_char);
        p = p.add(1);
        assert!(p != end && *p == b'\n' as c_char);
        p = p.add(1);

        assert!(p == end);

        /*
         * "text" is "mm->end_code - mm->start_code" at execve(2) time.
         * munmap() doesn't change it. It can be anything (just link
         * statically). It can't be 0 because executing to this point
         * implies at least 1 page of code.
         */
        assert!(text > 0);

        /*
         * These two are always equal. Always 0 for statically linked
         * executables and sometimes 0 for dynamically linked executables.
         * There is no way to tell one from another without parsing ELF
         * which is too much for this test.
         */
        assert!(resident == shared);

        libc::EXIT_SUCCESS
    }

    pub unsafe fn main_impl() -> c_int {
        let mut rv = libc::EXIT_SUCCESS;

        #[cfg(target_arch = "x86_64")]
        vsyscall();

        match G_VSYSCALL {
            0 => {
                G_PROC_PID_MAPS_VSYSCALL = PROC_PID_MAPS_VSYSCALL_0.as_ptr() as *const c_char;
                G_PROC_PID_SMAPS_VSYSCALL = PROC_PID_SMAPS_VSYSCALL_0.as_ptr() as *const c_char;
            }
            1 => {
                G_PROC_PID_MAPS_VSYSCALL = PROC_PID_MAPS_VSYSCALL_1.as_ptr() as *const c_char;
                G_PROC_PID_SMAPS_VSYSCALL = PROC_PID_SMAPS_VSYSCALL_1.as_ptr() as *const c_char;
            }
            2 => {
                G_PROC_PID_MAPS_VSYSCALL = PROC_PID_MAPS_VSYSCALL_2.as_ptr() as *const c_char;
                G_PROC_PID_SMAPS_VSYSCALL = PROC_PID_SMAPS_VSYSCALL_2.as_ptr() as *const c_char;
            }
            _ => libc::abort(),
        }

        G_PROTECTION_KEY_SUPPORT = protection_key_support();

        let pid = libc::fork();
        if pid == -1 {
            libc::perror(b"fork\0".as_ptr() as *const c_char);
            return libc::EXIT_FAILURE;
        } else if pid == 0 {
            rv = libc::ptrace(
                libc::PTRACE_TRACEME,
                0,
                ptr::null_mut::<c_void>(),
                ptr::null_mut::<c_void>(),
            ) as c_int;
            if rv != 0 {
                if errno() == libc::EPERM {
                    libc::fprintf(
                        libc::stderr,
                        b"Did you know? ptrace(PTRACE_TRACEME) doesn't work under strace.\n\0"
                            .as_ptr() as *const c_char,
                    );
                    libc::kill(libc::getppid(), libc::SIGTERM);
                    return libc::EXIT_FAILURE;
                }
                libc::perror(b"ptrace PTRACE_TRACEME\0".as_ptr() as *const c_char);
                return libc::EXIT_FAILURE;
            }

            /*
             * Hide "segfault at ..." messages. Signal handler won't run.
             */
            let mut act: libc::sigaction = core::mem::zeroed();
            act.sa_flags = libc::SA_SIGINFO;
            act.sa_sigaction = sigaction_SIGSEGV as usize;
            libc::sigaction(libc::SIGSEGV, &act, ptr::null_mut());

            #[cfg(target_arch = "x86_64")]
            {
                libc::munmap(ptr::null_mut(), ((1usize << 47) - 4096) as libc::size_t);
            }
            #[cfg(target_arch = "x86")]
            {
                let mut len: usize = usize::MAX - 4095;

                loop {
                    libc::munmap(ptr::null_mut(), len as libc::size_t);
                    len = len.wrapping_sub(4096);
                }
            }
            libc::EXIT_FAILURE
        } else {
            /*
             * TODO find reliable way to signal parent that munmap(2) completed.
             * Child can't do it directly because it effectively doesn't exist
             * anymore. Looking at child's VM files isn't 100% reliable either:
             * due to a bug they may not become empty or empty-like.
             */
            libc::sleep(1);

            if rv == libc::EXIT_SUCCESS {
                rv = test_proc_pid_maps(pid);
            }
            if rv == libc::EXIT_SUCCESS {
                rv = test_proc_pid_numa_maps(pid);
            }
            if rv == libc::EXIT_SUCCESS {
                rv = test_proc_pid_smaps(pid);
            }
            if rv == libc::EXIT_SUCCESS {
                rv = test_proc_pid_smaps_rollup(pid);
            }
            if rv == libc::EXIT_SUCCESS {
                rv = test_proc_pid_statm(pid);
            }

            /* Cut the rope. */
            let mut wstatus: c_int = 0;
            libc::waitpid(pid, &mut wstatus, 0);
            assert!(libc::WIFSTOPPED(wstatus));
            assert!(libc::WSTOPSIG(wstatus) == libc::SIGSEGV);
            rv
        }
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn main() {
    unsafe {
        std::process::exit(proc_empty_vm::main_impl());
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
fn main() {
    std::process::exit(4);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
