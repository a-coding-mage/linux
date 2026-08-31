/* SPDX-License-Identifier: GPL-2.0 */

/* Rust translation of include/linux/btf_ids.h. */

/* From <linux/types.h>. */
pub type u32 = ::core::ffi::c_uint;

#[repr(C)]
pub struct btf_id_set {
    pub cnt: u32,
    pub ids: [u32; 0],
}

/* This flag implies BTF_SET8 holds kfunc(s) */
pub const BTF_SET8_KFUNCS: u32 = 1 << 0;

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

/*
 * CONFIG_DEBUG_INFO_BTF:
 *
 * The C header defines a family of preprocessor macros which emit assembly into
 * the .BTF_ids section. Those macros are intentionally preserved here as Rust
 * macro interfaces. They require external kernel build support such as
 * core::arch::global_asm availability and the resolve_btfids linking phase.
 *
 * Following macros help to define lists of BTF IDs placed
 * in .BTF_ids section. They are initially filled with zeros
 * (during compilation) and resolved later during the
 * linking phase by resolve_btfids tool.
 *
 * Any change in list layout must be reflected in resolve_btfids
 * tool logic.
 */

pub const BTF_IDS_SECTION: &str = ".BTF_ids";

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! ____BTF_ID {
    ($symbol:ident, $word:expr) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".local ",
                stringify!($symbol),
                " ;                          \n",
                ".type  ",
                stringify!($symbol),
                ", STT_OBJECT;               \n",
                ".size  ",
                stringify!($symbol),
                ", 4;                        \n",
                stringify!($symbol),
                ":                                     \n",
                ".zero 4                                       \n",
                $word,
                ".popsection;                                  \n",
            )
        );
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __BTF_ID {
    ($symbol:ident, $word:expr) => {
        $crate::____BTF_ID!($symbol, $word);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __ID {
    ($prefix:ident) => {
        $prefix
    };
}

/*
 * The BTF_ID defines unique symbol for each ID pointing
 * to 4 zero bytes.
 *
 * Rust macro_rules cannot paste identifiers with __COUNTER__ and __LINE__ in
 * the same way as the C preprocessor. Callers needing the exact symbol spelling
 * must use __BTF_ID!/____BTF_ID! with a concrete symbol identifier.
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID {
    ($prefix:ident, $name:ident) => {
        compile_error!("BTF_ID requires C preprocessor identifier pasting; use __BTF_ID! with a concrete symbol");
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! ____BTF_ID_FLAGS {
    ($prefix:ident, $name:ident, $flags:expr) => {
        compile_error!("BTF_ID_FLAGS requires C preprocessor identifier pasting; use __BTF_ID! with a concrete symbol");
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __BTF_ID_FLAGS {
    ($prefix:ident, $name:ident, $flags:expr $(, $rest:tt)*) => {
        $crate::____BTF_ID_FLAGS!($prefix, $name, $flags);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_FLAGS {
    ($prefix:ident, $name:ident $(, $flags:expr)?) => {
        $crate::__BTF_ID_FLAGS!($prefix, $name, $($flags)? 0);
    };
}

/*
 * The BTF_ID_LIST macro defines pure (unsorted) list
 * of BTF IDs, with following layout:
 *
 * BTF_ID_LIST(list1)
 * BTF_ID(type1, name1)
 * BTF_ID(type2, name2)
 *
 * list1:
 * __BTF_ID__type1__name1__1:
 * .zero 4
 * __BTF_ID__type2__name2__2:
 * .zero 4
 *
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __BTF_ID_LIST {
    ($name:ident, local) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".local ",
                stringify!($name),
                ";                        \n",
                stringify!($name),
                ":;                                      \n",
                ".popsection;                                  \n",
            )
        );
    };
    ($name:ident, globl) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".globl ",
                stringify!($name),
                ";                        \n",
                stringify!($name),
                ":;                                      \n",
                ".popsection;                                  \n",
            )
        );
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_LIST {
    ($name:ident) => {
        $crate::__BTF_ID_LIST!($name, local);
        extern "C" {
            pub static mut $name: [u32; 0];
        }
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_LIST_GLOBAL {
    ($name:ident, $n:expr) => {
        $crate::__BTF_ID_LIST!($name, globl);
    };
}

/* The BTF_ID_LIST_SINGLE macro defines a BTF_ID_LIST with
 * a single entry.
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_LIST_SINGLE {
    ($name:ident, $prefix:ident, $typename:ident) => {
        $crate::BTF_ID_LIST!($name);
        $crate::BTF_ID!($prefix, $typename);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_LIST_GLOBAL_SINGLE {
    ($name:ident, $prefix:ident, $typename:ident) => {
        $crate::BTF_ID_LIST_GLOBAL!($name, 1);
        $crate::BTF_ID!($prefix, $typename);
    };
}

/*
 * The BTF_ID_UNUSED macro defines 4 zero bytes.
 * It's used when we want to define 'unused' entry
 * in BTF_ID_LIST, like:
 *
 *   BTF_ID_LIST(bpf_skb_output_btf_ids)
 *   BTF_ID(struct, sk_buff)
 *   BTF_ID_UNUSED
 *   BTF_ID(struct, task_struct)
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_ID_UNUSED {
    () => {
        ::core::arch::global_asm!(
            ".pushsection .BTF_ids,\"a\";       \n\
             .zero 4                                       \n\
             .popsection;                                  \n"
        );
    };
}

/*
 * The BTF_SET_START/END macros pair defines sorted list of
 * BTF IDs plus its members count, with following layout:
 *
 * BTF_SET_START(list)
 * BTF_ID(type1, name1)
 * BTF_ID(type2, name2)
 * BTF_SET_END(list)
 *
 * __BTF_ID__set__list:
 * .zero 4
 * list:
 * __BTF_ID__type1__name1__3:
 * .zero 4
 * __BTF_ID__type2__name2__4:
 * .zero 4
 *
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __BTF_SET_START {
    ($name:ident, local) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".local __BTF_ID__set__",
                stringify!($name),
                ";         \n",
                "__BTF_ID__set__",
                stringify!($name),
                ":;                    \n",
                ".zero 4                                       \n",
                ".popsection;                                  \n",
            )
        );
    };
    ($name:ident, globl) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".globl __BTF_ID__set__",
                stringify!($name),
                ";         \n",
                "__BTF_ID__set__",
                stringify!($name),
                ":;                    \n",
                ".zero 4                                       \n",
                ".popsection;                                  \n",
            )
        );
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_SET_START {
    ($name:ident) => {
        $crate::__BTF_ID_LIST!($name, local);
        $crate::__BTF_SET_START!($name, local);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_SET_START_GLOBAL {
    ($name:ident) => {
        $crate::__BTF_ID_LIST!($name, globl);
        $crate::__BTF_SET_START!($name, globl);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_SET_END {
    ($name:ident) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";      \n",
                ".size __BTF_ID__set__",
                stringify!($name),
                ", .-",
                stringify!($name),
                "  \n",
                ".popsection;                                 \n",
            )
        );
        extern "C" {
            pub static mut $name: $crate::btf_id_set;
        }
    };
}

/*
 * The BTF_SET8_START/END macros pair defines sorted list of
 * BTF IDs and their flags plus its members count, with the
 * following layout:
 *
 * BTF_SET8_START(list)
 * BTF_ID_FLAGS(type1, name1, flags)
 * BTF_ID_FLAGS(type2, name2, flags)
 * BTF_SET8_END(list)
 *
 * __BTF_ID__set8__list:
 * .zero 8
 * list:
 * __BTF_ID__type1__name1__3:
 * .zero 4
 * .word (1 << 0) | (1 << 2)
 * __BTF_ID__type2__name2__5:
 * .zero 4
 * .word (1 << 3) | (1 << 1) | (1 << 2)
 *
 */
#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! __BTF_SET8_START {
    ($name:ident, local, $flags:expr) => {
        $crate::__BTF_ID_LIST!($name, local);
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".local __BTF_ID__set8__",
                stringify!($name),
                ";        \n",
                "__BTF_ID__set8__",
                stringify!($name),
                ":;                   \n",
                ".zero 4                                       \n",
                ".long ",
                stringify!($flags),
                "\n",
                ".popsection;                                  \n",
            )
        );
    };
    ($name:ident, globl, $flags:expr) => {
        $crate::__BTF_ID_LIST!($name, local);
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";       \n",
                ".globl __BTF_ID__set8__",
                stringify!($name),
                ";        \n",
                "__BTF_ID__set8__",
                stringify!($name),
                ":;                   \n",
                ".zero 4                                       \n",
                ".long ",
                stringify!($flags),
                "\n",
                ".popsection;                                  \n",
            )
        );
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_SET8_START {
    ($name:ident) => {
        $crate::__BTF_SET8_START!($name, local, 0);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_SET8_END {
    ($name:ident) => {
        ::core::arch::global_asm!(
            concat!(
                ".pushsection .BTF_ids,\"a\";      \n",
                ".size __BTF_ID__set8__",
                stringify!($name),
                ", .-",
                stringify!($name),
                "  \n",
                ".popsection;                                 \n",
            )
        );
        extern "C" {
            pub static mut $name: $crate::btf_id_set8;
        }
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_KFUNCS_START {
    ($name:ident) => {
        $crate::__BTF_SET8_START!($name, local, $crate::BTF_SET8_KFUNCS);
    };
}

#[cfg(CONFIG_DEBUG_INFO_BTF)]
#[macro_export]
macro_rules! BTF_KFUNCS_END {
    ($name:ident) => {
        $crate::BTF_SET8_END!($name);
    };
}

/* !CONFIG_DEBUG_INFO_BTF fallback macro definitions from the C header. */
#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_LIST {
    ($name:ident) => {
        static mut $name: [u32; 128] = [0; 128];
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID {
    ($prefix:ident, $name:ident) => {};
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_FLAGS {
    ($prefix:ident, $name:ident $(, $rest:tt)*) => {};
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_UNUSED {
    () => {};
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_LIST_GLOBAL {
    ($name:ident, $n:expr) => {
        static mut $name: [u32; $n] = [0; $n];
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_LIST_SINGLE {
    ($name:ident, $prefix:ident, $typename:ident) => {
        static mut $name: [u32; 1] = [0; 1];
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_ID_LIST_GLOBAL_SINGLE {
    ($name:ident, $prefix:ident, $typename:ident) => {
        static mut $name: [u32; 1] = [0; 1];
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_SET_START {
    ($name:ident) => {
        static mut $name: $crate::btf_id_set = $crate::btf_id_set {
            cnt: 0,
            ids: [],
        };
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_SET_START_GLOBAL {
    ($name:ident) => {
        static mut $name: $crate::btf_id_set = $crate::btf_id_set {
            cnt: 0,
            ids: [],
        };
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_SET_END {
    ($name:ident) => {};
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_SET8_START {
    ($name:ident) => {
        static mut $name: $crate::btf_id_set8 = $crate::btf_id_set8 {
            cnt: 0,
            flags: 0,
            pairs: [],
        };
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_SET8_END {
    ($name:ident) => {};
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_KFUNCS_START {
    ($name:ident) => {
        static mut $name: $crate::btf_id_set8 = $crate::btf_id_set8 {
            cnt: 0,
            flags: $crate::BTF_SET8_KFUNCS,
            pairs: [],
        };
    };
}

#[cfg(not(CONFIG_DEBUG_INFO_BTF))]
#[macro_export]
macro_rules! BTF_KFUNCS_END {
    ($name:ident) => {};
}

/*
 * CONFIG_NET:
 * Define a list of socket types which can be the argument for
 * skc_to_*_sock() helpers. All these sockets should have
 * sock_common as the first argument in its memory layout.
 */
#[cfg(CONFIG_NET)]
#[macro_export]
macro_rules! BTF_SOCK_TYPE_xxx {
    ($BTF_SOCK_TYPE:ident) => {
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_INET, inet_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_INET_CONN, inet_connection_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_INET_REQ, inet_request_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_INET_TW, inet_timewait_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_REQ, request_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_SOCK, sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_SOCK_COMMON, sock_common);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_TCP, tcp_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_TCP_REQ, tcp_request_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_TCP_TW, tcp_timewait_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_TCP6, tcp6_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_UDP, udp_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_UDP6, udp6_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_UNIX, unix_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_MPTCP, mptcp_sock);
        $BTF_SOCK_TYPE!(BTF_SOCK_TYPE_SOCKET, socket);
    };
}

#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_INET: u32 = 0;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_INET_CONN: u32 = 1;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_INET_REQ: u32 = 2;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_INET_TW: u32 = 3;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_REQ: u32 = 4;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_SOCK: u32 = 5;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_SOCK_COMMON: u32 = 6;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_TCP: u32 = 7;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_TCP_REQ: u32 = 8;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_TCP_TW: u32 = 9;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_TCP6: u32 = 10;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_UDP: u32 = 11;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_UDP6: u32 = 12;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_UNIX: u32 = 13;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_MPTCP: u32 = 14;
#[cfg(CONFIG_NET)]
pub const BTF_SOCK_TYPE_SOCKET: u32 = 15;
#[cfg(CONFIG_NET)]
pub const MAX_BTF_SOCK_TYPE: u32 = 16;

#[cfg(CONFIG_NET)]
extern "C" {
    pub static mut btf_sock_ids: [u32; 0];
}

#[macro_export]
macro_rules! BTF_TRACING_TYPE_xxx {
    ($BTF_TRACING_TYPE:ident) => {
        $BTF_TRACING_TYPE!(BTF_TRACING_TYPE_TASK, task_struct);
        $BTF_TRACING_TYPE!(BTF_TRACING_TYPE_FILE, file);
        $BTF_TRACING_TYPE!(BTF_TRACING_TYPE_VMA, vm_area_struct);
    };
}

pub const BTF_TRACING_TYPE_TASK: u32 = 0;
pub const BTF_TRACING_TYPE_FILE: u32 = 1;
pub const BTF_TRACING_TYPE_VMA: u32 = 2;
pub const MAX_BTF_TRACING_TYPE: u32 = 3;

extern "C" {
    pub static mut btf_tracing_ids: [u32; 0];
    pub static mut bpf_cgroup_btf_id: [u32; 0];
    pub static mut bpf_local_storage_map_btf_id: [u32; 0];
    pub static mut btf_bpf_map_id: [u32; 0];
    pub static mut bpf_kmem_cache_btf_id: [u32; 0];
}
