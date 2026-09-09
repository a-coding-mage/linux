// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2plus display device setup / initialization.
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com/
 *	Senthilvadivu Guruswamy
 *	Sumit Semwal
 */

const DISPC_CONTROL: u32 = 0x0040;
const DISPC_CONTROL2: u32 = 0x0238;
const DISPC_CONTROL3: u32 = 0x0848;
const DISPC_IRQSTATUS: u32 = 0x0018;

const DSS_CONTROL: u32 = 0x40;
const DSS_SDI_CONTROL: u32 = 0x44;
const DSS_PLL_CONTROL: u32 = 0x48;

const LCD_EN_MASK: u32 = 0x1 << 0;
const DIGIT_EN_MASK: u32 = 0x1 << 1;

const FRAMEDONE_IRQ_SHIFT: u32 = 0;
const EVSYNC_EVEN_IRQ_SHIFT: u32 = 2;
const EVSYNC_ODD_IRQ_SHIFT: u32 = 3;
const FRAMEDONE2_IRQ_SHIFT: u32 = 22;
const FRAMEDONE3_IRQ_SHIFT: u32 = 30;
const FRAMEDONETV_IRQ_SHIFT: u32 = 24;
const FRAMEDONE_IRQ_TIMEOUT: i32 = 100;

// Build-time CONFIG_FB_OMAP2 condition from the C source.
#[cfg(CONFIG_FB_OMAP2)]
mod fb_omap2 {
    use super::*;

    const OMAP4_DSIPHY_SYSCON_OFFSET: u32 = 0x78;

    static mut omap_display_device: platform_device = platform_device {
        name: core::ptr::null(),
        id: -1,
        dev: device { platform_data: core::ptr::null_mut() },
    };

    static mut omap4_dsi_mux_syscon: *mut regmap = core::ptr::null_mut();

    unsafe fn omap4_dsi_mux_pads(dsi_id: i32, lanes: u32) -> i32 {
        let (enable_mask, enable_shift, pipd_mask, pipd_shift): (u32, u32, u32, u32);
        if dsi_id == 0 {
            enable_mask = OMAP4_DSI1_LANEENABLE_MASK;
            enable_shift = OMAP4_DSI1_LANEENABLE_SHIFT;
            pipd_mask = OMAP4_DSI1_PIPD_MASK;
            pipd_shift = OMAP4_DSI1_PIPD_SHIFT;
        } else if dsi_id == 1 {
            enable_mask = OMAP4_DSI2_LANEENABLE_MASK;
            enable_shift = OMAP4_DSI2_LANEENABLE_SHIFT;
            pipd_mask = OMAP4_DSI2_PIPD_MASK;
            pipd_shift = OMAP4_DSI2_PIPD_SHIFT;
        } else {
            return -ENODEV;
        }

        let mut reg: u32 = 0;
        let ret = regmap_read(omap4_dsi_mux_syscon, OMAP4_DSIPHY_SYSCON_OFFSET, &mut reg);
        if ret != 0 { return ret; }
        reg &= !enable_mask;
        reg &= !pipd_mask;
        reg |= (lanes << enable_shift) & enable_mask;
        reg |= (lanes << pipd_shift) & pipd_mask;
        regmap_write(omap4_dsi_mux_syscon, OMAP4_DSIPHY_SYSCON_OFFSET, reg);
        0
    }

    unsafe fn omap_dsi_enable_pads(dsi_id: i32, lane_mask: u32) -> i32 {
        if cpu_is_omap44xx() { omap4_dsi_mux_pads(dsi_id, lane_mask) } else { 0 }
    }

    unsafe fn omap_dsi_disable_pads(dsi_id: i32, _lane_mask: u32) {
        if cpu_is_omap44xx() { omap4_dsi_mux_pads(dsi_id, 0); }
    }

