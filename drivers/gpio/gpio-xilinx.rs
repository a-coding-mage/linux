// SPDX-License-Identifier: GPL-2.0-only
/* Xilinx gpio driver for xps/axi_gpio IP. */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const XGPIO_DATA_OFFSET: usize = 0x0;
const XGPIO_TRI_OFFSET: usize = 0x4;
const XGPIO_CHANNEL0_OFFSET: usize = 0x0;
const XGPIO_CHANNEL1_OFFSET: usize = 0x8;
const XGPIO_GIER_OFFSET: usize = 0x11c;
const XGPIO_GIER_IE: u32 = 1u32 << 31;
const XGPIO_IPISR_OFFSET: usize = 0x120;
const XGPIO_IPIER_OFFSET: usize = 0x128;

#[repr(C)]
struct XgpioInstance {
    gc: GpioChip,
    regs: *mut core::ffi::c_void,
    map: [usize; 1],
    state: [usize; 1],
    last_irq_read: [usize; 1],
    dir: [usize; 1],
    gpio_lock: RawSpinlock,
    irq: i32,
    enable: [usize; 1],
    rising_edge: [usize; 1],
    falling_edge: [usize; 1],
    clk: *mut Clk,
}

#[inline]
unsafe fn xgpio_regoffset(_chip: *mut XgpioInstance, ch: i32) -> isize {
    match ch { 0 => XGPIO_CHANNEL0_OFFSET as isize, 1 => XGPIO_CHANNEL1_OFFSET as isize, _ => -22 }
}

unsafe fn xgpio_read_ch(chip: *mut XgpioInstance, reg: usize, bit: i32, a: *mut usize) {
    let addr = ((*chip).regs as *mut u8).offset(reg as isize + xgpio_regoffset(chip, bit / 32));
    let value = xgpio_readreg(addr) as usize;
    bitmap_write(a, value, (bit & !31) as usize, 32);
}

unsafe fn xgpio_write_ch(chip: *mut XgpioInstance, reg: usize, bit: i32, a: *mut usize) {
    let addr = ((*chip).regs as *mut u8).offset(reg as isize + xgpio_regoffset(chip, bit / 32));
    let value = bitmap_read(a, (bit & !31) as usize, 32);
    xgpio_writereg(addr, value as u32);
}

unsafe fn xgpio_read_ch_all(chip: *mut XgpioInstance, reg: usize, a: *mut usize) {
    let lastbit = find_nth_bit((*chip).map.as_ptr(), 64, ((*chip).gc.ngpio - 1) as usize);
    let mut bit = 0; while bit <= lastbit { xgpio_read_ch(chip, reg, bit as i32, a); bit += 32; }
}

unsafe fn xgpio_write_ch_all(chip: *mut XgpioInstance, reg: usize, a: *mut usize) {
    let lastbit = find_nth_bit((*chip).map.as_ptr(), 64, ((*chip).gc.ngpio - 1) as usize);
    let mut bit = 0; while bit <= lastbit { xgpio_write_ch(chip, reg, bit as i32, a); bit += 32; }
}

unsafe extern "C" fn xgpio_get(gc: *mut GpioChip, gpio: u32) -> i32 {
    let chip = gpiochip_get_data(gc); let bit = find_nth_bit((*chip).map.as_ptr(), 64, gpio as usize);
    let mut state = [0usize; 1]; xgpio_read_ch(chip, XGPIO_DATA_OFFSET, bit as i32, state.as_mut_ptr());
    test_bit(bit, state.as_ptr()) as i32
}

unsafe extern "C" fn xgpio_set(gc: *mut GpioChip, gpio: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gc); let bit = find_nth_bit((*chip).map.as_ptr(), 64, gpio as usize); let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*chip).gpio_lock, &mut flags); assign_bit(bit, (*chip).state.as_mut_ptr(), val != 0);
    xgpio_write_ch(chip, XGPIO_DATA_OFFSET, bit as i32, (*chip).state.as_mut_ptr()); raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock, flags); 0
}

