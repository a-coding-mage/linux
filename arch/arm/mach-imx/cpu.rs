// SPDX-License-Identifier: GPL-2.0
//
// C dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_char;

// Equivalent of IMX_CHIP_REVISION_UNKNOWN from the included headers.
// The value is supplied by the translated hardware/common dependencies.
extern "C" {
    static mut IMX_CHIP_REVISION_UNKNOWN: i32;
    fn imx_writel(value: u32, address: *mut u8);
    fn imx_readl(address: *mut u8) -> u32;
    fn pr_info(format: *const c_char, ...);
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn warn_on(condition: bool) -> bool;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub static mut __mxc_cpu_type: u32 = 0;
static mut imx_soc_revision: u32 = 0;

pub unsafe fn mxc_set_cpu_type(type_: u32) {
    __mxc_cpu_type = type_;
}

pub unsafe fn imx_set_soc_revision(rev: u32) {
    imx_soc_revision = rev;
}

pub unsafe fn imx_get_soc_revision() -> u32 {
    imx_soc_revision
}

pub unsafe fn imx_print_silicon_rev(cpu: *const c_char, srev: i32) {
    if srev == IMX_CHIP_REVISION_UNKNOWN {
        pr_info(b"CPU identified as %s, unknown revision\n\0".as_ptr() as *const c_char, cpu);
    } else {
        pr_info(
            b"CPU identified as %s, silicon rev %d.%d\n\0".as_ptr() as *const c_char,
            cpu,
            (srev >> 4) & 0xf,
            srev & 0xf,
        );
    }
}

pub unsafe fn imx_set_aips(base: *mut u8) {
    let mut reg: u32;
    /*
     * Set all MPROTx to be non-bufferable, trusted for R/W,
     * not forced to user-mode.
     */
    imx_writel(0x77777777, base.add(0x0));
    imx_writel(0x77777777, base.add(0x4));

    /*
     * Set all OPACRx to be non-bufferable, to not require
     * supervisor privilege level for access, allow for
     * write access and untrusted master access.
     */
    imx_writel(0x0, base.add(0x40));
    imx_writel(0x0, base.add(0x44));
    imx_writel(0x0, base.add(0x48));
    imx_writel(0x0, base.add(0x4c));
    reg = imx_readl(base.add(0x50)) & 0x00ffffff;
    imx_writel(reg, base.add(0x50));
}

pub unsafe fn imx_aips_allow_unprivileged_access(compat: *const c_char) {
    let mut aips_base_addr: *mut u8;
    let mut np: *mut device_node;

    // for_each_compatible_node(np, NULL, compat) is a kernel macro supplied
    // by the translated device-tree dependencies.
    for_each_compatible_node!(np, core::ptr::null_mut(), compat);
    {
        aips_base_addr = of_iomap(np, 0);
        warn_on(aips_base_addr.is_null());
        imx_set_aips(aips_base_addr);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
