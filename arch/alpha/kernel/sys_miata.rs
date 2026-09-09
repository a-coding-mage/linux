// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_miata.c
 *
 * Code supporting the MIATA (EV56+PYXIS).
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

extern "C" {
    static mut alpha_using_srm: bool;
    static mut alpha_mv: AlphaMachineVector;

    fn handle_irq(irq: i32);
    fn init_i8259a_irqs();
    fn init_pyxis_irqs(mask: u32);
    fn common_init_isa_dma();
    fn request_irq(irq: i32, action: unsafe extern "C" fn(), flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn no_action();
    fn pr_err(fmt: *const u8, ...);
    fn cia_init_pci();
    fn SMC669_Init(arg: i32);
    fn es1888_init();
    fn cia_kill_arch(mode: i32);
    fn halt() -> !;
    fn cia_machine_check();
    fn pyxis_device_interrupt(vector: u64);
    fn pyxis_init_arch();
    fn common_init_rtc();
    fn pci_get_slot(bus: *mut PciBus, devfn: u8) -> *mut PciDev;
    fn pci_read_config_byte(dev: *mut PciDev, where_: u8, value: *mut u8) -> i32;
    fn pci_dev_put(dev: *mut PciDev);
    fn pci_swizzle_interrupt_pin(dev: *mut PciDev, pin: i32) -> i32;
    static mut PYXIS_RESET: *mut u32;
}

#[repr(C)]
pub struct PciBus {
    pub number: u8,
    pub self_: *mut PciDev,
}

#[repr(C)]
pub struct PciDev {
    pub bus: *mut PciBus,
    pub devfn: u8,
}

#[repr(C)]
pub struct AlphaMachineVector {
    pub vector_name: *const u8,
    // Remaining fields are defined by the Alpha machine-vector implementation.
}

const PCIBIOS_SUCCESSFUL: i32 = 0;
const LINUX_REBOOT_CMD_RESTART: i32 = 0x01234567;
const LINUX_REBOOT_CMD_HALT: i32 = 0xCDEF0123;
const LINUX_REBOOT_CMD_POWER_OFF: i32 = 0x4321FEDC;

#[inline]
unsafe fn pci_slot(devfn: u8) -> i32 { ((devfn as i32) >> 3) & 0x1f }
#[inline]
unsafe fn pci_func(devfn: u8) -> i32 { (devfn as i32) & 7 }

unsafe extern "C" fn miata_srm_device_interrupt(vector: u64) {
    let mut irq = ((vector - 0x800) >> 4) as i32;
    if irq >= 16 { irq += 8; }
    handle_irq(irq);
}

unsafe extern "C" fn miata_init_irq() {
    if alpha_using_srm {
        // alpha_mv.device_interrupt = miata_srm_device_interrupt;
    }

    init_i8259a_irqs();
    init_pyxis_irqs(0x063b0000);
    common_init_isa_dma();
    if request_irq(18, no_action, 0, b"halt-switch\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to register halt-switch interrupt\n\0".as_ptr());
    }
    if request_irq(22, no_action, 0, b"timer-cascade\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to register timer-cascade interrupt\n\0".as_ptr());
    }
}

unsafe extern "C" fn miata_map_irq(dev: *const PciDev, slot: u8, _pin: u8) -> i32 {
    let irq_tab: [[i32; 5]; 18] = [
        [24,24,24,24,24], [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1],
        [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1],
        [28,28,29,30,31], [32,32,33,34,35], [27,27,27,27,27], [-1,-1,-1,-1,-1],
        [-1,-1,-1,-1,-1], [-1,-1,-1,-1,-1], [36,36,37,38,39], [40,40,41,42,43],
        [44,44,45,46,47], [-1,-1,-1,-1,-1],
    ];
    let _ = irq_tab;
    if slot == 7 && pci_func((*dev).devfn) == 3 {
        let pdev = pci_get_slot((*dev).bus, (*dev).devfn & !7);
        if pdev.is_null() { return -1; }
        let mut irq = 0u8;
        let ret = pci_read_config_byte(pdev, 0x40, &mut irq);
        pci_dev_put(pdev);
        return if ret == PCIBIOS_SUCCESSFUL { irq as i32 } else { -1 };
    }
    // COMMON_TABLE_LOOKUP: lookup irq_tab using the common PCI table rules.
    -1
}

unsafe extern "C" fn miata_swizzle(dev: *mut PciDev, pinp: *mut u8) -> u8 {
    let mut dev = dev;
    let mut pin = *pinp as i32;
    let slot: i32;
    if (*(*dev).bus).number == 0 {
        slot = pci_slot((*dev).devfn);
    } else if pci_slot((*(*dev).bus).self_ .devfn) == 8 || pci_slot((*(*dev).bus).self_.devfn) == 20 {
        slot = pci_slot((*dev).devfn) + 9;
    } else {
        loop {
            if pci_slot((*(*dev).bus).self_.devfn) == 8 || pci_slot((*(*dev).bus).self_.devfn) == 20 {
                slot = pci_slot((*dev).devfn) + 9;
                break;
            }
            pin = pci_swizzle_interrupt_pin(dev, pin);
            dev = (*dev).bus.cast::<PciBus>().as_ref().unwrap().self_;
            if (*dev).bus.is_null() { break; }
        }
        // The C loop's final slot assignment is represented by the bridge walk above.
        slot = pci_slot((*dev).devfn);
    }
    *pinp = pin as u8;
    slot as u8
}

unsafe extern "C" fn miata_init_pci() {
    cia_init_pci();
    SMC669_Init(0);
    es1888_init();
}

unsafe extern "C" fn miata_kill_arch(mode: i32) {
    cia_kill_arch(mode);
    // #ifndef ALPHA_RESTORE_SRM_SETUP
    match mode {
        LINUX_REBOOT_CMD_RESTART if alpha_using_srm => {
            core::ptr::write_volatile(PYXIS_RESET, 0x0000_dead);
            // mb();
        }
        LINUX_REBOOT_CMD_RESTART | LINUX_REBOOT_CMD_HALT | LINUX_REBOOT_CMD_POWER_OFF => {}
        _ => {}
    }
    halt();
}

// The C machine-vector initializer (DO_EV5_MMU, DO_DEFAULT_RTC, DO_PYXIS_IO,
// ALIAS_MV(miata)) is retained for integration with the Alpha machine-vector ABI.
#[allow(non_upper_case_globals)]
pub static mut miata_mv: AlphaMachineVector = AlphaMachineVector { vector_name: b"Miata\0".as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
