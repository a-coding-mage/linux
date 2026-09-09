/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/pci.h>
// #include <linux/irq.h>
// #include <asm/pmac_feature.h>

// Opaque types supplied by the corresponding external dependencies.
#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_controller_ops {
    _private: [u8; 0],
}

// `time64_t` is supplied by the external kernel type definitions.
pub type time64_t = i64;

/*
 * Declaration for the various functions exported by the
 * pmac_* files. Mostly for use by pmac_setup
 */

extern "C" {
    pub static mut pmac_newworld: i32;

    pub fn g5_phy_disable_cpu1();

    pub fn pmac_time_init() -> i64;
    pub fn pmac_get_boot_time() -> time64_t;
    pub fn pmac_get_rtc_time(arg1: *mut rtc_time);
    pub fn pmac_set_rtc_time(arg1: *mut rtc_time) -> i32;
    pub fn pmac_read_rtc_time();
    pub fn pmac_calibrate_decr();
    pub fn pmac_pci_irq_fixup(arg1: *mut pci_dev);
    pub fn pmac_pci_init();

    pub fn pmac_nvram_update();
    pub fn pmac_nvram_read_byte(addr: i32) -> u8;
    pub fn pmac_nvram_write_byte(addr: i32, val: u8);
    pub fn pmac_pcibios_after_init();

    pub fn pmac_setup_pci_dma();
    pub fn pmac_check_ht_link();

    pub fn pmac_setup_smp();
    pub static mut psurge_secondary_virq: i32;
    pub fn low_cpu_offline_self() -> !;

    pub fn pmac_nvram_init() -> i32;
    pub fn pmac_pic_init();

    pub static mut pmac_pci_controller_ops: pci_controller_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
