/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DEBUG: bool = true;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const PATH_MAX: usize = 4096;
const BUF_SIZE: usize = 4096;
const SIGSTOP: c_int = 19;
const PTRACE_SEIZE: c_ulong = 0x4206;
const PTRACE_INTERRUPT: c_ulong = 0x4207;
const PTRACE_GETSIGINFO: c_ulong = 0x4202;
const PTRACE_DETACH: c_ulong = 17;

#[repr(C)]
struct siginfo_t {
	_data: [u8; 128],
}

#[repr(C)]
struct cgfreezer_test {
	fn_: unsafe extern "C" fn(*const c_char) -> c_int,
	name: *const c_char,
}

unsafe extern "C" {
	static mut stderr: *mut c_void;

	fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn free(ptr: *mut c_void);
	fn fork() -> c_int;
	fn getppid() -> c_int;
	fn kill(pid: c_int, sig: c_int) -> c_int;
	fn ptrace(request: c_ulong, ...) -> c_long;
	fn sleep(seconds: c_uint) -> c_uint;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn usleep(usec: c_uint) -> c_int;
	fn vfork() -> c_int;
	fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;

	fn cg_create(cgroup: *const c_char) -> c_int;
	fn cg_destroy(cgroup: *const c_char) -> c_int;
	fn cg_enter(cgroup: *const c_char, pid: c_int) -> c_int;
	fn cg_find_unified_root(root: *mut c_char, len: usize, mount_info: *mut c_void) -> c_int;
	fn cg_killall(cgroup: *const c_char) -> c_int;
	fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
	fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int;
	fn cg_read_key_long(cgroup: *const c_char, file: *const c_char, key: *const c_char) -> c_long;
	fn cg_read_strstr(cgroup: *const c_char, file: *const c_char, needle: *const c_char) -> c_int;
	fn cg_run_nowait(
		cgroup: *const c_char,
		fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
		arg: *mut c_void,
	) -> c_int;
	fn cg_wait_for(fd: c_int) -> c_int;
	fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
	fn cg_write(cgroup: *const c_char, file: *const c_char, value: *const c_char) -> c_int;
	fn proc_read_text(
		pid: c_int,
		tid: c_int,
		file: *const c_char,
		buf: *mut c_char,
		size: usize,
	) -> c_int;

	fn ksft_finished() -> !;
	fn ksft_print_header();
	fn ksft_set_plan(plan: c_uint);
	fn ksft_exit_skip(format: *const c_char, ...) -> !;
	fn ksft_test_result_fail(format: *const c_char, ...);
	fn ksft_test_result_pass(format: *const c_char, ...);
	fn ksft_test_result_skip(format: *const c_char, ...);
}

type c_uint = u32;

unsafe fn debug0(format: *const c_char) {
	if DEBUG {
		unsafe {
			fprintf(stderr, format);
		}
	}
}

unsafe fn debug1<T>(format: *const c_char, a: T) {
	if DEBUG {
		unsafe {
			fprintf(stderr, format, a);
		}
	}
}

unsafe fn debug2<T, U>(format: *const c_char, a: T, b: U) {
	if DEBUG {
		unsafe {
			fprintf(stderr, format, a, b);
		}
	}
}

/*
 * Check if the cgroup is frozen by looking at the cgroup.events::frozen value.
 */
unsafe extern "C" fn cg_check_frozen(cgroup: *const c_char, frozen: bool) -> c_int {
	unsafe {
		if frozen {
			if cg_read_strstr(cgroup, c"cgroup.events".as_ptr(), c"frozen 1".as_ptr()) != 0 {
				debug1(c"Cgroup %s isn't frozen\n".as_ptr(), cgroup);
				return -1;
			}
		} else {
			/*
			 * Check the cgroup.events::frozen value.
			 */
			if cg_read_strstr(cgroup, c"cgroup.events".as_ptr(), c"frozen 0".as_ptr()) != 0 {
				debug1(c"Cgroup %s is frozen\n".as_ptr(), cgroup);
				return -1;
			}
		}

		0
	}
}

/*
 * Freeze the given cgroup.
 */
unsafe extern "C" fn cg_freeze_nowait(cgroup: *const c_char, freeze: bool) -> c_int {
	unsafe {
		cg_write(
			cgroup,
			c"cgroup.freeze".as_ptr(),
			if freeze { c"1".as_ptr() } else { c"0".as_ptr() },
		)
	}
}

/*
 * Attach a task to the given cgroup and wait for a cgroup frozen event.
 * All transient events (e.g. populated) are ignored.
 */
unsafe extern "C" fn cg_enter_and_wait_for_frozen(
	cgroup: *const c_char,
	pid: c_int,
	frozen: bool,
) -> c_int {
	unsafe {
		let fd: c_int;
		let mut ret: c_int = -1;
		let mut attempts: c_int;

		fd = cg_prepare_for_wait(cgroup);
		if fd < 0 {
			return fd;
		}

		ret = cg_enter(cgroup, pid);
		if ret != 0 {
			close(fd);
			return ret;
		}

		attempts = 0;
		while attempts < 10 {
			ret = cg_wait_for(fd);
			if ret != 0 {
				break;
			}

			ret = cg_check_frozen(cgroup, frozen);
			if ret != 0 {
				attempts += 1;
				continue;
			}
			attempts += 1;
		}

		close(fd);
		ret
	}
}

