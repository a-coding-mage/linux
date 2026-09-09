// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/io.c
 *
 * OMAP1 I/O mapping code
 */

// The machine specific code may provide the extra mapping besides the
// default mapping provided here.
// `__initdata` is a linker/build-time attribute in the C implementation.
static mut omap1_io_desc: [map_desc; 3] = [
    map_desc {
        r#virtual: OMAP1_IO_VIRT,
        pfn: __phys_to_pfn(OMAP1_IO_PHYS),
        length: OMAP1_IO_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        r#virtual: OMAP1_DSP_BASE,
        pfn: __phys_to_pfn(OMAP1_DSP_START),
        length: OMAP1_DSP_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        r#virtual: OMAP1_DSPREG_BASE,
        pfn: __phys_to_pfn(OMAP1_DSPREG_START),
        length: OMAP1_DSPREG_SIZE,
        type_: MT_DEVICE,
    },
];

extern "C" {
    fn iotable_init(io_desc: *mut map_desc, nr: usize);
    fn omap_check_revision();
    fn omap_serial_wakeup_init();
    fn __raw_readb(addr: *const core::ffi::c_void) -> u8;
    fn __raw_readw(addr: *const core::ffi::c_void) -> u16;
    fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
    fn __raw_writeb(v: u8, addr: *mut core::ffi::c_void);
    fn __raw_writew(v: u16, addr: *mut core::ffi::c_void);
    fn __raw_writel(v: u32, addr: *mut core::ffi::c_void);
}

// Maps common IO regions for omap1
pub unsafe extern "C" fn omap1_map_io() {
    iotable_init(omap1_io_desc.as_mut_ptr(), omap1_io_desc.len());
}

// Common low-level hardware init for omap1.
pub unsafe extern "C" fn omap1_init_early() {
    omap_check_revision();

    /* REVISIT: Refer to OMAP5910 Errata, Advisory SYS_1: "Timeout Abort
     * on a Posted Write in the TIPB Bridge".
     */
    omap_writew(0x0, MPU_PUBLIC_TIPB_CNTL);
    omap_writew(0x0, MPU_PRIVATE_TIPB_CNTL);
}

pub unsafe extern "C" fn omap1_init_late() {
    omap_serial_wakeup_init();
}

/*
 * NOTE: Please use ioremap + __raw_read/write where possible instead of these
 */

pub unsafe extern "C" fn omap_readb(pa: u32) -> u8 {
    __raw_readb(OMAP1_IO_ADDRESS(pa))
}

pub unsafe extern "C" fn omap_readw(pa: u32) -> u16 {
    __raw_readw(OMAP1_IO_ADDRESS(pa))
}

pub unsafe extern "C" fn omap_readl(pa: u32) -> u32 {
    __raw_readl(OMAP1_IO_ADDRESS(pa))
}

pub unsafe extern "C" fn omap_writeb(v: u8, pa: u32) {
    __raw_writeb(v, OMAP1_IO_ADDRESS(pa));
}

pub unsafe extern "C" fn omap_writew(v: u16, pa: u32) {
    __raw_writew(v, OMAP1_IO_ADDRESS(pa));
}

pub unsafe extern "C" fn omap_writel(v: u32, pa: u32) {
    __raw_writel(v, OMAP1_IO_ADDRESS(pa));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
