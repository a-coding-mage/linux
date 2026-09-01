/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from include/uapi/asm-generic/unistd.h. */
/* Dependency intent: original C header included <asm/bitsperlong.h> for __BITS_PER_LONG. */


/*
 * This file contains the system call numbers, based on the
 * layout of the x86-64 architecture, which embeds the
 * pointer to the syscall in the table.
 *
 * As a basic principle, no duplication of functionality
 * should be added, e.g. we don't use lseek when llseek
 * is present. New architectures should use this file
 * and implement the less feature-full calls in user space.
 */

/* If __SYSCALL is not supplied by the including context, the C header defines it as a no-op. */

/* #if __BITS_PER_LONG == 32 || defined(__SYSCALL_COMPAT) */
/* #define __SC_3264(_nr, _32, _64) __SYSCALL(_nr, _32) */
/* #else */
/* #define __SC_3264(_nr, _32, _64) __SYSCALL(_nr, _64) */
/* #endif */

/* #ifdef __SYSCALL_COMPAT */
/* #define __SC_COMP(_nr, _sys, _comp) __SYSCALL(_nr, _comp) */
/* #define __SC_COMP_3264(_nr, _32, _64, _comp) __SYSCALL(_nr, _comp) */
/* #else */
/* #define __SC_COMP(_nr, _sys, _comp) __SYSCALL(_nr, _sys) */
/* #define __SC_COMP_3264(_nr, _32, _64, _comp) __SC_3264(_nr, _32, _64) */
/* #endif */

pub const __NR_io_setup: u32 = 0;
/* __SC_COMP(__NR_io_setup, sys_io_setup, compat_sys_io_setup) */
pub const __NR_io_destroy: u32 = 1;
/* __SYSCALL(__NR_io_destroy, sys_io_destroy) */
pub const __NR_io_submit: u32 = 2;
/* __SC_COMP(__NR_io_submit, sys_io_submit, compat_sys_io_submit) */
pub const __NR_io_cancel: u32 = 3;
/* __SYSCALL(__NR_io_cancel, sys_io_cancel) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_io_getevents: u32 = 4;
/* __SC_3264(__NR_io_getevents, sys_io_getevents_time32, sys_io_getevents) */
/* #endif */

pub const __NR_setxattr: u32 = 5;
/* __SYSCALL(__NR_setxattr, sys_setxattr) */
pub const __NR_lsetxattr: u32 = 6;
/* __SYSCALL(__NR_lsetxattr, sys_lsetxattr) */
pub const __NR_fsetxattr: u32 = 7;
/* __SYSCALL(__NR_fsetxattr, sys_fsetxattr) */
pub const __NR_getxattr: u32 = 8;
/* __SYSCALL(__NR_getxattr, sys_getxattr) */
pub const __NR_lgetxattr: u32 = 9;
/* __SYSCALL(__NR_lgetxattr, sys_lgetxattr) */
pub const __NR_fgetxattr: u32 = 10;
/* __SYSCALL(__NR_fgetxattr, sys_fgetxattr) */
pub const __NR_listxattr: u32 = 11;
/* __SYSCALL(__NR_listxattr, sys_listxattr) */
pub const __NR_llistxattr: u32 = 12;
/* __SYSCALL(__NR_llistxattr, sys_llistxattr) */
pub const __NR_flistxattr: u32 = 13;
/* __SYSCALL(__NR_flistxattr, sys_flistxattr) */
pub const __NR_removexattr: u32 = 14;
/* __SYSCALL(__NR_removexattr, sys_removexattr) */
pub const __NR_lremovexattr: u32 = 15;
/* __SYSCALL(__NR_lremovexattr, sys_lremovexattr) */
pub const __NR_fremovexattr: u32 = 16;
/* __SYSCALL(__NR_fremovexattr, sys_fremovexattr) */
pub const __NR_getcwd: u32 = 17;
/* __SYSCALL(__NR_getcwd, sys_getcwd) */
pub const __NR_lookup_dcookie: u32 = 18;
/* __SYSCALL(__NR_lookup_dcookie, sys_ni_syscall) */
pub const __NR_eventfd2: u32 = 19;
/* __SYSCALL(__NR_eventfd2, sys_eventfd2) */
pub const __NR_epoll_create1: u32 = 20;
/* __SYSCALL(__NR_epoll_create1, sys_epoll_create1) */
pub const __NR_epoll_ctl: u32 = 21;
/* __SYSCALL(__NR_epoll_ctl, sys_epoll_ctl) */
pub const __NR_epoll_pwait: u32 = 22;
/* __SC_COMP(__NR_epoll_pwait, sys_epoll_pwait, compat_sys_epoll_pwait) */
pub const __NR_dup: u32 = 23;
/* __SYSCALL(__NR_dup, sys_dup) */
pub const __NR_dup3: u32 = 24;
/* __SYSCALL(__NR_dup3, sys_dup3) */
pub const __NR3264_fcntl: u32 = 25;
/* __SC_COMP_3264(__NR3264_fcntl, sys_fcntl64, sys_fcntl, compat_sys_fcntl64) */
pub const __NR_inotify_init1: u32 = 26;
/* __SYSCALL(__NR_inotify_init1, sys_inotify_init1) */
pub const __NR_inotify_add_watch: u32 = 27;
/* __SYSCALL(__NR_inotify_add_watch, sys_inotify_add_watch) */
pub const __NR_inotify_rm_watch: u32 = 28;
/* __SYSCALL(__NR_inotify_rm_watch, sys_inotify_rm_watch) */
pub const __NR_ioctl: u32 = 29;
/* __SC_COMP(__NR_ioctl, sys_ioctl, compat_sys_ioctl) */
pub const __NR_ioprio_set: u32 = 30;
/* __SYSCALL(__NR_ioprio_set, sys_ioprio_set) */
pub const __NR_ioprio_get: u32 = 31;
/* __SYSCALL(__NR_ioprio_get, sys_ioprio_get) */
pub const __NR_flock: u32 = 32;
/* __SYSCALL(__NR_flock, sys_flock) */
pub const __NR_mknodat: u32 = 33;
/* __SYSCALL(__NR_mknodat, sys_mknodat) */
pub const __NR_mkdirat: u32 = 34;
/* __SYSCALL(__NR_mkdirat, sys_mkdirat) */
pub const __NR_unlinkat: u32 = 35;
/* __SYSCALL(__NR_unlinkat, sys_unlinkat) */
pub const __NR_symlinkat: u32 = 36;
/* __SYSCALL(__NR_symlinkat, sys_symlinkat) */
pub const __NR_linkat: u32 = 37;
/* __SYSCALL(__NR_linkat, sys_linkat) */