/*
 * Freeze the given cgroup and wait for the inotify signal.
 * If there are no events in 10 seconds, treat this as an error.
 * Then check that the cgroup is in the desired state.
 */
unsafe extern "C" fn cg_freeze_wait(cgroup: *const c_char, freeze: bool) -> c_int {
	unsafe {
		let fd: c_int;
		let mut ret: c_int = -1;

		fd = cg_prepare_for_wait(cgroup);
		if fd < 0 {
			return fd;
		}

		ret = cg_freeze_nowait(cgroup, freeze);
		if ret != 0 {
			debug0(c"Error: cg_freeze_nowait() failed\n".as_ptr());
			close(fd);
			return ret;
		}

		ret = cg_wait_for(fd);
		if ret == 0 {
			ret = cg_check_frozen(cgroup, freeze);
		}
		close(fd);
		ret
	}
}

/*
 * A simple process running in a sleep loop until being
 * re-parented.
 */
unsafe extern "C" fn child_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
	unsafe {
		let ppid: c_int = getppid();

		while getppid() == ppid {
			usleep(1000);
		}

		(getppid() == ppid) as c_int
	}
}

/*
 * A simple test for the cgroup freezer: populated the cgroup with 100
 * running processes and freeze it. Then unfreeze it. Then it kills all
 * processes and destroys the cgroup.
 */
unsafe extern "C" fn test_cgfreezer_simple(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut i: c_int;

		cgroup = cg_name(root, c"cg_test_simple".as_ptr());
		if cgroup.is_null() {
			free(cgroup.cast());
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}

			i = 0;
			while i < 100 {
				cg_run_nowait(cgroup, child_fn, ptr::null_mut());
				i += 1;
			}

			if cg_wait_for_proc_count(cgroup, 100) != 0 {
				break 'cleanup;
			}

			if cg_check_frozen(cgroup, false) != 0 {
				break 'cleanup;
			}

			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}

			if cg_freeze_wait(cgroup, false) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		if !cgroup.is_null() {
			cg_destroy(cgroup);
		}
		free(cgroup.cast());
		ret
	}
}

/*
 * The test creates the following hierarchy:
 *       A
 *    / / \ \
 *   B  E  I K
 *  /\  |
 * C  D F
 *      |
 *      G
 *      |
 *      H
 *
 * with a process in C, H and 3 processes in K.
 * Then it tries to freeze and unfreeze the whole tree.
 */
