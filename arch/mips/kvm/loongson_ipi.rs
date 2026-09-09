// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Loongson-3 Virtual IPI interrupt support.
 *
 * Copyright (C) 2019  Loongson Technologies, Inc.  All rights reserved.
 *
 * Authors: Chen Zhu <zhuchen@loongson.cn>
 * Authors: Huacai Chen <chenhc@lemote.com>
 */

// Dependency: <linux/kvm_host.h>
// Dependency: interrupt.h

const IPI_BASE: u64 = 0x3ff01000;

const CORE0_STATUS_OFF: u64 = 0x000;
const CORE0_EN_OFF: u64 = 0x004;
const CORE0_SET_OFF: u64 = 0x008;
const CORE0_CLEAR_OFF: u64 = 0x00c;
const CORE0_BUF_20: u64 = 0x020;
const CORE0_BUF_28: u64 = 0x028;
const CORE0_BUF_30: u64 = 0x030;
const CORE0_BUF_38: u64 = 0x038;

const CORE1_STATUS_OFF: u64 = 0x100;
const CORE1_EN_OFF: u64 = 0x104;
const CORE1_SET_OFF: u64 = 0x108;
const CORE1_CLEAR_OFF: u64 = 0x10c;
const CORE1_BUF_20: u64 = 0x120;
const CORE1_BUF_28: u64 = 0x128;
const CORE1_BUF_30: u64 = 0x130;
const CORE1_BUF_38: u64 = 0x138;

const CORE2_STATUS_OFF: u64 = 0x200;
const CORE2_EN_OFF: u64 = 0x204;
const CORE2_SET_OFF: u64 = 0x208;
const CORE2_CLEAR_OFF: u64 = 0x20c;
const CORE2_BUF_20: u64 = 0x220;
const CORE2_BUF_28: u64 = 0x228;
const CORE2_BUF_30: u64 = 0x230;
const CORE2_BUF_38: u64 = 0x238;

const CORE3_STATUS_OFF: u64 = 0x300;
const CORE3_EN_OFF: u64 = 0x304;
const CORE3_SET_OFF: u64 = 0x308;
const CORE3_CLEAR_OFF: u64 = 0x30c;
const CORE3_BUF_20: u64 = 0x320;
const CORE3_BUF_28: u64 = 0x328;
const CORE3_BUF_30: u64 = 0x330;
const CORE3_BUF_38: u64 = 0x338;

unsafe fn loongson_vipi_read(
    ipi: *mut loongson_kvm_ipi,
    addr: gpa_t,
    len: i32,
    val: *mut core::ffi::c_void,
) -> i32 {
    let core = ((addr >> 8) & 3) as u32;
    let node = ((addr >> 44) & 3) as u32;
    let id = core + node * 4;
    let offset = addr & 0xff;
    let s = &mut (*ipi).ipistate[id as usize];

    BUG_ON(offset & ((len - 1) as u64));

    match offset {
        CORE0_STATUS_OFF => *(val as *mut u64) = s.status,
        CORE0_EN_OFF => *(val as *mut u64) = s.en,
        CORE0_SET_OFF | CORE0_CLEAR_OFF => *(val as *mut u64) = 0,
        CORE0_BUF_20..=CORE0_BUF_38 => {
            let pbuf = (s.buf as *mut u8).add((offset - 0x20) as usize);
            if len == 8 {
                *(val as *mut u64) = *(pbuf as *const u64);
            } else {
                *(val as *mut u32) = *(pbuf as *const u32);
            }
        }
        _ => pr_notice!("{} with unknown addr {:x}\n", "loongson_vipi_read", addr),
    }
    0
}

unsafe fn loongson_vipi_write(
    ipi: *mut loongson_kvm_ipi,
    addr: gpa_t,
    len: i32,
    val: *const core::ffi::c_void,
) -> i32 {
    let core = ((addr >> 8) & 3) as u32;
    let node = ((addr >> 44) & 3) as u32;
    let id = core + node * 4;
    let offset = addr & 0xff;
    let data = *(val as *const u64);
    let s = &mut (*ipi).ipistate[id as usize];
    let kvm = (*ipi).kvm;

    BUG_ON(offset & ((len - 1) as u64));

    match offset {
        CORE0_STATUS_OFF => {}
        CORE0_EN_OFF => s.en = data,
        CORE0_SET_OFF => {
            s.status |= data;
            let irq = kvm_mips_interrupt { cpu: id, irq: 6 };
            kvm_vcpu_ioctl_interrupt(kvm_get_vcpu(kvm, id), &irq);
        }
        CORE0_CLEAR_OFF => {
            s.status &= !data;
            if s.status == 0 {
                let irq = kvm_mips_interrupt { cpu: id, irq: -6 };
                kvm_vcpu_ioctl_interrupt(kvm_get_vcpu(kvm, id), &irq);
            }
        }
        CORE0_BUF_20..=CORE0_BUF_38 => {
            let pbuf = (s.buf as *mut u8).add((offset - 0x20) as usize);
            if len == 8 {
                *(pbuf as *mut u64) = data;
            } else {
                *(pbuf as *mut u32) = data as u32;
            }
        }
        _ => pr_notice!("{} with unknown addr {:x}\n", "loongson_vipi_write", addr),
    }
    0
}

unsafe fn kvm_ipi_read(
    _vcpu: *mut kvm_vcpu,
    dev: *mut kvm_io_device,
    addr: gpa_t,
    len: i32,
    val: *mut core::ffi::c_void,
) -> i32 {
    let ipi_device = container_of!(dev, ipi_io_device, device);
    let ipi = (*ipi_device).ipi;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave!(&mut (*ipi).lock, &mut flags);
    loongson_vipi_read(ipi, addr, len, val);
    spin_unlock_irqrestore!(&mut (*ipi).lock, flags);
    0
}

unsafe fn kvm_ipi_write(
    _vcpu: *mut kvm_vcpu,
    dev: *mut kvm_io_device,
    addr: gpa_t,
    len: i32,
    val: *const core::ffi::c_void,
) -> i32 {
    let ipi_device = container_of!(dev, ipi_io_device, device);
    let ipi = (*ipi_device).ipi;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave!(&mut (*ipi).lock, &mut flags);
    loongson_vipi_write(ipi, addr, len, val);
    spin_unlock_irqrestore!(&mut (*ipi).lock, flags);
    0
}

static kvm_ipi_ops: kvm_io_device_ops = kvm_io_device_ops {
    read: Some(kvm_ipi_read),
    write: Some(kvm_ipi_write),
};

unsafe fn kvm_init_loongson_ipi(kvm: *mut kvm) {
    let s = &mut (*kvm).arch.ipi;
    s.kvm = kvm;
    spin_lock_init!(&mut s.lock);

    /* Initialize IPI device */
    for i in 0..4 {
        let device = &mut s.dev_ipi[i].device;
        kvm_iodevice_init!(device, &kvm_ipi_ops);
        let addr = ((i as u64) << 44) + IPI_BASE;
        mutex_lock!(&mut (*kvm).slots_lock);
        kvm_io_bus_register_dev(kvm, KVM_MMIO_BUS, addr, 0x400, device);
        mutex_unlock!(&mut (*kvm).slots_lock);
        s.dev_ipi[i].ipi = s;
        s.dev_ipi[i].node_id = i as i32;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
