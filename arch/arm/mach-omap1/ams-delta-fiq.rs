// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Amstrad E3 FIQ handling
 *
 *  Copyright (C) 2009 Janusz Krzysztofik
 *  Copyright (c) 2006 Matt Callow
 *  Copyright (c) 2004 Amstrad Plc
 *  Copyright (C) 2001 RidgeRun, Inc.
 *
 * Parts of this code are taken from linux/arch/arm/mach-omap/irq.c
 * in the MontaVista 2.4 kernel (and the Amstrad changes therein)
 */

// Kernel and board dependencies supplied by the surrounding repository.

#[repr(C)]
pub struct FiqHandler {
    pub name: *const core::ffi::c_char,
}

extern "C" {
    static mut qwerty_fiqin_start: u8;
    static mut qwerty_fiqin_end: u8;
    fn claim_fiq(fh: *mut FiqHandler) -> i32;
    fn release_fiq(fh: *mut FiqHandler);
    fn set_fiq_handler(start: *mut core::ffi::c_void, length: u32);
    fn set_fiq_regs(regs: *mut PtRegs);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                   flags: u32, name: *const core::ffi::c_char,
                   dev_id: *mut core::ffi::c_void) -> i32;
    fn generic_handle_irq(irq: u32);
    fn irq_get_irq_data(irq: u32) -> *mut IrqData;
    fn gpiod_to_irq(desc: *mut GpioDesc) -> u32;
    fn gpiochip_request_own_desc(chip: *mut GpioChip, hwnum: u32,
                                 label: *const core::ffi::c_char,
                                 flags: u32, dflags: u32) -> *mut GpioDesc;
    fn gpiochip_free_own_desc(desc: *mut GpioDesc);
    fn gpiod_direction_input(desc: *mut GpioDesc) -> i32;
    fn omap_readl(addr: u32) -> u32;
    fn omap_writel(value: u32, addr: u32);
}

#[repr(C)] pub struct GpioDesc;
#[repr(C)] pub struct GpioChip { pub irq: GpioChipIrq, pub label: *const core::ffi::c_char }
#[repr(C)] pub struct GpioChipIrq { pub chip: *mut IrqChip }
#[repr(C)] pub struct IrqChip { pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)> }
#[repr(C)] pub struct IrqData { pub irq: u32 }
#[repr(C)] pub struct PlatformDevice { pub resource: *mut Resource, pub dev: Device }
#[repr(C)] pub struct Resource { pub start: u32, pub end: u32 }
#[repr(C)] pub struct Device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct PtRegs { pub ARM_r9: u32 }

extern "C" {
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn warn_on_once(condition: bool) -> bool;
}

const IRQ_HANDLED: i32 = 1;
const IRQ_TYPE_EDGE_RISING: u32 = 1;

static mut fh: FiqHandler = FiqHandler { name: b"ams-delta-fiq\0".as_ptr() as *const _ };
static mut fiq_buffer: [u32; 1024] = [0; 1024];
static mut irq_chip: *mut IrqChip = core::ptr::null_mut();
static mut irq_data: [*mut IrqData; 16] = [core::ptr::null_mut(); 16];
static mut irq_counter: [u32; 16] = [0; 16];
static pin_name: [*const core::ffi::c_char; 16] = [
    b"keybrd_data\0".as_ptr() as *const _, b"keybrd_clk\0".as_ptr() as *const _,
    core::ptr::null(); 14
];

