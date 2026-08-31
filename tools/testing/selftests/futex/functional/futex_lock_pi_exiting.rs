// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 *
 * futex_lock_pi_exiting.c
 *
 * Coverage for the FUTEX_LOCK_PI owner-exiting path.  futex_wait_timeout.c
 * already covers FUTEX_LOCK_PI timeout semantics and robust_list.c covers
 * owner death via the robust list, but nothing exercises FUTEX_LOCK_PI when a
 * non-robust PI owner exits while holding the lock, nor the basic ownership /
 * EDEADLK / unlock word semantics.
 *
 * DESCRIPTION
 *      Three tests:
 *
 *      1. lock_unlock_basic - uncontended FUTEX_LOCK_PI semantics: the futex
 *         word carries the owner TID, a recursive lock by the owner returns
 *         EDEADLK, and FUTEX_UNLOCK_PI clears the word.
 *
 *      2. owner_dies_with_blocked_waiter - a thread acquires a PI futex and
 *         exits while holding it.  do_exit() runs futex_cleanup_begin() (which
 *         flips the task's futex state to FUTEX_STATE_EXITING) and
 *         exit_pi_state_list() (which hands off / tears down the pi_state).  A
 *         contending FUTEX_LOCK_PI waiter must end up in one of:
 *
 *           0          - ownership was transferred to / acquired by the waiter
 *           EOWNERDEAD - previous owner died holding the lock; the caller is
 *                        now the owner and must acknowledge by unlocking
 *           ESRCH      - the owner encoded in the futex word is already gone
 *
 *         and on the first two it must actually own the lock afterwards.
 *
 *      3. stress_owner_exits - hammer that same exiting-owner path.  This is
 *         where the following bug lived: the 'exiting' task pointer was not
 *         reset at the retry label, so after wait_for_owner_exiting() dropped
 *         its reference a subsequent retry that returned a non-EBUSY error fed
 *         the stale pointer back in and tripped WARN_ON_ONCE(exiting).  That
 *         warning is invisible to user space, so this test cannot observe it
 *         through a syscall return value; it only becomes a visible failure
 *         (crash) on a kernel booted with panic_on_warn=1 (or built with
 *         CONFIG_BUG_ON_DATA_CORRUPTION).  The loop drives the path so that
 *         such a kernel trips on it - the canonical way fuzz/CI catch these.
 *
 *        Fix:    210d36d892de ("futex: Clear stale exiting pointer in
 *                              futex_lock_pi() retry path")
 *        Fixes:  3ef240eaff36 ("futex: Prevent exit livelock")
 *
 * AUTHOR
 *      Based on futex test boilerplate by Darren Hart <dvhart@linux.intel.com>
 *
 *****************************************************************************/

// C dependencies: errno.h, pthread.h, stdint.h, string.h, unistd.h,
// sys/syscall.h, futextest.h, kselftest_harness.h.

use core::ffi::{c_int, c_long, c_void};
use core::ptr;

type futex_t = u32;
type pid_t = i32;
type pthread_t = usize;

#[repr(C)]
struct pthread_barrier_t {
	_private: [usize; 0],
}

const STRESS_ITERS: c_int = 1000;

extern "C" {
	static mut errno: c_int;

	static FUTEX_PRIVATE_FLAG: c_int;
	static FUTEX_TID_MASK: futex_t;
	static FUTEX_INITIALIZER: futex_t;

	static EDEADLK: c_int;
	static EOWNERDEAD: c_int;
	static ESRCH: c_int;
	static SYS_gettid: c_long;

	fn syscall(number: c_long, ...) -> c_long;
	fn strerror(errnum: c_int) -> *mut i8;
	fn usleep(usec: u32) -> c_int;

	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn pthread_exit(retval: *mut c_void) -> !;
	fn pthread_barrier_init(
		barrier: *mut pthread_barrier_t,
		attr: *const c_void,
		count: u32,
	) -> c_int;
	fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
	fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;

	fn futex_lock_pi(
		uaddr: *mut futex_t,
		timeout: *const c_void,
		detect: c_int,
		opflags: c_int,
	) -> c_int;
	fn futex_unlock_pi(uaddr: *mut futex_t, opflags: c_int) -> c_int;
}

extern "Rust" {
	fn TH_LOG(fmt: *const i8, ...);
}

macro_rules! ASSERT_EQ {
	($left:expr, $right:expr) => {
		assert_eq!($left, $right)
	};
}

macro_rules! ASSERT_TRUE {
	($cond:expr) => {
		assert!($cond)
	};
}

static mut pi_futex: futex_t = 0;
static mut locked_barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };
static mut release_barrier: pthread_barrier_t = pthread_barrier_t { _private: [] };

unsafe fn sys_gettid() -> pid_t {
	syscall(SYS_gettid) as pid_t
}

/*
 * Owner thread: acquire the PI futex and exit while still holding it.  Two
 * modes:
 *   park == 0: signal that we hold the lock, then exit immediately (racy; the
 *              waiter races against our exit path).
 *   park == 1: signal that we hold the lock and keep holding until released
 *              via release_barrier, so a waiter has time to contend as a real
 *              PI waiter before we die.
 */
unsafe extern "C" fn owner_thread(arg: *mut c_void) -> *mut c_void {
	let park = arg as c_long;

	if futex_lock_pi(&mut pi_futex, ptr::null(), 0, FUTEX_PRIVATE_FLAG) != 0 {
		return (-errno as isize) as *mut c_void;
	}

	pthread_barrier_wait(&mut locked_barrier);

	if park != 0 {
		pthread_barrier_wait(&mut release_barrier);
	}

	/* Die while still holding the lock. */
	pthread_exit(ptr::null_mut());
}

/*
 * Block on the PI futex as a waiter.  Returns 0 on acquisition, otherwise the
 * positive errno.
 */
unsafe fn waiter_lock_pi() -> c_int {
	let ret = futex_lock_pi(&mut pi_futex, ptr::null(), 0, FUTEX_PRIVATE_FLAG);

	if ret == 0 { 0 } else { errno }
}

unsafe fn outcome_ok(outcome: c_int) -> c_int {
	(outcome == 0 || outcome == EOWNERDEAD || outcome == ESRCH) as c_int
}

/* Results published by waiter_thread() for the owning thread to assert on. */
static mut waiter_outcome: c_int = 0;
static mut waiter_owns: c_int = 0;

/*
 * Waiter thread for the blocked-waiter test.  Contends for the lock and, when
 * it acquires, records whether the futex word actually carries its TID and
 * releases the lock itself (FUTEX_UNLOCK_PI must run in the owning thread).
 */
unsafe extern "C" fn waiter_thread(_arg: *mut c_void) -> *mut c_void {
	let tid: pid_t = sys_gettid();

	waiter_outcome = waiter_lock_pi();
	if waiter_outcome == 0 || waiter_outcome == EOWNERDEAD {
		waiter_owns = ((pi_futex & FUTEX_TID_MASK) == tid as futex_t) as c_int;
		futex_unlock_pi(&mut pi_futex, FUTEX_PRIVATE_FLAG);
	}
	ptr::null_mut()
}

struct lock_pi_exiting {}

unsafe fn lock_pi_exiting_setup() {}

unsafe fn lock_pi_exiting_teardown() {}

/*
 * Uncontended FUTEX_LOCK_PI semantics, fully deterministic.
 */
unsafe fn lock_pi_exiting_lock_unlock_basic() {
	let tid: pid_t = sys_gettid();
	let mut ret: c_int;

	pi_futex = FUTEX_INITIALIZER;

	/* Acquire: we become the owner, our TID lands in the futex word. */
	ret = futex_lock_pi(&mut pi_futex, ptr::null(), 0, FUTEX_PRIVATE_FLAG);
	ASSERT_EQ!(ret, 0);
	if ret != 0 {
		TH_LOG(
			b"lock failed: errno=%d (%s)\0".as_ptr() as *const i8,
			errno,
			strerror(errno),
		);
	}
	ASSERT_EQ!(pi_futex & FUTEX_TID_MASK, tid as futex_t);
	if (pi_futex & FUTEX_TID_MASK) != tid as futex_t {
		TH_LOG(
			b"owner TID not in futex word: 0x%08x\0".as_ptr() as *const i8,
			pi_futex,
		);
	}

	/* A recursive lock by the owner must be refused, not deadlock. */
	errno = 0;
	ret = futex_lock_pi(&mut pi_futex, ptr::null(), 0, FUTEX_PRIVATE_FLAG);
	ASSERT_EQ!(ret, -1);
	ASSERT_EQ!(errno, EDEADLK);
	if errno != EDEADLK {
		TH_LOG(
			b"recursive lock: expected EDEADLK, got errno=%d\0".as_ptr() as *const i8,
			errno,
		);
	}

	/* Release: the futex word is handed back clean. */
	ret = futex_unlock_pi(&mut pi_futex, FUTEX_PRIVATE_FLAG);
	ASSERT_EQ!(ret, 0);
	if ret != 0 {
		TH_LOG(b"unlock failed: errno=%d\0".as_ptr() as *const i8, errno);
	}
	ASSERT_EQ!(pi_futex, 0 as futex_t);
	if pi_futex != 0 as futex_t {
		TH_LOG(
			b"futex word not cleared after unlock: 0x%08x\0".as_ptr() as *const i8,
			pi_futex,
		);
	}
}

/*
 * A PI waiter inherits the lock when the owner dies holding it.
 *
 * The owner parks while holding the lock, this thread contends for it, then
 * the owner exits.  The waiter must come out cleanly (no hang, no unexpected
 * error) and, when it acquires, must actually own the lock.
 */
unsafe fn lock_pi_exiting_owner_dies_with_blocked_waiter() {
	let mut owner: pthread_t = 0;
	let mut waiter: pthread_t = 0;

	pthread_barrier_init(&mut locked_barrier, ptr::null(), 2);
	pthread_barrier_init(&mut release_barrier, ptr::null(), 2);
	pi_futex = FUTEX_INITIALIZER;
	waiter_outcome = -1;
	waiter_owns = 0;

	ASSERT_EQ!(
		pthread_create(&mut owner, ptr::null(), owner_thread, 1 as *mut c_void),
		0
	);

	/* Wait until the owner actually holds the lock. */
	pthread_barrier_wait(&mut locked_barrier);

	/* Start the waiter and give it time to block as a real PI waiter. */
	ASSERT_EQ!(
		pthread_create(&mut waiter, ptr::null(), waiter_thread, ptr::null_mut()),
		0
	);
	usleep(1000);

	/* Release the owner so it dies while the waiter is queued on it. */
	pthread_barrier_wait(&mut release_barrier);

	pthread_join(waiter, ptr::null_mut());
	pthread_join(owner, ptr::null_mut());

	ASSERT_TRUE!(outcome_ok(waiter_outcome) != 0);
	if outcome_ok(waiter_outcome) == 0 {
		TH_LOG(
			b"unexpected FUTEX_LOCK_PI outcome: %d (%s)\0".as_ptr() as *const i8,
			waiter_outcome,
			strerror(waiter_outcome),
		);
	}
	if waiter_outcome == 0 || waiter_outcome == EOWNERDEAD {
		ASSERT_TRUE!(waiter_owns != 0);
		if waiter_owns == 0 {
			TH_LOG(b"waiter acquired but futex word lacks its TID\0".as_ptr() as *const i8);
		}
	}

	pthread_barrier_destroy(&mut locked_barrier);
	pthread_barrier_destroy(&mut release_barrier);
}

/*
 * Stress: repeatedly let an owner exit while a waiter contends for the lock.
 *
 * Each iteration drives the FUTEX_STATE_EXITING -> -EBUSY -> retry path that
 * the stale-'exiting'-pointer bug lived on (210d36d892de).  The warning it
 * fixed is invisible to user space, so on a normally-configured kernel both
 * the buggy and fixed kernels pass here; the point is to make a kernel booted
 * with panic_on_warn=1 trip during one of these iterations.
 */
unsafe fn lock_pi_exiting_stress_owner_exits() {
	let mut i: c_int = 0;
	while i < STRESS_ITERS {
		let mut owner: pthread_t = 0;
		let outcome: c_int;

		pthread_barrier_init(&mut locked_barrier, ptr::null(), 2);
		pi_futex = FUTEX_INITIALIZER;

		ASSERT_EQ!(
			pthread_create(&mut owner, ptr::null(), owner_thread, ptr::null_mut()),
			0
		);

		/* Owner holds the lock; race FUTEX_LOCK_PI against its exit. */
		pthread_barrier_wait(&mut locked_barrier);

		outcome = waiter_lock_pi();
		ASSERT_TRUE!(outcome_ok(outcome) != 0);
		if outcome_ok(outcome) == 0 {
			TH_LOG(
				b"iter %d: unexpected outcome %d (%s)\0".as_ptr() as *const i8,
				i,
				outcome,
				strerror(outcome),
			);
		}
		if outcome == 0 || outcome == EOWNERDEAD {
			futex_unlock_pi(&mut pi_futex, FUTEX_PRIVATE_FLAG);
		}

		pthread_join(owner, ptr::null_mut());
		pthread_barrier_destroy(&mut locked_barrier);
		i += 1;
	}
}

fn main() {
	// TEST_HARNESS_MAIN
}