/* #ifdef __ARCH_WANT_RENAMEAT */
/* renameat is superseded with flags by renameat2 */
pub const __NR_renameat: u32 = 38;
/* __SYSCALL(__NR_renameat, sys_renameat) */
/* #endif /* __ARCH_WANT_RENAMEAT */ */

pub const __NR_umount2: u32 = 39;
/* __SYSCALL(__NR_umount2, sys_umount) */
pub const __NR_mount: u32 = 40;
/* __SYSCALL(__NR_mount, sys_mount) */
pub const __NR_pivot_root: u32 = 41;
/* __SYSCALL(__NR_pivot_root, sys_pivot_root) */
pub const __NR_nfsservctl: u32 = 42;
/* __SYSCALL(__NR_nfsservctl, sys_ni_syscall) */
pub const __NR3264_statfs: u32 = 43;
/* __SC_COMP_3264(__NR3264_statfs, sys_statfs64, sys_statfs, \ */
	       compat_sys_statfs64)
pub const __NR3264_fstatfs: u32 = 44;
/* __SC_COMP_3264(__NR3264_fstatfs, sys_fstatfs64, sys_fstatfs, \ */
	       compat_sys_fstatfs64)
pub const __NR3264_truncate: u32 = 45;
/* __SC_COMP_3264(__NR3264_truncate, sys_truncate64, sys_truncate, \ */
	       compat_sys_truncate64)
pub const __NR3264_ftruncate: u32 = 46;
/* __SC_COMP_3264(__NR3264_ftruncate, sys_ftruncate64, sys_ftruncate, \ */
	       compat_sys_ftruncate64)
pub const __NR_fallocate: u32 = 47;
/* __SC_COMP(__NR_fallocate, sys_fallocate, compat_sys_fallocate) */
pub const __NR_faccessat: u32 = 48;
/* __SYSCALL(__NR_faccessat, sys_faccessat) */
pub const __NR_chdir: u32 = 49;
/* __SYSCALL(__NR_chdir, sys_chdir) */
pub const __NR_fchdir: u32 = 50;
/* __SYSCALL(__NR_fchdir, sys_fchdir) */
pub const __NR_chroot: u32 = 51;
/* __SYSCALL(__NR_chroot, sys_chroot) */
pub const __NR_fchmod: u32 = 52;
/* __SYSCALL(__NR_fchmod, sys_fchmod) */
pub const __NR_fchmodat: u32 = 53;
/* __SYSCALL(__NR_fchmodat, sys_fchmodat) */
pub const __NR_fchownat: u32 = 54;
/* __SYSCALL(__NR_fchownat, sys_fchownat) */
pub const __NR_fchown: u32 = 55;
/* __SYSCALL(__NR_fchown, sys_fchown) */
pub const __NR_openat: u32 = 56;
/* __SYSCALL(__NR_openat, sys_openat) */
pub const __NR_close: u32 = 57;
/* __SYSCALL(__NR_close, sys_close) */
pub const __NR_vhangup: u32 = 58;
/* __SYSCALL(__NR_vhangup, sys_vhangup) */
pub const __NR_pipe2: u32 = 59;
/* __SYSCALL(__NR_pipe2, sys_pipe2) */
pub const __NR_quotactl: u32 = 60;
/* __SYSCALL(__NR_quotactl, sys_quotactl) */
pub const __NR_getdents64: u32 = 61;
/* __SYSCALL(__NR_getdents64, sys_getdents64) */
pub const __NR3264_lseek: u32 = 62;
/* __SC_3264(__NR3264_lseek, sys_llseek, sys_lseek) */
pub const __NR_read: u32 = 63;
/* __SYSCALL(__NR_read, sys_read) */
pub const __NR_write: u32 = 64;
/* __SYSCALL(__NR_write, sys_write) */
pub const __NR_readv: u32 = 65;
/* __SC_COMP(__NR_readv, sys_readv, sys_readv) */
pub const __NR_writev: u32 = 66;
/* __SC_COMP(__NR_writev, sys_writev, sys_writev) */
pub const __NR_pread64: u32 = 67;
/* __SC_COMP(__NR_pread64, sys_pread64, compat_sys_pread64) */
pub const __NR_pwrite64: u32 = 68;
/* __SC_COMP(__NR_pwrite64, sys_pwrite64, compat_sys_pwrite64) */
pub const __NR_preadv: u32 = 69;
/* __SC_COMP(__NR_preadv, sys_preadv, compat_sys_preadv) */
pub const __NR_pwritev: u32 = 70;
/* __SC_COMP(__NR_pwritev, sys_pwritev, compat_sys_pwritev) */
pub const __NR3264_sendfile: u32 = 71;
/* __SYSCALL(__NR3264_sendfile, sys_sendfile64) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_pselect6: u32 = 72;
/* __SC_COMP_3264(__NR_pselect6, sys_pselect6_time32, sys_pselect6, compat_sys_pselect6_time32) */
pub const __NR_ppoll: u32 = 73;
/* __SC_COMP_3264(__NR_ppoll, sys_ppoll_time32, sys_ppoll, compat_sys_ppoll_time32) */
/* #endif */

pub const __NR_signalfd4: u32 = 74;
/* __SC_COMP(__NR_signalfd4, sys_signalfd4, compat_sys_signalfd4) */
pub const __NR_vmsplice: u32 = 75;
/* __SYSCALL(__NR_vmsplice, sys_vmsplice) */
pub const __NR_splice: u32 = 76;
/* __SYSCALL(__NR_splice, sys_splice) */
pub const __NR_tee: u32 = 77;
/* __SYSCALL(__NR_tee, sys_tee) */
pub const __NR_readlinkat: u32 = 78;
/* __SYSCALL(__NR_readlinkat, sys_readlinkat) */

/* #if defined(__ARCH_WANT_NEW_STAT) || defined(__ARCH_WANT_STAT64) */
pub const __NR3264_fstatat: u32 = 79;
/* __SC_3264(__NR3264_fstatat, sys_fstatat64, sys_newfstatat) */
pub const __NR3264_fstat: u32 = 80;
/* __SC_3264(__NR3264_fstat, sys_fstat64, sys_newfstat) */
/* #endif */

pub const __NR_sync: u32 = 81;
/* __SYSCALL(__NR_sync, sys_sync) */
pub const __NR_fsync: u32 = 82;
/* __SYSCALL(__NR_fsync, sys_fsync) */
pub const __NR_fdatasync: u32 = 83;
/* __SYSCALL(__NR_fdatasync, sys_fdatasync) */

/* #ifdef __ARCH_WANT_SYNC_FILE_RANGE2 */
pub const __NR_sync_file_range2: u32 = 84;
/* __SC_COMP(__NR_sync_file_range2, sys_sync_file_range2, \ */
	  compat_sys_sync_file_range2)
/* #else */
pub const __NR_sync_file_range: u32 = 84;
/* __SC_COMP(__NR_sync_file_range, sys_sync_file_range, \ */
	  compat_sys_sync_file_range)
