/* SPDX-License-Identifier: GPL-2.0 */

// Declarations translated from Linux UID16_H.
// The C header guard is omitted because Rust items are already scoped by the
// containing module.

unsafe extern "C" {
    pub fn __sys_setuid(uid: uid_t) -> c_long;
    pub fn __sys_setgid(gid: gid_t) -> c_long;
    pub fn __sys_setreuid(ruid: uid_t, euid: uid_t) -> c_long;
    pub fn __sys_setregid(rgid: gid_t, egid: gid_t) -> c_long;
    pub fn __sys_setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_long;
    pub fn __sys_setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_long;
    pub fn __sys_setfsuid(uid: uid_t) -> c_long;
    pub fn __sys_setfsgid(gid: gid_t) -> c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
