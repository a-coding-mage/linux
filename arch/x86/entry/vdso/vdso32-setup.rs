// SPDX-License-Identifier: GPL-2.0
/*
 * (C) Copyright 2002 Linus Torvalds
 * Portions based on the vdso-randomization code from exec-shield:
 * Copyright(C) 2005-2006, Red Hat, Inc., Ingo Molnar
 *
 * This file contains the needed initializations to support sysenter.
 */

// Kernel and architecture declarations supplied by other translation units.

#[cfg(not(CONFIG_COMPAT_VDSO))]
const VDSO_DEFAULT: ::core::ffi::c_uint = 1;
#[cfg(CONFIG_COMPAT_VDSO)]
const VDSO_DEFAULT: ::core::ffi::c_uint = 0;

/*
 * Should the kernel map a VDSO page into processes and pass its
 * address down to glibc upon exec()?
 */
#[no_mangle]
pub static mut vdso32_enabled: ::core::ffi::c_uint = VDSO_DEFAULT;

extern "C" {
    fn simple_strtoul(
        s: *const ::core::ffi::c_char,
        endp: *mut *mut ::core::ffi::c_char,
        base: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_ulong;
}

#[allow(non_snake_case)]
unsafe fn vdso32_setup(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    vdso32_enabled = simple_strtoul(s, core::ptr::null_mut(), 0) as ::core::ffi::c_uint;

    if vdso32_enabled > 1 {
        // pr_warn("vdso32 values other than 0 and 1 are no longer allowed; vdso disabled\n");
        vdso32_enabled = 0;
    }

    1
}

/*
 * For consistency, the argument vdso32=[012] affects the 32-bit vDSO
 * behavior on both 64-bit and 32-bit kernels.
 * On 32-bit kernels, vdso=[012] means the same thing.
 */
// __setup("vdso32=", vdso32_setup);

#[cfg(CONFIG_X86_32)]
// __setup_param("vdso=", vdso_setup, vdso32_setup, 0);
const _: () = ();

#[cfg(CONFIG_SYSCTL)]
mod sysctl {
    use super::vdso32_enabled;

    // The ctl_table type, handlers, registration functions, and SYSCTL_* values
    // are supplied by the kernel sysctl implementation.
    extern "C" {
        fn register_sysctl(name: *const ::core::ffi::c_char, table: *const ctl_table);
        fn register_sysctl_init(name: *const ::core::ffi::c_char, table: *const ctl_table);
        fn proc_dointvec_minmax(
            table: *mut ctl_table,
            write: ::core::ffi::c_int,
            buffer: *mut ::core::ffi::c_void,
            length: *mut usize,
            ppos: *mut u64,
        ) -> ::core::ffi::c_int;
    }

    #[repr(C)]
    struct ctl_table {
        procname: *const ::core::ffi::c_char,
        data: *mut ::core::ffi::c_void,
        maxlen: ::core::ffi::c_int,
        mode: u16,
        proc_handler: Option<unsafe extern "C" fn(
            *mut ctl_table,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            *mut usize,
            *mut u64,
        ) -> ::core::ffi::c_int>,
        extra1: *const ::core::ffi::c_void,
        extra2: *const ::core::ffi::c_void,
    }

    static vdso_table: [ctl_table; 1] = [ctl_table {
        #[cfg(CONFIG_X86_64)]
        procname: b"vsyscall32\0".as_ptr() as *const _,
        #[cfg(not(CONFIG_X86_64))]
        procname: b"vdso_enabled\0".as_ptr() as *const _,
        data: unsafe { &vdso32_enabled as *const _ as *mut ::core::ffi::c_void },
        maxlen: core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: core::ptr::null(),
        extra2: core::ptr::null(),
    }];

    unsafe fn ia32_binfmt_init() -> ::core::ffi::c_int {
        #[cfg(CONFIG_X86_64)]
        {
            // Register vsyscall32 into the ABI table
            register_sysctl(b"abi\0".as_ptr() as *const _, vdso_table.as_ptr());
        }
        #[cfg(not(CONFIG_X86_64))]
        {
            register_sysctl_init(b"vm\0".as_ptr() as *const _, vdso_table.as_ptr());
        }
        0
    }

    // __initcall(ia32_binfmt_init);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
