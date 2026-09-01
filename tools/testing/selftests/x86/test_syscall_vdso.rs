// SPDX-License-Identifier: GPL-2.0-only
/*
 * 32-bit syscall ABI conformance test.
 *
 * Copyright (c) 2015 Denys Vlasenko
 */
/*
 * Can be built statically:
 * gcc -Os -Wall -static -m32 test_syscall_vdso.c thunks_32.S
 */

#[cfg(not(target_arch = "x86"))]
fn main() {
    println!("[SKIP]\tNot a 32-bit x86 userspace");
}

#[cfg(target_arch = "x86")]
mod test_syscall_vdso {
    use core::arch::{asm, global_asm};
    use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
    use core::mem;
    use core::ptr;

    const AT_NULL: u32 = 0;
    const AT_SYSINFO: u32 = 32;
    const SIGINT: c_int = 2;
    const SIGUSR2: c_int = 12;
    const SIGRTMAX: c_int = 64;
    const SIGSTOP: c_int = 19;
    const PTRACE_TRACEME: c_int = 0;
    const PTRACE_SYSCALL: c_int = 24;
    const __WALL: c_int = 0x40000000;

    #[repr(C)]
    union Elf32AuxvUn {
        a_val: u32,
    }

    #[repr(C)]
    struct Elf32AuxvT {
        a_type: u32,
        a_un: Elf32AuxvUn,
    }

    static mut syscall_addr: c_long = 0;

    unsafe fn get_syscall(mut envp: *mut *mut c_char) -> c_long {
        let mut auxv: *mut Elf32AuxvT;

        while !(*envp).is_null() {
            envp = envp.add(1);
        }
        envp = envp.add(1);

        auxv = envp as *mut Elf32AuxvT;
        while (*auxv).a_type != AT_NULL {
            if (*auxv).a_type == AT_SYSINFO {
                return (*auxv).a_un.a_val as c_long;
            }
            auxv = auxv.add(1);
        }
        printf(c"[WARN]\tAT_SYSINFO not supplied\n".as_ptr());
        0
    }

    global_asm!(
        "   .pushsection .text",
        "   .global int80",
        "int80:",
        "   int $0x80",
        "   ret",
        "   .popsection",
    );

    unsafe extern "C" {
        static int80: c_char;
    }

    #[repr(C)]
    struct regs64 {
        rax: u64,
        rbx: u64,
        rcx: u64,
        rdx: u64,
        rsi: u64,
        rdi: u64,
        rbp: u64,
        rsp: u64,
        r8: u64,
        r9: u64,
        r10: u64,
        r11: u64,
        r12: u64,
        r13: u64,
        r14: u64,
        r15: u64,
    }

    static mut regs64: regs64 = regs64 {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        rsp: 0,
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
        r15: 0,
    };
    static mut kernel_is_64bit: c_int = 0;

    global_asm!(
        "   .pushsection .text",
        "   .code64",
        "get_regs64:",
        "   push %rax",
        "   mov $regs64, %eax",
        "   pop 0*8(%rax)",
        "   movq %rbx, 1*8(%rax)",
        "   movq %rcx, 2*8(%rax)",
        "   movq %rdx, 3*8(%rax)",
        "   movq %rsi, 4*8(%rax)",
        "   movq %rdi, 5*8(%rax)",
        "   movq %rbp, 6*8(%rax)",
        "   movq %rsp, 7*8(%rax)",
        "   movq %r8,  8*8(%rax)",
        "   movq %r9,  9*8(%rax)",
        "   movq %r10, 10*8(%rax)",
        "   movq %r11, 11*8(%rax)",
        "   movq %r12, 12*8(%rax)",
        "   movq %r13, 13*8(%rax)",
        "   movq %r14, 14*8(%rax)",
        "   movq %r15, 15*8(%rax)",
        "   ret",
        "poison_regs64:",
        "   movq $0x7f7f7f7f, %r8",
        "   shl $32, %r8",
        "   orq $0x7f7f7f7f, %r8",
        "   movq %r8, %r9",
        "   incq %r9",
        "   movq %r9, %r10",
        "   incq %r10",
        "   movq %r10, %r11",
        "   incq %r11",
        "   movq %r11, %r12",
        "   incq %r12",
        "   movq %r12, %r13",
        "   incq %r13",
        "   movq %r13, %r14",
        "   incq %r14",
        "   movq %r14, %r15",
        "   incq %r15",
        "   ret",
        "   .code32",
        "   .popsection",
    );

