/*
 * P1022 RDK board specific routines
 *
 * Copyright 2012 Freescale Semiconductor, Inc.
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * Based on p1022_ds.c
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external symbols.

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
const CLKDVDR_PXCKEN: u32 = 0x8000_0000;
#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
const CLKDVDR_PXCKINV: u32 = 0x1000_0000;
#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
const CLKDVDR_PXCKDLY: u32 = 0x0600_0000;
#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
const CLKDVDR_PXCLK_MASK: u32 = 0x00ff_0000;

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
unsafe fn p1022rdk_set_pixel_clock(pixclock: u32) {
    let mut guts_np: *mut device_node = core::ptr::null_mut();
    let guts: *mut ccsr_guts;
    let freq: u64;
    let mut temp: u64 = 1_000_000_000_000;
    let mut pxclk: u32;

    /* Map the global utilities registers. */
    guts_np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"fsl,p1022-guts\0".as_ptr() as *const i8,
    );
    if guts_np.is_null() {
        pr_err(b"p1022rdk: missing global utilities device node\n\0".as_ptr() as *const i8);
        return;
    }

    guts = of_iomap(guts_np, 0);
    of_node_put(guts_np);
    if guts.is_null() {
        pr_err(b"p1022rdk: could not map global utilities device\n\0".as_ptr() as *const i8);
        return;
    }

    /* Convert pixclock from a wavelength to a frequency */
    temp /= pixclock as u64;
    freq = temp;

    /*
     * 'pxclk' is the ratio of the platform clock to the pixel clock.
     * This number is programmed into the CLKDVDR register, and the valid
     * range of values is 2-255.
     */
    pxclk = div_round_closest(fsl_get_sys_freq(), freq) as u32;
    pxclk = pxclk.clamp(2, 255);

    /* Disable the pixel clock, and set it to non-inverted and no delay */
    clrbits32(
        core::ptr::addr_of_mut!((*guts).clkdvdr),
        CLKDVDR_PXCKEN | CLKDVDR_PXCKDLY | CLKDVDR_PXCLK_MASK,
    );

    /* Enable the clock and set the pxclk */
    setbits32(
        core::ptr::addr_of_mut!((*guts).clkdvdr),
        CLKDVDR_PXCKEN | (pxclk << 16),
    );

    iounmap(guts as *mut core::ffi::c_void);
}

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
unsafe fn p1022rdk_valid_monitor_port(_port: fsl_diu_monitor_port) -> fsl_diu_monitor_port {
    FSL_DIU_PORT_DVI
}

unsafe fn p1022_rdk_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const i8,
    );
    BUG_ON(mpic.is_null());
    mpic_init(mpic);
}

/*
 * Setup the architecture
 */
unsafe fn p1022_rdk_setup_arch() {
    if !ppc_md.progress.is_none() {
        ppc_md.progress.unwrap()(b"p1022_rdk_setup_arch()\0".as_ptr() as *const i8, 0);
    }

    #[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
    {
        diu_ops.set_pixel_clock = Some(p1022rdk_set_pixel_clock);
        diu_ops.valid_monitor_port = Some(p1022rdk_valid_monitor_port);
    }

    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    pr_info(b"Freescale / iVeia P1022 RDK reference board\n\0".as_ptr() as *const i8);
}

// machine_arch_initcall(p1022_rdk, mpc85xx_common_publish_devices);
// define_machine(p1022_rdk) {
//     .name = "P1022 RDK",
//     .compatible = "fsl,p1022rdk",
//     .setup_arch = p1022_rdk_setup_arch,
//     .init_IRQ = p1022_rdk_pic_init,
// #ifdef CONFIG_PCI
//     .pcibios_fixup_bus = fsl_pcibios_fixup_bus,
//     .pcibios_fixup_phb = fsl_pcibios_fixup_phb,
// #endif
//     .get_irq = mpic_get_irq,
//     .progress = udbg_progress,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
