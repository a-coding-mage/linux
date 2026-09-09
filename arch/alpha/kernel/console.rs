// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/console.c
 *
 * Architecture-specific specific support for VGA device on
 * non-0 I/O hose
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[cfg(CONFIG_VGA_HOSE)]
static mut pci_vga_hose: *mut pci_controller = core::ptr::null_mut();

#[cfg(CONFIG_VGA_HOSE)]
static mut alpha_vga: resource = resource {
    name: b"alpha-vga+\0".as_ptr() as *const i8,
    flags: IORESOURCE_IO,
    start: 0x3c0,
    end: 0x3df,
};

#[cfg(CONFIG_VGA_HOSE)]
unsafe extern "C" fn default_vga_hose_select(
    h1: *mut pci_controller,
    h2: *mut pci_controller,
) -> *mut pci_controller {
    if (*h2).index < (*h1).index {
        h2
    } else {
        h1
    }
}

#[cfg(CONFIG_VGA_HOSE)]
pub unsafe extern "C" fn locate_and_init_vga(
    mut sel_func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>,
) {
    let mut hose: *mut pci_controller = core::ptr::null_mut();
    let mut dev: *mut pci_dev = core::ptr::null_mut();

    /* Default the select function */
    if sel_func.is_none() {
        sel_func = Some(core::mem::transmute(default_vga_hose_select));
    }

    /* Find the console VGA device */
    loop {
        dev = pci_get_class(PCI_CLASS_DISPLAY_VGA << 8, dev);
        if dev.is_null() {
            break;
        }
        if hose.is_null() {
            hose = (*dev).sysdata as *mut pci_controller;
        } else {
            hose = (sel_func.unwrap())(
                hose as *mut c_void,
                (*dev).sysdata as *mut c_void,
            ) as *mut pci_controller;
        }
    }

    /* Did we already initialize the correct one? Is there one? */
    if hose.is_null() || (conswitchp == &mut vga_con && pci_vga_hose == hose) {
        return;
    }

    /* Create a new VGA ioport resource WRT the hose it is on. */
    alpha_vga.start += (*(*hose).io_space).start;
    alpha_vga.end += (*(*hose).io_space).start;
    request_resource((*hose).io_space, &mut alpha_vga);

    /* Set the VGA hose and init the new console. */
    pci_vga_hose = hose;
    console_lock();
    do_take_over_console(&mut vga_con, 0, MAX_NR_CONSOLES - 1, 1);
    console_unlock();
}

#[cfg(CONFIG_VGA_HOSE)]
pub unsafe extern "C" fn find_console_vga_hose() {
    let pu64 = ((hwrpb as u64) + (*hwrpb).ctbt_offset) as *mut u64;

    if *pu64.add(7) == 3 {
        /* TERM_TYPE == graphics */
        let mut hose: *mut pci_controller;
        let h = ((*pu64.add(30) >> 24) & 0xff) as i32;

        /*
         * Our hose numbering DOES match the console's, so find
         * the right one...
         */
        hose = hose_head;
        while !hose.is_null() {
            if (*hose).index == h {
                break;
            }
            hose = (*hose).next;
        }

        if !hose.is_null() {
            printk(b"Console graphics on hose %d\n\0".as_ptr() as *const i8, h);
            pci_vga_hose = hose;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
