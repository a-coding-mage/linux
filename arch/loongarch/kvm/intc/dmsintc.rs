// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Loongson Technology Corporation Limited
 */

// Kernel dependencies supplied by the surrounding tree.
use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::sync::atomic::{AtomicU64, Ordering};

extern "C" {
    fn kvm_read_hw_gcsr(reg: c_uint) -> c_ulong;
    fn kvm_write_hw_gcsr(reg: c_uint, value: c_ulong);
    fn kvm_guest_has_msgint(arch: *const kvm_arch) -> bool;
    fn kvm_queue_irq(vcpu: *mut kvm_vcpu, irq: c_uint);
    fn kvm_vcpu_kick(vcpu: *mut kvm_vcpu);
    fn kvm_get_vcpu_by_cpuid(kvm: *mut kvm, cpu: c_uint) -> *mut kvm_vcpu;
    fn copy_from_user(to: *mut c_void, from: *const c_void, size: usize) -> usize;
    fn find_first_bit(addr: *const c_ulong, bits: usize) -> usize;
    fn kvm_pr_unimpl(fmt: *const u8, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kvm_register_device_ops(ops: *const kvm_device_ops, typ: c_uint) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const KVM_MAX_VCPUS: c_uint = 256;
const INT_AVEC: c_uint = 0;
const AVEC_IRQ_SHIFT: c_uint = 0;
const AVEC_IRQ_MASK: c_ulong = 0xff;
const AVEC_CPU_SHIFT: c_uint = 8;
const AVEC_CPU_BIT: c_ulong = 8;
const AVEC_CPU_MASK: c_ulong = 0xff;
const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_BASE: c_int = 0;
const KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_SIZE: c_int = 1;
const KVM_DEV_LOONGARCH_DMSINTC_GRP_CTRL: c_uint = 0;
const KVM_DEV_TYPE_LOONGARCH_DMSINTC: c_uint = 0;
const LOONGARCH_CSR_ISR0: c_uint = 0;
const LOONGARCH_CSR_ISR1: c_uint = 1;
const LOONGARCH_CSR_ISR2: c_uint = 2;
const LOONGARCH_CSR_ISR3: c_uint = 3;

#[repr(C)]
pub struct dmsintc_state {
    pub vector_map: [AtomicU64; 4],
}

#[repr(C)]
pub struct kvm_arch {
    pub dmsintc: *mut loongarch_dmsintc,
    pub _dmsintc_state: dmsintc_state,
}

#[repr(C)]
pub struct kvm_vcpu {
    pub arch: kvm_arch,
}

#[repr(C)]
pub struct kvm {
    pub arch: kvm_arch,
}

#[repr(C)]
pub struct loongarch_dmsintc {
    pub kvm: *mut kvm,
    pub msg_addr_base: c_ulong,
    pub msg_addr_size: c_ulong,
    pub cpu_mask: c_ulong,
}

#[repr(C)]
pub struct kvm_device {
    pub kvm: *mut kvm,
}

#[repr(C)]
pub struct kvm_device_attr {
    pub group: c_uint,
    pub attr: c_ulong,
    pub addr: c_ulong,
}

#[repr(C)]
pub struct kvm_device_ops {
    pub name: *const u8,
    pub create: Option<unsafe extern "C" fn(*mut kvm_device, c_uint) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut kvm_device)>,
    pub set_attr: Option<unsafe extern "C" fn(*mut kvm_device, *mut kvm_device_attr) -> c_int>,
}

#[no_mangle]
pub unsafe extern "C" fn dmsintc_inject_irq(vcpu: *mut kvm_vcpu) {
    let mut vector = [0usize; 4];
    let ds = &mut (*vcpu).arch._dmsintc_state;

    for i in 0..4 {
        let old = ds.vector_map[i].load(Ordering::SeqCst);
        vector[i] = if old != 0 {
            ds.vector_map[i].swap(0, Ordering::SeqCst) as usize
        } else {
            0
        };
    }

    let regs = [LOONGARCH_CSR_ISR0, LOONGARCH_CSR_ISR1, LOONGARCH_CSR_ISR2, LOONGARCH_CSR_ISR3];
    for i in 0..4 {
        if vector[i] != 0 {
            let old = kvm_read_hw_gcsr(regs[i]);
            kvm_write_hw_gcsr(regs[i], vector[i] as c_ulong | old);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dmsintc_deliver_msi_to_vcpu(
    _kvm: *mut kvm,
    vcpu: *mut kvm_vcpu,
    vector: u32,
    level: c_int,
) -> c_int {
    if level == 0 { return 0; }
    if vcpu.is_null() || vector >= 256 { return -EINVAL; }
    if !kvm_guest_has_msgint(&(*vcpu).arch) { return -EINVAL; }
    (*vcpu).arch._dmsintc_state.vector_map[(vector / 64) as usize]
        .fetch_or(1u64 << (vector % 64), Ordering::SeqCst);
    kvm_queue_irq(vcpu, INT_AVEC);
    kvm_vcpu_kick(vcpu);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dmsintc_set_irq(kvm: *mut kvm, addr: u64, _data: c_int, level: c_int) -> c_int {
    let irq = ((addr >> AVEC_IRQ_SHIFT) as c_ulong & AVEC_IRQ_MASK) as u32;
    let cpu = ((addr >> AVEC_CPU_SHIFT) as c_ulong & (*(*kvm).arch.dmsintc).cpu_mask) as c_uint;
    if cpu >= KVM_MAX_VCPUS { return -EINVAL; }
    let vcpu = kvm_get_vcpu_by_cpuid(kvm, cpu);
    if vcpu.is_null() { return -EINVAL; }
    dmsintc_deliver_msi_to_vcpu(kvm, vcpu, irq, level)
}

unsafe fn kvm_dmsintc_ctrl_access(dev: *mut kvm_device, attr: *mut kvm_device_attr, is_write: bool) -> c_int {
    let addr = (*attr).attr as c_int;
    let data = (*attr).addr as *mut c_void;
    let s = (*(*dev).kvm).arch.dmsintc;
    let mut val: c_ulong = 0;
    match addr {
        KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_BASE => {
            if is_write {
                if copy_from_user((&mut val as *mut c_ulong).cast(), data, core::mem::size_of::<c_ulong>()) != 0 { return -EFAULT; }
                if (*s).msg_addr_base != 0 || val & ((1u64 << AVEC_CPU_SHIFT) - 1) != 0 { return -EINVAL; }
                (*s).msg_addr_base = val;
                let cpu_bit = find_first_bit((&(*s).msg_addr_base as *const c_ulong), 64).saturating_sub(AVEC_CPU_SHIFT as usize).min(AVEC_CPU_BIT as usize);
                (*s).cpu_mask = if cpu_bit == 0 { 0 } else { (1u64 << cpu_bit) - 1 } & AVEC_CPU_MASK;
            }
        }
        KVM_DEV_LOONGARCH_DMSINTC_MSG_ADDR_SIZE => {
            if is_write {
                if copy_from_user((&mut val as *mut c_ulong).cast(), data, core::mem::size_of::<c_ulong>()) != 0 { return -EFAULT; }
                if (*s).msg_addr_size != 0 { return -EFAULT; }
                (*s).msg_addr_size = val;
            }
        }
        _ => return -ENXIO,
    }
    0
}

unsafe extern "C" fn kvm_dmsintc_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> c_int {
    if (*attr).group == KVM_DEV_LOONGARCH_DMSINTC_GRP_CTRL {
        kvm_dmsintc_ctrl_access(dev, attr, true)
    } else { -EINVAL }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dmsintc_create(dev: *mut kvm_device, _typ: c_uint) -> c_int {
    if dev.is_null() { return -EINVAL; }
    let kvm = (*dev).kvm;
    if !(*kvm).arch.dmsintc.is_null() { return -EINVAL; }
    let s = kzalloc(core::mem::size_of::<loongarch_dmsintc>(), GFP_KERNEL) as *mut loongarch_dmsintc;
    if s.is_null() { return -ENOMEM; }
    (*s).kvm = kvm;
    (*kvm).arch.dmsintc = s;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_dmsintc_destroy(dev: *mut kvm_device) {
    if dev.is_null() || (*dev).kvm.is_null() || (*(*dev).kvm).arch.dmsintc.is_null() { return; }
    kfree((*(*dev).kvm).arch.dmsintc.cast::<c_void>());
    kfree(dev.cast::<c_void>());
}

#[no_mangle]
pub unsafe extern "C" fn kvm_loongarch_register_dmsintc_device() -> c_int {
    kvm_register_device_ops(&kvm_dmsintc_dev_ops, KVM_DEV_TYPE_LOONGARCH_DMSINTC)
}

static kvm_dmsintc_dev_ops: kvm_device_ops = kvm_device_ops {
    name: b"kvm-loongarch-dmsintc\0".as_ptr(),
    create: Some(kvm_dmsintc_create),
    destroy: Some(kvm_dmsintc_destroy),
    set_attr: Some(kvm_dmsintc_set_attr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
