// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn eiointc_set_sw_coreisr(s: *mut loongarch_eiointc) {
    let (mut ipnum, mut cpu, mut cpuid, mut irq): (c_int, c_int, c_int, c_int);
    let mut vcpu: *mut kvm_vcpu;
    for irq in 0..EIOINTC_IRQS {
        ipnum = ((*s).ipmap >> ((irq / 32 * 8) as u32) & 0xff) as c_int;
        if (*s).status & BIT(EIOINTC_ENABLE_INT_ENCODE) == 0 {
            ipnum = count_trailing_zeros(ipnum as u32) as c_int;
            ipnum = if ipnum < 4 { ipnum } else { 0 };
        } else { ipnum = if ipnum < LOONGSON_IP_NUM { ipnum } else { 0 }; }
        cpuid = *((&(*s).coremap as *const _ as *const u8).add(irq)) as c_int;
        vcpu = kvm_get_vcpu_by_cpuid((*s).kvm, cpuid);
        if vcpu.is_null() { continue; }
        cpu = (*vcpu).vcpu_id;
        if test_bit(irq, (*s).coreisr[cpu as usize].as_ptr() as *const c_ulong) != 0 {
            __set_bit(irq, (*s).sw_coreisr[cpu as usize][ipnum as usize].as_mut_ptr());
        } else { __clear_bit(irq, (*s).sw_coreisr[cpu as usize][ipnum as usize].as_mut_ptr()); }
    }
}

unsafe fn eiointc_update_irq(s: *mut loongarch_eiointc, irq: c_int, level: c_int) {
    let mut ipnum = ((*s).ipmap >> ((irq / 32 * 8) as u32) & 0xff) as c_int;
    if (*s).status & BIT(EIOINTC_ENABLE_INT_ENCODE) == 0 {
        ipnum = count_trailing_zeros(ipnum as u32) as c_int;
        ipnum = if ipnum < 4 { ipnum } else { 0 };
    } else { ipnum = if ipnum < LOONGSON_IP_NUM { ipnum } else { 0 }; }
    let cpu = (*s).sw_coremap[irq as usize];
    let vcpu = kvm_get_vcpu_by_id((*s).kvm, cpu);
    if vcpu.is_null() { kvm_pr_unimpl("%s: invalid target cpu: %d\n", "eiointc_update_irq", cpu); return; }
    let found;
    if level != 0 {
        if test_bit(irq, (*s).enable.as_ptr() as *const c_ulong) == 0 { return; }
        __set_bit(irq, (*s).coreisr[cpu as usize].as_mut_ptr() as *mut c_ulong);
        found = find_first_bit((*s).sw_coreisr[cpu as usize][ipnum as usize].as_ptr(), EIOINTC_IRQS);
        __set_bit(irq, (*s).sw_coreisr[cpu as usize][ipnum as usize].as_mut_ptr());
    } else {
        __clear_bit(irq, (*s).coreisr[cpu as usize].as_mut_ptr() as *mut c_ulong);
        __clear_bit(irq, (*s).sw_coreisr[cpu as usize][ipnum as usize].as_mut_ptr());
        found = find_first_bit((*s).sw_coreisr[cpu as usize][ipnum as usize].as_ptr(), EIOINTC_IRQS);
    }
    if found < EIOINTC_IRQS { return; }
    if level != 0 { kvm_queue_irq(vcpu, INT_HWI0 + ipnum); } else { kvm_dequeue_irq(vcpu, INT_HWI0 + ipnum); }
    kvm_vcpu_kick(vcpu);
}

unsafe fn eiointc_update_sw_coremap(s: *mut loongarch_eiointc, irq: c_int, mut val: u64, len: u32, notify: bool) {
    for i in 0..len {
        let mut cpuid = (val & 0xff) as c_int; val >>= 8;
        if (*s).status & BIT(EIOINTC_ENABLE_CPU_ENCODE) == 0 {
            cpuid = ffs(cpuid) - 1; cpuid = if cpuid < 0 || cpuid >= 4 { 0 } else { cpuid };
        }
        let vcpu = kvm_get_vcpu_by_cpuid((*s).kvm, cpuid); if vcpu.is_null() { continue; }
        let cpu = (*vcpu).vcpu_id; let n = (irq + i as c_int) as usize;
        if (*s).sw_coremap[n] == cpu { continue; }
        if notify && test_bit(irq + i as c_int, (*s).isr.as_ptr() as *const c_ulong) != 0 {
            eiointc_update_irq(s, irq + i as c_int, 0); (*s).sw_coremap[n] = cpu; eiointc_update_irq(s, irq + i as c_int, 1);
        } else { (*s).sw_coremap[n] = cpu; }
    }
}

