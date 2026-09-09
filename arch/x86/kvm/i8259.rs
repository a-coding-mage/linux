/*
 * 8259 interrupt controller emulation
 *
 * Copyright (c) 2003-2004 Fabrice Bellard
 * Copyright (c) 2007 Intel Corporation
 * Copyright 2009 Red Hat, Inc. and/or its affiliates.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * Authors:
 *   Yaozu (Eddie) Dong <Eddie.dong@intel.com>
 *   Port from Qemu.
 */

// C headers and kernel build-time macros are supplied by the surrounding crate.

unsafe fn pic_lock(s: *mut kvm_pic) {
    spin_lock(&mut (*s).lock);
}

unsafe fn pic_unlock(s: *mut kvm_pic) {
    let wakeup = (*s).wakeup_needed;
    (*s).wakeup_needed = false;
    spin_unlock(&mut (*s).lock);
    if wakeup {
        let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
        let mut i: c_ulong = 0;
        kvm_for_each_vcpu(i, vcpu, (*s).kvm) {
            if kvm_apic_accept_pic_intr(vcpu) {
                kvm_make_request(KVM_REQ_EVENT, vcpu);
                kvm_vcpu_kick(vcpu);
                return;
            }
        }
    }
}

unsafe fn pic_clear_isr(s: *mut kvm_kpic_state, mut irq: c_int) {
    (*s).isr &= !(1u8 << irq);
    if s != &mut (*(*s).pics_state).pics[0] as *mut _ { irq += 8; }
    pic_unlock((*s).pics_state);
    kvm_notify_acked_irq((*(*s).pics_state).kvm, SELECT_PIC(irq), irq);
    pic_lock((*s).pics_state);
}

unsafe fn pic_set_irq1(s: *mut kvm_kpic_state, irq: c_int, level: c_int) -> c_int {
    let mask = 1u8 << irq;
    let mut ret = 1;
    if (*s).elcr & mask != 0 {
        if level != 0 { ret = ((*s).irr & mask == 0) as c_int; (*s).irr |= mask; (*s).last_irr |= mask; }
        else { (*s).irr &= !mask; (*s).last_irr &= !mask; }
    } else if level != 0 {
        if (*s).last_irr & mask == 0 { ret = ((*s).irr & mask == 0) as c_int; (*s).irr |= mask; }
        (*s).last_irr |= mask;
    } else { (*s).last_irr &= !mask; }
    if (*s).imr & mask != 0 { -1 } else { ret }
}

unsafe fn get_priority(s: *mut kvm_kpic_state, mask: u8) -> c_int {
    if mask == 0 { return 8; }
    let mut priority = 0;
    while mask & (1u8 << ((priority + (*s).priority_add) & 7)) == 0 { priority += 1; }
    priority
}

unsafe fn pic_get_irq(s: *mut kvm_kpic_state) -> c_int {
    let mask = (*s).irr & !(*s).imr;
    let priority = get_priority(s, mask);
    if priority == 8 { return -1; }
    let mut mask = (*s).isr;
    if (*s).special_fully_nested_mode != 0 && s == &mut (*(*s).pics_state).pics[0] as *mut _ { mask &= !(1u8 << 2); }
    let cur_priority = get_priority(s, mask);
    if priority < cur_priority { (priority + (*s).priority_add) & 7 } else { -1 }
}

unsafe fn pic_update_irq(s: *mut kvm_pic) {
    let irq2 = pic_get_irq(&mut (*s).pics[1]);
    if irq2 >= 0 { pic_set_irq1(&mut (*s).pics[0], 2, 1); pic_set_irq1(&mut (*s).pics[0], 2, 0); }
    let irq = pic_get_irq(&mut (*s).pics[0]);
    pic_irq_request((*s).kvm, (irq >= 0) as c_int);
}

pub unsafe fn kvm_pic_update_irq(s: *mut kvm_pic) { pic_lock(s); pic_update_irq(s); pic_unlock(s); }

pub unsafe fn kvm_pic_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, irq_source_id: c_int, level: c_int, _line_status: bool) -> c_int {
    let s = (*(*kvm).arch).vpic;
    let irq = (*e).irqchip.pin;
    if WARN_ON_ONCE(irq < 0 || irq >= PIC_NUM_PINS) { return -1; }
    pic_lock(s);
    let irq_level = __kvm_irq_line_state(&mut (*s).irq_states[irq as usize], irq_source_id, level);
    let ret = pic_set_irq1(&mut (*s).pics[(irq >> 3) as usize], irq & 7, irq_level);
    pic_update_irq(s);
    trace_kvm_pic_set_irq(irq >> 3, irq & 7, (*s).pics[(irq >> 3) as usize].elcr, (*s).pics[(irq >> 3) as usize].imr, ret == 0);
    pic_unlock(s); ret
}

