// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

/* update the isr according to irq level and route irq to eiointc */
unsafe fn pch_pic_update_irq(s: *mut loongarch_pch_pic, mut irq: c_int, level: c_int) {
    let mask: u64 = BIT(irq);

    /*
     * set isr and route irq to eiointc and
     * the route table is in htmsi_vector[]
     */
    if level != 0 {
        if (mask & (*s).irr & !(*s).mask) != 0 {
            (*s).isr |= mask;
            irq = (*s).htmsi_vector[irq as usize] as c_int;
            eiointc_set_irq((*(*s).kvm).arch.eiointc, irq, level);
        }
    } else if (mask & (*s).isr & !(*s).irr) != 0 {
        (*s).isr &= !mask;
        irq = (*s).htmsi_vector[irq as usize] as c_int;
        eiointc_set_irq((*(*s).kvm).arch.eiointc, irq, level);
    }
}

/* update batch irqs, the irq_mask is a bitmap of irqs */
unsafe fn pch_pic_update_batch_irqs(s: *mut loongarch_pch_pic, irq_mask: u64, level: c_int) {
    let mut irq: c_uint = 0;
    let mut irqs = irq_mask;
    while irqs != 0 {
        irq = irqs.trailing_zeros();
        pch_pic_update_irq(s, irq as c_int, level);
        irqs &= irqs - 1;
    }
}

/* called when a irq is triggered in pch pic */
pub unsafe fn pch_pic_set_irq(s: *mut loongarch_pch_pic, irq: c_int, level: c_int) {
    let mask: u64 = BIT(irq);

    spin_lock(&mut (*s).lock);
    if level != 0 {
        (*s).irr |= mask;
    } else {
        /*
         * In edge triggered mode, 0 does not mean to clear irq
         * The irr register variable is cleared when cpu writes to the
         * PCH_PIC_CLEAR_START address area
         */
        if ((*s).edge & mask) != 0 {
            spin_unlock(&mut (*s).lock);
            return;
        }
        (*s).irr &= !mask;
    }
    pch_pic_update_irq(s, irq, level);
    spin_unlock(&mut (*s).lock);
}

/* msi irq handler */
pub unsafe fn pch_msi_set_irq(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry, level: c_int) -> c_int {
    let msg_addr: u64 = (((*e).msi.address_hi as u64) << 32) | (*e).msi.address_lo as u64;

    if cpu_has_msgint && !(*kvm).arch.dmsintc.is_null()
        && msg_addr >= (*(*kvm).arch.dmsintc).msg_addr_base
        && msg_addr < (*(*kvm).arch.dmsintc).msg_addr_base + (*(*kvm).arch.dmsintc).msg_addr_size {
        return dmsintc_set_irq((*kvm).arch.dmsintc, msg_addr, (*e).msi.data, level);
    }
    eiointc_set_irq((*kvm).arch.eiointc, (*e).msi.data, level);
    0
}

unsafe fn loongarch_pch_pic_read(s: *mut loongarch_pch_pic, addr: gpa_t, len: c_int, val: *mut c_void) -> c_int {
    let mut offset: c_int = addr.wrapping_sub((*s).pch_pic_base) as c_int;
    let mut data: u64 = 0;
    let mut ptemp: *mut c_void;
    offset -= offset & 7;
    spin_lock(&mut (*s).lock);
    match offset {
        PCH_PIC_INT_ID_START..=PCH_PIC_INT_ID_END => data = (*s).id.data,
        PCH_PIC_MASK_START..=PCH_PIC_MASK_END => data = (*s).mask,
        PCH_PIC_HTMSI_EN_START..=PCH_PIC_HTMSI_EN_END => { /* read htmsi enable reg */ data = (*s).htmsi_en; }
        PCH_PIC_EDGE_START..=PCH_PIC_EDGE_END => { /* read edge enable reg */ data = (*s).edge; }
        PCH_PIC_AUTO_CTRL0_START..=PCH_PIC_AUTO_CTRL0_END | PCH_PIC_AUTO_CTRL1_START..=PCH_PIC_AUTO_CTRL1_END => { /* fixed interrupt distribution mode */ }
        PCH_PIC_ROUTE_ENTRY_START..=PCH_PIC_ROUTE_ENTRY_END => {
            /* only route to int0: eiointc */
            ptemp = ((*s).route_entry.as_mut_ptr() as *mut u8).add((offset - PCH_PIC_ROUTE_ENTRY_START) as usize) as *mut c_void;
            data = *(ptemp as *mut u64);
        }
        PCH_PIC_HTMSI_VEC_START..=PCH_PIC_HTMSI_VEC_END => {
            /* read htmsi vector */
            ptemp = ((*s).htmsi_vector.as_mut_ptr() as *mut u8).add((offset - PCH_PIC_HTMSI_VEC_START) as usize) as *mut c_void;
            data = *(ptemp as *mut u64);
        }
        PCH_PIC_POLARITY_START..=PCH_PIC_POLARITY_END => data = (*s).polarity,
        PCH_PIC_INT_IRR_START => data = (*s).irr,
        PCH_PIC_INT_ISR_START => data = (*s).isr,
        _ => {}
    }
    spin_unlock(&mut (*s).lock);
    offset = (addr.wrapping_sub((*s).pch_pic_base) & 7) as c_int;
    data >>= (offset * 8) as u32;
    memcpy(val, &data as *const u64 as *const c_void, len as usize);
    0
}

