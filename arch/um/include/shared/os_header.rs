/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the corresponding C headers are intentionally not
// implemented here.

pub const OS_TYPE_FILE: i32 = 1;
pub const OS_TYPE_DIR: i32 = 2;
pub const OS_TYPE_SYMLINK: i32 = 3;
pub const OS_TYPE_CHARDEV: i32 = 4;
pub const OS_TYPE_BLOCKDEV: i32 = 5;
pub const OS_TYPE_FIFO: i32 = 6;
pub const OS_TYPE_SOCK: i32 = 7;

pub const OS_ACC_F_OK: i32 = 0;
pub const OS_ACC_X_OK: i32 = 1;
pub const OS_ACC_W_OK: i32 = 2;
pub const OS_ACC_R_OK: i32 = 4;
pub const OS_ACC_RW_OK: i32 = OS_ACC_W_OK | OS_ACC_R_OK;

// CONFIG_64BIT selects "/usr/lib64/"; otherwise this is "/usr/lib/".
#[cfg(CONFIG_64BIT)]
pub const OS_LIB_PATH: &str = "/usr/lib64/";
#[cfg(not(CONFIG_64BIT))]
pub const OS_LIB_PATH: &str = "/usr/lib/";

pub const OS_SENDMSG_MAX_FDS: usize = 8;

#[repr(C)]
pub struct uml_stat {
    pub ust_dev: i32,
    pub ust_ino: u64,
    pub ust_mode: i32,
    pub ust_nlink: i32,
    pub ust_uid: i32,
    pub ust_gid: i32,
    pub ust_size: u64,
    pub ust_blksize: i32,
    pub ust_blocks: u64,
    pub ust_atime: usize,
    pub ust_mtime: usize,
    pub ust_ctime: usize,
}

// C unsigned-int bitfields occupy one unsigned-int storage unit.
#[repr(C)]
pub struct openflags {
    bits: u32,
}

impl openflags {
    const R: u32 = 1 << 0;
    const W: u32 = 1 << 1;
    const S: u32 = 1 << 2;
    const C: u32 = 1 << 3;
    const T: u32 = 1 << 4;
    const A: u32 = 1 << 5;
    const E: u32 = 1 << 6;
    const CL: u32 = 1 << 7;
}

pub const fn OPENFLAGS() -> openflags { openflags { bits: 0 } }

pub fn of_read(mut flags: openflags) -> openflags { flags.bits |= openflags::R; flags }
pub fn of_write(mut flags: openflags) -> openflags { flags.bits |= openflags::W; flags }
pub fn of_rdwr(flags: openflags) -> openflags { of_read(of_write(flags)) }
pub fn of_set_rw(mut flags: openflags, r: i32, w: i32) -> openflags {
    flags.bits = (flags.bits & !(openflags::R | openflags::W))
        | if r != 0 { openflags::R } else { 0 }
        | if w != 0 { openflags::W } else { 0 };
    flags
}
pub fn of_sync(mut flags: openflags) -> openflags { flags.bits |= openflags::S; flags }
pub fn of_create(mut flags: openflags) -> openflags { flags.bits |= openflags::C; flags }
pub fn of_trunc(mut flags: openflags) -> openflags { flags.bits |= openflags::T; flags }
pub fn of_append(mut flags: openflags) -> openflags { flags.bits |= openflags::A; flags }
pub fn of_excl(mut flags: openflags) -> openflags { flags.bits |= openflags::E; flags }
pub fn of_cloexec(mut flags: openflags) -> openflags { flags.bits |= openflags::CL; flags }

