// SPDX-License-Identifier: GPL-2.0-only
/* OMAP L3 Interconnect error handling driver */

// Kernel types, constants, helpers, and the declarations from omap_l3_noc.h
// are supplied by the surrounding kernel translation.

unsafe fn l3_handle_target(
    l3: *mut omap_l3,
    base: *mut core::ffi::c_void,
    flag_mux: *mut l3_flagmux_data,
    err_src: i32,
) -> i32 {
    let mut k: i32;
    let mut std_err_main: u32;
    let mut clear: u32;
    let mut masterid: u32;
    let op_code: u8;
    let m_req_info: u8;
    let l3_targ_base: *mut core::ffi::c_void;
    let l3_targ_stderr: *mut core::ffi::c_void;
    let l3_targ_slvofslsb: *mut core::ffi::c_void;
    let l3_targ_mstaddr: *mut core::ffi::c_void;
    let l3_targ_hdr: *mut core::ffi::c_void;
    let l3_targ_info: *mut core::ffi::c_void;
    let l3_targ_inst: *mut l3_target_data;
    let mut master: *mut l3_masters_data;
    let target_name: *const core::ffi::c_char;
    let mut master_name: *const core::ffi::c_char = c"UN IDENTIFIED".as_ptr();
    let err_description: *const core::ffi::c_char;
    let mut err_string = [0i8; 30];
    let mut info_string = [0i8; 60];

    BUG_ON(err_src > MAX_CLKDM_TARGETS);

    if err_src < (*flag_mux).num_targ_data {
        l3_targ_inst = (*flag_mux).l3_targ.add(err_src as usize);
        target_name = (*l3_targ_inst).name;
        l3_targ_base = (base as *mut u8).add((*l3_targ_inst).offset as usize) as *mut _;
    } else {
        target_name = L3_TARGET_NOT_SUPPORTED;
        l3_targ_base = core::ptr::null_mut();
    }

    if target_name == L3_TARGET_NOT_SUPPORTED { return -ENODEV; }

    l3_targ_stderr = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_MAIN as usize) as *mut _;
    l3_targ_slvofslsb = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_SLVOFSLSB as usize) as *mut _;
    std_err_main = readl_relaxed(l3_targ_stderr);

    match std_err_main & CUSTOM_ERROR {
        STANDARD_ERROR => {
            err_description = c"Standard".as_ptr();
            snprintf(err_string.as_mut_ptr(), err_string.len(), c": At Address: 0x%08X ".as_ptr(), readl_relaxed(l3_targ_slvofslsb));
            l3_targ_mstaddr = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_MSTADDR as usize) as *mut _;
            l3_targ_hdr = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_HDR as usize) as *mut _;
            l3_targ_info = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_INFO as usize) as *mut _;
        }
        CUSTOM_ERROR => {
            err_description = c"Custom".as_ptr();
            l3_targ_mstaddr = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_CINFO_MSTADDR as usize) as *mut _;
            l3_targ_hdr = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_CINFO_OPCODE as usize) as *mut _;
            l3_targ_info = (l3_targ_base as *mut u8).add(L3_TARG_STDERRLOG_CINFO_INFO as usize) as *mut _;
        }
        _ => return 0,
    }

    masterid = (readl_relaxed(l3_targ_mstaddr) & (*l3).mst_addr_mask) >> __ffs((*l3).mst_addr_mask);
    for k = 0, master = (*l3).l3_masters; k < (*l3).num_masters; k += 1 {
        if masterid == (*master).id { master_name = (*master).name; break; }
        master = master.add(1);
    }
    op_code = (readl_relaxed(l3_targ_hdr) & 0x7) as u8;
    m_req_info = (readl_relaxed(l3_targ_info) & 0xF) as u8;
    snprintf(info_string.as_mut_ptr(), info_string.len(), c": %s in %s mode during %s access".as_ptr(),
        if m_req_info & BIT(0) != 0 { c"Opcode Fetch".as_ptr() } else { c"Data Access".as_ptr() },
        if m_req_info & BIT(1) != 0 { c"Supervisor".as_ptr() } else { c"User".as_ptr() },
        if m_req_info & BIT(3) != 0 { c"Debug".as_ptr() } else { c"Functional".as_ptr() });
    WARN(true, c"%s:L3 %s Error: MASTER %s TARGET %s (%s)%s%s\n".as_ptr(), dev_name((*l3).dev), err_description,
        master_name, target_name, l3_transaction_type[op_code as usize], err_string.as_ptr(), info_string.as_ptr());
    clear = std_err_main | CLEAR_STDERR_LOG;
    writel_relaxed(clear, l3_targ_stderr);
    0
}

