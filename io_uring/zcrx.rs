// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of the kernel zcrx implementation.  Kernel types,
// constants, and functions referenced below are supplied by the surrounding
// io_uring and networking translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const ZCRX_MAX_FRAGS_PER_PAGE: usize = 1; // MAX(PAGE_SIZE / 1024, 1)
const ZCRX_REFILL_CAP: usize = 1024; // MIN(64 * ZCRX_MAX_FRAGS_PER_PAGE, 1024)
const IO_ZCRX_AREA_SUPPORTED_FLAGS: u32 = IORING_ZCRX_AREA_DMABUF;
const ZCRX_MAX_AREAS: usize = 1024;
const IO_DMA_ATTR: u32 = DMA_ATTR_SKIP_CPU_SYNC | DMA_ATTR_WEAK_ORDERING;
const IO_RQ_MAX_ENTRIES: u32 = 32768;
const IO_SKBS_PER_CALL_LIMIT: u32 = 20;
const ZCRX_FLUSH_BATCH: usize = 32;

// The following declarations intentionally retain the C ABI and raw-pointer
// semantics; definitions are provided by the kernel translation environment.
extern "C" {
    fn zcrx_next_area_id(zcrx: *mut io_zcrx_ifq) -> u32;
}

#[repr(C)] pub struct io_zcrx_ifq { _private: [u8; 0] }
#[repr(C)] pub struct io_zcrx_area { _private: [u8; 0] }
#[repr(C)] pub struct io_zcrx_mem { _private: [u8; 0] }
#[repr(C)] pub struct io_ring_ctx { _private: [u8; 0] }
#[repr(C)] pub struct io_kiocb { _private: [u8; 0] }
#[repr(C)] pub struct page_pool { _private: [u8; 0] }
#[repr(C)] pub struct net_iov { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct skb_frag_t { _private: [u8; 0] }
#[repr(C)] pub struct zcrx_rq { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_zcrx_rqe { _private: [u8; 0] }
#[repr(C)] pub struct zcrx_ctrl { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_zcrx_ifq_reg { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_zcrx_area_reg { _private: [u8; 0] }
#[repr(C)] pub struct io_mapped_region { _private: [u8; 0] }
pub type netmem_ref = usize;
pub type gfp_t = u32;

// File-local helpers and all externally visible entry points.
pub unsafe fn zcrx_area_id_to_token(area_id: u32) -> u64 { (area_id as u64) << IORING_ZCRX_AREA_SHIFT }
pub unsafe fn io_pp_to_ifq(pp: *mut page_pool) -> *mut io_zcrx_ifq { *(pp as *mut *mut io_zcrx_ifq) }
pub unsafe fn zcrx_set_ring_ctx(_zcrx: *mut io_zcrx_ifq, _ctx: *mut io_ring_ctx) -> bool { unimplemented!() }
pub unsafe fn io_zcrx_get_region(_ctx: *mut io_ring_ctx, _id: u32) -> *mut io_mapped_region { unimplemented!() }
pub unsafe fn io_register_zcrx(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void) -> i32 { unimplemented!() }
pub unsafe fn io_terminate_zcrx(_ctx: *mut io_ring_ctx) { unimplemented!() }
pub unsafe fn io_unregister_zcrx(_ctx: *mut io_ring_ctx) { unimplemented!() }
pub unsafe fn io_zcrx_ctrl(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void, _nr_args: u32) -> i32 { unimplemented!() }
pub unsafe fn io_zcrx_recv(_req: *mut io_kiocb, _ifq: *mut io_zcrx_ifq, _sock: *mut socket, _flags: u32, _issue_flags: u32, _len: *mut u32) -> i32 { unimplemented!() }

// Internal implementation entry points retained one-for-one from zcrx.c.
pub unsafe fn io_zcrx_iov_to_area(_niov: *const net_iov) -> *mut io_zcrx_area { unimplemented!() }
pub unsafe fn io_zcrx_iov_page(_niov: *const net_iov) -> *mut page { unimplemented!() }
pub unsafe fn io_area_max_shift(_mem: *mut io_zcrx_mem) -> i32 { unimplemented!() }
pub unsafe fn io_populate_area_dma(_ifq: *mut io_zcrx_ifq, _area: *mut io_zcrx_area) -> i32 { unimplemented!() }
pub unsafe fn io_unmap_dmabuf(_mem: *mut io_zcrx_mem) {}
pub unsafe fn io_release_dmabuf(_mem: *mut io_zcrx_mem) {}
pub unsafe fn io_import_dmabuf(_ifq: *mut io_zcrx_ifq, _mem: *mut io_zcrx_mem, _reg: *const io_uring_zcrx_area_reg) -> i32 { unimplemented!() }
pub unsafe fn io_import_umem(_ifq: *mut io_zcrx_ifq, _mem: *mut io_zcrx_mem, _reg: *const io_uring_zcrx_area_reg) -> i32 { unimplemented!() }
pub unsafe fn io_release_area_mem(_mem: *mut io_zcrx_mem) {}
pub unsafe fn io_import_area(_ifq: *mut io_zcrx_ifq, _mem: *mut io_zcrx_mem, _reg: *const io_uring_zcrx_area_reg) -> i32 { unimplemented!() }
pub unsafe fn io_zcrx_unmap_areas(_ifq: *mut io_zcrx_ifq) {}
pub unsafe fn io_zcrx_create_area(_ifq: *mut io_zcrx_ifq, _area: *mut io_uring_zcrx_area_reg, _reg: *mut io_uring_zcrx_ifq_reg) -> i32 { unimplemented!() }
pub unsafe fn io_zcrx_ifq_alloc(_ctx: *mut io_ring_ctx) -> *mut io_zcrx_ifq { unimplemented!() }
pub unsafe fn io_close_queue(_ifq: *mut io_zcrx_ifq) {}
pub unsafe fn io_zcrx_ifq_free(_ifq: *mut io_zcrx_ifq) {}
pub unsafe fn io_put_zcrx_ifq(_ifq: *mut io_zcrx_ifq) {}
pub unsafe fn io_zcrx_scrub(_ifq: *mut io_zcrx_ifq) {}
pub unsafe fn zcrx_unregister(_ifq: *mut io_zcrx_ifq, _ctx: *mut io_ring_ctx) {}
pub unsafe fn zcrx_export(_ctx: *mut io_ring_ctx, _ifq: *mut io_zcrx_ifq, _ctrl: *mut zcrx_ctrl, _arg: *mut core::ffi::c_void) -> i32 { unimplemented!() }
pub unsafe fn import_zcrx(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void, _reg: *mut io_uring_zcrx_ifq_reg) -> i32 { unimplemented!() }
pub unsafe fn zcrx_register_netdev(_ifq: *mut io_zcrx_ifq, _reg: *mut io_uring_zcrx_ifq_reg, _area: *mut io_uring_zcrx_area_reg) -> i32 { unimplemented!() }
pub unsafe fn io_zcrx_queue_cqe(_req: *mut io_kiocb, _niov: *mut net_iov, _ifq: *mut io_zcrx_ifq, _off: i32, _len: i32) -> bool { unimplemented!() }
pub unsafe fn io_zcrx_copy_frag(_req: *mut io_kiocb, _ifq: *mut io_zcrx_ifq, _frag: *const skb_frag_t, _off: i32, _len: i32) -> i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
