// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA PCIe mux handling
 *
 * Copyright (C) 2010  Paul Mundt
 */
// #define pr_fmt(fmt) "PCI: " fmt
// Linux and machine-specific dependencies are supplied by other files.

static mut SLOT4EN: ::core::ffi::c_uint = 0;

pub unsafe extern "C" fn pcibios_setup(str_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if libc::strcmp(str_, b"slot4en\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        SLOT4EN = 1;
        return core::ptr::null_mut();
    }

    str_
}

unsafe extern "C" fn sdk7786_pci_init() -> ::core::ffi::c_int {
    let mut data: u16 = fpga_read_reg(PCIECR);

    /*
     * Enable slot #4 if it's been specified on the command line.
     *
     * Optionally reroute if slot #4 has a card present while slot #3
     * does not, regardless of command line value.
     *
     * Card presence is logically inverted.
     */
    if SLOT4EN == 0 {
        SLOT4EN = ((data & PCIECR_PRST4) == 0 && (data & PCIECR_PRST3) != 0) as ::core::ffi::c_uint;
    }
    if SLOT4EN != 0 {
        pr_info!("Activating PCIe slot#4 (disabling slot#3)\n");

        data &= !PCIECR_PCIEMUX1;
        fpga_write_reg(data, PCIECR);

        /* Warn about forced rerouting if slot#3 is occupied */
        if (data & PCIECR_PRST3) == 0 {
            pr_warn!("Unreachable card detected in slot#3\n");
            return -EBUSY;
        }
    } else {
        pr_info!("PCIe slot#4 disabled\n");
    }

    0
}

// postcore_initcall(sdk7786_pci_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
