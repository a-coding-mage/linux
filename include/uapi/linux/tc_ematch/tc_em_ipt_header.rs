/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Header guard: __LINUX_TC_EM_IPT_H

// C dependencies:
// #include <linux/types.h>
// #include <linux/pkt_cls.h>

#[repr(i32)]
pub enum TcEmIpt {
    TCA_EM_IPT_UNSPEC = 0,
    TCA_EM_IPT_HOOK,
    TCA_EM_IPT_MATCH_NAME,
    TCA_EM_IPT_MATCH_REVISION,
    TCA_EM_IPT_NFPROTO,
    TCA_EM_IPT_MATCH_DATA,
    __TCA_EM_IPT_MAX,
}

pub const TCA_EM_IPT_MAX: i32 = (__TCA_EM_IPT_MAX as i32) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
