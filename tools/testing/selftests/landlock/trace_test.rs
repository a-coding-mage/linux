// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Tracepoints
 *
 * Copyright (c) 2026 Cloudflare, Inc.
 */

// C dependencies: errno.h, fcntl.h, linux/landlock.h, pthread.h, sched.h,
// stdio.h, string.h, sys/mount.h, sys/stat.h, sys/types.h, sys/wait.h,
// unistd.h, common.h, trace.h.

const TRACE_TASK: &str = "trace_test";

#[repr(C)]
struct trace {
	tracefs_ok: libc::c_int,
}

FIXTURE!(trace);

FIXTURE_SETUP!(trace, {
	let ret: libc::c_int;

	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, unsafe { libc::unshare(CLONE_NEWNS) });
	ASSERT_EQ!(0, unsafe {
		libc::mount(
			std::ptr::null(),
			c"/".as_ptr(),
			std::ptr::null(),
			MS_REC | MS_PRIVATE,
			std::ptr::null(),
		)
	});

	ret = tracefs_fixture_setup();
	if ret != 0 {
		clear_cap(_metadata, CAP_SYS_ADMIN);
		self.tracefs_ok = 0;
		SKIP!(return, "tracefs not available");
	}
	self.tracefs_ok = 1;

	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_RULESET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_DOMAIN_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ENFORCE_DOMAIN_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ADD_RULE_FS_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ADD_RULE_NET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CHECK_RULE_FS_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CHECK_RULE_NET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_ACCESS_FS_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_FREE_DOMAIN_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_FREE_RULESET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_clear());
	clear_cap(_metadata, CAP_SYS_ADMIN);
});

FIXTURE_TEARDOWN!(trace, {
	if self.tracefs_ok == 0 {
		return;
	}

	/* Disables landlock events and clears PID filter. */
	set_cap(_metadata, CAP_SYS_ADMIN);
	tracefs_enable_event(TRACEFS_CREATE_RULESET_ENABLE, false);
	tracefs_enable_event(TRACEFS_CREATE_DOMAIN_ENABLE, false);
	tracefs_enable_event(TRACEFS_ENFORCE_DOMAIN_ENABLE, false);
	tracefs_enable_event(TRACEFS_ADD_RULE_FS_ENABLE, false);
	tracefs_enable_event(TRACEFS_ADD_RULE_NET_ENABLE, false);
	tracefs_enable_event(TRACEFS_CHECK_RULE_FS_ENABLE, false);
	tracefs_enable_event(TRACEFS_CHECK_RULE_NET_ENABLE, false);
	tracefs_enable_event(TRACEFS_DENY_ACCESS_FS_ENABLE, false);
	tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, false);
	tracefs_enable_event(TRACEFS_FREE_DOMAIN_ENABLE, false);
	tracefs_enable_event(TRACEFS_FREE_RULESET_ENABLE, false);
	tracefs_clear_pid_filter();
	clear_cap(_metadata, CAP_SYS_ADMIN);

	/*
	 * The mount namespace is cleaned up automatically when the test process
	 * (harness child) exits.
	 */
});

/*
 * Verifies that no trace events are emitted when the tracepoints are disabled.
 */
TEST_F!(trace, no_trace_when_disabled, {
	let mut buf: *mut libc::c_char;

	/* Disable all landlock events. */
	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_RULESET_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_DOMAIN_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ENFORCE_DOMAIN_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ADD_RULE_FS_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ADD_RULE_NET_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CHECK_RULE_FS_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CHECK_RULE_NET_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_ACCESS_FS_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_PTRACE_ENABLE, false));
	ASSERT_EQ!(
		0,
		tracefs_enable_event(TRACEFS_DENY_SCOPE_SIGNAL_ENABLE, false)
	);
	ASSERT_EQ!(
		0,
		tracefs_enable_event(
			TRACEFS_DENY_SCOPE_ABSTRACT_UNIX_SOCKET_ENABLE,
			false,
		)
	);
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_FREE_DOMAIN_ENABLE, false));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_FREE_RULESET_ENABLE, false));
	ASSERT_EQ!(0, tracefs_clear());
	clear_cap(_metadata, CAP_SYS_ADMIN);

	/*
	 * Trigger both allowed and denied accesses to verify neither check_rule
	 * nor check_access events fire when disabled.
	 */
	sandbox_child_fs_access(
		_metadata,
		c"/usr".as_ptr(),
		LANDLOCK_ACCESS_FS_READ_DIR,
		LANDLOCK_ACCESS_FS_READ_DIR,
		c"/tmp".as_ptr(),
	);

	/* Read trace buffer and verify no landlock events at all. */
	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(0, tracefs_count_matches(buf, c"landlock_".as_ptr()), {
		TH_LOG!("Expected 0 landlock events when disabled\n%s", buf);
	});

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that landlock_create_ruleset emits a trace event with the correct
 * handled access masks.
 */
TEST_F!(trace, create_ruleset, {
	let ruleset_attr = landlock_ruleset_attr {
		handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE,
		handled_access_net: LANDLOCK_ACCESS_NET_BIND_TCP,
		..unsafe { std::mem::zeroed() }
	};
	let ruleset_fd: libc::c_int;
	let mut buf: *mut libc::c_char;
	let mut dot: *mut libc::c_char;
	let mut field = [0 as libc::c_char; 64];

	ruleset_fd = landlock_create_ruleset(
		&ruleset_attr,
		std::mem::size_of_val(&ruleset_attr),
		0,
	);
	ASSERT_LE!(0, ruleset_fd);
	ASSERT_EQ!(0, unsafe { libc::close(ruleset_fd) });

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_CREATE_RULESET!(TRACE_TASK)), {
		TH_LOG!("Expected 1 create_ruleset event\n%s", buf);
	});

	/* Verify handled_fs matches what we requested. */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_RULESET!(TRACE_TASK),
			c"handled_fs".as_ptr(),
			field.as_mut_ptr(),
			field.len(),
		)
	);
	EXPECT_STREQ!(c"read_file".as_ptr(), field.as_ptr());

	/* Verify handled_net matches. */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_RULESET!(TRACE_TASK),
			c"handled_net".as_ptr(),
			field.as_mut_ptr(),
			field.len(),
		)
	);
	EXPECT_STREQ!(c"bind_tcp".as_ptr(), field.as_ptr());

	/* Verify version is 0 at creation (no rules added yet). */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_RULESET!(TRACE_TASK),
			c"ruleset".as_ptr(),
			field.as_mut_ptr(),
			field.len(),
		)
	);
	/* Format is <hex>.<dec>; version is after the dot. */
	dot = unsafe { libc::strchr(field.as_ptr(), b'.' as libc::c_int) };
	ASSERT_NE!(0, !dot.is_null() as libc::c_int);
	EXPECT_STREQ!(c"0".as_ptr(), unsafe { dot.add(1) });

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that the ruleset version increments with each add_rule call and that
 * create_domain records the correct version.
 */
