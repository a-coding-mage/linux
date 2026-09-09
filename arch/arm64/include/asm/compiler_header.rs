/* SPDX-License-Identifier: GPL-2.0 */

// ARM64_ASM_ARCH, when supplied by the build, is intended to select the
// assembler architecture preamble.

#[cfg(ARM64_ASM_ARCH)]
const ARM64_ASM_PREAMBLE: &str = concat!(".arch ", env!("ARM64_ASM_ARCH"), "\n");

#[cfg(not(ARM64_ASM_ARCH))]
const ARM64_ASM_PREAMBLE: &str = "";

macro_rules! xpaclri {
    ($ptr:expr) => {{
        let mut __xpaclri_ptr: usize = ($ptr) as usize;
        unsafe {
            core::arch::asm!(
                "hint #7",
                inlateout("x30") __xpaclri_ptr,
                options(nostack),
            );
        }
        __xpaclri_ptr
    }};
}

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
macro_rules! ptrauth_strip_kernel_insn_pac {
    ($ptr:expr) => {
        xpaclri!($ptr)
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
macro_rules! ptrauth_strip_kernel_insn_pac {
    ($ptr:expr) => {
        ($ptr)
    };
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
macro_rules! ptrauth_strip_user_insn_pac {
    ($ptr:expr) => {
        xpaclri!($ptr)
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
macro_rules! ptrauth_strip_user_insn_pac {
    ($ptr:expr) => {
        ($ptr)
    };
}

// CONFIG_BUILTIN_RETURN_ADDRESS_STRIPS_PAC controls whether the compiler's
// builtin already strips pointer authentication codes.
#[cfg(not(CONFIG_BUILTIN_RETURN_ADDRESS_STRIPS_PAC))]
macro_rules! __builtin_return_address {
    ($val:expr) => {{
        ptrauth_strip_kernel_insn_pac!(unsafe {
            core::mem::transmute::<*mut core::ffi::c_void, usize>(
                __builtin_return_address!($val),
            )
        }) as *mut core::ffi::c_void
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