    unsafe extern "C" {
        fn get_regs64();
        fn poison_regs64();
        fn call64_from_32(function: unsafe extern "C" fn()) -> c_ulong;
    }

    unsafe fn print_regs64() {
        if kernel_is_64bit == 0 {
            return;
        }
        printf(
            c"ax:%016llx bx:%016llx cx:%016llx dx:%016llx\n".as_ptr(),
            regs64.rax,
            regs64.rbx,
            regs64.rcx,
            regs64.rdx,
        );
        printf(
            c"si:%016llx di:%016llx bp:%016llx sp:%016llx\n".as_ptr(),
            regs64.rsi,
            regs64.rdi,
            regs64.rbp,
            regs64.rsp,
        );
        printf(
            c" 8:%016llx  9:%016llx 10:%016llx 11:%016llx\n".as_ptr(),
            regs64.r8,
            regs64.r9,
            regs64.r10,
            regs64.r11,
        );
        printf(
            c"12:%016llx 13:%016llx 14:%016llx 15:%016llx\n".as_ptr(),
            regs64.r12,
            regs64.r13,
            regs64.r14,
            regs64.r15,
        );
    }

    unsafe fn check_regs64() -> c_int {
        let mut err: c_int = 0;
        let mut num: c_int = 8;
        let mut r64: *mut u64 = ptr::addr_of_mut!(regs64.r8);
        let mut expected: u64 = 0x7f7f7f7f7f7f7f7f_u64;

        if kernel_is_64bit == 0 {
            return 0;
        }

        loop {
            if *r64 == expected {
                expected = expected.wrapping_add(1);
                r64 = r64.add(1);
                num += 1;
                if num < 16 {
                    continue;
                }
                break;
            }
            expected = expected.wrapping_add(1);
            if syscall_addr != ptr::addr_of!(int80) as c_long {
                /*
                 * Non-INT80 syscall entrypoints are allowed to clobber R8+ regs:
                 * either clear them to 0, or for R11, load EFLAGS.
                 */
                if *r64 == 0 {
                    r64 = r64.add(1);
                    num += 1;
                    if num < 16 {
                        continue;
                    }
                    break;
                }
                if num == 11 {
                    printf(
                        c"[NOTE]\tR11 has changed:%016llx - assuming clobbered by SYSRET insn\n"
                            .as_ptr(),
                        *r64,
                    );
                    r64 = r64.add(1);
                    num += 1;
                    if num < 16 {
                        continue;
                    }
                    break;
                }
            } else {
                /*
                 * INT80 syscall entrypoint can be used by
                 * 64-bit programs too, unlike SYSCALL/SYSENTER.
                 * Therefore it must preserve R12+
                 * (they are callee-saved registers in 64-bit C ABI).
                 *
                 * Starting in Linux 4.17 (and any kernel that
                 * backports the change), R8..11 are preserved.
                 * Historically (and probably unintentionally), they
                 * were clobbered or zeroed.
                 */
            }
            printf(c"[FAIL]\tR%d has changed:%016llx\n".as_ptr(), num, *r64);
            err += 1;
            r64 = r64.add(1);
            num += 1;
            if num >= 16 {
                break;
            }
        }

        if err == 0 {
            printf(c"[OK]\tR8..R15 did not leak kernel data\n".as_ptr());
        }
        err
    }

    #[repr(C)]
    struct fd_set {
        fds_bits: [c_long; 1024 / (8 * mem::size_of::<c_long>())],
    }

    #[repr(C)]
    struct timespec {
        tv_sec: c_long,
        tv_nsec: c_long,
    }

    #[repr(C)]
    struct sigset_t {
        __val: [c_ulong; 1024 / (8 * mem::size_of::<c_ulong>())],
    }

