/* SPDX-License-Identifier: GPL-2.0 */

/* Network memory. Rust translation of netmem.h. */

/* External kernel types and helpers are supplied by the surrounding kernel
 * translation. */

#[repr(C)]
pub struct netmem_desc {
    pub _flags: ::core::ffi::c_ulong,
    pub pp_magic: ::core::ffi::c_ulong,
    pub pp: *mut page_pool,
    pub _pp_mapping_pad: ::core::ffi::c_ulong,
    pub dma_addr: ::core::ffi::c_ulong,
    pub pp_ref_count: atomic_long_t,
}

/* These offsets mirror struct page fields: flags/_flags, pp_magic/pp,
 * _pp_mapping_pad, dma_addr, and pp_ref_count. */

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_iov_type {
    NET_IOV_DMABUF,
    NET_IOV_IOURING,
}

#[repr(C)]
pub struct net_iov {
    pub desc: netmem_desc,
    pub type_: net_iov_type,
    pub owner: *mut net_iov_area,
}

#[repr(C)]
pub struct net_iov_area {
    /* Array of net_iovs for this area. */
    pub niovs: *mut net_iov,
    pub num_niovs: usize,
    /* Offset into the dma-buf where this chunk starts. */
    pub base_virtual: ::core::ffi::c_ulong,
}

pub const NET_IOV: ::core::ffi::c_ulong = 0x01;

pub type netmem_ref = ::core::ffi::c_ulong;

extern "C" {
    pub static page_pool_mem_providers: static_key_false;

    pub fn WARN_ON_ONCE(condition: bool) -> bool;
    pub fn DEBUG_NET_WARN_ON_ONCE(condition: bool) -> bool;
    pub fn virt_to_page(addr: *const ::core::ffi::c_void) -> *mut page;
    pub fn page_ref_count(page: *mut page) -> ::core::ffi::c_int;
    pub fn page_to_pfn(page: *mut page) -> ::core::ffi::c_ulong;
    pub fn page_pool_page_is_pp(page: *const page) -> bool;
    pub fn page_to_nid(page: *mut page) -> ::core::ffi::c_int;
    pub fn compound_head(page: *mut page) -> *mut page;
    pub fn page_address(page: *mut page) -> *mut ::core::ffi::c_void;
    pub fn page_is_pfmemalloc(page: *mut page) -> bool;
    pub fn get_page(page: *mut page);
    pub fn put_page(page: *mut page);
    pub fn dma_unmap_page_attrs(dev: *mut device, addr: dma_addr_t, size: usize,
                                dir: dma_data_direction, attrs: ::core::ffi::c_ulong);
    pub fn dma_unmap_addr_set(ptr: *mut ::core::ffi::c_void,
                              name: ::core::ffi::c_ulong, value: dma_addr_t);
}

pub unsafe fn net_iov_owner(niov: *const net_iov) -> *mut net_iov_area {
    (*niov).owner
}

pub unsafe fn net_iov_idx(niov: *const net_iov) -> usize {
    niov.offset_from(net_iov_owner(niov).as_ref().unwrap().niovs)
        as usize
}

pub unsafe fn net_iov_init(niov: *mut net_iov, owner: *mut net_iov_area,
                           type_: net_iov_type) {
    (*niov).owner = owner;
    (*niov).type_ = type_;
}

pub unsafe fn netmem_is_net_iov(netmem: netmem_ref) -> bool {
    netmem & NET_IOV != 0
}

pub unsafe fn __netmem_to_page(netmem: netmem_ref) -> *mut page {
    netmem as *mut page
}

pub unsafe fn netmem_to_page(netmem: netmem_ref) -> *mut page {
    if WARN_ON_ONCE(netmem_is_net_iov(netmem)) { return core::ptr::null_mut(); }
    __netmem_to_page(netmem)
}

pub unsafe fn netmem_to_net_iov(netmem: netmem_ref) -> *mut net_iov {
    if netmem_is_net_iov(netmem) { return (netmem & !NET_IOV) as *mut net_iov; }
    DEBUG_NET_WARN_ON_ONCE(true);
    core::ptr::null_mut()
}

pub unsafe fn net_iov_to_netmem(niov: *mut net_iov) -> netmem_ref {
    (niov as netmem_ref) | NET_IOV
}

