// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/sys_wildfire.c
 *
 *  Wildfire support.
 *
 *  Copyright (C) 2000 Andrea Arcangeli <andrea@suse.de> SuSE
 */

// Kernel and architecture headers supplying the external symbols below.

static mut CACHED_IRQ_MASK: [::core::ffi::c_ulong; WILDFIRE_NR_IRQS / (::core::mem::size_of::<::core::ffi::c_ulong>() * 8)] =
    [0; WILDFIRE_NR_IRQS / (::core::mem::size_of::<::core::ffi::c_ulong>() * 8)];

extern "C" {
    static mut wildfire_irq_lock: spinlock_t;
    static mut doing_init_irq_hw: ::core::ffi::c_int;

    fn wildfire_pca(qbbno: ::core::ffi::c_int, pcano: ::core::ffi::c_int) -> *mut wildfire_pca;
    fn WILDFIRE_PCA_EXISTS(qbbno: ::core::ffi::c_int, pcano: ::core::ffi::c_int) -> bool;
    fn mb();
    fn printk(fmt: *const ::core::ffi::c_char, ...);
    fn i8259a_enable_irq(d: *mut irq_data);
    fn i8259a_disable_irq(d: *mut irq_data);
    fn i8259a_mask_and_ack_irq(d: *mut irq_data);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn set_bit(nr: ::core::ffi::c_uint, addr: *mut ::core::ffi::c_ulong);
    fn clear_bit(nr: ::core::ffi::c_uint, addr: *mut ::core::ffi::c_ulong);
    fn irq_set_chip_and_handler(irq: ::core::ffi::c_uint, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn irq_set_status_flags(irq: ::core::ffi::c_uint, flags: ::core::ffi::c_uint);
    fn handle_level_irq();
    fn request_irq(irq: ::core::ffi::c_uint, handler: unsafe extern "C" fn(), flags: ::core::ffi::c_uint, name: *const ::core::ffi::c_char, dev: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
    fn init_i8259a_irqs();
    fn handle_irq(irq: ::core::ffi::c_int);
    fn wildfire_machine_check();
    fn wildfire_init_arch();
    fn common_init_rtc();
    fn common_init_pci();
    fn wildfire_kill_arch();
    fn common_swizzle();
}

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { pub irq: ::core::ffi::c_uint }
#[repr(C)] pub struct irq_chip {
    pub name: *const ::core::ffi::c_char,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)] pub struct wildfire_pca { pub pca_int: [wildfire_pca_int; 4] }
#[repr(C)] pub struct wildfire_pca_int { pub enable: ::core::ffi::c_ulong, pub target: ::core::ffi::c_ulong }
#[repr(C)] pub struct pci_dev { pub sysdata: *mut ::core::ffi::c_void }
#[repr(C)] pub struct pci_controller { pub index: ::core::ffi::c_int }

#[repr(C)] pub struct alpha_machine_vector { _private: [u8; 0] }
extern "C" {
    static WILDFIRE_NR_IRQS: usize;
    static WILDFIRE_MAX_QBB: ::core::ffi::c_int;
    static WILDFIRE_PCA_PER_QBB: ::core::ffi::c_int;
    static WILDFIRE_IRQ_PER_PCA: ::core::ffi::c_int;
    static IRQ_LEVEL: ::core::ffi::c_uint;
    static COMMON_TABLE_LOOKUP: ::core::ffi::c_int;
    fn WILDFIRE_QBB_EXISTS(qbbno: ::core::ffi::c_int) -> bool;
    fn no_action();
}

unsafe extern "C" fn wildfire_update_irq_hw(irq: ::core::ffi::c_uint) {
    let qbbno = ((irq >> 8) & (WILDFIRE_MAX_QBB - 1)) as ::core::ffi::c_int;
    let pcano = ((irq >> 6) & (WILDFIRE_PCA_PER_QBB - 1)) as ::core::ffi::c_int;
    if !WILDFIRE_PCA_EXISTS(qbbno, pcano) {
        if doing_init_irq_hw == 0 {
            printk(b"wildfire_update_irq_hw: got irq %d for non-existent PCA %d on QBB %d.\n\0".as_ptr() as _, irq, pcano, qbbno);
        }
        return;
    }
    let pca = wildfire_pca(qbbno, pcano);
    let enable0 = &mut (*pca).pca_int[0].enable as *mut _;
    *enable0 = CACHED_IRQ_MASK[(qbbno as usize) * WILDFIRE_PCA_PER_QBB as usize + pcano as usize];
    mb();
    ::core::ptr::read_volatile(enable0);
}

