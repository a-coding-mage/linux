/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/libfdt.h and linux/of.h

#[repr(C)]
pub struct mips_machine {
    pub matches: *const of_device_id,
    pub fdt: *const core::ffi::c_void,
    pub detect: Option<unsafe extern "C" fn() -> bool>,
    pub fixup_fdt: Option<
        unsafe extern "C" fn(
            fdt: *const core::ffi::c_void,
            match_data: *const core::ffi::c_void,
        ) -> *const core::ffi::c_void,
    >,
    pub measure_hpt_freq: Option<unsafe extern "C" fn() -> core::ffi::c_uint>,
}

unsafe extern "C" {
    pub static mut __mips_machines_start: core::ffi::c_long;
    pub static mut __mips_machines_end: core::ffi::c_long;
}

// C macro: declares a used machine descriptor in the .mips.machines.init section.
// The linker-section and identifier concatenation are retained as macro intent.
#[macro_export]
macro_rules! MIPS_MACHINE {
    ($name:ident) => {
        #[used]
        #[unsafe(link_section = ".mips.machines.init")]
        static __mips_mach_$name: $crate::mips_machine;
    };
}

// C macro equivalent: iterate over the linker-provided machine descriptor range.
#[macro_export]
macro_rules! for_each_mips_machine {
    ($mach:ident, $body:block) => {{
        let mut $mach = unsafe {
            &*((&raw mut __mips_machines_start) as *mut mips_machine)
        };
        let end = unsafe {
            (&raw mut __mips_machines_end) as *mut mips_machine
        };
        while (core::ptr::addr_of!(*$mach) as *const mips_machine) < end {
            $body
            $mach = unsafe { &*((*$mach as *const mips_machine).add(1)) };
        }
    }};
}

/**
 * mips_machine_is_compatible() - check if a machine is compatible with an FDT
 * @mach: the machine struct to check
 * @fdt: the FDT to check for compatibility with
 *
 * Check whether the given machine @mach is compatible with the given flattened
 * device tree @fdt, based upon the compatibility property of the root node.
 *
 * Return: the device id matched if any, else NULL
 */
pub unsafe fn mips_machine_is_compatible(
    mach: *const mips_machine,
    fdt: *const core::ffi::c_void,
) -> *const of_device_id {
    let mut match_: *const of_device_id;

    if (*mach).matches.is_null() {
        return core::ptr::null();
    }

    match_ = (*mach).matches;
    while (*match_).compatible[0] != 0 {
        if fdt_node_check_compatible(fdt, 0, (*match_).compatible.as_ptr()) == 0 {
            return match_;
        }
        match_ = match_.add(1);
    }

    core::ptr::null()
}

/**
 * struct mips_fdt_fixup - Describe a fixup to apply to an FDT
 * @apply: applies the fixup to @fdt, returns zero on success else -errno
 * @description: a short description of the fixup
 *
 * Describes a fixup applied to an FDT blob by the @apply function. The
 * @description field provides a short description of the fixup intended for
 * use in error messages if the @apply function returns non-zero.
 */
#[repr(C)]
pub struct mips_fdt_fixup {
    pub apply: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub description: *const core::ffi::c_char,
}

/**
 * apply_mips_fdt_fixups() - apply fixups to an FDT blob
 * @fdt_out: buffer in which to place the fixed-up FDT
 * @fdt_out_size: the size of the @fdt_out buffer
 * @fdt_in: the FDT blob
 * @fixups: pointer to an array of fixups to be applied
 *
 * Loop through the array of fixups pointed to by @fixups, calling the apply
 * function on each until either one returns an error or we reach the end of
 * the list as indicated by an entry with a NULL apply field.
 *
 * Return: zero on success, else -errno
 */
unsafe extern "C" {
    pub fn apply_mips_fdt_fixups(
        fdt_out: *mut core::ffi::c_void,
        fdt_out_size: usize,
        fdt_in: *const core::ffi::c_void,
        fixups: *const mips_fdt_fixup,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