    static mut nfds: c_int = 0;
    static mut rfds: fd_set = fd_set { fds_bits: [0; 1024 / (8 * mem::size_of::<c_long>())] };
    static mut wfds: fd_set = fd_set { fds_bits: [0; 1024 / (8 * mem::size_of::<c_long>())] };
    static mut efds: fd_set = fd_set { fds_bits: [0; 1024 / (8 * mem::size_of::<c_long>())] };
    static mut timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    static mut sigmask: sigset_t = sigset_t { __val: [0; 1024 / (8 * mem::size_of::<c_ulong>())] };

    #[repr(C)]
    struct sigmask_desc_t {
        sp: *mut sigset_t,
        sz: c_int,
    }

    static mut sigmask_desc: sigmask_desc_t = sigmask_desc_t {
        sp: ptr::null_mut(),
        sz: 0,
    };

    unsafe fn fd_zero(set: *mut fd_set) {
        (*set).fds_bits = [0; 1024 / (8 * mem::size_of::<c_long>())];
    }

    unsafe fn fd_set(fd: c_int, set: *mut fd_set) {
        let bits_per_long = 8 * mem::size_of::<c_long>() as c_int;
        (*set).fds_bits[(fd / bits_per_long) as usize] |= 1 << (fd % bits_per_long);
    }

    unsafe fn sigemptyset(set: *mut sigset_t) -> c_int {
        (*set).__val = [0; 1024 / (8 * mem::size_of::<c_ulong>())];
        0
    }

    unsafe fn sigaddset(set: *mut sigset_t, signo: c_int) -> c_int {
        let sig = signo - 1;
        let bits_per_long = 8 * mem::size_of::<c_ulong>() as c_int;
        (*set).__val[(sig / bits_per_long) as usize] |= 1 << (sig % bits_per_long);
        0
    }

    unsafe fn prep_args() {
        nfds = 42;
        fd_zero(ptr::addr_of_mut!(rfds));
        fd_zero(ptr::addr_of_mut!(wfds));
        fd_zero(ptr::addr_of_mut!(efds));
        fd_set(0, ptr::addr_of_mut!(rfds));
        fd_set(1, ptr::addr_of_mut!(wfds));
        fd_set(2, ptr::addr_of_mut!(efds));
        timeout.tv_sec = 0;
        timeout.tv_nsec = 123;
        sigemptyset(ptr::addr_of_mut!(sigmask));
        sigaddset(ptr::addr_of_mut!(sigmask), SIGINT);
        sigaddset(ptr::addr_of_mut!(sigmask), SIGUSR2);
        sigaddset(ptr::addr_of_mut!(sigmask), SIGRTMAX);
        sigmask_desc.sp = ptr::addr_of_mut!(sigmask);
        sigmask_desc.sz = 8; /* bytes */
    }

