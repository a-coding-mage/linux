/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000, 2001 Keith M Wesolowski
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 */

// Linux and IP32 dependencies supplied by the surrounding kernel translation.

/*
 * Handle errors from the bridge.  This includes master and target aborts,
 * various command and address errors, and the interrupt test.  This gets
 * registered on the bridge error irq.  It's conceivable that some of these
 * conditions warrant a panic.  Anybody care to say which ones?
 */
unsafe extern "C" {
    static mut mace: *mut mace_t;
    static mut PCIBIOS_MIN_IO: u32;
    static mut iomem_resource: resource;
    static mut ioport_resource: resource;

    static mace_pci_ops: pci_ops;

    fn printk(format: *const u8, ...) -> i32;
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: u32,
        name: *const u8,
        dev: *mut core::ffi::c_void,
    ) -> i32;
    fn register_pci_controller(controller: *mut pci_controller);
    fn BUG_ON(condition: bool);
}

#[repr(C)]
struct mace_t {
    pci: mace_pci,
}

#[repr(C)]
struct mace_pci {
    error: u32,
    error_addr: u32,
    control: u32,
    rev: u32,
}

#[repr(C)]
struct resource {
    name: *const u8,
    start: usize,
    end: usize,
    flags: u64,
}

#[repr(C)]
struct pci_ops {
    _opaque: [u8; 0],
}

#[repr(C)]
struct pci_controller {
    pci_ops: *const pci_ops,
    mem_resource: *mut resource,
    io_resource: *mut resource,
    mem_offset: usize,
    io_offset: usize,
    io_map_base: usize,
}

type irqreturn_t = i32;

const IRQ_HANDLED: irqreturn_t = 1;
const IORESOURCE_MEM: u64 = 0x00000200;
const IORESOURCE_IO: u64 = 0x00000100;
const MACE_PCI_BRIDGE_IRQ: u32 = 0;
const MACEPCI_ERROR_MEMORY_ADDR: u32 = 1 << 0;
const MACEPCI_ERROR_CONFIG_ADDR: u32 = 1 << 1;
const MACEPCI_ERROR_MASTER_ABORT: u32 = 1 << 2;
const MACEPCI_ERROR_TARGET_ABORT: u32 = 1 << 3;
const MACEPCI_ERROR_DATA_PARITY_ERR: u32 = 1 << 4;
const MACEPCI_ERROR_RETRY_ERR: u32 = 1 << 5;
const MACEPCI_ERROR_ILLEGAL_CMD: u32 = 1 << 6;
const MACEPCI_ERROR_SYSTEM_ERR: u32 = 1 << 7;
const MACEPCI_ERROR_PARITY_ERR: u32 = 1 << 8;
const MACEPCI_ERROR_OVERRUN: u32 = 1 << 9;
const MACEPCI_ERROR_SIG_TABORT: u32 = 1 << 10;
const MACEPCI_ERROR_INTERRUPT_TEST: u32 = 1 << 11;
const MACEPCI_HI_MEMORY: usize = 0x100000000;
const MACEPCI_LOW_MEMORY: usize = 0x08000000;
const MACEPCI_LOW_IO: usize = 0x00000000;

// The original source selects these values with CONFIG_64BIT.
#[cfg(target_pointer_width = "64")]
const MACE_PCI_MEM_OFFSET: usize = 0x200000000;
#[cfg(not(target_pointer_width = "64"))]
const MACE_PCI_MEM_OFFSET: usize = MACEPCI_LOW_MEMORY - 0x80000000;

unsafe fn macepci_error(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    let mut s: u8;
    let mut flags = (*mace).pci.error;
    let addr = (*mace).pci.error_addr;

    if flags & MACEPCI_ERROR_MEMORY_ADDR != 0 {
        s = b'M';
    } else if flags & MACEPCI_ERROR_CONFIG_ADDR != 0 {
        s = b'C';
    } else {
        s = b'X';
    }

    macro_rules! clear_error {
        ($bit:ident, $fmt:literal) => {
            if flags & $bit != 0 {
                printk(concat!($fmt, "\n", "\0").as_ptr(), addr, s as i32);
                flags &= !$bit;
            }
        };
    }

    clear_error!(MACEPCI_ERROR_MASTER_ABORT, "MACEPCI: Master abort at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_TARGET_ABORT, "MACEPCI: Target abort at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_DATA_PARITY_ERR, "MACEPCI: Data parity error at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_RETRY_ERR, "MACEPCI: Retry error at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_ILLEGAL_CMD, "MACEPCI: Illegal command at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_SYSTEM_ERR, "MACEPCI: System error at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_PARITY_ERR, "MACEPCI: Parity error at 0x%08x (%c)");
    clear_error!(MACEPCI_ERROR_OVERRUN, "MACEPCI: Overrun error at 0x%08x (%c)");

    if flags & MACEPCI_ERROR_SIG_TABORT != 0 {
        printk(b"MACEPCI: Signaled target abort (clearing)\n\0".as_ptr());
        flags &= !MACEPCI_ERROR_SIG_TABORT;
    }
    if flags & MACEPCI_ERROR_INTERRUPT_TEST != 0 {
        printk(b"MACEPCI: Interrupt test triggered (clearing)\n\0".as_ptr());
        flags &= !MACEPCI_ERROR_INTERRUPT_TEST;
    }

    (*mace).pci.error = flags;
    IRQ_HANDLED
}

#[cfg(target_pointer_width = "64")]
static mut mace_pci_mem_resource: resource = resource {
    name: b"SGI O2 PCI MEM\0".as_ptr(),
    start: MACEPCI_HI_MEMORY,
    end: 0x2FFFFFFFF,
    flags: IORESOURCE_MEM,
};
#[cfg(not(target_pointer_width = "64"))]
static mut mace_pci_mem_resource: resource = resource {
    name: b"SGI O2 PCI MEM\0".as_ptr(),
    start: MACEPCI_LOW_MEMORY,
    end: MACEPCI_LOW_MEMORY + 0x2000000 - 1,
    flags: IORESOURCE_MEM,
};

static mut mace_pci_io_resource: resource = resource {
    name: b"SGI O2 PCI IO\0".as_ptr(),
    start: 0,
    end: usize::MAX,
    flags: IORESOURCE_IO,
};

static mut mace_pci_controller: pci_controller = pci_controller {
    pci_ops: &mace_pci_ops,
    mem_resource: &raw mut mace_pci_mem_resource,
    io_resource: &raw mut mace_pci_io_resource,
    mem_offset: MACE_PCI_MEM_OFFSET,
    io_offset: 0,
    io_map_base: MACEPCI_LOW_IO,
};

unsafe fn mace_init() -> i32 {
    PCIBIOS_MIN_IO = 0x1000;

    /* Clear any outstanding errors and enable interrupts */
    (*mace).pci.error_addr = 0;
    (*mace).pci.error = 0;
    (*mace).pci.control = 0xff008500;

    printk(b"MACE PCI rev %d\n\0".as_ptr(), (*mace).pci.rev);

    let irq_result = request_irq(
        MACE_PCI_BRIDGE_IRQ,
        macepci_error,
        0,
        b"MACE PCI error\0".as_ptr(),
        core::ptr::null_mut(),
    );
    BUG_ON(irq_result != 0);

    /* extend memory resources */
    iomem_resource.end = mace_pci_mem_resource.end;
    ioport_resource = mace_pci_io_resource;

    register_pci_controller(&raw mut mace_pci_controller);
    0
}

// Original: arch_initcall(mace_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
