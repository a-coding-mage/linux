/* SPDX-License-Identifier: GPL-2.0 */
// TRACE_SYSTEM fib
// The C tracepoint declarations depend on Linux kernel types and tracepoint
// infrastructure supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct FibTableLookupEntry {
    pub tb_id: u32,
    pub err: c_int,
    pub oif: c_int,
    pub iif: c_int,
    pub proto: u8,
    pub tos: u8,
    pub scope: u8,
    pub flags: u8,
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub gw4: [u8; 4],
    pub gw6: [u8; 16],
    pub sport: u16,
    pub dport: u16,
    pub name: [c_char; IFNAMSIZ],
}

// C build-time dependency: IFNAMSIZ is supplied by <linux/if.h>.
// The value is kept as a local translation constant for the entry layout.
pub const IFNAMSIZ: usize = 16;

#[repr(C)]
pub struct Flowi4 {
    pub flowi4_oif: c_int,
    pub flowi4_iif: c_int,
    pub flowi4_dscp: u8,
    pub flowi4_scope: u8,
    pub flowi4_flags: u8,
    pub flowi4_proto: u8,
    pub saddr: u32,
    pub daddr: u32,
    pub fl4_sport: u16,
    pub fl4_dport: u16,
}

#[repr(C)]
pub struct FibNhCommon {
    pub nhc_dev: *mut NetDevice,
    pub nhc_gw_family: c_int,
    pub nhc_gw: FibNhGateway,
}

#[repr(C)]
pub union FibNhGateway {
    pub ipv4: u32,
    pub ipv6: In6Addr,
}

#[repr(C)]
pub struct NetDevice {
    pub name: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct In6Addr {
    pub in6_u: [u8; 16],
}

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;

unsafe extern "C" {
    pub static in6addr_any: In6Addr;
    pub fn inet_dscp_to_dsfield(dscp: u8) -> u8;
    pub fn ntohs(value: u16) -> u16;
    pub fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

// Translation of fib_table_lookup's TP_fast_assign and TP_printk bodies.
// Tracepoint registration and the kernel-provided __entry object are supplied
// by the external tracepoint implementation.
pub unsafe fn fib_table_lookup_fast_assign(
    entry: *mut FibTableLookupEntry,
    tb_id: u32,
    flp: *const Flowi4,
    nhc: *const FibNhCommon,
    err: c_int,
) {
    (*entry).tb_id = tb_id;
    (*entry).err = err;
    (*entry).oif = (*flp).flowi4_oif;
    (*entry).iif = (*flp).flowi4_iif;
    (*entry).tos = inet_dscp_to_dsfield((*flp).flowi4_dscp);
    (*entry).scope = (*flp).flowi4_scope;
    (*entry).flags = (*flp).flowi4_flags;
    (*entry).src = (*flp).saddr.to_ne_bytes();
    (*entry).dst = (*flp).daddr.to_ne_bytes();
    (*entry).proto = (*flp).flowi4_proto;
    if (*entry).proto == IPPROTO_TCP || (*entry).proto == IPPROTO_UDP {
        (*entry).sport = ntohs((*flp).fl4_sport);
        (*entry).dport = ntohs((*flp).fl4_dport);
    } else {
        (*entry).sport = 0;
        (*entry).dport = 0;
    }

    let dev = if !nhc.is_null() { (*nhc).nhc_dev } else { core::ptr::null_mut() };
    let dash = b"-\0";
    if dev.is_null() {
        strscpy((*entry).name.as_mut_ptr(), dash.as_ptr().cast(), IFNAMSIZ);
    } else {
        strscpy((*entry).name.as_mut_ptr(), (*dev).name.as_ptr(), IFNAMSIZ);
    }

    if !nhc.is_null() {
        if (*nhc).nhc_gw_family == AF_INET {
            (*entry).gw4 = (*nhc).nhc_gw.ipv4.to_ne_bytes();
            (*entry).gw6 = in6addr_any.in6_u;
        } else if (*nhc).nhc_gw_family == AF_INET6 {
            (*entry).gw4 = [0; 4];
            (*entry).gw6 = (*nhc).nhc_gw.ipv6.in6_u;
        }
    } else {
        (*entry).gw4 = [0; 4];
        (*entry).gw6 = in6addr_any.in6_u;
    }
}

// TP_printk format:
// table %u oif %d iif %d proto %u %pI4/%u -> %pI4/%u tos %d scope %d
// flags %x ==> dev %s gw %pI4/%pI6c err %d

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
