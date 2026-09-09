/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <uapi/asm/setup.h>

/*
 * This is set up by the setup-routine at boot-time
 */
extern "C" {
    pub static mut boot_params_page: [u8; 0];
}

// #define PARAM boot_params_page
pub const PARAM: *mut u8 = unsafe { core::ptr::addr_of_mut!(boot_params_page) as *mut u8 };

// These accessors preserve the C macros' volatile, unsigned-long lvalue behavior.
#[inline]
pub unsafe fn MOUNT_ROOT_RDONLY() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x000) as *mut core::ffi::c_ulong)
}

#[inline]
pub unsafe fn RAMDISK_FLAGS() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x004) as *mut core::ffi::c_ulong)
}

#[inline]
pub unsafe fn ORIG_ROOT_DEV() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x008) as *mut core::ffi::c_ulong)
}

#[inline]
pub unsafe fn LOADER_TYPE() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x00c) as *mut core::ffi::c_ulong)
}

#[inline]
pub unsafe fn INITRD_START() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x010) as *mut core::ffi::c_ulong)
}

#[inline]
pub unsafe fn INITRD_SIZE() -> &'static mut core::ffi::c_ulong {
    &mut *(PARAM.add(0x014) as *mut core::ffi::c_ulong)
}

/* ... */
#[inline]
pub unsafe fn COMMAND_LINE() -> *mut core::ffi::c_char {
    PARAM.add(0x100) as *mut core::ffi::c_char
}

unsafe extern "C" {
    pub fn sh_mv_setup();
    pub fn check_for_initrd();
    pub fn per_cpu_trap_init();
    pub fn sh_fdt_init(dt_phys: phys_addr_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
