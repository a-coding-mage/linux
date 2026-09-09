// SPDX-License-Identifier: GPL-2.0
/*
 * setup.c - boot time setup code
 */

// Dependencies supplied by the surrounding kernel sources.

#[repr(C)]
pub struct pci_reg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

extern "C" {
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut pci_reg: *mut pci_reg;
    static mut mips_machtype: u32;
    static mut epld_mask: *mut u32;

    fn writel(value: u32, address: usize);
    fn ioremap(address: usize, size: usize) -> *mut pci_reg;
    fn printk(format: *const core::ffi::c_char, ...);
    fn set_io_port_base(base: usize);
    fn write_c0_wired(value: u32);
}

const PCI0_BASE_ADDR: usize = 0;
const IORESOURCE_MEM: usize = 0;
const IDT434_REG_BASE: usize = 0;
const RST: usize = 0;
const KSEG1: usize = 0;
const MACH_MIKROTIK_RB532A: u32 = 0;

static mut pci0_res: [resource; 1] = [resource {
    name: b"pci_reg0\0".as_ptr() as *const core::ffi::c_char,
    start: PCI0_BASE_ADDR,
    end: PCI0_BASE_ADDR + core::mem::size_of::<pci_reg>(),
    flags: IORESOURCE_MEM,
}];

unsafe fn rb_machine_restart(_command: *mut core::ffi::c_char) {
    /* just jump to the reset vector */
    writel(0x80000001, IDT434_REG_BASE + RST);
    let reset_vector: unsafe extern "C" fn() = core::mem::transmute(0x1FC00000usize);
    reset_vector();
}

unsafe fn rb_machine_halt() {
    loop {
        core::hint::spin_loop();
    }
}

pub unsafe extern "C" fn plat_mem_setup() {
    let mut val: u32;

    _machine_restart = Some(rb_machine_restart);
    _machine_halt = Some(rb_machine_halt);
    pm_power_off = Some(rb_machine_halt);

    set_io_port_base(KSEG1);

    pci_reg = ioremap(
        pci0_res[0].start,
        pci0_res[0].end - pci0_res[0].start,
    );
    if pci_reg.is_null() {
        printk(b"Could not remap PCI registers\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    val = core::ptr::read_volatile(pci_reg.cast::<u32>());
    val &= 0xFFFFFF7;
    core::ptr::write_volatile(pci_reg.cast::<u32>(), val);

    // CONFIG_PCI: Enable PCI interrupts in EPLD Mask register.
    *epld_mask = 0x0;
    *epld_mask.add(1) = 0x0;

    write_c0_wired(0);
}

pub unsafe extern "C" fn get_system_type() -> *const core::ffi::c_char {
    match mips_machtype {
        MACH_MIKROTIK_RB532A => b"Mikrotik RB532A\0".as_ptr() as *const core::ffi::c_char,
        _ => b"Mikrotik RB532\0".as_ptr() as *const core::ffi::c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
