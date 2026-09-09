// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-pxa/pxa3xx.c
 * code specific to pxa3xx aka Monahans
 * Copyright (C) 2006 Marvell International Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

const fn pecr_ie(n: u32) -> u32 { (1u32 << (n * 2)) << 28 }
const fn pecr_is(n: u32) -> u32 { (1u32 << (n * 2)) << 29 }

extern "C" {
    fn pxa_dt_irq_init(fn_: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>);
}

// NAND NFC: DFI bus arbitration subset
// NDCR is the volatile register at NAND_VIRT + 0.
const NDCR_ND_ARB_EN: u32 = 1 << 12;
const NDCR_ND_ARB_CNTL: u32 = 1 << 19;
const CKEN_BOOT: u32 = 11;
const CKEN_TPM: u32 = 19;
const CKEN_HSIO2: u32 = 41;

#[cfg(feature = "CONFIG_PM")]
const ISRAM_START: usize = 0x5c000000;
#[cfg(feature = "CONFIG_PM")]
const ISRAM_SIZE: usize = 256 * 1024;

#[cfg(feature = "CONFIG_PM")]
static mut sram: *mut core::ffi::c_void = core::ptr::null_mut();
#[cfg(feature = "CONFIG_PM")]
static mut wakeup_src: usize = 0;

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa3xx_cpu_standby(pwrmode: u32) {
    let fn_: unsafe extern "C" fn(u32) = core::mem::transmute((sram as usize) + 0x8000);
    memcpy_toio((sram as usize + 0x8000) as *mut _, pm_enter_standby_start,
                pm_enter_standby_end as usize - pm_enter_standby_start as usize);
    AD2D0SR = !0; AD2D1SR = !0; AD2D0ER = wakeup_src; AD2D1ER = 0;
    ASCR = ASCR; ARSR = ARSR;
    local_fiq_disable(); fn_(pwrmode); local_fiq_enable();
    AD2D0ER = 0; AD2D1ER = 0;
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa3xx_cpu_pm_suspend() {
    let p = 0xc0000000usize as *mut usize;
    let saved_data = core::ptr::read_volatile(p);
    CKENA |= (1 << CKEN_BOOT) | (1 << CKEN_TPM);
    CKENB |= 1 << (CKEN_HSIO2 & 0x1f);
    AD3SR = !0; AD3ER = wakeup_src; ASCR = ASCR; ARSR = ARSR;
    PCFR |= 1u32 << 13; PCFR &= !((1u32 << 12) | (1u32 << 1));
    PSPR = 0x5c014000;
    core::ptr::write_volatile(p, __pa_symbol(cpu_resume));
    cpu_suspend(0, Some(pxa3xx_finish_suspend));
    core::ptr::write_volatile(p, saved_data); AD3ER = 0;
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa3xx_cpu_pm_enter(state: suspend_state_t) {
    if wakeup_src == 0 { printk(KERN_ERR, "Not suspending: no wakeup sources\n"); return; }
    match state {
        PM_SUSPEND_STANDBY => pxa3xx_cpu_standby(PXA3xx_PM_S0D2C2),
        PM_SUSPEND_MEM => pxa3xx_cpu_pm_suspend(),
        _ => {}
    }
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn pxa3xx_cpu_pm_valid(state: suspend_state_t) -> i32 {
    (state == PM_SUSPEND_MEM || state == PM_SUSPEND_STANDBY) as i32
}

#[cfg(feature = "CONFIG_PM")]
static mut pxa3xx_cpu_pm_fns: pxa_cpu_pm_fns = pxa_cpu_pm_fns { valid: Some(pxa3xx_cpu_pm_valid), enter: Some(pxa3xx_cpu_pm_enter) };

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa3xx_init_pm() {
    sram = ioremap(ISRAM_START, ISRAM_SIZE);
    if sram.is_null() { printk(KERN_ERR, "Unable to map ISRAM: disabling standby/suspend\n"); return; }
    AD1R |= ADXR_L2 | ADXR_R0; AD2R |= ADXR_L2 | ADXR_R0; AD3R |= ADXR_L2 | ADXR_R0;
    AD1D0ER = 0; AD2D0ER = 0; AD2D1ER = 0; AD3ER = 0;
    pxa_cpu_pm_fns = &mut pxa3xx_cpu_pm_fns;
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn pxa3xx_set_wake(d: *mut irq_data, on: u32) -> i32 {
    let mask = match (*d).irq {
        IRQ_SSP3 => ADXER_MFP_WSSP3, IRQ_MSL => ADXER_WMSL0,
        IRQ_USBH2 | IRQ_USBH1 => ADXER_WUSBH, IRQ_KEYPAD => ADXER_WKP,
        IRQ_AC97 => ADXER_MFP_WAC97, IRQ_USIM => ADXER_WUSIM0,
        IRQ_SSP2 => ADXER_MFP_WSSP2, IRQ_I2C => ADXER_MFP_WI2C,
        IRQ_STUART => ADXER_MFP_WUART3, IRQ_BTUART => ADXER_MFP_WUART2,
        IRQ_FFUART => ADXER_MFP_WUART1, IRQ_MMC => ADXER_MFP_WMMC1,
        IRQ_SSP => ADXER_MFP_WSSP1, IRQ_RTCAlrm => ADXER_WRTC,
        IRQ_SSP4 => ADXER_MFP_WSSP4, IRQ_TSI => ADXER_WTSI,
        IRQ_USIM2 => ADXER_WUSIM1, IRQ_MMC2 => ADXER_MFP_WMMC2,
        IRQ_NAND => ADXER_MFP_WFLASH, IRQ_USB2 => ADXER_WUSB2,
        IRQ_WAKEUP0 => ADXER_WEXTWAKE0, IRQ_WAKEUP1 => ADXER_WEXTWAKE1,
        IRQ_MMC3 => ADXER_MFP_GEN12, _ => return -EINVAL,
    };
    let mut flags = 0usize; local_irq_save(&mut flags);
    if on != 0 { wakeup_src |= mask as usize; } else { wakeup_src &= !(mask as usize); }
    local_irq_restore(flags); 0
}

#[cfg(not(feature = "CONFIG_PM"))]
unsafe fn pxa3xx_init_pm() {}
#[cfg(not(feature = "CONFIG_PM"))]
const pxa3xx_set_wake: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32> = None;

unsafe fn pxa_ack_ext_wakeup(d: *mut irq_data) { PECR |= pecr_is((*d).irq - IRQ_WAKEUP0); }
unsafe fn pxa_mask_ext_wakeup(d: *mut irq_data) { pxa_mask_irq(d); PECR &= !pecr_ie((*d).irq - IRQ_WAKEUP0); }
unsafe fn pxa_unmask_ext_wakeup(d: *mut irq_data) { pxa_unmask_irq(d); PECR |= pecr_ie((*d).irq - IRQ_WAKEUP0); }
unsafe fn pxa_set_ext_wakeup_type(d: *mut irq_data, flow_type: u32) -> i32 {
    if flow_type & IRQ_TYPE_EDGE_RISING != 0 { PWER |= 1 << ((*d).irq - IRQ_WAKEUP0); }
    if flow_type & IRQ_TYPE_EDGE_FALLING != 0 { PWER |= 1 << ((*d).irq - IRQ_WAKEUP0 + 2); } 0
}

static mut pxa_ext_wakeup_chip: irq_chip = irq_chip {
    name: "WAKEUP", irq_ack: Some(pxa_ack_ext_wakeup),
    irq_mask: Some(pxa_mask_ext_wakeup), irq_unmask: Some(pxa_unmask_ext_wakeup),
    irq_set_type: Some(pxa_set_ext_wakeup_type), irq_set_wake: None,
};

unsafe fn pxa_init_ext_wakeup_irq(fn_: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>) {
    let mut irq = IRQ_WAKEUP0; while irq <= IRQ_WAKEUP1 {
        irq_set_chip_and_handler(irq, &pxa_ext_wakeup_chip, handle_edge_irq);
        irq_clear_status_flags(irq, IRQ_NOREQUEST); irq += 1;
    }
    pxa_ext_wakeup_chip.irq_set_wake = fn_;
}

unsafe fn __pxa3xx_init_irq() { pxa_init_ext_wakeup_irq(pxa3xx_set_wake); }
unsafe extern "C" fn pxa3xx_dt_init_irq(_node: *mut device_node, _parent: *mut device_node) -> i32 {
    __pxa3xx_init_irq(); pxa_dt_irq_init(pxa3xx_set_wake); set_handle_irq(ichp_handle_irq); 0
}

static mut pxa3xx_io_desc: [map_desc; 2] = [
    map_desc { virtual_: SMEMC_VIRT as usize, pfn: __phys_to_pfn(PXA3XX_SMEMC_BASE), length: SMEMC_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: NAND_VIRT as usize, pfn: __phys_to_pfn(NAND_PHYS), length: NAND_SIZE, type_: MT_DEVICE },
];

unsafe fn pxa3xx_map_io() { pxa_map_io(); iotable_init(pxa3xx_io_desc.as_ptr(), 2); pxa3xx_get_clk_frequency_khz(1); }

unsafe extern "C" fn pxa3xx_init() -> i32 {
    if cpu_is_pxa3xx() {
        pxa_register_wdt(ARSR);
        ASCR &= !(ASCR_RDH | ASCR_D1S | ASCR_D2S | ASCR_D3S);
        NDCR = (NDCR & !NDCR_ND_ARB_EN) | NDCR_ND_ARB_CNTL;
        pxa3xx_init_pm(); enable_irq_wake(IRQ_WAKEUP0);
        if cpu_is_pxa320() { enable_irq_wake(IRQ_WAKEUP1); }
        register_syscore(&pxa_irq_syscore); register_syscore(&pxa3xx_mfp_syscore);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
