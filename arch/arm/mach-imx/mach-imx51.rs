// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2011 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_char;

extern "C" {
    fn mxc_set_cpu_type(cpu_type: u32);
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn imx_writel(value: u32, addr: *mut u8);
    fn imx_readl(addr: *mut u8) -> u32;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(node: *mut device_node);
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn imx_src_init();
    fn imx5_pmu_init();
    fn imx_aips_allow_unprivileged_access(compatible: *const c_char);
    fn mx51_neon_fixup();
    fn imx51_pm_init();
    fn pr_err(message: *const c_char);
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

const MXC_CPU_MX51: u32 = 51;

unsafe fn imx51_init_early() {
    mxc_set_cpu_type(MXC_CPU_MX51);
}

/*
 * The MIPI HSC unit has been removed from the i.MX51 Reference Manual by
 * the Freescale marketing division. However this did not remove the
 * hardware from the chip which still needs to be configured for proper
 * IPU support.
 */
const MX51_MIPI_HSC_BASE: usize = 0x83fdc000;

unsafe fn imx51_ipu_mipi_setup() {
    let hsc_addr = ioremap(MX51_MIPI_HSC_BASE, 16 * 1024);
    warn_on(hsc_addr.is_null());

    /* setup MIPI module to legacy mode */
    imx_writel(0xf00, hsc_addr);

    /* CSI mode: reserved; DI control mode: legacy (from Freescale BSP) */
    let control = imx_readl(hsc_addr.add(0x800)) | 0x30ff;
    imx_writel(control, hsc_addr.add(0x800));

    iounmap(hsc_addr);
}

unsafe fn imx51_m4if_setup() {
    let np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl,imx51-m4if\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        return;
    }

    let m4if_base = of_iomap(np, 0);
    of_node_put(np);
    if m4if_base.is_null() {
        pr_err(b"Unable to map M4IF registers\n\0".as_ptr() as *const c_char);
        return;
    }

    /*
     * Configure VPU and IPU with higher priorities
     * in order to avoid artifacts during video playback
     */
    writel_relaxed(0x00000203, m4if_base.add(0x40));
    writel_relaxed(0x00000000, m4if_base.add(0x44));
    writel_relaxed(0x00120125, m4if_base.add(0x9c));
    writel_relaxed(0x001901A3, m4if_base.add(0x48));
    iounmap(m4if_base);
}

unsafe fn imx51_dt_init() {
    imx51_ipu_mipi_setup();
    imx_src_init();
    imx51_m4if_setup();
    imx5_pmu_init();
    imx_aips_allow_unprivileged_access(b"fsl,imx51-aipstz\0".as_ptr() as *const c_char);
}

unsafe fn imx51_init_late() {
    mx51_neon_fixup();
    imx51_pm_init();
}

static IMX51_DT_BOARD_COMPAT: [*const c_char; 2] = [
    b"fsl,imx51\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/* DT_MACHINE_START(IMX51_DT, "Freescale i.MX51 (Device Tree Support)") */
#[repr(C)]
pub struct MachineDesc {
    pub init_early: unsafe fn(),
    pub init_machine: unsafe fn(),
    pub init_late: unsafe fn(),
    pub dt_compat: *const *const c_char,
}

#[no_mangle]
pub static IMX51_DT: MachineDesc = MachineDesc {
    init_early: imx51_init_early,
    init_machine: imx51_dt_init,
    init_late: imx51_init_late,
    dt_compat: IMX51_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
