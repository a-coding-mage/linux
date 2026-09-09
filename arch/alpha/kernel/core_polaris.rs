// SPDX-License-Identifier: GPL-2.0
/*
 *      linux/arch/alpha/kernel/core_polaris.c
 *
 * POLARIS chip-specific code
 */

// Dependencies supplied by the surrounding kernel translation unit.

const DEBUG_CONFIG: i32 = 0;

/*
 * Given a bus, device, and function number, compute resulting
 * configuration space address.  This is fairly straightforward
 * on POLARIS, since the chip itself generates Type 0 or Type 1
 * cycles automatically depending on the bus number (Bus 0 is
 * hardwired to Type 0, all others are Type 1.  Peer bridges
 * are not supported).
 */

unsafe fn mk_conf_addr(
    pbus: *mut pci_bus,
    device_fn: c_uint,
    where_: c_int,
    pci_addr: *mut c_ulong,
    type1: *mut u8,
) -> c_int {
    let bus: u8 = (*pbus).number;

    *type1 = if bus == 0 { 0 } else { 1 };
    *pci_addr = ((bus as c_ulong) << 16)
        | ((device_fn as c_ulong) << 8)
        | (where_ as c_ulong)
        | POLARIS_DENSE_CONFIG_BASE;

    0
}

unsafe extern "C" fn polaris_read_config(
    bus: *mut pci_bus,
    devfn: c_uint,
    where_: c_int,
    size: c_int,
    value: *mut u32,
) -> c_int {
    let mut addr: c_ulong = 0;
    let mut type1: u8 = 0;

    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    match size {
        1 => *value = core::ptr::read_volatile(addr as *const u8) as u32,
        2 => *value = core::ptr::read_volatile(addr as *const u16) as u32,
        4 => *value = core::ptr::read_volatile(addr as *const u32),
        _ => {}
    }

    PCIBIOS_SUCCESSFUL
}

unsafe extern "C" fn polaris_write_config(
    bus: *mut pci_bus,
    devfn: c_uint,
    where_: c_int,
    size: c_int,
    value: u32,
) -> c_int {
    let mut addr: c_ulong = 0;
    let mut type1: u8 = 0;

    if mk_conf_addr(bus, devfn, where_, &mut addr, &mut type1) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    match size {
        1 => {
            core::ptr::write_volatile(addr as *mut u8, value as u8);
            mb();
            core::ptr::read_volatile(addr as *const u8);
        }
        2 => {
            core::ptr::write_volatile(addr as *mut u16, value as u16);
            mb();
            core::ptr::read_volatile(addr as *const u16);
        }
        4 => {
            core::ptr::write_volatile(addr as *mut u32, value);
            mb();
            core::ptr::read_volatile(addr as *const u32);
        }
        _ => {}
    }

    PCIBIOS_SUCCESSFUL
}

pub static mut polaris_pci_ops: pci_ops = pci_ops {
    read: Some(polaris_read_config),
    write: Some(polaris_write_config),
};

pub unsafe extern "C" fn polaris_init_arch() {
    let mut hose: *mut pci_controller;

    pci_isa_hose = {
        hose = alloc_pci_controller();
        hose
    };
    (*hose).io_space = &mut ioport_resource;
    (*hose).mem_space = &mut iomem_resource;
    (*hose).index = 0;

    (*hose).sparse_mem_base = 0;
    (*hose).dense_mem_base = POLARIS_DENSE_MEM_BASE - IDENT_ADDR;
    (*hose).sparse_io_base = 0;
    (*hose).dense_io_base = POLARIS_DENSE_IO_BASE - IDENT_ADDR;

    (*hose).sg_isa = core::ptr::null_mut();
    (*hose).sg_pci = core::ptr::null_mut();

    /* The I/O window is fixed at 2G @ 2G.  */
    __direct_map_base = 0x80000000;
    __direct_map_size = 0x80000000;
}

unsafe fn polaris_pci_clr_err() {
    core::ptr::read_volatile(POLARIS_W_STATUS as *const u16);
    /* Write 1's to settable bits to clear errors */
    core::ptr::write_volatile(POLARIS_W_STATUS as *mut u16, 0x7800);
    mb();
    core::ptr::read_volatile(POLARIS_W_STATUS as *const u16);
}

pub unsafe extern "C" fn polaris_machine_check(vector: c_ulong, la_ptr: c_ulong) {
    /* Clear the error before any reporting.  */
    mb();
    mb();
    draina();
    polaris_pci_clr_err();
    wrmces(0x7);
    mb();

    process_mcheck_info(vector, la_ptr, b"POLARIS\0".as_ptr() as *const c_char, mcheck_expected(0));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
