// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Filesystem tracepoints
 *
 * Copyright © 2026 Cloudflare, Inc.
 */

/* Translated from trace_fs_test.c.  C dependencies: errno.h, fcntl.h,
 * linux/landlock.h, sched.h, stdio.h, string.h, sys/mount.h, sys/stat.h,
 * sys/types.h, sys/wait.h, unistd.h, common.h, trace.h.
 */

const TRACE_TASK: &str = "trace_fs_test";

/*
 * Like REGEX_DENY_ACCESS_FS(), but pins the logged field to a specific value
 * ("0" or "1") so a test can tell a suppressed (quiet) denial from a logged
 * one.  The tracepoint fires for every denial; logged carries the audit
 * verdict.
 */
macro_rules! REGEX_DENY_ACCESS_FS_LOGGED {
	($task:expr, $log:expr) => {
		concat!(
			TRACE_PREFIX!($task),
			"landlock_deny_access_fs: ",
			"domain=[0-9a-f]\\+ ",
			"same_exec=[01] ",
			"logged=", $log, " ",
			"blockers=[a-z_|]* ",
			"dev=[0-9]\\+:[0-9]\\+ ",
			"ino=[0-9]\\+ ",
			"path=[^ ]*$"
		)
	};
}

#[repr(C)]
struct trace_fs {
	tracefs_ok: libc::c_int,
}

FIXTURE_SETUP!(trace_fs, {
	let mut ret: libc::c_int;

	unsafe {
		set_cap(_metadata, CAP_SYS_ADMIN);
		ASSERT_EQ!(0, unshare(CLONE_NEWNS));
		ASSERT_EQ!(0, mount(std::ptr::null(), c"/".as_ptr(), std::ptr::null(), MS_REC | MS_PRIVATE, std::ptr::null()));

		ret = tracefs_fixture_setup();
		if ret != 0 {
			clear_cap(_metadata, CAP_SYS_ADMIN);
			(*self).tracefs_ok = 0;
			SKIP!(return, "tracefs not available");
		}
		(*self).tracefs_ok = 1;

		ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_ADD_RULE_FS_ENABLE, true));
		ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_CHECK_RULE_FS_ENABLE, true));
		ASSERT_EQ!(0, tracefs_enable_event(TRACEFS_DENY_ACCESS_FS_ENABLE, true));
		ASSERT_EQ!(0, tracefs_clear());
		clear_cap(_metadata, CAP_SYS_ADMIN);
	}
});

FIXTURE_TEARDOWN!(trace_fs, {
	unsafe {
		if (*self).tracefs_ok == 0 {
			return;
		}

		set_cap(_metadata, CAP_SYS_ADMIN);
		tracefs_enable_event(TRACEFS_ADD_RULE_FS_ENABLE, false);
		tracefs_enable_event(TRACEFS_CHECK_RULE_FS_ENABLE, false);
		tracefs_enable_event(TRACEFS_DENY_ACCESS_FS_ENABLE, false);
		tracefs_fixture_teardown();
		clear_cap(_metadata, CAP_SYS_ADMIN);
	}
});

/*
 * Baseline: verifies that without Landlock, the operation succeeds and no
 * check_rule or deny_access trace events fire.
 */
TEST_F!(trace_fs, unsandboxed, {
	let mut buf: *mut libc::c_char;
	let mut count: libc::c_int;
	let mut status: libc::c_int = 0;
	let mut fd: libc::c_int;
	let mut pid: libc::pid_t;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		pid = fork();
		ASSERT_LE!(0, pid);

		if pid == 0 {
			/*
			 * No sandbox: verify that a normal FS access does not produce
			 * Landlock trace events.
			 */
			fd = open(c"/usr".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
			if fd >= 0 {
				close(fd);
			}
			_exit(0);
		}

		ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
		ASSERT_TRUE!(WIFEXITED(status));
		EXPECT_EQ!(0, WEXITSTATUS(status));

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count = tracefs_count_matches(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK));
		EXPECT_EQ!(0, count);
		count = tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK));
		EXPECT_EQ!(0, count);

		free(buf as *mut libc::c_void);
	}
});

/*
 * Verifies that adding a filesystem rule emits a landlock_add_rule_fs trace
 * event with the expected path and field values: ruleset ID is non-zero,
 * access_rights is non-zero, and path matches.
 */
