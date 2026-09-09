// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_sable.c
 *
 * Code supporting the Sable, Sable-Gamma, and Lynx systems.
 *
 * Dependencies supplied by the Linux kernel and sibling architecture files
 * are intentionally referenced but not implemented here.
 */

use core::ffi::c_void;

// External kernel types, functions, constants, and objects.
#[repr(C)] pub struct irq_data { pub irq: usize }
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct irq_chip {
    pub name: *const u8,
    pub irq_unmask: Option<unsafe fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe fn(*mut irq_data)>,
}
#[repr(C)] pub struct alpha_machine_vector;

extern "C" {
    static mut sable_lynx_irq_lock: c_void;
    fn outb(value: i32, port: i32);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn irq_set_chip_and_handler(irq: i64, chip: *mut irq_chip, handler: unsafe extern "C" fn(i64));
    fn irq_set_status_flags(irq: i64, flags: i32);
    fn handle_irq(irq: i32);
    fn handle_level_irq(irq: i64);
    fn common_init_isa_dma();
    fn common_init_pci();
    fn common_init_rtc();
    fn common_swizzle();
    fn t2_machine_check();
    fn t2_init_arch();
    fn t2_kill_arch();
}

const IRQ_LEVEL: i32 = 1;

#[repr(C)]
pub struct irq_swizzle_t {
    pub irq_to_mask: [i8; 64],
    pub mask_to_irq: [i8; 64],
    // Mask bit is true for disabled IRQs.
    pub shadow_mask: usize,
    pub update_irq_hw: Option<unsafe fn(usize, usize)>,
    pub ack_irq_hw: Option<unsafe fn(usize)>,
}

static mut sable_lynx_irq_swizzle: *mut irq_swizzle_t = core::ptr::null_mut();

unsafe fn sable_update_irq_hw(bit: usize, mut mask: usize) {
    let mut port = 0x537i32;
    if bit >= 16 {
        port = 0x53d;
        mask >>= 16;
    } else if bit >= 8 {
        port = 0x53b;
        mask >>= 8;
    }
    outb(mask as i32, port);
}

unsafe fn sable_ack_irq_hw(bit: usize) {
    let (port, val1, val2);
    if bit >= 16 {
        port = 0x53c;
        val1 = 0xe0 | (bit - 16) as i32;
        val2 = 0xe0 | 4;
    } else if bit >= 8 {
        port = 0x53a;
        val1 = 0xe0 | (bit - 8) as i32;
        val2 = 0xe0 | 3;
    } else {
        port = 0x536;
        val1 = 0xe0 | bit as i32;
        val2 = 0xe0 | 1;
    }
    outb(val1, port); // ack the slave
    outb(val2, 0x534); // ack the master
}

static mut sable_irq_swizzle: irq_swizzle_t = irq_swizzle_t {
    irq_to_mask: [
        -1, 6, -1, 8, 15, 12, 7, 9, -1, 16, 17, 18, 3, -1, 21, 22,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        2, 1, 0, 4, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    mask_to_irq: [
        34, 33, 32, 12, 35, 36, 1, 6, 3, 7, -1, -1, 5, -1, -1, 4,
        9, 10, 11, -1, -1, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    shadow_mask: usize::MAX,
    update_irq_hw: Some(sable_update_irq_hw),
    ack_irq_hw: Some(sable_ack_irq_hw),
};

unsafe fn sable_init_irq() {
    outb(-1, 0x537); // slave 0
    outb(-1, 0x53b); // slave 1
    outb(-1, 0x53d); // slave 2
    outb(0x44, 0x535); // enable cascades in master
    sable_lynx_irq_swizzle = &raw mut sable_irq_swizzle;
    sable_lynx_init_irq(40);
}

unsafe fn sable_map_irq(_dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    static IRQ_TAB: [[i8; 5]; 9] = [
        [32, 32, 32, 32, 32], [33, 33, 33, 33, 33], [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [34, 34, 34, 34, 34], [35, 35, 35, 35, 35], [36, 36, 36, 36, 36],
    ];
    IRQ_TAB[slot as usize][pin as usize] as i32
}

unsafe fn sable_lynx_enable_irq(d: *mut irq_data) {
    let bit = (*sable_lynx_irq_swizzle).irq_to_mask[(*d).irq] as usize;
    spin_lock(&raw mut sable_lynx_irq_lock);
    let mask = (*sable_lynx_irq_swizzle).shadow_mask &= !(1usize << bit);
    ((*sable_lynx_irq_swizzle).update_irq_hw.unwrap())(bit, mask);
    spin_unlock(&raw mut sable_lynx_irq_lock);
}

unsafe fn sable_lynx_disable_irq(d: *mut irq_data) {
    let bit = (*sable_lynx_irq_swizzle).irq_to_mask[(*d).irq] as usize;
    spin_lock(&raw mut sable_lynx_irq_lock);
    let mask = (*sable_lynx_irq_swizzle).shadow_mask |= 1usize << bit;
    ((*sable_lynx_irq_swizzle).update_irq_hw.unwrap())(bit, mask);
    spin_unlock(&raw mut sable_lynx_irq_lock);
}

unsafe fn sable_lynx_mask_and_ack_irq(d: *mut irq_data) {
    let bit = (*sable_lynx_irq_swizzle).irq_to_mask[(*d).irq] as usize;
    spin_lock(&raw mut sable_lynx_irq_lock);
    let mask = (*sable_lynx_irq_swizzle).shadow_mask |= 1usize << bit;
    ((*sable_lynx_irq_swizzle).update_irq_hw.unwrap())(bit, mask);
    ((*sable_lynx_irq_swizzle).ack_irq_hw.unwrap())(bit);
    spin_unlock(&raw mut sable_lynx_irq_lock);
}

static mut sable_lynx_irq_type: irq_chip = irq_chip {
    name: b"SABLE/LYNX\0".as_ptr(),
    irq_unmask: Some(sable_lynx_enable_irq),
    irq_mask: Some(sable_lynx_disable_irq),
    irq_mask_ack: Some(sable_lynx_mask_and_ack_irq),
};

unsafe fn sable_lynx_srm_device_interrupt(vector: usize) {
    let bit = ((vector - 0x800) >> 4) as usize;
    let irq = (*sable_lynx_irq_swizzle).mask_to_irq[bit] as i32;
    handle_irq(irq);
}

unsafe fn sable_lynx_init_irq(nr_of_irqs: i64) {
    for i in 0..nr_of_irqs {
        irq_set_chip_and_handler(i, &raw mut sable_lynx_irq_type, handle_level_irq);
        irq_set_status_flags(i, IRQ_LEVEL);
    }
    common_init_isa_dma();
}

unsafe fn sable_lynx_init_pci() { common_init_pci(); }

// The alpha_machine_vector definition and machine-vector registration are
// supplied by the architecture headers; preserve the original configuration
// gated Sable-Gamma vector fields here for the dependent build environment.
#[cfg(any(feature = "CONFIG_ALPHA_GENERIC", feature = "CONFIG_ALPHA_SABLE"))]
#[no_mangle]
pub static mut sable_gamma_mv: alpha_machine_vector = alpha_machine_vector;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
