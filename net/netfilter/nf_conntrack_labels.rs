// SPDX-License-Identifier: GPL-2.0-only
/*
 * test/set flag bits stored in conntrack extension area.
 *
 * (C) 2013 Astaro GmbH & Co KG
 */

// External Linux/netfilter declarations and build-time definitions are supplied
// by the surrounding translation unit.

unsafe fn replace_u32(address: *mut u32, mask: u32, new: u32) -> i32 {
    let mut old: u32;
    let mut tmp: u32;

    loop {
        old = unsafe { core::ptr::read_volatile(address) };
        tmp = (old & mask) ^ new;
        if old == tmp {
            return 0;
        }
        if unsafe { cmpxchg(address, old, tmp) } == old {
            break;
        }
    }

    1
}

pub unsafe fn nf_connlabels_replace(
    ct: *mut nf_conn,
    data: *const u32,
    mask: *const u32,
    mut words32: u32,
) -> i32 {
    let labels: *mut nf_conn_labels;
    let mut size: usize;
    let mut i: u32;
    let mut changed: i32 = 0;
    let dst: *mut u32;

    labels = unsafe { nf_ct_labels_find(ct) };
    if labels.is_null() {
        return -ENOSPC;
    }

    size = core::mem::size_of_val(unsafe { &(*labels).bits });
    if size < (words32 as usize * core::mem::size_of::<u32>()) {
        words32 = (size / core::mem::size_of::<u32>()) as u32;
    }

    dst = unsafe { (*labels).bits.as_mut_ptr() as *mut u32 };
    i = 0;
    while i < words32 {
        let m = if !mask.is_null() {
            !(unsafe { *mask.add(i as usize) })
        } else {
            0
        };
        changed |= unsafe { replace_u32(dst.add(i as usize), m, *data.add(i as usize)) };
        i += 1;
    }

    size /= core::mem::size_of::<u32>();
    i = words32;
    while (i as usize) < size {
        // pad
        unsafe { replace_u32(dst.add(i as usize), 0, 0) };
        i += 1;
    }

    if changed != 0 {
        unsafe { nf_conntrack_event_cache(IPCT_LABEL, ct) };
    }
    0
}

// EXPORT_SYMBOL_GPL(nf_connlabels_replace);

pub unsafe fn nf_connlabels_get(net: *mut net, bits: u32) -> i32 {
    let v: i32;

    if bit_word(bits) >= NF_CT_LABELS_MAX_SIZE / core::mem::size_of::<libc::c_long>() {
        return -ERANGE;
    }

    // BUILD_BUG_ON(NF_CT_LABELS_MAX_SIZE / sizeof(long) >= U8_MAX);
    v = unsafe { atomic_inc_return_relaxed(&mut (*net).ct.labels_used) };
    unsafe { WARN_ON_ONCE(v <= 0) };

    0
}

// EXPORT_SYMBOL_GPL(nf_connlabels_get);

pub unsafe fn nf_connlabels_put(net: *mut net) {
    let v = unsafe { atomic_dec_return_relaxed(&mut (*net).ct.labels_used) };

    unsafe { WARN_ON_ONCE(v < 0) };
}

// EXPORT_SYMBOL_GPL(nf_connlabels_put);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