TEST_F!(trace_fs, add_rule_fs, {
	let mut ruleset_attr = landlock_ruleset_attr {
		handled_access_fs: LANDLOCK_ACCESS_FS_READ_FILE |
				   LANDLOCK_ACCESS_FS_WRITE_FILE |
				   LANDLOCK_ACCESS_FS_READ_DIR,
		..unsafe { std::mem::zeroed() }
	};
	let mut path_beneath = landlock_path_beneath_attr {
		allowed_access: LANDLOCK_ACCESS_FS_READ_FILE,
		..unsafe { std::mem::zeroed() }
	};
	let mut buf: *mut libc::c_char;
	let mut field_buf: [libc::c_char; 64] = [0; 64];
	let mut ruleset_fd: libc::c_int;
	let mut count: libc::c_int;

	unsafe {
		ruleset_fd =
			landlock_create_ruleset(&mut ruleset_attr, std::mem::size_of_val(&ruleset_attr), 0);
		ASSERT_LE!(0, ruleset_fd);

		path_beneath.parent_fd = open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC);
		ASSERT_LE!(0, path_beneath.parent_fd);

		ASSERT_EQ!(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
						&mut path_beneath, 0));
		ASSERT_EQ!(0, close(path_beneath.parent_fd));
		ASSERT_EQ!(0, close(ruleset_fd));

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count = tracefs_count_matches(buf, REGEX_ADD_RULE_FS!(TRACE_TASK));
		EXPECT_EQ!(1, count, {
			TH_LOG!("Expected 1 add_rule_fs event, got %d\n%s", count, buf);
		});

		/* Ruleset ID should be non-zero. */
		ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_ADD_RULE_FS!(TRACE_TASK),
						    c"ruleset".as_ptr(), field_buf.as_mut_ptr(),
						    field_buf.len()));
		EXPECT_STRNE!(c"0".as_ptr(), field_buf.as_ptr());

		/* Access rights should be non-zero. */
		ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_ADD_RULE_FS!(TRACE_TASK),
						    c"access_rights".as_ptr(), field_buf.as_mut_ptr(),
						    field_buf.len()));
		EXPECT_STRNE!(c"".as_ptr(), field_buf.as_ptr());

		/* Path should be /usr. */
		ASSERT_EQ!(0,
			   tracefs_extract_field(buf, REGEX_ADD_RULE_FS!(TRACE_TASK),
						 c"path".as_ptr(), field_buf.as_mut_ptr(), field_buf.len()));
		EXPECT_STREQ!(c"/usr".as_ptr(), field_buf.as_ptr());

		free(buf as *mut libc::c_void);
	}
});

/*
 * Verifies that an allowed access emits check_rule events (rule matched during
 * pathwalk) but does NOT emit deny_access events (no denial).
 */
TEST_F!(trace_fs, allowed_access, {
	let mut buf: *mut libc::c_char;
	let mut field_buf: [libc::c_char; 64] = [0; 64];
	let mut count: libc::c_int;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		/* Rule allows READ_DIR for /usr, access /usr which is allowed. */
		sandbox_child_fs_access(_metadata, c"/usr".as_ptr(), LANDLOCK_ACCESS_FS_READ_DIR,
					LANDLOCK_ACCESS_FS_READ_DIR, c"/usr".as_ptr());

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count = tracefs_count_matches(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK));
		EXPECT_LE!(1, count);

		/* Single-layer grants array, intersected with the request. */
		ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK),
						    c"grants".as_ptr(), field_buf.as_mut_ptr(),
						    field_buf.len()));
		EXPECT_STREQ!(c"{read_dir}".as_ptr(), field_buf.as_ptr());

		count = tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK));
		EXPECT_EQ!(0, count);

		free(buf as *mut libc::c_void);
	}
});

/*
 * Verifies that accessing a path whose access type is not in the handled set
 * does not emit landlock_check_rule events.  The ruleset handles READ_FILE, but
 * the directory open checks READ_DIR which is unhandled; Landlock has no
 * opinion and no rule evaluation occurs.
 */
TEST_F!(trace_fs, check_rule_unhandled, {
	let mut buf: *mut libc::c_char;
	let mut count: libc::c_int;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		/* Handles READ_FILE only; READ_DIR is unhandled. */
		sandbox_child_fs_access(_metadata, c"/usr".as_ptr(), LANDLOCK_ACCESS_FS_READ_FILE,
					LANDLOCK_ACCESS_FS_READ_FILE, c"/tmp".as_ptr());

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		/* No check_rule events because READ_DIR is not in the handled set. */
		count = tracefs_count_matches(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK));
		EXPECT_EQ!(0, count);

		free(buf as *mut libc::c_void);
	}
});

