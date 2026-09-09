/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct plat_smp_ops {
    pub smp_setup: Option<unsafe extern "C" fn()>,
    pub smp_processor_id: Option<unsafe extern "C" fn() -> core::ffi::c_uint>,
    pub prepare_cpus: Option<unsafe extern "C" fn(max_cpus: core::ffi::c_uint)>,
    pub start_cpu:
        Option<unsafe extern "C" fn(cpu: core::ffi::c_uint, entry_point: core::ffi::c_ulong)>,
    pub send_ipi:
        Option<unsafe extern "C" fn(cpu: core::ffi::c_uint, message: core::ffi::c_uint)>,
    pub cpu_disable: Option<unsafe extern "C" fn(cpu: core::ffi::c_uint) -> core::ffi::c_int>,
    pub cpu_die: Option<unsafe extern "C" fn(cpu: core::ffi::c_uint)>,
    pub play_dead: Option<unsafe extern "C" fn()>,
}

extern "C" {
    pub static mut mp_ops: *mut plat_smp_ops;
    pub static mut shx3_smp_ops: plat_smp_ops;

    fn BUG() -> !;
}

// CONFIG_SMP is a build-time condition from the C source.
#[cfg(CONFIG_SMP)]
pub unsafe fn plat_smp_setup() {
    if mp_ops.is_null() {
        BUG();
    }
    ((*mp_ops).smp_setup.expect("smp_setup is required"))();
}

#[cfg(CONFIG_SMP)]
pub unsafe fn play_dead() -> ! {
    ((*mp_ops).play_dead.expect("play_dead is required"))();
    BUG();
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn register_smp_ops(ops: *mut plat_smp_ops);
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn plat_smp_setup() {
    /* UP, nothing to do ... */
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn register_smp_ops(_ops: *mut plat_smp_ops) {
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn play_dead() -> ! {
    BUG();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
