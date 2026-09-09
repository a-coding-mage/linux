// SPDX-License-Identifier: GPL-2.0
/*
 * This file implement the Wireless Extensions proc API.
 *
 * Authors : Jean Tourrilhes - HPL - <jt@hpl.hp.com>
 * Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
 */

/*
 * The /proc/net/wireless file is a human readable user-space interface
 * exporting various wireless specific statistics from the wireless devices.
 * This interface is a pure clone of /proc/net/dev (in net/core/dev.c).
 * The content of the file is basically the content of "struct iw_statistics".
 */

// C headers and configuration symbols are supplied by the surrounding kernel
// translation unit.

unsafe fn wireless_seq_printf_stats(seq: *mut seq_file, dev: *mut net_device) {
    // Get stats from the driver
    let mut stats: *mut iw_statistics = get_wireless_stats(dev);
    static mut NULLSTATS: iw_statistics = iw_statistics::zeroed();

    // show device if it's wireless regardless of current stats
    if stats.is_null() {
        // CONFIG_WIRELESS_EXT
        if (*dev).wireless_handlers != core::ptr::null_mut() {
            stats = core::ptr::addr_of_mut!(NULLSTATS);
        }
        // CONFIG_CFG80211
        if (*dev).ieee80211_ptr != core::ptr::null_mut() {
            stats = core::ptr::addr_of_mut!(NULLSTATS);
        }
    }

    if !stats.is_null() {
        seq_printf(
            seq,
            "%6s: %04x  %3d%c  %3d%c  %3d%c  %6d %6d %6d %6d %6d   %6d\n",
            (*dev).name,
            (*stats).status,
            (*stats).qual.qual,
            if (*stats).qual.updated & IW_QUAL_QUAL_UPDATED != 0 { b'.' } else { b' ' },
            ((*stats).qual.level as i32)
                - if (*stats).qual.updated & IW_QUAL_DBM != 0 { 0x100 } else { 0 },
            if (*stats).qual.updated & IW_QUAL_LEVEL_UPDATED != 0 { b'.' } else { b' ' },
            ((*stats).qual.noise as i32)
                - if (*stats).qual.updated & IW_QUAL_DBM != 0 { 0x100 } else { 0 },
            if (*stats).qual.updated & IW_QUAL_NOISE_UPDATED != 0 { b'.' } else { b' ' },
            (*stats).discard.nwid,
            (*stats).discard.code,
            (*stats).discard.fragment,
            (*stats).discard.retries,
            (*stats).discard.misc,
            (*stats).miss.beacon,
        );

        if stats != core::ptr::addr_of_mut!(NULLSTATS) {
            (*stats).qual.updated &= !IW_QUAL_ALL_UPDATED;
        }
    }
}

/* ---------------------------------------------------------------- */
/* Print info for /proc/net/wireless (print all entries) */
unsafe fn wireless_dev_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    might_sleep();

    if v == SEQ_START_TOKEN {
        seq_printf(
            seq,
            "Inter-| sta-|   Quality        |   Discarded packets               | Missed | WE\n face | tus | link level noise |  nwid  crypt   frag  retry   misc | beacon | %d\n",
            WIRELESS_EXT,
        );
    } else {
        wireless_seq_printf_stats(seq, v as *mut net_device);
    }
    0
}

unsafe fn wireless_dev_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let net: *mut net = seq_file_net(seq);
    let mut off: loff_t;
    let mut dev: *mut net_device;

    rtnl_lock();
    if *pos == 0 {
        return SEQ_START_TOKEN;
    }

    off = 1;
    for_each_netdev(net, dev) {
        if {
            off += 1;
            off - 1
        } == *pos {
            return dev as *mut core::ffi::c_void;
        }
    }
    core::ptr::null_mut()
}

unsafe fn wireless_dev_seq_next(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let net: *mut net = seq_file_net(seq);

    *pos += 1;

    if v == SEQ_START_TOKEN {
        first_net_device(net) as *mut core::ffi::c_void
    } else {
        next_net_device(v as *mut net_device) as *mut core::ffi::c_void
    }
}

unsafe fn wireless_dev_seq_stop(_seq: *mut seq_file, _v: *mut core::ffi::c_void) {
    rtnl_unlock();
}

static wireless_seq_ops: seq_operations = seq_operations {
    start: Some(wireless_dev_seq_start),
    next: Some(wireless_dev_seq_next),
    stop: Some(wireless_dev_seq_stop),
    show: Some(wireless_dev_seq_show),
};

unsafe fn wext_proc_init(net: *mut net) -> i32 {
    // Create /proc/net/wireless entry
    if proc_create_net(
        b"wireless\0".as_ptr() as *const core::ffi::c_char,
        0o444,
        (*net).proc_net,
        &wireless_seq_ops,
        core::mem::size_of::<seq_net_private>(),
    )
    .is_null()
    {
        return -ENOMEM;
    }

    0
}

unsafe fn wext_proc_exit(net: *mut net) {
    remove_proc_entry(
        b"wireless\0".as_ptr() as *const core::ffi::c_char,
        (*net).proc_net,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