TEST_F!(trace, ruleset_version, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let mut buf: *mut libc::c_char;
	let mut dot: *mut libc::c_char;
	let mut field = [0 as libc::c_char; 64];

	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);

	if pid == 0 {
		let ruleset_attr = landlock_ruleset_attr {
			handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let mut path_beneath = landlock_path_beneath_attr {
			allowed_access: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let ruleset_fd: libc::c_int;

		ruleset_fd = landlock_create_ruleset(
			&ruleset_attr,
			std::mem::size_of_val(&ruleset_attr),
			0,
		);
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}

		/* First rule: version becomes 1. */
		path_beneath.parent_fd = unsafe {
			libc::open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC)
		};
		if path_beneath.parent_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		landlock_add_rule(
			ruleset_fd,
			LANDLOCK_RULE_PATH_BENEATH,
			(&path_beneath as *const landlock_path_beneath_attr).cast(),
			0,
		);
		unsafe { libc::close(path_beneath.parent_fd) };

		/* Second rule: version becomes 2. */
		path_beneath.parent_fd = unsafe {
			libc::open(c"/tmp".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC)
		};
		if path_beneath.parent_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		landlock_add_rule(
			ruleset_fd,
			LANDLOCK_RULE_PATH_BENEATH,
			(&path_beneath as *const landlock_path_beneath_attr).cast(),
			0,
		);
		unsafe { libc::close(path_beneath.parent_fd) };

		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(ruleset_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };
		unsafe { libc::_exit(0) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	/* Verify create_ruleset has version=0. */
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_RULESET!(TRACE_TASK),
			c"ruleset".as_ptr(),
			field.as_mut_ptr(),
			field.len(),
		)
	);
	dot = unsafe { libc::strchr(field.as_ptr(), b'.' as libc::c_int) };
	ASSERT_NE!(0, !dot.is_null() as libc::c_int);
	EXPECT_STREQ!(c"0".as_ptr(), unsafe { dot.add(1) });

	/* Verify 2 add_rule_fs events were emitted. */
	EXPECT_EQ!(2, tracefs_count_matches(buf, REGEX_ADD_RULE_FS!(TRACE_TASK)), {
		TH_LOG!("Expected 2 add_rule_fs events\n%s", buf);
	});

	/*
	 * Verify create_domain records version=2 (after 2 add_rule calls).  The
	 * ruleset field format is <hex_id>.<dec_version>.
	 */
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"ruleset".as_ptr(),
			field.as_mut_ptr(),
			field.len(),
		)
	);
	dot = unsafe { libc::strchr(field.as_ptr(), b'.' as libc::c_int) };
	ASSERT_NE!(0, !dot.is_null() as libc::c_int);
	EXPECT_STREQ!(c"2".as_ptr(), unsafe { dot.add(1) });

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that landlock_create_domain emits a trace event linking the ruleset
 * ID to the new domain ID.
 */
TEST_F!(trace, create_domain, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let check_count: libc::c_int;
	let mut buf: *mut libc::c_char;
	let mut parent_id = [0 as libc::c_char; 64];
	let mut domain_id = [0 as libc::c_char; 64];
	let mut check_domain = [0 as libc::c_char; 64];

	/* Clear before the sandboxed child. */
	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);

	if pid == 0 {
		let ruleset_attr = landlock_ruleset_attr {
			handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let mut path_beneath = landlock_path_beneath_attr {
			allowed_access: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let ruleset_fd: libc::c_int;
		let fd: libc::c_int;

		ruleset_fd = landlock_create_ruleset(
			&ruleset_attr,
			std::mem::size_of_val(&ruleset_attr),
			0,
		);
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}

		path_beneath.parent_fd = unsafe {
			libc::open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC)
		};
		if path_beneath.parent_fd < 0 {
			unsafe { libc::_exit(1) };
		}

		landlock_add_rule(
			ruleset_fd,
			LANDLOCK_RULE_PATH_BENEATH,
			(&path_beneath as *const landlock_path_beneath_attr).cast(),
			0,
		);
		unsafe { libc::close(path_beneath.parent_fd) };

		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(ruleset_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };

		/* Trigger a check_rule to verify domain_id correlation. */
		fd = unsafe { libc::open(c"/usr".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if fd >= 0 {
			unsafe { libc::close(fd) };
		}

		unsafe { libc::_exit(0) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	/* Verify create_domain event exists. */
	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)), {
		TH_LOG!("Expected 1 create_domain event\n%s", buf);
	});

	/* Extract the domain ID from create_domain. */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"domain".as_ptr(),
			domain_id.as_mut_ptr(),
			domain_id.len(),
		)
	);

	/* Verify domain ID is non-zero. */
	EXPECT_NE!(0, unsafe { libc::strcmp(domain_id.as_ptr(), c"0".as_ptr()) });

	/* Verify parent=0 (first restriction, no prior domain). */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"parent".as_ptr(),
			parent_id.as_mut_ptr(),
			parent_id.len(),
		)
	);
	EXPECT_STREQ!(c"0".as_ptr(), parent_id.as_ptr());

	/*
	 * Verify the same domain ID appears in the check_rule event, confirming
	 * end-to-end correlation.
	 */
	check_count = tracefs_count_matches(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK));
	ASSERT_LE!(1, check_count, {
		TH_LOG!("Expected check_rule_fs events\n%s", buf);
	});

	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CHECK_RULE_FS!(TRACE_TASK),
			c"domain".as_ptr(),
			check_domain.as_mut_ptr(),
			check_domain.len(),
		)
	);
	EXPECT_STREQ!(domain_id.as_ptr(), check_domain.as_ptr());

	unsafe { libc::free(buf.cast()) };
});

/* Builds a rule-less scope-based ruleset; returns the fd or -1. */
fn build_enforce_ruleset() -> libc::c_int {
	let attr = landlock_ruleset_attr {
		scoped: LANDLOCK_SCOPE_SIGNAL,
		..unsafe { std::mem::zeroed() }
	};

	landlock_create_ruleset(&attr, std::mem::size_of_val(&attr), 0)
}