unsafe extern "C" fn l3_interrupt_handler(irq: i32, _l3: *mut core::ffi::c_void) -> irqreturn_t {
    let l3 = _l3 as *mut omap_l3;
    let inttype = if irq == (*l3).app_irq { L3_APPLICATION_ERROR } else { L3_DEBUG_ERROR };
    for i in 0..(*l3).num_modules {
        let base = (*l3).l3_base[i as usize];
        let flag_mux = (*l3).l3_flagmux[i as usize];
        let mut err_reg = readl_relaxed((base as *mut u8).add(((*flag_mux).offset + L3_FLAGMUX_REGERR0 + (inttype << 3)) as usize) as *mut _);
        err_reg &= !(if inttype != 0 { (*flag_mux).mask_app_bits } else { (*flag_mux).mask_dbg_bits });
        if err_reg != 0 {
            let err_src = __ffs(err_reg);
            let ret = l3_handle_target(l3, base, flag_mux, err_src as i32);
            if ret != 0 {
                dev_err((*l3).dev, c"L3 %s error: target %d mod:%d %s\n".as_ptr(), if inttype != 0 { c"debug".as_ptr() } else { c"application".as_ptr() }, err_src, i, c"(unclearable)".as_ptr());
                let mask_reg = (base as *mut u8).add(((*flag_mux).offset + L3_FLAGMUX_MASK0 + (inttype << 3)) as usize) as *mut _;
                let mut mask_val = readl_relaxed(mask_reg) & !(1u32 << err_src);
                writel_relaxed(mask_val, mask_reg);
                if inttype != 0 { (*flag_mux).mask_app_bits |= 1 << err_src; } else { (*flag_mux).mask_dbg_bits |= 1 << err_src; }
            }
            return IRQ_HANDLED;
        }
    }
    dev_err((*l3).dev, c"L3 %s IRQ not handled!!\n".as_ptr(), if inttype != 0 { c"debug".as_ptr() } else { c"application".as_ptr() });
    IRQ_NONE
}