unsafe fn kvm_pch_pic_read(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *mut c_void) -> c_int {
    let s = (*vcpu).kvm.arch.pch_pic;
    if s.is_null() { kvm_pr_unimpl("%s: pch pic irqchip not valid!\n", "kvm_pch_pic_read"); return 0; }
    if addr & (len as u64 - 1) != 0 { kvm_pr_unimpl("%s: pch pic not aligned addr %llx len %d\n", "kvm_pch_pic_read", addr, len); return 0; }
    (*vcpu).stat.pch_pic_read_exits += 1;
    loongarch_pch_pic_read(s, addr, len, val)
}

unsafe fn loongarch_pch_pic_write(s: *mut loongarch_pch_pic, addr: gpa_t, len: c_int, val: *const c_void) -> c_int {
    let (mut data, mut mask) = match len { 1 => (*(val as *const u8) as u64, 0xff), 2 => (*(val as *const u16) as u64, USHRT_MAX as u64), 4 => (*(val as *const u32) as u64, UINT_MAX as u64), _ => (*(val as *const u64, ULONG_MAX as u64)) };
    let mut offset = (addr.wrapping_sub((*s).pch_pic_base) & 7) as c_int;
    mask <<= (offset * 8) as u32; data <<= (offset * 8) as u32;
    offset = addr.wrapping_sub((*s).pch_pic_base) as c_int - offset;
    spin_lock(&mut (*s).lock);
    match offset {
        PCH_PIC_MASK_START => { let old=(*s).mask; (*s).mask=(old&!mask)|data; if old&!data != 0 { pch_pic_update_batch_irqs(s,old&!data,1); } if !old&data != 0 { pch_pic_update_batch_irqs(s,!old&data,0); } }
        PCH_PIC_HTMSI_EN_START => (*s).htmsi_en=((*s).htmsi_en&!mask)|data,
        PCH_PIC_EDGE_START => (*s).edge=((*s).edge&!mask)|data,
        PCH_PIC_POLARITY_START => (*s).polarity=((*s).polarity&!mask)|data,
        PCH_PIC_CLEAR_START => { let old=(*s).irr&(*s).edge&data; if old != 0 { (*s).irr &= !old; pch_pic_update_batch_irqs(s,old,0); } }
        PCH_PIC_HTMSI_VEC_START..=PCH_PIC_HTMSI_VEC_END => { let p=((*s).htmsi_vector.as_mut_ptr() as *mut u8).add((offset-PCH_PIC_HTMSI_VEC_START) as usize) as *mut u64; *p=(*p&!mask)|data; }
        _ => {}
    }
    spin_unlock(&mut (*s).lock); 0
}

unsafe fn kvm_pch_pic_write(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *const c_void) -> c_int {
    let s=(*vcpu).kvm.arch.pch_pic; if s.is_null() { kvm_pr_unimpl("%s: pch pic irqchip not valid!\n", "kvm_pch_pic_write"); return 0; }
    if addr&(len as u64-1)!=0 { kvm_pr_unimpl("%s: pch pic not aligned addr %llx len %d\n", "kvm_pch_pic_write",addr,len); return 0; }
    (*vcpu).stat.pch_pic_write_exits+=1; loongarch_pch_pic_write(s,addr,len,val)
}

pub unsafe fn kvm_loongarch_register_pch_pic_device() -> c_int {
    kvm_register_device_ops(&mut kvm_pch_pic_dev_ops, KVM_DEV_TYPE_LOONGARCH_PCHPIC)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