    unsafe fn print_flags(name: *const c_char, r: c_ulong) {
        static BITARRAY: [*const c_char; 44] = [
            c"\n".as_ptr(), c"c\n".as_ptr(), /* Carry Flag */
            c"0 ".as_ptr(), c"1 ".as_ptr(), /* Bit 1 - always on */
            c"".as_ptr(), c"p ".as_ptr(), /* Parity Flag */
            c"0 ".as_ptr(), c"3? ".as_ptr(),
            c"".as_ptr(), c"a ".as_ptr(), /* Auxiliary carry Flag */
            c"0 ".as_ptr(), c"5? ".as_ptr(),
            c"".as_ptr(), c"z ".as_ptr(), /* Zero Flag */
            c"".as_ptr(), c"s ".as_ptr(), /* Sign Flag */
            c"".as_ptr(), c"t ".as_ptr(), /* Trap Flag */
            c"".as_ptr(), c"i ".as_ptr(), /* Interrupt Flag */
            c"".as_ptr(), c"d ".as_ptr(), /* Direction Flag */
            c"".as_ptr(), c"o ".as_ptr(), /* Overflow Flag */
            c"0 ".as_ptr(), c"1 ".as_ptr(), /* I/O Privilege Level (2 bits) */
            c"0".as_ptr(), c"1".as_ptr(), /* I/O Privilege Level (2 bits) */
            c"".as_ptr(), c"n ".as_ptr(), /* Nested Task */
            c"0 ".as_ptr(), c"15? ".as_ptr(),
            c"".as_ptr(), c"r ".as_ptr(), /* Resume Flag */
            c"".as_ptr(), c"v ".as_ptr(), /* Virtual Mode */
            c"".as_ptr(), c"ac ".as_ptr(), /* Alignment Check/Access Control */
            c"".as_ptr(), c"vif ".as_ptr(), /* Virtual Interrupt Flag */
            c"".as_ptr(), c"vip ".as_ptr(), /* Virtual Interrupt Pending */
            c"".as_ptr(), c"id ".as_ptr(), /* CPUID detection */
        ];
        let mut bitstr: *const *const c_char;
        let mut bit: c_int;

        printf(c"%s=%016lx ".as_ptr(), name, r);
        bitstr = BITARRAY.as_ptr().add(42);
        bit = 21;
        if (r >> 22) != 0 {
            printf(c"(extra bits are set) ".as_ptr());
        }
        loop {
            let s = *bitstr.add(((r >> bit) & 1) as usize);
            if *s != 0 {
                fputs(s, stdout);
            }
            bitstr = bitstr.sub(2);
            bit -= 1;
            if bit < 0 {
                break;
            }
        }
    }

    unsafe fn run_syscall() -> c_int {
        let flags: c_long;
        let bad_arg: c_long;

        prep_args();

        if kernel_is_64bit != 0 {
            call64_from_32(poison_regs64);
        }
        /*print_regs64();*/

        asm!(
            "\n",
            /* Try 6-arg syscall: pselect. It should return quickly */
            "push ebp",
            "mov eax, 308",
            "mov ebx, [{nfds}]",
            "mov ecx, {rfds}",
            "mov edx, {wfds}",
            "mov esi, {efds}",
            "mov edi, {timeout}",
            "mov ebp, {sigmask_desc}",
            "push 0x200ed7",
            "popfd",
            "call dword ptr [{syscall_addr}]",
            /* Check that registers are not clobbered */
            "pushfd",
            "pop eax",
            "cld",
            "cmp ebx, [{nfds}]",
            "mov ebx, 1",
            "jne 2f",
            "cmp ecx, {rfds}",
            "mov ebx, 2",
            "jne 2f",
            "cmp edx, {wfds}",
            "mov ebx, 3",
            "jne 2f",
            "cmp esi, {efds}",
            "mov ebx, 4",
            "jne 2f",
            "cmp edi, {timeout}",
            "mov ebx, 5",
            "jne 2f",
            "cmp ebp, {sigmask_desc}",
            "mov ebx, 6",
            "jne 2f",
            "mov ebx, 0",
            "2:",
            "pop ebp",
            nfds = sym nfds,
            rfds = sym rfds,
            wfds = sym wfds,
            efds = sym efds,
            timeout = sym timeout,
            sigmask_desc = sym sigmask_desc,
            syscall_addr = sym syscall_addr,
            lateout("eax") flags,
            lateout("ebx") bad_arg,
            out("ecx") _,
            out("edx") _,
            out("esi") _,
            out("edi") _,
        );

        if kernel_is_64bit != 0 {
            memset(
                ptr::addr_of_mut!(regs64) as *mut c_void,
                0x77,
                mem::size_of::<regs64>(),
            );
            call64_from_32(get_regs64);
            /*print_regs64();*/
        }

        /*
         * On paravirt kernels, flags are not preserved across syscalls.
         * Thus, we do not consider it a bug if some are changed.
         * We just show ones which do.
         */
        if (0x200ed7 ^ flags) != 0 {
            print_flags(c"[WARN]\tFlags before".as_ptr(), 0x200ed7);
            print_flags(c"[WARN]\tFlags  after".as_ptr(), flags as c_ulong);
            print_flags(c"[WARN]\tFlags change".as_ptr(), (0x200ed7 ^ flags) as c_ulong);
        }

        if bad_arg != 0 {
            printf(c"[FAIL]\targ#%ld clobbered\n".as_ptr(), bad_arg);
            return 1;
        }
        printf(c"[OK]\tArguments are preserved across syscall\n".as_ptr());

        check_regs64()
    }

