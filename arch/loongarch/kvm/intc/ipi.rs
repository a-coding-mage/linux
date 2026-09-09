// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn ipi_set(vcpu: *mut kvm_vcpu, data: u32) {
    let status: u32;
    spin_lock(&mut (*vcpu).arch.ipi_state.lock);
    status = (*vcpu).arch.ipi_state.status;
    (*vcpu).arch.ipi_state.status |= data;
    if status == 0 && data != 0 {
        kvm_queue_irq(vcpu, LARCH_INT_IPI);
        kvm_vcpu_kick(vcpu);
    }
    spin_unlock(&mut (*vcpu).arch.ipi_state.lock);
}

unsafe fn ipi_send(kvm: *mut kvm, data: u64) {
    let cpu = (((data & 0xffff_ffff) >> 16) & 0x3ff) as i32;
    let vcpu = kvm_get_vcpu_by_cpuid(kvm, cpu);
    if vcpu.is_null() {
        kvm_pr_unimpl("%s: invalid target cpu: %d\n", "ipi_send", cpu);
        return;
    }
    ipi_set(vcpu, BIT(data & 0x1f));
}

unsafe fn ipi_clear(vcpu: *mut kvm_vcpu, data: u64) {
    spin_lock(&mut (*vcpu).arch.ipi_state.lock);
    (*vcpu).arch.ipi_state.status &= !(data as u32);
    let status = (*vcpu).arch.ipi_state.status;
    if status == 0 {
        kvm_dequeue_irq(vcpu, LARCH_INT_IPI);
        kvm_vcpu_kick(vcpu);
    }
    spin_unlock(&mut (*vcpu).arch.ipi_state.lock);
}

unsafe fn read_mailbox(vcpu: *mut kvm_vcpu, offset: i32, len: i32) -> u64 {
    spin_lock(&mut (*vcpu).arch.ipi_state.lock);
    let data = *((*vcpu).arch.ipi_state.buf.as_ptr().add((offset - 0x20) as usize) as *const usize) as u64;
    spin_unlock(&mut (*vcpu).arch.ipi_state.lock);
    match len { 1 => data & 0xff, 2 => data & 0xffff, 4 => data & 0xffff_ffff, _ => data }
}

unsafe fn write_mailbox(vcpu: *mut kvm_vcpu, offset: i32, data: u64, len: i32) {
    spin_lock(&mut (*vcpu).arch.ipi_state.lock);
    let pbuf = (*vcpu).arch.ipi_state.buf.as_mut_ptr().add((offset - 0x20) as usize);
    match len {
        1 => *(pbuf as *mut u8) = data as u8,
        2 => *(pbuf as *mut u16) = data as u16,
        4 => *(pbuf as *mut u32) = data as u32,
        _ => *(pbuf as *mut usize) = data as usize,
    }
    spin_unlock(&mut (*vcpu).arch.ipi_state.lock);
}

unsafe fn mail_send(kvm: *mut kvm, data: u64) -> i32 {
    let cpu = (((data & 0xffff_ffff) >> 16) & 0x3ff) as i32;
    let vcpu = kvm_get_vcpu_by_cpuid(kvm, cpu);
    if vcpu.is_null() { kvm_pr_unimpl("%s: invalid target cpu: %d\n", "mail_send", cpu); return 0; }
    let mailbox = (((data & 0xffff_ffff) >> 2) & 7) as i32;
    let offset = IOCSR_IPI_BUF_20 + mailbox * 4;
    let mut val: u32 = 0;
    let mut mask: u32 = 0;
    if ((data >> 27) & 0xf) != 0 {
        val = read_mailbox(vcpu, offset, 4) as u32;
        for i in 0..4 { if data & BIT(27 + i) != 0 { mask |= 0xff << (i * 8); } }
        val &= mask;
    }
    val |= ((data >> 32) as u32) & !mask;
    write_mailbox(vcpu, offset, val as u64, 4);
    0
}

