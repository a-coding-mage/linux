/* SPDX-License-Identifier: GPL-2.0 */

// Bit fields within a clockid:
//
// The most significant 29 bits hold either a pid or a file descriptor.
//
// Bit 2 indicates whether a cpu clock refers to a thread or a process.
//
// Bits 1 and 0 give the type: PROF=0, VIRT=1, SCHED=2, or FD=3.
//
// A clockid is invalid if bits 2, 1, and 0 are all set.
#[inline]
pub unsafe fn CPUCLOCK_PID(clock: clockid_t) -> pid_t {
    (!(clock >> 3)) as pid_t
}

#[inline]
pub fn CPUCLOCK_PERTHREAD(clock: clockid_t) -> bool {
    (clock & CPUCLOCK_PERTHREAD_MASK as clockid_t) != 0
}

pub const CPUCLOCK_PERTHREAD_MASK: u32 = 4;

#[inline]
pub fn CPUCLOCK_WHICH(clock: clockid_t) -> clockid_t {
    clock & CPUCLOCK_CLOCK_MASK as clockid_t
}

pub const CPUCLOCK_CLOCK_MASK: u32 = 3;
pub const CPUCLOCK_PROF: u32 = 0;
pub const CPUCLOCK_VIRT: u32 = 1;
pub const CPUCLOCK_SCHED: u32 = 2;
pub const CPUCLOCK_MAX: usize = 3;
pub const CLOCKFD: u32 = CPUCLOCK_MAX as u32;
pub const CLOCKFD_MASK: u32 = CPUCLOCK_PERTHREAD_MASK | CPUCLOCK_CLOCK_MASK;

// CONFIG_POSIX_TIMERS controls whether the following fields are present.
#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[repr(C)]
pub struct posix_cputimer_base {
    /// Earliest-expiration cache
    pub nextevt: u64,
    /// timerqueue head for cpu_timers
    pub tqhead: timerqueue_head,
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[repr(C)]
pub struct posix_cputimers {
    /// Base container for posix CPU clocks
    pub bases: [posix_cputimer_base; CPUCLOCK_MAX],
    /// Timers are queued.
    pub timers_active: u32,
    /// Timer expiry is active. Used for process wide timers to avoid multiple task trying to handle expiry concurrently
    pub expiry_active: u32,
}

#[cfg(feature = "CONFIG_POSIX_TIMERS")]
#[repr(C)]
pub struct posix_cputimers_work {
    /// The task work to be scheduled
    pub work: callback_head,
    /// Mutex held around expiry in context of this task work
    pub mutex: mutex,
    /// @work has been scheduled already, no further processing
    pub scheduled: u32,
}

#[cfg(not(feature = "CONFIG_POSIX_TIMERS"))]
#[repr(C)]
pub struct posix_cputimers {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
