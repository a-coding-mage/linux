/* SPDX-License-Identifier: GPL-2.0 */

// index into the page table containing the entry OFW occupies
pub const OLPC_OFW_PDE_NR: i32 = 1022;

pub const OLPC_OFW_SIG: u32 = 0x2057_464f; // aka "OFW "

#[cfg(CONFIG_OLPC)]
extern "C" {
    pub fn olpc_ofw_is_installed() -> bool;

    // run an OFW command by calling into the firmware
    pub fn __olpc_ofw(
        name: *const core::ffi::c_char,
        nr_args: core::ffi::c_int,
        args: *mut *const core::ffi::c_void,
        nr_res: core::ffi::c_int,
        res: *mut *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    // determine whether OFW is available and lives in the proper memory
    pub fn olpc_ofw_detect();

    // install OFW's pde permanently into the kernel's pgtable
    pub fn setup_olpc_ofw_pgd();

    // check if OFW was detected during boot
    pub fn olpc_ofw_present() -> bool;

    pub fn olpc_dt_build_devicetree();
}

// Run an OFW command by calling into the firmware. This preserves the C
// ARRAY_SIZE-based interface for array arguments.
#[cfg(CONFIG_OLPC)]
#[macro_export]
macro_rules! olpc_ofw {
    ($name:expr, $args:expr, $res:expr) => {
        unsafe {
            $crate::__olpc_ofw(
                $name,
                $args.len() as core::ffi::c_int,
                $args.as_mut_ptr(),
                $res.len() as core::ffi::c_int,
                $res.as_mut_ptr(),
            )
        }
    };
}

#[cfg(not(CONFIG_OLPC))]
#[inline]
pub fn olpc_ofw_detect() {}

#[cfg(not(CONFIG_OLPC))]
#[inline]
pub fn setup_olpc_ofw_pgd() {}

#[cfg(not(CONFIG_OLPC))]
#[inline]
pub fn olpc_dt_build_devicetree() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
