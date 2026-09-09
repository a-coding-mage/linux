/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Each securesetting is implemented using two bits. One bit specifies
   whether the setting is on or off. The other bit specify whether the
   setting is locked or not. A setting which is locked cannot be
   changed from user-level. */
macro_rules! issecure_mask {
    ($x:expr) => {
        1i32 << ($x)
    };
}

pub const SECUREBITS_DEFAULT: i32 = 0x00000000;

/* When set UID 0 has no special privileges. When unset, we support
   inheritance of root-permissions and suid-root executable under
   compatibility mode. We raise the effective and inheritable bitmasks
   *of the executable file* if the effective uid of the new process is
   0. If the real uid is 0, we raise the effective (legacy) bit of the
   executable file. */
pub const SECURE_NOROOT: i32 = 0;
pub const SECURE_NOROOT_LOCKED: i32 = 1; /* make bit-0 immutable */

pub const SECBIT_NOROOT: i32 = issecure_mask!(SECURE_NOROOT);
pub const SECBIT_NOROOT_LOCKED: i32 = issecure_mask!(SECURE_NOROOT_LOCKED);

/* When set, setuid to/from uid 0 does not trigger capability-"fixup".
   When unset, to provide compatiblility with old programs relying on
   set*uid to gain/lose privilege, transitions to/from uid 0 cause
   capabilities to be gained/lost. */
pub const SECURE_NO_SETUID_FIXUP: i32 = 2;
pub const SECURE_NO_SETUID_FIXUP_LOCKED: i32 = 3; /* make bit-2 immutable */

pub const SECBIT_NO_SETUID_FIXUP: i32 = issecure_mask!(SECURE_NO_SETUID_FIXUP);
pub const SECBIT_NO_SETUID_FIXUP_LOCKED: i32 =
    issecure_mask!(SECURE_NO_SETUID_FIXUP_LOCKED);

/* When set, a process can retain its capabilities even after
   transitioning to a non-root user (the set-uid fixup suppressed by
   bit 2). Bit-4 is cleared when a process calls exec(); setting both
   bit 4 and 5 will create a barrier through exec that no exec()'d
   child can use this feature again. */
pub const SECURE_KEEP_CAPS: i32 = 4;
pub const SECURE_KEEP_CAPS_LOCKED: i32 = 5; /* make bit-4 immutable */

pub const SECBIT_KEEP_CAPS: i32 = issecure_mask!(SECURE_KEEP_CAPS);
pub const SECBIT_KEEP_CAPS_LOCKED: i32 = issecure_mask!(SECURE_KEEP_CAPS_LOCKED);

/* When set, a process cannot add new capabilities to its ambient set. */
pub const SECURE_NO_CAP_AMBIENT_RAISE: i32 = 6;
pub const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: i32 = 7; /* make bit-6 immutable */

pub const SECBIT_NO_CAP_AMBIENT_RAISE: i32 =
    issecure_mask!(SECURE_NO_CAP_AMBIENT_RAISE);
pub const SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED: i32 =
    issecure_mask!(SECURE_NO_CAP_AMBIENT_RAISE_LOCKED);

/* See Documentation/userspace-api/check_exec.rst */
pub const SECURE_EXEC_RESTRICT_FILE: i32 = 8;
pub const SECURE_EXEC_RESTRICT_FILE_LOCKED: i32 = 9; /* make bit-8 immutable */

pub const SECBIT_EXEC_RESTRICT_FILE: i32 = issecure_mask!(SECURE_EXEC_RESTRICT_FILE);
pub const SECBIT_EXEC_RESTRICT_FILE_LOCKED: i32 =
    issecure_mask!(SECURE_EXEC_RESTRICT_FILE_LOCKED);

/* See Documentation/userspace-api/check_exec.rst */
pub const SECURE_EXEC_DENY_INTERACTIVE: i32 = 10;
pub const SECURE_EXEC_DENY_INTERACTIVE_LOCKED: i32 = 11; /* make bit-10 immutable */

pub const SECBIT_EXEC_DENY_INTERACTIVE: i32 =
    issecure_mask!(SECURE_EXEC_DENY_INTERACTIVE);
pub const SECBIT_EXEC_DENY_INTERACTIVE_LOCKED: i32 =
    issecure_mask!(SECURE_EXEC_DENY_INTERACTIVE_LOCKED);

pub const SECURE_ALL_BITS: i32 = issecure_mask!(SECURE_NOROOT)
    | issecure_mask!(SECURE_NO_SETUID_FIXUP)
    | issecure_mask!(SECURE_KEEP_CAPS)
    | issecure_mask!(SECURE_NO_CAP_AMBIENT_RAISE)
    | issecure_mask!(SECURE_EXEC_RESTRICT_FILE)
    | issecure_mask!(SECURE_EXEC_DENY_INTERACTIVE);
pub const SECURE_ALL_LOCKS: i32 = SECURE_ALL_BITS << 1;

pub const SECURE_ALL_UNPRIVILEGED: i32 = issecure_mask!(SECURE_EXEC_RESTRICT_FILE)
    | issecure_mask!(SECURE_EXEC_DENY_INTERACTIVE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
