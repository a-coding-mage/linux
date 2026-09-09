/*
 * EIM driver for Freescale's i.MX chips
 *
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Kernel dependencies supplied by the surrounding repository.

#[repr(C)]
struct imx_weim_devtype {
    cs_count: c_uint,
    cs_regs_count: c_uint,
    cs_stride: c_uint,
    wcr_offset: c_uint,
    wcr_bcm: c_uint,
    wcr_cont_bclk: c_uint,
}

static imx1_weim_devtype: imx_weim_devtype = imx_weim_devtype {
    cs_count: 6,
    cs_regs_count: 2,
    cs_stride: 0x08,
    wcr_offset: 0,
    wcr_bcm: 0,
    wcr_cont_bclk: 0,
};

static imx27_weim_devtype: imx_weim_devtype = imx_weim_devtype {
    cs_count: 6,
    cs_regs_count: 3,
    cs_stride: 0x10,
    wcr_offset: 0,
    wcr_bcm: 0,
    wcr_cont_bclk: 0,
};

static imx50_weim_devtype: imx_weim_devtype = imx_weim_devtype {
    cs_count: 4,
    cs_regs_count: 6,
    cs_stride: 0x18,
    wcr_offset: 0x90,
    wcr_bcm: 1 << 0,
    wcr_cont_bclk: 1 << 3,
};

static imx51_weim_devtype: imx_weim_devtype = imx_weim_devtype {
    cs_count: 6,
    cs_regs_count: 6,
    cs_stride: 0x18,
    wcr_offset: 0,
    wcr_bcm: 0,
    wcr_cont_bclk: 0,
};

const MAX_CS_REGS_COUNT: usize = 6;
const MAX_CS_COUNT: usize = 6;
const OF_REG_SIZE: usize = 3;

#[repr(C)]
struct cs_timing {
    is_applied: bool,
    regs: [u32; MAX_CS_REGS_COUNT],
}

#[repr(C)]
struct cs_timing_state {
    cs: [cs_timing; MAX_CS_COUNT],
}

#[repr(C)]
struct weim_priv {
    base: *mut core::ffi::c_void,
    timing_state: cs_timing_state,
}

static weim_id_table: [of_device_id; 6] = [
    of_device_id { compatible: "fsl,imx1-weim", data: &imx1_weim_devtype as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx27-weim", data: &imx27_weim_devtype as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx50-weim", data: &imx50_weim_devtype as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx6q-weim", data: &imx50_weim_devtype as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx51-weim", data: &imx51_weim_devtype as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn imx_weim_gpr_setup(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut parser: of_range_parser = core::mem::zeroed();
    let mut range: of_range = core::mem::zeroed();
    let gpr = syscon_regmap_lookup_by_phandle(np, b"fsl,weim-cs-gpr\0".as_ptr() as *const _);
    if is_err(gpr) {
        dev_dbg(&(*pdev).dev, "failed to find weim-cs-gpr\0");
        return 0;
    }
    let gprvals: [u32; 4] = [0o5, 0o33, 0o113, 0o1111];
    let mut gprval: u32 = 0;
    let mut count = 0;
    if of_range_parser_init(&mut parser, np) != 0 { return -EINVAL; }
    while for_each_of_range(&mut parser, &mut range) != 0 {
        let cs = range.bus_addr >> 32;
        let val = (range.size / SZ_32M) | 1;
        gprval |= val << (cs * 3);
        count += 1;
    }
    if count == 0 { return -EINVAL; }
    for value in gprvals {
        if gprval == value {
            regmap_update_bits(gpr, IOMUXC_GPR1, 0xfff, gprval);
            return 0;
        }
    }
    dev_err(&(*pdev).dev, "Invalid 'ranges' configuration\0");
    -EINVAL
}

unsafe fn weim_timing_setup(dev: *mut device, np: *mut device_node,
                            devtype: *const imx_weim_devtype) -> c_int {
    if (*devtype).cs_regs_count as usize > MAX_CS_REGS_COUNT || (*devtype).cs_count as usize > MAX_CS_COUNT { return -EINVAL; }
    let priv_ = dev_get_drvdata(dev) as *mut weim_priv;
    let mut value = [0u32; MAX_CS_REGS_COUNT];
    let ret = of_property_read_u32_array(np, b"fsl,weim-cs-timing\0".as_ptr() as *const _, value.as_mut_ptr(), (*devtype).cs_regs_count);
    if ret != 0 { return ret; }
    let num_regs = of_property_count_elems_of_size(np, b"reg\0".as_ptr() as *const _, OF_REG_SIZE);
    if num_regs <= 0 { return if num_regs < 0 { num_regs } else { -EINVAL }; }
    for reg_idx in 0..num_regs as usize {
        let mut cs_idx = 0u32;
        let ret = of_property_read_u32_index(np, b"reg\0".as_ptr() as *const _, (reg_idx * OF_REG_SIZE) as c_uint, &mut cs_idx);
        if ret != 0 { break; }
        if cs_idx >= (*devtype).cs_count { return -EINVAL; }
        let cst = &mut (*priv_).timing_state.cs[cs_idx as usize];
        if cst.is_applied && cst.regs[..(*devtype).cs_regs_count as usize] != value[..(*devtype).cs_regs_count as usize] { return -EINVAL; }
        for i in 0..(*devtype).cs_regs_count as usize {
            writel(value[i], ((*priv_).base as *mut u8).add(cs_idx as usize * (*devtype).cs_stride as usize + i * 4) as *mut _);
        }
        if !cst.is_applied { cst.is_applied = true; cst.regs[..(*devtype).cs_regs_count as usize].copy_from_slice(&value[..(*devtype).cs_regs_count as usize]); }
    }
    0
}

unsafe fn weim_parse_dt(pdev: *mut platform_device) -> c_int {
    let devtype = device_get_match_data(&mut (*pdev).dev) as *const imx_weim_devtype;
    let priv_ = dev_get_drvdata(&mut (*pdev).dev) as *mut weim_priv;
    if devtype == &imx50_weim_devtype { let ret = imx_weim_gpr_setup(pdev); if ret != 0 { return ret; } }
    if of_property_read_bool((*pdev).dev.of_node, b"fsl,burst-clk-enable\0".as_ptr() as *const _) {
        if (*devtype).wcr_bcm == 0 { return -EINVAL; }
        let addr = ((*priv_).base as *mut u8).add((*devtype).wcr_offset as usize) as *mut _;
        let mut reg = readl(addr) | (*devtype).wcr_bcm;
        if of_property_read_bool((*pdev).dev.of_node, b"fsl,continuous-burst-clk\0".as_ptr() as *const _) {
            if (*devtype).wcr_cont_bclk == 0 { return -EINVAL; }
            reg |= (*devtype).wcr_cont_bclk;
        }
        writel(reg, addr);
    }
    let mut have_child = false;
    let mut child = (*pdev).dev.of_node;
    while for_each_available_child_of_node((*pdev).dev.of_node, &mut child) != 0 {
        if weim_timing_setup(&mut (*pdev).dev, child, devtype) == 0 { have_child = true; }
    }
    if have_child { of_platform_default_populate((*pdev).dev.of_node, core::ptr::null_mut(), &mut (*pdev).dev) } else { 0 }
}

unsafe fn weim_probe(pdev: *mut platform_device) -> c_int {
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<weim_priv>(), GFP_KERNEL) as *mut weim_priv;
    if priv_.is_null() { return -ENOMEM; }
    let base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) { return ptr_err(base); }
    (*priv_).base = base;
    dev_set_drvdata(&mut (*pdev).dev, priv_ as *mut _);
    let clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if is_err(clk) { return ptr_err(clk); }
    weim_parse_dt(pdev)
}

static mut weim_driver: platform_driver = platform_driver { driver: driver { name: "imx-weim", of_match_table: weim_id_table.as_ptr() }, probe: Some(weim_probe) };

#if IS_ENABLED(CONFIG_OF_DYNAMIC)
unsafe fn of_weim_notify(_nb: *mut notifier_block, action: c_ulong, arg: *mut core::ffi::c_void) -> c_int {
    let rd = arg as *mut of_reconfig_data;
    let mut ret = NOTIFY_OK;
    match of_reconfig_get_state_change(action, rd) {
        OF_RECONFIG_CHANGE_ADD => {
            let of_id = of_match_node(weim_id_table.as_ptr(), (*rd).dn.parent);
            if of_id.is_null() { return NOTIFY_OK; }
            let devtype = (*of_id).data as *const imx_weim_devtype;
            let pdev = of_find_device_by_node((*rd).dn.parent);
            if pdev.is_null() { return notifier_from_errno(-EINVAL); }
            if weim_timing_setup(&mut (*pdev).dev, (*rd).dn, devtype) != 0 { dev_warn(&(*pdev).dev, "Failed to setup timing for '%pOF'\0"); }
            if !of_node_check_flag((*rd).dn, OF_POPULATED) && of_platform_device_create((*rd).dn, core::ptr::null_mut(), &mut (*pdev).dev).is_null() { ret = notifier_from_errno(-EINVAL); }
            platform_device_put(pdev);
        }
        OF_RECONFIG_CHANGE_REMOVE => {
            if !of_node_check_flag((*rd).dn, OF_POPULATED) { return NOTIFY_OK; }
            if of_match_node(weim_id_table.as_ptr(), (*rd).dn.parent).is_null() { return NOTIFY_OK; }
            let pdev = of_find_device_by_node((*rd).dn);
            if pdev.is_null() { ret = notifier_from_errno(-EINVAL); } else { of_platform_device_destroy(&mut (*pdev).dev, core::ptr::null_mut()); platform_device_put(pdev); }
        }
        _ => {}
    }
    ret
}

static mut weim_of_notifier: notifier_block = notifier_block { notifier_call: Some(of_weim_notify) };
#endif

unsafe fn weim_init() -> c_int { platform_driver_register(&mut weim_driver) }
unsafe fn weim_exit() { platform_driver_unregister(&mut weim_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
