/*
 * Board setup routines for the Buffalo Linkstation / Kurobox Platform.
 *
 * Copyright (C) 2006 G. Liakhovetski (g.liakhovetski@gmx.de)
 *
 * Based on sandpoint.c by Mark A. Greer
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of
 * any kind, whether express or implied.
 */

// Kernel and architecture declarations are supplied by the surrounding tree.

extern "C" {
    fn printk(fmt: *const i8, ...);
    fn of_platform_bus_probe(np: *mut device_node, ids: *const of_device_id,
                             matches: *mut core::ffi::c_void) -> i32;
    fn of_get_property(np: *mut device_node, name: *const i8,
                       lenp: *mut i32) -> *const i32;
    fn pcibios_alloc_controller(np: *mut device_node) -> *mut pci_controller;
    fn setup_indirect_pci(hose: *mut pci_controller, cfg_addr: u32,
                          cfg_data: u32, flags: i32);
    fn pci_process_bridge_OF_ranges(hose: *mut pci_controller,
                                    dev: *mut device_node, primary: i32);
    fn mpic_alloc(ops: *mut core::ffi::c_void, flags: i32, isu_size: i32,
                  irq_count: i32, offset: i32, name: *const i8) -> *mut mpic;
    fn mpic_assign_isu(mpic: *mut mpic, isu: i32, paddr: u64);
    fn mpic_init(mpic: *mut mpic);
    fn mpic_get_irq() -> i32;
    fn local_irq_disable();
    fn avr_uart_configure();
    fn avr_uart_send(c: i8);
    fn seq_printf(m: *mut seq_file, fmt: *const i8, ...);
}

#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct pci_controller {
    pub first_busno: u8,
    pub last_busno: u8,
    pub paddr: u64,
}
#[repr(C)]
pub struct mpic { pub paddr: u64 }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct of_device_id {
    pub type_: *const i8,
    pub compatible: *const i8,
}

static OF_BUS_IDS: [of_device_id; 3] = [
    of_device_id { type_: b"soc\0".as_ptr() as *const i8, compatible: core::ptr::null() },
    of_device_id { type_: core::ptr::null(), compatible: b"simple-bus\0".as_ptr() as *const i8 },
    of_device_id { type_: core::ptr::null(), compatible: core::ptr::null() },
];

unsafe extern "C" fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

unsafe extern "C" fn linkstation_add_bridge(dev: *mut device_node) -> i32 {
    #[cfg(CONFIG_PCI)]
    {
        let mut len = 0i32;
        let bus_range = of_get_property(dev, b"bus-range\0".as_ptr() as *const i8, &mut len);
        if bus_range.is_null() || len < 2 * core::mem::size_of::<i32>() as i32 {
            printk(b"Can't get bus-range for %pOF, assume bus 0\n\0".as_ptr() as *const i8, dev);
        }
        let hose = pcibios_alloc_controller(dev);
        if hose.is_null() { return -12; }
        (*hose).first_busno = if !bus_range.is_null() { *bus_range as u8 } else { 0 };
        (*hose).last_busno = if !bus_range.is_null() { *bus_range.add(1) as u8 } else { 0xff };
        setup_indirect_pci(hose, 0xfec00000, 0xfee00000, 0);
        // Interpret the "ranges" property; this also maps I/O and sets isa_io/mem_base.
        pci_process_bridge_OF_ranges(hose, dev, 1);
    }
    0
}

unsafe extern "C" fn linkstation_setup_arch() {
    printk(b"BUFFALO Network Attached Storage Series\n\0".as_ptr() as *const i8);
    printk(b"(C) 2002-2005 BUFFALO INC.\n\0".as_ptr() as *const i8);
}

unsafe extern "C" fn linkstation_setup_pci() {
    // for_each_compatible_node(np, "pci", "mpc10x-pci")
    //     linkstation_add_bridge(np);
}

unsafe extern "C" fn linkstation_init_IRQ() {
    let mpic = mpic_alloc(core::ptr::null_mut(), 0, 0, 4, 0, b" EPIC     \0".as_ptr() as *const i8);
    assert!(!mpic.is_null());
    mpic_assign_isu(mpic, 0, (*mpic).paddr + 0x10200);
    mpic_assign_isu(mpic, 1, (*mpic).paddr + 0x11000);
    mpic_assign_isu(mpic, 2, (*mpic).paddr + 0x11100);
    mpic_init(mpic);
}

unsafe extern "C" fn linkstation_restart(_cmd: *mut i8) -> ! {
    local_irq_disable(); avr_uart_configure(); avr_uart_send(b'C' as i8);
    loop { avr_uart_send(b'G' as i8); }
}

unsafe extern "C" fn linkstation_power_off() -> ! {
    local_irq_disable(); avr_uart_configure(); avr_uart_send(b'E' as i8);
    loop { avr_uart_send(b'G' as i8); }
}

unsafe extern "C" fn linkstation_halt() -> ! { linkstation_power_off() }

unsafe extern "C" fn linkstation_show_cpuinfo(m: *mut seq_file) {
    seq_printf(m, b"vendor\t\t: Buffalo Technology\n\0".as_ptr() as *const i8);
    seq_printf(m, b"machine\t\t: Linkstation I/Kurobox(HG)\n\0".as_ptr() as *const i8);
}

unsafe extern "C" fn linkstation_probe() -> i32 {
    // pm_power_off = linkstation_power_off;
    1
}

// define_machine(linkstation) — the platform machine descriptor is supplied by the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
