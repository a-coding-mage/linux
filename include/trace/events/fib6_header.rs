/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/in6.h, net/flow.h, net/ip6_fib.h, and linux/tracepoint.h.
// The C TRACE_EVENT declaration is represented here by its trace-entry
// layout and the equivalent assignment routine.

#[repr(C)]
pub struct Fib6TableLookupEntry {
    pub tb_id: u32,
    pub err: i32,
    pub oif: i32,
    pub iif: i32,
    pub flowlabel: u32,
    pub tos: u8,
    pub scope: u8,
    pub flags: u8,
    pub src: [u8; 16],
    pub dst: [u8; 16],
    pub sport: u16,
    pub dport: u16,
    pub proto: u8,
    pub rt_type: u8,
    pub name: [core::ffi::c_char; IFNAMSIZ],
    pub gw: [u8; 16],
}

// Equivalent to TP_fast_assign for fib6_table_lookup.
pub unsafe fn fib6_table_lookup(
    entry: *mut Fib6TableLookupEntry,
    net: *const Net,
    res: *const Fib6Result,
    table: *mut Fib6Table,
    flp: *const Flowi6,
) {
    let mut in6: *mut In6Addr;

    (*entry).tb_id = (*table).tb6_id;
    (*entry).err = ip6_rt_type_to_error((*res).fib6_type);
    (*entry).oif = (*flp).flowi6_oif;
    (*entry).iif = (*flp).flowi6_iif;
    (*entry).flowlabel = ntohl(flowi6_get_flowlabel(flp));
    (*entry).tos = ip6_tclass((*flp).flowlabel);
    (*entry).scope = (*flp).flowi6_scope;
    (*entry).flags = (*flp).flowi6_flags;

    in6 = (*entry).src.as_mut_ptr().cast::<In6Addr>();
    *in6 = (*flp).saddr;

    in6 = (*entry).dst.as_mut_ptr().cast::<In6Addr>();
    *in6 = (*flp).daddr;

    (*entry).proto = (*flp).flowi6_proto;
    if (*entry).proto == IPPROTO_TCP || (*entry).proto == IPPROTO_UDP {
        (*entry).sport = ntohs((*flp).fl6_sport);
        (*entry).dport = ntohs((*flp).fl6_dport);
    } else {
        (*entry).sport = 0;
        (*entry).dport = 0;
    }

    if !(*res).nh.is_null() && !(*(*res).nh).fib_nh_dev.is_null() {
        strscpy(
            (*entry).name.as_mut_ptr(),
            (*(*(*res).nh).fib_nh_dev).name.as_ptr(),
            IFNAMSIZ,
        );
    } else {
        strcpy((*entry).name.as_mut_ptr(), b"-\0".as_ptr().cast());
    }
    if (*res).f6i == (*net).ipv6.fib6_null_entry {
        in6 = (*entry).gw.as_mut_ptr().cast::<In6Addr>();
        *in6 = in6addr_any;
    } else if !(*res).nh.is_null() {
        in6 = (*entry).gw.as_mut_ptr().cast::<In6Addr>();
        *in6 = (*(*res).nh).fib_nh_gw6;
    }
}

// TP_printk format:
// "table %3u oif %d iif %d proto %u %pI6c/%u -> %pI6c/%u flowlabel %#x
//  tos %d scope %d flags %x ==> dev %s gw %pI6c err %d"

// External types, constants, globals, and functions are provided by the
// corresponding translated kernel headers.
extern "C" {
    pub type Net;
    pub type Fib6Result;
    pub type Fib6Table;
    pub type Flowi6;
    pub type In6Addr;

    pub static in6addr_any: In6Addr;
    pub static IFNAMSIZ: usize;
    pub static IPPROTO_TCP: u8;
    pub static IPPROTO_UDP: u8;

    pub fn ip6_rt_type_to_error(ty: u8) -> i32;
    pub fn flowi6_get_flowlabel(flp: *const Flowi6) -> u32;
    pub fn ip6_tclass(flowlabel: u32) -> u8;
    pub fn ntohl(value: u32) -> u32;
    pub fn ntohs(value: u16) -> u16;
    pub fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: usize);
    pub fn strcpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
