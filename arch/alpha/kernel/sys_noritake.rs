// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_noritake.c
 *
 * Code supporting the NORITAKE (AlphaServer 1000A),
 * CORELLE (AlphaServer 800), and ALCOR Primo (AlphaStation 600A).
 */

// Linux and architecture dependencies supplied by the surrounding kernel.

static mut CACHED_IRQ_MASK: i32 = 0;

unsafe extern "C" {
    static mut alpha_using_srm: bool;
    static mut alpha_mv: AlphaMachineVector;

    fn outw(value: i32, port: i32);
    fn inw(port: i32) -> u16;
    fn inb(port: i32) -> u8;
    fn isa_device_interrupt(vector: u64);
    fn handle_irq(irq: u32);
    fn irq_set_chip_and_handler(irq: i32, chip: *mut IrqChip, handler: unsafe extern "C" fn());
    fn irq_set_status_flags(irq: i32, flags: u32);
    fn init_i8259a_irqs();
    fn common_init_isa_dma();
    fn cia_machine_check();
    fn cia_init_arch();
    fn common_init_rtc();
    fn cia_init_pci();
    fn cia_kill_arch();
    fn pci_swizzle_interrupt_pin(dev: *mut PciDev, pin: u8) -> u8;
    fn common_table_lookup(
        dev: *const PciDev,
        slot: u8,
        pin: u8,
        min_idsel: i64,
        max_idsel: i64,
        irqs_per_slot: i64,
        irq_tab: *const i8,
    ) -> i32;
}

#[repr(C)]
pub struct IrqData {
    pub irq: i32,
}

#[repr(C)]
pub struct IrqChip {
    pub name: *const u8,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut IrqData)>,
}

#[repr(C)]
pub struct PciBus {
    pub number: u8,
    pub self_: *mut PciDev,
}

#[repr(C)]
pub struct PciDev {
    pub devfn: u8,
    pub bus: *mut PciBus,
}

#[repr(C)]
pub struct AlphaMachineVector {
    pub vector_name: *const u8,
    pub machine_check: unsafe extern "C" fn(),
    pub max_isa_dma_address: u64,
    pub min_io_address: u64,
    pub min_mem_address: u64,
    pub nr_irqs: i32,
    pub device_interrupt: unsafe extern "C" fn(u64),
    pub init_arch: unsafe extern "C" fn(),
    pub init_irq: unsafe extern "C" fn(),
    pub init_rtc: unsafe extern "C" fn(),
    pub init_pci: unsafe extern "C" fn(),
    pub kill_arch: unsafe extern "C" fn(),
    pub pci_map_irq: unsafe extern "C" fn(*const PciDev, u8, u8) -> i32,
    pub pci_swizzle: unsafe extern "C" fn(*mut PciDev, *mut u8) -> u8,
}

unsafe extern "C" fn handle_level_irq() {}

unsafe fn noritake_update_irq_hw(irq: i32, mut mask: i32) {
    let mut port = 0x54a;
    if irq >= 32 {
        mask >>= 16;
        port = 0x54c;
    }
    outw(mask, port);
}

unsafe extern "C" fn noritake_enable_irq(d: *mut IrqData) {
    CACHED_IRQ_MASK |= 1 << ((*d).irq - 16);
    noritake_update_irq_hw((*d).irq, CACHED_IRQ_MASK);
}

unsafe extern "C" fn noritake_disable_irq(d: *mut IrqData) {
    CACHED_IRQ_MASK &= !(1 << ((*d).irq - 16));
    noritake_update_irq_hw((*d).irq, CACHED_IRQ_MASK);
}

static mut NORITAKE_IRQ_TYPE: IrqChip = IrqChip {
    name: b"NORITAKE\0".as_ptr(),
    irq_unmask: Some(noritake_enable_irq),
    irq_mask: Some(noritake_disable_irq),
    irq_mask_ack: Some(noritake_disable_irq),
};

unsafe extern "C" fn noritake_device_interrupt(vector: u64) {
    let mut pld = ((inw(0x54c) as u64) << 32)
        | ((inw(0x54a) as u64) << 16)
        | ((inb(0xa0) as u64) << 8)
        | inb(0x20) as u64;

    while pld != 0 {
        let i = (!pld).trailing_zeros();
        pld &= pld - 1;
        if i < 16 {
            isa_device_interrupt(vector);
        } else {
            handle_irq(i);
        }
    }
}

unsafe extern "C" fn noritake_srm_device_interrupt(vector: u64) {
    let mut irq = ((vector - 0x800) >> 4) as i32;
    if irq >= 16 {
        irq += 1;
    }
    handle_irq(irq as u32);
}

unsafe extern "C" fn noritake_init_irq() {
    if alpha_using_srm {
        alpha_mv.device_interrupt = noritake_srm_device_interrupt;
    }
    outw(0, 0x54a);
    outw(0, 0x54c);
    let mut i = 16;
    while i < 48 {
        irq_set_chip_and_handler(i, &raw mut NORITAKE_IRQ_TYPE, handle_level_irq);
        irq_set_status_flags(i, 0x00000004);
        i += 1;
    }
    init_i8259a_irqs();
    common_init_isa_dma();
}

unsafe extern "C" fn noritake_map_irq(dev: *const PciDev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 15] = [
        [17, 17, 17, 17, 17], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [18, 18, 19, 34, 35], [20, 20, 21, 36, 37], [22, 22, 23, 38, 39],
        [24, 24, 25, 40, 41], [17, 17, 17, 17, 17], [24, 24, 25, 40, 41],
        [26, 26, 27, 42, 43], [28, 28, 29, 44, 45], [30, 30, 31, 46, 47],
    ];
    common_table_lookup(dev, slot, pin, 5, 19, 5, irq_tab.as_ptr() as *const i8)
}

unsafe extern "C" fn noritake_swizzle(dev: *mut PciDev, pinp: *mut u8) -> u8 {
    let mut pin = *pinp;
    let slot;
    if (*(*dev).bus).number == 0 {
        slot = (*dev).devfn >> 3;
    } else if ((*(*(*dev).bus).self_).devfn >> 3) == 8 {
        slot = ((*dev).devfn >> 3) + 15;
    } else {
        let mut current = dev;
        loop {
            if ((*(*(*current).bus).self_).devfn >> 3) == 8 {
                slot = ((*current).devfn >> 3) + 15;
                break;
            }
            pin = pci_swizzle_interrupt_pin(current, pin);
            current = (*current).bus.as_mut().unwrap().self_;
        }
    }
    *pinp = pin;
    slot
}

#[no_mangle]
pub static mut noritake_primo_mv: AlphaMachineVector = AlphaMachineVector {
    vector_name: b"Noritake-Primo\0".as_ptr(),
    machine_check: cia_machine_check,
    max_isa_dma_address: 0,
    min_io_address: 0,
    min_mem_address: 0,
    nr_irqs: 48,
    device_interrupt: noritake_device_interrupt,
    init_arch: cia_init_arch,
    init_irq: noritake_init_irq,
    init_rtc: common_init_rtc,
    init_pci: cia_init_pci,
    kill_arch: cia_kill_arch,
    pci_map_irq: noritake_map_irq,
    pci_swizzle: noritake_swizzle,
};

// ALIAS_MV(noritake_primo)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
