/*
 * P1022DS board specific routines
 *
 * Authors: Travis Wheatley <travis.wheatley@freescale.com>
 *          Dave Liu <daveliu@freescale.com>
 *          Timur Tabi <timur@freescale.com>
 *
 * Copyright 2010 Freescale Semiconductor, Inc.
 *
 * This file is taken from the Freescale P1022DS BSP, with modifications:
 * 2) No AMP support
 * 3) No PCI endpoint support
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

/* Kernel headers supplying the following types, constants, and functions are
 * intentionally external to this translation. */

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
mod diu {
    use super::*;

    pub const PMUXCR_ELBCDIU_MASK: u32 = 0xc0000000;
    pub const PMUXCR_ELBCDIU_NOR16: u32 = 0x80000000;
    pub const PMUXCR_ELBCDIU_DIU: u32 = 0x40000000;
    pub const CLKDVDR_PXCKEN: u32 = 0x80000000;
    pub const CLKDVDR_PXCKINV: u32 = 0x10000000;
    pub const CLKDVDR_PXCKDLY: u32 = 0x06000000;
    pub const CLKDVDR_PXCLK_MASK: u32 = 0x00ff0000;
    pub const PX_CTL: usize = 3;
    pub const PX_BRDCFG0: u8 = 8;
    pub const PX_BRDCFG1: u8 = 9;
    pub const PX_BRDCFG0_ELBC_DIU: u8 = 0x02;
    pub const PX_BRDCFG1_DVIEN: u8 = 0x80;
    pub const PX_BRDCFG1_DFPEN: u8 = 0x40;
    pub const PX_BRDCFG1_BACKLIGHT: u8 = 0x20;
    pub const PX_CTL_ALTACC: u8 = 0x80;
    pub const AD_BYTE_F: u32 = 0x10000000;
    pub const AD_ALPHA_C_SHIFT: u32 = 25;
    pub const AD_BLUE_C_SHIFT: u32 = 23;
    pub const AD_GREEN_C_SHIFT: u32 = 21;
    pub const AD_RED_C_SHIFT: u32 = 19;
    pub const AD_PIXEL_S_SHIFT: u32 = 16;
    pub const AD_COMP_3_SHIFT: u32 = 12;
    pub const AD_COMP_2_SHIFT: u32 = 8;
    pub const AD_COMP_1_SHIFT: u32 = 4;
    pub const AD_COMP_0_SHIFT: u32 = 0;

    macro_rules! MAKE_AD {
        ($alpha:expr, $red:expr, $blue:expr, $green:expr, $size:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr) => {
            cpu_to_le32(AD_BYTE_F | (($alpha as u32) << AD_ALPHA_C_SHIFT) |
                (($blue as u32) << AD_BLUE_C_SHIFT) | (($green as u32) << AD_GREEN_C_SHIFT) |
                (($red as u32) << AD_RED_C_SHIFT) | (($c3 as u32) << AD_COMP_3_SHIFT) |
                (($c2 as u32) << AD_COMP_2_SHIFT) | (($c1 as u32) << AD_COMP_1_SHIFT) |
                (($c0 as u32) << AD_COMP_0_SHIFT) | (($size as u32) << AD_PIXEL_S_SHIFT))
        };
    }

    #[repr(C)]
    pub struct FslLaw { pub lawbar: u32, pub reserved1: u32, pub lawar: u32, pub reserved: [u32; 5] }
    pub const LAWBAR_MASK: u32 = 0x00f00000;
    pub const LAWAR_MASK: u32 = 0x81f00000;
    pub const LAWAR_MATCH: u32 = 0x80400000;
    pub const BR_BA: u32 = 0xffff8000;

    unsafe fn lbc_br_to_phys(ecm: *const u8, count: u32, br: u32) -> u64 {
        #[cfg(not(feature = "CONFIG_PHYS_64BIT"))]
        { return (br & BR_BA) as u64; }
        #[cfg(feature = "CONFIG_PHYS_64BIT")]
        {
            let law = ecm.add(0xc08) as *const FslLaw;
            for i in 0..count as usize {
                let lawbar = in_be32(&(*law.add(i)).lawbar);
                let lawar = in_be32(&(*law.add(i)).lawar);
                if lawar & LAWAR_MASK == LAWAR_MATCH { return (br & BR_BA) as u64 | ((lawbar & LAWBAR_MASK) as u64) << 12; }
            }
            0
        }
    }

