// Translated from stub_exe.c.  Kernel and project constants/types are supplied
// by the corresponding external headers and build configuration.

use core::ffi::c_void;

#[repr(C)]
pub struct stub_init_data {
    pub stub_start: usize,
    pub stub_code_fd: usize,
    pub stub_code_offset: usize,
    pub stub_data_fd: usize,
    pub stub_data_offset: usize,
    pub signal_handler: usize,
    pub signal_restorer: usize,
    pub seccomp: i32,
}

#[repr(C)]
pub struct sock_filter {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

#[repr(C)]
pub struct sock_fprog {
    pub len: u16,
    pub filter: *mut sock_filter,
}

unsafe extern "C" {
    fn stub_syscall0(nr: usize) -> usize;
    fn stub_syscall1(nr: usize, arg1: usize) -> usize;
    fn stub_syscall2(nr: usize, arg1: usize, arg2: usize) -> usize;
    fn stub_syscall3(nr: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn stub_syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize;
    fn stub_syscall5(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize;
    fn stub_syscall6(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> usize;
    fn stub_start(entry: unsafe extern "C" fn());
}

unsafe fn real_init() {
    let mut init_data: stub_init_data = core::mem::zeroed();
    let mut res: usize;
    let mut stack = Stack {
        ss_sp: core::ptr::null_mut(),
        ss_flags: 0,
        ss_size: STUB_DATA_PAGES * UM_KERN_PAGE_SIZE,
    };
    let mut sa = SigAction {
        sa_handler_: core::ptr::null_mut(),
        sa_flags: SA_ONSTACK | SA_NODEFER | SA_SIGINFO | 0x04000000,
        sa_restorer: core::ptr::null_mut(),
        sa_mask: 0,
    };

    stub_syscall2(__NR_prctl, PR_SET_NAME, b"uml-userspace\0".as_ptr() as usize);
    stub_syscall2(__NR_prctl, PR_SET_PDEATHSIG, SIGKILL);
    stub_syscall5(__NR_prctl, PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);

    res = stub_syscall3(__NR_read, 0, &mut init_data as *mut _ as usize, core::mem::size_of::<stub_init_data>());
    if res != core::mem::size_of::<stub_init_data>() { stub_syscall1(__NR_exit, 10); }

    if init_data.seccomp == 0 {
        stub_syscall1(__NR_close, 0);
    } else {
        stub_syscall3(__NR_fcntl, 0, F_SETFL, O_NONBLOCK);
    }

    res = stub_syscall6(STUB_MMAP_NR, init_data.stub_start, UM_KERN_PAGE_SIZE,
        PROT_READ | PROT_EXEC, MAP_FIXED | MAP_SHARED, init_data.stub_code_fd, init_data.stub_code_offset);
    if res != init_data.stub_start { stub_syscall1(__NR_exit, 11); }

    res = stub_syscall6(STUB_MMAP_NR, init_data.stub_start + UM_KERN_PAGE_SIZE,
        STUB_DATA_PAGES * UM_KERN_PAGE_SIZE, PROT_READ | PROT_WRITE,
        MAP_FIXED | MAP_SHARED, init_data.stub_data_fd, init_data.stub_data_offset);
    if res != init_data.stub_start + UM_KERN_PAGE_SIZE { stub_syscall1(__NR_exit, 12); }

    if init_data.seccomp != 0 {
        res = stub_syscall3(__NR_close_range, 1, !0u32 as usize, 0);
        if res != 0 { stub_syscall1(__NR_exit, 13); }
    }

    stack.ss_sp = (init_data.stub_start + UM_KERN_PAGE_SIZE) as *mut c_void;
    stub_syscall2(__NR_sigaltstack, &stack as *const _ as usize, 0);

    sa.sa_handler_ = init_data.signal_handler as *mut c_void;
    sa.sa_restorer = init_data.signal_restorer as *mut c_void;
    if init_data.seccomp == 0 {
        sa.sa_mask = 0;
        res = stub_syscall4(__NR_rt_sigaction, SIGSEGV, &sa as *const _ as usize, 0, core::mem::size_of::<u64>());
        if res != 0 { stub_syscall1(__NR_exit, 14); }
    } else {
        sa.sa_mask = !0u64;
        for (signal, status) in [(SIGSEGV, 15), (SIGSYS, 16), (SIGALRM, 17), (SIGTRAP, 18), (SIGILL, 19), (SIGFPE, 20)] {
            res = stub_syscall4(__NR_rt_sigaction, signal, &sa as *const _ as usize, 0, core::mem::size_of::<u64>());
            if res != 0 { stub_syscall1(__NR_exit, status); }
        }
    }

    if init_data.seccomp != 0 {
        let mut filter: [sock_filter; 19] = [sock_filter { code: 0, jt: 0, jf: 0, k: 0 }; 19];
        // BPF_STMT/BPF_JUMP entries are populated by the platform's BPF definitions.
        filter[0] = BPF_STMT(BPF_LD | BPF_W | BPF_ABS, core::mem::offset_of!(SeccompData, instruction_pointer) as u32 + 4);
        filter[1] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, (init_data.stub_start >> 32) as u32, 0, 3);
        filter[2] = BPF_STMT(BPF_LD | BPF_W | BPF_ABS, core::mem::offset_of!(SeccompData, instruction_pointer) as u32);
        filter[3] = BPF_STMT(BPF_ALU | BPF_AND | BPF_K, 0xfffff000);
        filter[4] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, (init_data.stub_start & 0xfffff000) as u32, 1, 0);
        filter[5] = BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_TRAP);
        filter[6] = BPF_STMT(BPF_LD | BPF_W | BPF_ABS, core::mem::offset_of!(SeccompData, arch) as u32);
        filter[7] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, UM_SECCOMP_ARCH_NATIVE, 1, 0);
        filter[8] = BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_KILL_PROCESS);
        filter[9] = BPF_STMT(BPF_LD | BPF_W | BPF_ABS, core::mem::offset_of!(SeccompData, nr) as u32);
        filter[10] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_futex, 7, 0);
        filter[11] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_recvmsg, 6, 0);
        filter[12] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_close, 5, 0);
        filter[13] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, STUB_MMAP_NR, 4, 0);
        filter[14] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_munmap, 3, 0);
        filter[15] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_arch_prctl, 2, 0);
        filter[16] = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_rt_sigreturn, 1, 0);
        filter[17] = BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_KILL_PROCESS);
        filter[18] = BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW);
        let prog = sock_fprog { len: filter.len() as u16, filter: filter.as_mut_ptr() };
        if stub_syscall3(__NR_seccomp, SECCOMP_SET_MODE_FILTER, SECCOMP_FILTER_FLAG_TSYNC, &prog as *const _ as usize) != 0 { stub_syscall1(__NR_exit, 21); }
    } else {
        stub_syscall4(__NR_ptrace, PTRACE_TRACEME, 0, 0, 0);
        stub_syscall2(__NR_kill, stub_syscall0(__NR_getpid), SIGSTOP);
    }
    stub_syscall1(__NR_exit, 30);
    core::hint::unreachable_unchecked();
}

#[repr(C)] struct Stack { ss_sp: *mut c_void, ss_flags: i32, ss_size: usize }
#[repr(C)] struct SigAction { sa_handler_: *mut c_void, sa_flags: usize, sa_restorer: *mut c_void, sa_mask: u64 }

#[unsafe(naked)]
pub unsafe extern "C" fn _start() {
    core::arch::naked_asm!("call {0}", sym real_init);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