/*
 * Verifies that nested landlock_restrict_self calls produce trace events with
 * correct parent domain IDs: the second create_domain's parent should be the
 * first domain's ID.
 */
TEST_F!(trace, create_domain_nested, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let mut buf: *mut libc::c_char;
	let mut after_first: *const libc::c_char;
	let mut first_domain = [0 as libc::c_char; 64];
	let mut first_parent = [0 as libc::c_char; 64];
	let mut second_parent = [0 as libc::c_char; 64];

	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);

	if pid == 0 {
		let mut ruleset_fd: libc::c_int;

		/* First restriction. */
		ruleset_fd = build_enforce_ruleset();
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(ruleset_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };

		/* Second restriction (nested). */
		ruleset_fd = build_enforce_ruleset();
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		if landlock_restrict_self(ruleset_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };

		unsafe { libc::_exit(0) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	/* Should have 2 create_domain events. */
	EXPECT_EQ!(2, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)), {
		TH_LOG!("Expected 2 create_domain events\n%s", buf);
	});

	/*
	 * Extract domain and parent from each create_domain event.  The first
	 * event (parent=0) is the outer domain; the second (parent!=0) is the
	 * nested domain whose parent should match the first domain's ID.
	 */
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"domain".as_ptr(),
			first_domain.as_mut_ptr(),
			first_domain.len(),
		)
	);
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"parent".as_ptr(),
			first_parent.as_mut_ptr(),
			first_parent.len(),
		)
	);
	EXPECT_STREQ!(c"0".as_ptr(), first_parent.as_ptr());

	/*
	 * Find the second create_domain by scanning past the first.
	 * tracefs_extract_field returns the first match, so search in the
	 * buffer after the first event.
	 *
	 * Skip past the first create_domain line. tracefs_extract_field matches
	 * the first line that matches the regex, so passing the buffer after
	 * the first matching line gives us the second event.
	 */
	after_first = unsafe { libc::strstr(buf, c"landlock_create_domain:".as_ptr()) };
	ASSERT_NE!(std::ptr::null(), after_first);
	after_first = unsafe { libc::strchr(after_first, b'\n' as libc::c_int) };
	ASSERT_NE!(std::ptr::null(), after_first);

	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			unsafe { after_first.add(1) },
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"parent".as_ptr(),
			second_parent.as_mut_ptr(),
			second_parent.len(),
		)
	);

	/* The second domain's parent should be the first domain's ID. */
	EXPECT_STREQ!(first_domain.as_ptr(), second_parent.as_ptr());

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that landlock_add_rule does not emit a trace event when the syscall
 * fails (e.g., invalid ruleset fd).
 */
TEST_F!(trace, add_rule_invalid_fd, {
	let mut path_beneath = landlock_path_beneath_attr {
		allowed_access: LANDLOCK_ACCESS_FS_READ_FILE,
		..unsafe { std::mem::zeroed() }
	};
	let mut buf: *mut libc::c_char;

	path_beneath.parent_fd = unsafe {
		libc::open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC)
	};
	ASSERT_LE!(0, path_beneath.parent_fd);

	/* Invalid ruleset fd (-1). */
	ASSERT_EQ!(
		-1,
		landlock_add_rule(
			-1,
			LANDLOCK_RULE_PATH_BENEATH,
			(&path_beneath as *const landlock_path_beneath_attr).cast(),
			0,
		)
	);
	ASSERT_EQ!(0, unsafe { libc::close(path_beneath.parent_fd) });

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(0, tracefs_count_matches(buf, REGEX_ADD_RULE_FS!(TRACE_TASK)), {
		TH_LOG!("No add_rule_fs event expected on invalid fd\n%s", buf);
	});

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that landlock_create_domain does not emit a trace event when the
 * syscall fails (e.g., invalid ruleset fd or unknown flags).
 */
TEST_F!(trace, create_domain_invalid, {
	let ruleset_fd: libc::c_int;
	let mut buf: *mut libc::c_char;

	ruleset_fd = build_enforce_ruleset();
	ASSERT_LE!(0, ruleset_fd);

	/* Clear the trace buffer after create_ruleset event. */
	ASSERT_EQ!(0, tracefs_clear_buf());

	/* Invalid fd. */
	ASSERT_EQ!(-1, landlock_restrict_self(-1, 0));

	/* Unknown flags. */
	ASSERT_EQ!(-1, landlock_restrict_self(ruleset_fd, -1i32 as __u32));

	ASSERT_EQ!(0, unsafe { libc::close(ruleset_fd) });

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(0, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)), {
		TH_LOG!("No create_domain event expected on error\n%s", buf);
	});

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that trace_landlock_free_domain fires when a domain is deallocated,
 * with the correct denials count.
 */
TEST_F!(trace, free_domain, {
	let mut buf: *mut libc::c_char = std::ptr::null_mut();
	let mut count: libc::c_int = 0;
	let mut denials_field = [0 as libc::c_char; 32];

	ASSERT_EQ!(0, tracefs_clear_buf());

	/*
	 * The domain is freed via a work queue (kworker), so the free_domain
	 * trace event is emitted from a different PID.  Clear the PID filter
	 * BEFORE the child exits, so the kworker event passes the filter when
	 * it fires.
	 */
	set_cap(_metadata, CAP_SYS_ADMIN);
	tracefs_clear_pid_filter();
	clear_cap(_metadata, CAP_SYS_ADMIN);

	sandbox_child_fs_access(
		_metadata,
		c"/usr".as_ptr(),
		LANDLOCK_ACCESS_FS_READ_DIR,
		LANDLOCK_ACCESS_FS_READ_DIR,
		c"/tmp".as_ptr(),
	);

	/*
	 * Wait for the deferred deallocation work to run.  The domain is freed
	 * asynchronously from a kworker; poll until the event appears or a
	 * timeout is reached.
	 */
	for _retry in 0..10 {
		unsafe { libc::usleep(100000) };

		set_cap(_metadata, CAP_SYS_ADMIN);
		buf = tracefs_read_trace();
		clear_cap(_metadata, CAP_SYS_ADMIN);
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count = tracefs_count_matches(buf, REGEX_FREE_DOMAIN!(KWORKER_TASK));
		if count >= 1 {
			break;
		}
		unsafe { libc::free(buf.cast()) };
		buf = std::ptr::null_mut();
	}

	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, tracefs_set_pid_filter(unsafe { libc::getpid() }));
	clear_cap(_metadata, CAP_SYS_ADMIN);

	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, count, {
		TH_LOG!("Expected free_domain event, got %d\n%s", count, buf);
	});

	/* Verify denials count matches the single denial we triggered. */
	EXPECT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_FREE_DOMAIN!(KWORKER_TASK),
			c"denials".as_ptr(),
			denials_field.as_mut_ptr(),
			denials_field.len(),
		)
	);
	EXPECT_STREQ!(c"1".as_ptr(), denials_field.as_ptr());

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that deny_access_fs includes the enriched fields: same_exec and
 * logged.
 */
