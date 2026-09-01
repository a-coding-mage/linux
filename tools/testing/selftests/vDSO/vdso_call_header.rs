/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Macro to call vDSO functions
 *
 * Copyright (C) 2024 Christophe Leroy <christophe.leroy@csgroup.eu>, CS GROUP France
 */

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
macro_rules! LOADARGS_1 {
    ($fn:expr, $__arg1:expr) => {{
        _r0 = $fn as *mut core::ffi::c_void;
        _r3 = $__arg1 as isize;
    }};
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
macro_rules! LOADARGS_2 {
    ($fn:expr, $__arg1:expr, $__arg2:expr) => {{
        _r0 = $fn as *mut core::ffi::c_void;
        _r3 = $__arg1 as isize;
        _r4 = $__arg2 as isize;
    }};
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
macro_rules! LOADARGS_3 {
    ($fn:expr, $__arg1:expr, $__arg2:expr, $__arg3:expr) => {{
        _r0 = $fn as *mut core::ffi::c_void;
        _r3 = $__arg1 as isize;
        _r4 = $__arg2 as isize;
        _r5 = $__arg3 as isize;
    }};
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
macro_rules! LOADARGS_5 {
    ($fn:expr, $__arg1:expr, $__arg2:expr, $__arg3:expr, $__arg4:expr, $__arg5:expr) => {{
        _r0 = $fn as *mut core::ffi::c_void;
        _r3 = $__arg1 as isize;
        _r4 = $__arg2 as isize;
        _r5 = $__arg3 as isize;
        _r6 = $__arg4 as isize;
        _r7 = $__arg5 as isize;
    }};
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
macro_rules! VDSO_CALL {
    ($fn:expr, 1, $($args:expr),*) => {{
        let mut _r0: *mut core::ffi::c_void;
        let mut _r3: isize;
        let mut _r4: isize;
        let mut _r5: isize;
        let mut _r6: isize;
        let mut _r7: isize;
        let mut _r8: isize;

        LOADARGS_1!($fn, $($args),*);

        unsafe {
            core::arch::asm!(
                "mtctr {0}",
                "bctrl",
                "bns+ 1f",
                "neg 3, 3",
                "1:",
                inout(reg) _r0,
                inout("r3") _r3,
                inout("r4") _r4,
                inout("r5") _r5,
                inout("r6") _r6,
                inout("r7") _r7,
                inout("r8") _r8,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                clobber_abi("C"),
                options(nostack)
            );
        }
        _r3
    }};
    ($fn:expr, 2, $($args:expr),*) => {{
        let mut _r0: *mut core::ffi::c_void;
        let mut _r3: isize;
        let mut _r4: isize;
        let mut _r5: isize;
        let mut _r6: isize;
        let mut _r7: isize;
        let mut _r8: isize;

        LOADARGS_2!($fn, $($args),*);

        unsafe {
            core::arch::asm!(
                "mtctr {0}",
                "bctrl",
                "bns+ 1f",
                "neg 3, 3",
                "1:",
                inout(reg) _r0,
                inout("r3") _r3,
                inout("r4") _r4,
                inout("r5") _r5,
                inout("r6") _r6,
                inout("r7") _r7,
                inout("r8") _r8,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                clobber_abi("C"),
                options(nostack)
            );
        }
        _r3
    }};
    ($fn:expr, 3, $($args:expr),*) => {{
        let mut _r0: *mut core::ffi::c_void;
        let mut _r3: isize;
        let mut _r4: isize;
        let mut _r5: isize;
        let mut _r6: isize;
        let mut _r7: isize;
        let mut _r8: isize;

        LOADARGS_3!($fn, $($args),*);

        unsafe {
            core::arch::asm!(
                "mtctr {0}",
                "bctrl",
                "bns+ 1f",
                "neg 3, 3",
                "1:",
                inout(reg) _r0,
                inout("r3") _r3,
                inout("r4") _r4,
                inout("r5") _r5,
                inout("r6") _r6,
                inout("r7") _r7,
                inout("r8") _r8,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                clobber_abi("C"),
                options(nostack)
            );
        }
        _r3
    }};
    ($fn:expr, 5, $($args:expr),*) => {{
        let mut _r0: *mut core::ffi::c_void;
        let mut _r3: isize;
        let mut _r4: isize;
        let mut _r5: isize;
        let mut _r6: isize;
        let mut _r7: isize;
        let mut _r8: isize;

        LOADARGS_5!($fn, $($args),*);

        unsafe {
            core::arch::asm!(
                "mtctr {0}",
                "bctrl",
                "bns+ 1f",
                "neg 3, 3",
                "1:",
                inout(reg) _r0,
                inout("r3") _r3,
                inout("r4") _r4,
                inout("r5") _r5,
                inout("r6") _r6,
                inout("r7") _r7,
                inout("r8") _r8,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                clobber_abi("C"),
                options(nostack)
            );
        }
        _r3
    }};
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
macro_rules! VDSO_CALL {
    ($fn:expr, $nr:expr $(, $args:expr)*) => {{
        $fn($($args),*)
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
