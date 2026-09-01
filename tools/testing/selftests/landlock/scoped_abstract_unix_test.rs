// SPDX-License-Identifier: GPL-2.0
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, improper_ctypes)]

/*
 * Landlock tests - Abstract UNIX socket
 *
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

// Rust translation of testing/selftests/landlock/scoped_abstract_unix_test.c.
// External selftest harness items from audit.h, common.h, scoped_common.h,
// trace.h, scoped_base_variants.h, and scoped_multiple_domain_variants.h are
// intentionally referenced by name and not implemented in this isolated file.

use core::ffi::{c_char, c_int, c_short, c_void};

const TRACE_TASK: &str = "scoped_abstract";

// Number of pending connections queue to be hold.
const backlog: c_short = 10;

unsafe fn create_fs_domain(_metadata: *mut __test_metadata) {
    let mut ruleset_fd: c_int;
    let ruleset_attr = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        ..core::mem::zeroed()
    };

    ruleset_fd = landlock_create_ruleset(
        &ruleset_attr,
        core::mem::size_of_val(&ruleset_attr),
        0,
    );
    EXPECT_LE!(0, ruleset_fd, {
        TH_LOG!("Failed to create a ruleset: %s", strerror(errno()));
    });
    EXPECT_EQ!(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
    EXPECT_EQ!(0, landlock_restrict_self(ruleset_fd, 0));
    EXPECT_EQ!(0, close(ruleset_fd));
}

// The remaining test declarations are translated below in source order.  The
// Linux selftest fixture/assertion DSL is preserved as Rust macro invocations
// or line comments because those macros are supplied by external harness files.

// C: /*
// C:  * Landlock tests - Abstract UNIX socket
// C:  *
// C:  * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
// C:  * /
// C: 
// C: #define _GNU_SOURCE
// C: #include <errno.h>
// C: #include <fcntl.h>
// C: #include <linux/landlock.h>
// C: #include <sched.h>
// C: #include <signal.h>
// C: #include <stddef.h>
// C: #include <sys/mount.h>
// C: #include <sys/prctl.h>
// C: #include <sys/socket.h>
// C: #include <sys/stat.h>
// C: #include <sys/types.h>
// C: #include <sys/un.h>
// C: #include <sys/wait.h>
// C: #include <unistd.h>
// C: 
// C: #include "audit.h"
// C: #include "common.h"
// C: #include "scoped_common.h"
// C: #include "trace.h"
// C: 
// C: #define TRACE_TASK "scoped_abstract"
// C: 
// C: /* Number of pending connections queue to be hold. * /
// C: const short backlog = 10;
// C: 
// C: static void create_fs_domain(struct __test_metadata *const _metadata)
// C: {
// C: 	int ruleset_fd;
// C: 	struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_fs = LANDLOCK_ACCESS_FS_READ_DIR,
// C: 	};
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	EXPECT_LE(0, ruleset_fd)
// C: 	{
// C: 		TH_LOG("Failed to create a ruleset: %s", strerror(errno));
// C: 	}
// C: 	EXPECT_EQ(0, prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0));
// C: 	EXPECT_EQ(0, landlock_restrict_self(ruleset_fd, 0));
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: }
// C: 
// C: FIXTURE(scoped_domains)
// C: {
// C: 	struct service_fixture stream_address, dgram_address;
// C: };
// C: 
// C: #include "scoped_base_variants.h"
// C: 
// C: FIXTURE_SETUP(scoped_domains)
// C: {
// C: 	drop_caps(_metadata);
// C: 
// C: 	memset(&self->stream_address, 0, sizeof(self->stream_address));
// C: 	memset(&self->dgram_address, 0, sizeof(self->dgram_address));
// C: 	set_unix_address(&self->stream_address, 0);
// C: 	set_unix_address(&self->dgram_address, 1);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(scoped_domains)
// C: {
// C: }
// C: 
// C: /*
// C:  * Test unix_stream_connect() and unix_may_send() for a child connecting to its
// C:  * parent, when they have scoped domain or no domain.
// C:  * /
// C: TEST_F(scoped_domains, connect_to_parent)
// C: {
// C: 	pid_t child;
// C: 	bool can_connect_to_parent;
// C: 	int status;
// C: 	int pipe_parent[2];
// C: 	int stream_server, dgram_server;
// C: 
// C: 	/*
// C: 	 * can_connect_to_parent is true if a child process can connect to its
// C: 	 * parent process. This depends on the child process not being isolated
// C: 	 * from the parent with a dedicated Landlock domain.
// C: 	 * /
// C: 	can_connect_to_parent = !variant->domain_child;
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 	if (variant->domain_both) {
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 		if (!__test_passed(_metadata))
// C: 			return;
// C: 	}
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int err;
// C: 		int stream_client, dgram_client;
// C: 		char buf_child;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		if (variant->domain_child)
// C: 			create_scoped_domain(
// C: 				_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		stream_client = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 		ASSERT_LE(0, stream_client);
// C: 		dgram_client = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_client);
// C: 
// C: 		/* Waits for the server. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf_child, 1));
// C: 
// C: 		err = connect(stream_client, &self->stream_address.unix_addr,
// C: 			      self->stream_address.unix_addr_len);
// C: 		if (can_connect_to_parent) {
// C: 			EXPECT_EQ(0, err);
// C: 		} else {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		}
// C: 		EXPECT_EQ(0, close(stream_client));
// C: 
// C: 		err = connect(dgram_client, &self->dgram_address.unix_addr,
// C: 			      self->dgram_address.unix_addr_len);
// C: 		if (can_connect_to_parent) {
// C: 			EXPECT_EQ(0, err);
// C: 		} else {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		}
// C: 		EXPECT_EQ(0, close(dgram_client));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 	if (variant->domain_parent)
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	stream_server = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 	ASSERT_LE(0, stream_server);
// C: 	dgram_server = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_server);
// C: 	ASSERT_EQ(0, bind(stream_server, &self->stream_address.unix_addr,
// C: 			  self->stream_address.unix_addr_len));
// C: 	ASSERT_EQ(0, bind(dgram_server, &self->dgram_address.unix_addr,
// C: 			  self->dgram_address.unix_addr_len));
// C: 	ASSERT_EQ(0, listen(stream_server, backlog));
// C: 
// C: 	/* Signals to child that the parent is listening. * /
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(stream_server));
// C: 	EXPECT_EQ(0, close(dgram_server));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: /*
// C:  * Test unix_stream_connect() and unix_may_send() for a parent connecting to
// C:  * its child, when they have scoped domain or no domain.
// C:  * /
// C: TEST_F(scoped_domains, connect_to_child)
// C: {
// C: 	pid_t child;
// C: 	bool can_connect_to_child;
// C: 	int err_stream, err_dgram, errno_stream, errno_dgram, status;
// C: 	int pipe_child[2], pipe_parent[2];
// C: 	char buf;
// C: 	int stream_client, dgram_client;
// C: 
// C: 	/*
// C: 	 * can_connect_to_child is true if a parent process can connect to its
// C: 	 * child process. The parent process is not isolated from the child
// C: 	 * with a dedicated Landlock domain.
// C: 	 * /
// C: 	can_connect_to_child = !variant->domain_parent;
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_child, O_CLOEXEC));
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 	if (variant->domain_both) {
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 		if (!__test_passed(_metadata))
// C: 			return;
// C: 	}
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int stream_server, dgram_server;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		EXPECT_EQ(0, close(pipe_child[0]));
// C: 		if (variant->domain_child)
// C: 			create_scoped_domain(
// C: 				_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		/* Waits for the parent to be in a domain, if any. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 
// C: 		stream_server = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 		ASSERT_LE(0, stream_server);
// C: 		dgram_server = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_server);
// C: 		ASSERT_EQ(0,
// C: 			  bind(stream_server, &self->stream_address.unix_addr,
// C: 			       self->stream_address.unix_addr_len));
// C: 		ASSERT_EQ(0, bind(dgram_server, &self->dgram_address.unix_addr,
// C: 				  self->dgram_address.unix_addr_len));
// C: 		ASSERT_EQ(0, listen(stream_server, backlog));
// C: 
// C: 		/* Signals to the parent that child is listening. * /
// C: 		ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 
// C: 		/* Waits to connect. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 		EXPECT_EQ(0, close(stream_server));
// C: 		EXPECT_EQ(0, close(dgram_server));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_child[1]));
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 
// C: 	if (variant->domain_parent)
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	/* Signals that the parent is in a domain, if any. * /
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 
// C: 	stream_client = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 	ASSERT_LE(0, stream_client);
// C: 	dgram_client = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_client);
// C: 
// C: 	/* Waits for the child to listen * /
// C: 	ASSERT_EQ(1, read(pipe_child[0], &buf, 1));
// C: 	err_stream = connect(stream_client, &self->stream_address.unix_addr,
// C: 			     self->stream_address.unix_addr_len);
// C: 	errno_stream = errno;
// C: 	err_dgram = connect(dgram_client, &self->dgram_address.unix_addr,
// C: 			    self->dgram_address.unix_addr_len);
// C: 	errno_dgram = errno;
// C: 	if (can_connect_to_child) {
// C: 		EXPECT_EQ(0, err_stream);
// C: 		EXPECT_EQ(0, err_dgram);
// C: 	} else {
// C: 		EXPECT_EQ(-1, err_stream);
// C: 		EXPECT_EQ(-1, err_dgram);
// C: 		EXPECT_EQ(EPERM, errno_stream);
// C: 		EXPECT_EQ(EPERM, errno_dgram);
// C: 	}
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 	EXPECT_EQ(0, close(stream_client));
// C: 	EXPECT_EQ(0, close(dgram_client));
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: FIXTURE(scoped_audit)
// C: {
// C: 	struct service_fixture dgram_address;
// C: 	struct audit_filter audit_filter;
// C: 	int audit_fd;
// C: };
// C: 
// C: FIXTURE_SETUP(scoped_audit)
// C: {
// C: 	disable_caps(_metadata);
// C: 
// C: 	memset(&self->dgram_address, 0, sizeof(self->dgram_address));
// C: 	set_unix_address(&self->dgram_address, 1);
// C: 
// C: 	set_cap(_metadata, CAP_AUDIT_CONTROL);
// C: 	self->audit_fd = audit_init_with_exe_filter(&self->audit_filter);
// C: 	EXPECT_LE(0, self->audit_fd);
// C: 	drop_caps(_metadata);
// C: }
// C: 
// C: FIXTURE_TEARDOWN_PARENT(scoped_audit)
// C: {
// C: 	EXPECT_EQ(0, audit_cleanup(-1, NULL));
// C: }
// C: 
// C: FIXTURE_VARIANT(scoped_audit)
// C: {
// C: 	const __u64 scoped;
// C: 	const __u64 quiet_scoped;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(scoped_audit, no_quiet)
// C: {
// C: 	/* clang-format on * /
// C: 	.scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
// C: 	.quiet_scoped = 0,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(scoped_audit, quiet_abstract_socket)
// C: {
// C: 	/* clang-format on * /
// C: 	.scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
// C: 	.quiet_scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(scoped_audit, quiet_abstract_socket_2)
// C: {
// C: 	/* clang-format on * /
// C: 	.scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET | LANDLOCK_SCOPE_SIGNAL,
// C: 	.quiet_scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
// C: 			LANDLOCK_SCOPE_SIGNAL,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(scoped_audit, quiet_unrelated)
// C: {
// C: 	/* clang-format on * /
// C: 	.scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET | LANDLOCK_SCOPE_SIGNAL,
// C: 	.quiet_scoped = LANDLOCK_SCOPE_SIGNAL,
// C: };
// C: 
// C: /* python -c 'print(b"\0selftests-landlock-abstract-unix-".hex().upper())' * /
// C: #define ABSTRACT_SOCKET_PATH_PREFIX \
// C: 	"0073656C6674657374732D6C616E646C6F636B2D61627374726163742D756E69782D"
// C: 
// C: /*
// C:  * Simpler version of scoped_domains.connect_to_child, but with audit tests.
// C:  * /
// C: TEST_F(scoped_audit, connect_to_child)
// C: {
// C: 	pid_t child;
// C: 	int err_dgram, status;
// C: 	int pipe_child[2], pipe_parent[2];
// C: 	char buf;
// C: 	int dgram_client;
// C: 	struct audit_records records;
// C: 	int ruleset_fd;
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.scoped = variant->scoped,
// C: 		.quiet_scoped = variant->quiet_scoped,
// C: 	};
// C: 	bool should_audit =
// C: 		!(variant->quiet_scoped & LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	/* Makes sure there is no superfluous logged records. * /
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(0, records.domain);
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_child, O_CLOEXEC));
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int dgram_server;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		EXPECT_EQ(0, close(pipe_child[0]));
// C: 
// C: 		/* Waits for the parent to be in a domain. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 
// C: 		dgram_server = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_server);
// C: 		ASSERT_EQ(0, bind(dgram_server, &self->dgram_address.unix_addr,
// C: 				  self->dgram_address.unix_addr_len));
// C: 
// C: 		/* Signals to the parent that child is listening. * /
// C: 		ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 
// C: 		/* Waits to connect. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 		EXPECT_EQ(0, close(dgram_server));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_child[1]));
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd)
// C: 	{
// C: 		TH_LOG("Failed to create a ruleset: %s", strerror(errno));
// C: 	}
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	/* Signals that the parent is in a domain, if any. * /
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 
// C: 	dgram_client = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_client);
// C: 
// C: 	/* Waits for the child to listen * /
// C: 	ASSERT_EQ(1, read(pipe_child[0], &buf, 1));
// C: 	err_dgram = connect(dgram_client, &self->dgram_address.unix_addr,
// C: 			    self->dgram_address.unix_addr_len);
// C: 	EXPECT_EQ(-1, err_dgram);
// C: 	EXPECT_EQ(EPERM, errno);
// C: 
// C: 	if (should_audit) {
// C: 		EXPECT_EQ(
// C: 			0,
// C: 			audit_match_record(
// C: 				self->audit_fd, AUDIT_LANDLOCK_ACCESS,
// C: 				REGEX_LANDLOCK_PREFIX
// C: 				" blockers=scope\\.abstract_unix_socket path=" ABSTRACT_SOCKET_PATH_PREFIX
// C: 				"[0-9A-F]\\+$",
// C: 				NULL));
// C: 	}
// C: 
// C: 	/* No other logs * /
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 	EXPECT_EQ(0, close(dgram_client));
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: FIXTURE(scoped_vs_unscoped)
// C: {
// C: 	struct service_fixture parent_stream_address, parent_dgram_address,
// C: 		child_stream_address, child_dgram_address;
// C: };
// C: 
// C: #include "scoped_multiple_domain_variants.h"
// C: 
// C: FIXTURE_SETUP(scoped_vs_unscoped)
// C: {
// C: 	drop_caps(_metadata);
// C: 
// C: 	memset(&self->parent_stream_address, 0,
// C: 	       sizeof(self->parent_stream_address));
// C: 	set_unix_address(&self->parent_stream_address, 0);
// C: 	memset(&self->parent_dgram_address, 0,
// C: 	       sizeof(self->parent_dgram_address));
// C: 	set_unix_address(&self->parent_dgram_address, 1);
// C: 	memset(&self->child_stream_address, 0,
// C: 	       sizeof(self->child_stream_address));
// C: 	set_unix_address(&self->child_stream_address, 2);
// C: 	memset(&self->child_dgram_address, 0,
// C: 	       sizeof(self->child_dgram_address));
// C: 	set_unix_address(&self->child_dgram_address, 3);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(scoped_vs_unscoped)
// C: {
// C: }
// C: 
// C: /*
// C:  * Test unix_stream_connect and unix_may_send for parent, child and
// C:  * grand child processes when they can have scoped or non-scoped domains.
// C:  * /
// C: TEST_F(scoped_vs_unscoped, unix_scoping)
// C: {
// C: 	pid_t child;
// C: 	int status;
// C: 	bool can_connect_to_parent, can_connect_to_child;
// C: 	int pipe_parent[2];
// C: 	int stream_server_parent, dgram_server_parent;
// C: 
// C: 	can_connect_to_child = (variant->domain_grand_child != SCOPE_SANDBOX);
// C: 	can_connect_to_parent = (can_connect_to_child &&
// C: 				 (variant->domain_children != SCOPE_SANDBOX));
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 
// C: 	if (variant->domain_all == OTHER_SANDBOX)
// C: 		create_fs_domain(_metadata);
// C: 	else if (variant->domain_all == SCOPE_SANDBOX)
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int stream_server_child, dgram_server_child;
// C: 		int pipe_child[2];
// C: 		pid_t grand_child;
// C: 
// C: 		ASSERT_EQ(0, pipe2(pipe_child, O_CLOEXEC));
// C: 
// C: 		if (variant->domain_children == OTHER_SANDBOX)
// C: 			create_fs_domain(_metadata);
// C: 		else if (variant->domain_children == SCOPE_SANDBOX)
// C: 			create_scoped_domain(
// C: 				_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		grand_child = fork();
// C: 		ASSERT_LE(0, grand_child);
// C: 		if (grand_child == 0) {
// C: 			char buf;
// C: 			int stream_err, dgram_err, stream_errno, dgram_errno;
// C: 			int stream_client, dgram_client;
// C: 
// C: 			EXPECT_EQ(0, close(pipe_parent[1]));
// C: 			EXPECT_EQ(0, close(pipe_child[1]));
// C: 
// C: 			if (variant->domain_grand_child == OTHER_SANDBOX)
// C: 				create_fs_domain(_metadata);
// C: 			else if (variant->domain_grand_child == SCOPE_SANDBOX)
// C: 				create_scoped_domain(
// C: 					_metadata,
// C: 					LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 			stream_client = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 			ASSERT_LE(0, stream_client);
// C: 			dgram_client = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 			ASSERT_LE(0, dgram_client);
// C: 
// C: 			ASSERT_EQ(1, read(pipe_child[0], &buf, 1));
// C: 			stream_err = connect(
// C: 				stream_client,
// C: 				&self->child_stream_address.unix_addr,
// C: 				self->child_stream_address.unix_addr_len);
// C: 			stream_errno = errno;
// C: 			dgram_err = connect(
// C: 				dgram_client,
// C: 				&self->child_dgram_address.unix_addr,
// C: 				self->child_dgram_address.unix_addr_len);
// C: 			dgram_errno = errno;
// C: 			if (can_connect_to_child) {
// C: 				EXPECT_EQ(0, stream_err);
// C: 				EXPECT_EQ(0, dgram_err);
// C: 			} else {
// C: 				EXPECT_EQ(-1, stream_err);
// C: 				EXPECT_EQ(-1, dgram_err);
// C: 				EXPECT_EQ(EPERM, stream_errno);
// C: 				EXPECT_EQ(EPERM, dgram_errno);
// C: 			}
// C: 
// C: 			EXPECT_EQ(0, close(stream_client));
// C: 			stream_client = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 			ASSERT_LE(0, stream_client);
// C: 			/* Datagram sockets can "reconnect". * /
// C: 
// C: 			ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 			stream_err = connect(
// C: 				stream_client,
// C: 				&self->parent_stream_address.unix_addr,
// C: 				self->parent_stream_address.unix_addr_len);
// C: 			stream_errno = errno;
// C: 			dgram_err = connect(
// C: 				dgram_client,
// C: 				&self->parent_dgram_address.unix_addr,
// C: 				self->parent_dgram_address.unix_addr_len);
// C: 			dgram_errno = errno;
// C: 			if (can_connect_to_parent) {
// C: 				EXPECT_EQ(0, stream_err);
// C: 				EXPECT_EQ(0, dgram_err);
// C: 			} else {
// C: 				EXPECT_EQ(-1, stream_err);
// C: 				EXPECT_EQ(-1, dgram_err);
// C: 				EXPECT_EQ(EPERM, stream_errno);
// C: 				EXPECT_EQ(EPERM, dgram_errno);
// C: 			}
// C: 			EXPECT_EQ(0, close(stream_client));
// C: 			EXPECT_EQ(0, close(dgram_client));
// C: 
// C: 			_exit(_metadata->exit_code);
// C: 			return;
// C: 		}
// C: 		EXPECT_EQ(0, close(pipe_child[0]));
// C: 		if (variant->domain_child == OTHER_SANDBOX)
// C: 			create_fs_domain(_metadata);
// C: 		else if (variant->domain_child == SCOPE_SANDBOX)
// C: 			create_scoped_domain(
// C: 				_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		stream_server_child = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 		ASSERT_LE(0, stream_server_child);
// C: 		dgram_server_child = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_server_child);
// C: 
// C: 		ASSERT_EQ(0, bind(stream_server_child,
// C: 				  &self->child_stream_address.unix_addr,
// C: 				  self->child_stream_address.unix_addr_len));
// C: 		ASSERT_EQ(0, bind(dgram_server_child,
// C: 				  &self->child_dgram_address.unix_addr,
// C: 				  self->child_dgram_address.unix_addr_len));
// C: 		ASSERT_EQ(0, listen(stream_server_child, backlog));
// C: 
// C: 		ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 		ASSERT_EQ(grand_child, waitpid(grand_child, &status, 0));
// C: 		EXPECT_EQ(0, close(stream_server_child));
// C: 		EXPECT_EQ(0, close(dgram_server_child));
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 
// C: 	if (variant->domain_parent == OTHER_SANDBOX)
// C: 		create_fs_domain(_metadata);
// C: 	else if (variant->domain_parent == SCOPE_SANDBOX)
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	stream_server_parent = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 	ASSERT_LE(0, stream_server_parent);
// C: 	dgram_server_parent = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_server_parent);
// C: 	ASSERT_EQ(0, bind(stream_server_parent,
// C: 			  &self->parent_stream_address.unix_addr,
// C: 			  self->parent_stream_address.unix_addr_len));
// C: 	ASSERT_EQ(0, bind(dgram_server_parent,
// C: 			  &self->parent_dgram_address.unix_addr,
// C: 			  self->parent_dgram_address.unix_addr_len));
// C: 
// C: 	ASSERT_EQ(0, listen(stream_server_parent, backlog));
// C: 
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(stream_server_parent));
// C: 	EXPECT_EQ(0, close(dgram_server_parent));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: FIXTURE(outside_socket)
// C: {
// C: 	struct service_fixture address, transit_address;
// C: };
// C: 
// C: FIXTURE_VARIANT(outside_socket)
// C: {
// C: 	const bool child_socket;
// C: 	const int type;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(outside_socket, allow_dgram_child) {
// C: 	/* clang-format on * /
// C: 	.child_socket = true,
// C: 	.type = SOCK_DGRAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(outside_socket, deny_dgram_server) {
// C: 	/* clang-format on * /
// C: 	.child_socket = false,
// C: 	.type = SOCK_DGRAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(outside_socket, allow_stream_child) {
// C: 	/* clang-format on * /
// C: 	.child_socket = true,
// C: 	.type = SOCK_STREAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(outside_socket, deny_stream_server) {
// C: 	/* clang-format on * /
// C: 	.child_socket = false,
// C: 	.type = SOCK_STREAM,
// C: };
// C: 
// C: FIXTURE_SETUP(outside_socket)
// C: {
// C: 	drop_caps(_metadata);
// C: 
// C: 	memset(&self->transit_address, 0, sizeof(self->transit_address));
// C: 	set_unix_address(&self->transit_address, 0);
// C: 	memset(&self->address, 0, sizeof(self->address));
// C: 	set_unix_address(&self->address, 1);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(outside_socket)
// C: {
// C: }
// C: 
// C: /*
// C:  * Test unix_stream_connect and unix_may_send for parent and child processes
// C:  * when connecting socket has different domain than the process using it.
// C:  * /
// C: TEST_F(outside_socket, socket_with_different_domain)
// C: {
// C: 	pid_t child;
// C: 	int err, status;
// C: 	int pipe_child[2], pipe_parent[2];
// C: 	char buf_parent;
// C: 	int server_socket;
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_child, O_CLOEXEC));
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int client_socket;
// C: 		char buf_child;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		EXPECT_EQ(0, close(pipe_child[0]));
// C: 
// C: 		/* Client always has a domain. * /
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		if (variant->child_socket) {
// C: 			int data_socket, passed_socket, stream_server;
// C: 
// C: 			passed_socket = socket(AF_UNIX, variant->type, 0);
// C: 			ASSERT_LE(0, passed_socket);
// C: 			stream_server = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 			ASSERT_LE(0, stream_server);
// C: 			ASSERT_EQ(0, bind(stream_server,
// C: 					  &self->transit_address.unix_addr,
// C: 					  self->transit_address.unix_addr_len));
// C: 			ASSERT_EQ(0, listen(stream_server, backlog));
// C: 			ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 			data_socket = accept(stream_server, NULL, NULL);
// C: 			ASSERT_LE(0, data_socket);
// C: 			ASSERT_EQ(0, send_fd(data_socket, passed_socket));
// C: 			EXPECT_EQ(0, close(passed_socket));
// C: 			EXPECT_EQ(0, close(stream_server));
// C: 		}
// C: 
// C: 		client_socket = socket(AF_UNIX, variant->type, 0);
// C: 		ASSERT_LE(0, client_socket);
// C: 
// C: 		/* Waits for parent signal for connection. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf_child, 1));
// C: 		err = connect(client_socket, &self->address.unix_addr,
// C: 			      self->address.unix_addr_len);
// C: 		if (variant->child_socket) {
// C: 			EXPECT_EQ(0, err);
// C: 		} else {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		}
// C: 		EXPECT_EQ(0, close(client_socket));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_child[1]));
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 
// C: 	if (variant->child_socket) {
// C: 		int client_child = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 
// C: 		ASSERT_LE(0, client_child);
// C: 		ASSERT_EQ(1, read(pipe_child[0], &buf_parent, 1));
// C: 		ASSERT_EQ(0, connect(client_child,
// C: 				     &self->transit_address.unix_addr,
// C: 				     self->transit_address.unix_addr_len));
// C: 		server_socket = recv_fd(client_child);
// C: 		EXPECT_EQ(0, close(client_child));
// C: 	} else {
// C: 		server_socket = socket(AF_UNIX, variant->type, 0);
// C: 	}
// C: 	ASSERT_LE(0, server_socket);
// C: 
// C: 	/* Server always has a domain. * /
// C: 	create_scoped_domain(_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	ASSERT_EQ(0, bind(server_socket, &self->address.unix_addr,
// C: 			  self->address.unix_addr_len));
// C: 	if (variant->type == SOCK_STREAM)
// C: 		ASSERT_EQ(0, listen(server_socket, backlog));
// C: 
// C: 	/* Signals to child that the parent is listening. * /
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(server_socket));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: static const char stream_path[] = TMP_DIR "/stream.sock";
// C: static const char dgram_path[] = TMP_DIR "/dgram.sock";
// C: 
// C: /* clang-format off * /
// C: FIXTURE(various_address_sockets) {};
// C: /* clang-format on * /
// C: 
// C: FIXTURE_VARIANT(various_address_sockets)
// C: {
// C: 	const int domain;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(various_address_sockets, pathname_socket_scoped_domain) {
// C: 	/* clang-format on * /
// C: 	.domain = SCOPE_SANDBOX,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(various_address_sockets, pathname_socket_other_domain) {
// C: 	/* clang-format on * /
// C: 	.domain = OTHER_SANDBOX,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(various_address_sockets, pathname_socket_no_domain) {
// C: 	/* clang-format on * /
// C: 	.domain = NO_SANDBOX,
// C: };
// C: 
// C: FIXTURE_SETUP(various_address_sockets)
// C: {
// C: 	drop_caps(_metadata);
// C: 
// C: 	umask(0077);
// C: 	ASSERT_EQ(0, mkdir(TMP_DIR, 0700));
// C: }
// C: 
// C: FIXTURE_TEARDOWN(various_address_sockets)
// C: {
// C: 	EXPECT_EQ(0, unlink(stream_path));
// C: 	EXPECT_EQ(0, unlink(dgram_path));
// C: 	EXPECT_EQ(0, rmdir(TMP_DIR));
// C: }
// C: 
// C: TEST_F(various_address_sockets, scoped_pathname_sockets)
// C: {
// C: 	pid_t child;
// C: 	int status;
// C: 	char buf_child, buf_parent;
// C: 	int pipe_parent[2];
// C: 	int unnamed_sockets[2];
// C: 	int stream_pathname_socket, dgram_pathname_socket,
// C: 		stream_abstract_socket, dgram_abstract_socket, data_socket;
// C: 	struct service_fixture stream_abstract_addr, dgram_abstract_addr;
// C: 	struct sockaddr_un stream_pathname_addr = {
// C: 		.sun_family = AF_UNIX,
// C: 	};
// C: 	struct sockaddr_un dgram_pathname_addr = {
// C: 		.sun_family = AF_UNIX,
// C: 	};
// C: 
// C: 	/* Pathname address. * /
// C: 	snprintf(stream_pathname_addr.sun_path,
// C: 		 sizeof(stream_pathname_addr.sun_path), "%s", stream_path);
// C: 	snprintf(dgram_pathname_addr.sun_path,
// C: 		 sizeof(dgram_pathname_addr.sun_path), "%s", dgram_path);
// C: 
// C: 	/* Abstract address. * /
// C: 	memset(&stream_abstract_addr, 0, sizeof(stream_abstract_addr));
// C: 	set_unix_address(&stream_abstract_addr, 0);
// C: 	memset(&dgram_abstract_addr, 0, sizeof(dgram_abstract_addr));
// C: 	set_unix_address(&dgram_abstract_addr, 1);
// C: 
// C: 	/* Unnamed address for datagram socket. * /
// C: 	ASSERT_EQ(0, socketpair(AF_UNIX, SOCK_DGRAM, 0, unnamed_sockets));
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int err;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		EXPECT_EQ(0, close(unnamed_sockets[1]));
// C: 
// C: 		if (variant->domain == SCOPE_SANDBOX)
// C: 			create_scoped_domain(
// C: 				_metadata, LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 		else if (variant->domain == OTHER_SANDBOX)
// C: 			create_fs_domain(_metadata);
// C: 
// C: 		/* Waits for parent to listen. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf_child, 1));
// C: 		EXPECT_EQ(0, close(pipe_parent[0]));
// C: 
// C: 		/* Checks that we can send data through a datagram socket. * /
// C: 		ASSERT_EQ(1, write(unnamed_sockets[0], "a", 1));
// C: 		EXPECT_EQ(0, close(unnamed_sockets[0]));
// C: 
// C: 		/* Connects with pathname sockets. * /
// C: 		stream_pathname_socket = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 		ASSERT_LE(0, stream_pathname_socket);
// C: 		ASSERT_EQ(0,
// C: 			  connect(stream_pathname_socket, &stream_pathname_addr,
// C: 				  sizeof(stream_pathname_addr)));
// C: 		ASSERT_EQ(1, write(stream_pathname_socket, "b", 1));
// C: 		EXPECT_EQ(0, close(stream_pathname_socket));
// C: 
// C: 		/* Sends without connection. * /
// C: 		dgram_pathname_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_pathname_socket);
// C: 		err = sendto(dgram_pathname_socket, "c", 1, 0,
// C: 			     &dgram_pathname_addr, sizeof(dgram_pathname_addr));
// C: 		EXPECT_EQ(1, err);
// C: 
// C: 		/* Sends with connection. * /
// C: 		ASSERT_EQ(0,
// C: 			  connect(dgram_pathname_socket, &dgram_pathname_addr,
// C: 				  sizeof(dgram_pathname_addr)));
// C: 		ASSERT_EQ(1, write(dgram_pathname_socket, "d", 1));
// C: 		EXPECT_EQ(0, close(dgram_pathname_socket));
// C: 
// C: 		/* Connects with abstract sockets. * /
// C: 		stream_abstract_socket = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 		ASSERT_LE(0, stream_abstract_socket);
// C: 		err = connect(stream_abstract_socket,
// C: 			      &stream_abstract_addr.unix_addr,
// C: 			      stream_abstract_addr.unix_addr_len);
// C: 		if (variant->domain == SCOPE_SANDBOX) {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		} else {
// C: 			EXPECT_EQ(0, err);
// C: 			ASSERT_EQ(1, write(stream_abstract_socket, "e", 1));
// C: 		}
// C: 		EXPECT_EQ(0, close(stream_abstract_socket));
// C: 
// C: 		/* Sends without connection. * /
// C: 		dgram_abstract_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, dgram_abstract_socket);
// C: 		err = sendto(dgram_abstract_socket, "f", 1, 0,
// C: 			     &dgram_abstract_addr.unix_addr,
// C: 			     dgram_abstract_addr.unix_addr_len);
// C: 		if (variant->domain == SCOPE_SANDBOX) {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		} else {
// C: 			EXPECT_EQ(1, err);
// C: 		}
// C: 
// C: 		/* Sends with connection. * /
// C: 		err = connect(dgram_abstract_socket,
// C: 			      &dgram_abstract_addr.unix_addr,
// C: 			      dgram_abstract_addr.unix_addr_len);
// C: 		if (variant->domain == SCOPE_SANDBOX) {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EPERM, errno);
// C: 		} else {
// C: 			EXPECT_EQ(0, err);
// C: 			ASSERT_EQ(1, write(dgram_abstract_socket, "g", 1));
// C: 		}
// C: 		EXPECT_EQ(0, close(dgram_abstract_socket));
// C: 
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 	EXPECT_EQ(0, close(unnamed_sockets[0]));
// C: 
// C: 	/* Sets up pathname servers. * /
// C: 	stream_pathname_socket = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 	ASSERT_LE(0, stream_pathname_socket);
// C: 	ASSERT_EQ(0, bind(stream_pathname_socket, &stream_pathname_addr,
// C: 			  sizeof(stream_pathname_addr)));
// C: 	ASSERT_EQ(0, listen(stream_pathname_socket, backlog));
// C: 
// C: 	dgram_pathname_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_pathname_socket);
// C: 	ASSERT_EQ(0, bind(dgram_pathname_socket, &dgram_pathname_addr,
// C: 			  sizeof(dgram_pathname_addr)));
// C: 
// C: 	/* Sets up abstract servers. * /
// C: 	stream_abstract_socket = socket(AF_UNIX, SOCK_STREAM, 0);
// C: 	ASSERT_LE(0, stream_abstract_socket);
// C: 	ASSERT_EQ(0,
// C: 		  bind(stream_abstract_socket, &stream_abstract_addr.unix_addr,
// C: 		       stream_abstract_addr.unix_addr_len));
// C: 
// C: 	dgram_abstract_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, dgram_abstract_socket);
// C: 	ASSERT_EQ(0, bind(dgram_abstract_socket, &dgram_abstract_addr.unix_addr,
// C: 			  dgram_abstract_addr.unix_addr_len));
// C: 	ASSERT_EQ(0, listen(stream_abstract_socket, backlog));
// C: 
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 	EXPECT_EQ(0, close(pipe_parent[1]));
// C: 
// C: 	/* Reads from unnamed socket. * /
// C: 	ASSERT_EQ(1, read(unnamed_sockets[1], &buf_parent, sizeof(buf_parent)));
// C: 	ASSERT_EQ('a', buf_parent);
// C: 	EXPECT_LE(0, close(unnamed_sockets[1]));
// C: 
// C: 	/* Reads from pathname sockets. * /
// C: 	data_socket = accept(stream_pathname_socket, NULL, NULL);
// C: 	ASSERT_LE(0, data_socket);
// C: 	ASSERT_EQ(1, read(data_socket, &buf_parent, sizeof(buf_parent)));
// C: 	ASSERT_EQ('b', buf_parent);
// C: 	EXPECT_EQ(0, close(data_socket));
// C: 	EXPECT_EQ(0, close(stream_pathname_socket));
// C: 
// C: 	ASSERT_EQ(1,
// C: 		  read(dgram_pathname_socket, &buf_parent, sizeof(buf_parent)));
// C: 	ASSERT_EQ('c', buf_parent);
// C: 	ASSERT_EQ(1,
// C: 		  read(dgram_pathname_socket, &buf_parent, sizeof(buf_parent)));
// C: 	ASSERT_EQ('d', buf_parent);
// C: 	EXPECT_EQ(0, close(dgram_pathname_socket));
// C: 
// C: 	if (variant->domain != SCOPE_SANDBOX) {
// C: 		/* Reads from abstract sockets if allowed to send. * /
// C: 		data_socket = accept(stream_abstract_socket, NULL, NULL);
// C: 		ASSERT_LE(0, data_socket);
// C: 		ASSERT_EQ(1,
// C: 			  read(data_socket, &buf_parent, sizeof(buf_parent)));
// C: 		ASSERT_EQ('e', buf_parent);
// C: 		EXPECT_EQ(0, close(data_socket));
// C: 
// C: 		ASSERT_EQ(1, read(dgram_abstract_socket, &buf_parent,
// C: 				  sizeof(buf_parent)));
// C: 		ASSERT_EQ('f', buf_parent);
// C: 		ASSERT_EQ(1, read(dgram_abstract_socket, &buf_parent,
// C: 				  sizeof(buf_parent)));
// C: 		ASSERT_EQ('g', buf_parent);
// C: 	}
// C: 
// C: 	/* Waits for all abstract socket tests. * /
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(stream_abstract_socket));
// C: 	EXPECT_EQ(0, close(dgram_abstract_socket));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: TEST(datagram_sockets)
// C: {
// C: 	struct service_fixture connected_addr, non_connected_addr;
// C: 	int server_conn_socket, server_unconn_socket;
// C: 	int pipe_parent[2], pipe_child[2];
// C: 	int status;
// C: 	char buf;
// C: 	pid_t child;
// C: 
// C: 	drop_caps(_metadata);
// C: 	memset(&connected_addr, 0, sizeof(connected_addr));
// C: 	set_unix_address(&connected_addr, 0);
// C: 	memset(&non_connected_addr, 0, sizeof(non_connected_addr));
// C: 	set_unix_address(&non_connected_addr, 1);
// C: 
// C: 	ASSERT_EQ(0, pipe2(pipe_parent, O_CLOEXEC));
// C: 	ASSERT_EQ(0, pipe2(pipe_child, O_CLOEXEC));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int client_conn_socket, client_unconn_socket;
// C: 
// C: 		EXPECT_EQ(0, close(pipe_parent[1]));
// C: 		EXPECT_EQ(0, close(pipe_child[0]));
// C: 
// C: 		client_conn_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		client_unconn_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 		ASSERT_LE(0, client_conn_socket);
// C: 		ASSERT_LE(0, client_unconn_socket);
// C: 
// C: 		/* Waits for parent to listen. * /
// C: 		ASSERT_EQ(1, read(pipe_parent[0], &buf, 1));
// C: 		ASSERT_EQ(0,
// C: 			  connect(client_conn_socket, &connected_addr.unix_addr,
// C: 				  connected_addr.unix_addr_len));
// C: 
// C: 		/*
// C: 		 * Both connected and non-connected sockets can send data when
// C: 		 * the domain is not scoped.
// C: 		 * /
// C: 		ASSERT_EQ(1, send(client_conn_socket, ".", 1, 0));
// C: 		ASSERT_EQ(1, sendto(client_unconn_socket, ".", 1, 0,
// C: 				    &non_connected_addr.unix_addr,
// C: 				    non_connected_addr.unix_addr_len));
// C: 		ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 
// C: 		/* Scopes the domain. * /
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		/*
// C: 		 * Connected socket sends data to the receiver, but the
// C: 		 * non-connected socket must fail to send data.
// C: 		 * /
// C: 		ASSERT_EQ(1, send(client_conn_socket, ".", 1, 0));
// C: 		ASSERT_EQ(-1, sendto(client_unconn_socket, ".", 1, 0,
// C: 				     &non_connected_addr.unix_addr,
// C: 				     non_connected_addr.unix_addr_len));
// C: 		ASSERT_EQ(EPERM, errno);
// C: 		ASSERT_EQ(1, write(pipe_child[1], ".", 1));
// C: 
// C: 		EXPECT_EQ(0, close(client_conn_socket));
// C: 		EXPECT_EQ(0, close(client_unconn_socket));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(0, close(pipe_parent[0]));
// C: 	EXPECT_EQ(0, close(pipe_child[1]));
// C: 
// C: 	server_conn_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	server_unconn_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, server_conn_socket);
// C: 	ASSERT_LE(0, server_unconn_socket);
// C: 
// C: 	ASSERT_EQ(0, bind(server_conn_socket, &connected_addr.unix_addr,
// C: 			  connected_addr.unix_addr_len));
// C: 	ASSERT_EQ(0, bind(server_unconn_socket, &non_connected_addr.unix_addr,
// C: 			  non_connected_addr.unix_addr_len));
// C: 	ASSERT_EQ(1, write(pipe_parent[1], ".", 1));
// C: 
// C: 	/* Waits for child to test. * /
// C: 	ASSERT_EQ(1, read(pipe_child[0], &buf, 1));
// C: 	ASSERT_EQ(1, recv(server_conn_socket, &buf, 1, 0));
// C: 	ASSERT_EQ(1, recv(server_unconn_socket, &buf, 1, 0));
// C: 
// C: 	/*
// C: 	 * Connected datagram socket will receive data, but
// C: 	 * non-connected datagram socket does not receive data.
// C: 	 * /
// C: 	ASSERT_EQ(1, read(pipe_child[0], &buf, 1));
// C: 	ASSERT_EQ(1, recv(server_conn_socket, &buf, 1, 0));
// C: 
// C: 	/* Waits for all tests to finish. * /
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(server_conn_socket));
// C: 	EXPECT_EQ(0, close(server_unconn_socket));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: TEST(self_connect)
// C: {
// C: 	struct service_fixture connected_addr, non_connected_addr;
// C: 	int connected_socket, non_connected_socket, status;
// C: 	pid_t child;
// C: 
// C: 	drop_caps(_metadata);
// C: 	memset(&connected_addr, 0, sizeof(connected_addr));
// C: 	set_unix_address(&connected_addr, 0);
// C: 	memset(&non_connected_addr, 0, sizeof(non_connected_addr));
// C: 	set_unix_address(&non_connected_addr, 1);
// C: 
// C: 	connected_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	non_connected_socket = socket(AF_UNIX, SOCK_DGRAM, 0);
// C: 	ASSERT_LE(0, connected_socket);
// C: 	ASSERT_LE(0, non_connected_socket);
// C: 
// C: 	ASSERT_EQ(0, bind(connected_socket, &connected_addr.unix_addr,
// C: 			  connected_addr.unix_addr_len));
// C: 	ASSERT_EQ(0, bind(non_connected_socket, &non_connected_addr.unix_addr,
// C: 			  non_connected_addr.unix_addr_len));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		/* Child's domain is scoped. * /
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 		/*
// C: 		 * The child inherits the sockets, and cannot connect or
// C: 		 * send data to them.
// C: 		 * /
// C: 		ASSERT_EQ(-1,
// C: 			  connect(connected_socket, &connected_addr.unix_addr,
// C: 				  connected_addr.unix_addr_len));
// C: 		ASSERT_EQ(EPERM, errno);
// C: 
// C: 		ASSERT_EQ(-1, sendto(connected_socket, ".", 1, 0,
// C: 				     &connected_addr.unix_addr,
// C: 				     connected_addr.unix_addr_len));
// C: 		ASSERT_EQ(EPERM, errno);
// C: 
// C: 		ASSERT_EQ(-1, sendto(non_connected_socket, ".", 1, 0,
// C: 				     &non_connected_addr.unix_addr,
// C: 				     non_connected_addr.unix_addr_len));
// C: 		ASSERT_EQ(EPERM, errno);
// C: 
// C: 		EXPECT_EQ(0, close(connected_socket));
// C: 		EXPECT_EQ(0, close(non_connected_socket));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 
// C: 	/* Waits for all tests to finish. * /
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(0, close(connected_socket));
// C: 	EXPECT_EQ(0, close(non_connected_socket));
// C: 
// C: 	if (WIFSIGNALED(status) || !WIFEXITED(status) ||
// C: 	    WEXITSTATUS(status) != EXIT_SUCCESS)
// C: 		_metadata->exit_code = KSFT_FAIL;
// C: }
// C: 
// C: /* Trace tests * /
// C: 
// C: /* clang-format off * /
// C: FIXTURE(trace_unix) {
// C: 	/* clang-format on * /
// C: 	int tracefs_ok;
// C: };
// C: 
// C: FIXTURE_SETUP(trace_unix)
// C: {
// C: 	int ret;
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	ASSERT_EQ(0, unshare(CLONE_NEWNS));
// C: 	ASSERT_EQ(0, mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL));
// C: 
// C: 	ret = tracefs_fixture_setup();
// C: 	if (ret) {
// C: 		clear_cap(_metadata, CAP_SYS_ADMIN);
// C: 		self->tracefs_ok = 0;
// C: 		SKIP(return, "tracefs not available");
// C: 	}
// C: 	self->tracefs_ok = 1;
// C: 
// C: 	ASSERT_EQ(0, tracefs_enable_event(
// C: 			     TRACEFS_DENY_SCOPE_ABSTRACT_UNIX_SOCKET_ENABLE,
// C: 			     true));
// C: 	ASSERT_EQ(0, tracefs_clear());
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(trace_unix)
// C: {
// C: 	if (!self->tracefs_ok)
// C: 		return;
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	tracefs_enable_event(TRACEFS_DENY_SCOPE_ABSTRACT_UNIX_SOCKET_ENABLE,
// C: 			     false);
// C: 	tracefs_fixture_teardown();
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT(trace_unix) {
// C: 	/* clang-format on * /
// C: 	int sock_type; /* SOCK_STREAM (connect) or SOCK_DGRAM (sendto). * /
// C: 	bool sandbox;
// C: 	bool sandbox_target; /* Peer owned by a domain: peer_domain != 0. * /
// C: 	int expect_denied;
// C: };
// C: 
// C: /* clang-format off * /
// C: 
// C: /* Stream: sandboxed client connect() to an unsandboxed peer (peer_domain=0). * /
// C: FIXTURE_VARIANT_ADD(trace_unix, stream_denied) {
// C: 	.sock_type = SOCK_STREAM, .sandbox = true,
// C: 	.sandbox_target = false, .expect_denied = 1,
// C: };
// C: 
// C: /* Stream: peer socket owned by a domain, so peer_domain != 0. * /
// C: FIXTURE_VARIANT_ADD(trace_unix, stream_denied_scoped_peer) {
// C: 	.sock_type = SOCK_STREAM, .sandbox = true,
// C: 	.sandbox_target = true, .expect_denied = 1,
// C: };
// C: 
// C: /* Stream: unsandboxed client, connect() succeeds, no event. * /
// C: FIXTURE_VARIANT_ADD(trace_unix, stream_allowed) {
// C: 	.sock_type = SOCK_STREAM, .sandbox = false,
// C: 	.sandbox_target = false, .expect_denied = 0,
// C: };
// C: 
// C: /* Datagram: sandboxed client sendto() an unsandboxed peer (peer_domain=0). * /
// C: FIXTURE_VARIANT_ADD(trace_unix, dgram_denied) {
// C: 	.sock_type = SOCK_DGRAM, .sandbox = true,
// C: 	.sandbox_target = false, .expect_denied = 1,
// C: };
// C: 
// C: /* Datagram: peer socket owned by a domain, so peer_domain != 0. * /
// C: FIXTURE_VARIANT_ADD(trace_unix, dgram_denied_scoped_peer) {
// C: 	.sock_type = SOCK_DGRAM, .sandbox = true,
// C: 	.sandbox_target = true, .expect_denied = 1,
// C: };
// C: 
// C: /* Datagram: unsandboxed client, sendto() succeeds, no event. * /
// C: FIXTURE_VARIANT_ADD(trace_unix, dgram_allowed) {
// C: 	.sock_type = SOCK_DGRAM, .sandbox = false,
// C: 	.sandbox_target = false, .expect_denied = 0,
// C: };
// C: 
// C: /* clang-format on * /
// C: 
// C: /*
// C:  * A sandboxed thread reaching an abstract unix socket peer through connect(2)
// C:  * (stream) or sendto(2) (datagram) is denied and emits
// C:  * landlock_deny_scope_abstract_unix_socket.  The abstract name is crafted with
// C:  * a space and an embedded NUL followed by an "END" marker to check the
// C:  * tracepoint escaping and its length handling (a raw space would break the
// C:  * sun_path field regex; strlen() would truncate at the NUL and drop "END").
// C:  * peer_pid is only meaningful for a stream peer (a datagram peer has no
// C:  * SO_PEERCRED), so it is asserted only there.
// C:  * /
// C: TEST_F(trace_unix, deny_scope_unix)
// C: {
// C: 	struct sockaddr_un addr = {
// C: 		.sun_family = AF_UNIX,
// C: 	};
// C: 	char *buf, field[128], expected_pid[16];
// C: 	int server_fd, count, status, name_len, addr_len;
// C: 	pid_t child;
// C: 
// C: 	if (!self->tracefs_ok)
// C: 		SKIP(return, "tracefs not available");
// C: 
// C: 	/*
// C: 	 * For the non-zero peer_domain case, sandbox the parent before it
// C: 	 * creates the server socket, so the socket carries the parent's domain
// C: 	 * and peer_domain= is non-zero.
// C: 	 * /
// C: 	if (variant->sandbox_target)
// C: 		create_scoped_domain(_metadata,
// C: 				     LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET);
// C: 
// C: 	server_fd = socket(AF_UNIX, variant->sock_type | SOCK_CLOEXEC, 0);
// C: 	ASSERT_LE(0, server_fd);
// C: 
// C: 	addr.sun_path[0] = '\0';
// C: 	name_len = snprintf(addr.sun_path + 1, sizeof(addr.sun_path) - 1,
// C: 			    "landlock_trace_test_%d ", getpid());
// C: 	addr.sun_path[1 + name_len] = '\0';
// C: 	memcpy(addr.sun_path + 1 + name_len + 1, "END", 3);
// C: 	addr_len =
// C: 		offsetof(struct sockaddr_un, sun_path) + 1 + name_len + 1 + 3;
// C: 
// C: 	ASSERT_EQ(0, bind(server_fd, (struct sockaddr *)&addr, addr_len));
// C: 	if (variant->sock_type == SOCK_STREAM)
// C: 		ASSERT_EQ(0, listen(server_fd, 1));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 
// C: 	if (child == 0) {
// C: 		int client_fd, ret;
// C: 
// C: 		if (variant->sandbox) {
// C: 			struct landlock_ruleset_attr ruleset_attr = {
// C: 				.scoped = LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET,
// C: 			};
// C: 			int ruleset_fd;
// C: 
// C: 			ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			if (ruleset_fd < 0)
// C: 				_exit(1);
// C: 
// C: 			prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
// C: 			if (landlock_restrict_self(ruleset_fd, 0)) {
// C: 				close(ruleset_fd);
// C: 				_exit(1);
// C: 			}
// C: 			close(ruleset_fd);
// C: 		}
// C: 
// C: 		client_fd =
// C: 			socket(AF_UNIX, variant->sock_type | SOCK_CLOEXEC, 0);
// C: 		if (client_fd < 0)
// C: 			_exit(1);
// C: 
// C: 		if (variant->sock_type == SOCK_STREAM)
// C: 			ret = connect(client_fd, (struct sockaddr *)&addr,
// C: 				      addr_len);
// C: 		else
// C: 			ret = sendto(client_fd, ".", 1, 0,
// C: 				     (struct sockaddr *)&addr, addr_len);
// C: 
// C: 		if (variant->sandbox) {
// C: 			/* Reaching the peer should be denied. * /
// C: 			if (ret != -1 || errno != EPERM) {
// C: 				close(client_fd);
// C: 				_exit(2);
// C: 			}
// C: 		} else {
// C: 			/* No sandbox: stream connect() == 0, sendto() == 1. * /
// C: 			int ok = variant->sock_type == SOCK_STREAM ? 0 : 1;
// C: 
// C: 			if (ret != ok) {
// C: 				close(client_fd);
// C: 				_exit(2);
// C: 			}
// C: 		}
// C: 		close(client_fd);
// C: 		_exit(0);
// C: 	}
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	ASSERT_TRUE(WIFEXITED(status));
// C: 	EXPECT_EQ(0, WEXITSTATUS(status));
// C: 	close(server_fd);
// C: 
// C: 	buf = tracefs_read_buf();
// C: 	ASSERT_NE(NULL, buf);
// C: 
// C: 	count = tracefs_count_matches(
// C: 		buf, REGEX_DENY_SCOPE_ABSTRACT_UNIX_SOCKET(TRACE_TASK));
// C: 	if (!variant->expect_denied) {
// C: 		EXPECT_EQ(0, count)
// C: 		{
// C: 			TH_LOG("Expected 0 deny_scope events, got %d\n%s",
// C: 			       count, buf);
// C: 		}
// C: 		free(buf);
// C: 		return;
// C: 	}
// C: 
// C: 	EXPECT_EQ(variant->expect_denied, count)
// C: 	{
// C: 		TH_LOG("Expected deny_scope_abstract_unix_socket event, "
// C: 		       "got %d\n%s",
// C: 		       count, buf);
// C: 	}
// C: 
// C: 	/*
// C: 	 * sun_path is escaped: a raw space would break this field's [^ ]*$
// C: 	 * regex, so a successful extract proves the space was escaped, and its
// C: 	 * full length is honored: the "END" marker after the embedded NUL must
// C: 	 * survive (strlen() would truncate it at the NUL).
// C: 	 * /
// C: 	ASSERT_EQ(0, tracefs_extract_field(
// C: 			     buf,
// C: 			     REGEX_DENY_SCOPE_ABSTRACT_UNIX_SOCKET(TRACE_TASK),
// C: 			     "sun_path", field, sizeof(field)));
// C: 	EXPECT_NE(NULL, strstr(field, "END"))
// C: 	{
// C: 		TH_LOG("sun_path truncated or unescaped: %s", field);
// C: 	}
// C: 
// C: 	/* peer_pid is the parent's PID for a stream peer (0 for datagram). * /
// C: 	if (variant->sock_type == SOCK_STREAM) {
// C: 		snprintf(expected_pid, sizeof(expected_pid), "%d", getpid());
// C: 		ASSERT_EQ(0, tracefs_extract_field(
// C: 				     buf,
// C: 				     REGEX_DENY_SCOPE_ABSTRACT_UNIX_SOCKET(
// C: 					     TRACE_TASK),
// C: 				     "peer_pid", field, sizeof(field)));
// C: 		EXPECT_STREQ(expected_pid, field);
// C: 	}
// C: 
// C: 	/* peer_domain: 0 when the peer is unsandboxed, non-zero otherwise. * /
// C: 	ASSERT_EQ(0, tracefs_extract_field(
// C: 			     buf,
// C: 			     REGEX_DENY_SCOPE_ABSTRACT_UNIX_SOCKET(TRACE_TASK),
// C: 			     "peer_domain", field, sizeof(field)));
// C: 	EXPECT_EQ(variant->sandbox_target, strcmp("0", field) != 0)
// C: 	{
// C: 		TH_LOG("Unexpected peer_domain=%s", field);
// C: 	}
// C: 
// C: 	free(buf);
// C: }
// C: 
// C: TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
