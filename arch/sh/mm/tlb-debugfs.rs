/*
 * arch/sh/mm/tlb-debugfs.c
 *
 * debugfs ops for SH-4 ITLB/UTLBs.
 *
 * Copyright (C) 2010  Matt Fleming
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum tlb_type {
    TLB_TYPE_ITLB,
    TLB_TYPE_UTLB,
}

#[repr(C)]
struct TlbSize {
    bits: i32,
    size: *const core::ffi::c_char,
}

static tlb_sizes: [TlbSize; 8] = [
    TlbSize { bits: 0x0, size: b"  1KB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x1, size: b"  4KB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x2, size: b"  8KB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x4, size: b" 64KB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x5, size: b"256KB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x7, size: b"  1MB\0".as_ptr() as *const _ },
    TlbSize { bits: 0x8, size: b"  4MB\0".as_ptr() as *const _ },
    TlbSize { bits: 0xc, size: b" 64MB\0".as_ptr() as *const _ },
];

unsafe fn tlb_seq_show(file: *mut seq_file, _iter: *mut core::ffi::c_void) -> i32 {
    let tlb_type = (*file).private as u32;
    let (mut addr1, mut addr2, mut data1, mut data2): (usize, usize, usize, usize);
    let mut flags: usize = 0;
    let mmucr: usize;
    let (nentries, urb): (u32, u32);

    mmucr = __raw_readl(MMUCR);
    if (mmucr & 0x1) == 0 {
        seq_printf(file, b"address translation disabled\0".as_ptr() as *const _,);
        return 0;
    }

    if tlb_type == TLB_TYPE_ITLB as u32 {
        addr1 = MMU_ITLB_ADDRESS_ARRAY; addr2 = MMU_ITLB_ADDRESS_ARRAY2;
        data1 = MMU_ITLB_DATA_ARRAY; data2 = MMU_ITLB_DATA_ARRAY2;
        nentries = 4;
    } else {
        addr1 = MMU_UTLB_ADDRESS_ARRAY; addr2 = MMU_UTLB_ADDRESS_ARRAY2;
        data1 = MMU_UTLB_DATA_ARRAY; data2 = MMU_UTLB_DATA_ARRAY2;
        nentries = 64;
    }

    local_irq_save(&mut flags);
    jump_to_uncached();
    urb = ((mmucr & MMUCR_URB) >> MMUCR_URB_SHIFT) as u32;
    let mut urb = if urb == 0 { MMUCR_URB_NENTRIES + 1 } else { urb };

    if tlb_type == TLB_TYPE_ITLB as u32 {
        addr1 = MMU_ITLB_ADDRESS_ARRAY; addr2 = MMU_ITLB_ADDRESS_ARRAY2;
        data1 = MMU_ITLB_DATA_ARRAY; data2 = MMU_ITLB_DATA_ARRAY2;
    } else {
        addr1 = MMU_UTLB_ADDRESS_ARRAY; addr2 = MMU_UTLB_ADDRESS_ARRAY2;
        data1 = MMU_UTLB_DATA_ARRAY; data2 = MMU_UTLB_DATA_ARRAY2;
    }

    seq_printf(file, b"entry:     vpn        ppn     asid  size valid wired\n\0".as_ptr() as *const _,);
    for entry in 0..nentries {
        let val = __raw_readl(addr1 | ((entry as usize) << MMU_TLB_ENTRY_SHIFT)); ctrl_barrier();
        let vpn = val & 0xfffffc00; let valid = val & 0x100;
        let val = __raw_readl(addr2 | ((entry as usize) << MMU_TLB_ENTRY_SHIFT)); ctrl_barrier();
        let asid = val & MMU_CONTEXT_ASID_MASK;
        let val = __raw_readl(data1 | ((entry as usize) << MMU_TLB_ENTRY_SHIFT)); ctrl_barrier();
        let ppn = (val & 0x0ffffc00) << 4;
        let val = __raw_readl(data2 | ((entry as usize) << MMU_TLB_ENTRY_SHIFT)); ctrl_barrier();
        let size = (val & 0xf0) >> 4;
        let mut sz = b"    ?\0".as_ptr();
        for item in &tlb_sizes { if item.bits as usize == size { sz = item.size as *const u8; break; } }
        let _ = (vpn, ppn, asid, sz, valid, urb, entry);
    }
    back_to_cached(); local_irq_restore(flags); 0
}

unsafe fn tlb_debugfs_open(inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, Some(tlb_seq_show), (*inode).i_private)
}

static tlb_debugfs_fops: file_operations = file_operations {
    owner: THIS_MODULE, open: Some(tlb_debugfs_open), read: Some(seq_read),
    llseek: Some(seq_lseek), release: Some(single_release),
};

unsafe fn tlb_debugfs_init() -> i32 {
    debugfs_create_file(b"itlb\0".as_ptr(), S_IRUSR, arch_debugfs_dir,
        TLB_TYPE_ITLB as usize as *mut _, &tlb_debugfs_fops);
    debugfs_create_file(b"utlb\0".as_ptr(), S_IRUSR, arch_debugfs_dir,
        TLB_TYPE_UTLB as usize as *mut _, &tlb_debugfs_fops);
    0
}

module_init!(tlb_debugfs_init);
module_license!(b"GPL v2\0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
