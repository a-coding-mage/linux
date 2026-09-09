// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARC HSDK Platform support code
 *
 * Copyright (C) 2017 Synopsys, Inc. (www.synopsys.com)
 */

use core::ffi::{c_char, c_void};

extern "C" {
    static mut initial_boot_params: *mut c_void;
    fn iowrite32(value: u32, address: *mut c_void);
    fn writel(value: u32, address: *mut c_void);
    fn readl(address: *const c_void) -> u32;
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> i32;
    fn fdt_getprop(
        fdt: *const c_void,
        node: i32,
        name: *const c_char,
        lenp: *mut i32,
    ) -> *const c_void;
    fn fdt_delprop(fdt: *mut c_void, node: i32, name: *const c_char) -> i32;
    fn fdt_setprop(
        fdt: *mut c_void,
        node: i32,
        name: *const c_char,
        value: *const c_void,
        len: i32,
    ) -> i32;
    fn pr_err(format: *const c_char, ...);
}

pub static mut arc_hsdk_axi_dmac_coherent: i32 = 0;

const ARC_CCM_UNUSED_ADDR: usize = 0x60000000;
const ARC_PERIPHERAL_BASE: usize = 0xf0000000;
const CREG_BASE: usize = ARC_PERIPHERAL_BASE + 0x1000;
const SDIO_BASE: usize = ARC_PERIPHERAL_BASE + 0xA000;
const SDIO_UHS_REG_EXT: usize = SDIO_BASE + 0x108;
const SDIO_UHS_REG_EXT_DIV_2: u32 = 2 << 30;
const HSDK_GPIO_INTC: usize = ARC_PERIPHERAL_BASE + 0x3000;

unsafe fn hsdk_enable_gpio_intc_wire() {
    /*
     * Peripherals on CPU Card are wired to cpu intc via intermediate
     * DW APB GPIO blocks (mainly for debouncing)
     *
     *         ---------------------
     *        |  snps,archs-intc  |
     *        ---------------------
     *                  |
     *        ----------------------
     *        | snps,archs-idu-intc |
     *        ----------------------
     *         |   |     |   |    |
     *         | [eth] [USB]    [... other peripherals]
     *         |
     * -------------------
     * | snps,dw-apb-intc |
     * -------------------
     *  |      |   |   |
     * [Bt] [HAPS]   [... other peripherals]
     *
     * Current implementation of "irq-dw-apb-ictl" driver doesn't work well
     * with stacked INTCs. In particular problem happens if its master INTC
     * not yet instantiated. See discussion here -
     * https://lore.kernel.org/lkml/54F6FE2C.7020309@synopsys.com
     *
     * So setup the first gpio block as a passive pass thru and hide it from
     * DT hardware topology - connect intc directly to cpu intc
     * The GPIO "wire" needs to be init nevertheless (here)
     *
     * One side adv is that peripheral interrupt handling avoids one nested
     * intc ISR hop
     *
     * According to HSDK User's Manual [1], "Table 2 Interrupt Mapping"
     * we have the following GPIO input lines used as sources of interrupt:
     * - GPIO[0] - Bluetooth interrupt of RS9113 module
     * - GPIO[2] - HAPS interrupt (on HapsTrak 3 connector)
     * - GPIO[3] - Audio codec (MAX9880A) interrupt
     * - GPIO[8-23] - Available on Arduino and PMOD_x headers
     * For now there's no use of Arduino and PMOD_x headers in Linux
     * use-case so we only enable lines 0, 2 and 3.
     *
     * [1] https://github.com/foss-for-synopsys-dwc-arc-processors/ARC-Development-Systems-Forum/wiki/docs/ARC_HSDK_User_Guide.pdf
     */
    const GPIO_INTMASK: usize = HSDK_GPIO_INTC + 0x34;
    const GPIO_INTTYPE_LEVEL: usize = HSDK_GPIO_INTC + 0x38;
    const GPIO_INT_POLARITY: usize = HSDK_GPIO_INTC + 0x3c;
    const GPIO_INTEN: usize = HSDK_GPIO_INTC + 0x30;
    const GPIO_INT_CONNECTED_MASK: u32 = 0x0d;

    iowrite32(0xffffffff, GPIO_INTMASK as *mut c_void);
    iowrite32(!GPIO_INT_CONNECTED_MASK, GPIO_INTMASK as *mut c_void);
    iowrite32(0x00000000, GPIO_INTTYPE_LEVEL as *mut c_void);
    iowrite32(0xffffffff, GPIO_INT_POLARITY as *mut c_void);
    iowrite32(GPIO_INT_CONNECTED_MASK, GPIO_INTEN as *mut c_void);
}

unsafe fn hsdk_tweak_node_coherency(path: *const c_char, coherent: bool) -> i32 {
    let fdt = initial_boot_params;
    let mut ret: i32;
    let node = fdt_path_offset(fdt, path);
    if node < 0 {
        return tweak_fail(path, coherent);
    }

    let prop = fdt_getprop(fdt, node, b"dma-coherent\0".as_ptr() as *const c_char, &mut ret);
    if prop.is_null() && ret != -FDT_ERR_NOTFOUND {
        return tweak_fail(path, coherent);
    }

    let dt_coh_set = ret != -FDT_ERR_NOTFOUND;
    ret = 0;
    if dt_coh_set && !coherent {
        ret = fdt_delprop(fdt, node, b"dma-coherent\0".as_ptr() as *const c_char);
    }
    if !dt_coh_set && coherent {
        ret = fdt_setprop(
            fdt,
            node,
            b"dma-coherent\0".as_ptr() as *const c_char,
            core::ptr::null(),
            0,
        );
    }
    if ret < 0 {
        return tweak_fail(path, coherent);
    }
    0
}

