/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * This file has definitions for the hub and snac interfaces.
 *
 * Copyright (C) 1992 - 1997, 1999, 2000 Silcon Graphics, Inc.
 * Copyright (C) 1999, 2000 Ralf Baechle (ralf@gnu.org)
 */

// Dependencies supplied by the surrounding translation unit:
// asm/sn/addrs.h, asm/sn/arch.h, and the CONFIG_SGI_IP27/CONFIG_SGI_IP35
// hub definitions.

/*
 * NIC register macros
 *
 * HUB_NIC_ADDR is present only for CONFIG_SGI_IP27, as in the source header.
 */

#[cfg(CONFIG_SGI_IP27)]
macro_rules! HUB_NIC_ADDR {
    ($cpuid:expr) => {
        REMOTE_HUB_ADDR!(cpu_to_node($cpuid), MD_MLAN_CTL)
    };
}

macro_rules! SET_HUB_NIC {
    ($my_cpuid:expr, $val:expr) => {{
        HUB_S!(HUB_NIC_ADDR!($my_cpuid), $val)
    }};
}

macro_rules! SET_MY_HUB_NIC {
    ($v:expr) => {
        SET_HUB_NIC!(cpuid!(), $v)
    };
}

macro_rules! GET_HUB_NIC {
    ($my_cpuid:expr) => {
        HUB_L!(HUB_NIC_ADDR!($my_cpuid))
    };
}

macro_rules! GET_MY_HUB_NIC {
    () => {
        GET_HUB_NIC!(cpuid!())
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