/* #endif */

pub const __NR_timerfd_create: u32 = 85;
/* __SYSCALL(__NR_timerfd_create, sys_timerfd_create) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_timerfd_settime: u32 = 86;
/* __SC_3264(__NR_timerfd_settime, sys_timerfd_settime32, \ */
	  sys_timerfd_settime)
pub const __NR_timerfd_gettime: u32 = 87;
/* __SC_3264(__NR_timerfd_gettime, sys_timerfd_gettime32, \ */
	  sys_timerfd_gettime)
/* #endif */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_utimensat: u32 = 88;
/* __SC_3264(__NR_utimensat, sys_utimensat_time32, sys_utimensat) */
/* #endif */

pub const __NR_acct: u32 = 89;
/* __SYSCALL(__NR_acct, sys_acct) */
pub const __NR_capget: u32 = 90;
/* __SYSCALL(__NR_capget, sys_capget) */
pub const __NR_capset: u32 = 91;
/* __SYSCALL(__NR_capset, sys_capset) */
pub const __NR_personality: u32 = 92;
/* __SYSCALL(__NR_personality, sys_personality) */
pub const __NR_exit: u32 = 93;
/* __SYSCALL(__NR_exit, sys_exit) */
pub const __NR_exit_group: u32 = 94;
/* __SYSCALL(__NR_exit_group, sys_exit_group) */
pub const __NR_waitid: u32 = 95;
/* __SC_COMP(__NR_waitid, sys_waitid, compat_sys_waitid) */
pub const __NR_set_tid_address: u32 = 96;
/* __SYSCALL(__NR_set_tid_address, sys_set_tid_address) */
pub const __NR_unshare: u32 = 97;
/* __SYSCALL(__NR_unshare, sys_unshare) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_futex: u32 = 98;
/* __SC_3264(__NR_futex, sys_futex_time32, sys_futex) */
/* #endif */

pub const __NR_set_robust_list: u32 = 99;
/* __SC_COMP(__NR_set_robust_list, sys_set_robust_list, \ */
	  compat_sys_set_robust_list)
pub const __NR_get_robust_list: u32 = 100;
/* __SC_COMP(__NR_get_robust_list, sys_get_robust_list, \ */
	  compat_sys_get_robust_list)

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_nanosleep: u32 = 101;
/* __SC_3264(__NR_nanosleep, sys_nanosleep_time32, sys_nanosleep) */
/* #endif */

pub const __NR_getitimer: u32 = 102;
/* __SC_COMP(__NR_getitimer, sys_getitimer, compat_sys_getitimer) */
pub const __NR_setitimer: u32 = 103;
/* __SC_COMP(__NR_setitimer, sys_setitimer, compat_sys_setitimer) */
pub const __NR_kexec_load: u32 = 104;
/* __SC_COMP(__NR_kexec_load, sys_kexec_load, compat_sys_kexec_load) */
pub const __NR_init_module: u32 = 105;
/* __SYSCALL(__NR_init_module, sys_init_module) */
pub const __NR_delete_module: u32 = 106;
/* __SYSCALL(__NR_delete_module, sys_delete_module) */
pub const __NR_timer_create: u32 = 107;
/* __SC_COMP(__NR_timer_create, sys_timer_create, compat_sys_timer_create) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_timer_gettime: u32 = 108;
/* __SC_3264(__NR_timer_gettime, sys_timer_gettime32, sys_timer_gettime) */
/* #endif */

pub const __NR_timer_getoverrun: u32 = 109;
/* __SYSCALL(__NR_timer_getoverrun, sys_timer_getoverrun) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_timer_settime: u32 = 110;
/* __SC_3264(__NR_timer_settime, sys_timer_settime32, sys_timer_settime) */
/* #endif */

pub const __NR_timer_delete: u32 = 111;
/* __SYSCALL(__NR_timer_delete, sys_timer_delete) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_clock_settime: u32 = 112;
/* __SC_3264(__NR_clock_settime, sys_clock_settime32, sys_clock_settime) */
pub const __NR_clock_gettime: u32 = 113;
/* __SC_3264(__NR_clock_gettime, sys_clock_gettime32, sys_clock_gettime) */
pub const __NR_clock_getres: u32 = 114;
/* __SC_3264(__NR_clock_getres, sys_clock_getres_time32, sys_clock_getres) */
pub const __NR_clock_nanosleep: u32 = 115;
/* __SC_3264(__NR_clock_nanosleep, sys_clock_nanosleep_time32, \ */
	  sys_clock_nanosleep)
/* #endif */

pub const __NR_syslog: u32 = 116;
/* __SYSCALL(__NR_syslog, sys_syslog) */
pub const __NR_ptrace: u32 = 117;
/* __SC_COMP(__NR_ptrace, sys_ptrace, compat_sys_ptrace) */
pub const __NR_sched_setparam: u32 = 118;
/* __SYSCALL(__NR_sched_setparam, sys_sched_setparam) */
pub const __NR_sched_setscheduler: u32 = 119;
/* __SYSCALL(__NR_sched_setscheduler, sys_sched_setscheduler) */
pub const __NR_sched_getscheduler: u32 = 120;
/* __SYSCALL(__NR_sched_getscheduler, sys_sched_getscheduler) */
pub const __NR_sched_getparam: u32 = 121;
/* __SYSCALL(__NR_sched_getparam, sys_sched_getparam) */
pub const __NR_sched_setaffinity: u32 = 122;
/* __SC_COMP(__NR_sched_setaffinity, sys_sched_setaffinity, \ */
	  compat_sys_sched_setaffinity)
pub const __NR_sched_getaffinity: u32 = 123;
/* __SC_COMP(__NR_sched_getaffinity, sys_sched_getaffinity, \ */
	  compat_sys_sched_getaffinity)
pub const __NR_sched_yield: u32 = 124;
/* __SYSCALL(__NR_sched_yield, sys_sched_yield) */
pub const __NR_sched_get_priority_max: u32 = 125;
/* __SYSCALL(__NR_sched_get_priority_max, sys_sched_get_priority_max) */
pub const __NR_sched_get_priority_min: u32 = 126;
/* __SYSCALL(__NR_sched_get_priority_min, sys_sched_get_priority_min) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_sched_rr_get_interval: u32 = 127;
/* __SC_3264(__NR_sched_rr_get_interval, sys_sched_rr_get_interval_time32, \ */
	  sys_sched_rr_get_interval)
/* #endif */