unsafe fn pic_intack(s: *mut kvm_kpic_state, irq: c_int) {
    (*s).isr |= 1u8 << irq;
    if (*s).elcr & (1u8 << irq) == 0 { (*s).irr &= !(1u8 << irq); }
    if (*s).auto_eoi != 0 { if (*s).rotate_on_auto_eoi != 0 { (*s).priority_add = (irq + 1) & 7; } pic_clear_isr(s, irq); }
}

pub unsafe fn kvm_pic_read_irq(kvm: *mut kvm) -> c_int {
    let s = (*(*kvm).arch).vpic; (*s).output = 0; pic_lock(s);
    let mut irq = pic_get_irq(&mut (*s).pics[0]);
    let intno;
    if irq >= 0 { pic_intack(&mut (*s).pics[0], irq); if irq == 2 { let irq2 = pic_get_irq(&mut (*s).pics[1]); let irq2 = if irq2 >= 0 { pic_intack(&mut (*s).pics[1], irq2); irq2 } else { 7 }; intno = (*s).pics[1].irq_base + irq2; } else { intno = (*s).pics[0].irq_base + irq; } }
    else { irq = 7; intno = (*s).pics[0].irq_base + irq; }
    pic_update_irq(s); pic_unlock(s); intno
}

unsafe fn kvm_pic_reset(s: *mut kvm_kpic_state) {
    let edge_irr = (*s).irr & !(*s).elcr; let mut found = false;
    (*s).last_irr = 0; (*s).irr &= (*s).elcr; (*s).imr = 0; (*s).priority_add = 0; (*s).special_mask = 0; (*s).read_reg_select = 0;
    if (*s).init4 == 0 { (*s).special_fully_nested_mode = 0; (*s).auto_eoi = 0; } (*s).init_state = 1;
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut(); let mut i: c_ulong = 0;
    kvm_for_each_vcpu(i, vcpu, (*(*s).pics_state).kvm) { if kvm_apic_accept_pic_intr(vcpu) { found = true; break; } }
    if !found { return; }
    for irq in 0..PIC_NUM_PINS / 2 { if edge_irr & (1u8 << irq) != 0 { pic_clear_isr(s, irq as c_int); } }
}

unsafe fn pic_ioport_write(opaque: *mut kvm_kpic_state, mut addr: u32, val: u32) {
    let s = opaque; addr &= 1;
    if addr == 0 {
        if val & 0x10 != 0 { (*s).init4 = val & 1; if val & 2 != 0 { pr_pic_unimpl("single mode not supported"); } if val & 8 != 0 { pr_pic_unimpl("level sensitive irq not supported"); } kvm_pic_reset(s); }
        else if val & 8 != 0 { if val & 4 != 0 { (*s).poll = 1; } if val & 2 != 0 { (*s).read_reg_select = val & 1; } if val & 0x40 != 0 { (*s).special_mask = (val >> 5) & 1; } }
        else { let cmd = val >> 5; match cmd { 0 | 4 => (*s).rotate_on_auto_eoi = cmd >> 2, 1 | 5 => { let priority = get_priority(s, (*s).isr); if priority != 8 { let irq = (priority + (*s).priority_add) & 7; if cmd == 5 { (*s).priority_add = (irq + 1) & 7; } pic_clear_isr(s, irq); pic_update_irq((*s).pics_state); } }, 3 => { pic_clear_isr(s, (val & 7) as c_int); pic_update_irq((*s).pics_state); }, 6 => { (*s).priority_add = (val as c_int + 1) & 7; pic_update_irq((*s).pics_state); }, 7 => { let irq = (val & 7) as c_int; (*s).priority_add = (irq + 1) & 7; pic_clear_isr(s, irq); pic_update_irq((*s).pics_state); }, _ => {} } }
    } else { match (*s).init_state { 0 => { let diff = (*s).imr ^ val as u8; let off = if s == &mut (*(*s).pics_state).pics[0] as *mut _ { 0 } else { 8 }; (*s).imr = val as u8; for irq in 0..PIC_NUM_PINS / 2 { if diff & (1u8 << irq) != 0 { kvm_fire_mask_notifiers((*(*s).pics_state).kvm, SELECT_PIC(irq as c_int + off), irq as c_int + off, ((*s).imr & (1u8 << irq)) != 0); } } pic_update_irq((*s).pics_state); }, 1 => { (*s).irq_base = val as u8 & 0xf8; (*s).init_state = 2; }, 2 => { (*s).init_state = if (*s).init4 != 0 { 3 } else { 0 }; }, 3 => { (*s).special_fully_nested_mode = (val >> 4) & 1; (*s).auto_eoi = (val >> 1) & 1; (*s).init_state = 0; }, _ => {} } }
}

