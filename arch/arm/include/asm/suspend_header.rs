/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct sleep_save_sp {
    pub save_ptr_stash: *mut u32,
    pub save_ptr_stash_phys: u32,
}

extern "C" {
    pub fn cpu_resume();
    pub fn cpu_resume_no_hyp();
    pub fn cpu_resume_arm();
    pub fn cpu_suspend(
        arg: core::ffi::c_ulong,
        fn_ptr: Option<unsafe extern "C" fn(core::ffi::c_ulong) -> core::ffi::c_int>,
    ) -> core::ffi::c_int;
    pub fn __cpu_suspend_save(
        ptr: *mut u32,
        ptrsz: u32,
        sp: u32,
        save_ptr: *mut u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