pub const __NR_restart_syscall: u32 = 128;
/* __SYSCALL(__NR_restart_syscall, sys_restart_syscall) */
pub const __NR_kill: u32 = 129;
/* __SYSCALL(__NR_kill, sys_kill) */
pub const __NR_tkill: u32 = 130;
/* __SYSCALL(__NR_tkill, sys_tkill) */
pub const __NR_tgkill: u32 = 131;
/* __SYSCALL(__NR_tgkill, sys_tgkill) */
pub const __NR_sigaltstack: u32 = 132;
/* __SC_COMP(__NR_sigaltstack, sys_sigaltstack, compat_sys_sigaltstack) */
pub const __NR_rt_sigsuspend: u32 = 133;
/* __SC_COMP(__NR_rt_sigsuspend, sys_rt_sigsuspend, compat_sys_rt_sigsuspend) */
pub const __NR_rt_sigaction: u32 = 134;
/* __SC_COMP(__NR_rt_sigaction, sys_rt_sigaction, compat_sys_rt_sigaction) */
pub const __NR_rt_sigprocmask: u32 = 135;
/* __SC_COMP(__NR_rt_sigprocmask, sys_rt_sigprocmask, compat_sys_rt_sigprocmask) */
pub const __NR_rt_sigpending: u32 = 136;
/* __SC_COMP(__NR_rt_sigpending, sys_rt_sigpending, compat_sys_rt_sigpending) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_rt_sigtimedwait: u32 = 137;
/* __SC_COMP_3264(__NR_rt_sigtimedwait, sys_rt_sigtimedwait_time32, \ */
	  sys_rt_sigtimedwait, compat_sys_rt_sigtimedwait_time32)
/* #endif */

pub const __NR_rt_sigqueueinfo: u32 = 138;
/* __SC_COMP(__NR_rt_sigqueueinfo, sys_rt_sigqueueinfo, \ */
	  compat_sys_rt_sigqueueinfo)
pub const __NR_rt_sigreturn: u32 = 139;
/* __SC_COMP(__NR_rt_sigreturn, sys_rt_sigreturn, compat_sys_rt_sigreturn) */
pub const __NR_setpriority: u32 = 140;
/* __SYSCALL(__NR_setpriority, sys_setpriority) */
pub const __NR_getpriority: u32 = 141;
/* __SYSCALL(__NR_getpriority, sys_getpriority) */
pub const __NR_reboot: u32 = 142;
/* __SYSCALL(__NR_reboot, sys_reboot) */
pub const __NR_setregid: u32 = 143;
/* __SYSCALL(__NR_setregid, sys_setregid) */
pub const __NR_setgid: u32 = 144;
/* __SYSCALL(__NR_setgid, sys_setgid) */
pub const __NR_setreuid: u32 = 145;
/* __SYSCALL(__NR_setreuid, sys_setreuid) */
pub const __NR_setuid: u32 = 146;
/* __SYSCALL(__NR_setuid, sys_setuid) */
pub const __NR_setresuid: u32 = 147;
/* __SYSCALL(__NR_setresuid, sys_setresuid) */
pub const __NR_getresuid: u32 = 148;
/* __SYSCALL(__NR_getresuid, sys_getresuid) */
pub const __NR_setresgid: u32 = 149;
/* __SYSCALL(__NR_setresgid, sys_setresgid) */
pub const __NR_getresgid: u32 = 150;
/* __SYSCALL(__NR_getresgid, sys_getresgid) */
pub const __NR_setfsuid: u32 = 151;
/* __SYSCALL(__NR_setfsuid, sys_setfsuid) */
pub const __NR_setfsgid: u32 = 152;
/* __SYSCALL(__NR_setfsgid, sys_setfsgid) */
pub const __NR_times: u32 = 153;
/* __SC_COMP(__NR_times, sys_times, compat_sys_times) */
pub const __NR_setpgid: u32 = 154;
/* __SYSCALL(__NR_setpgid, sys_setpgid) */
pub const __NR_getpgid: u32 = 155;
/* __SYSCALL(__NR_getpgid, sys_getpgid) */
pub const __NR_getsid: u32 = 156;
/* __SYSCALL(__NR_getsid, sys_getsid) */
pub const __NR_setsid: u32 = 157;
/* __SYSCALL(__NR_setsid, sys_setsid) */
pub const __NR_getgroups: u32 = 158;
/* __SYSCALL(__NR_getgroups, sys_getgroups) */
pub const __NR_setgroups: u32 = 159;
/* __SYSCALL(__NR_setgroups, sys_setgroups) */
pub const __NR_uname: u32 = 160;
/* __SYSCALL(__NR_uname, sys_newuname) */
pub const __NR_sethostname: u32 = 161;
/* __SYSCALL(__NR_sethostname, sys_sethostname) */
pub const __NR_setdomainname: u32 = 162;
/* __SYSCALL(__NR_setdomainname, sys_setdomainname) */

/* #ifdef __ARCH_WANT_SET_GET_RLIMIT */
/* getrlimit and setrlimit are superseded with prlimit64 */
pub const __NR_getrlimit: u32 = 163;
/* __SC_COMP(__NR_getrlimit, sys_getrlimit, compat_sys_getrlimit) */
pub const __NR_setrlimit: u32 = 164;
/* __SC_COMP(__NR_setrlimit, sys_setrlimit, compat_sys_setrlimit) */
/* #endif */

pub const __NR_getrusage: u32 = 165;
/* __SC_COMP(__NR_getrusage, sys_getrusage, compat_sys_getrusage) */
pub const __NR_umask: u32 = 166;
/* __SYSCALL(__NR_umask, sys_umask) */
pub const __NR_prctl: u32 = 167;
/* __SYSCALL(__NR_prctl, sys_prctl) */
pub const __NR_getcpu: u32 = 168;
/* __SYSCALL(__NR_getcpu, sys_getcpu) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_gettimeofday: u32 = 169;
/* __SC_COMP(__NR_gettimeofday, sys_gettimeofday, compat_sys_gettimeofday) */
pub const __NR_settimeofday: u32 = 170;
/* __SC_COMP(__NR_settimeofday, sys_settimeofday, compat_sys_settimeofday) */
pub const __NR_adjtimex: u32 = 171;
/* __SC_3264(__NR_adjtimex, sys_adjtimex_time32, sys_adjtimex) */
/* #endif */

