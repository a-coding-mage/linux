// SPDX-License-Identifier: GPL-2.0

// C source included: errno.h, linux/types.h, poll.h, signal.h, stdbool.h,
// stdio.h, stdlib.h, string.h, syscall.h, sys/wait.h, unistd.h, pidfd.h,
// and kselftest.h.

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};
use core::ptr;

const EAGAIN: c_int = 11;
const EXIT_SUCCESS: c_int = 0;
const POLLIN: c_short = 0x001;
const SIGALRM: c_int = 14;
const SIGKILL: c_int = 9;
const SIG_ERR: usize = usize::MAX;

#[repr(C)]
struct pollfd {
	fd: c_int,
	events: c_short,
	revents: c_short,
}

type sighandler_t = extern "C" fn(c_int);

unsafe extern "C" {
	fn __errno_location() -> *mut c_int;
	fn alarm(seconds: c_uint) -> c_uint;
	fn atoi(nptr: *const c_char) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn exit(status: c_int) -> !;
	fn fork() -> c_int;
	fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
	fn signal(signum: c_int, handler: sighandler_t) -> usize;
	fn sleep(seconds: c_uint) -> c_uint;
	fn strerror(errnum: c_int) -> *mut c_char;
	fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;

	fn sys_pidfd_open(pid: c_int, flags: c_uint) -> c_int;
	fn sys_pidfd_send_signal(
		pidfd: c_int,
		sig: c_int,
		info: *mut c_void,
		flags: c_uint,
	) -> c_int;

	fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
	fn ksft_print_msg(fmt: *const c_char, ...);
	fn ksft_test_result_pass(fmt: *const c_char, ...);
	fn ksft_exit_pass() -> !;
}

static mut timeout: bool = false;

extern "C" fn handle_alarm(sig: c_int) {
	let _ = sig;
	unsafe {
		timeout = true;
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut fds: pollfd = pollfd {
		fd: 0,
		events: 0,
		revents: 0,
	};
	let mut iter: c_int;
	let mut nevents: c_int;
	let mut nr_iterations: c_int = 10000;

	fds.events = POLLIN;

	if argc > 2 {
		ksft_exit_fail_msg(c"Unexpected command line argument\n".as_ptr());
	}

	if argc == 2 {
		nr_iterations = atoi(*argv.add(1));
		if nr_iterations <= 0 {
			ksft_exit_fail_msg(
				c"invalid input parameter %s\n".as_ptr(),
				*argv.add(1),
			);
		}
	}

	ksft_print_msg(
		c"running pidfd poll test for %d iterations\n".as_ptr(),
		nr_iterations,
	);

	iter = 0;
	while iter < nr_iterations {
		let pidfd: c_int;
		let child_pid: c_int = fork();

		if child_pid < 0 {
			if *__errno_location() == EAGAIN {
				iter -= 1;
				iter += 1;
				continue;
			}
			ksft_exit_fail_msg(
				c"%s - failed to fork a child process\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}

		if child_pid == 0 {
			/* Child process just sleeps for a min and exits */
			sleep(60);
			exit(EXIT_SUCCESS);
		}

		/* Parent kills the child and waits for its death */
		pidfd = sys_pidfd_open(child_pid, 0);
		if pidfd < 0 {
			ksft_exit_fail_msg(
				c"%s - pidfd_open failed\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}

		/* Setup 3 sec alarm - plenty of time */
		if signal(SIGALRM, handle_alarm) == SIG_ERR {
			ksft_exit_fail_msg(
				c"%s - signal failed\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}
		alarm(3);

		/* Send SIGKILL to the child */
		if sys_pidfd_send_signal(pidfd, SIGKILL, ptr::null_mut(), 0) != 0 {
			ksft_exit_fail_msg(
				c"%s - pidfd_send_signal failed\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}

		/* Wait for the death notification */
		fds.fd = pidfd;
		nevents = poll(&mut fds, 1, -1);

		/* Check for error conditions */
		if nevents < 0 {
			ksft_exit_fail_msg(
				c"%s - poll failed\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}

		if nevents != 1 {
			ksft_exit_fail_msg(c"unexpected poll result: %d\n".as_ptr(), nevents);
		}

		if (fds.revents & POLLIN) == 0 {
			ksft_exit_fail_msg(
				c"unexpected event type received: 0x%x\n".as_ptr(),
				fds.revents as c_int,
			);
		}

		if timeout {
			ksft_exit_fail_msg(c"death notification wait timeout\n".as_ptr());
		}

		close(pidfd);
		/* Wait for child to prevent zombies */
		if waitpid(child_pid, ptr::null_mut(), 0) < 0 {
			ksft_exit_fail_msg(
				c"%s - waitpid failed\n".as_ptr(),
				strerror(*__errno_location()),
			);
		}

		iter += 1;
	}

	ksft_test_result_pass(c"pidfd poll test: pass\n".as_ptr());
	ksft_exit_pass();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
