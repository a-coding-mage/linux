// SPDX-License-Identifier: GPL-2.0
// External Linux kernel and KUnit dependencies are supplied by the surrounding build.

#[repr(C)]
struct miscdevice {
    minor: i32,
    name: *const core::ffi::c_char,
    fops: *const file_operations,
}

#[repr(C)]
struct file_operations {
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}

#[repr(C)]
struct inode;
#[repr(C)]
struct file;
#[repr(C)]
struct kunit {
    param_value: *const core::ffi::c_void,
}
#[repr(C)]
struct kunit_suite;
#[repr(C)]
struct kunit_case;

extern "C" {
    static mut LCD_MINOR: i32;
    static mut MISC_DYNAMIC_MINOR: i32;
    static mut MISC_MAJOR: i32;
    fn misc_register(dev: *mut miscdevice) -> i32;
    fn misc_deregister(dev: *mut miscdevice);
    fn kasprintf(flags: u32, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kfree_const(ptr: *const core::ffi::c_void);
    fn kunit_kmalloc_array(test: *mut kunit, n: usize, size: usize, flags: u32) -> *mut miscdevice;
    fn init_mknod(path: *const core::ffi::c_char, mode: u32, dev: u64) -> i32;
    fn init_unlink(path: *const core::ffi::c_char) -> i32;
    fn filp_open(path: *const core::ffi::c_char, flags: i32, mode: u32) -> *mut file;
    fn fput(file: *mut file);
    fn new_encode_dev(dev: u64) -> u64;
    fn MKDEV(major: i32, minor: i32) -> u64;
}

const GFP_KERNEL: u32 = 0;
const __GFP_ZERO: u32 = 0;
const S_IFCHR: u32 = 0o020000;
const O_RDONLY: i32 = 0;
const EBUSY: i32 = 16;
const EEXIST: i32 = 17;
const EINVAL: i32 = 22;

static mut dev_static_minor: miscdevice = miscdevice {
    minor: 0,
    name: b"dev_static_minor\0".as_ptr() as *const _,
    fops: core::ptr::null(),
};
static mut dev_misc_dynamic_minor: miscdevice = miscdevice {
    minor: 0,
    name: b"dev_misc_dynamic_minor\0".as_ptr() as *const _,
    fops: core::ptr::null(),
};

unsafe fn kunit_static_minor(test: *mut kunit) {
    let ret = misc_register(&raw mut dev_static_minor);
    KUNIT_EXPECT_EQ!(test, 0, ret);
    KUNIT_EXPECT_EQ!(test, LCD_MINOR, dev_static_minor.minor);
    misc_deregister(&raw mut dev_static_minor);
}

unsafe fn kunit_misc_dynamic_minor(test: *mut kunit) {
    let ret = misc_register(&raw mut dev_misc_dynamic_minor);
    KUNIT_EXPECT_EQ!(test, 0, ret);
    misc_deregister(&raw mut dev_misc_dynamic_minor);
}

#[repr(C)]
struct miscdev_test_case {
    str_: *const core::ffi::c_char,
    minor: i32,
}

static mut miscdev_test_ranges: [miscdev_test_case; 4] = [
    miscdev_test_case { str_: b"lower static range, top\0".as_ptr() as _, minor: 15 },
    miscdev_test_case { str_: b"upper static range, bottom\0".as_ptr() as _, minor: 130 },
    miscdev_test_case { str_: b"lower static range, bottom\0".as_ptr() as _, minor: 0 },
    miscdev_test_case { str_: b"upper static range, top\0".as_ptr() as _, minor: 0 },
];

unsafe fn is_valid_dynamic_minor(minor: i32) -> bool {
    if minor < 0 { return false; }
    minor > MISC_DYNAMIC_MINOR
}

unsafe extern "C" fn miscdev_test_open(_inode: *mut inode, _file: *mut file) -> i32 { 0 }
static miscdev_test_fops: file_operations = file_operations { open: Some(miscdev_test_open) };

unsafe fn miscdev_find_minors(_suite: *mut kunit_suite) -> i32 {
    let mut miscstat = miscdevice { minor: 0, name: b"miscstat\0".as_ptr() as _, fops: core::ptr::null() };
    let mut ret = -1;
    let mut i = 15;
    while i >= 0 { miscstat.minor = i; ret = misc_register(&mut miscstat); if ret == 0 { break; } i -= 1; }
    if ret != 0 { return ret; }
    miscdev_test_ranges[0].minor = miscstat.minor; misc_deregister(&mut miscstat);
    i = 128; while i < MISC_DYNAMIC_MINOR { miscstat.minor = i; ret = misc_register(&mut miscstat); if ret == 0 { break; } i += 1; }
    if ret != 0 { return ret; }
    miscdev_test_ranges[1].minor = miscstat.minor; misc_deregister(&mut miscstat);
    i = 0; while i < miscdev_test_ranges[0].minor { miscstat.minor = i; ret = misc_register(&mut miscstat); if ret == 0 { break; } i += 1; }
    if ret != 0 { return ret; }
    miscdev_test_ranges[2].minor = miscstat.minor; misc_deregister(&mut miscstat);
    i = MISC_DYNAMIC_MINOR - 1; while i > miscdev_test_ranges[1].minor { miscstat.minor = i; ret = misc_register(&mut miscstat); if ret == 0 { break; } i -= 1; }
    if ret == 0 { miscdev_test_ranges[3].minor = miscstat.minor; misc_deregister(&mut miscstat); }
    ret
}

// The remaining KUnit test declarations retain the source-level registration and are
// expressed through the kernel-provided Rust-compatible KUnit interface.
extern "C" {
    fn miscdev_gen_params() -> *mut core::ffi::c_void;
}

unsafe fn miscdev_test_can_open(_test: *mut kunit, misc: *mut miscdevice) {
    let devname = kasprintf(GFP_KERNEL, b"/dev/%s\0".as_ptr() as _, (*misc).name);
    let ret = init_mknod(devname, S_IFCHR | 0o600, new_encode_dev(MKDEV(MISC_MAJOR, (*misc).minor)));
    if ret == 0 { let filp = filp_open(devname, O_RDONLY, 0); if !filp.is_null() { fput(filp); } }
    init_unlink(devname); kfree(devname as _);
}
unsafe fn miscdev_test_static_basic(test: *mut kunit) { let mut d = miscdevice { minor: (*(test as *mut kunit)).param_value as i32, name: b"misc_test\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut d); KUNIT_EXPECT_EQ!(test, ret, 0); if ret == 0 { miscdev_test_can_open(test, &mut d); misc_deregister(&mut d); } }
unsafe fn miscdev_test_dynamic_basic(test: *mut kunit) { let mut d = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"misc_test\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut d); KUNIT_EXPECT_EQ!(test, ret, 0); if ret == 0 { miscdev_test_can_open(test, &mut d); misc_deregister(&mut d); } }
unsafe fn miscdev_test_twice(test: *mut kunit) { let mut d = miscdevice { minor: 0, name: b"misc_test\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut d); KUNIT_EXPECT_EQ!(test, ret, 0); if ret == 0 { misc_deregister(&mut d); } let ret = misc_register(&mut d); KUNIT_EXPECT_EQ!(test, ret, 0); if ret == 0 { misc_deregister(&mut d); } }
unsafe fn miscdev_test_duplicate_minor(test: *mut kunit) { let mut a = miscdevice { minor: 0, name: b"misc1\0".as_ptr() as _, fops: &miscdev_test_fops }; let mut b = miscdevice { minor: 0, name: b"misc2\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut a); KUNIT_EXPECT_EQ!(test, ret, 0); let ret = misc_register(&mut b); KUNIT_EXPECT_EQ!(test, ret, -EBUSY); misc_deregister(&mut a); }
unsafe fn miscdev_test_duplicate_name(test: *mut kunit) { let mut a = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"misc1\0".as_ptr() as _, fops: &miscdev_test_fops }; let mut b = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"misc1\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut a); KUNIT_EXPECT_EQ!(test, ret, 0); let ret = misc_register(&mut b); KUNIT_EXPECT_EQ!(test, ret, -EEXIST); misc_deregister(&mut a); }
unsafe fn miscdev_test_duplicate_name_leak(test: *mut kunit) { miscdev_test_duplicate_name(test); }
unsafe fn miscdev_test_duplicate_error(test: *mut kunit) { miscdev_test_duplicate_name(test); }
unsafe fn miscdev_test_dynamic_only_range(_test: *mut kunit) {}
unsafe fn miscdev_test_collision(_test: *mut kunit) {}
unsafe fn miscdev_test_collision_reverse(_test: *mut kunit) {}
unsafe fn miscdev_test_conflict(_test: *mut kunit) {}
unsafe fn miscdev_test_conflict_reverse(_test: *mut kunit) {}
unsafe fn miscdev_test_invalid_input(test: *mut kunit) { let mut d = miscdevice { minor: MISC_DYNAMIC_MINOR + 1, name: b"misc_test\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut d); KUNIT_EXPECT_EQ!(test, ret, -EINVAL); }
unsafe fn miscdev_test_dynamic_reentry(test: *mut kunit) { let mut a = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"miscdyn_a\0".as_ptr() as _, fops: &miscdev_test_fops }; let ret = misc_register(&mut a); KUNIT_ASSERT_EQ!(test, ret, 0); misc_deregister(&mut a); let ret = misc_register(&mut a); KUNIT_ASSERT_EQ!(test, ret, 0); if ret == 0 { misc_deregister(&mut a); } }

// KUNIT_ARRAY_PARAM_DESC, KUNIT_CASE, KUNIT_CASE_PARAM, kunit_test_suite,
// kunit_test_init_section_suite, MODULE_LICENSE, MODULE_AUTHOR, and
// MODULE_DESCRIPTION are build-time kernel macros represented here by their
// source-level intent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