pub const __NR_getpid: u32 = 172;
/* __SYSCALL(__NR_getpid, sys_getpid) */
pub const __NR_getppid: u32 = 173;
/* __SYSCALL(__NR_getppid, sys_getppid) */
pub const __NR_getuid: u32 = 174;
/* __SYSCALL(__NR_getuid, sys_getuid) */
pub const __NR_geteuid: u32 = 175;
/* __SYSCALL(__NR_geteuid, sys_geteuid) */
pub const __NR_getgid: u32 = 176;
/* __SYSCALL(__NR_getgid, sys_getgid) */
pub const __NR_getegid: u32 = 177;
/* __SYSCALL(__NR_getegid, sys_getegid) */
pub const __NR_gettid: u32 = 178;
/* __SYSCALL(__NR_gettid, sys_gettid) */
pub const __NR_sysinfo: u32 = 179;
/* __SC_COMP(__NR_sysinfo, sys_sysinfo, compat_sys_sysinfo) */
pub const __NR_mq_open: u32 = 180;
/* __SC_COMP(__NR_mq_open, sys_mq_open, compat_sys_mq_open) */
pub const __NR_mq_unlink: u32 = 181;
/* __SYSCALL(__NR_mq_unlink, sys_mq_unlink) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_mq_timedsend: u32 = 182;
/* __SC_3264(__NR_mq_timedsend, sys_mq_timedsend_time32, sys_mq_timedsend) */
pub const __NR_mq_timedreceive: u32 = 183;
/* __SC_3264(__NR_mq_timedreceive, sys_mq_timedreceive_time32, \ */
	  sys_mq_timedreceive)
/* #endif */

pub const __NR_mq_notify: u32 = 184;
/* __SC_COMP(__NR_mq_notify, sys_mq_notify, compat_sys_mq_notify) */
pub const __NR_mq_getsetattr: u32 = 185;
/* __SC_COMP(__NR_mq_getsetattr, sys_mq_getsetattr, compat_sys_mq_getsetattr) */
pub const __NR_msgget: u32 = 186;
/* __SYSCALL(__NR_msgget, sys_msgget) */
pub const __NR_msgctl: u32 = 187;
/* __SC_COMP(__NR_msgctl, sys_msgctl, compat_sys_msgctl) */
pub const __NR_msgrcv: u32 = 188;
/* __SC_COMP(__NR_msgrcv, sys_msgrcv, compat_sys_msgrcv) */
pub const __NR_msgsnd: u32 = 189;
/* __SC_COMP(__NR_msgsnd, sys_msgsnd, compat_sys_msgsnd) */
pub const __NR_semget: u32 = 190;
/* __SYSCALL(__NR_semget, sys_semget) */
pub const __NR_semctl: u32 = 191;
/* __SC_COMP(__NR_semctl, sys_semctl, compat_sys_semctl) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_semtimedop: u32 = 192;
/* __SC_3264(__NR_semtimedop, sys_semtimedop_time32, sys_semtimedop) */
/* #endif */

pub const __NR_semop: u32 = 193;
/* __SYSCALL(__NR_semop, sys_semop) */
pub const __NR_shmget: u32 = 194;
/* __SYSCALL(__NR_shmget, sys_shmget) */
pub const __NR_shmctl: u32 = 195;
/* __SC_COMP(__NR_shmctl, sys_shmctl, compat_sys_shmctl) */
pub const __NR_shmat: u32 = 196;
/* __SC_COMP(__NR_shmat, sys_shmat, compat_sys_shmat) */
pub const __NR_shmdt: u32 = 197;
/* __SYSCALL(__NR_shmdt, sys_shmdt) */
pub const __NR_socket: u32 = 198;
/* __SYSCALL(__NR_socket, sys_socket) */
pub const __NR_socketpair: u32 = 199;
/* __SYSCALL(__NR_socketpair, sys_socketpair) */
pub const __NR_bind: u32 = 200;
/* __SYSCALL(__NR_bind, sys_bind) */
pub const __NR_listen: u32 = 201;
/* __SYSCALL(__NR_listen, sys_listen) */
pub const __NR_accept: u32 = 202;
/* __SYSCALL(__NR_accept, sys_accept) */
pub const __NR_connect: u32 = 203;
/* __SYSCALL(__NR_connect, sys_connect) */
pub const __NR_getsockname: u32 = 204;
/* __SYSCALL(__NR_getsockname, sys_getsockname) */
pub const __NR_getpeername: u32 = 205;
/* __SYSCALL(__NR_getpeername, sys_getpeername) */
pub const __NR_sendto: u32 = 206;
/* __SYSCALL(__NR_sendto, sys_sendto) */
pub const __NR_recvfrom: u32 = 207;
/* __SC_COMP(__NR_recvfrom, sys_recvfrom, compat_sys_recvfrom) */
pub const __NR_setsockopt: u32 = 208;
/* __SC_COMP(__NR_setsockopt, sys_setsockopt, sys_setsockopt) */
pub const __NR_getsockopt: u32 = 209;
/* __SC_COMP(__NR_getsockopt, sys_getsockopt, sys_getsockopt) */
pub const __NR_shutdown: u32 = 210;
/* __SYSCALL(__NR_shutdown, sys_shutdown) */
pub const __NR_sendmsg: u32 = 211;
/* __SC_COMP(__NR_sendmsg, sys_sendmsg, compat_sys_sendmsg) */
pub const __NR_recvmsg: u32 = 212;
/* __SC_COMP(__NR_recvmsg, sys_recvmsg, compat_sys_recvmsg) */
pub const __NR_readahead: u32 = 213;
/* __SC_COMP(__NR_readahead, sys_readahead, compat_sys_readahead) */
pub const __NR_brk: u32 = 214;
/* __SYSCALL(__NR_brk, sys_brk) */
pub const __NR_munmap: u32 = 215;
/* __SYSCALL(__NR_munmap, sys_munmap) */
pub const __NR_mremap: u32 = 216;
/* __SYSCALL(__NR_mremap, sys_mremap) */
pub const __NR_add_key: u32 = 217;
/* __SYSCALL(__NR_add_key, sys_add_key) */
pub const __NR_request_key: u32 = 218;
/* __SYSCALL(__NR_request_key, sys_request_key) */
pub const __NR_keyctl: u32 = 219;
/* __SC_COMP(__NR_keyctl, sys_keyctl, compat_sys_keyctl) */
pub const __NR_clone: u32 = 220;
/* __SYSCALL(__NR_clone, sys_clone) */
pub const __NR_execve: u32 = 221;
/* __SC_COMP(__NR_execve, sys_execve, compat_sys_execve) */
pub const __NR3264_mmap: u32 = 222;
/* __SC_3264(__NR3264_mmap, sys_mmap2, sys_mmap) */
pub const __NR3264_fadvise64: u32 = 223;
/* __SC_COMP(__NR3264_fadvise64, sys_fadvise64_64, compat_sys_fadvise64_64) */