unsafe fn tweak_fail(path: *const c_char, coherent: bool) -> i32 {
    pr_err(
        b"failed to tweak %s to %scoherent\n\0".as_ptr() as *const c_char,
        path,
        if coherent { b"\0".as_ptr() } else { b"non\0".as_ptr() },
    );
    -EFAULT
}

const FDT_ERR_NOTFOUND: i32 = 1;
const EFAULT: i32 = 14;

#[repr(i32)]
enum HsdkAxiMasters {
    M_HS_CORE = 0,
    M_HS_RTT,
    M_AXI_TUN,
    M_HDMI_VIDEO,
    M_HDMI_AUDIO,
    M_USB_HOST,
    M_ETHERNET,
    M_SDIO,
    M_GPU,
    M_DMAC_0,
    M_DMAC_1,
    M_DVFS,
}

const UPDATE_VAL: u32 = 1;

#[inline]
fn axi(base: usize, m: HsdkAxiMasters, offset: usize) -> *mut c_void {
    (base + 0x20 * (m as usize) + offset) as *mut c_void
}

unsafe fn hsdk_init_memory_bridge_axi_dmac() {
    let coherent = arc_hsdk_axi_dmac_coherent != 0;
    let (axi_m_slv1, axi_m_oft1) = if coherent {
        (0x77999999, 0x76DCBA98)
    } else {
        (0x77777777, 0x76543210)
    };
    if hsdk_tweak_node_coherency(b"/soc/dmac@80000\0".as_ptr() as *const c_char, coherent) != 0 {
        return;
    }
    for m in [HsdkAxiMasters::M_DMAC_0, HsdkAxiMasters::M_DMAC_1] {
        writel(0x77777777, axi(CREG_BASE, m, 0));
        writel(0xFEDCBA98, axi(CREG_BASE, m, 8));
        writel(axi_m_slv1, axi(CREG_BASE, m, 4));
        writel(axi_m_oft1, axi(CREG_BASE, m, 12));
        writel(UPDATE_VAL, axi(CREG_BASE, m, 20));
    }
}

unsafe fn hsdk_init_memory_bridge() {
    let boot = (CREG_BASE + 0x010) as *const c_void;
    let reg = readl(boot) & !0x3;
    writel(reg, boot as *mut c_void);

    let configs: &[(HsdkAxiMasters, u32, u32, u32, u32)] = &[
        (HsdkAxiMasters::M_HS_CORE, 0x11111111, 0x63111111, 0xFEDCBA98, 0x0E543210),
        (HsdkAxiMasters::M_HS_RTT, 0x77777777, 0x77777777, 0xFEDCBA98, 0x76543210),
        (HsdkAxiMasters::M_AXI_TUN, 0x88888888, 0x88888888, 0xFEDCBA98, 0x76543210),
        (HsdkAxiMasters::M_HDMI_VIDEO, 0x77777777, 0x77777777, 0xFEDCBA98, 0x76543210),
        (HsdkAxiMasters::M_HDMI_AUDIO, 0x77777777, 0x77777777, 0xFEDCBA98, 0x76543210),
        (HsdkAxiMasters::M_USB_HOST, 0x77777777, 0x77999999, 0xFEDCBA98, 0x76DCBA98),
        (HsdkAxiMasters::M_ETHERNET, 0x77777777, 0x77999999, 0xFEDCBA98, 0x76DCBA98),
        (HsdkAxiMasters::M_SDIO, 0x77777777, 0x77999999, 0xFEDCBA98, 0x76DCBA98),
        (HsdkAxiMasters::M_GPU, 0x77777777, 0x77777777, 0xFEDCBA98, 0x76543210),
        (HsdkAxiMasters::M_DVFS, 0x00000000, 0x60000000, 0x00000000, 0x00000000),
    ];
    for &(m, slv0, slv1, oft0, oft1) in configs {
        writel(slv0, axi(CREG_BASE, m, 0));
        writel(slv1, axi(CREG_BASE, m, 4));
        writel(oft0, axi(CREG_BASE, m, 8));
        writel(oft1, axi(CREG_BASE, m, 12));
        writel(UPDATE_VAL, axi(CREG_BASE, m, 20));
    }
    hsdk_init_memory_bridge_axi_dmac();
    writel(0x00000000, (CREG_BASE + 0x180) as *mut c_void);
    writel(UPDATE_VAL, (CREG_BASE + 0x194) as *mut c_void);
}

unsafe fn hsdk_init_early() {
    hsdk_init_memory_bridge();
    iowrite32(SDIO_UHS_REG_EXT_DIV_2, SDIO_UHS_REG_EXT as *mut c_void);
    hsdk_enable_gpio_intc_wire();
}

static HSDK_COMPAT: &[&[u8]] = &[b"snps,hsdk\0", b"\0"];

// MACHINE_START(SIMULATION, "hsdk")
//     .dt_compat = hsdk_compat,
//     .init_early = hsdk_init_early,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