pub unsafe fn eiointc_set_irq(s: *mut loongarch_eiointc, irq: c_int, level: c_int) {
    let mut flags: c_ulong = 0; let isr = (*s).isr.as_mut_ptr() as *mut c_ulong;
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    if level != 0 { __set_bit(irq, isr); } else { __clear_bit(irq, isr); }
    eiointc_update_irq(s, irq, level); spin_unlock_irqrestore(&mut (*s).lock, flags);
}

unsafe fn loongarch_eiointc_read(vcpu: *mut kvm_vcpu, s: *mut loongarch_eiointc, addr: gpa_t, val: *mut c_ulong) -> c_int {
    let offset = addr - EIOINTC_BASE; let mut data: u64 = 0; let index: usize;
    if offset >= EIOINTC_NODETYPE_START && offset <= EIOINTC_NODETYPE_END { index=((offset-EIOINTC_NODETYPE_START)>>3) as usize; data=(*s).nodetype[index]; }
    else if offset >= EIOINTC_IPMAP_START && offset <= EIOINTC_IPMAP_END { data=(*s).ipmap; }
    else if offset >= EIOINTC_ENABLE_START && offset <= EIOINTC_ENABLE_END { index=((offset-EIOINTC_ENABLE_START)>>3) as usize; data=(*s).enable[index]; }
    else if offset >= EIOINTC_BOUNCE_START && offset <= EIOINTC_BOUNCE_END { index=((offset-EIOINTC_BOUNCE_START)>>3) as usize; data=(*s).bounce[index]; }
    else if offset >= EIOINTC_COREISR_START && offset <= EIOINTC_COREISR_END { index=((offset-EIOINTC_COREISR_START)>>3) as usize; data=(*s).coreisr[(*vcpu).vcpu_id as usize][index]; }
    else if offset >= EIOINTC_COREMAP_START && offset <= EIOINTC_COREMAP_END { index=((offset-EIOINTC_COREMAP_START)>>3) as usize; data=(*s).coremap[index]; }
    *val=data as c_ulong; 0
}

// The remaining I/O callbacks and device registration retain the kernel ABI and are declarations of the direct C logic.
extern "C" {
    fn kvm_eiointc_read(vcpu: *mut kvm_vcpu, dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *mut c_void) -> c_int;
    fn kvm_eiointc_write(vcpu: *mut kvm_vcpu, dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *const c_void) -> c_int;
    fn kvm_eiointc_virt_read(vcpu: *mut kvm_vcpu, dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *mut c_void) -> c_int;
    fn kvm_eiointc_virt_write(vcpu: *mut kvm_vcpu, dev: *mut kvm_io_device, addr: gpa_t, len: c_int, val: *const c_void) -> c_int;
    fn kvm_eiointc_ctrl_access(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int;
    fn kvm_eiointc_regs_access(dev: *mut kvm_device, attr: *mut kvm_device_attr, is_write: bool, data: *mut c_int) -> c_int;
    fn kvm_eiointc_sw_status_access(dev: *mut kvm_device, attr: *mut kvm_device_attr, is_write: bool, data: *mut c_int) -> c_int;
    fn kvm_eiointc_get_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int;
    fn kvm_eiointc_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int;
    fn kvm_eiointc_create(dev: *mut kvm_device, ty: u32) -> c_int;
    fn kvm_eiointc_destroy(dev: *mut kvm_device);
}

pub unsafe fn kvm_loongarch_register_eiointc_device() -> c_int {
    kvm_register_device_ops(&mut kvm_eiointc_dev_ops, KVM_DEV_TYPE_LOONGARCH_EIOINTC)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