/* CONFIG_MMU only */
/* #ifndef __ARCH_NOMMU */
pub const __NR_swapon: u32 = 224;
/* __SYSCALL(__NR_swapon, sys_swapon) */
pub const __NR_swapoff: u32 = 225;
/* __SYSCALL(__NR_swapoff, sys_swapoff) */
pub const __NR_mprotect: u32 = 226;
/* __SYSCALL(__NR_mprotect, sys_mprotect) */
pub const __NR_msync: u32 = 227;
/* __SYSCALL(__NR_msync, sys_msync) */
pub const __NR_mlock: u32 = 228;
/* __SYSCALL(__NR_mlock, sys_mlock) */
pub const __NR_munlock: u32 = 229;
/* __SYSCALL(__NR_munlock, sys_munlock) */
pub const __NR_mlockall: u32 = 230;
/* __SYSCALL(__NR_mlockall, sys_mlockall) */
pub const __NR_munlockall: u32 = 231;
/* __SYSCALL(__NR_munlockall, sys_munlockall) */
pub const __NR_mincore: u32 = 232;
/* __SYSCALL(__NR_mincore, sys_mincore) */
pub const __NR_madvise: u32 = 233;
/* __SYSCALL(__NR_madvise, sys_madvise) */
pub const __NR_remap_file_pages: u32 = 234;
/* __SYSCALL(__NR_remap_file_pages, sys_remap_file_pages) */
pub const __NR_mbind: u32 = 235;
/* __SYSCALL(__NR_mbind, sys_mbind) */
pub const __NR_get_mempolicy: u32 = 236;
/* __SYSCALL(__NR_get_mempolicy, sys_get_mempolicy) */
pub const __NR_set_mempolicy: u32 = 237;
/* __SYSCALL(__NR_set_mempolicy, sys_set_mempolicy) */
pub const __NR_migrate_pages: u32 = 238;
/* __SYSCALL(__NR_migrate_pages, sys_migrate_pages) */
pub const __NR_move_pages: u32 = 239;
/* __SYSCALL(__NR_move_pages, sys_move_pages) */
/* #endif */

pub const __NR_rt_tgsigqueueinfo: u32 = 240;
/* __SC_COMP(__NR_rt_tgsigqueueinfo, sys_rt_tgsigqueueinfo, \ */
	  compat_sys_rt_tgsigqueueinfo)
pub const __NR_perf_event_open: u32 = 241;
/* __SYSCALL(__NR_perf_event_open, sys_perf_event_open) */
pub const __NR_accept4: u32 = 242;
/* __SYSCALL(__NR_accept4, sys_accept4) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_recvmmsg: u32 = 243;
/* __SC_COMP_3264(__NR_recvmmsg, sys_recvmmsg_time32, sys_recvmmsg, compat_sys_recvmmsg_time32) */
/* #endif */

/*
 * Architectures may provide up to 16 syscalls of their own
 * starting with this value.
 */
pub const __NR_arch_specific_syscall: u32 = 244;

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_wait4: u32 = 260;
/* __SC_COMP(__NR_wait4, sys_wait4, compat_sys_wait4) */
/* #endif */

pub const __NR_prlimit64: u32 = 261;
/* __SYSCALL(__NR_prlimit64, sys_prlimit64) */
pub const __NR_fanotify_init: u32 = 262;
/* __SYSCALL(__NR_fanotify_init, sys_fanotify_init) */
pub const __NR_fanotify_mark: u32 = 263;
/* __SYSCALL(__NR_fanotify_mark, sys_fanotify_mark) */
pub const __NR_name_to_handle_at: u32 = 264;
/* __SYSCALL(__NR_name_to_handle_at, sys_name_to_handle_at) */
pub const __NR_open_by_handle_at: u32 = 265;
/* __SYSCALL(__NR_open_by_handle_at, sys_open_by_handle_at) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_clock_adjtime: u32 = 266;
/* __SC_3264(__NR_clock_adjtime, sys_clock_adjtime32, sys_clock_adjtime) */
/* #endif */

pub const __NR_syncfs: u32 = 267;
/* __SYSCALL(__NR_syncfs, sys_syncfs) */
pub const __NR_setns: u32 = 268;
/* __SYSCALL(__NR_setns, sys_setns) */
pub const __NR_sendmmsg: u32 = 269;
/* __SC_COMP(__NR_sendmmsg, sys_sendmmsg, compat_sys_sendmmsg) */
pub const __NR_process_vm_readv: u32 = 270;
/* __SYSCALL(__NR_process_vm_readv, sys_process_vm_readv) */
pub const __NR_process_vm_writev: u32 = 271;
/* __SYSCALL(__NR_process_vm_writev, sys_process_vm_writev) */
pub const __NR_kcmp: u32 = 272;
/* __SYSCALL(__NR_kcmp, sys_kcmp) */
pub const __NR_finit_module: u32 = 273;
/* __SYSCALL(__NR_finit_module, sys_finit_module) */
pub const __NR_sched_setattr: u32 = 274;
/* __SYSCALL(__NR_sched_setattr, sys_sched_setattr) */
pub const __NR_sched_getattr: u32 = 275;
/* __SYSCALL(__NR_sched_getattr, sys_sched_getattr) */
pub const __NR_renameat2: u32 = 276;
/* __SYSCALL(__NR_renameat2, sys_renameat2) */
pub const __NR_seccomp: u32 = 277;
/* __SYSCALL(__NR_seccomp, sys_seccomp) */
pub const __NR_getrandom: u32 = 278;
/* __SYSCALL(__NR_getrandom, sys_getrandom) */
pub const __NR_memfd_create: u32 = 279;
/* __SYSCALL(__NR_memfd_create, sys_memfd_create) */
pub const __NR_bpf: u32 = 280;
/* __SYSCALL(__NR_bpf, sys_bpf) */
pub const __NR_execveat: u32 = 281;
/* __SC_COMP(__NR_execveat, sys_execveat, compat_sys_execveat) */
pub const __NR_userfaultfd: u32 = 282;
/* __SYSCALL(__NR_userfaultfd, sys_userfaultfd) */
pub const __NR_membarrier: u32 = 283;
/* __SYSCALL(__NR_membarrier, sys_membarrier) */
pub const __NR_mlock2: u32 = 284;
/* __SYSCALL(__NR_mlock2, sys_mlock2) */
pub const __NR_copy_file_range: u32 = 285;
/* __SYSCALL(__NR_copy_file_range, sys_copy_file_range) */
pub const __NR_preadv2: u32 = 286;
/* __SC_COMP(__NR_preadv2, sys_preadv2, compat_sys_preadv2) */
pub const __NR_pwritev2: u32 = 287;
/* __SC_COMP(__NR_pwritev2, sys_pwritev2, compat_sys_pwritev2) */
pub const __NR_pkey_mprotect: u32 = 288;
/* __SYSCALL(__NR_pkey_mprotect, sys_pkey_mprotect) */
pub const __NR_pkey_alloc: u32 = 289;
/* __SYSCALL(__NR_pkey_alloc,    sys_pkey_alloc) */
pub const __NR_pkey_free: u32 = 290;
/* __SYSCALL(__NR_pkey_free,     sys_pkey_free) */
pub const __NR_statx: u32 = 291;
/* __SYSCALL(__NR_statx,     sys_statx) */

