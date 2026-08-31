// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original include dependencies:
// linux/compiler.h, sys/types.h, sys/wait.h, sys/user.h, syscall.h, unistd.h,
// stdio.h, stdlib.h, string.h, sys/ptrace.h, asm/ptrace.h, errno.h,
// "debug.h", "tests/tests.h", "arch-tests.h".

use core::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem::offset_of;
use core::ptr;

#[repr(C)]
pub struct test_suite {
	_private: [u8; 0],
}

#[repr(C)]
pub struct user {
	pub u_debugreg: [c_ulong; 8],
}

#[repr(C)]
pub struct user_regs_struct {
	pub rip: c_ulong,
}

type pid_t = c_int;

unsafe extern "C" {
	static PTRACE_TRACEME: c_int;
	static PTRACE_POKEUSER: c_int;
	static PTRACE_CONT: c_int;
	static PTRACE_PEEKUSER: c_int;
	static PTRACE_DETACH: c_int;
	static SIGCONT: c_int;
	static TEST_OK: c_int;
	static TEST_FAIL: c_int;

	fn fork() -> pid_t;
	fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
	fn ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_long;
	fn raise(sig: c_int) -> c_int;
	fn exit(status: c_int) -> !;
	fn WIFEXITED(status: c_int) -> c_int;
}

#[inline(never)]
unsafe fn bp_1() -> c_int {
	pr_debug!("in %s\n", "bp_1");
	0
}

#[inline(never)]
unsafe fn bp_2() -> c_int {
	pr_debug!("in %s\n", "bp_2");
	0
}

unsafe fn spawn_child() -> c_int {
	let child: c_int = unsafe { fork() };

	if child == 0 {
		/*
		 * The child sets itself for as tracee and
		 * waits in signal for parent to trace it,
		 * then it calls bp_1 and quits.
		 */
		let err: c_int = unsafe {
			ptrace(
				PTRACE_TRACEME,
				0,
				ptr::null_mut(),
				ptr::null_mut(),
			) as c_int
		};

		if err != 0 {
			pr_debug!("failed to PTRACE_TRACEME\n");
			unsafe { exit(1) };
		}

		unsafe {
			raise(SIGCONT);
			bp_1();
			exit(0);
		}
	}

	child
}

/*
 * This tests creates HW breakpoint, tries to
 * change it and checks it was properly changed.
 */