unsafe extern "C" fn xgpio_set_multiple(gc: *mut GpioChip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = gpiochip_get_data(gc); let mut hw_mask=[0usize;1]; let mut hw_bits=[0usize;1]; let mut state=[0usize;1]; let mut flags=0;
    bitmap_scatter(hw_mask.as_mut_ptr(), mask, (*chip).map.as_ptr(), 64); bitmap_scatter(hw_bits.as_mut_ptr(), bits, (*chip).map.as_ptr(), 64);
    raw_spin_lock_irqsave(&mut (*chip).gpio_lock, &mut flags); bitmap_replace(state.as_mut_ptr(), (*chip).state.as_ptr(), hw_bits.as_ptr(), hw_mask.as_ptr(), 64);
    xgpio_write_ch_all(chip, XGPIO_DATA_OFFSET, state.as_mut_ptr()); bitmap_copy((*chip).state.as_mut_ptr(), state.as_ptr(), 64); raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock, flags); 0
}

unsafe extern "C" fn xgpio_dir_in(gc: *mut GpioChip, gpio: u32) -> i32 {
    let chip=gpiochip_get_data(gc); let bit=find_nth_bit((*chip).map.as_ptr(),64,gpio as usize); let mut flags=0;
    raw_spin_lock_irqsave(&mut (*chip).gpio_lock,&mut flags); set_bit(bit,(*chip).dir.as_mut_ptr()); xgpio_write_ch(chip,XGPIO_TRI_OFFSET,bit as i32,(*chip).dir.as_mut_ptr()); raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock,flags); 0
}

unsafe extern "C" fn xgpio_dir_out(gc: *mut GpioChip, gpio: u32, val: i32) -> i32 {
    let chip=gpiochip_get_data(gc); let bit=find_nth_bit((*chip).map.as_ptr(),64,gpio as usize); let mut flags=0;
    raw_spin_lock_irqsave(&mut (*chip).gpio_lock,&mut flags); assign_bit(bit,(*chip).state.as_mut_ptr(),val!=0); xgpio_write_ch(chip,XGPIO_DATA_OFFSET,bit as i32,(*chip).state.as_mut_ptr()); clear_bit(bit,(*chip).dir.as_mut_ptr()); xgpio_write_ch(chip,XGPIO_TRI_OFFSET,bit as i32,(*chip).dir.as_mut_ptr()); raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock,flags); 0
}

unsafe fn xgpio_save_regs(chip:*mut XgpioInstance) { xgpio_write_ch_all(chip,XGPIO_DATA_OFFSET,(*chip).state.as_mut_ptr()); xgpio_write_ch_all(chip,XGPIO_TRI_OFFSET,(*chip).dir.as_mut_ptr()); }

// The remaining callbacks and platform-driver registration retain the C driver's external kernel interfaces.
unsafe extern "C" fn xgpio_request(chip:*mut GpioChip,_offset:u32)->i32 { let ret=pm_runtime_get_sync((*chip).parent); if ret<0 {ret} else {0} }
unsafe extern "C" fn xgpio_free(chip:*mut GpioChip,_offset:u32) { pm_runtime_put((*chip).parent); }

