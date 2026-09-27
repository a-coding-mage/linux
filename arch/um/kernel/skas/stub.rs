// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Benjamin Berg <benjamin@sipsolutions.net>
 */

// C dependencies supplied by the surrounding low-level environment:
// <sysdep/stub.h>, <linux/futex.h>, <sys/socket.h>, and <errno.h>.

/*
 * Known security issues
 *
 * Userspace can jump to this address to execute *any* syscall that is
 * permitted by the stub. As we will return afterwards, it can do
 * whatever it likes, including:
 * - Tricking the kernel into handing out the memory FD
 * - Using this memory FD to read/write all physical memory
 * - Running in parallel to the kernel processing a syscall
 *   (possibly creating data races?)
 * - Blocking e.g. SIGALRM to avoid time based scheduling
 *
 * To avoid this, the permitted location for each syscall needs to be
 * checked for in the SECCOMP filter (which is reasonably simple). Also,
 * more care will need to go into considerations how the code might be
 * tricked by using a prepared stack (or even modifying the stack from
 * another thread in case SMP support is added).
 *
 * As for the SIGALRM, the best counter measure will be to check in the
 * kernel that the process is reporting back the SIGALRM in a timely
 * fashion.
 */

extern "C" {
    fn get_stub_data() -> *mut stub_data;
    fn stub_syscall0(nr: kernel::ffi::c_ulong) -> kernel::ffi::c_long;
    fn stub_syscall1(nr: kernel::ffi::c_ulong, a1: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    fn stub_syscall2(nr: kernel::ffi::c_ulong, a1: kernel::ffi::c_ulong, a2: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    fn stub_syscall3(nr: kernel::ffi::c_ulong, a1: kernel::ffi::c_ulong, a2: kernel::ffi::c_ulong, a3: kernel::ffi::c_ulong) -> kernel::ffi::c_long;
    fn stub_syscall4(nr: kernel::ffi::c_ulong, a1: kernel::ffi::c_ulong, a2: kernel::ffi::c_ulong, a3: kernel::ffi::c_ulong, a4: kernel::ffi::c_ulong) -> kernel::ffi::c_long;
    fn stub_syscall6(nr: kernel::ffi::c_ulong, a1: kernel::ffi::c_ulong, a2: kernel::ffi::c_ulong, a3: kernel::ffi::c_ulong, a4: kernel::ffi::c_ulong, a5: kernel::ffi::c_ulong, a6: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    fn trap_myself();
    fn stub_seccomp_restore_state(data: *mut arch_data);
}

#[repr(C)]
struct stub_data {
    syscall_data_len: kernel::ffi::c_int,
    syscall_data: *mut stub_syscall,
    err: kernel::ffi::c_ulong,
    signal: kernel::ffi::c_int,
    si_offset: kernel::ffi::c_ulong,
    mctx_offset: kernel::ffi::c_ulong,
    sigstack: [kernel::ffi::c_char; 0],
    futex: kernel::ffi::c_int,
    restart_wait: kernel::ffi::c_int,
    arch_data: arch_data,
}

#[repr(C)]
struct stub_syscall {
    syscall: kernel::ffi::c_int,
    mem: stub_mem,
}

#[repr(C)]
struct stub_mem {
    fd: kernel::ffi::c_int,
    addr: kernel::ffi::c_ulong,
    length: kernel::ffi::c_ulong,
    prot: kernel::ffi::c_ulong,
    offset: kernel::ffi::c_ulong,
}

#[repr(C)]
struct arch_data {
    _opaque: [u8; 0],
}

const STUB_SYSCALL_MMAP: kernel::ffi::c_int = 0;
const STUB_SYSCALL_MUNMAP: kernel::ffi::c_int = 1;
const STUB_MAX_FDS: usize = 16;
const STUB_MMAP_NR: kernel::ffi::c_ulong = 9;
const FUTEX_IN_KERN: kernel::ffi::c_int = 1;
const FUTEX_WAKE: kernel::ffi::c_ulong = 1;
const FUTEX_WAIT: kernel::ffi::c_ulong = 0;
const MAP_SHARED: kernel::ffi::c_ulong = 1;
const MAP_FIXED: kernel::ffi::c_ulong = 0x10;
const __NR_munmap: kernel::ffi::c_ulong = 11;
const __NR_futex: kernel::ffi::c_ulong = 202;
const __NR_recvmsg: kernel::ffi::c_ulong = 47;
const __NR_close: kernel::ffi::c_ulong = 3;
const __NR_exit_group: kernel::ffi::c_ulong = 231;
const __NR_rt_sigreturn: kernel::ffi::c_ulong = 15;
const __NR_EINTR: kernel::ffi::c_long = 4;
const __NR_EAGAIN: kernel::ffi::c_long = 11;
const SIGSYS: kernel::ffi::c_int = 31;

#[inline(always)]
unsafe fn syscall_handler(fd_map: *mut kernel::ffi::c_int) -> kernel::ffi::c_int {
    let d = get_stub_data();
    let mut i = 0;
    let mut res: kernel::ffi::c_ulong;
    let mut fd: kernel::ffi::c_int;

    while i < (*d).syscall_data_len {
        let sc = &mut *(*d).syscall_data.add(i as usize);
        match (*sc).syscall {
            STUB_SYSCALL_MMAP => {
                fd = if !fd_map.is_null() { *fd_map.add((*sc).mem.fd as usize) } else { (*sc).mem.fd };
                res = stub_syscall6(STUB_MMAP_NR, (*sc).mem.addr, (*sc).mem.length,
                    (*sc).mem.prot, MAP_SHARED | MAP_FIXED, fd as kernel::ffi::c_ulong, (*sc).mem.offset);
                if res != (*sc).mem.addr { (*d).err = res; (*d).syscall_data_len = i; return -1; }
            }
            STUB_SYSCALL_MUNMAP => {
                res = stub_syscall2(__NR_munmap, (*sc).mem.addr, (*sc).mem.length);
                if res != 0 { (*d).err = res; (*d).syscall_data_len = i; return -1; }
            }
            _ => { (*d).err = 95; (*d).syscall_data_len = i; return -1; }
        }
        i += 1;
    }
    (*d).err = 0;
    (*d).syscall_data_len = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn stub_syscall_handler() {
    syscall_handler(core::ptr::null_mut());
    trap_myself();
}

#[no_mangle]
pub unsafe extern "C" fn stub_signal_interrupt(_sig: kernel::ffi::c_int, _info: *mut libc::siginfo_t, _p: *mut kernel::ffi::c_void) {
    let d = get_stub_data();
    (*d).signal = _sig;
    // The C implementation initializes a one-byte iovec and a control
    // buffer containing CMSG_SPACE(sizeof(int) * STUB_MAX_FDS), then receives
    // ancillary file descriptors with recvmsg.
    let mut rcv_data: kernel::ffi::c_char = 0;
    let mut ctrl = [0u8; 256];
    let mut iov = libc::iovec { iov_base: &mut rcv_data as *mut _ as *mut kernel::ffi::c_void, iov_len: 1 };
    let mut msghdr = libc::msghdr {
        msg_name: core::ptr::null_mut(), msg_namelen: 0, msg_iov: &mut iov,
        msg_iovlen: 1, msg_control: ctrl.as_mut_ptr() as *mut kernel::ffi::c_void,
        msg_controllen: ctrl.len(), msg_flags: 0,
    };
    let mut fd_map: *mut kernel::ffi::c_int;
    let mut num_fds: kernel::ffi::c_int;
    let mut fd_msg: *mut libc::cmsghdr;
    loop {
        (*d).futex = FUTEX_IN_KERN;
        let mut res;
        loop { res = stub_syscall3(__NR_futex, &mut (*d).futex as *mut _ as kernel::ffi::c_ulong, FUTEX_WAKE, 1); if res != -__NR_EINTR { break; } }
        loop { res = stub_syscall4(__NR_futex, &mut (*d).futex as *mut _ as kernel::ffi::c_ulong, FUTEX_WAIT, FUTEX_IN_KERN as kernel::ffi::c_ulong, 0); if res != -__NR_EINTR && (*d).futex != FUTEX_IN_KERN { break; } }
        if res < 0 && res != -__NR_EAGAIN { stub_syscall1(__NR_exit_group, 1); }
        if (*d).syscall_data_len != 0 {
            loop { res = stub_syscall3(__NR_recvmsg, 0, &mut msghdr as *mut _ as kernel::ffi::c_ulong, 0); if res != -__NR_EINTR { break; } }
            if res < 0 && res != -__NR_EAGAIN { stub_syscall1(__NR_exit_group, 1); }
            num_fds = 0;
            fd_msg = msghdr.msg_control as *mut libc::cmsghdr;
            fd_map = (fd_msg as *mut u8).add(core::mem::size_of::<libc::cmsghdr>()) as *mut kernel::ffi::c_int;
            if res == iov.iov_len as kernel::ffi::c_long && msghdr.msg_controllen > core::mem::size_of::<libc::cmsghdr>() { num_fds = ((*fd_msg).cmsg_len as usize - core::mem::size_of::<libc::cmsghdr>()) as kernel::ffi::c_int / core::mem::size_of::<kernel::ffi::c_int>() as kernel::ffi::c_int; }
            res = syscall_handler(fd_map) as kernel::ffi::c_long;
            while num_fds > 0 { num_fds -= 1; stub_syscall2(__NR_close, *fd_map.add(num_fds as usize) as kernel::ffi::c_ulong, 0); }
        } else { res = 0; }
        if res < 0 || (*d).restart_wait != 0 { (*d).signal = SIGSYS; (*d).restart_wait = 0; continue; }
        stub_seccomp_restore_state(&mut (*d).arch_data);
        return;
    }
}

#[no_mangle]
pub unsafe extern "C" fn stub_signal_restorer() {
    stub_syscall0(__NR_rt_sigreturn);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
