// SPDX-License-Identifier: GPL-2.0

unsafe extern "C" {
    fn copy_to_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;

    #[cfg(feature = "DEBUG")]
    fn printk(format: *const core::ffi::c_char, ...) -> i32;
}

pub unsafe fn stfiwx(fr_s: *mut u32, ea: *mut core::ffi::c_void) -> i32 {
    #[cfg(feature = "DEBUG")]
    {
        // Equivalent to: printk("%s: %p %p\n", __func__, frS, ea);
        let format = b"%s: %p %p\n\0";
        let function = b"stfiwx\0";
        unsafe {
            printk(
                format.as_ptr() as *const core::ffi::c_char,
                function.as_ptr() as *const core::ffi::c_char,
                fr_s,
                ea,
            );
        }
    }

    if unsafe {
        copy_to_user(
            ea,
            fr_s.add(1) as *const core::ffi::c_void,
            core::mem::size_of::<u32>(),
        )
    } != 0
    {
        return -14; // -EFAULT
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