TEST_F!(trace, deny_access_fs_fields, {
	let mut buf: *mut libc::c_char;
	let mut field_buf = [0 as libc::c_char; 64];

	ASSERT_EQ!(0, tracefs_clear_buf());

	/* Trigger a denial: rule for /usr, access /tmp. */
	sandbox_child_fs_access(
		_metadata,
		c"/usr".as_ptr(),
		LANDLOCK_ACCESS_FS_READ_DIR,
		LANDLOCK_ACCESS_FS_READ_DIR,
		c"/tmp".as_ptr(),
	);

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	/* Verify the enriched fields are present and have valid values. */
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_DENY_ACCESS_FS!(TRACE_TASK),
			c"same_exec".as_ptr(),
			field_buf.as_mut_ptr(),
			field_buf.len(),
		)
	);
	/* Child is the same exec that restricted itself. */
	EXPECT_STREQ!(c"1".as_ptr(), field_buf.as_ptr());

	/* Same exec with default flags: audit would log this denial. */
	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_DENY_ACCESS_FS!(TRACE_TASK),
			c"logged".as_ptr(),
			field_buf.as_mut_ptr(),
			field_buf.len(),
		)
	);
	EXPECT_STREQ!(c"1".as_ptr(), field_buf.as_ptr());

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that same_exec is 1 (true) for denials from the same executable that
 * called landlock_restrict_self().
 */
TEST_F!(trace, same_exec_before_exec, {
	let pid = unsafe { libc::fork() };
	let mut status: libc::c_int = 0;
	let mut field = [0 as libc::c_char; 64];
	ASSERT_EQ!(0, tracefs_clear_buf());
	ASSERT_LE!(0, pid);
	if pid == 0 {
		let attr = landlock_ruleset_attr {
			handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let ruleset_fd = landlock_create_ruleset(&attr, std::mem::size_of_val(&attr), 0);
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		/* No rules: all read_dir access is denied. */
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(ruleset_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };
		/* Trigger denial without exec (same executable). */
		let dir_fd = unsafe { libc::open(c".".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if dir_fd >= 0 {
			unsafe { libc::close(dir_fd) };
		}
		unsafe { libc::_exit(0) };
	}
	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK)));
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK), c"same_exec".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"1".as_ptr(), field.as_ptr());
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK), c"logged".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"1".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that same_exec is 0 (false) for denials from a process that has
 * exec'd a new binary after landlock_restrict_self().  The sandboxed child
 * exec's true which opens "." and triggers a read_dir denial.  Covers the
 * "trace-only" visibility condition: with same_exec=0 and the default
 * log_new_exec=0, audit suppresses the denial (logged=0) but the trace event
 * still fires.
 */
TEST_F!(trace, same_exec_after_exec, {
	let mut field = [0 as libc::c_char; 64];
	ASSERT_EQ!(0, tracefs_clear_buf());
	sandbox_child_exec_true(_metadata, 0);
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!("true")));
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!("true"), c"same_exec".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"0".as_ptr(), field.as_ptr());
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!("true"), c"logged".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"0".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF suppresses logging
 * (logged=0) for a denial from the same executable.
 */
TEST_F!(trace, log_flags_same_exec_off, {
	let pid = unsafe { libc::fork() };
	let mut status: libc::c_int = 0;
	let mut field = [0 as libc::c_char; 64];
	ASSERT_EQ!(0, tracefs_clear_buf());
	ASSERT_LE!(0, pid);
	if pid == 0 {
		let attr = landlock_ruleset_attr {
			handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let ruleset_fd = landlock_create_ruleset(&attr, std::mem::size_of_val(&attr), 0);
		if ruleset_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(ruleset_fd) };
		let dir_fd = unsafe { libc::open(c".".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if dir_fd >= 0 {
			unsafe { libc::close(dir_fd) };
		}
		unsafe { libc::_exit(0) };
	}
	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK)));
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK), c"logged".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"0".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON causes a post-exec
 * denial to be logged (logged=1).  The child exec's true so that the denial
 * comes from a new executable (same_exec=0).
 */
TEST_F!(trace, log_flags_new_exec_on, {
	let mut field = [0 as libc::c_char; 64];
	ASSERT_EQ!(0, tracefs_clear_buf());
	sandbox_child_exec_true(_metadata, LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON);
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!("true")));
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!("true"), c"same_exec".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"0".as_ptr(), field.as_ptr());
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!("true"), c"logged".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"1".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that denials suppressed by audit log flags are still counted in
 * num_denials.  The child restricts itself with default flags (log_same_exec=1,
 * log_new_exec=0), then execs true which attempts to read a denied directory.
 * After exec, same_exec=0 and log_new_exec=0, so audit suppresses the denial.
 * But the trace event fires unconditionally and free_domain must report the
 * correct denials count.
 */
