/* SPDX-License-Identifier: MIT */
/* Shared producer-consumer ring macros, translated from ring.h. */

/* Dependencies supplied by the surrounding Xen bindings. */

pub type RING_IDX = ::core::ffi::c_uint;

#[inline]
pub const fn __rd2(x: u32) -> u32 { if x & 0x00000002 != 0 { 0x2 } else { x & 0x1 } }
#[inline]
pub const fn __rd4(x: u32) -> u32 { if x & 0x0000000c != 0 { __rd2(x >> 2) << 2 } else { __rd2(x) } }
#[inline]
pub const fn __rd8(x: u32) -> u32 { if x & 0x000000f0 != 0 { __rd4(x >> 4) << 4 } else { __rd4(x) } }
#[inline]
pub const fn __rd16(x: u32) -> u32 { if x & 0x0000ff00 != 0 { __rd8(x >> 8) << 8 } else { __rd8(x) } }
#[inline]
pub const fn __rd32(x: u32) -> u32 { if x & 0xffff0000 != 0 { __rd16(x >> 16) << 16 } else { __rd16(x) } }

/* __CONST_RING_SIZE and __RING_SIZE depend on the concrete shared-ring type. */

#[macro_export]
macro_rules! DEFINE_RING_TYPES {
    ($entry:ident, $sring:ident, $front:ident, $back:ident, $req:ty, $rsp:ty) => {
        #[repr(C)]
        pub union $entry { pub req: $req, pub rsp: $rsp }
        #[repr(C)]
        pub struct $sring {
            pub req_prod: $crate::RING_IDX, pub req_event: $crate::RING_IDX,
            pub rsp_prod: $crate::RING_IDX, pub rsp_event: $crate::RING_IDX,
            pub __pad: [u8; 48], pub ring: [ $entry; 0 ],
        }
        #[repr(C)]
        pub struct $front {
            pub req_prod_pvt: $crate::RING_IDX, pub rsp_cons: $crate::RING_IDX,
            pub nr_ents: ::core::ffi::c_uint, pub sring: *mut $sring,
        }
        #[repr(C)]
        pub struct $back {
            pub rsp_prod_pvt: $crate::RING_IDX, pub req_cons: $crate::RING_IDX,
            pub nr_ents: ::core::ffi::c_uint, pub sring: *mut $sring,
        }
    };
}

#[inline]
pub unsafe fn shared_ring_init<T>(s: *mut T) {
    /* Concrete ring fields are initialized by the corresponding generated type. */
    let _ = s;
}

#[macro_export]
macro_rules! RING_SIZE { ($r:expr) => { (*$r).nr_ents } }
#[macro_export]
macro_rules! RING_FREE_REQUESTS { ($r:expr) => { RING_SIZE!($r).wrapping_sub((*$r).req_prod_pvt.wrapping_sub((*$r).rsp_cons)) } }
#[macro_export]
macro_rules! RING_FULL { ($r:expr) => { RING_FREE_REQUESTS!($r) == 0 } }
#[macro_export]
macro_rules! XEN_RING_NR_UNCONSUMED_RESPONSES { ($r:expr) => { (*(*$r).sring).rsp_prod.wrapping_sub((*$r).rsp_cons) } }
#[macro_export]
macro_rules! XEN_RING_NR_UNCONSUMED_REQUESTS {
    ($r:expr) => {{ let req = (*(*$r).sring).req_prod.wrapping_sub((*$r).req_cons); let rsp = RING_SIZE!($r).wrapping_sub((*$r).req_cons.wrapping_sub((*$r).rsp_prod_pvt)); if req < rsp { req } else { rsp } }}
}
#[macro_export]
macro_rules! RING_HAS_UNCONSUMED_RESPONSES { ($r:expr) => { XEN_RING_NR_UNCONSUMED_RESPONSES!($r) != 0 } }
#[macro_export]
macro_rules! RING_HAS_UNCONSUMED_REQUESTS { ($r:expr) => { XEN_RING_NR_UNCONSUMED_REQUESTS!($r) != 0 } }

#[macro_export]
macro_rules! RING_REQUEST_CONS_OVERFLOW { ($r:expr, $c:expr) => { ($c).wrapping_sub((*$r).rsp_prod_pvt) >= RING_SIZE!($r) } }
#[macro_export]
macro_rules! RING_REQUEST_PROD_OVERFLOW { ($r:expr, $p:expr) => { ($p).wrapping_sub((*$r).rsp_prod_pvt) > RING_SIZE!($r) } }
#[macro_export]
macro_rules! RING_RESPONSE_PROD_OVERFLOW { ($r:expr, $p:expr) => { ($p).wrapping_sub((*$r).rsp_cons) > RING_SIZE!($r) } }

/* Memory barriers are supplied by the target Xen environment. */
extern "C" { pub fn virt_wmb(); pub fn virt_mb(); }

#[macro_export]
macro_rules! RING_PUSH_REQUESTS { ($r:expr) => {{ unsafe { $crate::virt_wmb(); (*(*$r).sring).req_prod = (*$r).req_prod_pvt; } }} }
#[macro_export]
macro_rules! RING_PUSH_RESPONSES { ($r:expr) => {{ unsafe { $crate::virt_wmb(); (*(*$r).sring).rsp_prod = (*$r).rsp_prod_pvt; } }} }

/* Flexible-ring helpers. XEN_PAGE_SHIFT is 12 unless supplied by the build. */
pub const XEN_PAGE_SHIFT: u32 = 12;
#[macro_export]
macro_rules! XEN_FLEX_RING_SIZE { ($order:expr) => { 1usize << (($order) + $crate::XEN_PAGE_SHIFT - 1) } }

#[macro_export]
macro_rules! DEFINE_XEN_FLEX_RING {
    ($name:ident, $data:ident) => {
        #[inline] pub fn $name##_mask(idx: $crate::RING_IDX, ring_size: $crate::RING_IDX) -> $crate::RING_IDX { idx & (ring_size - 1) }
        #[repr(C)] pub struct $data { pub r#in: *mut u8, pub out: *mut u8 }
    };
}

/* grant_ref_t is supplied by Xen grant_table bindings. */
#[macro_export]
macro_rules! DEFINE_XEN_FLEX_RING_AND_INTF {
    ($name:ident, $intf:ident, $data:ident, $grant_ref_t:ty) => {
        #[repr(C)] pub struct $intf { pub in_cons: $crate::RING_IDX, pub in_prod: $crate::RING_IDX, pub pad1: [u8;56], pub out_cons: $crate::RING_IDX, pub out_prod: $crate::RING_IDX, pub pad2: [u8;56], pub ring_order: $crate::RING_IDX, pub r#ref: [$grant_ref_t; 0] }
        DEFINE_XEN_FLEX_RING!($name, $data);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
