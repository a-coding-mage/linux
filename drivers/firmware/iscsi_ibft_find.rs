// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright 2007-2010 Red Hat, Inc.
 *  by Peter Jones <pjones@redhat.com>
 *  Copyright 2007 IBM, Inc.
 *  by Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
 *  Copyright 2008
 *  by Konrad Rzeszutek <ketuzsezr@darnok.org>
 *
 * This code finds the iSCSI Boot Format Table.
 */

// Linux kernel dependencies supplied externally.

extern "C" {
    fn efi_enabled(feature: u32) -> bool;
    fn early_memunmap(addr: *mut core::ffi::c_void, size: usize);
    fn early_memremap_ro(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn memblock_reserve(addr: usize, size: usize);
}

const PAGE_SIZE: usize = 4096;

// EFI_BOOT, IBFT_START, and IBFT_END are build-provided kernel constants.

/* Physical location of iSCSI Boot Format Table. */
#[no_mangle]
pub static mut ibft_phys_addr: usize = 0;

#[repr(C)]
struct IbftSign {
    sign: *const u8,
}

static IBFT_SIGNS: [IbftSign; 2] = [
    IbftSign { sign: b"iBFT\0".as_ptr() },
    IbftSign { sign: b"BIFT\0".as_ptr() }, // Broadcom iSCSI Offload
];

const IBFT_SIGN_LEN: usize = 4;
const VGA_MEM: usize = 0xA0000; // VGA buffer
const VGA_SIZE: usize = 0x20000; // 128kB

/*
 * Routine used to find and reserve the iSCSI Boot Format Table
 */
#[no_mangle]
pub unsafe extern "C" fn reserve_ibft_region() {
    let mut pos: usize;
    let mut virt_pos: usize = 0;
    let mut len: u32 = 0;
    let mut virt: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut i: usize;

    ibft_phys_addr = 0;

    /* iBFT 1.03 section 1.4.3.1 mandates that UEFI machines will
     * only use ACPI for this
     */
    if efi_enabled(EFI_BOOT) {
        return;
    }

    pos = IBFT_START;
    while pos < IBFT_END {
        /* The table can't be inside the VGA BIOS reserved space,
         * so skip that area */
        if pos == VGA_MEM {
            pos += VGA_SIZE;
        }

        /* Map page by page */
        if pos % PAGE_SIZE == 0 {
            if !virt.is_null() {
                early_memunmap(virt, PAGE_SIZE);
            }
            virt = early_memremap_ro(pos, PAGE_SIZE);
            virt_pos = pos;
        }

        i = 0;
        while i < IBFT_SIGNS.len() {
            let sig = IBFT_SIGNS[i].sign;
            let candidate = (virt as *const u8).add(pos - virt_pos);
            if core::slice::from_raw_parts(candidate, IBFT_SIGN_LEN)
                == core::slice::from_raw_parts(sig, IBFT_SIGN_LEN)
            {
                let addr = (candidate.add(4)) as *const usize;
                len = core::ptr::read_unaligned(addr as *const u32);
                /* if the length of the table extends past 1M,
                 * the table cannot be valid. */
                if pos + len as usize <= IBFT_END - 1 {
                    ibft_phys_addr = pos;
                    memblock_reserve(ibft_phys_addr, (len as usize + PAGE_SIZE - 1) & !(PAGE_SIZE - 1));
                    // pr_info("iBFT found at %pa.\n", &ibft_phys_addr);
                    break;
                }
            }
            i += 1;
        }

        if ibft_phys_addr != 0 {
            break;
        }
        pos += 16;
    }

    early_memunmap(virt, PAGE_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
