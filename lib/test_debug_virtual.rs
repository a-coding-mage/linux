// SPDX-License-Identifier: GPL-2.0-only
// Linux kernel dependencies supplied by the surrounding build.
// CONFIG_MIPS conditionally supplies <asm/bootinfo.h>.

use core::ffi::c_void;

// C: struct foo { unsigned int bar; };
#[repr(C)]
pub struct foo {
    pub bar: u32,
}

// C: static struct foo *foo;
static mut foo: *mut foo = core::ptr::null_mut();

// External kernel symbols/macros used by this translation.
extern "C" {
    static VMALLOC_START: usize;
    fn virt_to_phys(address: *const c_void) -> usize;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(address: *mut c_void);
}

// C: static int __init test_debug_virtual_init(void)
#[no_mangle]
pub unsafe extern "C" fn test_debug_virtual_init() -> i32 {
    let mut pa: usize;
    let mut va: *mut c_void;

    va = VMALLOC_START as *mut c_void;
    pa = virt_to_phys(va);

    // C: pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);
    pr_info("PA: %pa for VA: 0x%lx\n", &pa as *const usize, va as usize);

    foo = kzalloc_obj::<foo>();
    if foo.is_null() {
        return -12; // -ENOMEM
    }

    pa = virt_to_phys(foo as *const c_void);
    va = foo as *mut c_void;
    // C: pr_info("PA: %pa for VA: 0x%lx\n", &pa, (unsigned long)va);
    pr_info("PA: %pa for VA: 0x%lx\n", &pa as *const usize, va as usize);

    0
}

// C: module_init(test_debug_virtual_init);

// C: static void __exit test_debug_virtual_exit(void)
#[no_mangle]
pub unsafe extern "C" fn test_debug_virtual_exit() {
    kfree(foo as *mut c_void);
}

// C: module_exit(test_debug_virtual_exit);
// C: MODULE_LICENSE("GPL");
// C: MODULE_DESCRIPTION("Test module for CONFIG_DEBUG_VIRTUAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