unsafe fn pic_poll_read(s: *mut kvm_kpic_state, addr1: u32) -> u32 { let ret = pic_get_irq(s); if ret >= 0 { if addr1 >> 7 != 0 { (*(*s).pics_state).pics[0].isr &= !(1u8 << 2); (*(*s).pics_state).pics[0].irr &= !(1u8 << 2); } (*s).irr &= !(1u8 << ret); pic_clear_isr(s, ret); if addr1 >> 7 != 0 || ret != 2 { pic_update_irq((*s).pics_state); } (ret as u32) | 0x80 } else { pic_update_irq((*s).pics_state); 7 } }
unsafe fn pic_ioport_read(opaque: *mut kvm_kpic_state, addr: u32) -> u32 { let s = opaque; if (*s).poll != 0 { (*s).poll = 0; pic_poll_read(s, addr) } else if addr & 1 == 0 { if (*s).read_reg_select != 0 { (*s).isr as u32 } else { (*s).irr as u32 } } else { (*s).imr as u32 } }
unsafe fn elcr_ioport_write(opaque: *mut kvm_kpic_state, val: u32) { (*opaque).elcr = val as u8 & (*opaque).elcr_mask; }
unsafe fn elcr_ioport_read(opaque: *mut kvm_kpic_state) -> u32 { (*opaque).elcr as u32 }

unsafe fn pic_irq_request(kvm: *mut kvm, level: c_int) { let s = (*(*kvm).arch).vpic; if !(*s).output && level != 0 { (*s).wakeup_needed = true; } (*s).output = level != 0; }

pub unsafe fn kvm_pic_init(kvm: *mut kvm) -> c_int { let s = kzalloc_obj::<kvm_pic>(GFP_KERNEL_ACCOUNT); if s.is_null() { return -ENOMEM; } spin_lock_init(&mut (*s).lock); (*s).kvm = kvm; (*s).pics[0].elcr_mask = 0xf8; (*s).pics[1].elcr_mask = 0xde; (*s).pics[0].pics_state = s; (*s).pics[1].pics_state = s; kvm_iodevice_init(&mut (*s).dev_master, &picdev_master_ops); kvm_iodevice_init(&mut (*s).dev_slave, &picdev_slave_ops); kvm_iodevice_init(&mut (*s).dev_elcr, &picdev_elcr_ops); mutex_lock(&mut (*kvm).slots_lock); let mut ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0x20, 2, &mut (*s).dev_master); if ret < 0 { mutex_unlock(&mut (*kvm).slots_lock); kfree(s); return ret; } ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0xa0, 2, &mut (*s).dev_slave); if ret < 0 { kvm_io_bus_unregister_dev(kvm, KVM_PIO_BUS, &mut (*s).dev_master); mutex_unlock(&mut (*kvm).slots_lock); kfree(s); return ret; } ret = kvm_io_bus_register_dev(kvm, KVM_PIO_BUS, 0x4d0, 2, &mut (*s).dev_elcr); if ret < 0 { kvm_io_bus_unregister_dev(kvm, KVM_PIO_BUS, &mut (*s).dev_slave); kvm_io_bus_unregister_dev(kvm, KVM_PIO_BUS, &mut (*s).dev_master); mutex_unlock(&mut (*kvm).slots_lock); kfree(s); return ret; } mutex_unlock(&mut (*kvm).slots_lock); (*(*kvm).arch).vpic = s; 0 }

pub unsafe fn kvm_pic_destroy(kvm: *mut kvm) { let s = (*(*kvm).arch).vpic; if s.is_null() { return; } mutex_lock(&mut (*kvm).slots_lock); kvm_io_bus_unregister_dev((*s).kvm, KVM_PIO_BUS, &mut (*s).dev_master); kvm_io_bus_unregister_dev((*s).kvm, KVM_PIO_BUS, &mut (*s).dev_slave); kvm_io_bus_unregister_dev((*s).kvm, KVM_PIO_BUS, &mut (*s).dev_elcr); mutex_unlock(&mut (*kvm).slots_lock); (*(*kvm).arch).vpic = core::ptr::null_mut(); kfree(s); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