TEST_F!(trace, non_audit_visible_denial_counting, {
	let mut buf: *mut libc::c_char = std::ptr::null_mut();
	let mut denials_field = [0 as libc::c_char; 32];
	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, tracefs_clear());
	tracefs_clear_pid_filter();
	clear_cap(_metadata, CAP_SYS_ADMIN);
	sandbox_child_exec_true(_metadata, 0);
	for _retry in 0..10 {
		unsafe { libc::usleep(100000) };
		set_cap(_metadata, CAP_SYS_ADMIN);
		buf = tracefs_read_trace();
		clear_cap(_metadata, CAP_SYS_ADMIN);
		if buf.is_null() {
			break;
		}
		if tracefs_count_matches(buf, REGEX_FREE_DOMAIN!(KWORKER_TASK)) >= 1 {
			break;
		}
		unsafe { libc::free(buf.cast()) };
		buf = std::ptr::null_mut();
	}
	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, tracefs_set_pid_filter(unsafe { libc::getpid() }));
	clear_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_NE!(std::ptr::null_mut(), buf, {
		TH_LOG!("free_domain event not found after 10 retries");
	});
	EXPECT_EQ!(0, tracefs_extract_field(buf, REGEX_FREE_DOMAIN!(KWORKER_TASK), c"denials".as_ptr(), denials_field.as_mut_ptr(), denials_field.len()));
	EXPECT_STREQ!(c"1".as_ptr(), denials_field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that landlock_add_rule_net emits a trace event with the correct port
 * and allowed access mask fields.
 */
TEST_F!(trace, add_rule_net_fields, {
	let ruleset_attr = landlock_ruleset_attr {
		handled_access_net: LANDLOCK_ACCESS_NET_BIND_TCP,
		..unsafe { std::mem::zeroed() }
	};
	let net_port = landlock_net_port_attr {
		allowed_access: LANDLOCK_ACCESS_NET_BIND_TCP,
		port: 8080,
	};
	let mut field = [0 as libc::c_char; 64];
	let ruleset_fd = landlock_create_ruleset(&ruleset_attr, std::mem::size_of_val(&ruleset_attr), 0);
	ASSERT_LE!(0, ruleset_fd);
	ASSERT_EQ!(0, tracefs_clear_buf());
	ASSERT_EQ!(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT, (&net_port as *const landlock_net_port_attr).cast(), 0));
	unsafe { libc::close(ruleset_fd) };
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_ADD_RULE_NET!(TRACE_TASK)), {
		TH_LOG!("Expected 1 add_rule_net event\n%s", buf);
	});
	EXPECT_EQ!(0, tracefs_extract_field(buf, REGEX_ADD_RULE_NET!(TRACE_TASK), c"port".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"8080".as_ptr(), field.as_ptr());
	EXPECT_EQ!(0, tracefs_extract_field(buf, REGEX_ADD_RULE_NET!(TRACE_TASK), c"access_rights".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"bind_tcp|connect_tcp|bind_udp|connect_send_udp".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies that LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF suppresses audit
 * logging for child domains (logged=0) even though the child's own
 * per-execution flags are the defaults, while the trace event still fires
 * (tracing is unconditional).  The parent creates a domain with
 * LOG_SUBDOMAINS_OFF, then the child creates a sub-domain and triggers a
 * denial.
 */
TEST_F!(trace, log_flags_subdomains_off, {
	let pid = unsafe { libc::fork() };
	let mut status: libc::c_int = 0;
	let mut field = [0 as libc::c_char; 64];
	ASSERT_EQ!(0, tracefs_clear_buf());
	ASSERT_LE!(0, pid);
	if pid == 0 {
		let attr = landlock_ruleset_attr {
			handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
			..unsafe { std::mem::zeroed() }
		};
		let parent_fd = landlock_create_ruleset(&attr, std::mem::size_of_val(&attr), 0);
		if parent_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(parent_fd, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(parent_fd) };
		let child_fd = landlock_create_ruleset(&attr, std::mem::size_of_val(&attr), 0);
		if child_fd < 0 {
			unsafe { libc::_exit(1) };
		}
		if landlock_restrict_self(child_fd, 0) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::close(child_fd) };
		let dir_fd = unsafe { libc::open(c".".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if dir_fd >= 0 {
			unsafe { libc::close(dir_fd) };
		}
		unsafe { libc::_exit(0) };
	}
	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK)), {
		TH_LOG!("Expected deny_access_fs event despite LOG_SUBDOMAINS_OFF\n%s", buf);
	});
	ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK), c"logged".as_ptr(), field.as_mut_ptr(), field.len()));
	EXPECT_STREQ!(c"0".as_ptr(), field.as_ptr());
	unsafe { libc::free(buf.cast()) };
});

/* Verifies that landlock_free_ruleset fires when a ruleset FD is closed. */
TEST_F!(trace, free_ruleset_on_close, {
	let ruleset_attr = landlock_ruleset_attr {
		handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
		..unsafe { std::mem::zeroed() }
	};
	let ruleset_fd = landlock_create_ruleset(&ruleset_attr, std::mem::size_of_val(&ruleset_attr), 0);
	ASSERT_LE!(0, ruleset_fd);
	ASSERT_EQ!(0, tracefs_clear_buf());
	/* Closing the FD should trigger free_ruleset. */
	unsafe { libc::close(ruleset_fd) };
	let buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);
	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_FREE_RULESET!(TRACE_TASK)), {
		TH_LOG!("Expected 1 free_ruleset event\n%s", buf);
	});
	unsafe { libc::free(buf.cast()) };
});

/*
 * Counts landlock_enforce_domain lines, filtered by @domain (NULL matches any),
 * @complete and @process_wide (a negative value matches any).  Builds the
 * anchored regex dynamically so a single helper covers every field assertion.
 */
fn count_enforce_matches(
	buf: *const libc::c_char,
	domain: *const libc::c_char,
	complete: libc::c_int,
	process_wide: libc::c_int,
	no_new_privs: libc::c_int,
) -> libc::c_int {
	let mut pattern = [0 as libc::c_char; 512];
	let mut dom = [0 as libc::c_char; 80];
	let mut comp = [0 as libc::c_char; 8];
	let mut pw = [0 as libc::c_char; 8];
	let mut nnp = [0 as libc::c_char; 8];

	if !domain.is_null() {
		unsafe { libc::snprintf(dom.as_mut_ptr(), dom.len(), c"%s".as_ptr(), domain) };
	} else {
		unsafe { libc::snprintf(dom.as_mut_ptr(), dom.len(), c"[0-9a-f]\\+".as_ptr()) };
	}
	if complete < 0 {
		unsafe { libc::snprintf(comp.as_mut_ptr(), comp.len(), c"[01]".as_ptr()) };
	} else {
		unsafe { libc::snprintf(comp.as_mut_ptr(), comp.len(), c"%d".as_ptr(), complete) };
	}
	if process_wide < 0 {
		unsafe { libc::snprintf(pw.as_mut_ptr(), pw.len(), c"[01]".as_ptr()) };
	} else {
		unsafe { libc::snprintf(pw.as_mut_ptr(), pw.len(), c"%d".as_ptr(), process_wide) };
	}
	if no_new_privs < 0 {
		unsafe { libc::snprintf(nnp.as_mut_ptr(), nnp.len(), c"[01]".as_ptr()) };
	} else {
		unsafe { libc::snprintf(nnp.as_mut_ptr(), nnp.len(), c"%d".as_ptr(), no_new_privs) };
	}

	unsafe {
		libc::snprintf(
			pattern.as_mut_ptr(),
			pattern.len(),
			c"%slandlock_enforce_domain: domain=%s complete=%s process_wide=%s no_new_privs=%s$".as_ptr(),
			TRACE_PREFIX!(TRACE_TASK),
			dom.as_ptr(),
			comp.as_ptr(),
			pw.as_ptr(),
			nnp.as_ptr(),
		);
	}
	tracefs_count_matches(buf, pattern.as_ptr())
}

