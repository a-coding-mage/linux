#[repr(C)]
pub struct soft_mask_table_entry {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct restart_table_entry {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub fixup: ::core::ffi::c_ulong,
}

extern "C" {
    pub static mut __start___soft_mask_table: [soft_mask_table_entry; 0];
    pub static mut __stop___soft_mask_table: [soft_mask_table_entry; 0];

    pub static mut __start___restart_table: [restart_table_entry; 0];
    pub static mut __stop___restart_table: [restart_table_entry; 0];
}

/* Given an address, look for it in the soft mask table */
pub unsafe fn search_kernel_soft_mask_table(addr: ::core::ffi::c_ulong) -> bool {
    let mut smte = __start___soft_mask_table.as_mut_ptr();
    let stop = __stop___soft_mask_table.as_mut_ptr();

    while (smte as usize) < (stop as usize) {
        let start = (*smte).start;
        let end = (*smte).end;

        if addr >= start && addr < end {
            return true;
        }

        smte = smte.add(1);
    }
    false
}
// NOKPROBE_SYMBOL(search_kernel_soft_mask_table);

/* Given an address, look for it in the kernel exception table */
pub unsafe fn search_kernel_restart_table(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mut rte = __start___restart_table.as_mut_ptr();
    let stop = __stop___restart_table.as_mut_ptr();

    while (rte as usize) < (stop as usize) {
        let start = (*rte).start;
        let end = (*rte).end;
        let fixup = (*rte).fixup;

        if addr >= start && addr < end {
            return fixup;
        }

        rte = rte.add(1);
    }
    0
}
// NOKPROBE_SYMBOL(search_kernel_restart_table);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
