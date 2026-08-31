// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020 Google LLC.
 */

// C dependencies translated as external declarations:
// <test_progs.h>, <linux/limits.h>, "bprm_opts.skel.h",
// "network_helpers.h", and "task_local_storage_helpers.h".

use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

const BPF_NOEXIST: u64 = 1;
const EINVAL: c_int = 22;
const O_WRONLY: c_int = 1;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bprm_opts_maps {
	pub secure_exec_task_map: *mut bpf_map,
}

#[repr(C)]
pub struct bprm_opts {
	pub maps: bprm_opts_maps,
}

unsafe extern "C" {
	fn __errno_location() -> *mut c_int;
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: u64,
	) -> c_int;
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bprm_opts__attach(obj: *mut bprm_opts) -> c_int;
	fn bprm_opts__destroy(obj: *mut bprm_opts);
	fn bprm_opts__open_and_load() -> *mut bprm_opts;
	fn CHECK(condition: c_int, tag: *const c_char, format: *const c_char, ...) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
	fn execle(path: *const c_char, arg: *const c_char, ...) -> c_int;
	fn exit(status: c_int) -> !;
	fn fork() -> c_int;
	fn getpid() -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn sys_pidfd_open(pid: c_int, flags: c_uint) -> c_int;
	fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
}

static mut bash_envp: [*const c_char; 2] = [
	b"TMPDIR=shouldnotbeset\0".as_ptr() as *const c_char,
	ptr::null(),
];

unsafe fn errno() -> c_int {
	*__errno_location()
}

fn WEXITSTATUS(status: c_int) -> c_int {
	(status & 0xff00) >> 8
}

unsafe fn update_storage(map_fd: c_int, secureexec: c_int) -> c_int {
	let task_fd: c_int;
	let mut ret: c_int = 0;

	task_fd = sys_pidfd_open(getpid(), 0);
	if task_fd < 0 {
		return errno();
	}

	ret = bpf_map_update_elem(
		map_fd,
		&task_fd as *const c_int as *const c_void,
		&secureexec as *const c_int as *const c_void,
		BPF_NOEXIST,
	);
	if ret != 0 {
		ret = errno();
	}

	close(task_fd);
	ret
}

unsafe fn run_set_secureexec(map_fd: c_int, secureexec: c_int) -> c_int {
	let child_pid: c_int;
	let mut child_status: c_int = 0;
	let mut ret: c_int;
	let null_fd: c_int;

	child_pid = fork();
	if child_pid == 0 {
		null_fd = open(b"/dev/null\0".as_ptr() as *const c_char, O_WRONLY);
		if null_fd == -1 {
			exit(errno());
		}
		dup2(null_fd, STDOUT_FILENO);
		dup2(null_fd, STDERR_FILENO);
		close(null_fd);

		/* Ensure that all executions from hereon are
		 * secure by setting a local storage which is read by
		 * the bprm_creds_for_exec hook and sets bprm->secureexec.
		 */
		ret = update_storage(map_fd, secureexec);
		if ret != 0 {
			exit(ret);
		}

		/* If the binary is executed with securexec=1, the dynamic
		 * loader ignores and unsets certain variables like LD_PRELOAD,
		 * TMPDIR etc. TMPDIR is used here to simplify the example, as
		 * LD_PRELOAD requires a real .so file.
		 *
		 * If the value of TMPDIR is set, the bash command returns 10
		 * and if the value is unset, it returns 20.
		 */
		execle(
			b"/bin/bash\0".as_ptr() as *const c_char,
			b"bash\0".as_ptr() as *const c_char,
			b"-c\0".as_ptr() as *const c_char,
			b"[[ -z \"${TMPDIR}\" ]] || exit 10 && exit 20\0".as_ptr() as *const c_char,
			ptr::null::<c_char>(),
			bash_envp.as_ptr(),
		);
		exit(errno());
	} else if child_pid > 0 {
		waitpid(child_pid, &mut child_status as *mut c_int, 0);
		ret = WEXITSTATUS(child_status);

		/* If a secureexec occurred, the exit status should be 20 */
		if secureexec != 0 && ret == 20 {
			return 0;
		}

		/* If normal execution happened, the exit code should be 10 */
		if secureexec == 0 && ret == 10 {
			return 0;
		}
	}

	-EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn test_test_bprm_opts() {
	let mut err: c_int;
	let duration: c_int = 0;
	let mut skel: *mut bprm_opts = ptr::null_mut();

	skel = bprm_opts__open_and_load();
	if CHECK(
		skel.is_null() as c_int,
		b"skel_load\0".as_ptr() as *const c_char,
		b"skeleton failed\n\0".as_ptr() as *const c_char,
	) != 0
	{
		goto_close_prog(skel);
		return;
	}

	err = bprm_opts__attach(skel);
	if CHECK(
		err,
		b"attach\0".as_ptr() as *const c_char,
		b"attach failed: %d\n\0".as_ptr() as *const c_char,
		err,
	) != 0
	{
		goto_close_prog(skel);
		return;
	}

	/* Run the test with the secureexec bit unset */
	err = run_set_secureexec(
		bpf_map__fd((*skel).maps.secure_exec_task_map),
		0, /* secureexec */
	);
	if CHECK(
		err,
		b"run_set_secureexec:0\0".as_ptr() as *const c_char,
		b"err = %d\n\0".as_ptr() as *const c_char,
		err,
	) != 0
	{
		goto_close_prog(skel);
		return;
	}

	/* Run the test with the secureexec bit set */
	err = run_set_secureexec(
		bpf_map__fd((*skel).maps.secure_exec_task_map),
		1, /* secureexec */
	);
	if CHECK(
		err,
		b"run_set_secureexec:1\0".as_ptr() as *const c_char,
		b"err = %d\n\0".as_ptr() as *const c_char,
		err,
	) != 0
	{
		goto_close_prog(skel);
		return;
	}

	let _ = duration;
	goto_close_prog(skel);
}

unsafe fn goto_close_prog(skel: *mut bprm_opts) {
	bprm_opts__destroy(skel);
}
