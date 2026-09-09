// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015-2017, 2026 Broadcom

#[repr(C)]
enum GioRegIndex { GIO_REG_ODEN = 0, GIO_REG_DATA, GIO_REG_IODIR, GIO_REG_EC, GIO_REG_EI, GIO_REG_MASK, GIO_REG_LEVEL, GIO_REG_STAT, NUMBER_OF_GIO_REGISTERS }

const GIO_BANK_SIZE: usize = (GioRegIndex::NUMBER_OF_GIO_REGISTERS as usize) * core::mem::size_of::<u32>();
const fn gio_bank_off(bank: usize, off: usize) -> usize { bank * GIO_BANK_SIZE + off * core::mem::size_of::<u32>() }
const fn gio_oden(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_ODEN as usize) }
const fn gio_data(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_DATA as usize) }
const fn gio_iodir(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_IODIR as usize) }
const fn gio_ec(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_EC as usize) }
const fn gio_ei(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_EI as usize) }
const fn gio_mask(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_MASK as usize) }
const fn gio_level(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_LEVEL as usize) }
const fn gio_stat(bank: usize) -> usize { gio_bank_off(bank, GioRegIndex::GIO_REG_STAT as usize) }

#[repr(C)]
struct BrcmstbGpioBank { node: list_head, id: i32, chip: gpio_generic_chip, parent_priv: *mut BrcmstbGpioPriv, width: u32, wake_active: u32, saved_regs: [u32; GioRegIndex::GIO_REG_STAT as usize] }
#[repr(C)]
struct BrcmstbGpioPriv { bank_list: list_head, reg_base: *mut core::ffi::c_void, pdev: *mut platform_device, irq_domain: *mut irq_domain, irq_chip: irq_chip, parent_irq: i32, num_gpios: i32, parent_wake_irq: i32, suspended: bool }

const MAX_GPIO_PER_BANK: u32 = 32;
const fn gpio_bank(gpio: u32) -> u32 { gpio >> 5 }
const fn gpio_bit(gpio: u32) -> u32 { gpio & (MAX_GPIO_PER_BANK - 1) }

unsafe fn brcmstb_gpio_gc_to_priv(gc: *mut gpio_chip) -> *mut BrcmstbGpioPriv { (*gpiochip_get_data(gc)).parent_priv }
unsafe fn __brcmstb_gpio_get_active_irqs(bank: *mut BrcmstbGpioBank) -> usize { let base = (*(*bank).parent_priv).reg_base as *mut u8; (gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_stat((*bank).id as usize))) & gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_mask((*bank).id as usize)))) as usize }
unsafe fn brcmstb_gpio_get_active_irqs(bank: *mut BrcmstbGpioBank) -> usize { let _guard = gpio_generic_lock_irqsave(&mut (*bank).chip); __brcmstb_gpio_get_active_irqs(bank) }
unsafe fn brcmstb_gpio_hwirq_to_offset(hwirq: irq_hw_number_t, bank: *mut BrcmstbGpioBank) -> i32 { hwirq as i32 - (*bank).chip.gc.offset }
unsafe fn __brcmstb_gpio_set_imask(bank: *mut BrcmstbGpioBank, hwirq: irq_hw_number_t, enable: bool) { let priv_ = (*bank).parent_priv; let mask = 1u32 << brcmstb_gpio_hwirq_to_offset(hwirq, bank); let base = (*priv_).reg_base as *mut u8; let mut imask = gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_mask((*bank).id as usize))); if enable { imask |= mask } else { imask &= !mask }; gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_mask((*bank).id as usize)), imask); }
unsafe fn brcmstb_gpio_set_imask(bank: *mut BrcmstbGpioBank, hwirq: irq_hw_number_t, enable: bool) { let _guard = gpio_generic_lock_irqsave(&mut (*bank).chip); __brcmstb_gpio_set_imask(bank, hwirq, enable); }
unsafe fn brcmstb_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 { let priv_ = brcmstb_gpio_gc_to_priv(gc); let hwirq = offset as i32 + (*gc).offset; if hwirq >= (*priv_).num_gpios { return -ENXIO }; irq_create_mapping((*priv_).irq_domain, hwirq as irq_hw_number_t) }

