/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: __IP27_COMMON_H

extern "C" {
    pub static mut master_nasid: nasid_t;

    pub fn cpu_node_probe();
    pub fn hub_rt_clock_event_init();
    pub fn hub_rtc_init(nasid: nasid_t);
    pub fn install_cpu_nmi_handler(slice: ::core::ffi::c_int);
    pub fn install_ipi();
    pub fn ip27_be_init();
    pub fn ip27_reboot_setup();
    pub static ip27_smp_ops: plat_smp_ops;
    pub fn node_getfirstfree(nasid: nasid_t) -> ::core::ffi::c_ulong;
    pub fn per_cpu_init();
    pub fn replicate_kernel_text();
    pub fn setup_replication_mask();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