/* #if defined(__ARCH_WANT_TIME32_SYSCALLS) || __BITS_PER_LONG != 32 */
pub const __NR_io_pgetevents: u32 = 292;
/* __SC_COMP_3264(__NR_io_pgetevents, sys_io_pgetevents_time32, sys_io_pgetevents, compat_sys_io_pgetevents) */
/* #endif */

pub const __NR_rseq: u32 = 293;
/* __SYSCALL(__NR_rseq, sys_rseq) */
pub const __NR_kexec_file_load: u32 = 294;
/* __SYSCALL(__NR_kexec_file_load,     sys_kexec_file_load) */

/* 295 through 402 are unassigned to sync up with generic numbers, don't use */

/* #if defined(__SYSCALL_COMPAT) || __BITS_PER_LONG == 32 */
pub const __NR_clock_gettime64: u32 = 403;
/* __SYSCALL(__NR_clock_gettime64, sys_clock_gettime) */
pub const __NR_clock_settime64: u32 = 404;
/* __SYSCALL(__NR_clock_settime64, sys_clock_settime) */
pub const __NR_clock_adjtime64: u32 = 405;
/* __SYSCALL(__NR_clock_adjtime64, sys_clock_adjtime) */
pub const __NR_clock_getres_time64: u32 = 406;
/* __SYSCALL(__NR_clock_getres_time64, sys_clock_getres) */
pub const __NR_clock_nanosleep_time64: u32 = 407;
/* __SYSCALL(__NR_clock_nanosleep_time64, sys_clock_nanosleep) */
pub const __NR_timer_gettime64: u32 = 408;
/* __SYSCALL(__NR_timer_gettime64, sys_timer_gettime) */
pub const __NR_timer_settime64: u32 = 409;
/* __SYSCALL(__NR_timer_settime64, sys_timer_settime) */
pub const __NR_timerfd_gettime64: u32 = 410;
/* __SYSCALL(__NR_timerfd_gettime64, sys_timerfd_gettime) */
pub const __NR_timerfd_settime64: u32 = 411;
/* __SYSCALL(__NR_timerfd_settime64, sys_timerfd_settime) */
pub const __NR_utimensat_time64: u32 = 412;
/* __SYSCALL(__NR_utimensat_time64, sys_utimensat) */
pub const __NR_pselect6_time64: u32 = 413;
/* __SC_COMP(__NR_pselect6_time64, sys_pselect6, compat_sys_pselect6_time64) */
pub const __NR_ppoll_time64: u32 = 414;
/* __SC_COMP(__NR_ppoll_time64, sys_ppoll, compat_sys_ppoll_time64) */
pub const __NR_io_pgetevents_time64: u32 = 416;
/* __SC_COMP(__NR_io_pgetevents_time64, sys_io_pgetevents, compat_sys_io_pgetevents_time64) */
pub const __NR_recvmmsg_time64: u32 = 417;
/* __SC_COMP(__NR_recvmmsg_time64, sys_recvmmsg, compat_sys_recvmmsg_time64) */
pub const __NR_mq_timedsend_time64: u32 = 418;
/* __SYSCALL(__NR_mq_timedsend_time64, sys_mq_timedsend) */
pub const __NR_mq_timedreceive_time64: u32 = 419;
/* __SYSCALL(__NR_mq_timedreceive_time64, sys_mq_timedreceive) */
pub const __NR_semtimedop_time64: u32 = 420;
/* __SYSCALL(__NR_semtimedop_time64, sys_semtimedop) */
pub const __NR_rt_sigtimedwait_time64: u32 = 421;
/* __SC_COMP(__NR_rt_sigtimedwait_time64, sys_rt_sigtimedwait, compat_sys_rt_sigtimedwait_time64) */
pub const __NR_futex_time64: u32 = 422;
/* __SYSCALL(__NR_futex_time64, sys_futex) */
pub const __NR_sched_rr_get_interval_time64: u32 = 423;
/* __SYSCALL(__NR_sched_rr_get_interval_time64, sys_sched_rr_get_interval) */
/* #endif */

pub const __NR_pidfd_send_signal: u32 = 424;
/* __SYSCALL(__NR_pidfd_send_signal, sys_pidfd_send_signal) */
pub const __NR_io_uring_setup: u32 = 425;
/* __SYSCALL(__NR_io_uring_setup, sys_io_uring_setup) */
pub const __NR_io_uring_enter: u32 = 426;
/* __SYSCALL(__NR_io_uring_enter, sys_io_uring_enter) */
pub const __NR_io_uring_register: u32 = 427;
/* __SYSCALL(__NR_io_uring_register, sys_io_uring_register) */
pub const __NR_open_tree: u32 = 428;
/* __SYSCALL(__NR_open_tree, sys_open_tree) */
pub const __NR_move_mount: u32 = 429;
/* __SYSCALL(__NR_move_mount, sys_move_mount) */
pub const __NR_fsopen: u32 = 430;
/* __SYSCALL(__NR_fsopen, sys_fsopen) */
pub const __NR_fsconfig: u32 = 431;
/* __SYSCALL(__NR_fsconfig, sys_fsconfig) */
pub const __NR_fsmount: u32 = 432;
/* __SYSCALL(__NR_fsmount, sys_fsmount) */
pub const __NR_fspick: u32 = 433;
/* __SYSCALL(__NR_fspick, sys_fspick) */
pub const __NR_pidfd_open: u32 = 434;
/* __SYSCALL(__NR_pidfd_open, sys_pidfd_open) */
pub const __NR_clone3: u32 = 435;
/* __SYSCALL(__NR_clone3, sys_clone3) */
pub const __NR_close_range: u32 = 436;
/* __SYSCALL(__NR_close_range, sys_close_range) */
pub const __NR_openat2: u32 = 437;
/* __SYSCALL(__NR_openat2, sys_openat2) */
pub const __NR_pidfd_getfd: u32 = 438;
/* __SYSCALL(__NR_pidfd_getfd, sys_pidfd_getfd) */
pub const __NR_faccessat2: u32 = 439;
/* __SYSCALL(__NR_faccessat2, sys_faccessat2) */
pub const __NR_process_madvise: u32 = 440;
/* __SYSCALL(__NR_process_madvise, sys_process_madvise) */
pub const __NR_epoll_pwait2: u32 = 441;
/* __SC_COMP(__NR_epoll_pwait2, sys_epoll_pwait2, compat_sys_epoll_pwait2) */
pub const __NR_mount_setattr: u32 = 442;
/* __SYSCALL(__NR_mount_setattr, sys_mount_setattr) */
pub const __NR_quotactl_fd: u32 = 443;
/* __SYSCALL(__NR_quotactl_fd, sys_quotactl_fd) */
pub const __NR_landlock_create_ruleset: u32 = 444;
/* __SYSCALL(__NR_landlock_create_ruleset, sys_landlock_create_ruleset) */
pub const __NR_landlock_add_rule: u32 = 445;
/* __SYSCALL(__NR_landlock_add_rule, sys_landlock_add_rule) */
pub const __NR_landlock_restrict_self: u32 = 446;
/* __SYSCALL(__NR_landlock_restrict_self, sys_landlock_restrict_self) */