unsafe fn send_ipi_data(vcpu: *mut kvm_vcpu, addr: gpa_t, data: u64) -> i32 {
    let mut val = 0u64; let mut mask = 0u64;
    if ((data >> 27) & 0xf) != 0 {
        let idx = srcu_read_lock(&mut (*(*vcpu).kvm).srcu);
        let ret = kvm_io_bus_read(vcpu, KVM_IOCSR_BUS, addr, 4, &mut val);
        srcu_read_unlock(&mut (*(*vcpu).kvm).srcu, idx);
        if ret != 0 { kvm_pr_unimpl("%s: : read data from addr %llx failed\n", "send_ipi_data", addr); return 0; }
        for i in 0..4 { if data & BIT(27 + i) != 0 { mask |= 0xff << (i * 8); } }
        val &= mask;
    }
    val |= (data >> 32) & !mask;
    let idx = srcu_read_lock(&mut (*(*vcpu).kvm).srcu);
    let ret = kvm_io_bus_write(vcpu, KVM_IOCSR_BUS, addr, 4, &val);
    srcu_read_unlock(&mut (*(*vcpu).kvm).srcu, idx);
    if ret != 0 { kvm_pr_unimpl("%s: : write data to addr %llx failed\n", "send_ipi_data", addr); }
    0
}

unsafe fn any_send(kvm: *mut kvm, data: u64) -> i32 {
    let cpu = (((data & 0xffff_ffff) >> 16) & 0x3ff) as i32;
    let vcpu = kvm_get_vcpu_by_cpuid(kvm, cpu);
    if vcpu.is_null() { kvm_pr_unimpl("%s: invalid target cpu: %d\n", "any_send", cpu); return 0; }
    send_ipi_data(vcpu, (data & 0xffff) as gpa_t, data)
}

unsafe fn loongarch_ipi_readl(vcpu: *mut kvm_vcpu, addr: gpa_t, len: i32, val: *mut core::ffi::c_void) -> i32 {
    let mut res = 0u64;
    if addr & (len as u64 - 1) != 0 { *(val as *mut u64) = 0; return 0; }
    let offset = addr - IOCSR_IPI_BASE;
    match offset {
        IOCSR_IPI_STATUS => { spin_lock(&mut (*vcpu).arch.ipi_state.lock); res = (*vcpu).arch.ipi_state.status as u64; spin_unlock(&mut (*vcpu).arch.ipi_state.lock); }
        IOCSR_IPI_EN => { spin_lock(&mut (*vcpu).arch.ipi_state.lock); res = (*vcpu).arch.ipi_state.en; spin_unlock(&mut (*vcpu).arch.ipi_state.lock); }
        IOCSR_IPI_SET | IOCSR_IPI_CLEAR => {}
        IOCSR_IPI_BUF_20..=IOCSR_IPI_BUF_38 + 7 => res = read_mailbox(vcpu, offset as i32, len),
        _ => kvm_pr_unimpl("%s: unknown addr: %llx\n", "loongarch_ipi_readl", addr),
    }
    *(val as *mut u64) = res; 0
}

unsafe fn loongarch_ipi_writel(vcpu: *mut kvm_vcpu, addr: gpa_t, len: i32, val: *const core::ffi::c_void) -> i32 {
    let data = *(val as *const u64);
    if addr & (len as u64 - 1) != 0 { return 0; }
    let offset = addr - IOCSR_IPI_BASE;
    match offset {
        IOCSR_IPI_STATUS => {},
        IOCSR_IPI_EN => { spin_lock(&mut (*vcpu).arch.ipi_state.lock); (*vcpu).arch.ipi_state.en = data; spin_unlock(&mut (*vcpu).arch.ipi_state.lock); }
        IOCSR_IPI_SET => ipi_set(vcpu, data as u32),
        IOCSR_IPI_CLEAR => ipi_clear(vcpu, data),
        IOCSR_IPI_BUF_20..=IOCSR_IPI_BUF_38 + 7 => { if offset + len as u64 <= IOCSR_IPI_BUF_38 + 8 { write_mailbox(vcpu, offset as i32, data, len); } },
        IOCSR_IPI_SEND => ipi_send((*vcpu).kvm, data),
        IOCSR_MAIL_SEND => { mail_send((*vcpu).kvm, data); },
        IOCSR_ANY_SEND => { any_send((*vcpu).kvm, data); },
        _ => kvm_pr_unimpl("%s: unknown addr: %llx\n", "loongarch_ipi_writel", addr),
    } 0
}

