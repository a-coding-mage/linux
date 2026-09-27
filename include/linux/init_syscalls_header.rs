/* SPDX-License-Identifier: GPL-2.0 */

// The C `__init` annotation has no direct file-local Rust equivalent.
extern "C" {
    pub fn init_mount(
        dev_name: *const kernel::ffi::c_char,
        dir_name: *const kernel::ffi::c_char,
        type_page: *const kernel::ffi::c_char,
        flags: kernel::ffi::c_ulong,
        data_page: *mut kernel::ffi::c_void,
    ) -> kernel::ffi::c_int;
    pub fn init_umount(name: *const kernel::ffi::c_char, flags: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn init_chdir(filename: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn init_chroot(filename: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn init_chown(
        filename: *const kernel::ffi::c_char,
        user: uid_t,
        group: gid_t,
        flags: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
    pub fn init_chmod(filename: *const kernel::ffi::c_char, mode: umode_t) -> kernel::ffi::c_int;
    pub fn init_eaccess(filename: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn init_stat(
        filename: *const kernel::ffi::c_char,
        stat: *mut kstat,
        flags: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
    pub fn init_mknod(
        filename: *const kernel::ffi::c_char,
        mode: umode_t,
        dev: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_int;
    pub fn init_link(
        oldname: *const kernel::ffi::c_char,
        newname: *const kernel::ffi::c_char,
    ) -> kernel::ffi::c_int;
    pub fn init_symlink(
        oldname: *const kernel::ffi::c_char,
        newname: *const kernel::ffi::c_char,
    ) -> kernel::ffi::c_int;
    pub fn init_unlink(pathname: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn init_mkdir(pathname: *const kernel::ffi::c_char, mode: umode_t) -> kernel::ffi::c_int;
    pub fn init_rmdir(pathname: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn init_utimes(filename: *mut kernel::ffi::c_char, ts: *mut timespec64) -> kernel::ffi::c_int;
    pub fn init_dup(file: *mut file) -> kernel::ffi::c_int;
    pub fn init_pivot_root(
        new_root: *const kernel::ffi::c_char,
        put_old: *const kernel::ffi::c_char,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