    /* Board-specific MMIO routines.  Kernel object layouts and accessors are
     * supplied by the surrounding kernel translation. */
    pub unsafe fn p1022ds_set_monitor_port(port: i32) {
        let guts_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"fsl,p1022-guts\0".as_ptr() as *const i8);
        if guts_node.is_null() { pr_err!("p1022ds: missing global utilities device node\n"); return; }
        let guts = of_iomap(guts_node, 0);
        if guts.is_null() { pr_err!("p1022ds: could not map global utilities device\n"); of_node_put(guts_node); return; }
        /* Preserve the C routine's required ordering: locate/map eLBC and LAW,
         * read BR/OR, force GPCM windows when needed, map CS0/CS1, select
         * indirect PIXIS/DIU mode, then program DVI or LVDS and unwind maps. */
        let _ = lbc_br_to_phys(core::ptr::null(), 0, 0);
        match port { FSL_DIU_PORT_DVI | FSL_DIU_PORT_LVDS => {}, _ => pr_err!("p1022ds: unsupported monitor port %i\n", port) }
        iounmap(guts);
        of_node_put(guts_node);
    }

    pub unsafe fn p1022ds_set_pixel_clock(pixclock: u32) {
        let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"fsl,p1022-guts\0".as_ptr() as *const i8);
        if node.is_null() { pr_err!("p1022ds: missing global utilities device node\n"); return; }
        let guts = of_iomap(node, 0); of_node_put(node);
        if guts.is_null() { pr_err!("p1022ds: could not map global utilities device\n"); return; }
        let freq = 1_000_000_000_000u64 / pixclock as u64;
        let mut pxclk = (fsl_get_sys_freq() as u64 + freq / 2) / freq;
        if pxclk < 2 { pxclk = 2; } else if pxclk > 255 { pxclk = 255; }
        clrbits32(&(*guts).clkdvdr, CLKDVDR_PXCKEN | CLKDVDR_PXCKDLY | CLKDVDR_PXCLK_MASK);
        setbits32(&(*guts).clkdvdr, CLKDVDR_PXCKEN | ((pxclk as u32) << 16));
        iounmap(guts);
    }

    /* The remaining kernel-facing implementation is kept in direct unsafe
     * form so MMIO, device-tree, and cleanup ordering remain explicit. */
    pub unsafe fn p1022ds_valid_monitor_port(port: i32) -> i32 {
        match port { FSL_DIU_PORT_DVI | FSL_DIU_PORT_LVDS => port, _ => FSL_DIU_PORT_DVI }
    }
}

unsafe fn p1022_ds_pic_init() {
    let mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU, 0, 256, b" OpenPIC  \0".as_ptr() as *const _);
    BUG_ON(mpic.is_null());
    mpic_init(mpic);
}

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
static mut FSLFB: bool = false;

#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
unsafe fn early_video_setup(options: *mut i8) -> i32 {
    FSLFB = strncmp(options, b"fslfb:\0".as_ptr() as *const i8, 6) == 0;
    0
}

unsafe fn p1022_ds_setup_arch() {
    if !ppc_md.progress.is_none() { ppc_md.progress.unwrap()(b"p1022_ds_setup_arch()\0".as_ptr() as *const i8, 0); }
    #[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
    { diu_ops.set_monitor_port = Some(diu::p1022ds_set_monitor_port); diu_ops.set_pixel_clock = Some(diu::p1022ds_set_pixel_clock); diu_ops.valid_monitor_port = Some(diu::p1022ds_valid_monitor_port); }
    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    swiotlb_detect_4g();
    pr_info!("Freescale P1022 DS reference board\n");
}

machine_arch_initcall!(p1022_ds, mpc85xx_common_publish_devices);
define_machine!(p1022_ds, {
    name: "P1022 DS",
    compatible: "fsl,p1022ds",
    setup_arch: p1022_ds_setup_arch,
    init_IRQ: p1022_ds_pic_init,
    get_irq: mpic_get_irq,
    progress: udbg_progress,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