/* #ifdef __ARCH_WANT_MEMFD_SECRET */
pub const __NR_memfd_secret: u32 = 447;
/* __SYSCALL(__NR_memfd_secret, sys_memfd_secret) */
/* #endif */

pub const __NR_process_mrelease: u32 = 448;
/* __SYSCALL(__NR_process_mrelease, sys_process_mrelease) */
pub const __NR_futex_waitv: u32 = 449;
/* __SYSCALL(__NR_futex_waitv, sys_futex_waitv) */
pub const __NR_set_mempolicy_home_node: u32 = 450;
/* __SYSCALL(__NR_set_mempolicy_home_node, sys_set_mempolicy_home_node) */
pub const __NR_cachestat: u32 = 451;
/* __SYSCALL(__NR_cachestat, sys_cachestat) */
pub const __NR_fchmodat2: u32 = 452;
/* __SYSCALL(__NR_fchmodat2, sys_fchmodat2) */
pub const __NR_map_shadow_stack: u32 = 453;
/* __SYSCALL(__NR_map_shadow_stack, sys_map_shadow_stack) */
pub const __NR_futex_wake: u32 = 454;
/* __SYSCALL(__NR_futex_wake, sys_futex_wake) */
pub const __NR_futex_wait: u32 = 455;
/* __SYSCALL(__NR_futex_wait, sys_futex_wait) */
pub const __NR_futex_requeue: u32 = 456;
/* __SYSCALL(__NR_futex_requeue, sys_futex_requeue) */

pub const __NR_statmount: u32 = 457;
/* __SYSCALL(__NR_statmount, sys_statmount) */

pub const __NR_listmount: u32 = 458;
/* __SYSCALL(__NR_listmount, sys_listmount) */

pub const __NR_lsm_get_self_attr: u32 = 459;
/* __SYSCALL(__NR_lsm_get_self_attr, sys_lsm_get_self_attr) */
pub const __NR_lsm_set_self_attr: u32 = 460;
/* __SYSCALL(__NR_lsm_set_self_attr, sys_lsm_set_self_attr) */
pub const __NR_lsm_list_modules: u32 = 461;
/* __SYSCALL(__NR_lsm_list_modules, sys_lsm_list_modules) */

pub const __NR_mseal: u32 = 462;
/* __SYSCALL(__NR_mseal, sys_mseal) */

pub const __NR_setxattrat: u32 = 463;
/* __SYSCALL(__NR_setxattrat, sys_setxattrat) */
pub const __NR_getxattrat: u32 = 464;
/* __SYSCALL(__NR_getxattrat, sys_getxattrat) */
pub const __NR_listxattrat: u32 = 465;
/* __SYSCALL(__NR_listxattrat, sys_listxattrat) */
pub const __NR_removexattrat: u32 = 466;
/* __SYSCALL(__NR_removexattrat, sys_removexattrat) */
pub const __NR_open_tree_attr: u32 = 467;
/* __SYSCALL(__NR_open_tree_attr, sys_open_tree_attr) */

/* fs/inode.c */
pub const __NR_file_getattr: u32 = 468;
/* __SYSCALL(__NR_file_getattr, sys_file_getattr) */
pub const __NR_file_setattr: u32 = 469;
/* __SYSCALL(__NR_file_setattr, sys_file_setattr) */
pub const __NR_listns: u32 = 470;
/* __SYSCALL(__NR_listns, sys_listns) */

pub const __NR_rseq_slice_yield: u32 = 471;
/* __SYSCALL(__NR_rseq_slice_yield, sys_rseq_slice_yield) */

/* fs/open.c */
pub const __NR_fchroot: u32 = 472;
/* __SYSCALL(__NR_fchroot, sys_fchroot) */

/* #undef __NR_syscalls */
pub const __NR_syscalls: u32 = 473;

/*
 * 32 bit systems traditionally used different
 * syscalls for off_t and loff_t arguments, while
 * 64 bit systems only need the off_t version.
 * For new 32 bit platforms, there is no need to
 * implement the old 32 bit off_t syscalls, so
 * they take different names.
 * Here we map the numbers so that both versions
 * use the same syscall table layout.
 */
/* #if __BITS_PER_LONG == 64 && !defined(__SYSCALL_COMPAT) */
pub const __NR_fcntl: u32 = __NR3264_fcntl;
pub const __NR_statfs: u32 = __NR3264_statfs;
pub const __NR_fstatfs: u32 = __NR3264_fstatfs;
pub const __NR_truncate: u32 = __NR3264_truncate;
pub const __NR_ftruncate: u32 = __NR3264_ftruncate;
pub const __NR_lseek: u32 = __NR3264_lseek;
pub const __NR_sendfile: u32 = __NR3264_sendfile;
/* #if defined(__ARCH_WANT_NEW_STAT) || defined(__ARCH_WANT_STAT64) */
pub const __NR_newfstatat: u32 = __NR3264_fstatat;
pub const __NR_fstat: u32 = __NR3264_fstat;
/* #endif */
pub const __NR_mmap: u32 = __NR3264_mmap;
pub const __NR_fadvise64: u32 = __NR3264_fadvise64;
/* #ifdef __NR3264_stat */
pub const __NR_stat: u32 = __NR3264_stat;
pub const __NR_lstat: u32 = __NR3264_lstat;
/* #endif */
/* #else */
pub const __NR_fcntl64: u32 = __NR3264_fcntl;
pub const __NR_statfs64: u32 = __NR3264_statfs;
pub const __NR_fstatfs64: u32 = __NR3264_fstatfs;
pub const __NR_truncate64: u32 = __NR3264_truncate;
pub const __NR_ftruncate64: u32 = __NR3264_ftruncate;
pub const __NR_llseek: u32 = __NR3264_lseek;
pub const __NR_sendfile64: u32 = __NR3264_sendfile;
/* #if defined(__ARCH_WANT_NEW_STAT) || defined(__ARCH_WANT_STAT64) */
pub const __NR_fstatat64: u32 = __NR3264_fstatat;
pub const __NR_fstat64: u32 = __NR3264_fstat;
/* #endif */
pub const __NR_mmap2: u32 = __NR3264_mmap;
pub const __NR_fadvise64_64: u32 = __NR3264_fadvise64;
/* #ifdef __NR3264_stat */
pub const __NR_stat64: u32 = __NR3264_stat;
pub const __NR_lstat64: u32 = __NR3264_lstat;
/* #endif */
/* #endif */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
