// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_rawhide.c
 *
 * Code supporting the RAWHIDE.
 */

// Linux and Alpha architecture dependencies supplied by the surrounding tree.

extern "C" {
    static mut hose_head: *mut pci_controller;
    static mut rawhide_irq_lock: raw_spinlock_t;

    fn mcpcia_init_hoses();
    fn mcpcia_machine_check();
    fn irq_set_chip_and_handler(irq: c_int, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn irq_set_status_flags(irq: c_int, flags: c_uint);
    fn init_i8259a_irqs();
    fn common_init_isa_dma();
    fn handle_irq(irq: c_int);
    fn common_init_rtc();
    fn common_init_pci();
    fn common_swizzle();
    fn mcpcia_init_arch();
    fn handle_level_irq();
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
}

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct irq_data { pub irq: c_uint }
#[repr(C)]
pub struct irq_chip {
    pub name: *const c_char,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub sysdata: *mut c_void }
#[repr(C)] pub struct pci_controller { pub next: *mut pci_controller, pub index: c_uint }
#[repr(C)] pub struct alpha_machine_vector {
    pub vector_name: *const c_char,
    pub machine_check: Option<unsafe extern "C" fn()>,
    pub max_isa_dma_address: c_ulong,
    pub min_io_address: c_ulong,
    pub min_mem_address: c_ulong,
    pub pci_dac_offset: c_ulong,
    pub nr_irqs: c_uint,
    pub device_interrupt: Option<unsafe extern "C" fn(c_ulong)>,
    pub init_arch: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub init_rtc: Option<unsafe extern "C" fn()>,
    pub init_pci: Option<unsafe extern "C" fn()>,
    pub kill_arch: Option<unsafe extern "C" fn()>,
    pub pci_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8, u8) -> c_int>,
    pub pci_swizzle: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn MCPCIA_INT_MASK0(mid: c_int) -> *mut c_uint;
    fn MCPCIA_INT_MASK1(mid: c_int) -> *mut c_uint;
    fn MCPCIA_INT_REQ(mid: c_int) -> *mut c_uint;
    fn MCPCIA_HOSE2MID(hose: c_int) -> c_int;
}

const MCPCIA_MAX_HOSES: usize = 4;
const IRQ_LEVEL: c_uint = 0;

static mut hose_irq_masks: [c_uint; 4] = [0xff0000, 0xfe0000, 0xff0000, 0xff0000];
static mut cached_irq_masks: [c_uint; 4] = [0; 4];

unsafe fn rawhide_update_irq_hw(hose: c_int, mask: c_uint) {
    core::ptr::write_volatile(MCPCIA_INT_MASK0(MCPCIA_HOSE2MID(hose)), mask);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    let _ = core::ptr::read_volatile(MCPCIA_INT_MASK0(MCPCIA_HOSE2MID(hose)));
}

unsafe fn hose_exists(h: usize) -> bool {
    h < MCPCIA_MAX_HOSES && cached_irq_masks[h] != 0
}

unsafe extern "C" fn rawhide_enable_irq(d: *mut irq_data) {
    let mut irq = (*d).irq;
    let mut flags: c_ulong = 0;
    irq -= 16;
    let hose = (irq / 24) as usize;
    if !hose_exists(hose) { return; }
    irq -= (hose as c_uint) * 24;
    let mask = 1u32 << irq;
    raw_spin_lock_irqsave(&mut rawhide_irq_lock, &mut flags);
    let mask = mask | cached_irq_masks[hose];
    cached_irq_masks[hose] = mask;
    rawhide_update_irq_hw(hose as c_int, mask);
    raw_spin_unlock_irqrestore(&mut rawhide_irq_lock, flags);
}

unsafe extern "C" fn rawhide_disable_irq(d: *mut irq_data) {
    let mut irq = (*d).irq;
    let mut flags: c_ulong = 0;
    irq -= 16;
    let hose = (irq / 24) as usize;
    if !hose_exists(hose) { return; }
    irq -= (hose as c_uint) * 24;
    let mut mask = !(1u32 << irq) | hose_irq_masks[hose];
    raw_spin_lock_irqsave(&mut rawhide_irq_lock, &mut flags);
    mask &= cached_irq_masks[hose];
    cached_irq_masks[hose] = mask;
    rawhide_update_irq_hw(hose as c_int, mask);
    raw_spin_unlock_irqrestore(&mut rawhide_irq_lock, flags);
}