pub unsafe fn page_to_netmem(p: *const page) -> netmem_ref { p as netmem_ref }

pub unsafe fn virt_to_netmem(data: *const ::core::ffi::c_void) -> netmem_ref {
    page_to_netmem(virt_to_page(data))
}

pub unsafe fn netmem_ref_count(netmem: netmem_ref) -> ::core::ffi::c_int {
    if netmem_is_net_iov(netmem) { 1 } else { page_ref_count(netmem_to_page(netmem)) }
}

pub unsafe fn netmem_pfn_trace(netmem: netmem_ref) -> ::core::ffi::c_ulong {
    if netmem_is_net_iov(netmem) { 0 } else { page_to_pfn(netmem_to_page(netmem)) }
}

pub unsafe fn __netmem_to_nmdesc(netmem: netmem_ref) -> *mut netmem_desc {
    netmem as *mut netmem_desc
}

pub unsafe fn netmem_to_nmdesc(netmem: netmem_ref) -> *mut netmem_desc {
    let p = (netmem & !NET_IOV) as *mut ::core::ffi::c_void;
    if netmem_is_net_iov(netmem) {
        &mut (*(p as *mut net_iov)).desc
    } else {
        p as *mut netmem_desc
    }
}

pub unsafe fn __netmem_get_pp(netmem: netmem_ref) -> *mut page_pool {
    (*__netmem_to_nmdesc(netmem)).pp
}

pub unsafe fn netmem_get_pp(netmem: netmem_ref) -> *mut page_pool {
    (*netmem_to_nmdesc(netmem)).pp
}

pub unsafe fn netmem_get_pp_ref_count_ref(netmem: netmem_ref) -> *mut atomic_long_t {
    &mut (*netmem_to_nmdesc(netmem)).pp_ref_count
}

pub unsafe fn netmem_is_pref_nid(netmem: netmem_ref, pref_nid: ::core::ffi::c_int) -> bool {
    if netmem_is_net_iov(netmem) { true } else { page_to_nid(netmem_to_page(netmem)) == pref_nid }
}

pub unsafe fn netmem_compound_head(netmem: netmem_ref) -> netmem_ref {
    if netmem_is_net_iov(netmem) { netmem } else { page_to_netmem(compound_head(netmem_to_page(netmem))) }
}

pub unsafe fn __netmem_address(netmem: netmem_ref) -> *mut ::core::ffi::c_void {
    page_address(__netmem_to_page(netmem))
}

pub unsafe fn netmem_address(netmem: netmem_ref) -> *mut ::core::ffi::c_void {
    if netmem_is_net_iov(netmem) { core::ptr::null_mut() } else { __netmem_address(netmem) }
}

pub unsafe fn netmem_is_pfmemalloc(netmem: netmem_ref) -> bool {
    if netmem_is_net_iov(netmem) { false } else { page_is_pfmemalloc(netmem_to_page(netmem)) }
}

pub unsafe fn netmem_get_dma_addr(netmem: netmem_ref) -> ::core::ffi::c_ulong {
    (*netmem_to_nmdesc(netmem)).dma_addr
}

#[cfg(feature = "CONFIG_NET_DEVMEM")]
pub unsafe fn net_is_devmem_iov(niov: *const net_iov) -> bool {
    (*niov).type_ == net_iov_type::NET_IOV_DMABUF
}

#[cfg(not(feature = "CONFIG_NET_DEVMEM"))]
pub unsafe fn net_is_devmem_iov(_niov: *const net_iov) -> bool { false }

extern "C" {
    pub fn __get_netmem(netmem: netmem_ref);
    pub fn __put_netmem(netmem: netmem_ref);
}

pub unsafe fn get_netmem(netmem: netmem_ref) {
    if netmem_is_net_iov(netmem) { __get_netmem(netmem); } else { get_page(netmem_to_page(netmem)); }
}

pub unsafe fn put_netmem(netmem: netmem_ref) {
    if netmem_is_net_iov(netmem) { __put_netmem(netmem); } else { put_page(netmem_to_page(netmem)); }
}

pub unsafe fn netmem_dma_unmap_page_attrs(dev: *mut device, addr: dma_addr_t, size: usize,
                                           dir: dma_data_direction, attrs: ::core::ffi::c_ulong) {
    if addr == 0 { return; }
    dma_unmap_page_attrs(dev, addr, size, dir, attrs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