/*
 * Verifies that nested domains (child sandboxed under a parent domain) emit
 * check_rule events from both layers and produce a deny_access event when the
 * inner domain's rule does not cover the access.
 */
TEST_F!(trace_fs, check_rule_nested, {
	let mut buf: *mut libc::c_char;
	let mut field_buf: [libc::c_char; 64] = [0; 64];
	let mut comma: *mut libc::c_char;
	let mut first_len: libc::size_t;
	let mut second_len: libc::size_t;
	let mut count_rule: libc::c_int;
	let mut count_access: libc::c_int;
	let mut status: libc::c_int = 0;
	let mut pid: libc::pid_t;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		pid = fork();
		ASSERT_LE!(0, pid);

		if pid == 0 {
			let mut ruleset_attr = landlock_ruleset_attr {
				handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
				..std::mem::zeroed()
			};
			let mut path_beneath = landlock_path_beneath_attr {
				allowed_access: LANDLOCK_ACCESS_FS_READ_DIR,
				..std::mem::zeroed()
			};
			let mut ruleset_fd: libc::c_int;
			let mut fd: libc::c_int;

			/* First layer: allow /usr. */
			ruleset_fd = landlock_create_ruleset(&mut ruleset_attr,
							     std::mem::size_of_val(&ruleset_attr), 0);
			if ruleset_fd < 0 {
				_exit(1);
			}

			path_beneath.parent_fd =
				open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC);
			if path_beneath.parent_fd < 0 {
				close(ruleset_fd);
				_exit(1);
			}

			if landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
					     &mut path_beneath, 0) != 0 {
				close(path_beneath.parent_fd);
				close(ruleset_fd);
				_exit(1);
			}
			close(path_beneath.parent_fd);

			prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
			if landlock_restrict_self(ruleset_fd, 0) != 0 {
				close(ruleset_fd);
				_exit(1);
			}
			close(ruleset_fd);

			/* Second layer: also allow /usr. */
			ruleset_fd = landlock_create_ruleset(&mut ruleset_attr,
							     std::mem::size_of_val(&ruleset_attr), 0);
			if ruleset_fd < 0 {
				_exit(1);
			}

			path_beneath.parent_fd =
				open(c"/usr".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC);
			if path_beneath.parent_fd < 0 {
				close(ruleset_fd);
				_exit(1);
			}

			if landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
					     &mut path_beneath, 0) != 0 {
				close(path_beneath.parent_fd);
				close(ruleset_fd);
				_exit(1);
			}
			close(path_beneath.parent_fd);

			if landlock_restrict_self(ruleset_fd, 0) != 0 {
				close(ruleset_fd);
				_exit(1);
			}
			close(ruleset_fd);

			/* Access /usr which is allowed by both layers. */
			fd = open(c"/usr".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
			if fd >= 0 {
				close(fd);
			}

			/* Access /tmp which has no rule in either layer. */
			fd = open(c"/tmp".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
			if fd >= 0 {
				close(fd);
			}

			_exit(0);
		}

		ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
		ASSERT_TRUE!(WIFEXITED(status));
		EXPECT_EQ!(0, WEXITSTATUS(status));

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count_rule =
			tracefs_count_matches(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK));
		EXPECT_LE!(1, count_rule);

		/*
		 * Both layers have the same rule, so the grants array must have two
		 * identical symbolic entries, e.g. {read_dir,read_dir}.
		 */
		ASSERT_EQ!(0, tracefs_extract_field(buf, REGEX_CHECK_RULE_FS!(TRACE_TASK),
						    c"grants".as_ptr(), field_buf.as_mut_ptr(),
						    field_buf.len()));
		comma = strchr(field_buf.as_mut_ptr(), b',' as libc::c_int);
		EXPECT_NE!(0, (comma != std::ptr::null_mut()) as libc::c_int);
		if !comma.is_null() {
			/*
			 * Verify both entries are identical: compare the substring
			 * before the comma with the substring after it (stripping the
			 * braces).
			 */
			first_len = comma.offset_from(field_buf.as_mut_ptr()) as libc::size_t - 1;
			second_len = strlen(comma.add(1)) - 1;
			EXPECT_EQ!(first_len, second_len);
			EXPECT_EQ!(0, strncmp(field_buf.as_mut_ptr().add(1), comma.add(1), first_len));
		}

		count_access =
			tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK));
		EXPECT_LE!(1, count_access);

		free(buf as *mut libc::c_void);
	}
});

/*
 * Verifies that a denied FS access emits a landlock_deny_access_fs trace event
 * with the blocked access and path.
 */
