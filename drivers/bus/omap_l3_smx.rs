// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OMAP3XXX L3 Interconnect Driver
 *
 * Copyright (C) 2011 Texas Corporation
 *	Felipe Balbi <balbi@ti.com>
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 *	Sricharan <r.sricharan@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the original C includes.

#[inline]
unsafe fn omap3_l3_readll(base: *mut core::ffi::c_void, reg: u16) -> u64 {
    __raw_readll((base as *mut u8).add(reg as usize) as *mut core::ffi::c_void)
}

#[inline]
unsafe fn omap3_l3_writell(base: *mut core::ffi::c_void, reg: u16, value: u64) {
    __raw_writell(value, (base as *mut u8).add(reg as usize) as *mut core::ffi::c_void);
}

#[inline]
unsafe fn omap3_l3_decode_error_code(error: u64) -> omap3_l3_code {
    ((error & 0x0f000000) >> L3_ERROR_LOG_CODE) as omap3_l3_code
}

#[inline]
fn omap3_l3_decode_addr(error_addr: u64) -> u32 { (error_addr & 0xffffffff) as u32 }

#[inline]
fn omap3_l3_decode_cmd(error: u64) -> u32 { ((error & 0x07) >> L3_ERROR_LOG_CMD) as u32 }

#[inline]
fn omap3_l3_decode_initid(error: u64) -> omap3_l3_initiator_id {
    ((error & 0xff00) >> L3_ERROR_LOG_INITID) as omap3_l3_initiator_id
}

#[inline]
fn omap3_l3_decode_req_info(error: u64) -> u32 { ((error >> 32) & 0xffff) as u32 }

unsafe fn omap3_l3_code_string(code: u8) -> *mut u8 {
    match code {
        OMAP_L3_CODE_NOERROR => b"No Error\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_UNSUP_CMD => b"Unsupported Command\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_ADDR_HOLE => b"Address Hole\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_PROTECT_VIOLATION => b"Protection Violation\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_IN_BAND_ERR => b"In-band Error\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_REQ_TOUT_NOT_ACCEPT => b"Request Timeout Not Accepted\0".as_ptr() as *mut u8,
        OMAP_L3_CODE_REQ_TOUT_NO_RESP => b"Request Timeout, no response\0".as_ptr() as *mut u8,
        _ => b"UNKNOWN error\0".as_ptr() as *mut u8,
    }
}

unsafe fn omap3_l3_initiator_string(initid: u8) -> *mut u8 {
    match initid {
        OMAP_L3_LCD => b"LCD\0".as_ptr() as *mut u8,
        OMAP_L3_SAD2D => b"SAD2D\0".as_ptr() as *mut u8,
        OMAP_L3_IA_MPU_SS_1 | OMAP_L3_IA_MPU_SS_2 | OMAP_L3_IA_MPU_SS_3 |
        OMAP_L3_IA_MPU_SS_4 | OMAP_L3_IA_MPU_SS_5 => b"MPU\0".as_ptr() as *mut u8,
        OMAP_L3_IA_IVA_SS_1 | OMAP_L3_IA_IVA_SS_2 | OMAP_L3_IA_IVA_SS_3 => b"IVA_SS\0".as_ptr() as *mut u8,
        OMAP_L3_IA_IVA_SS_DMA_1 | OMAP_L3_IA_IVA_SS_DMA_2 | OMAP_L3_IA_IVA_SS_DMA_3 |
        OMAP_L3_IA_IVA_SS_DMA_4 | OMAP_L3_IA_IVA_SS_DMA_5 | OMAP_L3_IA_IVA_SS_DMA_6 => b"IVA_SS_DMA\0".as_ptr() as *mut u8,
        OMAP_L3_IA_SGX => b"SGX\0".as_ptr() as *mut u8,
        OMAP_L3_IA_CAM_1 | OMAP_L3_IA_CAM_2 | OMAP_L3_IA_CAM_3 => b"CAM\0".as_ptr() as *mut u8,
        OMAP_L3_IA_DAP => b"DAP\0".as_ptr() as *mut u8,
        OMAP_L3_SDMA_WR_1 | OMAP_L3_SDMA_WR_2 => b"SDMA_WR\0".as_ptr() as *mut u8,
        OMAP_L3_SDMA_RD_1 | OMAP_L3_SDMA_RD_2 | OMAP_L3_SDMA_RD_3 | OMAP_L3_SDMA_RD_4 => b"SDMA_RD\0".as_ptr() as *mut u8,
        OMAP_L3_USBOTG => b"USB_OTG\0".as_ptr() as *mut u8,
        OMAP_L3_USBHOST => b"USB_HOST\0".as_ptr() as *mut u8,
        _ => b"UNKNOWN Initiator\0".as_ptr() as *mut u8,
    }
}