unsafe extern "C" fn wildfire_init_irq_hw() {
    doing_init_irq_hw = 1;
    let mut i = 0;
    while i < WILDFIRE_NR_IRQS {
        wildfire_update_irq_hw(i as _);
        i += WILDFIRE_IRQ_PER_PCA;
    }
    doing_init_irq_hw = 0;
}

unsafe extern "C" fn wildfire_enable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    if irq < 16 { i8259a_enable_irq(d); }
    spin_lock(&mut wildfire_irq_lock);
    set_bit(irq, CACHED_IRQ_MASK.as_mut_ptr() as _);
    wildfire_update_irq_hw(irq);
    spin_unlock(&mut wildfire_irq_lock);
}

unsafe extern "C" fn wildfire_disable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    if irq < 16 { i8259a_disable_irq(d); }
    spin_lock(&mut wildfire_irq_lock);
    clear_bit(irq, CACHED_IRQ_MASK.as_mut_ptr() as _);
    wildfire_update_irq_hw(irq);
    spin_unlock(&mut wildfire_irq_lock);
}

unsafe extern "C" fn wildfire_mask_and_ack_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    if irq < 16 { i8259a_mask_and_ack_irq(d); }
    spin_lock(&mut wildfire_irq_lock);
    clear_bit(irq, CACHED_IRQ_MASK.as_mut_ptr() as _);
    wildfire_update_irq_hw(irq);
    spin_unlock(&mut wildfire_irq_lock);
}

static mut WILDFIRE_IRQ_TYPE: irq_chip = irq_chip {
    name: b"WILDFIRE\0".as_ptr() as _, irq_unmask: Some(wildfire_enable_irq),
    irq_mask: Some(wildfire_disable_irq), irq_mask_ack: Some(wildfire_mask_and_ack_irq),
};

unsafe extern "C" fn wildfire_init_irq_per_pca(qbbno: ::core::ffi::c_int, pcano: ::core::ffi::c_int) {
    let irq_bias = qbbno * (WILDFIRE_PCA_PER_QBB * WILDFIRE_IRQ_PER_PCA) + pcano * WILDFIRE_IRQ_PER_PCA;
    let mut i = 0;
    while i < 16 {
        if i != 2 { irq_set_chip_and_handler((i + irq_bias) as _, &mut WILDFIRE_IRQ_TYPE, handle_level_irq); irq_set_status_flags((i + irq_bias) as _, IRQ_LEVEL); }
        i += 1;
    }
    irq_set_chip_and_handler((36 + irq_bias) as _, &mut WILDFIRE_IRQ_TYPE, handle_level_irq);
    irq_set_status_flags((36 + irq_bias) as _, IRQ_LEVEL);
    i = 40;
    while i < 64 { irq_set_chip_and_handler((i + irq_bias) as _, &mut WILDFIRE_IRQ_TYPE, handle_level_irq); irq_set_status_flags((i + irq_bias) as _, IRQ_LEVEL); i += 1; }
    if request_irq((32 + irq_bias) as _, no_action, 0, b"isa_enable\0".as_ptr() as _, core::ptr::null_mut()) != 0 { pr_err(b"Failed to register isa_enable interrupt\n\0".as_ptr() as _); }
}

unsafe extern "C" fn wildfire_init_irq() {
    wildfire_init_irq_hw();
    init_i8259a_irqs();
    let mut qbbno = 0;
    while qbbno < WILDFIRE_MAX_QBB { let mut pcano = 0; while pcano < WILDFIRE_PCA_PER_QBB { if WILDFIRE_QBB_EXISTS(qbbno) && WILDFIRE_PCA_EXISTS(qbbno, pcano) { wildfire_init_irq_per_pca(qbbno, pcano); } pcano += 1; } qbbno += 1; }
}

unsafe extern "C" fn wildfire_device_interrupt(vector: ::core::ffi::c_ulong) { handle_irq(((vector - 0x800) >> 4) as _); }

unsafe extern "C" fn wildfire_map_irq(dev: *const pci_dev, _slot: u8, _pin: u8) -> ::core::ffi::c_int {
    let hose = (*dev).sysdata as *mut pci_controller;
    let mut irq = COMMON_TABLE_LOOKUP;
    if irq > 0 { irq += (((*hose).index >> 3) << 8) + (((*hose).index >> 1) & 3) << 6; }
    irq
}

#[no_mangle]
pub static mut wildfire_mv: alpha_machine_vector = alpha_machine_vector { _private: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
