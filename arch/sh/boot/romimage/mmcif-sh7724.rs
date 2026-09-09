/*
 * sh7724 MMCIF loader
 *
 * Copyright (C) 2010 Magnus Damm
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by <linux/platform_data/sh_mmcif.h> and
// <mach/romimage.h> are intentionally left as external symbols.

const MMCIF_BASE: *mut core::ffi::c_void = 0xa4ca0000 as *mut core::ffi::c_void;

const MSTPCR2: usize = 0xa4150038;
const PTWCR: usize = 0xa4050146;
const PTXCR: usize = 0xa4050148;
const PSELA: usize = 0xa405014e;
const PSELE: usize = 0xa4050156;
const HIZCRC: usize = 0xa405015c;
const DRVCRA: usize = 0xa405018a;

const MMCIF_PROGRESS_ENTER: i32 = 0;
const MMCIF_PROGRESS_INIT: i32 = 1;
const MMCIF_PROGRESS_LOAD: i32 = 2;
const MMCIF_PROGRESS_DONE: i32 = 3;

extern "C" {
    fn mmcif_update_progress(progress: i32);
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn sh_mmcif_boot_init(base: *mut core::ffi::c_void);
    fn sh_mmcif_boot_do_read(
        base: *mut core::ffi::c_void,
        block: u32,
        blocks: u32,
        buf: *mut u8,
    );
}

// SH_MMCIF_BBS is supplied by <linux/platform_data/sh_mmcif.h>.
extern "C" {
    static SH_MMCIF_BBS: u32;
}

/* SH7724 specific MMCIF loader
 *
 * loads the romImage from an MMC card starting from block 512
 * use the following line to write the romImage to an MMC card
 * # dd if=arch/sh/boot/romImage of=/dev/sdx bs=512 seek=512
 */
pub unsafe extern "C" fn mmcif_loader(buf: *mut u8, no_bytes: usize) {
    mmcif_update_progress(MMCIF_PROGRESS_ENTER);

    /* enable clock to the MMCIF hardware block */
    __raw_writel(__raw_readl(MSTPCR2) & !0x20000000, MSTPCR2);

    /* setup pins D7-D0 */
    __raw_writew(0x0000, PTWCR);

    /* setup pins MMC_CLK, MMC_CMD */
    __raw_writew(__raw_readw(PTXCR) & !0x000f, PTXCR);

    /* select D3-D0 pin function */
    __raw_writew(__raw_readw(PSELA) & !0x2000, PSELA);

    /* select D7-D4 pin function */
    __raw_writew(__raw_readw(PSELE) & !0x3000, PSELE);

    /* disable Hi-Z for the MMC pins */
    __raw_writew(__raw_readw(HIZCRC) & !0x0620, HIZCRC);

    /* high drive capability for MMC pins */
    __raw_writew(__raw_readw(DRVCRA) | 0x3000, DRVCRA);

    mmcif_update_progress(MMCIF_PROGRESS_INIT);

    /* setup MMCIF hardware */
    sh_mmcif_boot_init(MMCIF_BASE);

    mmcif_update_progress(MMCIF_PROGRESS_LOAD);

    /* load kernel via MMCIF interface */
    sh_mmcif_boot_do_read(
        MMCIF_BASE,
        512,
        (no_bytes.wrapping_add(SH_MMCIF_BBS as usize).wrapping_sub(1)
            / SH_MMCIF_BBS as usize) as u32,
        buf,
    );

    /* disable clock to the MMCIF hardware block */
    __raw_writel(__raw_readl(MSTPCR2) | 0x20000000, MSTPCR2);

    mmcif_update_progress(MMCIF_PROGRESS_DONE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