unsafe fn omap3_l3_block_irq(l3: *mut omap3_l3, error: u64, error_addr: u64) -> irqreturn_t {
    let code = omap3_l3_decode_error_code(error) as u8;
    let initid = omap3_l3_decode_initid(error) as u8;
    let multi = (error & L3_ERROR_LOG_MULTI) as u8;
    let address = omap3_l3_decode_addr(error_addr);
    pr_err!("%s seen by %s %s at address %x\n", omap3_l3_code_string(code),
            omap3_l3_initiator_string(initid),
            if multi != 0 { "Multiple Errors" } else { "" }, address);
    WARN_ON!(true);
    IRQ_HANDLED
}

unsafe fn omap3_l3_app_irq(irq: i32, _l3: *mut core::ffi::c_void) -> irqreturn_t {
    let l3 = _l3 as *mut omap3_l3;
    let int_type = if irq == (*l3).app_irq { L3_APPLICATION_ERROR } else { L3_DEBUG_ERROR };
    let status = if int_type == 0 { omap3_l3_readll((*l3).rt, L3_SI_FLAG_STATUS_0) }
                 else { omap3_l3_readll((*l3).rt, L3_SI_FLAG_STATUS_1) };
    let err_source = status.trailing_zeros() as usize;
    let base = ((*l3).rt as *mut u8).add(omap3_l3_bases[int_type as usize][err_source] as usize) as *mut core::ffi::c_void;
    let error = omap3_l3_readll(base, L3_ERROR_LOG);
    let mut ret = IRQ_NONE;
    if error != 0 { ret |= omap3_l3_block_irq(l3, error, omap3_l3_readll(base, L3_ERROR_LOG_ADDR)); }
    BUG_ON!(int_type == 0 && (status & L3_STATUS_0_TIMEOUT_MASK) != 0);
    let clear = (L3_AGENT_STATUS_CLEAR_IA << int_type) | L3_AGENT_STATUS_CLEAR_TA;
    omap3_l3_writell(base, L3_AGENT_STATUS, clear);
    omap3_l3_writell(base, L3_ERROR_LOG, error);
    ret
}

// The remaining platform-driver declarations and registration use the kernel
// types and APIs supplied by the corresponding translated dependencies.
unsafe fn omap3_l3_probe(pdev: *mut platform_device) -> i32 {
    let l3 = kzalloc_obj::<omap3_l3>();
    if l3.is_null() { return -12; }
    platform_set_drvdata(pdev, l3 as *mut core::ffi::c_void);
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() { kfree(l3); return -19; }
    (*l3).rt = ioremap((*res).start, resource_size(res));
    if (*l3).rt.is_null() { kfree(l3); return -12; }
    (*l3).debug_irq = platform_get_irq(pdev, 0);
    let mut ret = request_irq((*l3).debug_irq, omap3_l3_app_irq, IRQF_TRIGGER_RISING, b"l3-debug-irq\0".as_ptr(), l3 as *mut _);
    if ret != 0 { iounmap((*l3).rt); kfree(l3); return ret; }
    (*l3).app_irq = platform_get_irq(pdev, 1);
    ret = request_irq((*l3).app_irq, omap3_l3_app_irq, IRQF_TRIGGER_RISING, b"l3-app-irq\0".as_ptr(), l3 as *mut _);
    if ret != 0 { free_irq((*l3).debug_irq, l3 as *mut _); iounmap((*l3).rt); kfree(l3); }
    ret
}

unsafe fn omap3_l3_remove(pdev: *mut platform_device) {
    let l3 = platform_get_drvdata(pdev) as *mut omap3_l3;
    free_irq((*l3).app_irq, l3 as *mut _); free_irq((*l3).debug_irq, l3 as *mut _);
    iounmap((*l3).rt); kfree(l3);
}

unsafe fn omap3_l3_init() -> i32 { platform_driver_register(&mut omap3_l3_driver) }
unsafe fn omap3_l3_exit() { platform_driver_unregister(&mut omap3_l3_driver); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