/* Idle sibling: waits on the barrier so it is a live thread, then sleeps. */
unsafe extern "C" fn enforce_idle(arg: *mut libc::c_void) -> *mut libc::c_void {
	let barrier = arg as *mut pthread_barrier_t;

	pthread_barrier_wait(barrier);
	loop {
		libc::sleep(1);
	}
}

/*
 * Child body: spawns @nthreads idle siblings (barrier-synchronized so they are
 * live when the syscall runs), then enforces a domain with @flags.  Returns 0
 * on success; the process exits afterwards, reaping the siblings.
 */
fn child_enforce(nthreads: libc::c_int, flags: __u32) -> libc::c_int {
	let mut threads = [unsafe { std::mem::zeroed::<pthread_t>() }; 8];
	let mut barrier = unsafe { std::mem::zeroed::<pthread_barrier_t>() };
	let ruleset_fd: libc::c_int;
	let mut i: libc::c_int;

	if nthreads > 0 {
		if unsafe { pthread_barrier_init(&mut barrier, std::ptr::null(), nthreads as libc::c_uint + 1) } != 0 {
			return 1;
		}
		i = 0;
		while i < nthreads {
			if unsafe {
				pthread_create(
					&mut threads[i as usize],
					std::ptr::null(),
					Some(enforce_idle),
					(&mut barrier as *mut pthread_barrier_t).cast(),
				)
			} != 0 {
				return 1;
			}
			i += 1;
		}
		unsafe { pthread_barrier_wait(&mut barrier) };
	}

	ruleset_fd = build_enforce_ruleset();
	if ruleset_fd < 0 {
		return 1;
	}

	/*
	 * LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS sets no_new_privs itself, so skip
	 * the prctl() to exercise that path; otherwise Landlock requires
	 * no_new_privs up front.
	 */
	if flags & LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS == 0 {
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
	}
	if landlock_restrict_self(ruleset_fd, flags) != 0 {
		return 1;
	}
	unsafe { libc::close(ruleset_fd) };
	0
}

/*
 * Runs in a spawned thread after the group leader called pthread_exit().  The
 * leader lingers as an un-reaped zombie, so get_nr_threads() still counts it
 * and this non-leader is not the only thread; enforcing here therefore reports
 * process_wide=0.
 */
unsafe extern "C" fn enforce_nonleader(_arg: *mut libc::c_void) -> *mut libc::c_void {
	let ruleset_fd: libc::c_int;

	ruleset_fd = build_enforce_ruleset();
	if ruleset_fd < 0 {
		libc::_exit(1);
	}
	libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
	if landlock_restrict_self(ruleset_fd, 0) != 0 {
		libc::_exit(1);
	}
	libc::_exit(0);
}

#[repr(C)]
struct trace_enforce {
	tracefs_ok: libc::c_int,
}

FIXTURE!(trace_enforce);

FIXTURE_SETUP!(trace_enforce, {
	let ret: libc::c_int;

	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, unsafe { libc::unshare(CLONE_NEWNS) });
	ASSERT_EQ!(0, unsafe {
		libc::mount(
			std::ptr::null(),
			c"/".as_ptr(),
			std::ptr::null(),
			MS_REC | MS_PRIVATE,
			std::ptr::null(),
		)
	});

	ret = tracefs_fixture_setup();
	if ret != 0 {
		clear_cap(_metadata, CAP_SYS_ADMIN);
		self.tracefs_ok = 0;
		SKIP!(return, "tracefs not available");
	}
	self.tracefs_ok = 1;

	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_RULESET_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CREATE_DOMAIN_ENABLE, true));
	ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ENFORCE_DOMAIN_ENABLE, true));
	ASSERT_EQ!(0, tracefs_clear());
	clear_cap(_metadata, CAP_SYS_ADMIN);
});

FIXTURE_TEARDOWN!(trace_enforce, {
	if self.tracefs_ok == 0 {
		return;
	}

	set_cap(_metadata, CAP_SYS_ADMIN);
	tracefs_enable_event(TRACEFS_CREATE_RULESET_ENABLE, false);
	tracefs_enable_event(TRACEFS_CREATE_DOMAIN_ENABLE, false);
	tracefs_enable_event(TRACEFS_ENFORCE_DOMAIN_ENABLE, false);
	tracefs_fixture_teardown();
	clear_cap(_metadata, CAP_SYS_ADMIN);
});

#[repr(C)]
struct trace_enforce_variant {
	/* Inputs to child_enforce(). */
	nthreads: libc::c_int,
	flags: __u32,
	/* Expected enforce_domain event counts. */
	total: libc::c_int,
	complete: libc::c_int,
	process_wide: libc::c_int,
	no_new_privs: libc::c_int,
}

FIXTURE_VARIANT!(trace_enforce);

/* Single thread, no flags: prctl-backed no_new_privs. */
FIXTURE_VARIANT_ADD!(trace_enforce, single, trace_enforce_variant {
	nthreads: 0, flags: 0,
	total: 1, complete: 1, process_wide: 1, no_new_privs: 1,
});

/* Single thread: the NO_NEW_PRIVS flag sets no_new_privs (no prctl). */
FIXTURE_VARIANT_ADD!(trace_enforce, no_new_privs, trace_enforce_variant {
	nthreads: 0, flags: LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS,
	total: 1, complete: 1, process_wide: 1, no_new_privs: 1,
});

/* TSYNC on a lone thread still concludes, process-wide. */
FIXTURE_VARIANT_ADD!(trace_enforce, tsync_single, trace_enforce_variant {
	nthreads: 0, flags: LANDLOCK_RESTRICT_SELF_TSYNC,
	total: 1, complete: 1, process_wide: 1, no_new_privs: 1,
});

/* TSYNC sweeps N siblings; the caller's prctl-backed nnp propagates to all. */
FIXTURE_VARIANT_ADD!(trace_enforce, tsync_multithread, trace_enforce_variant {
	nthreads: 3, flags: LANDLOCK_RESTRICT_SELF_TSYNC,
	total: 4, complete: 1, process_wide: 4, no_new_privs: 4,
});

/* TSYNC + NO_NEW_PRIVS flag sets nnp on the caller and every swept sibling. */
FIXTURE_VARIANT_ADD!(trace_enforce, tsync_no_new_privs, trace_enforce_variant {
	nthreads: 3,
	flags: LANDLOCK_RESTRICT_SELF_TSYNC | LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS,
	total: 4, complete: 1, process_wide: 4, no_new_privs: 4,
});