unsafe extern "C" fn test_cgfreezer_tree(root: *const c_char) -> c_int {
	unsafe {
		let mut cgroup: [*mut c_char; 10] = [ptr::null_mut(); 10];
		let mut ret: c_int = KSFT_FAIL;
		let mut i: c_int;

		cgroup[0] = cg_name(root, c"cg_test_tree_A".as_ptr());
		if cgroup[0].is_null() {
			return ret;
		}

		cgroup[1] = cg_name(cgroup[0], c"B".as_ptr());
		if cgroup[1].is_null() {
			goto_cleanup_10(&mut cgroup, ret)
		} else {
			cgroup[2] = cg_name(cgroup[1], c"C".as_ptr());
			if cgroup[2].is_null() {
				goto_cleanup_10(&mut cgroup, ret)
			} else {
				cgroup[3] = cg_name(cgroup[1], c"D".as_ptr());
				if cgroup[3].is_null() {
					goto_cleanup_10(&mut cgroup, ret)
				} else {
					cgroup[4] = cg_name(cgroup[0], c"E".as_ptr());
					if cgroup[4].is_null() {
						goto_cleanup_10(&mut cgroup, ret)
					} else {
						cgroup[5] = cg_name(cgroup[4], c"F".as_ptr());
						if cgroup[5].is_null() {
							goto_cleanup_10(&mut cgroup, ret)
						} else {
							cgroup[6] = cg_name(cgroup[5], c"G".as_ptr());
							if cgroup[6].is_null() {
								goto_cleanup_10(&mut cgroup, ret)
							} else {
								cgroup[7] = cg_name(cgroup[6], c"H".as_ptr());
								if cgroup[7].is_null() {
									goto_cleanup_10(&mut cgroup, ret)
								} else {
									cgroup[8] = cg_name(cgroup[0], c"I".as_ptr());
									if cgroup[8].is_null() {
										goto_cleanup_10(&mut cgroup, ret)
									} else {
										cgroup[9] = cg_name(cgroup[0], c"K".as_ptr());
										if cgroup[9].is_null() {
											goto_cleanup_10(&mut cgroup, ret)
										} else {
											'cleanup: loop {
												i = 0;
												while i < 10 {
													if cg_create(cgroup[i as usize]) != 0 {
														break 'cleanup;
													}
													i += 1;
												}

												cg_run_nowait(cgroup[2], child_fn, ptr::null_mut());
												cg_run_nowait(cgroup[7], child_fn, ptr::null_mut());
												cg_run_nowait(cgroup[9], child_fn, ptr::null_mut());
												cg_run_nowait(cgroup[9], child_fn, ptr::null_mut());
												cg_run_nowait(cgroup[9], child_fn, ptr::null_mut());

												/*
												 * Wait until all child processes will enter
												 * corresponding cgroups.
												 */

												if cg_wait_for_proc_count(cgroup[2], 1) != 0
													|| cg_wait_for_proc_count(cgroup[7], 1) != 0
													|| cg_wait_for_proc_count(cgroup[9], 3) != 0
												{
													break 'cleanup;
												}

												/*
												 * Freeze B.
												 */
												if cg_freeze_wait(cgroup[1], true) != 0 {
													break 'cleanup;
												}

												/*
												 * Freeze F.
												 */
												if cg_freeze_wait(cgroup[5], true) != 0 {
													break 'cleanup;
												}

												/*
												 * Freeze G.
												 */
												if cg_freeze_wait(cgroup[6], true) != 0 {
													break 'cleanup;
												}

												/*
												 * Check that A and E are not frozen.
												 */
												if cg_check_frozen(cgroup[0], false) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[4], false) != 0 {
													break 'cleanup;
												}

												/*
												 * Freeze A. Check that A, B and E are frozen.
												 */
												if cg_freeze_wait(cgroup[0], true) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[1], true) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[4], true) != 0 {
													break 'cleanup;
												}

												/*
												 * Unfreeze B, F and G
												 */
												if cg_freeze_nowait(cgroup[1], false) != 0 {
													break 'cleanup;
												}

												if cg_freeze_nowait(cgroup[5], false) != 0 {
													break 'cleanup;
												}

												if cg_freeze_nowait(cgroup[6], false) != 0 {
													break 'cleanup;
												}

												/*
												 * Check that C and H are still frozen.
												 */
												if cg_check_frozen(cgroup[2], true) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[7], true) != 0 {
													break 'cleanup;
												}

												/*
												 * Unfreeze A. Check that A, C and K are not frozen.
												 */
												if cg_freeze_wait(cgroup[0], false) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[2], false) != 0 {
													break 'cleanup;
												}

												if cg_check_frozen(cgroup[9], false) != 0 {
													break 'cleanup;
												}

												ret = KSFT_PASS;
												break 'cleanup;
											}
											goto_cleanup_10(&mut cgroup, ret)
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

unsafe fn goto_cleanup_10(cgroup: &mut [*mut c_char; 10], ret: c_int) -> c_int {
	unsafe {
		let mut i: c_int = 9;
		while i >= 0 && !cgroup[i as usize].is_null() {
			cg_destroy(cgroup[i as usize]);
			free(cgroup[i as usize].cast());
			i -= 1;
		}
		ret
	}
}

/*
 * A fork bomb emulator.
 */
unsafe extern "C" fn forkbomb_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
	unsafe {
		let ppid: c_int;

		fork();
		fork();

		ppid = getppid();

		while getppid() == ppid {
			usleep(1000);
		}

		(getppid() == ppid) as c_int
	}
}

unsafe fn destroy_free_one(cgroup: *mut c_char) {
	unsafe {
		if !cgroup.is_null() {
			cg_destroy(cgroup);
		}
		free(cgroup.cast());
	}
}

/*
 * The test runs a fork bomb in a cgroup and tries to freeze it.
 * Then it kills all processes and checks that cgroup isn't populated
 * anymore.
 */
unsafe extern "C" fn test_cgfreezer_forkbomb(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();

		cgroup = cg_name(root, c"cg_forkbomb_test".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}

			cg_run_nowait(cgroup, forkbomb_fn, ptr::null_mut());

			usleep(100000);

			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}

			if cg_killall(cgroup) != 0 {
				break 'cleanup;
			}

			if cg_wait_for_proc_count(cgroup, 0) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * The test creates a cgroups and freezes it. Then it creates a child cgroup
 * and populates it with a task. After that it checks that the child cgroup
 * is frozen and the parent cgroup remains frozen too.
 */
unsafe extern "C" fn test_cgfreezer_mkdir(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let parent: *mut c_char;
		let mut child: *mut c_char = ptr::null_mut();
		let pid: c_int;

		parent = cg_name(root, c"cg_test_mkdir_A".as_ptr());
		if parent.is_null() {
			return ret;
		}

		child = cg_name(parent, c"cg_test_mkdir_B".as_ptr());
		if child.is_null() {
			destroy_free_one(parent);
			return ret;
		}

		'cleanup: loop {
			if cg_create(parent) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(parent, true) != 0 {
				break 'cleanup;
			}
			if cg_create(child) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(child, child_fn, ptr::null_mut());
			if pid < 0 {
				break 'cleanup;
			}
			if cg_wait_for_proc_count(child, 1) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(child, true) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(parent, true) != 0 {
				break 'cleanup;
			}
			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(child);
		destroy_free_one(parent);
		ret
	}
}

/*
 * The test creates two nested cgroups, freezes the parent
 * and removes the child. Then it checks that the parent cgroup
 * remains frozen and it's possible to create a new child
 * without unfreezing. The new child is frozen too.
 */
unsafe extern "C" fn test_cgfreezer_rmdir(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let parent: *mut c_char;
		let mut child: *mut c_char = ptr::null_mut();

		parent = cg_name(root, c"cg_test_rmdir_A".as_ptr());
		if parent.is_null() {
			return ret;
		}

		child = cg_name(parent, c"cg_test_rmdir_B".as_ptr());
		if child.is_null() {
			destroy_free_one(parent);
			return ret;
		}

		'cleanup: loop {
			if cg_create(parent) != 0 {
				break 'cleanup;
			}
			if cg_create(child) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(parent, true) != 0 {
				break 'cleanup;
			}
			if cg_destroy(child) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(parent, true) != 0 {
				break 'cleanup;
			}
			if cg_create(child) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(child, true) != 0 {
				break 'cleanup;
			}
			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(child);
		destroy_free_one(parent);
		ret
	}
}

/*
 * The test creates two cgroups: A and B, runs a process in A
 * and performs several migrations:
 * 1) A (running) -> B (frozen)
 * 2) B (frozen) -> A (running)
 * 3) A (frozen) -> B (frozen)
 *
 * On each step it checks the actual state of both cgroups.
 */
unsafe extern "C" fn test_cgfreezer_migrate(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: [*mut c_char; 2] = [ptr::null_mut(); 2];
		let pid: c_int;

		cgroup[0] = cg_name(root, c"cg_test_migrate_A".as_ptr());
		if cgroup[0].is_null() {
			return ret;
		}
		cgroup[1] = cg_name(root, c"cg_test_migrate_B".as_ptr());
		if cgroup[1].is_null() {
			return cleanup_2(&mut cgroup, ret);
		}

		'cleanup: loop {
			if cg_create(cgroup[0]) != 0 {
				break 'cleanup;
			}
			if cg_create(cgroup[1]) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(cgroup[0], child_fn, ptr::null_mut());
			if pid < 0 {
				break 'cleanup;
			}
			if cg_wait_for_proc_count(cgroup[0], 1) != 0 {
				break 'cleanup;
			}

			/*
			 * Migrate from A (running) to B (frozen)
			 */
			if cg_freeze_wait(cgroup[1], true) != 0 {
				break 'cleanup;
			}
			if cg_enter_and_wait_for_frozen(cgroup[1], pid, true) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(cgroup[0], false) != 0 {
				break 'cleanup;
			}

			/*
			 * Migrate from B (frozen) to A (running)
			 */
			if cg_enter_and_wait_for_frozen(cgroup[0], pid, false) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(cgroup[1], true) != 0 {
				break 'cleanup;
			}

			/*
			 * Migrate from A (frozen) to B (frozen)
			 */
			if cg_freeze_wait(cgroup[0], true) != 0 {
				break 'cleanup;
			}
			if cg_enter_and_wait_for_frozen(cgroup[1], pid, true) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(cgroup[0], true) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		cleanup_2(&mut cgroup, ret)
	}
}

unsafe fn cleanup_2(cgroup: &mut [*mut c_char; 2], ret: c_int) -> c_int {
	unsafe {
		if !cgroup[0].is_null() {
			cg_destroy(cgroup[0]);
		}
		free(cgroup[0].cast());
		if !cgroup[1].is_null() {
			cg_destroy(cgroup[1]);
		}
		free(cgroup[1].cast());
		ret
	}
}

/*
 * The test checks that ptrace works with a tracing process in a frozen cgroup.
 */
unsafe extern "C" fn test_cgfreezer_ptrace(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut siginfo: siginfo_t = core::mem::zeroed();
		let pid: c_int;

		cgroup = cg_name(root, c"cg_test_ptrace".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(cgroup, child_fn, ptr::null_mut());
			if pid < 0 {
				break 'cleanup;
			}
			if cg_wait_for_proc_count(cgroup, 1) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_SEIZE, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_INTERRUPT, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}
			waitpid(pid, ptr::null_mut(), 0);

			/*
			 * Cgroup has to remain frozen, however the test task
			 * is in traced state.
			 */
			if cg_check_frozen(cgroup, true) != 0 {
				break 'cleanup;
			}

			if ptrace(PTRACE_GETSIGINFO, pid, ptr::null_mut::<c_void>(), &mut siginfo as *mut _ as *mut c_void) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_DETACH, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(cgroup, true) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * Check if the process is stopped.
 */
unsafe extern "C" fn proc_check_stopped(pid: c_int) -> c_int {
	unsafe {
		let mut buf: [c_char; BUF_SIZE] = [0; BUF_SIZE];
		let len: c_int;

		len = proc_read_text(pid, 0, c"stat".as_ptr(), buf.as_mut_ptr(), size_of::<[c_char; BUF_SIZE]>());
		if len == -1 {
			debug1(c"Can't get %d stat\n".as_ptr(), pid);
			return -1;
		}

		if strstr(buf.as_ptr(), c"(test_freezer) T ".as_ptr()).is_null() {
			debug2(c"Process %d in the unexpected state: %s\n".as_ptr(), pid, buf.as_ptr());
			return -1;
		}

		0
	}
}

/*
 * Test that it's possible to freeze a cgroup with a stopped process.
 */
unsafe extern "C" fn test_cgfreezer_stopped(root: *const c_char) -> c_int {
	unsafe {
		let pid: c_int;
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();

		cgroup = cg_name(root, c"cg_test_stopped".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(cgroup, child_fn, ptr::null_mut());
			if cg_wait_for_proc_count(cgroup, 1) != 0 {
				break 'cleanup;
			}
			if kill(pid, SIGSTOP) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(cgroup, false) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(cgroup, false) != 0 {
				break 'cleanup;
			}
			if proc_check_stopped(pid) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * Test that it's possible to freeze a cgroup with a ptraced process.
 */
unsafe extern "C" fn test_cgfreezer_ptraced(root: *const c_char) -> c_int {
	unsafe {
		let pid: c_int;
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut siginfo: siginfo_t = core::mem::zeroed();

		cgroup = cg_name(root, c"cg_test_ptraced".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(cgroup, child_fn, ptr::null_mut());
			if cg_wait_for_proc_count(cgroup, 1) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_SEIZE, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_INTERRUPT, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}
			waitpid(pid, ptr::null_mut(), 0);
			if cg_check_frozen(cgroup, false) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}

			/*
			 * cg_check_frozen(cgroup, true) will fail here,
			 * because the task is in the TRACEd state.
			 */
			if cg_freeze_wait(cgroup, false) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_GETSIGINFO, pid, ptr::null_mut::<c_void>(), &mut siginfo as *mut _ as *mut c_void) != 0 {
				break 'cleanup;
			}
			if ptrace(PTRACE_DETACH, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

unsafe extern "C" fn vfork_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
	unsafe {
		let pid: c_int = vfork();

		if pid == 0 {
			while true {
				sleep(1);
			}
		}

		pid
	}
}

/*
 * Test that it's possible to freeze a cgroup with a process,
 * which called vfork() and is waiting for a child.
 */
unsafe extern "C" fn test_cgfreezer_vfork(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();

		cgroup = cg_name(root, c"cg_test_vfork".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			cg_run_nowait(cgroup, vfork_fn, ptr::null_mut());
			if cg_wait_for_proc_count(cgroup, 2) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(cgroup, true) != 0 {
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * Get the current frozen_usec for the cgroup.
 */
unsafe extern "C" fn cg_check_freezetime(cgroup: *const c_char) -> c_long {
	unsafe {
		cg_read_key_long(
			cgroup,
			c"cgroup.stat.local".as_ptr(),
			c"frozen_usec ".as_ptr(),
		)
	}
}

unsafe fn cleanup_parent_child(parent: *mut c_char, child: *mut c_char, ret: c_int) -> c_int {
	unsafe {
		if !child.is_null() {
			cg_destroy(child);
		}
		free(child.cast());
		if !parent.is_null() {
			cg_destroy(parent);
		}
		free(parent.cast());
		ret
	}
}

/*
 * Test that the freeze time will behave as expected for an empty cgroup.
 */
unsafe extern "C" fn test_cgfreezer_time_empty(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut prev: c_long;
		let mut curr: c_long;

		cgroup = cg_name(root, c"cg_time_test_empty".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			/*
			 * 1) Create an empty cgroup and check that its freeze time
			 *    is 0.
			 */
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			curr = cg_check_freezetime(cgroup);
			if curr < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if curr > 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup, true) != 0 {
				break 'cleanup;
			}

			/*
			 * 2) Sleep for 1000 us. Check that the freeze time is at
			 *    least 1000 us.
			 */
			usleep(1000);
			curr = cg_check_freezetime(cgroup);
			if curr < 1000 {
				debug1(c"Expect time (%ld) to be at least 1000 us\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 3) Unfreeze the cgroup. Check that the freeze time is
			 *    larger than at 2).
			 */
			if cg_freeze_nowait(cgroup, false) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 4) Check the freeze time again to ensure that it has not
			 *    changed.
			 */
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr != prev {
				debug2(c"Expect time (%ld) to be unchanged from previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * A simple test for cgroup freezer time accounting. This test follows
 * the same flow as test_cgfreezer_time_empty, but with a single process
 * in the cgroup.
 */
unsafe extern "C" fn test_cgfreezer_time_simple(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut prev: c_long;
		let mut curr: c_long;

		cgroup = cg_name(root, c"cg_time_test_simple".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			/*
			 * 1) Create a cgroup and check that its freeze time is 0.
			 */
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			curr = cg_check_freezetime(cgroup);
			if curr < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if curr > 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 2) Populate the cgroup with one child and check that the
			 *    freeze time is still 0.
			 */
			cg_run_nowait(cgroup, child_fn, ptr::null_mut());
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr > prev {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup, true) != 0 {
				break 'cleanup;
			}

			/*
			 * 3) Sleep for 1000 us. Check that the freeze time is at
			 *    least 1000 us.
			 */
			usleep(1000);
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr < 1000 {
				debug1(c"Expect time (%ld) to be at least 1000 us\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 4) Unfreeze the cgroup. Check that the freeze time is
			 *    larger than at 3).
			 */
			if cg_freeze_nowait(cgroup, false) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 5) Sleep for 1000 us. Check that the freeze time is the
			 *    same as at 4).
			 */
			usleep(1000);
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr != prev {
				debug2(c"Expect time (%ld) to be unchanged from previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * Test that freezer time accounting works as expected, even while we're
 * populating a cgroup with processes.
 */
unsafe extern "C" fn test_cgfreezer_time_populate(root: *const c_char) -> c_int {
	unsafe {
		let mut ret: c_int = KSFT_FAIL;
		let mut cgroup: *mut c_char = ptr::null_mut();
		let mut prev: c_long;
		let mut curr: c_long;
		let mut i: c_int;

		cgroup = cg_name(root, c"cg_time_test_populate".as_ptr());
		if cgroup.is_null() {
			destroy_free_one(cgroup);
			return ret;
		}

		'cleanup: loop {
			if cg_create(cgroup) != 0 {
				break 'cleanup;
			}
			curr = cg_check_freezetime(cgroup);
			if curr < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if curr > 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 1) Populate the cgroup with 100 processes. Check that
			 *    the freeze time is 0.
			 */
			i = 0;
			while i < 100 {
				cg_run_nowait(cgroup, child_fn, ptr::null_mut());
				i += 1;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr != prev {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 2) Wait for the group to become fully populated. Check
			 *    that the freeze time is 0.
			 */
			if cg_wait_for_proc_count(cgroup, 100) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr != prev {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr);
				break 'cleanup;
			}

			/*
			 * 3) Freeze the cgroup and then populate it with 100 more
			 *    processes. Check that the freeze time continues to grow.
			 */
			if cg_freeze_nowait(cgroup, true) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}
			i = 0;
			while i < 100 {
				cg_run_nowait(cgroup, child_fn, ptr::null_mut());
				i += 1;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 4) Wait for the group to become fully populated. Check
			 *    that the freeze time is larger than at 3).
			 */
			if cg_wait_for_proc_count(cgroup, 200) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 5) Unfreeze the cgroup. Check that the freeze time is
			 *    larger than at 4).
			 */
			if cg_freeze_nowait(cgroup, false) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 6) Kill the processes. Check that the freeze time is the
			 *    same as it was at 5).
			 */
			if cg_killall(cgroup) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr != prev {
				debug2(c"Expect time (%ld) to be unchanged from previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			/*
			 * 7) Freeze and unfreeze the cgroup. Check that the freeze
			 *    time is larger than it was at 6).
			 */
			if cg_freeze_nowait(cgroup, true) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup, false) != 0 {
				break 'cleanup;
			}
			prev = curr;
			curr = cg_check_freezetime(cgroup);
			if curr <= prev {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr, prev);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		destroy_free_one(cgroup);
		ret
	}
}

/*
 * Test that frozen time for a cgroup continues to work as expected,
 * even as processes are migrated. Frozen cgroup A's freeze time should
 * continue to increase and running cgroup B's should stay 0.
 */
unsafe extern "C" fn test_cgfreezer_time_migrate(root: *const c_char) -> c_int {
	unsafe {
		let mut prev_a: c_long;
		let mut curr_a: c_long;
		let mut curr_b: c_long;
		let mut cgroup: [*mut c_char; 2] = [ptr::null_mut(); 2];
		let mut ret: c_int = KSFT_FAIL;
		let pid: c_int;

		cgroup[0] = cg_name(root, c"cg_time_test_migrate_A".as_ptr());
		if cgroup[0].is_null() {
			return ret;
		}
		cgroup[1] = cg_name(root, c"cg_time_test_migrate_B".as_ptr());
		if cgroup[1].is_null() {
			return cleanup_2(&mut cgroup, ret);
		}

		'cleanup: loop {
			if cg_create(cgroup[0]) != 0 {
				break 'cleanup;
			}
			if cg_check_freezetime(cgroup[0]) < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if cg_create(cgroup[1]) != 0 {
				break 'cleanup;
			}
			pid = cg_run_nowait(cgroup[0], child_fn, ptr::null_mut());
			if pid < 0 {
				break 'cleanup;
			}
			if cg_wait_for_proc_count(cgroup[0], 1) != 0 {
				break 'cleanup;
			}
			curr_a = cg_check_freezetime(cgroup[0]);
			if curr_a != 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr_a);
				break 'cleanup;
			}
			curr_b = cg_check_freezetime(cgroup[1]);
			if curr_b != 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr_b);
				break 'cleanup;
			}

			/*
			 * Freeze cgroup A.
			 */
			if cg_freeze_wait(cgroup[0], true) != 0 {
				break 'cleanup;
			}
			prev_a = curr_a;
			curr_a = cg_check_freezetime(cgroup[0]);
			if curr_a <= prev_a {
				debug1(c"Expect time (%ld) to be > 0\n".as_ptr(), curr_a);
				break 'cleanup;
			}

			/*
			 * Migrate from A (frozen) to B (running).
			 */
			if cg_enter(cgroup[1], pid) != 0 {
				break 'cleanup;
			}
			usleep(1000);
			curr_b = cg_check_freezetime(cgroup[1]);
			if curr_b != 0 {
				debug1(c"Expect time (%ld) to be 0\n".as_ptr(), curr_b);
				break 'cleanup;
			}
			prev_a = curr_a;
			curr_a = cg_check_freezetime(cgroup[0]);
			if curr_a <= prev_a {
				debug2(c"Expect time (%ld) to be more than previous check (%ld)\n".as_ptr(), curr_a, prev_a);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		cleanup_2(&mut cgroup, ret)
	}
}

/*
 * The test creates a cgroup and freezes it. Then it creates a child cgroup.
 * After that it checks that the child cgroup has a non-zero freeze time
 * that is less than the parent's. Next, it freezes the child, unfreezes
 * the parent, and sleeps. Finally, it checks that the child's freeze
 * time has grown larger than the parent's.
 */
unsafe extern "C" fn test_cgfreezer_time_parent(root: *const c_char) -> c_int {
	unsafe {
		let parent: *mut c_char;
		let mut child: *mut c_char = ptr::null_mut();
		let mut ret: c_int = KSFT_FAIL;
		let mut ptime: c_long;
		let mut ctime: c_long;

		parent = cg_name(root, c"cg_test_parent_A".as_ptr());
		if parent.is_null() {
			return ret;
		}
		child = cg_name(parent, c"cg_test_parent_B".as_ptr());
		if child.is_null() {
			return cleanup_parent_child(parent, child, ret);
		}

		'cleanup: loop {
			if cg_create(parent) != 0 {
				break 'cleanup;
			}
			if cg_check_freezetime(parent) < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if cg_freeze_wait(parent, true) != 0 {
				break 'cleanup;
			}
			usleep(1000);
			if cg_create(child) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(child, true) != 0 {
				break 'cleanup;
			}

			/*
			 * Since the parent was frozen the entire time the child cgroup
			 * was being created, we expect the parent's freeze time to be
			 * larger than the child's.
			 *
			 * Ideally, we would be able to check both times simultaneously,
			 * but here we get the child's after we get the parent's.
			 */
			ptime = cg_check_freezetime(parent);
			ctime = cg_check_freezetime(child);
			if ptime <= ctime {
				debug2(c"Expect ptime (%ld) > ctime (%ld)\n".as_ptr(), ptime, ctime);
				break 'cleanup;
			}
			if cg_freeze_nowait(child, true) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(parent, false) != 0 {
				break 'cleanup;
			}
			if cg_check_frozen(child, true) != 0 {
				break 'cleanup;
			}
			usleep(100000);
			ctime = cg_check_freezetime(child);
			ptime = cg_check_freezetime(parent);
			if ctime <= ptime {
				debug2(c"Expect ctime (%ld) > ptime (%ld)\n".as_ptr(), ctime, ptime);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		cleanup_parent_child(parent, child, ret)
	}
}

/*
 * The test creates a parent cgroup and a child cgroup. Then, it freezes
 * the child and checks that the child's freeze time is greater than the
 * parent's, which should be zero.
 */
unsafe extern "C" fn test_cgfreezer_time_child(root: *const c_char) -> c_int {
	unsafe {
		let parent: *mut c_char;
		let mut child: *mut c_char = ptr::null_mut();
		let mut ret: c_int = KSFT_FAIL;
		let ptime: c_long;
		let ctime: c_long;

		parent = cg_name(root, c"cg_test_child_A".as_ptr());
		if parent.is_null() {
			return ret;
		}
		child = cg_name(parent, c"cg_test_child_B".as_ptr());
		if child.is_null() {
			return cleanup_parent_child(parent, child, ret);
		}

		'cleanup: loop {
			if cg_create(parent) != 0 {
				break 'cleanup;
			}
			if cg_check_freezetime(parent) < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if cg_create(child) != 0 {
				break 'cleanup;
			}
			if cg_freeze_wait(child, true) != 0 {
				break 'cleanup;
			}
			ctime = cg_check_freezetime(child);
			ptime = cg_check_freezetime(parent);
			if ptime != 0 {
				debug1(c"Expect ptime (%ld) to be 0\n".as_ptr(), ptime);
				break 'cleanup;
			}
			if ctime <= ptime {
				debug2(c"Expect ctime (%ld) > ptime (%ld)\n".as_ptr(), ctime, ptime);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		cleanup_parent_child(parent, child, ret)
	}
}

/*
 * The test creates the following hierarchy:
 *    A
 *    |
 *    B
 *    |
 *    C
 *
 * Then it freezes the cgroups in the order C, B, A.
 * Then it unfreezes the cgroups in the order A, B, C.
 * Then it checks that C's freeze time is larger than B's and
 * that B's is larger than A's.
 */
unsafe extern "C" fn test_cgfreezer_time_nested(root: *const c_char) -> c_int {
	unsafe {
		let mut cgroup: [*mut c_char; 3] = [ptr::null_mut(); 3];
		let mut ret: c_int = KSFT_FAIL;
		let mut time: [c_long; 3] = [0; 3];
		let mut i: c_int;

		cgroup[0] = cg_name(root, c"cg_test_time_A".as_ptr());
		if cgroup[0].is_null() {
			return ret;
		}
		cgroup[1] = cg_name(cgroup[0], c"B".as_ptr());
		if cgroup[1].is_null() {
			return cleanup_3(&mut cgroup, ret);
		}
		cgroup[2] = cg_name(cgroup[1], c"C".as_ptr());
		if cgroup[2].is_null() {
			return cleanup_3(&mut cgroup, ret);
		}

		'cleanup: loop {
			if cg_create(cgroup[0]) != 0 {
				break 'cleanup;
			}
			if cg_check_freezetime(cgroup[0]) < 0 {
				ret = KSFT_SKIP;
				break 'cleanup;
			}
			if cg_create(cgroup[1]) != 0 {
				break 'cleanup;
			}
			if cg_create(cgroup[2]) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup[2], true) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup[1], true) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup[0], true) != 0 {
				break 'cleanup;
			}
			usleep(1000);
			if cg_freeze_nowait(cgroup[0], false) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup[1], false) != 0 {
				break 'cleanup;
			}
			if cg_freeze_nowait(cgroup[2], false) != 0 {
				break 'cleanup;
			}
			time[2] = cg_check_freezetime(cgroup[2]);
			time[1] = cg_check_freezetime(cgroup[1]);
			time[0] = cg_check_freezetime(cgroup[0]);
			if time[2] <= time[1] {
				debug2(c"Expect C's time (%ld) > B's time (%ld)".as_ptr(), time[2], time[1]);
				break 'cleanup;
			}
			if time[1] <= time[0] {
				debug2(c"Expect B's time (%ld) > A's time (%ld)".as_ptr(), time[1], time[0]);
				break 'cleanup;
			}

			ret = KSFT_PASS;
			break 'cleanup;
		}

		i = 2;
		while i >= 0 && !cgroup[i as usize].is_null() {
			cg_destroy(cgroup[i as usize]);
			free(cgroup[i as usize].cast());
			i -= 1;
		}

		ret
	}
}

