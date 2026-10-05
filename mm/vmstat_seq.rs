// SPDX-License-Identifier: GPL-2.0-only
unsafe extern "C" fn frag_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut node = *pos;
        let mut p = first_online_pgdat();
        while !p.is_null() && node != 0 {
            node = node.wrapping_sub(1);
            p = next_online_pgdat(p);
        }
        p.cast()
    }
}
unsafe extern "C" fn frag_next(
    _m: *mut seq_file,
    arg: *mut c_void,
    pos: *mut loff_t,
) -> *mut c_void {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        *pos = (*pos).wrapping_add(1);
        next_online_pgdat(arg.cast()).cast()
    }
}
unsafe extern "C" fn frag_stop(_m: *mut seq_file, _arg: *mut c_void) {}
unsafe fn walk_zones_in_node(
    m: *mut seq_file,
    p: *mut pglist_data,
    assert_populated: bool,
    nolock: bool,
    print: unsafe fn(*mut seq_file, *mut pglist_data, *mut zone),
) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        for i in 0..MAX_NR_ZONES as usize {
            let z = addr_of_mut!((*p).node_zones[i]);
            if assert_populated && !populated(z) {
                continue;
            }
            let flags = if nolock {
                0
            } else {
                rust_vmstat_zone_lock_irqsave(z)
            };
            print(m, p, z);
            if !nolock {
                rust_vmstat_zone_unlock_irqrestore(z, flags);
            }
        }
    }
}