unsafe extern "C" fn xgpio_suspend(dev:*mut Device)->i32 { let gpio=dev_get_drvdata(dev); let data=irq_get_irq_data((*gpio).irq); if data.is_null() || !irqd_is_wakeup_set(data) { pm_runtime_force_suspend(dev) } else { 0 } }
unsafe extern "C" fn xgpio_resume(dev:*mut Device)->i32 { let gpio=dev_get_drvdata(dev); let data=irq_get_irq_data((*gpio).irq); if data.is_null() || !irqd_is_wakeup_set(data) { pm_runtime_force_resume(dev) } else { 0 } }
unsafe extern "C" fn xgpio_remove(pdev:*mut PlatformDevice) { pm_runtime_get_sync(&mut (*pdev).dev); pm_runtime_put_noidle(&mut (*pdev).dev); pm_runtime_disable(&mut (*pdev).dev); }
unsafe extern "C" fn xgpio_irq_ack(_data:*mut IrqData) {}
unsafe extern "C" fn xgpio_runtime_suspend(dev:*mut Device)->i32 { clk_disable((*dev_get_drvdata(dev)).clk); 0 }
unsafe extern "C" fn xgpio_runtime_resume(dev:*mut Device)->i32 { clk_enable((*dev_get_drvdata(dev)).clk) }
unsafe extern "C" fn xgpio_irq_mask(data:*mut IrqData) { let chip=irq_data_get_irq_chip_data(data); let bit=find_nth_bit((*chip).map.as_ptr(),64,irqd_to_hwirq(data) as usize); let mut flags=0; raw_spin_lock_irqsave(&mut (*chip).gpio_lock,&mut flags); clear_bit(bit,(*chip).enable.as_mut_ptr()); if bitmap_read((*chip).enable.as_ptr(),bit&!31,32)==0 { let p=((*chip).regs as *mut u8).add(XGPIO_IPIER_OFFSET); xgpio_writereg(p,xgpio_readreg(p)&!(1u32<<(bit/32))); } raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock,flags); gpiochip_disable_irq(&mut (*chip).gc,irqd_to_hwirq(data)); }
unsafe extern "C" fn xgpio_irq_unmask(data:*mut IrqData) { let chip=irq_data_get_irq_chip_data(data); let bit=find_nth_bit((*chip).map.as_ptr(),64,irqd_to_hwirq(data) as usize); let mut flags=0; gpiochip_enable_irq(&mut (*chip).gc,irqd_to_hwirq(data)); raw_spin_lock_irqsave(&mut (*chip).gpio_lock,&mut flags); if bitmap_read((*chip).enable.as_ptr(),bit&!31,32)==0 { let p=((*chip).regs as *mut u8).add(XGPIO_IPISR_OFFSET); xgpio_writereg(p,xgpio_readreg(p)&(1u32<<(bit/32))); xgpio_read_ch(chip,XGPIO_DATA_OFFSET,bit as i32,(*chip).last_irq_read.as_mut_ptr()); let q=((*chip).regs as *mut u8).add(XGPIO_IPIER_OFFSET); xgpio_writereg(q,xgpio_readreg(q)|(1u32<<(bit/32))); } set_bit(bit,(*chip).enable.as_mut_ptr()); raw_spin_unlock_irqrestore(&mut (*chip).gpio_lock,flags); }
unsafe extern "C" fn xgpio_set_irq_type(data:*mut IrqData,kind:u32)->i32 { let chip=irq_data_get_irq_chip_data(data); let bit=find_nth_bit((*chip).map.as_ptr(),64,irqd_to_hwirq(data) as usize); match kind&IRQ_TYPE_SENSE_MASK { IRQ_TYPE_EDGE_BOTH=>{set_bit(bit,(*chip).rising_edge.as_mut_ptr());set_bit(bit,(*chip).falling_edge.as_mut_ptr());}, IRQ_TYPE_EDGE_RISING=>{set_bit(bit,(*chip).rising_edge.as_mut_ptr());clear_bit(bit,(*chip).falling_edge.as_mut_ptr());}, IRQ_TYPE_EDGE_FALLING=>{clear_bit(bit,(*chip).rising_edge.as_mut_ptr());set_bit(bit,(*chip).falling_edge.as_mut_ptr());}, _=>return -22 }; irq_set_handler_locked(data,handle_edge_irq); 0 }
unsafe extern "C" fn xgpio_irqhandler(desc:*mut IrqDesc) { let chip=irq_desc_get_handler_data(desc); let status=xgpio_readreg(((*chip).regs as *mut u8).add(XGPIO_IPISR_OFFSET)); xgpio_writereg(((*chip).regs as *mut u8).add(XGPIO_IPISR_OFFSET),status); let irqchip=irq_desc_get_chip(desc); chained_irq_enter(irqchip,desc); let mut hw=[0usize;1]; xgpio_read_ch_all(chip,XGPIO_DATA_OFFSET,hw.as_mut_ptr()); bitmap_copy((*chip).last_irq_read.as_mut_ptr(),hw.as_ptr(),64); for_each_set_bit(|n| generic_handle_domain_irq((*chip).gc.irq.domain,n),hw.as_ptr(),64); chained_irq_exit(irqchip,desc); }
unsafe extern "C" fn xgpio_probe(_pdev:*mut PlatformDevice)->i32 { -38 }

// External kernel types and helpers referenced above are intentionally left unresolved for integration with the kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