static l3_noc_match: [of_device_id; 5] = [
    of_device_id { compatible: c"ti,omap4-l3-noc".as_ptr(), data: &omap4_l3_data as *const _ },
    of_device_id { compatible: c"ti,omap5-l3-noc".as_ptr(), data: &omap5_l3_data as *const _ },
    of_device_id { compatible: c"ti,dra7-l3-noc".as_ptr(), data: &dra_l3_data as *const _ },
    of_device_id { compatible: c"ti,am4372-l3-noc".as_ptr(), data: &am4372_l3_data as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn omap_l3_probe(pdev: *mut platform_device) -> i32 {
    let of_id = of_match_device(l3_noc_match.as_ptr(), &mut (*pdev).dev);
    if of_id.is_null() { dev_err(&mut (*pdev).dev, c"OF data missing\n".as_ptr()); return -EINVAL; }
    static mut L3: *mut omap_l3 = core::ptr::null_mut();
    L3 = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<omap_l3>(), GFP_KERNEL) as *mut omap_l3;
    if L3.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping((*of_id).data as *const omap_l3, L3, 1);
    (*L3).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, L3 as *mut _);
    let mut res_idx = 0;
    for i in 0..(*L3).num_modules {
        if (*L3).l3_base[i as usize] == L3_BASE_IS_SUBMODULE { BUG_ON(i == 0); (*L3).l3_base[i as usize] = (*L3).l3_base[(i - 1) as usize]; continue; }
        let res = platform_get_resource(pdev, IORESOURCE_MEM, res_idx);
        (*L3).l3_base[i as usize] = devm_ioremap_resource(&mut (*pdev).dev, res);
        if IS_ERR((*L3).l3_base[i as usize]) { dev_err((*L3).dev, c"ioremap %d failed\n".as_ptr(), i); return PTR_ERR((*L3).l3_base[i as usize]); }
        res_idx += 1;
    }
    (*L3).debug_irq = platform_get_irq(pdev, 0);
    let ret = devm_request_irq((*L3).dev, (*L3).debug_irq, l3_interrupt_handler, IRQF_NO_THREAD, c"l3-dbg-irq".as_ptr(), L3 as *mut _);
    if ret != 0 { dev_err((*L3).dev, c"request_irq failed for %d\n".as_ptr(), (*L3).debug_irq); return ret; }
    (*L3).app_irq = platform_get_irq(pdev, 1);
    let ret = devm_request_irq((*L3).dev, (*L3).app_irq, l3_interrupt_handler, IRQF_NO_THREAD, c"l3-app-irq".as_ptr(), L3 as *mut _);
    if ret != 0 { dev_err((*L3).dev, c"request_irq failed for %d\n".as_ptr(), (*L3).app_irq); }
    ret
}

// CONFIG_PM_SLEEP conditional preserved from the C source.
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn l3_resume_noirq(dev: *mut device) -> i32 {
    let l3 = dev_get_drvdata(dev) as *mut omap_l3;
    let mut mask_regx = core::ptr::null_mut();
    for i in 0..(*l3).num_modules {
        let base = (*l3).l3_base[i as usize]; let flag_mux = (*l3).l3_flagmux[i as usize];
        if (*flag_mux).mask_app_bits == 0 && (*flag_mux).mask_dbg_bits == 0 { continue; }
        mask_regx = (base as *mut u8).add(((*flag_mux).offset + L3_FLAGMUX_MASK0 + (L3_APPLICATION_ERROR << 3)) as usize) as *mut _;
        writel_relaxed(readl_relaxed(mask_regx) & !(*flag_mux).mask_app_bits, mask_regx);
        mask_regx = (base as *mut u8).add(((*flag_mux).offset + L3_FLAGMUX_MASK0 + (L3_DEBUG_ERROR << 3)) as usize) as *mut _;
        writel_relaxed(readl_relaxed(mask_regx) & !(*flag_mux).mask_dbg_bits, mask_regx);
    }
    if !mask_regx.is_null() { let _ = readl(mask_regx); } 0
}

// Platform-driver registration and module metadata are provided by the kernel
// integration layer; the C source registers omap_l3_probe at postcore init and
// unregisters it at module exit.
extern "C" {
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

static mut OMAP_L3_DRIVER: platform_driver = platform_driver {
    probe: Some(omap_l3_probe),
    driver: driver {
        name: c"omap_l3_noc".as_ptr(),
        pm: core::ptr::null(),
        of_match_table: l3_noc_match.as_ptr(),
    },
};

unsafe fn omap_l3_init() -> i32 {
    platform_driver_register(&mut OMAP_L3_DRIVER)
}

unsafe fn omap_l3_exit() {
    platform_driver_unregister(&mut OMAP_L3_DRIVER);
}

// postcore_initcall_sync(omap_l3_init), module_exit(omap_l3_exit)
// MODULE_AUTHOR("Santosh Shilimkar");
// MODULE_AUTHOR("Sricharan R");
// MODULE_DESCRIPTION("OMAP L3 Interconnect error handling driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
