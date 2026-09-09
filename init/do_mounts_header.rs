/* SPDX-License-Identifier: GPL-2.0 */
// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust symbols.

extern "C" {
    pub fn mount_root_generic(name: *mut ::std::os::raw::c_char,
                               pretty_name: *mut ::std::os::raw::c_char,
                               flags: ::std::os::raw::c_int);
    pub fn mount_root(root_device_name: *mut ::std::os::raw::c_char);
    pub static mut root_mountflags: ::std::os::raw::c_int;
}

// static inline __init int create_dev(char *name, dev_t dev)
#[inline]
pub unsafe fn create_dev(name: *mut ::std::os::raw::c_char, dev: dev_t) -> ::std::os::raw::c_int {
    init_unlink(name);
    init_mknod(name, S_IFBLK | 0o600, new_encode_dev(dev))
}

// #ifdef CONFIG_BLK_DEV_RAM
extern "C" {
    pub fn rd_load_image() -> ::std::os::raw::c_int;
}
// #else: static inline int rd_load_image(void) { return 0; }
// #endif

// #ifdef CONFIG_BLK_DEV_INITRD
extern "C" {
    pub fn initrd_load();
}
// #else: static inline void initrd_load(void) { }
// #endif

/* Ensure that async file closing finished to prevent spurious errors. */
#[inline]
pub unsafe fn init_flush_fput() {
    flush_delayed_fput();
    task_work_run();
}

extern "C" {
    fn init_unlink(name: *mut ::std::os::raw::c_char);
    fn init_mknod(name: *mut ::std::os::raw::c_char,
                  mode: ::std::os::raw::c_uint,
                  dev: ::std::os::raw::c_ulong) -> ::std::os::raw::c_int;
    fn new_encode_dev(dev: dev_t) -> ::std::os::raw::c_ulong;
    fn flush_delayed_fput();
    fn task_work_run();
}

// Types and constants provided by the included kernel headers.
type dev_t = ::std::os::raw::c_ulong;
extern "C" {
    static S_IFBLK: ::std::os::raw::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