/* Non-TSYNC on a multi-threaded process enforces only the caller. */
FIXTURE_VARIANT_ADD!(trace_enforce, multithread_non_tsync, trace_enforce_variant {
	nthreads: 3, flags: 0,
	total: 1, complete: 1, process_wide: 0, no_new_privs: 1,
});

TEST_F!(trace_enforce, enforce, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let mut buf: *mut libc::c_char;
	let mut domain = [0 as libc::c_char; 64];

	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);
	if pid == 0 {
		unsafe { libc::_exit(child_enforce(variant.nthreads, variant.flags)) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)));
	EXPECT_EQ!(variant.total, count_enforce_matches(buf, std::ptr::null(), -1, -1, -1), {
		TH_LOG!("Expected %d enforce_domain events\n%s", variant.total, buf);
	});
	EXPECT_EQ!(variant.complete, count_enforce_matches(buf, std::ptr::null(), 1, -1, -1));
	EXPECT_EQ!(variant.total - variant.complete, count_enforce_matches(buf, std::ptr::null(), 0, -1, -1));
	EXPECT_EQ!(variant.process_wide, count_enforce_matches(buf, std::ptr::null(), -1, 1, -1));
	EXPECT_EQ!(variant.total - variant.process_wide, count_enforce_matches(buf, std::ptr::null(), -1, 0, -1));
	EXPECT_EQ!(variant.no_new_privs, count_enforce_matches(buf, std::ptr::null(), -1, -1, 1));

	ASSERT_EQ!(
		0,
		tracefs_extract_field(
			buf,
			REGEX_CREATE_DOMAIN!(TRACE_TASK),
			c"domain".as_ptr(),
			domain.as_mut_ptr(),
			domain.len(),
		)
	);
	EXPECT_EQ!(variant.total, count_enforce_matches(buf, domain.as_ptr(), -1, -1, -1));

	unsafe { libc::free(buf.cast()) };
});

/*
 * A non-leader thread enforcing a domain while the group leader lingers as an
 * un-reaped zombie reports process_wide=0: get_nr_threads() counts the zombie
 * leader, so the group is not single-threaded.  This is the reachable half of
 * the caveat that process_wide==0 never proves the process is multi-threaded
 * (get_nr_threads(), unlike the leader-relative thread_group_empty(), counts
 * the zombie leader).
 */
TEST_F!(trace, enforce_single_non_leader, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let buf: *mut libc::c_char;

	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);
	if pid == 0 {
		let mut worker = unsafe { std::mem::zeroed::<pthread_t>() };

		if unsafe {
			pthread_create(
				&mut worker,
				std::ptr::null(),
				Some(enforce_nonleader),
				std::ptr::null_mut(),
			)
		} != 0 {
			unsafe { libc::_exit(1) };
		}
		/* Leader leaves; the worker enforces as a non-leader. */
		unsafe { pthread_exit(std::ptr::null_mut()) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(1, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)));
	EXPECT_EQ!(1, count_enforce_matches(buf, std::ptr::null(), 1, 0, -1), {
		TH_LOG!("Expected complete=1 process_wide=0 for non-leader\n%s", buf);
	});

	unsafe { libc::free(buf.cast()) };
});

/*
 * Verifies the flags-only path (ruleset_fd == -1) creates no domain and emits
 * neither create_domain nor enforce_domain, with and without TSYNC.
 */
TEST_F!(trace, enforce_flags_only, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let buf: *mut libc::c_char;

	ASSERT_EQ!(0, tracefs_clear_buf());

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);
	if pid == 0 {
		unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
		if landlock_restrict_self(-1, LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF) != 0 {
			unsafe { libc::_exit(1) };
		}
		if landlock_restrict_self(
			-1,
			LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF | LANDLOCK_RESTRICT_SELF_TSYNC,
		) != 0 {
			unsafe { libc::_exit(1) };
		}
		unsafe { libc::_exit(0) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	buf = tracefs_read_buf();
	ASSERT_NE!(std::ptr::null_mut(), buf);

	EXPECT_EQ!(0, tracefs_count_matches(buf, REGEX_CREATE_DOMAIN!(TRACE_TASK)));
	EXPECT_EQ!(0, count_enforce_matches(buf, std::ptr::null(), -1, -1, -1), {
		TH_LOG!("No enforce_domain expected on flags-only path\n%s", buf);
	});

	unsafe { libc::free(buf.cast()) };
});

fn enforce_nop_handler(_sig: libc::c_int) {}

#[repr(C)]
struct abort_signaler_data {
	target: pthread_t,
	stop: bool,
}

/*
 * Hammers the target thread with SIGUSR1 to interrupt the TSYNC prepare wait.
 */
unsafe extern "C" fn abort_signaler(arg: *mut libc::c_void) -> *mut libc::c_void {
	let data = arg as *mut abort_signaler_data;

	while !std::ptr::read_volatile(std::ptr::addr_of!((*data).stop)) {
		pthread_kill((*data).target, SIGUSR1);
	}
	std::ptr::null_mut()
}

/*
 * Child body for the abort test: with idle siblings and a signaler interrupting
 * it, repeatedly enforces under TSYNC.  An interrupted attempt aborts its
 * just-created domain (create_domain + free_domain, zero enforce_domain) while
 * -ERESTARTNOINTR transparently restarts the syscall, so a successful retry may
 * add its own full lifecycle.
 */
fn child_abort(nsiblings: libc::c_int, attempts: libc::c_int) -> libc::c_int {
	let mut threads = [unsafe { std::mem::zeroed::<pthread_t>() }; 200];
	let mut signaler = unsafe { std::mem::zeroed::<pthread_t>() };
	let mut barrier = unsafe { std::mem::zeroed::<pthread_barrier_t>() };
	let mut data = abort_signaler_data {
		target: unsafe { std::mem::zeroed() },
		stop: false,
	};
	let mut sa = unsafe { std::mem::zeroed::<libc::sigaction>() };
	let mut i: libc::c_int;

	sa.sa_sigaction = enforce_nop_handler as usize;
	if unsafe { libc::sigaction(SIGUSR1, &sa, std::ptr::null_mut()) } != 0 {
		return 1;
	}

	if unsafe { pthread_barrier_init(&mut barrier, std::ptr::null(), nsiblings as libc::c_uint + 1) } != 0 {
		return 1;
	}
	i = 0;
	while i < nsiblings {
		if unsafe {
			pthread_create(
				&mut threads[i as usize],
				std::ptr::null(),
				Some(enforce_idle),
				(&mut barrier as *mut pthread_barrier_t).cast(),
			)
		} != 0 {
			return 1;
		}
		i += 1;
	}
	unsafe { pthread_barrier_wait(&mut barrier) };

	data.target = unsafe { pthread_self() };
	if unsafe {
		pthread_create(
			&mut signaler,
			std::ptr::null(),
			Some(abort_signaler),
			(&mut data as *mut abort_signaler_data).cast(),
		)
	} != 0 {
		return 1;
	}

	unsafe { libc::prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
	i = 0;
	while i < attempts {
		let ruleset_fd = build_enforce_ruleset();

		if ruleset_fd < 0 {
			break;
		}
		/*
		 * Ignore the result: an abort returns an error, that is fine.
		 */
		landlock_restrict_self(ruleset_fd, LANDLOCK_RESTRICT_SELF_TSYNC);
		unsafe { libc::close(ruleset_fd) };
		i += 1;
	}

	unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!(data.stop), true) };
	unsafe { pthread_join(signaler, std::ptr::null_mut()) };
	0
}