    unsafe fn run_syscall_twice() -> c_int {
        let mut exitcode: c_int = 0;
        let sv: c_long;

        if syscall_addr != 0 {
            printf(c"[RUN]\tExecuting 6-argument 32-bit syscall via VDSO\n".as_ptr());
            exitcode = run_syscall();
        }
        sv = syscall_addr;
        syscall_addr = ptr::addr_of!(int80) as c_long;
        printf(c"[RUN]\tExecuting 6-argument 32-bit syscall via INT 80\n".as_ptr());
        exitcode += run_syscall();
        syscall_addr = sv;
        exitcode
    }

    unsafe fn wifexited(status: c_int) -> bool {
        (status & 0x7f) == 0
    }

    unsafe fn wexitstatus(status: c_int) -> c_int {
        (status & 0xff00) >> 8
    }

    unsafe fn wifsignaled(status: c_int) -> bool {
        (((status & 0x7f) + 1) >> 1) > 0
    }

    unsafe fn wtermsig(status: c_int) -> c_int {
        status & 0x7f
    }

    unsafe fn wifstopped(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    unsafe fn ptrace_me() {
        let mut pid: pid_t;

        fflush(ptr::null_mut());
        pid = fork();
        if pid < 0 {
            exit(1);
        }
        if pid == 0 {
            /* child */
            if ptrace(PTRACE_TRACEME, 0, 0, 0) != 0 {
                exit(0);
            }
            raise(SIGSTOP);
            return;
        }
        /* parent */
        printf(c"[RUN]\tRunning tests under ptrace\n".as_ptr());
        loop {
            let mut status: c_int = 0;
            pid = waitpid(-1, &mut status, __WALL);
            if wifexited(status) {
                exit(wexitstatus(status));
            }
            if wifsignaled(status) {
                exit(wtermsig(status));
            }
            if pid <= 0 || !wifstopped(status) {
                /* paranoia */
                exit(255);
            }
            /*
             * Note: we do not inject sig = WSTOPSIG(status).
             * We probably should, but careful: do not inject SIGTRAP
             * generated by syscall entry/exit stops.
             * That kills the child.
             */
            ptrace(PTRACE_SYSCALL, pid, 0, 0 /*sig*/);
        }
    }

    type pid_t = c_int;

    unsafe extern "C" {
        static mut stdout: *mut c_void;

        fn printf(format: *const c_char, ...) -> c_int;
        fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
        fn fflush(stream: *mut c_void) -> c_int;
        fn fork() -> pid_t;
        fn exit(status: c_int) -> !;
        fn raise(sig: c_int) -> c_int;
        fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
        fn ptrace(request: c_int, ...) -> c_long;
        fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    }

    pub unsafe fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int {
        let mut exitcode: c_int = 0;
        let cs: c_int;

        asm!(
            "\n",
            "mov {0:e}, cs",
            out(reg) cs,
        );
        kernel_is_64bit = (cs == 0x23) as c_int;
        if kernel_is_64bit == 0 {
            printf(c"[NOTE]\tNot a 64-bit kernel, won't test R8..R15 leaks\n".as_ptr());
        }

        /* This only works for non-static builds:
         * syscall_addr = dlsym(dlopen("linux-gate.so.1", RTLD_NOW), "__kernel_vsyscall");
         */
        syscall_addr = get_syscall(envp);

        exitcode += run_syscall_twice();
        ptrace_me();
        exitcode += run_syscall_twice();

        exitcode
    }
}

#[cfg(target_arch = "x86")]
fn main() {
    unsafe {
        unsafe extern "C" {
            static mut environ: *mut *mut core::ffi::c_char;
        }
        test_syscall_vdso::main(0, core::ptr::null_mut(), environ);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
