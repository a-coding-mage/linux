// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2012-2015 Altera Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    fn of_find_compatible_node(from: *mut c_void, typ: *mut c_void, compat: *const i8) -> *mut c_void;
    fn of_property_read_u32(np: *mut c_void, propname: *const i8, out: *mut u32) -> i32;
    fn pr_err(fmt: *const i8, ...);
    fn smp_wmb();
    fn sync_cache_w(addr: *const c_void);
    fn of_iomap(np: *mut c_void, index: i32) -> *mut c_void;
    fn irqchip_init();
    fn socfpga_init_l2_ecc();
    fn socfpga_init_ocram_ecc();
    fn socfpga_init_arria10_l2_ecc();
    fn socfpga_init_arria10_ocram_ecc();
    fn socfpga_reset_init();
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

#[no_mangle]
pub static mut sys_manager_base_addr: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub static mut rst_manager_base_addr: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub static mut sdr_ctl_base_addr: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub static mut socfpga_cpu1start_addr: usize = 0;

unsafe fn socfpga_sysmgr_init() {
    let mut np: *mut c_void = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"altr,sys-mgr\0".as_ptr() as *const i8,
    );

    if of_property_read_u32(
        np,
        b"cpu1-start-addr\0".as_ptr() as *const i8,
        &mut socfpga_cpu1start_addr as *mut usize as *mut u32,
    ) != 0 {
        pr_err(b"SMP: Need cpu1-start-addr in device tree.\n\0".as_ptr() as *const i8);
    }

    // Ensure that socfpga_cpu1start_addr is visible to other CPUs.
    smp_wmb();
    sync_cache_w(&socfpga_cpu1start_addr as *const usize as *const c_void);

    sys_manager_base_addr = of_iomap(np, 0);

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"altr,rst-mgr\0".as_ptr() as *const i8,
    );
    rst_manager_base_addr = of_iomap(np, 0);

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"altr,sdr-ctl\0".as_ptr() as *const i8,
    );
    sdr_ctl_base_addr = of_iomap(np, 0);
}

unsafe fn socfpga_init_irq() {
    irqchip_init();
    socfpga_sysmgr_init();
    // Preserved build-time condition: IS_ENABLED(CONFIG_EDAC_ALTERA_L2C).
    #[cfg(feature = "CONFIG_EDAC_ALTERA_L2C")]
    socfpga_init_l2_ecc();
    // Preserved build-time condition: IS_ENABLED(CONFIG_EDAC_ALTERA_OCRAM).
    #[cfg(feature = "CONFIG_EDAC_ALTERA_OCRAM")]
    socfpga_init_ocram_ecc();
    socfpga_reset_init();
}

unsafe fn socfpga_arria10_init_irq() {
    irqchip_init();
    socfpga_sysmgr_init();
    // Preserved build-time condition: IS_ENABLED(CONFIG_EDAC_ALTERA_L2C).
    #[cfg(feature = "CONFIG_EDAC_ALTERA_L2C")]
    socfpga_init_arria10_l2_ecc();
    // Preserved build-time condition: IS_ENABLED(CONFIG_EDAC_ALTERA_OCRAM).
    #[cfg(feature = "CONFIG_EDAC_ALTERA_OCRAM")]
    socfpga_init_arria10_ocram_ecc();
    socfpga_reset_init();
}

unsafe fn socfpga_cyclone5_restart(mode: i32, _cmd: *const i8) {
    let mut temp = readl((rst_manager_base_addr as usize + SOCFPGA_RSTMGR_CTRL as usize) as *const c_void);
    if mode == REBOOT_WARM {
        temp |= RSTMGR_CTRL_SWWARMRSTREQ;
    } else {
        temp |= RSTMGR_CTRL_SWCOLDRSTREQ;
    }
    writel(temp, (rst_manager_base_addr as usize + SOCFPGA_RSTMGR_CTRL as usize) as *mut c_void);
}

unsafe fn socfpga_arria10_restart(mode: i32, _cmd: *const i8) {
    let mut temp = readl((rst_manager_base_addr as usize + SOCFPGA_A10_RSTMGR_CTRL as usize) as *const c_void);
    if mode == REBOOT_WARM {
        temp |= RSTMGR_CTRL_SWWARMRSTREQ;
    } else {
        temp |= RSTMGR_CTRL_SWCOLDRSTREQ;
    }
    writel(temp, (rst_manager_base_addr as usize + SOCFPGA_A10_RSTMGR_CTRL as usize) as *mut c_void);
}

static altera_dt_match: [*const i8; 2] = [b"altr,socfpga\0".as_ptr() as *const i8, core::ptr::null()];
static altera_a10_dt_match: [*const i8; 2] = [b"altr,socfpga-arria10\0".as_ptr() as *const i8, core::ptr::null()];

// DT_MACHINE_START(SOCFPGA, "Altera SOCFPGA") / MACHINE_END
// .l2c_aux_val = 0, .l2c_aux_mask = ~0,
// .init_irq = socfpga_init_irq, .restart = socfpga_cyclone5_restart,
// .dt_compat = altera_dt_match

// DT_MACHINE_START(SOCFPGA_A10, "Altera SOCFPGA Arria10") / MACHINE_END
// .l2c_aux_val = 0, .l2c_aux_mask = ~0,
// .init_irq = socfpga_arria10_init_irq, .restart = socfpga_arria10_restart,
// .dt_compat = altera_a10_dt_match

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
