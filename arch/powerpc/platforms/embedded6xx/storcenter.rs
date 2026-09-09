/*
 * Board setup routines for the storcenter
 *
 * Copyright 2007 (C) Oyvind Repvik (nail@nslu2-linux.org)
 * Copyright 2007 Andy Wilcox, Jon Loeliger
 *
 * Based on linkstation.c by G. Liakhovetski
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty
 * of any kind, whether express or implied.
 */

use core::ffi::{c_char, c_int, c_void};

/* Declarations supplied by the kernel and architecture headers. */
#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PciController {
    pub first_busno: u8,
    pub last_busno: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mpic {
    pub paddr: usize,
    _private: [u8; 0],
}

extern "C" {
    fn of_platform_bus_probe(
        node: *mut DeviceNode,
        matches: *const OfDeviceId,
        parent: *mut c_void,
    ) -> c_int;
    fn pcibios_alloc_controller(dev: *mut DeviceNode) -> *mut PciController;
    fn of_get_property(dev: *mut DeviceNode, name: *const c_char, len: *mut c_int) -> *const c_int;
    fn setup_indirect_pci(hose: *mut PciController, addr: c_int, data: c_int, offset: c_int);
    fn pci_process_bridge_of_ranges(hose: *mut PciController, dev: *mut DeviceNode, primary: c_int);
    fn printk(fmt: *const c_char, ...);
    fn mpic_alloc(
        node: *mut c_void,
        flags: c_int,
        isu_size: c_int,
        irq_count: c_int,
        irq_offset: c_int,
        name: *const c_char,
    ) -> *mut Mpic;
    fn mpic_assign_isu(mpic: *mut Mpic, isu: c_int, paddr: usize);
    fn mpic_init(mpic: *mut Mpic);
    fn mpic_get_irq() -> c_int;
    fn local_irq_disable();
    fn mfmsr() -> usize;
    fn mtmsr(value: usize);
    fn isync();
    fn bug_on(condition: bool);
}

#[repr(C)]
pub struct OfDeviceId {
    pub name: *const c_char,
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub setup_arch: unsafe extern "C" fn(),
    pub discover_phbs: unsafe extern "C" fn(),
    pub init_irq: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> c_int,
    pub restart: unsafe extern "C" fn(*mut c_char) -> !,
}

static STORCENTER_OF_BUS: [OfDeviceId; 2] = [
    OfDeviceId { name: c"soc".as_ptr() },
    OfDeviceId { name: core::ptr::null() },
];

unsafe extern "C" fn storcenter_device_probe() -> c_int {
    of_platform_bus_probe(core::ptr::null_mut(), STORCENTER_OF_BUS.as_ptr(), core::ptr::null_mut());
    0
}

/* machine_device_initcall(storcenter, storcenter_device_probe); */

unsafe extern "C" fn storcenter_add_bridge(dev: *mut DeviceNode) -> c_int {
    /* CONFIG_PCI conditional from the original source. */
    #[cfg(CONFIG_PCI)]
    {
        let mut len: c_int = 0;
        let hose = pcibios_alloc_controller(dev);
        if hose.is_null() {
            return -12; /* -ENOMEM */
        }

        let bus_range = of_get_property(dev, c"bus-range".as_ptr(), &mut len);
        (*hose).first_busno = if !bus_range.is_null() { *bus_range as u8 } else { 0 };
        (*hose).last_busno = if !bus_range.is_null() { *bus_range.add(1) as u8 } else { 0xff };

        setup_indirect_pci(hose, 0, 0, 0);
        pci_process_bridge_of_ranges(hose, dev, 1);
    }
    0
}

unsafe extern "C" fn storcenter_setup_arch() {
    printk(c"IOMEGA StorCenter\n".as_ptr());
}

unsafe extern "C" fn storcenter_setup_pci() {
    /* for_each_compatible_node(np, "pci", "mpc10x-pci") */
    let mut np: *mut DeviceNode = core::ptr::null_mut();
    while !np.is_null() {
        storcenter_add_bridge(np);
        break;
    }
}

/*
 * Interrupt setup and service.  Interrupts on the turbostation come
 * from the four PCI slots plus onboard 8241 devices: I2C, DUART.
 */
unsafe extern "C" fn storcenter_init_IRQ() {
    let mpic = mpic_alloc(core::ptr::null_mut(), 0, 0, 16, 0, c" OpenPIC  ".as_ptr());
    bug_on(mpic.is_null());

    /* 16 Serial Interrupts followed by 16 Internal Interrupts. */
    mpic_assign_isu(mpic, 0, (*mpic).paddr + 0x10200);
    mpic_assign_isu(mpic, 1, (*mpic).paddr + 0x11000);
    mpic_init(mpic);
}

unsafe extern "C" fn storcenter_restart(_cmd: *mut c_char) -> ! {
    local_irq_disable();
    /* Set exception prefix high - to the firmware */
    mtmsr(mfmsr() | (1usize << 6)); /* MSR_IP */
    isync();
    /* Wait for reset to happen */
    loop {}
}

#[no_mangle]
pub static STORCENTER_MACHINE: MachineDesc = MachineDesc {
    name: c"IOMEGA StorCenter".as_ptr(),
    compatible: c"iomega,storcenter".as_ptr(),
    setup_arch: storcenter_setup_arch,
    discover_phbs: storcenter_setup_pci,
    init_irq: storcenter_init_IRQ,
    get_irq: mpic_get_irq,
    restart: storcenter_restart,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
