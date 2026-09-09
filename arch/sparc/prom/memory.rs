// SPDX-License-Identifier: GPL-2.0
/* memory.c: Prom routine for acquiring various bits of information
 *           about RAM on the machine, both virtual and physical.
 *
 * Copyright (C) 1995, 2008 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1997 Michael A. Griffith (grif@acm.org)
 */

// External kernel, OpenPROM, and page definitions are supplied by other files.

unsafe fn prom_meminit_v0() -> i32 {
    let mut p: *mut linux_mlist_v0;
    let mut index: i32;

    index = 0;
    p = (*romvec).pv_v0mem.v0_available as *mut linux_mlist_v0;
    while !p.is_null() {
        (*sp_banks.add(index as usize)).base_addr = (*p).start_adr as libc::c_ulong;
        (*sp_banks.add(index as usize)).num_bytes = (*p).num_bytes;
        index += 1;
        p = (*p).theres_more;
    }

    index
}

unsafe fn prom_meminit_v2() -> i32 {
    let mut reg: [linux_prom_registers; 64] = core::mem::zeroed();
    let node: phandle;
    let size: i32;
    let num_ents: i32;
    let mut i: i32;

    node = prom_searchsiblings(prom_getchild(prom_root_node), b"memory\0".as_ptr() as *const libc::c_char);
    size = prom_getproperty(
        node,
        b"available\0".as_ptr() as *const libc::c_char,
        reg.as_mut_ptr() as *mut libc::c_char,
        core::mem::size_of::<[linux_prom_registers; 64]>() as i32,
    );
    num_ents = size / core::mem::size_of::<linux_prom_registers>() as i32;

    i = 0;
    while i < num_ents {
        (*sp_banks.add(i as usize)).base_addr = reg[i as usize].phys_addr;
        (*sp_banks.add(i as usize)).num_bytes = reg[i as usize].reg_size;
        i += 1;
    }

    num_ents
}

unsafe extern "C" fn sp_banks_cmp(a: *const libc::c_void, b: *const libc::c_void) -> i32 {
    let x = a as *const sparc_phys_banks;
    let y = b as *const sparc_phys_banks;

    if (*x).base_addr > (*y).base_addr {
        return 1;
    }
    if (*x).base_addr < (*y).base_addr {
        return -1;
    }
    0
}

/* Initialize the memory lists based upon the prom version. */
pub unsafe fn prom_meminit() {
    let mut i: i32;
    let mut num_ents: i32 = 0;

    match prom_vers {
        PROM_V0 => {
            num_ents = prom_meminit_v0();
        }
        PROM_V2 | PROM_V3 => {
            num_ents = prom_meminit_v2();
        }
        _ => {}
    }
    sort(
        sp_banks as *mut libc::c_void,
        num_ents as usize,
        core::mem::size_of::<sparc_phys_banks>(),
        Some(sp_banks_cmp),
        core::ptr::null_mut(),
    );

    /* Sentinel.  */
    (*sp_banks.add(num_ents as usize)).base_addr = 0xdeadbeef;
    (*sp_banks.add(num_ents as usize)).num_bytes = 0;

    i = 0;
    while i < num_ents {
        (*sp_banks.add(i as usize)).num_bytes &= PAGE_MASK;
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
