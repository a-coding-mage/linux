// SPDX-License-Identifier: GPL-2.0
/*
 * ppc64 "iomap" interface implementation.
 *
 * (C) Copyright 2004 Linus Torvalds
 */
// C dependencies: linux/init.h, linux/pci.h, linux/mm.h, linux/export.h,
// linux/io.h, and asm/pci-bridge.h.

static mut HOSE_SPINLOCK: Spinlock = DEFINE_SPINLOCK!();
pub static mut hose_list: ListHead = LIST_HEAD!();

pub static mut isa_io_base: c_ulong = 0;
EXPORT_SYMBOL!(isa_io_base);

unsafe fn pcibios_io_size(hose: *const pci_controller) -> resource_size_t {
    resource_size(unsafe { &(*hose).io_resource })
}

pub unsafe fn pcibios_vaddr_is_ioport(address: *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    let mut hose: *mut pci_controller;
    let mut size: resource_size_t;

    spin_lock(&raw mut HOSE_SPINLOCK);
    list_for_each_entry!(hose, &raw mut hose_list, list_node) {
        size = pcibios_io_size(hose);
        if address >= (*hose).io_base_virt
            && address < ((*hose).io_base_virt).add(size as usize)
        {
            ret = 1;
            break;
        }
    }
    spin_unlock(&raw mut HOSE_SPINLOCK);
    ret
}

/* Display the domain number in /proc */
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> c_int {
    pci_domain_nr(bus)
}

pub unsafe fn pci_iounmap(dev: *mut pci_dev, addr: *mut c_void) {
    if isa_vaddr_is_ioport(addr) != 0 {
        return;
    }
    if pcibios_vaddr_is_ioport(addr) != 0 {
        return;
    }
    iounmap(addr);
}
EXPORT_SYMBOL!(pci_iounmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
