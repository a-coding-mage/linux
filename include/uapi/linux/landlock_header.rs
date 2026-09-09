/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Landlock - User space API
 *
 * Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2021-2025 Microsoft Corporation
 */

// Dependency supplied by the Linux UAPI type definitions: __u64 and __s32.

/**
 * struct landlock_ruleset_attr - Ruleset definition.
 *
 * Argument of sys_landlock_create_ruleset().
 *
 * This structure defines handled access rights and actions which should be
 * denied by default when the ruleset is enacted.  Rights not listed here are
 * not denied by this ruleset.  LANDLOCK_ACCESS_FS_REFER is denied by default
 * for historical reasons and must be explicitly handled to add such rules.
 *
 * This structure can grow in future Landlock versions.
 */
#[repr(C)]
pub struct landlock_ruleset_attr {
    /// Bitmask of handled filesystem actions.
    pub handled_access_fs: u64,
    /// Bitmask of handled network actions.
    pub handled_access_net: u64,
    /// Bitmask of scopes restricting access outside resources.
    pub scoped: u64,
    /// Filesystem actions not logged when the per-object quiet flag is set.
    pub quiet_access_fs: u64,
    /// Network actions not logged when the per-object quiet flag is set.
    pub quiet_access_net: u64,
    /// Scoped actions which should not be logged.
    pub quiet_scoped: u64,
}

pub const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1u32 << 0;
pub const LANDLOCK_CREATE_RULESET_ERRATA: u32 = 1u32 << 1;

/// LANDLOCK_ADD_RULE_QUIET controls audit logging for objects covered by a rule.
pub const LANDLOCK_ADD_RULE_QUIET: u32 = 1u32 << 0;

pub const LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF: u32 = 1u32 << 0;
pub const LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON: u32 = 1u32 << 1;
pub const LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF: u32 = 1u32 << 2;
pub const LANDLOCK_RESTRICT_SELF_TSYNC: u32 = 1u32 << 3;
pub const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS: u32 = 1u32 << 4;

/// Landlock rule type, used by sys_landlock_add_rule().
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum landlock_rule_type {
    LANDLOCK_RULE_PATH_BENEATH = 1,
    LANDLOCK_RULE_NET_PORT,
}

/// Path hierarchy definition, used by sys_landlock_add_rule().
#[repr(C, packed)]
pub struct landlock_path_beneath_attr {
    /// Bitmask of allowed actions for this file hierarchy.
    pub allowed_access: u64,
    /// File descriptor identifying the parent directory or file.
    pub parent_fd: i32,
}

/// Network port definition, used by sys_landlock_add_rule().
#[repr(C)]
pub struct landlock_net_port_attr {
    /// Bitmask of allowed network actions for a port.
    pub allowed_access: u64,
    /// Network port in host endianness. Port 0 represents an ephemeral port.
    pub port: u64,
}

pub const LANDLOCK_ACCESS_FS_EXECUTE: u64 = 1u64 << 0;
pub const LANDLOCK_ACCESS_FS_WRITE_FILE: u64 = 1u64 << 1;
pub const LANDLOCK_ACCESS_FS_READ_FILE: u64 = 1u64 << 2;
pub const LANDLOCK_ACCESS_FS_READ_DIR: u64 = 1u64 << 3;
pub const LANDLOCK_ACCESS_FS_REMOVE_DIR: u64 = 1u64 << 4;
pub const LANDLOCK_ACCESS_FS_REMOVE_FILE: u64 = 1u64 << 5;
pub const LANDLOCK_ACCESS_FS_MAKE_CHAR: u64 = 1u64 << 6;
pub const LANDLOCK_ACCESS_FS_MAKE_DIR: u64 = 1u64 << 7;
pub const LANDLOCK_ACCESS_FS_MAKE_REG: u64 = 1u64 << 8;
pub const LANDLOCK_ACCESS_FS_MAKE_SOCK: u64 = 1u64 << 9;
pub const LANDLOCK_ACCESS_FS_MAKE_FIFO: u64 = 1u64 << 10;
pub const LANDLOCK_ACCESS_FS_MAKE_BLOCK: u64 = 1u64 << 11;
pub const LANDLOCK_ACCESS_FS_MAKE_SYM: u64 = 1u64 << 12;
pub const LANDLOCK_ACCESS_FS_REFER: u64 = 1u64 << 13;
pub const LANDLOCK_ACCESS_FS_TRUNCATE: u64 = 1u64 << 14;
pub const LANDLOCK_ACCESS_FS_IOCTL_DEV: u64 = 1u64 << 15;
pub const LANDLOCK_ACCESS_FS_RESOLVE_UNIX: u64 = 1u64 << 16;

pub const LANDLOCK_ACCESS_NET_BIND_TCP: u64 = 1u64 << 0;
pub const LANDLOCK_ACCESS_NET_CONNECT_TCP: u64 = 1u64 << 1;
pub const LANDLOCK_ACCESS_NET_BIND_UDP: u64 = 1u64 << 2;
pub const LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP: u64 = 1u64 << 3;

pub const LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET: u64 = 1u64 << 0;
pub const LANDLOCK_SCOPE_SIGNAL: u64 = 1u64 << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
