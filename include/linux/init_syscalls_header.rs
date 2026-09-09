/* SPDX-License-Identifier: GPL-2.0 */

// The C `__init` annotation has no direct file-local Rust equivalent.
extern "C" {
    pub fn init_mount(
        dev_name: *const core::ffi::c_char,
        dir_name: *const core::ffi::c_char,
        type_page: *const core::ffi::c_char,
        flags: core::ffi::c_ulong,
        data_page: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn init_umount(name: *const core::ffi::c_char, flags: core::ffi::c_int) -> core::ffi::c_int;
    pub fn init_chdir(filename: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn init_chroot(filename: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn init_chown(
        filename: *const core::ffi::c_char,
        user: uid_t,
        group: gid_t,
        flags: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn init_chmod(filename: *const core::ffi::c_char, mode: umode_t) -> core::ffi::c_int;
    pub fn init_eaccess(filename: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn init_stat(
        filename: *const core::ffi::c_char,
        stat: *mut kstat,
        flags: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn init_mknod(
        filename: *const core::ffi::c_char,
        mode: umode_t,
        dev: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn init_link(
        oldname: *const core::ffi::c_char,
        newname: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn init_symlink(
        oldname: *const core::ffi::c_char,
        newname: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn init_unlink(pathname: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn init_mkdir(pathname: *const core::ffi::c_char, mode: umode_t) -> core::ffi::c_int;
    pub fn init_rmdir(pathname: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn init_utimes(filename: *mut core::ffi::c_char, ts: *mut timespec64) -> core::ffi::c_int;
    pub fn init_dup(file: *mut file) -> core::ffi::c_int;
    pub fn init_pivot_root(
        new_root: *const core::ffi::c_char,
        put_old: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