unsafe fn cleanup_3(cgroup: &mut [*mut c_char; 3], ret: c_int) -> c_int {
	unsafe {
		let mut i: c_int = 2;
		while i >= 0 && !cgroup[i as usize].is_null() {
			cg_destroy(cgroup[i as usize]);
			free(cgroup[i as usize].cast());
			i -= 1;
		}
		ret
	}
}

static TESTS: [cgfreezer_test; 17] = [
	cgfreezer_test { fn_: test_cgfreezer_simple, name: c"test_cgfreezer_simple".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_tree, name: c"test_cgfreezer_tree".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_forkbomb, name: c"test_cgfreezer_forkbomb".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_mkdir, name: c"test_cgfreezer_mkdir".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_rmdir, name: c"test_cgfreezer_rmdir".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_migrate, name: c"test_cgfreezer_migrate".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_ptrace, name: c"test_cgfreezer_ptrace".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_stopped, name: c"test_cgfreezer_stopped".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_ptraced, name: c"test_cgfreezer_ptraced".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_vfork, name: c"test_cgfreezer_vfork".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_empty, name: c"test_cgfreezer_time_empty".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_simple, name: c"test_cgfreezer_time_simple".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_populate, name: c"test_cgfreezer_time_populate".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_migrate, name: c"test_cgfreezer_time_migrate".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_parent, name: c"test_cgfreezer_time_parent".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_child, name: c"test_cgfreezer_time_child".as_ptr() },
	cgfreezer_test { fn_: test_cgfreezer_time_nested, name: c"test_cgfreezer_time_nested".as_ptr() },
];

unsafe fn array_size_tests() -> c_uint {
	TESTS.len() as c_uint
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
	unsafe {
		let mut root: [c_char; PATH_MAX] = [0; PATH_MAX];
		let mut i: usize;

		ksft_print_header();
		ksft_set_plan(array_size_tests());
		if cg_find_unified_root(root.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), ptr::null_mut()) != 0 {
			ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
		}
		i = 0;
		while i < TESTS.len() {
			match (TESTS[i].fn_)(root.as_ptr()) {
				KSFT_PASS => {
					ksft_test_result_pass(c"%s\n".as_ptr(), TESTS[i].name);
				}
				KSFT_SKIP => {
					ksft_test_result_skip(c"%s\n".as_ptr(), TESTS[i].name);
				}
				_ => {
					ksft_test_result_fail(c"%s\n".as_ptr(), TESTS[i].name);
				}
			}
			i += 1;
		}

		ksft_finished();
	}
}