unsafe extern "C" fn rawhide_mask_and_ack_irq(d: *mut irq_data) {
    let mut irq = (*d).irq;
    let mut flags: c_ulong = 0;
    irq -= 16;
    let hose = (irq / 24) as usize;
    if !hose_exists(hose) { return; }
    irq -= (hose as c_uint) * 24;
    let mask1 = 1u32 << irq;
    let mut mask = !mask1 | hose_irq_masks[hose];
    raw_spin_lock_irqsave(&mut rawhide_irq_lock, &mut flags);
    mask &= cached_irq_masks[hose];
    cached_irq_masks[hose] = mask;
    rawhide_update_irq_hw(hose as c_int, mask);
    core::ptr::write_volatile(MCPCIA_INT_REQ(MCPCIA_HOSE2MID(hose as c_int)), mask1);
    raw_spin_unlock_irqrestore(&mut rawhide_irq_lock, flags);
}

static mut rawhide_irq_type: irq_chip = irq_chip {
    name: b"RAWHIDE\0".as_ptr() as *const c_char,
    irq_unmask: Some(rawhide_enable_irq), irq_mask: Some(rawhide_disable_irq),
    irq_mask_ack: Some(rawhide_mask_and_ack_irq),
};

unsafe extern "C" fn rawhide_srm_device_interrupt(vector: c_ulong) {
    let mut irq = ((vector - 0x800) >> 4) as c_int;
    if irq == 52 { irq = 72; }
    irq -= ((irq + 16) >> 2) & 0x38;
    handle_irq(irq);
}

unsafe extern "C" fn rawhide_init_irq() {
    mcpcia_init_hoses();
    for i in 0..MCPCIA_MAX_HOSES { cached_irq_masks[i] = 0; }
    let mut hose = hose_head;
    while !hose.is_null() {
        let h = (*hose).index as usize;
        let mask = hose_irq_masks[h];
        cached_irq_masks[h] = mask;
        core::ptr::write_volatile(MCPCIA_INT_MASK0(MCPCIA_HOSE2MID(h as c_int)), mask);
        core::ptr::write_volatile(MCPCIA_INT_MASK1(MCPCIA_HOSE2MID(h as c_int)), 0);
        hose = (*hose).next;
    }
    for i in 16..128 { irq_set_chip_and_handler(i, &mut rawhide_irq_type, handle_level_irq); irq_set_status_flags(i, IRQ_LEVEL); }
    init_i8259a_irqs();
    common_init_isa_dma();
}

unsafe extern "C" fn rawhide_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    static irq_tab: [[c_int; 5]; 5] = [
        [32,32,32,32,32], [16,16,17,18,19], [20,20,21,22,23],
        [24,24,25,26,27], [28,28,29,30,31]
    ];
    let hose = (*(dev)).sysdata as *mut pci_controller;
    let irq = if slot >= 1 && slot <= 5 && pin <= 4 { irq_tab[(slot - 1) as usize][pin as usize] } else { -1 };
    if irq >= 0 { irq + 24 * (*hose).index as c_int } else { irq }
}

#[no_mangle]
pub static mut rawhide_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"Rawhide\0".as_ptr() as *const c_char,
    machine_check: Some(mcpcia_machine_check), max_isa_dma_address: 0,
    min_io_address: 0, min_mem_address: 0, pci_dac_offset: 0, nr_irqs: 128,
    device_interrupt: Some(rawhide_srm_device_interrupt), init_arch: Some(mcpcia_init_arch),
    init_irq: Some(rawhide_init_irq), init_rtc: Some(common_init_rtc), init_pci: Some(common_init_pci),
    kill_arch: None, pci_map_irq: Some(rawhide_map_irq), pci_swizzle: Some(common_swizzle),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
