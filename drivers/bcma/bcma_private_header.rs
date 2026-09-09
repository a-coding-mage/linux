/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/bcma/bcma.h, linux/delay.h

#[allow(unused_macros)]
macro_rules! bcma_err {
    ($bus:expr, $fmt:expr $(, $args:expr)*) => {
        dev_err(($bus).dev, concat!("bus%d: ", $fmt), ($bus).num $(, $args)*)
    };
}
#[allow(unused_macros)]
macro_rules! bcma_warn {
    ($bus:expr, $fmt:expr $(, $args:expr)*) => {
        dev_warn(($bus).dev, concat!("bus%d: ", $fmt), ($bus).num $(, $args)*)
    };
}
#[allow(unused_macros)]
macro_rules! bcma_info {
    ($bus:expr, $fmt:expr $(, $args:expr)*) => {
        dev_info(($bus).dev, concat!("bus%d: ", $fmt), ($bus).num $(, $args)*)
    };
}
#[allow(unused_macros)]
macro_rules! bcma_debug {
    ($bus:expr, $fmt:expr $(, $args:expr)*) => {
        dev_dbg(($bus).dev, concat!("bus%d: ", $fmt), ($bus).num $(, $args)*)
    };
}

pub struct bcma_bus;

extern "C" {
pub unsafe extern "C" fn bcma_wait_value(core: *mut bcma_device, reg: u16, mask: u32, value: u32, timeout: i32) -> bool;
pub unsafe extern "C" fn bcma_prepare_core(bus: *mut bcma_bus, core: *mut bcma_device);
pub unsafe extern "C" fn bcma_init_bus(bus: *mut bcma_bus);
pub unsafe extern "C" fn bcma_unregister_cores(bus: *mut bcma_bus);
pub unsafe extern "C" fn bcma_bus_register(bus: *mut bcma_bus) -> i32;
pub unsafe extern "C" fn bcma_bus_unregister(bus: *mut bcma_bus);
pub unsafe extern "C" fn bcma_bus_early_register(bus: *mut bcma_bus) -> i32;

// CONFIG_PM
pub unsafe extern "C" fn bcma_bus_suspend(bus: *mut bcma_bus) -> i32;
pub unsafe extern "C" fn bcma_bus_resume(bus: *mut bcma_bus) -> i32;

pub unsafe extern "C" fn bcma_detect_chip(bus: *mut bcma_bus);
pub unsafe extern "C" fn bcma_bus_scan(bus: *mut bcma_bus) -> i32;
pub unsafe extern "C" fn bcma_sprom_get(bus: *mut bcma_bus) -> i32;

pub unsafe extern "C" fn bcma_core_chipcommon_early_init(cc: *mut bcma_drv_cc);
pub unsafe extern "C" fn bcma_core_chipcommon_init(cc: *mut bcma_drv_cc);
pub unsafe extern "C" fn bcma_chipco_bcm4331_ext_pa_lines_ctl(cc: *mut bcma_drv_cc, enable: bool);
// CONFIG_BCMA_DRIVER_MIPS
pub unsafe extern "C" fn bcma_chipco_serial_init(cc: *mut bcma_drv_cc);

pub unsafe extern "C" fn bcma_core_chipcommon_b_init(ccb: *mut bcma_drv_cc_b) -> i32;
pub unsafe extern "C" fn bcma_core_chipcommon_b_free(ccb: *mut bcma_drv_cc_b);
pub unsafe extern "C" fn bcma_pmu_early_init(cc: *mut bcma_drv_cc);
pub unsafe extern "C" fn bcma_pmu_init(cc: *mut bcma_drv_cc);
pub unsafe extern "C" fn bcma_pmu_get_alp_clock(cc: *mut bcma_drv_cc) -> u32;
pub unsafe extern "C" fn bcma_pmu_get_cpu_clock(cc: *mut bcma_drv_cc) -> u32;

// driver_chipcommon_sflash.c
pub unsafe extern "C" fn bcma_pflash_init(cc: *mut bcma_drv_cc) -> i32;
pub unsafe extern "C" fn bcma_sflash_init(cc: *mut bcma_drv_cc) -> i32;
pub unsafe extern "C" fn bcma_nflash_init(cc: *mut bcma_drv_cc) -> i32;

pub static mut bcma_pflash_dev: platform_device;
pub static mut bcma_sflash_dev: platform_device;

pub unsafe extern "C" fn bcma_host_pci_init() -> i32;
pub unsafe extern "C" fn bcma_host_pci_exit();
pub unsafe extern "C" fn bcma_host_soc_register_driver() -> i32;
pub unsafe extern "C" fn bcma_host_soc_unregister_driver();

pub unsafe extern "C" fn bcma_pcie_read(pc: *mut bcma_drv_pci, address: u32) -> u32;
pub unsafe extern "C" fn bcma_core_pci_early_init(pc: *mut bcma_drv_pci);
pub unsafe extern "C" fn bcma_core_pci_init(pc: *mut bcma_drv_pci);
pub unsafe extern "C" fn bcma_core_pci_up(pc: *mut bcma_drv_pci);
pub unsafe extern "C" fn bcma_core_pci_down(pc: *mut bcma_drv_pci);
pub unsafe extern "C" fn bcma_core_pcie2_init(pcie2: *mut bcma_drv_pcie2);
pub unsafe extern "C" fn bcma_core_pcie2_up(pcie2: *mut bcma_drv_pcie2);

pub unsafe extern "C" fn bcma_chipco_watchdog_register(cc: *mut bcma_drv_cc) -> i32;
pub unsafe extern "C" fn bcma_core_pci_is_in_hostmode(pc: *mut bcma_drv_pci) -> bool;
pub unsafe extern "C" fn bcma_core_pci_hostmode_init(pc: *mut bcma_drv_pci);

pub unsafe extern "C" fn bcma_core_mips_irq(dev: *mut bcma_device) -> u32;
pub unsafe extern "C" fn bcma_core_mips_early_init(mcore: *mut bcma_drv_mips);
pub unsafe extern "C" fn bcma_core_mips_init(mcore: *mut bcma_drv_mips);
pub unsafe extern "C" fn bcma_core_gmac_cmn_init(gc: *mut bcma_drv_gmac_cmn);

pub unsafe extern "C" fn bcma_gpio_init(cc: *mut bcma_drv_cc) -> i32;
pub unsafe extern "C" fn bcma_gpio_unregister(cc: *mut bcma_drv_cc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
