/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2003+ Evgeniy Polyakov <johnpol@2ka.mxt.ru>
 */

// Dependencies supplied by linux/types.h and linux/netfilter/nfnetlink_osf.h.

pub use NF_OSF_GENRE as XT_OSF_GENRE;
pub use NF_OSF_INVERT as XT_OSF_INVERT;

pub use NF_OSF_TTL as XT_OSF_TTL;
pub use NF_OSF_LOG as XT_OSF_LOG;

pub use NF_OSF_LOGLEVEL_ALL as XT_OSF_LOGLEVEL_ALL;
pub use NF_OSF_LOGLEVEL_FIRST as XT_OSF_LOGLEVEL_FIRST;
pub use NF_OSF_LOGLEVEL_ALL_KNOWN as XT_OSF_LOGLEVEL_ALL_KNOWN;

pub use NF_OSF_TTL_TRUE as XT_OSF_TTL_TRUE;
pub use NF_OSF_TTL_NOCHECK as XT_OSF_TTL_NOCHECK;
pub use NF_OSF_TTL_LESS as XT_OSF_TTL_LESS;

pub use nf_osf_wc as xt_osf_wc;
pub use nf_osf_opt as xt_osf_opt;
pub use nf_osf_info as xt_osf_info;
pub use nf_osf_user_finger as xt_osf_user_finger;
pub use nf_osf_finger as xt_osf_finger;
pub use nf_osf_nlmsg as xt_osf_nlmsg;

pub use nf_osf_window_size_options as xt_osf_window_size_options;
pub use nf_osf_attr_type as xt_osf_attr_type;
pub use nf_osf_msg_types as xt_osf_msg_types;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