/*
 * Verifies the abort contract: a domain aborted by a thread-sync failure emits
 * create_domain and free_domain but zero enforce_domain.  The signal race is
 * probabilistic and -ERESTARTNOINTR may add a successful retry's lifecycle, so
 * events are grouped by domain ID and the test SKIPs if no abort occurred.
 */
TEST_F!(trace, enforce_abort, {
	let pid: libc::pid_t;
	let mut status: libc::c_int = 0;
	let mut buf: *mut libc::c_char = std::ptr::null_mut();
	let mut cursor: *const libc::c_char;
	let mut domain = [0 as libc::c_char; 64];
	let mut abort_found = false;

	ASSERT_EQ!(0, tracefs_clear_buf());

	/* free_domain fires from a kworker, so widen the filter first. */
	set_cap(_metadata, CAP_SYS_ADMIN);
	tracefs_clear_pid_filter();
	clear_cap(_metadata, CAP_SYS_ADMIN);

	pid = unsafe { libc::fork() };
	ASSERT_LE!(0, pid);
	if pid == 0 {
		/*
		 * Match tsync_test's NUM_IDLE_THREADS: enough siblings that
		 * credential preparation runs in several serialized waves,
		 * giving the signaler a window to interrupt the thread-sync
		 * wait and abort the operation.  A handful of threads finishes
		 * in a single wave, leaving no window (the abort never fires).
		 */
		unsafe { libc::_exit(child_abort(200, 8)) };
	}

	ASSERT_EQ!(pid, unsafe { libc::waitpid(pid, &mut status, 0) });
	ASSERT_TRUE!(WIFEXITED!(status));
	EXPECT_EQ!(0, WEXITSTATUS!(status));

	/* Poll for the asynchronous free_domain events. */
	for _retry in 0..10 {
		unsafe { libc::usleep(100000) };
		set_cap(_metadata, CAP_SYS_ADMIN);
		unsafe { libc::free(buf.cast()) };
		buf = tracefs_read_trace();
		clear_cap(_metadata, CAP_SYS_ADMIN);
		ASSERT_NE!(std::ptr::null_mut(), buf);
	}

	set_cap(_metadata, CAP_SYS_ADMIN);
	ASSERT_EQ!(0, tracefs_set_pid_filter(unsafe { libc::getpid() }));
	clear_cap(_metadata, CAP_SYS_ADMIN);

	/*
	 * Walk every create_domain and look for one whose domain ID has zero
	 * enforce_domain events but a matching free_domain: that is an aborted
	 * domain (created, never enforced, freed).
	 */
	cursor = buf;
	while tracefs_extract_field(
		cursor,
		REGEX_CREATE_DOMAIN!(TRACE_TASK),
		c"domain".as_ptr(),
		domain.as_mut_ptr(),
		domain.len(),
	) == 0 {
		let cd: *const libc::c_char;
		let nl: *const libc::c_char;
		let mut free_pattern = [0 as libc::c_char; 256];

		if count_enforce_matches(buf, domain.as_ptr(), -1, -1, -1) == 0 {
			unsafe {
				libc::snprintf(
					free_pattern.as_mut_ptr(),
					free_pattern.len(),
					c"%slandlock_free_domain: domain=%s denials=[0-9]\\+$".as_ptr(),
					TRACE_PREFIX!(KWORKER_TASK),
					domain.as_ptr(),
				);
			}
			if tracefs_count_matches(buf, free_pattern.as_ptr()) >= 1 {
				abort_found = true;
			}
		}

		cd = unsafe { libc::strstr(cursor, c"landlock_create_domain:".as_ptr()) };
		if cd.is_null() {
			break;
		}
		nl = unsafe { libc::strchr(cd, b'\n' as libc::c_int) };
		if nl.is_null() {
			break;
		}
		cursor = unsafe { nl.add(1) };
	}

	if !abort_found {
		unsafe { libc::free(buf.cast()) };
		SKIP!(return, "signal race did not produce a thread-sync abort");
	}

	unsafe { libc::free(buf.cast()) };
});

/*
 * The following tests are intentionally elided because the underlying kernel
 * mechanisms are already validated by audit tests:
 *
 * - Domain ID monotonicity: validated by audit_test.c:layers.  The same
 *   landlock_get_id_range() function serves both audit and trace.
 *
 * - Domain deallocation order (LIFO): validated by audit_test.c:layers.  Trace
 *   events fire from the same free_domain_work() code path.
 *
 * - Max-layer stacking (16 domains): validated by audit_test.c:layers.
 *
 * - IPv6 network tests: IPv6 hook dispatch uses the same
 *   current_check_access_socket() as IPv4, validated by net_test.c:audit tests.
 *
 * - Per-access-right full matrix (all 16 FS rights): hook dispatch is validated
 *   by fs_test.c:audit tests.  Trace tests verify representative samples to
 *   ensure bitmask encoding is correct.
 *
 * - Combined log flag variants (e.g., LOG_SUBDOMAINS_OFF + LOG_NEW_EXEC_ON):
 *   individual flag tests above cover each flag's effect on trace fields.  Flag
 *   combination logic is validated by audit_test.c:audit_flags tests.
 *
 * - fs.refer multi-record denials and fs.change_topology (mount):
 *   trace_denial() uses the same code path for all FS request types.  The
 *   DENTRY union member is validated by the deny_access_fs_fields
 *   test.  Audit tests in fs_test.c cover refer and mount denial specifics.
 */

// TEST_HARNESS_MAIN
TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
