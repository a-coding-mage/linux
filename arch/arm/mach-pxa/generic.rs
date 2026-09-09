// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/generic.c
 *
 *  Code common to all PXA machines.
 *
 *  Since this file should be linked before any other machine specific file,
 *  the __initcall() here will be executed first.  This serves as default
 *  initialization stuff for PXA machines which can be overridden later if
 *  need be.
 */

// External declarations supplied by the surrounding kernel translation.
extern "C" {
    fn cpu_is_pxa2xx() -> bool;
    fn cpu_is_pxa25x() -> bool;
    fn cpu_is_pxa27x() -> bool;
    fn cpu_is_pxa3xx() -> bool;
    fn pxa2xx_clear_reset_status(mask: u32);
    fn pxa25x_clocks_init(base: *mut core::ffi::c_void);
    fn pxa27x_clocks_init(base: *mut core::ffi::c_void);
    fn pxa3xx_clocks_init(
        base1: *mut core::ffi::c_void,
        base2: *mut core::ffi::c_void,
    );
    fn pxa_timer_nodt_init(irq: u32, base: *mut core::ffi::c_void);
    fn debug_ll_io_init();
    fn iotable_init(desc: *const map_desc, size: usize);
    fn io_p2v(addr: usize) -> *mut core::ffi::c_void;
    fn __raw_writel(value: u32, address: *mut u32);
    static mut ARSR: u32;
    static mut MECR: u32;
    static mut MDREFR: *mut core::ffi::c_void;
}

#[repr(C)]
struct map_desc {
    virtual_: usize,
    pfn: usize,
    length: usize,
    type_: u32,
}

// Build-time constants and register macros are supplied by the PXA headers.
extern "C" {
    fn MCMEM(sock: i32) -> *mut u32;
    fn MCATT(sock: i32) -> *mut u32;
    fn MCIO(sock: i32) -> *mut u32;
}

const MECR_CIT: u32 = 1 << 0;
const MECR_NOS: u32 = 1 << 1;

pub unsafe fn clear_reset_status(mask: u32) {
    if cpu_is_pxa2xx() {
        pxa2xx_clear_reset_status(mask);
    } else {
        /* RESET_STATUS_* has a 1:1 mapping with ARSR */
        ARSR = mask;
    }
}

/*
 * For non device-tree builds, keep legacy timer init
 */
pub unsafe fn pxa_timer_init() {
    if cpu_is_pxa25x() {
        pxa25x_clocks_init(io_p2v(0x41300000));
    }
    if cpu_is_pxa27x() {
        pxa27x_clocks_init(io_p2v(0x41300000));
    }
    if cpu_is_pxa3xx() {
        pxa3xx_clocks_init(io_p2v(0x41340000), io_p2v(0x41350000));
    }
    pxa_timer_nodt_init(IRQ_OST0, io_p2v(0x40a00000));
}

pub unsafe fn pxa_smemc_set_pcmcia_timing(
    sock: i32,
    mcmem: u32,
    mcatt: u32,
    mcio: u32,
) {
    __raw_writel(mcmem, MCMEM(sock));
    __raw_writel(mcatt, MCATT(sock));
    __raw_writel(mcio, MCIO(sock));
}

pub unsafe fn pxa_smemc_set_pcmcia_socket(nr: i32) {
    match nr {
        0 => {
            __raw_writel(0, &raw mut MECR);
        }
        1 => {
            /*
             * We have at least one socket, so set MECR:CIT
             * (Card Is There)
             */
            __raw_writel(MECR_CIT, &raw mut MECR);
        }
        2 => {
            /* Set CIT and MECR:NOS (Number Of Sockets) */
            __raw_writel(MECR_CIT | MECR_NOS, &raw mut MECR);
        }
        _ => {}
    }
}

pub unsafe fn pxa_smemc_get_mdrefr() -> *mut core::ffi::c_void {
    MDREFR
}

/*
 * Intel PXA2xx internal register mapping.
 *
 * Note: virtual 0xfffe0000-0xffffffff is reserved for the vector table
 *       and cache flush area.
 */
static mut common_io_desc: [map_desc; 1] = [map_desc {
    virtual_: PERIPH_VIRT,
    pfn: __phys_to_pfn(PERIPH_PHYS),
    length: PERIPH_SIZE,
    type_: MT_DEVICE,
}];

pub unsafe fn pxa_map_io() {
    debug_ll_io_init();
    iotable_init(common_io_desc.as_ptr(), common_io_desc.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
