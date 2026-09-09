// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// Linux kernel, printk, ptrace, and PowerPC register definitions are supplied
// by the surrounding kernel translation.

#[repr(C)]
pub struct pt_regs {
    pub msr: ::core::ffi::c_ulong,
    pub nip: ::core::ffi::c_ulong,
    pub dar: ::core::ffi::c_ulong,
}

extern "C" {
    fn pr_err(fmt: *const u8, ...);
    fn pr_cont(fmt: *const u8, ...);
    fn bad_page_fault(regs: *mut pt_regs, sig: ::core::ffi::c_int);
    static SIGBUS: ::core::ffi::c_int;
}

pub unsafe fn machine_check_8xx(regs: *mut pt_regs) -> ::core::ffi::c_int {
    let reason: ::core::ffi::c_ulong = (*regs).msr;

    pr_err(b"Machine check in kernel mode.\n\0".as_ptr());
    pr_err(
        b"Caused by (from SRR1=%lx): \0".as_ptr(),
        reason,
    );
    if reason & 0x40000000 != 0 {
        pr_cont(
            b"Fetch error at address %lx\n\0".as_ptr(),
            (*regs).nip,
        );
    } else {
        pr_cont(
            b"Data access error at address %lx\n\0".as_ptr(),
            (*regs).dar,
        );
    }

    // CONFIG_PCI controls this block at build time.
    #[cfg(CONFIG_PCI)]
    {
        /* the qspan pci read routines can cause machine checks -- Cort
         *
         * yuck !!! that totally needs to go away ! There are better ways
         * to deal with that than having a wart in the mcheck handler.
         * -- BenH
         */
        bad_page_fault(regs, SIGBUS);
        return 1;
    }
    #[cfg(not(CONFIG_PCI))]
    {
        return 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
