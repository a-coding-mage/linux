/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::ffi::c_void;

#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut c_void,
}
pub type sigval_t = sigval;

pub const SI_MAX_SIZE: usize = 128;

/* __ARCH_SI_BAND_T defaults to long; architectures may override it. */
pub type __ARCH_SI_BAND_T = isize;
/* __ARCH_SI_CLOCK_T defaults to __kernel_clock_t; architectures may override it. */
pub type __ARCH_SI_CLOCK_T = __kernel_clock_t;

#[repr(C)]
pub union __sifields {
    pub _kill: __sifields_kill,
    pub _timer: __sifields_timer,
    pub _rt: __sifields_rt,
    pub _sigchld: __sifields_sigchld,
    pub _sigfault: __sifields_sigfault,
    pub _sigpoll: __sifields_sigpoll,
    pub _sigsys: __sifields_sigsys,
}

#[repr(C)]
pub struct __sifields_kill {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}

#[repr(C)]
pub struct __sifields_timer {
    pub _tid: __kernel_timer_t,
    pub _overrun: i32,
    pub _sigval: sigval_t,
    pub _sys_private: i32,
}

#[repr(C)]
pub struct __sifields_rt {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}

#[repr(C)]
pub struct __sifields_sigchld {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: i32,
    pub _utime: __ARCH_SI_CLOCK_T,
    pub _stime: __ARCH_SI_CLOCK_T,
}

#[repr(C)]
pub struct __sifields_sigfault {
    pub _addr: *mut c_void,
    pub _addr_union: __sifields_sigfault_addr,
}

#[repr(C)]
pub union __sifields_sigfault_addr {
    pub _trapno: i32,
    pub _addr_lsb: i16,
    pub _addr_bnd: __sifields_addr_bnd,
    pub _addr_pkey: __sifields_addr_pkey,
    pub _perf: __sifields_perf,
}

#[repr(C)]
pub struct __sifields_addr_bnd {
    pub _dummy_bnd: [u8; core::mem::size_of::<*mut c_void>().max(core::mem::size_of::<i16>())],
    pub _lower: *mut c_void,
    pub _upper: *mut c_void,
}

#[repr(C)]
pub struct __sifields_addr_pkey {
    pub _dummy_pkey: [u8; core::mem::size_of::<*mut c_void>().max(core::mem::size_of::<i16>())],
    pub _pkey: u32,
}

#[repr(C)]
pub struct __sifields_perf {
    pub _data: usize,
    pub _type: u32,
    pub _flags: u32,
}

#[repr(C)]
pub struct __sifields_sigpoll {
    pub _band: __ARCH_SI_BAND_T,
    pub _fd: i32,
}

#[repr(C)]
pub struct __sifields_sigsys {
    pub _call_addr: *mut c_void,
    pub _syscall: i32,
    pub _arch: u32,
}

#[repr(C)]
pub struct __siginfo {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub _sifields: __sifields,
}

#[repr(C)]
pub union siginfo_union {
    pub __siginfo: __siginfo,
    pub _si_pad: [i32; SI_MAX_SIZE / core::mem::size_of::<i32>()],
}

#[repr(C)]
pub struct siginfo {
    pub _data: siginfo_union,
}
pub type siginfo_t = siginfo;

/* Field-access macros from the C header are represented by these accessors. */
macro_rules! siginfo_field {
    ($name:ident, $field:ident, $subfield:ident) => {
        pub unsafe fn $name(siptr: *mut siginfo_t) -> *mut _ {
            &mut (*(*siptr)._data.__siginfo._sifields.$field).$subfield
        }
    };
}

pub const SI_USER: i32 = 0;
pub const SI_KERNEL: i32 = 0x80;
pub const SI_QUEUE: i32 = -1;
pub const SI_TIMER: i32 = -2;
pub const SI_MESGQ: i32 = -3;
pub const SI_ASYNCIO: i32 = -4;
pub const SI_SIGIO: i32 = -5;
pub const SI_TKILL: i32 = -6;
pub const SI_DETHREAD: i32 = -7;
pub const SI_ASYNCNL: i32 = -60;

#[inline]
pub unsafe fn SI_FROMUSER(siptr: *const siginfo_t) -> bool { (*siptr)._data.__siginfo.si_code <= 0 }
#[inline]
pub unsafe fn SI_FROMKERNEL(siptr: *const siginfo_t) -> bool { (*siptr)._data.__siginfo.si_code > 0 }

pub const ILL_ILLOPC: i32 = 1;
pub const ILL_ILLOPN: i32 = 2;
pub const ILL_ILLADR: i32 = 3;
pub const ILL_ILLTRP: i32 = 4;
pub const ILL_PRVOPC: i32 = 5;
pub const ILL_PRVREG: i32 = 6;
pub const ILL_COPROC: i32 = 7;
pub const ILL_BADSTK: i32 = 8;
pub const ILL_BADIADDR: i32 = 9;
pub const __ILL_BREAK: i32 = 10;
pub const __ILL_BNDMOD: i32 = 11;
pub const NSIGILL: i32 = 11;

