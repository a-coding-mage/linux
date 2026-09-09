/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux Rust declarations:
// linux/seq_file.h and net/net_trackers.h

#[repr(C)]
pub struct net;

unsafe extern "C" {
    pub static mut init_net: net;
}

#[repr(C)]
pub struct netns_tracker;

#[repr(C)]
pub struct seq_net_private {
    // CONFIG_NET_NS controls whether network namespaces are enabled.
    #[cfg(CONFIG_NET_NS)]
    pub net: *mut net,
    #[cfg(CONFIG_NET_NS)]
    pub ns_tracker: netns_tracker,
}

// This function is an inline helper in the C header.
#[inline]
pub unsafe fn seq_file_net(seq: *mut seq_file) -> *mut net {
    #[cfg(CONFIG_NET_NS)]
    {
        return (*( (*(seq as *mut seq_file)).private as *mut seq_net_private)).net;
    }
    #[cfg(not(CONFIG_NET_NS))]
    {
        return &raw mut init_net;
    }
}

/*
 * This one is needed for proc_create_net_single since net is stored directly
 * in private not as a struct i.e. seq_file_net can't be used.
 */
#[inline]
pub unsafe fn seq_file_single_net(seq: *mut seq_file) -> *mut net {
    #[cfg(CONFIG_NET_NS)]
    {
        return (*(seq as *mut seq_file)).private as *mut net;
    }
    #[cfg(not(CONFIG_NET_NS))]
    {
        return &raw mut init_net;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
