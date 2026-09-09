/* SPDX-License-Identifier: GPL-2.0-only */
/* Faithful low-level Rust translation of linux/iommu.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const IOMMU_READ: u32 = 1 << 0;
pub const IOMMU_WRITE: u32 = 1 << 1;
pub const IOMMU_CACHE: u32 = 1 << 2;
pub const IOMMU_NOEXEC: u32 = 1 << 3;
pub const IOMMU_MMIO: u32 = 1 << 4;
pub const IOMMU_PRIV: u32 = 1 << 5;
pub const IOMMU_FAULT_PERM_READ: u32 = 1 << 0;
pub const IOMMU_FAULT_PERM_WRITE: u32 = 1 << 1;
pub const IOMMU_FAULT_PERM_EXEC: u32 = 1 << 2;
pub const IOMMU_FAULT_PERM_PRIV: u32 = 1 << 3;

pub type u8_ = u8; pub type u32_ = u32; pub type u64_ = u64;
pub type ioasid_t = u32; pub type dma_addr_t = u64; pub type phys_addr_t = u64;
pub type gfp_t = u32; pub type ssize_t = isize;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: u32 }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct device { pub iommu: *mut dev_iommu }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub orig_nents: u32 }
#[repr(C)] pub struct iova_bitmap { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub iommu_mm: *mut iommu_mm_data }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct msi_desc { _private: [u8; 0] }
#[repr(C)] pub struct iommufd_hw_pagetable { _private: [u8; 0] }
#[repr(C)] pub struct iommufd_viommu { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { _private: [u8; 0] }
#[repr(C)] pub struct iommu_group { _private: [u8; 0] }
#[repr(C)] pub struct iommu_device { _private: [u8; 0] }

#[repr(u32)] pub enum iommu_fault_type { IOMMU_FAULT_PAGE_REQ = 1 }
#[repr(C)] pub struct iommu_fault_page_request { pub flags:u32, pub pasid:u32, pub grpid:u32, pub perm:u32, pub addr:u64, pub private_data:[u64;2] }
#[repr(C)] pub struct iommu_fault { pub type_:u32, pub prm:iommu_fault_page_request }
#[repr(u32)] pub enum iommu_page_response_code { IOMMU_PAGE_RESP_SUCCESS=0, IOMMU_PAGE_RESP_INVALID, IOMMU_PAGE_RESP_FAILURE }
#[repr(C)] pub struct iommu_page_response { pub pasid:u32, pub grpid:u32, pub code:u32 }
#[repr(C)] pub struct iopf_fault { pub fault:iommu_fault, pub list:list_head }
#[repr(C)] pub struct iopf_group { pub last_fault:iopf_fault, pub faults:list_head, pub fault_count:usize, pub pending_node:list_head, pub work:work_struct, pub attach_handle:*mut iommu_attach_handle, pub fault_param:*mut iommu_fault_param, pub node:list_head, pub cookie:u32 }
#[repr(C)] pub struct iopf_queue { pub wq:*mut workqueue_struct, pub devices:list_head, pub lock:mutex }
pub type iommu_fault_handler_t = unsafe extern "C" fn(*mut iommu_domain,*mut device,usize,i32,*mut core::ffi::c_void)->i32;
#[repr(C)] pub struct iommu_domain_geometry { pub aperture_start:dma_addr_t, pub aperture_end:dma_addr_t, pub force_aperture:bool }
#[repr(u32)] pub enum iommu_domain_cookie_type { IOMMU_COOKIE_NONE, IOMMU_COOKIE_DMA_IOVA, IOMMU_COOKIE_DMA_MSI, IOMMU_COOKIE_FAULT_HANDLER, IOMMU_COOKIE_SVA, IOMMU_COOKIE_IOMMUFD }
pub const __IOMMU_DOMAIN_PAGING:u32=1; pub const __IOMMU_DOMAIN_DMA_API:u32=2; pub const __IOMMU_DOMAIN_PT:u32=4; pub const __IOMMU_DOMAIN_DMA_FQ:u32=8; pub const __IOMMU_DOMAIN_SVA:u32=16; pub const __IOMMU_DOMAIN_PLATFORM:u32=32; pub const __IOMMU_DOMAIN_NESTED:u32=64;
pub const IOMMU_DOMAIN_BLOCKED:u32=0; pub const IOMMU_DOMAIN_IDENTITY:u32=4; pub const IOMMU_DOMAIN_UNMANAGED:u32=1; pub const IOMMU_DOMAIN_DMA:u32=3; pub const IOMMU_DOMAIN_DMA_FQ:u32=11; pub const IOMMU_DOMAIN_SVA:u32=16; pub const IOMMU_DOMAIN_PLATFORM:u32=32; pub const IOMMU_DOMAIN_NESTED:u32=64;
#[repr(C)] pub struct iommu_domain { pub type_:u32, pub cookie_type:iommu_domain_cookie_type, pub is_iommupt:bool, pub ops:*const iommu_domain_ops, pub dirty_ops:*const iommu_dirty_ops, pub owner:*const iommu_ops, pub pgsize_bitmap:usize, pub geometry:iommu_domain_geometry, pub iopf_handler:Option<unsafe extern "C" fn(*mut iopf_group)->i32> }
#[inline] pub unsafe fn iommu_is_dma_domain(d:*mut iommu_domain)->bool { (*d).type_ & __IOMMU_DOMAIN_DMA_API != 0 }
#[repr(u32)] pub enum iommu_cap { IOMMU_CAP_CACHE_COHERENCY, IOMMU_CAP_NOEXEC, IOMMU_CAP_PRE_BOOT_PROTECTION, IOMMU_CAP_ENFORCE_CACHE_COHERENCY, IOMMU_CAP_DEFERRED_FLUSH, IOMMU_CAP_DIRTY_TRACKING, IOMMU_CAP_PCI_ATS_SUPPORTED }
#[repr(u32)] pub enum iommu_resv_type { IOMMU_RESV_DIRECT, IOMMU_RESV_DIRECT_RELAXABLE, IOMMU_RESV_RESERVED, IOMMU_RESV_MSI, IOMMU_RESV_SW_MSI }
#[repr(C)] pub struct iommu_resv_region { pub list:list_head, pub start:phys_addr_t, pub length:usize, pub prot:i32, pub type_:iommu_resv_type, pub free:Option<unsafe extern "C" fn(*mut device,*mut iommu_resv_region)> }
#[repr(C)] pub struct iommu_iort_rmr_data { pub rr:iommu_resv_region, pub sids:*const u32, pub num_sids:u32 }
pub const IOMMU_NO_PASID:ioasid_t=0; pub const IOMMU_FIRST_GLOBAL_PASID:ioasid_t=1; pub const IOMMU_PASID_INVALID:ioasid_t=u32::MAX; pub const IOMMU_DIRTY_NO_CLEAR:u32=1;
#[repr(C)] pub struct iommu_pages_list { pub pages:list_head }
#[repr(C)] pub struct iommu_iotlb_gather { pub start:usize, pub end:usize, pub pgsize:usize, pub leaf_levels_bitmap:u8, pub table_levels_bitmap:u8, pub freelist:iommu_pages_list, pub queued:bool }
#[repr(C)] pub struct iommu_dirty_bitmap { pub bitmap:*mut iova_bitmap, pub gather:*mut iommu_iotlb_gather }
#[repr(C)] pub struct iommu_user_data { pub type_:u32, pub uptr:*mut core::ffi::c_void, pub len:usize }
#[repr(C)] pub struct iommu_user_data_array { pub type_:u32, pub uptr:*mut core::ffi::c_void, pub entry_len:usize, pub entry_num:u32 }
#[repr(C)] pub struct iommu_dirty_ops { pub set_dirty_tracking:Option<unsafe extern "C" fn(*mut iommu_domain,bool)->i32>, pub read_and_clear_dirty:Option<unsafe extern "C" fn(*mut iommu_domain,usize,usize,usize,*mut iommu_dirty_bitmap)->i32> }
#[repr(C)] pub struct iommu_domain_ops { pub attach_dev:Option<unsafe extern "C" fn(*mut iommu_domain,*mut device,*mut iommu_domain)->i32>, pub set_dev_pasid:Option<unsafe extern "C" fn(*mut iommu_domain,*mut device,ioasid_t,*mut iommu_domain)->i32>, pub free:Option<unsafe extern "C" fn(*mut iommu_domain)> }
#[repr(C)] pub struct iommu_ops { pub capable:Option<unsafe extern "C" fn(*mut device,iommu_cap)->bool>, pub domain_alloc_identity:Option<unsafe extern "C" fn(*mut device)->*mut iommu_domain>, pub default_domain_ops:*const iommu_domain_ops, pub owner:*mut module, pub identity_domain:*mut iommu_domain, pub blocked_domain:*mut iommu_domain, pub release_domain:*mut iommu_domain, pub default_domain:*mut iommu_domain, pub user_pasid_table:u8 }
#[repr(C)] pub struct iommu_fault_param { pub lock:mutex, pub users:refcount_t, pub rcu:rcu_head, pub dev:*mut device, pub queue:*mut iopf_queue, pub queue_list:list_head, pub partial:list_head, pub faults:list_head }
#[repr(C)] pub struct dev_iommu { pub lock:mutex, pub fault_param:*mut iommu_fault_param, pub fwspec:*mut iommu_fwspec, pub iommu_dev:*mut iommu_device, pub priv_:*mut core::ffi::c_void, pub max_pasids:u32, pub attach_deferred:u32, pub pci_32bit_workaround:u32, pub require_direct:u32, pub shadow_on_flush:u32 }
#[repr(C)] pub struct iommu_fwspec { pub iommu_fwnode:*mut fwnode_handle, pub flags:u32, pub num_ids:u32, pub ids:[u32;0] }
#[repr(C)] pub struct iommu_attach_handle { pub domain:*mut iommu_domain }
#[repr(C)] pub struct iommu_sva { pub handle:iommu_attach_handle, pub dev:*mut device, pub users:refcount_t }
#[repr(C)] pub struct iommu_mm_data { pub pasid:u32, pub mm:*mut mm_struct, pub sva_domains:list_head, pub mm_list_elm:list_head }

extern "C" {
    pub fn iommu_device_register(iommu:*mut iommu_device, ops:*const iommu_ops, hwdev:*mut device)->i32;
    pub fn iommu_device_unregister(iommu:*mut iommu_device);
    pub fn iommu_paging_domain_alloc_flags(dev:*mut device, flags:u32)->*mut iommu_domain;
    pub fn iommu_domain_free(domain:*mut iommu_domain);
    pub fn iommu_attach_device(domain:*mut iommu_domain, dev:*mut device)->i32;
    pub fn iommu_detach_device(domain:*mut iommu_domain, dev:*mut device);
    pub fn iommu_get_domain_for_dev(dev:*mut device)->*mut iommu_domain;
    pub fn iommu_map(domain:*mut iommu_domain,iova:usize,paddr:phys_addr_t,size:usize,prot:i32,gfp:gfp_t)->i32;
    pub fn iommu_unmap(domain:*mut iommu_domain,iova:usize,size:usize)->usize;
    pub fn iommu_iova_to_phys(domain:*mut iommu_domain,iova:dma_addr_t)->phys_addr_t;
    pub fn iommu_map_sg(domain:*mut iommu_domain,iova:usize,sg:*mut scatterlist,nents:u32,prot:i32,gfp:gfp_t)->ssize_t;
    pub fn iommu_group_alloc()->*mut iommu_group; pub fn iommu_group_put(group:*mut iommu_group);
    pub fn iommu_group_add_device(group:*mut iommu_group,dev:*mut device)->i32;
    pub fn iommu_fwspec_init(dev:*mut device,node:*mut fwnode_handle)->i32;
    pub fn iommu_fwspec_add_ids(dev:*mut device,ids:*const u32,num_ids:i32)->i32;
}

#[inline] pub unsafe fn iommu_paging_domain_alloc(dev:*mut device)->*mut iommu_domain { iommu_paging_domain_alloc_flags(dev,0) }
#[inline] pub unsafe fn dev_iommu_fwspec_get(dev:*mut device)->*mut iommu_fwspec { if (*dev).iommu.is_null(){core::ptr::null_mut()}else{(*(*dev).iommu).fwspec} }
#[inline] pub unsafe fn dev_iommu_priv_get(dev:*mut device)->*mut core::ffi::c_void { if (*dev).iommu.is_null(){core::ptr::null_mut()}else{(*(*dev).iommu).priv_} }
#[inline] pub unsafe fn mm_pasid_init(mm:*mut mm_struct){(*mm).iommu_mm=core::ptr::null_mut()}
#[inline] pub unsafe fn mm_valid_pasid(mm:*mut mm_struct)->bool{!(*mm).iommu_mm.is_null()}
#[inline] pub unsafe fn mm_get_enqcmd_pasid(mm:*mut mm_struct)->u32{if (*mm).iommu_mm.is_null(){IOMMU_PASID_INVALID}else{(*(*mm).iommu_mm).pasid}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
