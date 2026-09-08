// SPDX-License-Identifier: GPL-2.0

// Kernel includes (treated as external dependencies):
// #include <linux/kernel.h>
// #include <linux/sched.h>
// #include <linux/cred.h>
// #include <linux/err.h>
// #include <linux/efi.h>
// #include <linux/slab.h>
// #include <keys/asymmetric-type.h>
// #include <keys/system_keyring.h>
// #include <asm/boot_data.h>
// #include "../integrity.h"

extern "C" {
    fn __va(addr: usize) -> *mut core::ffi::c_void;
    fn add_to_platform_keyring(
        keyring: *const u8,
        data: *mut core::ffi::c_void,
        len: u32,
    );

    static ipl_cert_list_addr: usize;
    static ipl_cert_list_size: usize;
}

/// Load the certs contained in the IPL report created by the machine loader
/// into the platform trusted keyring.
unsafe fn load_ipl_certs() -> i32 {
    let mut ptr: *mut core::ffi::c_void;
    let end: *mut core::ffi::c_void;
    let mut len: u32;

    if ipl_cert_list_addr == 0 {
        return 0;
    }

    // Copy the certificates to the platform keyring
    ptr = __va(ipl_cert_list_addr);
    end = (ptr as usize + ipl_cert_list_size) as *mut core::ffi::c_void;

    while (ptr as usize) < (end as usize) {
        len = *(ptr as *const u32);
        ptr = (ptr as usize + core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
        add_to_platform_keyring(b"IPL:db\0".as_ptr(), ptr, len);
        ptr = (ptr as usize + len as usize) as *mut core::ffi::c_void;
    }
    0
}

// Kernel macro: late_initcall(load_ipl_certs)
// This registers load_ipl_certs to be called during the late initialization phase.
// In Linux kernel Rust code, module_init!() or similar kernel initialization
// macros would be used to achieve this.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
