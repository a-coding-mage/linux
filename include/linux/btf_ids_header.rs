/* SPDX-License-Identifier: GPL-2.0 */

#![allow(non_camel_case_types, non_upper_case_globals, unused_macros)]

#[repr(C)]
pub struct btf_id_set {
    pub cnt: u32,
    pub ids: [u32; 0],
}

/* This flag implies BTF_SET8 holds kfunc(s). */
pub const BTF_SET8_KFUNCS: u32 = 1u32 << 0;

#[repr(C)]
pub struct btf_id_set8_pair {
    pub id: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct btf_id_set8 {
    pub cnt: u32,
    pub flags: u32,
    pub pairs: [btf_id_set8_pair; 0],
}

/* The following macros describe symbols and layouts emitted in .BTF_ids by
 * the C build and resolved later by resolve_btfids. */
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
pub const BTF_IDS_SECTION: &str = ".BTF_ids";

#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! ____BTF_ID {
    ($symbol:ident, $word:expr) => {
        core::arch::global_asm!(concat!(
            ".pushsection .BTF_ids,\"a\";\n.local ", stringify!($symbol),
            ";\n.type ", stringify!($symbol), ", STT_OBJECT;\n.size ",
            stringify!($symbol), ", 4;\n", stringify!($symbol),
            ":\n.zero 4\n", $word, ".popsection;\n"
        ));
    };
}
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! __BTF_ID { ($symbol:ident, $word:expr) => { ____BTF_ID!($symbol, $word); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! __ID { ($prefix:ident) => { concat_idents::concat_idents!($prefix, "__COUNTER__", "__LINE__") }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID { ($prefix:ident, $name:ident) => {}; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_FLAGS { ($prefix:ident, $name:ident $(, $flags:expr)* $(,)?) => {}; }

#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_LIST { ($name:ident) => {
    extern "C" { pub static mut $name: [u32; 0]; }
}; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_LIST_GLOBAL { ($name:ident, $n:expr) => {}; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_LIST_SINGLE { ($name:ident, $prefix:ident, $typename:ident) => { BTF_ID_LIST!($name); BTF_ID!($prefix, $typename); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_LIST_GLOBAL_SINGLE { ($name:ident, $prefix:ident, $typename:ident) => { BTF_ID_LIST_GLOBAL!($name, 1); BTF_ID!($prefix, $typename); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_ID_UNUSED { () => {}; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_SET_START { ($name:ident) => { BTF_ID_LIST!($name); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_SET_START_GLOBAL { ($name:ident) => { BTF_ID_LIST_GLOBAL!($name, 0); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_SET_END { ($name:ident) => { extern "C" { pub static mut $name: btf_id_set; } }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_SET8_START { ($name:ident) => { BTF_ID_LIST!($name); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_SET8_END { ($name:ident) => { extern "C" { pub static mut $name: btf_id_set8; } }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_KFUNCS_START { ($name:ident) => { BTF_SET8_START!($name); }; }
#[cfg(feature = "CONFIG_DEBUG_INFO_BTF")]
macro_rules! BTF_KFUNCS_END { ($name:ident) => { BTF_SET8_END!($name); }; }

#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_LIST { ($name:ident) => { static mut $name: [u32; 128] = [0; 128]; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID { ($prefix:ident, $name:ident) => {}; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_FLAGS { ($prefix:ident, $name:ident $(, $flags:expr)* $(,)?) => {}; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_UNUSED { () => {}; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_LIST_GLOBAL { ($name:ident, $n:expr) => { static mut $name: [u32; $n] = [0; $n]; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_LIST_SINGLE { ($name:ident, $prefix:ident, $typename:ident) => { static mut $name: [u32; 1] = [0; 1]; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_ID_LIST_GLOBAL_SINGLE { ($name:ident, $prefix:ident, $typename:ident) => { static mut $name: [u32; 1] = [0; 1]; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_SET_START { ($name:ident) => { static mut $name: btf_id_set = btf_id_set { cnt: 0, ids: [] }; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_SET_START_GLOBAL { ($name:ident) => { BTF_SET_START!($name); }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_SET_END { ($name:ident) => {}; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_SET8_START { ($name:ident) => { static mut $name: btf_id_set8 = btf_id_set8 { cnt: 0, flags: 0, pairs: [] }; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_SET8_END { ($name:ident) => {}; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_KFUNCS_START { ($name:ident) => { static mut $name: btf_id_set8 = btf_id_set8 { cnt: 0, flags: BTF_SET8_KFUNCS, pairs: [] }; }; }
#[cfg(not(feature = "CONFIG_DEBUG_INFO_BTF"))]
macro_rules! BTF_KFUNCS_END { ($name:ident) => {}; }

#[cfg(feature = "CONFIG_NET")]
#[repr(u32)]
pub enum btf_sock_type {
    BTF_SOCK_TYPE_INET,
    BTF_SOCK_TYPE_INET_CONN,
    BTF_SOCK_TYPE_INET_REQ,
    BTF_SOCK_TYPE_INET_TW,
    BTF_SOCK_TYPE_REQ,
    BTF_SOCK_TYPE_SOCK,
    BTF_SOCK_TYPE_SOCK_COMMON,
    BTF_SOCK_TYPE_TCP,
    BTF_SOCK_TYPE_TCP_REQ,
    BTF_SOCK_TYPE_TCP_TW,
    BTF_SOCK_TYPE_TCP6,
    BTF_SOCK_TYPE_UDP,
    BTF_SOCK_TYPE_UDP6,
    BTF_SOCK_TYPE_UNIX,
    BTF_SOCK_TYPE_MPTCP,
    BTF_SOCK_TYPE_SOCKET,
    MAX_BTF_SOCK_TYPE,
}
#[cfg(feature = "CONFIG_NET")]
extern "C" { pub static mut btf_sock_ids: [u32; 0]; }

#[repr(u32)]
pub enum btf_tracing_type {
    BTF_TRACING_TYPE_TASK,
    BTF_TRACING_TYPE_FILE,
    BTF_TRACING_TYPE_VMA,
    MAX_BTF_TRACING_TYPE,
}
extern "C" {
    pub static mut btf_tracing_ids: [u32; 0];
    pub static mut bpf_cgroup_btf_id: [u32; 0];
    pub static mut bpf_local_storage_map_btf_id: [u32; 0];
    pub static mut btf_bpf_map_id: [u32; 0];
    pub static mut bpf_kmem_cache_btf_id: [u32; 0];
    pub static mut bpf_multi_func_btf_id: [u32; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
