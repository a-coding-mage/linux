/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent preserved from <linux/cpuidle.h> and <linux/linkage.h>.

unsafe extern "C" {
    pub static mut cpu_wait: Option<unsafe extern "C" fn()>;
    pub unsafe fn r4k_wait();
    pub unsafe fn r4k_wait_irqoff();

    pub fn check_wait();

    pub unsafe fn mips_cpuidle_wait_enter(
        dev: *mut cpuidle_device,
        drv: *mut cpuidle_driver,
        index: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn using_skipover_handler() -> bool {
    cpu_wait == Some(r4k_wait)
}

macro_rules! MIPS_CPUIDLE_WAIT_STATE {
    () => {
        cpuidle_state {
            enter: Some(mips_cpuidle_wait_enter),
            exit_latency: 1,
            target_residency: 1,
            power_usage: u32::MAX,
            name: "wait",
            desc: "MIPS wait",
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