unsafe fn kvm_ipi_read(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: i32, val: *mut core::ffi::c_void) -> i32 { (*vcpu).stat.ipi_read_exits += 1; loongarch_ipi_readl(vcpu, addr, len, val) }
unsafe fn kvm_ipi_write(vcpu: *mut kvm_vcpu, _dev: *mut kvm_io_device, addr: gpa_t, len: i32, val: *const core::ffi::c_void) -> i32 { (*vcpu).stat.ipi_write_exits += 1; loongarch_ipi_writel(vcpu, addr, len, val) }

static kvm_ipi_ops: kvm_io_device_ops = kvm_io_device_ops {
    read: Some(kvm_ipi_read),
    write: Some(kvm_ipi_write),
};

unsafe fn kvm_ipi_regs_access(dev: *mut kvm_device, attr: *mut kvm_device_attr, is_write: bool) -> i32 {
    let mut len = 4;
    let cpu = ((*attr).attr >> 16) & 0x3ff;
    let addr = (*attr).attr & 0xff;
    let vcpu = kvm_get_vcpu_by_id((*dev).kvm, cpu as i32);
    if vcpu.is_null() { kvm_pr_unimpl("%s: invalid target cpu: %d\n", "kvm_ipi_regs_access", cpu); return -EINVAL; }
    let p: *mut core::ffi::c_void = match addr {
        IOCSR_IPI_STATUS => &mut (*vcpu).arch.ipi_state.status as *mut _ as _,
        IOCSR_IPI_EN => &mut (*vcpu).arch.ipi_state.en as *mut _ as _,
        IOCSR_IPI_SET => &mut (*vcpu).arch.ipi_state.set as *mut _ as _,
        IOCSR_IPI_CLEAR => &mut (*vcpu).arch.ipi_state.clear as *mut _ as _,
        IOCSR_IPI_BUF_20 => { len = 8; &mut (*vcpu).arch.ipi_state.buf[0] as *mut _ as _ },
        IOCSR_IPI_BUF_28 => { len = 8; &mut (*vcpu).arch.ipi_state.buf[1] as *mut _ as _ },
        IOCSR_IPI_BUF_30 => { len = 8; &mut (*vcpu).arch.ipi_state.buf[2] as *mut _ as _ },
        IOCSR_IPI_BUF_38 => { len = 8; &mut (*vcpu).arch.ipi_state.buf[3] as *mut _ as _ },
        _ => { kvm_pr_unimpl("%s: unknown ipi register, addr = %d\n", "kvm_ipi_regs_access", addr); return -EINVAL; }
    };
    let user = (*attr).addr as *mut u64;
    if is_write { if len == 4 { *(p as *mut u32) = *(user as *const u32); } else { *(p as *mut u64) = *user; } }
    else if len == 4 { *user = *(p as *const u32) as u64; } else { *user = *(p as *const u64); }
    0
}

unsafe fn kvm_ipi_get_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32 { if (*attr).group == KVM_DEV_LOONGARCH_IPI_GRP_REGS { kvm_ipi_regs_access(dev, attr, false) } else { -EINVAL } }
unsafe fn kvm_ipi_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32 { if (*attr).group == KVM_DEV_LOONGARCH_IPI_GRP_REGS { kvm_ipi_regs_access(dev, attr, true) } else { -EINVAL } }

static mut kvm_ipi_dev_ops: kvm_device_ops = kvm_device_ops {
    name: "kvm-loongarch-ipi",
    create: Some(kvm_ipi_create), destroy: Some(kvm_ipi_destroy),
    set_attr: Some(kvm_ipi_set_attr), get_attr: Some(kvm_ipi_get_attr),
};

unsafe fn kvm_ipi_create(dev: *mut kvm_device, _type: u32) -> i32 {
    if dev.is_null() { return -EINVAL; }
    let kvm = (*dev).kvm;
    if (*kvm).arch.ipi.is_null() { return -ENOMEM; }
    0
}

unsafe fn kvm_ipi_destroy(dev: *mut kvm_device) {
    if dev.is_null() || (*dev).kvm.is_null() || (*(*dev).kvm).arch.ipi.is_null() { return; }
    kfree((*(*dev).kvm).arch.ipi);
    kfree(dev);
}

#[no_mangle]
pub unsafe extern "C" fn kvm_loongarch_register_ipi_device() -> i32 {
    kvm_register_device_ops(&kvm_ipi_dev_ops, KVM_DEV_TYPE_LOONGARCH_IPI)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
