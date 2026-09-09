/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Public types and definitions
 *
 * Copyright © 2016-2026 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2026 Cloudflare, Inc.
 */

// C dependencies: <linux/types.h> and <uapi/linux/landlock.h>.

/*
 * Access-right and scope names, shared between the audit records (get_blocker()
 * in security/landlock/audit.c) and the trace events
 * (include/trace/events/landlock.h).  A consumer defines
 * _LANDLOCK_NAME_ENTRY(mask, name) before expanding a list and undefines it
 * afterwards: audit maps each entry to a "[bit] = name" slot for O(1) lookup,
 * the trace events map it to a __print_flags() { mask, name } pair.  The bit
 * value lives only in the LANDLOCK_* UAPI constant each entry references.
 * Names are unprefixed; audit prepends the "fs./"net."/"scope." category.
 */

/* C X-macro equivalent: invoke the supplied entry macro for each item. */
#[macro_export]
macro_rules! _LANDLOCK_ACCESS_FS_NAMES {
    ($entry:ident) => {
        $entry!(LANDLOCK_ACCESS_FS_EXECUTE, "execute");
        $entry!(LANDLOCK_ACCESS_FS_WRITE_FILE, "write_file");
        $entry!(LANDLOCK_ACCESS_FS_READ_FILE, "read_file");
        $entry!(LANDLOCK_ACCESS_FS_READ_DIR, "read_dir");
        $entry!(LANDLOCK_ACCESS_FS_REMOVE_DIR, "remove_dir");
        $entry!(LANDLOCK_ACCESS_FS_REMOVE_FILE, "remove_file");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_CHAR, "make_char");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_DIR, "make_dir");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_REG, "make_reg");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_SOCK, "make_sock");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_FIFO, "make_fifo");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_BLOCK, "make_block");
        $entry!(LANDLOCK_ACCESS_FS_MAKE_SYM, "make_sym");
        $entry!(LANDLOCK_ACCESS_FS_REFER, "refer");
        $entry!(LANDLOCK_ACCESS_FS_TRUNCATE, "truncate");
        $entry!(LANDLOCK_ACCESS_FS_IOCTL_DEV, "ioctl_dev");
        $entry!(LANDLOCK_ACCESS_FS_RESOLVE_UNIX, "resolve_unix");
    };
}

#[macro_export]
macro_rules! _LANDLOCK_ACCESS_NET_NAMES {
    ($entry:ident) => {
        $entry!(LANDLOCK_ACCESS_NET_BIND_TCP, "bind_tcp");
        $entry!(LANDLOCK_ACCESS_NET_CONNECT_TCP, "connect_tcp");
        $entry!(LANDLOCK_ACCESS_NET_BIND_UDP, "bind_udp");
        $entry!(LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP, "connect_send_udp");
    };
}

#[macro_export]
macro_rules! _LANDLOCK_SCOPE_NAMES {
    ($entry:ident) => {
        $entry!(LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET, "abstract_unix_socket");
        $entry!(LANDLOCK_SCOPE_SIGNAL, "signal");
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