extern "C" {
    pub fn os_stat_file(file_name: *const core::ffi::c_char, buf: *mut uml_stat) -> i32;
    pub fn os_stat_fd(fd: i32, buf: *mut uml_stat) -> i32;
    pub fn os_access(file: *const core::ffi::c_char, mode: i32) -> i32;
    pub fn os_set_exec_close(fd: i32) -> i32;
    pub fn os_ioctl_generic(fd: i32, cmd: u32, arg: usize) -> i32;
    pub fn os_get_ifname(fd: i32, namebuf: *mut core::ffi::c_char) -> i32;
    pub fn os_mode_fd(fd: i32, mode: i32) -> i32;
    pub fn os_seek_file(fd: i32, offset: u64) -> i32;
    pub fn os_open_file(file: *const core::ffi::c_char, flags: openflags, mode: i32) -> i32;
    pub fn os_read_file(fd: i32, buf: *mut core::ffi::c_void, len: i32) -> i32;
    pub fn os_write_file(fd: i32, buf: *const core::ffi::c_void, count: i32) -> i32;
    pub fn os_sync_file(fd: i32) -> i32;
    pub fn os_file_size(file: *const core::ffi::c_char, size_out: *mut u64) -> i32;
    pub fn os_pread_file(fd: i32, buf: *mut core::ffi::c_void, len: i32, offset: u64) -> i32;
    pub fn os_pwrite_file(fd: i32, buf: *const core::ffi::c_void, count: i32, offset: u64) -> i32;
    pub fn os_file_modtime(file: *const core::ffi::c_char, modtime: *mut i64) -> i32;
    pub fn os_pipe(fd: *mut i32, stream: i32, close_on_exec: i32) -> i32;
    pub fn os_set_fd_async(fd: i32) -> i32;
    pub fn os_clear_fd_async(fd: i32) -> i32;
    pub fn os_set_fd_block(fd: i32, blocking: i32) -> i32;
    pub fn os_accept_connection(fd: i32) -> i32;
    pub fn os_create_unix_socket(file: *const core::ffi::c_char, len: i32, close_on_exec: i32) -> i32;
    pub fn os_shutdown_socket(fd: i32, r: i32, w: i32) -> i32;
    pub fn os_dup_file(fd: i32) -> i32;
    pub fn os_close_file(fd: i32);
    pub fn os_rcv_fd_msg(fd: i32, fds: *mut i32, n_fds: u32, data: *mut core::ffi::c_void, data_len: usize) -> isize;
    pub fn os_connect_socket(name: *const core::ffi::c_char) -> i32;
    pub fn os_file_type(file: *mut core::ffi::c_char) -> i32;
    pub fn os_file_mode(file: *const core::ffi::c_char, mode_out: *mut openflags) -> i32;
    pub fn os_lock_file(fd: i32, excl: i32) -> i32;
    pub fn os_flush_stdout();
    pub fn os_major(dev: u64) -> u32;
    pub fn os_minor(dev: u64) -> u32;
    pub fn os_makedev(major: u32, minor: u32) -> u64;
    pub fn os_falloc_punch(fd: i32, offset: u64, count: i32) -> i32;
    pub fn os_falloc_zeroes(fd: i32, offset: u64, count: i32) -> i32;
    pub fn os_eventfd(initval: u32, flags: i32) -> i32;
    pub fn os_sendmsg_fds(fd: i32, buf: *const core::ffi::c_void, len: u32, fds: *const i32, fds_num: u32) -> i32;
    pub fn os_poll(n: u32, fds: *const i32) -> i32;
    pub fn os_mmap_rw_shared(fd: i32, size: usize) -> *mut core::ffi::c_void;
    pub fn os_mremap_rw_shared(old_addr: *mut core::ffi::c_void, old_size: usize, new_size: usize) -> *mut core::ffi::c_void;
    pub fn os_early_checks();
    pub fn os_check_bugs();
    pub fn check_host_supports_tls(supports_tls: *mut i32, tls_min: *mut i32);
    pub fn create_mem_file(len: u64) -> i32;
    pub fn report_enomem();
    pub fn os_reap_child() -> i32;
    pub fn os_alarm_process(pid: i32);
    pub fn os_kill_process(pid: i32, reap_child: i32);
    pub fn os_kill_ptraced_process(pid: i32, reap_child: i32);
    pub fn os_getpid() -> i32;
    pub fn init_new_thread_signals();
    pub fn os_map_memory(virt: *mut core::ffi::c_void, fd: i32, off: u64, len: usize, r: i32, w: i32, x: i32) -> i32;
    pub fn os_protect_memory(addr: *mut core::ffi::c_void, len: usize, r: i32, w: i32, x: i32) -> i32;
    pub fn os_unmap_memory(addr: *mut core::ffi::c_void, len: i32) -> i32;
    pub fn os_drop_memory(addr: *mut core::ffi::c_void, length: i32) -> i32;
    pub fn can_drop_memory() -> i32;
    pub fn os_set_pdeathsig();
    pub fn os_futex_wait(uaddr: *mut core::ffi::c_void, val: u32) -> i32;
    pub fn os_futex_wake(uaddr: *mut core::ffi::c_void) -> i32;
    pub fn execvp_noalloc(buf: *mut core::ffi::c_char, file: *const core::ffi::c_char, argv: *mut *mut core::ffi::c_char) -> i32;
    pub fn helper_wait(pid: i32) -> i32;
    pub fn os_fix_helper_thread_signals();
    pub fn umid_file_name(name: *mut core::ffi::c_char, buf: *mut core::ffi::c_char, len: i32) -> i32;
    pub fn set_umid(name: *mut core::ffi::c_char) -> i32;
    pub fn get_umid() -> *mut core::ffi::c_char;
}

#[repr(C)] pub struct os_helper_thread { _private: [u8; 0] }

extern "C" {
    pub fn get_host_cpu_features(flags_helper_func: Option<unsafe extern "C" fn(*mut core::ffi::c_char), cache_helper_func: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>);
    pub fn run_helper(pre_exec: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pre_data: *mut core::ffi::c_void, argv: *mut *mut core::ffi::c_char) -> i32;
    pub fn run_helper_thread(proc: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, arg: *mut core::ffi::c_void, flags: u32, stack_out: *mut usize) -> i32;
    pub fn os_run_helper_thread(td_out: *mut *mut os_helper_thread, routine: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void>, arg: *mut core::ffi::c_void) -> i32;
    pub fn os_kill_helper_thread(td: *mut os_helper_thread);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