pub const FPE_INTDIV: i32 = 1;
pub const FPE_INTOVF: i32 = 2;
pub const FPE_FLTDIV: i32 = 3;
pub const FPE_FLTOVF: i32 = 4;
pub const FPE_FLTUND: i32 = 5;
pub const FPE_FLTRES: i32 = 6;
pub const FPE_FLTINV: i32 = 7;
pub const FPE_FLTSUB: i32 = 8;
pub const __FPE_DECOVF: i32 = 9;
pub const __FPE_DECDIV: i32 = 10;
pub const __FPE_DECERR: i32 = 11;
pub const __FPE_INVASC: i32 = 12;
pub const __FPE_INVDEC: i32 = 13;
pub const FPE_FLTUNK: i32 = 14;
pub const FPE_CONDTRAP: i32 = 15;
pub const NSIGFPE: i32 = 15;

pub const SEGV_MAPERR: i32 = 1;
pub const SEGV_ACCERR: i32 = 2;
pub const SEGV_BNDERR: i32 = 3;
pub const SEGV_PKUERR: i32 = 4;
pub const SEGV_ACCADI: i32 = 5;
pub const SEGV_ADIDERR: i32 = 6;
pub const SEGV_ADIPERR: i32 = 7;
pub const SEGV_MTEAERR: i32 = 8;
pub const SEGV_MTESERR: i32 = 9;
pub const SEGV_CPERR: i32 = 10;
pub const NSIGSEGV: i32 = 10;

pub const BUS_ADRALN: i32 = 1;
pub const BUS_ADRERR: i32 = 2;
pub const BUS_OBJERR: i32 = 3;
pub const BUS_MCEERR_AR: i32 = 4;
pub const BUS_MCEERR_AO: i32 = 5;
pub const NSIGBUS: i32 = 5;

pub const TRAP_BRKPT: i32 = 1;
pub const TRAP_TRACE: i32 = 2;
pub const TRAP_BRANCH: i32 = 3;
pub const TRAP_HWBKPT: i32 = 4;
pub const TRAP_UNK: i32 = 5;
pub const TRAP_PERF: i32 = 6;
pub const NSIGTRAP: i32 = 6;
pub const TRAP_PERF_FLAG_ASYNC: u32 = 1u32 << 0;

pub const CLD_EXITED: i32 = 1;
pub const CLD_KILLED: i32 = 2;
pub const CLD_DUMPED: i32 = 3;
pub const CLD_TRAPPED: i32 = 4;
pub const CLD_STOPPED: i32 = 5;
pub const CLD_CONTINUED: i32 = 6;
pub const NSIGCHLD: i32 = 6;

pub const POLL_IN: i32 = 1;
pub const POLL_OUT: i32 = 2;
pub const POLL_MSG: i32 = 3;
pub const POLL_ERR: i32 = 4;
pub const POLL_PRI: i32 = 5;
pub const POLL_HUP: i32 = 6;
pub const NSIGPOLL: i32 = 6;
pub const SYS_SECCOMP: i32 = 1;
pub const SYS_USER_DISPATCH: i32 = 2;
pub const NSIGSYS: i32 = 2;
pub const EMT_TAGOVF: i32 = 1;
pub const NSIGEMT: i32 = 1;

pub const SIGEV_SIGNAL: i32 = 0;
pub const SIGEV_NONE: i32 = 1;
pub const SIGEV_THREAD: i32 = 2;
pub const SIGEV_THREAD_ID: i32 = 4;
pub const SIGEV_MAX_SIZE: usize = 64;
pub const __ARCH_SIGEV_PREAMBLE_SIZE: usize = core::mem::size_of::<i32>() * 2 + core::mem::size_of::<sigval_t>();
pub const SIGEV_PAD_SIZE: usize = (SIGEV_MAX_SIZE - __ARCH_SIGEV_PREAMBLE_SIZE) / core::mem::size_of::<i32>();

#[repr(C)]
pub struct sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: i32,
    pub sigev_notify: i32,
    pub _sigev_un: sigevent_union,
}

#[repr(C)]
pub union sigevent_union {
    pub _pad: [i32; SIGEV_PAD_SIZE],
    pub _tid: i32,
    pub _sigev_thread: sigevent_thread,
}

#[repr(C)]
pub struct sigevent_thread {
    pub _function: Option<unsafe extern "C" fn(sigval_t)>,
    pub _attribute: *mut c_void,
}
pub type sigevent_t = sigevent;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