unsafe fn bp_modify1() -> c_int {
	let child: pid_t;
	let mut status: c_int = 0;
	let mut rip: c_ulong = 0;
	let dr7: c_ulong = 1;

	child = unsafe { spawn_child() };

	unsafe {
		waitpid(child, &mut status, 0);
	}
	if unsafe { WIFEXITED(status) } != 0 {
		pr_debug!("tracee exited prematurely 1\n");
		return unsafe { TEST_FAIL };
	}

	/*
	 * The parent does following steps:
	 *  - creates a new breakpoint (id 0) for bp_2 function
	 *  - changes that breakpoint to bp_1 function
	 *  - waits for the breakpoint to hit and checks
	 *    it has proper rip of bp_1 function
	 *  - detaches the child
	 */
	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			offset_of!(user, u_debugreg) as *mut c_void,
			bp_2 as usize as *mut c_void,
		)
	} != 0
	{
		pr_debug!("failed to set breakpoint, 1st time: %m\n");
		goto_out_modify1(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			offset_of!(user, u_debugreg) as *mut c_void,
			bp_1 as usize as *mut c_void,
		)
	} != 0
	{
		pr_debug!("failed to set breakpoint, 2nd time: %m\n");
		goto_out_modify1(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			(offset_of!(user, u_debugreg) + 7 * core::mem::size_of::<c_ulong>()) as *mut c_void,
			dr7 as usize as *mut c_void,
		)
	} != 0
	{
		pr_debug!("failed to set dr7: %m\n");
		goto_out_modify1(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe { ptrace(PTRACE_CONT, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_CONT: %m\n");
		goto_out_modify1(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	unsafe {
		waitpid(child, &mut status, 0);
	}
	if unsafe { WIFEXITED(status) } != 0 {
		pr_debug!("tracee exited prematurely 2\n");
		return unsafe { TEST_FAIL };
	}

	rip = unsafe {
		ptrace(
			PTRACE_PEEKUSER,
			child,
			offset_of!(user_regs_struct, rip) as *mut c_void,
			ptr::null_mut(),
		) as c_ulong
	};
	if rip == (-1isize) as c_ulong {
		pr_debug!("failed to PTRACE_PEEKUSER: %m\n");
		goto_out_modify1(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	pr_debug!("rip %lx, bp_1 %p\n", rip, bp_1 as usize as *const c_void);

	if unsafe { ptrace(PTRACE_DETACH, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_DETACH: %m\n");
		return unsafe { TEST_FAIL };
	}
	if rip == bp_1 as usize as c_ulong {
		unsafe { TEST_OK }
	} else {
		unsafe { TEST_FAIL }
	}
}

unsafe fn goto_out_modify1(rip: &mut c_ulong, child: pid_t) {
	let _ = rip;
	if unsafe { ptrace(PTRACE_DETACH, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_DETACH: %m\n");
	}
}

/*
 * This tests creates HW breakpoint, tries to
 * change it to bogus value and checks the original
 * breakpoint is hit.
 */
unsafe fn bp_modify2() -> c_int {
	let child: pid_t;
	let mut status: c_int = 0;
	let mut rip: c_ulong = 0;
	let dr7: c_ulong = 1;

	child = unsafe { spawn_child() };

	unsafe {
		waitpid(child, &mut status, 0);
	}
	if unsafe { WIFEXITED(status) } != 0 {
		pr_debug!("tracee exited prematurely 1\n");
		return unsafe { TEST_FAIL };
	}

	/*
	 * The parent does following steps:
	 *  - creates a new breakpoint (id 0) for bp_1 function
	 *  - tries to change that breakpoint to (-1) address
	 *  - waits for the breakpoint to hit and checks
	 *    it has proper rip of bp_1 function
	 *  - detaches the child
	 */
	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			offset_of!(user, u_debugreg) as *mut c_void,
			bp_1 as usize as *mut c_void,
		)
	} != 0
	{
		pr_debug!("failed to set breakpoint: %m\n");
		goto_out_modify2(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			(offset_of!(user, u_debugreg) + 7 * core::mem::size_of::<c_ulong>()) as *mut c_void,
			dr7 as usize as *mut c_void,
		)
	} != 0
	{
		pr_debug!("failed to set dr7: %m\n");
		goto_out_modify2(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe {
		ptrace(
			PTRACE_POKEUSER,
			child,
			offset_of!(user, u_debugreg) as *mut c_void,
			(-1isize) as c_ulong as usize as *mut c_void,
		)
	} == 0
	{
		pr_debug!("failed, breakpoint set to bogus address\n");
		goto_out_modify2(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	if unsafe { ptrace(PTRACE_CONT, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_CONT: %m\n");
		goto_out_modify2(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	unsafe {
		waitpid(child, &mut status, 0);
	}
	if unsafe { WIFEXITED(status) } != 0 {
		pr_debug!("tracee exited prematurely 2\n");
		return unsafe { TEST_FAIL };
	}

	rip = unsafe {
		ptrace(
			PTRACE_PEEKUSER,
			child,
			offset_of!(user_regs_struct, rip) as *mut c_void,
			ptr::null_mut(),
		) as c_ulong
	};
	if rip == (-1isize) as c_ulong {
		pr_debug!("failed to PTRACE_PEEKUSER: %m\n");
		goto_out_modify2(&mut rip, child);
		return if rip == bp_1 as usize as c_ulong {
			unsafe { TEST_OK }
		} else {
			unsafe { TEST_FAIL }
		};
	}

	pr_debug!("rip %lx, bp_1 %p\n", rip, bp_1 as usize as *const c_void);

	if unsafe { ptrace(PTRACE_DETACH, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_DETACH: %m\n");
		return unsafe { TEST_FAIL };
	}

	if rip == bp_1 as usize as c_ulong {
		unsafe { TEST_OK }
	} else {
		unsafe { TEST_FAIL }
	}
}

unsafe fn goto_out_modify2(rip: &mut c_ulong, child: pid_t) {
	let _ = rip;
	if unsafe { ptrace(PTRACE_DETACH, child, ptr::null_mut(), ptr::null_mut()) } != 0 {
		pr_debug!("failed to PTRACE_DETACH: %m\n");
	}
}

pub unsafe extern "C" fn test__bp_modify(
	test: *mut test_suite,
	subtest: c_int,
) -> c_int {
	let _ = test;
	let _ = subtest;

	TEST_ASSERT_VAL!("modify test 1 failed\n", unsafe { bp_modify1() } == 0);
	TEST_ASSERT_VAL!("modify test 2 failed\n", unsafe { bp_modify2() } == 0);

	0
}