unsafe fn brcmstb_gpio_irq_mask(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); brcmstb_gpio_set_imask(gpiochip_get_data(gc), irqd_to_hwirq(d), false) }
unsafe fn brcmstb_gpio_irq_mask_ack(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let bank = gpiochip_get_data(gc); let priv_ = (*bank).parent_priv; let hwirq = irqd_to_hwirq(d); let mask = 1u32 << brcmstb_gpio_hwirq_to_offset(hwirq, bank); let _guard = gpio_generic_lock_irqsave(&mut (*bank).chip); __brcmstb_gpio_set_imask(bank, hwirq, false); let base = (*priv_).reg_base as *mut u8; gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_stat((*bank).id as usize)), mask); }
unsafe fn brcmstb_gpio_irq_unmask(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); brcmstb_gpio_set_imask(gpiochip_get_data(gc), irqd_to_hwirq(d), true) }
unsafe fn brcmstb_gpio_irq_ack(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let bank = gpiochip_get_data(gc); let priv_ = (*bank).parent_priv; let mask = 1u32 << brcmstb_gpio_hwirq_to_offset(irqd_to_hwirq(d), bank); let base = (*priv_).reg_base as *mut u8; gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_stat((*bank).id as usize)), mask); }

unsafe fn brcmstb_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> i32 { let gc = irq_data_get_irq_chip_data(d); let bank = gpiochip_get_data(gc); let priv_ = (*bank).parent_priv; let mask = 1u32 << brcmstb_gpio_hwirq_to_offset(irqd_to_hwirq(d), bank); let (level, edge_config, edge_insensitive) = match type_ { IRQ_TYPE_LEVEL_LOW => (mask,0,0), IRQ_TYPE_LEVEL_HIGH => (mask,mask,0), IRQ_TYPE_EDGE_FALLING => (0,0,0), IRQ_TYPE_EDGE_RISING => (0,mask,0), IRQ_TYPE_EDGE_BOTH => (0,0,mask), _ => return -EINVAL }; let _guard = gpio_generic_lock_irqsave(&mut (*bank).chip); let base = (*priv_).reg_base as *mut u8; let ec = gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_ec((*bank).id as usize))) & !mask; let ei = gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_ei((*bank).id as usize))) & !mask; let lev = gpio_generic_read_reg(&mut (*bank).chip, base.add(gio_level((*bank).id as usize))) & !mask; gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_ec((*bank).id as usize)), ec | edge_config); gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_ei((*bank).id as usize)), ei | edge_insensitive); gpio_generic_write_reg(&mut (*bank).chip, base.add(gio_level((*bank).id as usize)), lev | level); 0 }

unsafe fn brcmstb_gpio_priv_set_wake(priv_: *mut BrcmstbGpioPriv, enable: u32) -> i32 { if (*priv_).parent_wake_irq == (*priv_).parent_irq { return 0 }; let ret = if enable != 0 { enable_irq_wake((*priv_).parent_wake_irq) } else { disable_irq_wake((*priv_).parent_wake_irq) }; if ret != 0 { dev_err(&(*(*priv_).pdev).dev, "failed to %s wake-up interrupt\n", str_enable_disable(enable)); } ret }
unsafe fn brcmstb_gpio_irq_set_wake(d: *mut irq_data, enable: u32) -> i32 { let gc = irq_data_get_irq_chip_data(d); let bank = gpiochip_get_data(gc); let priv_ = (*bank).parent_priv; let mask = 1u32 << brcmstb_gpio_hwirq_to_offset(irqd_to_hwirq(d), bank); if enable != 0 { (*bank).wake_active |= mask } else { (*bank).wake_active &= !mask }; brcmstb_gpio_priv_set_wake(priv_, enable) }
unsafe fn brcmstb_gpio_wake_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t { let priv_ = data as *mut BrcmstbGpioPriv; if priv_.is_null() || irq != (*priv_).parent_wake_irq { return IRQ_NONE }; IRQ_HANDLED }