TEST_F!(trace_fs, deny_access_fs_denied, {
	let mut buf: *mut libc::c_char;
	let mut count: libc::c_int;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		/*
		 * Rule allows READ_DIR for /usr, but access /tmp which has no rule.
		 * READ_DIR access to /tmp is denied by absence and should emit a
		 * deny_access_fs event.
		 */
		sandbox_child_fs_access(_metadata, c"/usr".as_ptr(), LANDLOCK_ACCESS_FS_READ_DIR,
					LANDLOCK_ACCESS_FS_READ_DIR, c"/tmp".as_ptr());

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		count = tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS!(TRACE_TASK));
		EXPECT_LE!(1, count);

		free(buf as *mut libc::c_void);
	}
});

/*
 * A denied FS access covered by a quiet rule (LANDLOCK_ADD_RULE_QUIET with the
 * access listed in quiet_access_fs) still emits a landlock_deny_access_fs
 * event, but with logged=0, the same audit-logging verdict audit would apply to
 * suppress the record.
 */
TEST_F!(trace_fs, deny_access_fs_quiet, {
	let mut buf: *mut libc::c_char;
	let mut field: [libc::c_char; 64] = [0; 64];
	let mut pid: libc::pid_t;
	let mut status: libc::c_int = 0;

	unsafe {
		ASSERT_EQ!(0, tracefs_clear_buf());

		pid = fork();
		ASSERT_LE!(0, pid);
		if pid == 0 {
			let mut ruleset_attr = landlock_ruleset_attr {
				handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
				quiet_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
				..std::mem::zeroed()
			};
			let mut path_beneath = landlock_path_beneath_attr {
				allowed_access: 0,
				..std::mem::zeroed()
			};
			let mut ruleset_fd: libc::c_int;
			let mut fd: libc::c_int;

			ruleset_fd = landlock_create_ruleset(&mut ruleset_attr,
							     std::mem::size_of_val(&ruleset_attr), 0);
			if ruleset_fd < 0 {
				_exit(1);
			}

			/* Marks /tmp quiet without granting any access. */
			path_beneath.parent_fd =
				open(c"/tmp".as_ptr(), O_PATH | O_DIRECTORY | O_CLOEXEC);
			if path_beneath.parent_fd < 0 {
				close(ruleset_fd);
				_exit(1);
			}
			if landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
					     &mut path_beneath, LANDLOCK_ADD_RULE_QUIET) != 0 {
				close(path_beneath.parent_fd);
				close(ruleset_fd);
				_exit(1);
			}
			close(path_beneath.parent_fd);

			prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
			if landlock_restrict_self(ruleset_fd, 0) != 0 {
				close(ruleset_fd);
				_exit(1);
			}
			close(ruleset_fd);

			/* Denied READ_DIR on the quiet /tmp: suppressed, logged=0. */
			fd = open(c"/tmp".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
			if fd >= 0 {
				close(fd);
			}
			_exit(0);
		}
		ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
		ASSERT_TRUE!(WIFEXITED(status));
		EXPECT_EQ!(0, WEXITSTATUS(status));

		buf = tracefs_read_buf();
		ASSERT_NE!(std::ptr::null_mut(), buf);

		/* The event fires with the suppressed verdict. */
		EXPECT_LE!(1, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS_LOGGED!(
							TRACE_TASK, "0")));
		/* The quiet rule must not leave the denial logged. */
		EXPECT_EQ!(0, tracefs_count_matches(buf, REGEX_DENY_ACCESS_FS_LOGGED!(
							TRACE_TASK, "1")));

		/*
		 * Quiet suppresses only the logged verdict: the rest of the denial
		 * event stays populated (non-zero domain, non-empty blockers).
		 */
		ASSERT_EQ!(0, tracefs_extract_field(
				     buf, REGEX_DENY_ACCESS_FS_LOGGED!(TRACE_TASK, "0"),
				     c"domain".as_ptr(), field.as_mut_ptr(), field.len()));
		EXPECT_STRNE!(c"0".as_ptr(), field.as_ptr());
		ASSERT_EQ!(0, tracefs_extract_field(
				     buf, REGEX_DENY_ACCESS_FS_LOGGED!(TRACE_TASK, "0"),
				     c"blockers".as_ptr(), field.as_mut_ptr(), field.len()));
		EXPECT_STRNE!(c"".as_ptr(), field.as_ptr());

		free(buf as *mut libc::c_void);
	}
});

TEST_HARNESS_MAIN!();