    unsafe fn omap_display_get_version() -> omapdss_version {
        if cpu_is_omap24xx() { OMAPDSS_VER_OMAP24xx }
        else if cpu_is_omap3630() { OMAPDSS_VER_OMAP3630 }
        else if cpu_is_omap34xx() {
            if soc_is_am35xx() { OMAPDSS_VER_AM35xx }
            else if omap_rev() < OMAP3430_REV_ES3_0 { OMAPDSS_VER_OMAP34xx_ES1 }
            else { OMAPDSS_VER_OMAP34xx_ES3 }
        } else if omap_rev() == OMAP4430_REV_ES1_0 { OMAPDSS_VER_OMAP4430_ES1 }
        else if omap_rev() == OMAP4430_REV_ES2_0 || omap_rev() == OMAP4430_REV_ES2_1 || omap_rev() == OMAP4430_REV_ES2_2 { OMAPDSS_VER_OMAP4430_ES2 }
        else if cpu_is_omap44xx() { OMAPDSS_VER_OMAP4 }
        else if soc_is_omap54xx() { OMAPDSS_VER_OMAP5 }
        else if soc_is_am43xx() { OMAPDSS_VER_AM43xx }
        else if soc_is_dra7xx() { OMAPDSS_VER_DRA7xx }
        else { OMAPDSS_VER_UNKNOWN }
    }

    unsafe fn omapdss_init_fbdev() -> i32 {
        static mut board_data: omap_dss_board_info = omap_dss_board_info { version: OMAPDSS_VER_UNKNOWN, dsi_enable_pads: Some(omap_dsi_enable_pads), dsi_disable_pads: Some(omap_dsi_disable_pads) };
        board_data.version = omap_display_get_version();
        if board_data.version == OMAPDSS_VER_UNKNOWN { pr_err("DSS not supported on this SoC\n"); return -ENODEV; }
        omap_display_device.dev.platform_data = &mut board_data as *mut _ as *mut core::ffi::c_void;
        let mut r = platform_device_register(&mut omap_display_device);
        if r < 0 { pr_err("Unable to register omapdss device\n"); return r; }
        r = omap_init_vrfb(); if r < 0 { pr_err("Unable to register omapvrfb device\n"); return r; }
        r = omap_init_fb(); if r < 0 { pr_err("Unable to register omapfb device\n"); return r; }
        r = omap_init_vout(); if r < 0 { pr_err("Unable to register omap_vout device\n"); return r; }
        let node = of_find_node_by_name(core::ptr::null_mut(), b"omap4_padconf_global\0".as_ptr() as *const i8);
        if !node.is_null() { omap4_dsi_mux_syscon = syscon_node_to_regmap(node); }
        of_node_put(node);
        0
    }

    static omapdss_compat_names: [&[u8]; 5] = [b"ti,omap2-dss\0", b"ti,omap3-dss\0", b"ti,omap4-dss\0", b"ti,omap5-dss\0", b"ti,dra7-dss\0"];

    unsafe fn omapdss_find_dss_of_node() -> *mut device_node {
        for name in omapdss_compat_names.iter() {
            let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), name.as_ptr() as *const i8);
            if !node.is_null() { return node; }
        }
        core::ptr::null_mut()
    }

    unsafe fn omapdss_init_of() -> i32 {
        let node = omapdss_find_dss_of_node();
        if node.is_null() { return 0; }
        if !of_device_is_available(node) { of_node_put(node); return 0; }
        let pdev = of_find_device_by_node(node);
        if pdev.is_null() { pr_err("Unable to find DSS platform device\n"); of_node_put(node); return -ENODEV; }
        let r = of_platform_populate(node, core::ptr::null(), core::ptr::null(), &mut (*pdev).dev);
        put_device(&mut (*pdev).dev); of_node_put(node);
        if r != 0 { pr_err("Unable to populate DSS submodule devices\n"); return r; }
        omapdss_init_fbdev()
    }
}

