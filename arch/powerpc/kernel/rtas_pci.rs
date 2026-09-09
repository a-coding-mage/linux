// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001 Dave Engebretsen, IBM Corporation
 * Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
 *
 * RTAS specific routines for PCI.
 *
 * Based on code from pci.c, chrp_pci.c and pSeries_pci.c
 */

// Linux and PowerPC declarations used by this implementation are supplied by
// the surrounding kernel translation.

static mut read_pci_config: i32 = 0;
static mut write_pci_config: i32 = 0;
static mut ibm_read_pci_config: i32 = 0;
static mut ibm_write_pci_config: i32 = 0;

unsafe fn config_access_valid(dn: *mut pci_dn, where_: i32) -> i32 {
    if where_ < 256 { return 1; }
    if where_ < 4096 && (*dn).pci_ext_config_space { return 1; }
    0
}

pub unsafe fn rtas_pci_dn_read_config(pdn: *mut pci_dn, where_: i32, size: i32, val: *mut u32) -> i32 {
    let mut returnval: i32 = -1;
    let mut buid: u64;
    let addr: u64;
    let ret: i32;

    if pdn.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    if config_access_valid(pdn, where_) == 0 { return PCIBIOS_BAD_REGISTER_NUMBER; }
    #[cfg(CONFIG_EEH)]
    if !(*pdn).edev.is_null() && !(*(*pdn).edev).pe.is_null() &&
       ((*(*(*pdn).edev).pe).state & EEH_PE_CFG_BLOCKED) != 0 {
        return PCIBIOS_SET_FAILED;
    }

    addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, where_);
    buid = (*(*pdn).phb).buid;
    if buid != 0 {
        ret = rtas_call(ibm_read_pci_config, 4, 2, &mut returnval, addr,
                        BUID_HI(buid), BUID_LO(buid), size);
    } else {
        ret = rtas_call(read_pci_config, 2, 2, &mut returnval, addr, size);
    }
    *val = returnval as u32;
    if ret != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn rtas_pci_read_config(_bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: *mut u32) -> i32 {
    let pdn: *mut pci_dn;
    *val = 0xffff_ffff;
    pdn = pci_get_pdn_by_devfn(_bus, devfn);
    let ret = rtas_pci_dn_read_config(pdn, where_, size, val);
    if *val == EEH_IO_ERROR_VALUE(size) && eeh_dev_check_failure(pdn_to_eeh_dev(pdn)) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }
    ret
}

pub unsafe fn rtas_pci_dn_write_config(pdn: *mut pci_dn, where_: i32, size: i32, val: u32) -> i32 {
    let buid: u64;
    let addr: u64;
    let ret: i32;
    if pdn.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    if config_access_valid(pdn, where_) == 0 { return PCIBIOS_BAD_REGISTER_NUMBER; }
    #[cfg(CONFIG_EEH)]
    if !(*pdn).edev.is_null() && !(*(*pdn).edev).pe.is_null() &&
       ((*(*(*pdn).edev).pe).state & EEH_PE_CFG_BLOCKED) != 0 {
        return PCIBIOS_SET_FAILED;
    }
    addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, where_);
    buid = (*(*pdn).phb).buid;
    if buid != 0 {
        ret = rtas_call(ibm_write_pci_config, 5, 1, core::ptr::null_mut(), addr,
                        BUID_HI(buid), BUID_LO(buid), size, val as u64);
    } else {
        ret = rtas_call(write_pci_config, 3, 1, core::ptr::null_mut(), addr, size, val as u64);
    }
    if ret != 0 { return PCIBIOS_DEVICE_NOT_FOUND; }
    PCIBIOS_SUCCESSFUL
}

unsafe fn rtas_pci_write_config(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, val: u32) -> i32 {
    let pdn = pci_get_pdn_by_devfn(bus, devfn);
    rtas_pci_dn_write_config(pdn, where_, size, val)
}

static mut rtas_pci_ops: pci_ops = pci_ops { read: rtas_pci_read_config, write: rtas_pci_write_config };

unsafe fn is_python(dev: *mut device_node) -> i32 {
    let model = of_get_property(dev, b"model\0".as_ptr() as *const i8, core::ptr::null_mut());
    if !model.is_null() && strstr(model, b"Python\0".as_ptr() as *const i8) != core::ptr::null() { return 1; }
    0
}

unsafe fn python_countermeasures(dev: *mut device_node) {
    let mut registers = resource::default();
    if of_address_to_resource(dev, 0, &mut registers) != 0 {
        printk(KERN_ERR, b"Can't get address for Python workarounds !\n\0".as_ptr());
        return;
    }
    let chip_regs = ioremap(registers.start & !0xfffff_u64, 0x100000);
    const PRG_CL_RESET_VALID: u32 = 0x00010000;
    let mut val = in_be32(chip_regs.add(0xf6030 / 4));
    if val & PRG_CL_RESET_VALID != 0 {
        printk(KERN_INFO, b"Python workaround: \0".as_ptr());
        val &= !PRG_CL_RESET_VALID;
        out_be32(chip_regs.add(0xf6030 / 4), val);
        val = in_be32(chip_regs.add(0xf6030 / 4));
        printk(b"reg0: %x\n\0".as_ptr(), val);
    }
    iounmap(chip_regs);
}

pub unsafe fn init_pci_config_tokens() {
    read_pci_config = rtas_function_token(RTAS_FN_READ_PCI_CONFIG);
    write_pci_config = rtas_function_token(RTAS_FN_WRITE_PCI_CONFIG);
    ibm_read_pci_config = rtas_function_token(RTAS_FN_IBM_READ_PCI_CONFIG);
    ibm_write_pci_config = rtas_function_token(RTAS_FN_IBM_WRITE_PCI_CONFIG);
}

pub unsafe fn get_phb_buid(phb: *mut device_node) -> u64 {
    let mut r = resource::default();
    if ibm_read_pci_config == -1 || of_address_to_resource(phb, 0, &mut r) != 0 { return 0; }
    r.start
}

unsafe fn phb_set_bus_ranges(dev: *mut device_node, phb: *mut pci_controller) -> i32 {
    let mut len: u32 = 0;
    let bus_range = of_get_property(dev, b"bus-range\0".as_ptr() as *const i8, &mut len);
    if bus_range.is_null() || len < 2 * core::mem::size_of::<i32>() as u32 { return 1; }
    (*phb).first_busno = be32_to_cpu(*bus_range.add(0));
    (*phb).last_busno = be32_to_cpu(*bus_range.add(1));
    0
}

pub unsafe fn rtas_setup_phb(phb: *mut pci_controller) -> i32 {
    let dev = (*phb).dn;
    if is_python(dev) != 0 { python_countermeasures(dev); }
    if phb_set_bus_ranges(dev, phb) != 0 { return 1; }
    (*phb).ops = &mut rtas_pci_ops;
    (*phb).buid = get_phb_buid(dev);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
