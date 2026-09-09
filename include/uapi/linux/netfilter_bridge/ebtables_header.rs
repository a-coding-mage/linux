/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translation of uapi/linux/netfilter_bridge/ebtables.h. */

// C dependencies: linux/types.h, linux/if.h, linux/netfilter_bridge.h

pub const EBT_TABLE_MAXNAMELEN: usize = 32;
pub const EBT_CHAIN_MAXNAMELEN: usize = EBT_TABLE_MAXNAMELEN;
pub const EBT_FUNCTION_MAXNAMELEN: usize = EBT_TABLE_MAXNAMELEN;
pub const EBT_EXTENSION_MAXNAMELEN: usize = 31;

pub const EBT_ACCEPT: i32 = -1;
pub const EBT_DROP: i32 = -2;
pub const EBT_CONTINUE: i32 = -3;
pub const EBT_RETURN: i32 = -4;
pub const NUM_STANDARD_TARGETS: i32 = 4;
pub const EBT_VERDICT_BITS: u32 = 0x0000000f;

#[repr(C)]
pub struct xt_match {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct xt_target {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ebt_counter {
    pub pcnt: u64,
    pub bcnt: u64,
}

#[repr(C)]
pub struct ebt_replace {
    pub name: [::core::ffi::c_char; EBT_TABLE_MAXNAMELEN],
    pub valid_hooks: ::core::ffi::c_uint,
    pub nentries: ::core::ffi::c_uint,
    pub entries_size: ::core::ffi::c_uint,
    pub hook_entry: [*mut ebt_entries; NF_BR_NUMHOOKS],
    pub num_counters: ::core::ffi::c_uint,
    pub counters: *mut ebt_counter,
    pub entries: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub struct ebt_replace_kernel {
    pub name: [::core::ffi::c_char; EBT_TABLE_MAXNAMELEN],
    pub valid_hooks: ::core::ffi::c_uint,
    pub nentries: ::core::ffi::c_uint,
    pub entries_size: ::core::ffi::c_uint,
    pub hook_entry: [*mut ebt_entries; NF_BR_NUMHOOKS],
    pub num_counters: ::core::ffi::c_uint,
    pub counters: *mut ebt_counter,
    pub entries: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub struct ebt_entries {
    pub distinguisher: ::core::ffi::c_uint,
    pub name: [::core::ffi::c_char; EBT_CHAIN_MAXNAMELEN],
    pub counter_offset: ::core::ffi::c_uint,
    pub policy: ::core::ffi::c_int,
    pub nentries: ::core::ffi::c_uint,
    pub data: [::core::ffi::c_char; 0],
}

pub const EBT_ENTRY_OR_ENTRIES: u32 = 0x01;
pub const EBT_NOPROTO: u32 = 0x02;
pub const EBT_802_3: u32 = 0x04;
pub const EBT_SOURCEMAC: u32 = 0x08;
pub const EBT_DESTMAC: u32 = 0x10;
pub const EBT_F_MASK: u32 = EBT_NOPROTO | EBT_802_3 | EBT_SOURCEMAC | EBT_DESTMAC | EBT_ENTRY_OR_ENTRIES;
pub const EBT_IPROTO: u32 = 0x01;
pub const EBT_IIN: u32 = 0x02;
pub const EBT_IOUT: u32 = 0x04;
pub const EBT_ISOURCE: u32 = 0x08;
pub const EBT_IDEST: u32 = 0x10;
pub const EBT_ILOGICALIN: u32 = 0x20;
pub const EBT_ILOGICALOUT: u32 = 0x40;
pub const EBT_INV_MASK: u32 = EBT_IPROTO | EBT_IIN | EBT_IOUT | EBT_ILOGICALIN | EBT_ILOGICALOUT | EBT_ISOURCE | EBT_IDEST;

#[repr(C)]
pub union ebt_entry_match_u {
    pub name_revision: ebt_name_revision,
    pub match_: *mut xt_match,
}
#[repr(C)]
pub union ebt_entry_watcher_u {
    pub name_revision: ebt_name_revision,
    pub watcher: *mut xt_target,
}
#[repr(C)]
pub union ebt_entry_target_u {
    pub name_revision: ebt_name_revision,
    pub target: *mut xt_target,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_name_revision {
    pub name: [::core::ffi::c_char; EBT_EXTENSION_MAXNAMELEN],
    pub revision: u8,
}

#[repr(C)]
pub struct ebt_entry_match {
    pub u: ebt_entry_match_u,
    pub match_size: ::core::ffi::c_uint,
    pub data: [u8; 0],
}
#[repr(C)]
pub struct ebt_entry_watcher {
    pub u: ebt_entry_watcher_u,
    pub watcher_size: ::core::ffi::c_uint,
    pub data: [u8; 0],
}
#[repr(C)]
pub struct ebt_entry_target {
    pub u: ebt_entry_target_u,
    pub target_size: ::core::ffi::c_uint,
    pub data: [u8; 0],
}

pub const EBT_STANDARD_TARGET: &[u8] = b"standard\0";
#[repr(C)]
pub struct ebt_standard_target {
    pub target: ebt_entry_target,
    pub verdict: ::core::ffi::c_int,
}

#[repr(C)]
pub struct ebt_entry {
    pub bitmask: ::core::ffi::c_uint,
    pub invflags: ::core::ffi::c_uint,
    pub ethproto: u16,
    pub in_: [::core::ffi::c_char; IFNAMSIZ],
    pub logical_in: [::core::ffi::c_char; IFNAMSIZ],
    pub out: [::core::ffi::c_char; IFNAMSIZ],
    pub logical_out: [::core::ffi::c_char; IFNAMSIZ],
    pub sourcemac: [u8; ETH_ALEN],
    pub sourcemsk: [u8; ETH_ALEN],
    pub destmac: [u8; ETH_ALEN],
    pub destmsk: [u8; ETH_ALEN],
    pub watchers_offset: ::core::ffi::c_uint,
    pub target_offset: ::core::ffi::c_uint,
    pub next_offset: ::core::ffi::c_uint,
    pub elems: [u8; 0],
}

#[inline]
pub unsafe fn ebt_get_target(e: *mut ebt_entry) -> *mut ebt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut ebt_entry_target
}

pub const EBT_BASE_CTL: i32 = 128;
pub const EBT_SO_SET_ENTRIES: i32 = EBT_BASE_CTL;
pub const EBT_SO_SET_COUNTERS: i32 = EBT_SO_SET_ENTRIES + 1;
pub const EBT_SO_SET_MAX: i32 = EBT_SO_SET_COUNTERS + 1;
pub const EBT_SO_GET_INFO: i32 = EBT_BASE_CTL;
pub const EBT_SO_GET_ENTRIES: i32 = EBT_SO_GET_INFO + 1;
pub const EBT_SO_GET_INIT_INFO: i32 = EBT_SO_GET_ENTRIES + 1;
pub const EBT_SO_GET_INIT_ENTRIES: i32 = EBT_SO_GET_INIT_INFO + 1;
pub const EBT_SO_GET_MAX: i32 = EBT_SO_GET_INIT_ENTRIES + 1;

// C variadic statement-expression macros translated as Rust macros.
#[macro_export]
macro_rules! EBT_MATCH_ITERATE {
    ($e:expr, $fn:expr $(, $args:expr)*) => {{
        let mut __i: usize = core::mem::size_of::<$crate::ebt_entry>();
        let mut __ret: i32 = 0;
        while __i < (*$e).watchers_offset as usize {
            let __match = ($e as *mut u8).add(__i) as *mut $crate::ebt_entry_match;
            __ret = $fn(__match $(, $args)*);
            if __ret != 0 { break; }
            __i += (*__match).match_size as usize + core::mem::size_of::<$crate::ebt_entry_match>();
        }
        if __ret == 0 && __i != (*$e).watchers_offset as usize { __ret = -22; }
        __ret
    }};
}

#[macro_export]
macro_rules! EBT_WATCHER_ITERATE {
    ($e:expr, $fn:expr $(, $args:expr)*) => {{
        let mut __i = (*$e).watchers_offset as usize;
        let mut __ret: i32 = 0;
        while __i < (*$e).target_offset as usize {
            let __watcher = ($e as *mut u8).add(__i) as *mut $crate::ebt_entry_watcher;
            __ret = $fn(__watcher $(, $args)*);
            if __ret != 0 { break; }
            __i += (*__watcher).watcher_size as usize + core::mem::size_of::<$crate::ebt_entry_watcher>();
        }
        if __ret == 0 && __i != (*$e).target_offset as usize { __ret = -22; }
        __ret
    }};
}

#[macro_export]
macro_rules! EBT_ENTRY_ITERATE {
    ($entries:expr, $size:expr, $fn:expr $(, $args:expr)*) => {{
        let mut __i: usize = 0;
        let mut __ret: i32 = 0;
        while __i < $size as usize {
            let __entry = ($entries as *mut u8).add(__i) as *mut $crate::ebt_entry;
            __ret = $fn(__entry $(, $args)*);
            if __ret != 0 { break; }
            __i += if (*__entry).bitmask != 0 { (*__entry).next_offset as usize } else { core::mem::size_of::<$crate::ebt_entries>() };
        }
        if __ret == 0 && __i != $size as usize { __ret = -22; }
        __ret
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