unsafe extern "C" fn deferred_fiq(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    for gpio in AMS_DELTA_GPIO_PIN_KEYBRD_CLK..=AMS_DELTA_GPIO_PIN_HOOK_SWITCH {
        let d = irq_data[gpio as usize];
        let irq_num = (*d).irq;
        let fiq_count = fiq_buffer[(FIQ_CNT_INT_00 + gpio) as usize];
        if irq_counter[gpio as usize] < fiq_count && gpio != AMS_DELTA_GPIO_PIN_KEYBRD_CLK {
            if !warn_on_once((*irq_chip).irq_unmask.is_none()) {
                ((*irq_chip).irq_unmask.unwrap())(d);
            }
        }
        while irq_counter[gpio as usize] < fiq_count {
            generic_handle_irq(irq_num);
            irq_counter[gpio as usize] = irq_counter[gpio as usize].wrapping_add(1);
        }
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn ams_delta_init_fiq(chip: *mut GpioChip, serio: *mut PlatformDevice) {
    let mut data: *mut GpioDesc = core::ptr::null_mut();
    let mut clk: *mut GpioDesc = core::ptr::null_mut();
    irq_chip = (*chip).irq.chip;
    if irq_chip.is_null() { return; }
    for i in 0..16u32 {
        let gpiod = gpiochip_request_own_desc(chip, i, pin_name[i as usize], 0, 1);
        if gpiod.is_null() { return; }
        irq_data[i as usize] = irq_get_irq_data(gpiod_to_irq(gpiod));
        match i {
            AMS_DELTA_GPIO_PIN_KEYBRD_DATA => { data = gpiod; gpiod_direction_input(data); }
            AMS_DELTA_GPIO_PIN_KEYBRD_CLK => { clk = gpiod; gpiod_direction_input(clk); }
            _ => gpiochip_free_own_desc(gpiod),
        }
    }
    if data.is_null() || clk.is_null() { if !data.is_null() { gpiochip_free_own_desc(data); } if !clk.is_null() { gpiochip_free_own_desc(clk); } return; }
    let start = &mut qwerty_fiqin_start as *mut u8 as *mut core::ffi::c_void;
    let length = (&qwerty_fiqin_end as *const u8 as usize).wrapping_sub(&qwerty_fiqin_start as *const u8 as usize) as u32;
    if claim_fiq(&mut fh) != 0 { goto_out(data, clk); return; }
    if request_irq(INT_DEFERRED_FIQ, deferred_fiq, IRQ_TYPE_EDGE_RISING, b"deferred_fiq\0".as_ptr() as *const _, core::ptr::null_mut()) < 0 { release_fiq(&mut fh); goto_out(data, clk); return; }
    let offset = IRQ_ILR0_REG_OFFSET + (((INT_DEFERRED_FIQ - NR_IRQS_LEGACY) & 0x1f) * 4);
    let val = omap_readl(DEFERRED_FIQ_IH_BASE + offset) & !(1 << 1);
    omap_writel(val, DEFERRED_FIQ_IH_BASE + offset);
    set_fiq_handler(start, length);
    fiq_buffer[FIQ_GPIO_INT_MASK as usize] = 0; fiq_buffer[FIQ_MASK as usize] = 0; fiq_buffer[FIQ_STATE as usize] = 0; fiq_buffer[FIQ_KEY as usize] = 0; fiq_buffer[FIQ_KEYS_CNT as usize] = 0; fiq_buffer[FIQ_KEYS_HICNT as usize] = 0; fiq_buffer[FIQ_TAIL_OFFSET as usize] = 0; fiq_buffer[FIQ_HEAD_OFFSET as usize] = 0; fiq_buffer[FIQ_BUF_LEN as usize] = 256; fiq_buffer[FIQ_MISSED_KEYS as usize] = 0;
    fiq_buffer[FIQ_BUFFER_START as usize] = fiq_buffer.as_ptr().add(FIQ_CIRC_BUFF as usize) as usize as u32;
    for i in FIQ_CNT_INT_00..=FIQ_CNT_INT_15 { fiq_buffer[i as usize] = 0; }
    let mut regs = PtRegs { ARM_r9: fiq_buffer.as_ptr() as usize as u32 }; set_fiq_regs(&mut regs);
    let offset = IRQ_ILR0_REG_OFFSET + (INT_GPIO_BANK1 - NR_IRQS_LEGACY) * 4;
    let val = omap_readl(OMAP_IH1_BASE + offset) | 1; omap_writel(val, OMAP_IH1_BASE + offset);
    (*serio).resource.add(0).start = gpiod_to_irq(clk); (*serio).resource.add(0).end = (*serio).resource.add(0).start; (*serio).dev.platform_data = fiq_buffer.as_mut_ptr() as *mut _;
}

unsafe fn goto_out(data: *mut GpioDesc, clk: *mut GpioDesc) { gpiochip_free_own_desc(data); gpiochip_free_own_desc(clk); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