unsafe fn brcmstb_gpio_irq_bank_handler(bank: *mut BrcmstbGpioBank) { let priv_ = (*bank).parent_priv; let domain = (*priv_).irq_domain; let hwbase = (*bank).chip.gc.offset; while { let status = brcmstb_gpio_get_active_irqs(bank); if status == 0 { break }; if (*priv_).suspended && (*bank).wake_active & status as u32 != 0 { (*priv_).suspended = false; pm_wakeup_event(&(*(*priv_).pdev).dev, 0); } for offset in 0..32 { if status & (1usize << offset) != 0 { if offset as u32 >= (*bank).width { dev_warn(&(*(*priv_).pdev).dev, "IRQ for invalid GPIO (bank={}, offset={})\n", (*bank).id, offset); } generic_handle_domain_irq(domain, (hwbase + offset as i32) as irq_hw_number_t); } } true } {} }
unsafe fn brcmstb_gpio_irq_handler(desc: *mut irq_desc) { let priv_ = irq_desc_get_handler_data(desc) as *mut BrcmstbGpioPriv; let chip = irq_desc_get_chip(desc); BUG_ON(priv_.is_null() || chip.is_null()); chained_irq_enter(chip, desc); list_for_each_entry!(bank, &mut (*priv_).bank_list, node, { brcmstb_gpio_irq_bank_handler(bank); }); chained_irq_exit(chip, desc); }

// Remaining platform-driver declarations and callbacks are retained as external-kernel bindings.
unsafe fn brcmstb_gpio_hwirq_to_bank(priv_: *mut BrcmstbGpioPriv, hwirq: irq_hw_number_t) -> *mut BrcmstbGpioBank { let mut result = core::ptr::null_mut(); list_for_each_entry!(bank, &mut (*priv_).bank_list, node, { if hwirq >= (*bank).chip.gc.offset as irq_hw_number_t && hwirq < ((*bank).chip.gc.offset + (*bank).chip.gc.ngpio) as irq_hw_number_t { result = bank; } }); result }
unsafe fn brcmstb_gpio_irq_unmap(_d: *mut irq_domain, irq: u32) { irq_set_chip_and_handler(irq, core::ptr::null_mut(), None); irq_set_chip_data(irq, core::ptr::null_mut()); }
unsafe fn brcmstb_gpio_bank_save(priv_: *mut BrcmstbGpioPriv, bank: *mut BrcmstbGpioBank) { for i in 0..GioRegIndex::GIO_REG_STAT as usize { (*bank).saved_regs[i] = gpio_generic_read_reg(&mut (*bank).chip, ((*priv_).reg_base as *mut u8).add(gio_bank_off((*bank).id as usize, i))); } }
unsafe fn brcmstb_gpio_bank_restore(priv_: *mut BrcmstbGpioPriv, bank: *mut BrcmstbGpioBank) { for i in 0..GioRegIndex::GIO_REG_STAT as usize { gpio_generic_write_reg(&mut (*bank).chip, ((*priv_).reg_base as *mut u8).add(gio_bank_off((*bank).id as usize, i)), (*bank).saved_regs[i]); } }
unsafe fn brcmstb_gpio_quiesce(priv_: *mut BrcmstbGpioPriv, save: bool) { list_for_each_entry!(bank, &mut (*priv_).bank_list, node, { if save { brcmstb_gpio_bank_save(priv_, bank); } let mask = if (*priv_).parent_wake_irq != 0 { (*bank).wake_active } else { 0 }; gpio_generic_write_reg(&mut (*bank).chip, ((*priv_).reg_base as *mut u8).add(gio_mask((*bank).id as usize)), mask); }); }
unsafe fn brcmstb_gpio_suspend(_dev: *mut device) -> i32 { 0 }
unsafe fn brcmstb_gpio_suspend_noirq(_dev: *mut device) -> i32 { 0 }
unsafe fn brcmstb_gpio_resume(_dev: *mut device) -> i32 { 0 }

extern "C" {
    fn brcmstb_gpio_remove(pdev: *mut platform_device);
    fn brcmstb_gpio_probe(pdev: *mut platform_device) -> i32;
    fn brcmstb_gpio_shutdown(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