unsafe fn dispc_disable_outputs() {
    let oh = omap_hwmod_lookup(b"dss_dispc\0".as_ptr() as *const i8);
    if oh.is_null() { WARN(1, "display: could not disable outputs during reset - could not find dss_dispc hwmod\n"); return; }
    if (*oh).dev_attr.is_null() { pr_err("display: could not disable outputs during reset due to missing dev_attr\n"); return; }
    let da = (*oh).dev_attr as *mut omap_dss_dispc_dev_attr;
    let mut v = omap_hwmod_read(oh, DISPC_CONTROL);
    let lcd_en = v & LCD_EN_MASK != 0; let digit_en = v & DIGIT_EN_MASK != 0;
    let mut lcd2_en = false; let mut lcd3_en = false;
    if (*da).manager_count > 2 { lcd2_en = omap_hwmod_read(oh, DISPC_CONTROL2) & LCD_EN_MASK != 0; }
    if (*da).manager_count > 3 { lcd3_en = omap_hwmod_read(oh, DISPC_CONTROL3) & LCD_EN_MASK != 0; }
    if !(lcd_en || digit_en || lcd2_en || lcd3_en) { return; }
    let mut irq_mask = 0u32;
    if lcd_en { irq_mask |= 1 << FRAMEDONE_IRQ_SHIFT; }
    if digit_en { if (*da).has_framedonetv_irq { irq_mask |= 1 << FRAMEDONETV_IRQ_SHIFT; } else { irq_mask |= (1 << EVSYNC_EVEN_IRQ_SHIFT) | (1 << EVSYNC_ODD_IRQ_SHIFT); } }
    if lcd2_en { irq_mask |= 1 << FRAMEDONE2_IRQ_SHIFT; } if lcd3_en { irq_mask |= 1 << FRAMEDONE3_IRQ_SHIFT; }
    omap_hwmod_write(irq_mask, oh, DISPC_IRQSTATUS);
    v = omap_hwmod_read(oh, DISPC_CONTROL) & !(LCD_EN_MASK | DIGIT_EN_MASK); omap_hwmod_write(v, oh, DISPC_CONTROL);
    if (*da).manager_count > 2 { v = omap_hwmod_read(oh, DISPC_CONTROL2) & !LCD_EN_MASK; omap_hwmod_write(v, oh, DISPC_CONTROL2); }
    if (*da).manager_count > 3 { v = omap_hwmod_read(oh, DISPC_CONTROL3) & !LCD_EN_MASK; omap_hwmod_write(v, oh, DISPC_CONTROL3); }
    let mut i = 0; while (omap_hwmod_read(oh, DISPC_IRQSTATUS) & irq_mask) != irq_mask { i += 1; if i > FRAMEDONE_IRQ_TIMEOUT { pr_err("didn't get FRAMEDONE1/2/3 or TV interrupt\n"); break; } mdelay(1); }
}

pub unsafe fn omap_dss_reset(oh: *mut omap_hwmod) -> i32 {
    if (*(*oh).class).sysc.sysc_flags & SYSS_HAS_RESET_STATUS == 0 { pr_err("dss_core: hwmod data doesn't contain reset data\n"); return -EINVAL; }
    let mut c = 0; let mut i = (*oh).opt_clks_cnt; let mut oc = (*oh).opt_clks;
    while i > 0 { clk_prepare_enable((*oc)._clk); i -= 1; oc = oc.add(1); }
    dispc_disable_outputs();
    if cpu_is_omap3430() { omap_hwmod_write(0, oh, DSS_SDI_CONTROL); omap_hwmod_write(0, oh, DSS_PLL_CONTROL); }
    omap_hwmod_write(0, oh, DSS_CONTROL);
    omap_test_timeout(omap_hwmod_read(oh, (*(*oh).class).sysc.syss_offs) & SYSS_RESETDONE_MASK, MAX_MODULE_SOFTRESET_WAIT, &mut c);
    if c == MAX_MODULE_SOFTRESET_WAIT { pr_warn("dss_core: waiting for reset to finish failed\n"); } else { pr_debug("dss_core: softreset done\n"); }
    let mut i = (*oh).opt_clks_cnt; let mut oc = (*oh).opt_clks; while i > 0 { clk_disable_unprepare((*oc)._clk); i -= 1; oc = oc.add(1); }
    if c == MAX_MODULE_SOFTRESET_WAIT { -ETIMEDOUT } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
