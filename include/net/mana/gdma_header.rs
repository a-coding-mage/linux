/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2021, Microsoft Corporation. */
// Linux dependencies and build-time macros are supplied by the surrounding translation.

pub const GDMA_STATUS_MORE_ENTRIES: u32 = 0x00000105;
pub const GDMA_STATUS_CMD_UNSUPPORTED: u32 = 0xffff_ffff;

#[repr(u32)] pub enum gdma_request_type { GDMA_VERIFY_VF_DRIVER_VERSION=1, GDMA_QUERY_MAX_RESOURCES=2, GDMA_LIST_DEVICES=3, GDMA_REGISTER_DEVICE=4, GDMA_DEREGISTER_DEVICE=5, GDMA_GENERATE_TEST_EQE=10, GDMA_CREATE_QUEUE=12, GDMA_DISABLE_QUEUE=13, GDMA_ALLOCATE_RESOURCE_RANGE=22, GDMA_DESTROY_RESOURCE_RANGE=24, GDMA_CREATE_DMA_REGION=25, GDMA_DMA_REGION_ADD_PAGES=26, GDMA_DESTROY_DMA_REGION=27, GDMA_CREATE_PD=29, GDMA_DESTROY_PD=30, GDMA_CREATE_MR=31, GDMA_DESTROY_MR=32, GDMA_QUERY_HWC_TIMEOUT=84, GDMA_ALLOC_DM=96, GDMA_DESTROY_DM=97 }
pub const GDMA_RESOURCE_DOORBELL_PAGE: u32 = 27;
#[repr(u32)] pub enum gdma_queue_type { GDMA_INVALID_QUEUE, GDMA_SQ, GDMA_RQ, GDMA_CQ, GDMA_EQ, GDMA_DIM }
#[repr(u32)] pub enum gdma_work_request_flags { GDMA_WR_NONE=0, GDMA_WR_OOB_IN_SGL=1, GDMA_WR_PAD_BY_SGE0=2 }
#[repr(u32)] pub enum gdma_eqe_type { GDMA_EQE_COMPLETION=3, GDMA_EQE_TEST_EVENT=64, GDMA_EQE_HWC_INIT_EQ_ID_DB=129, GDMA_EQE_HWC_INIT_DATA=130, GDMA_EQE_HWC_INIT_DONE=131, GDMA_EQE_HWC_FPGA_RECONFIG=132, GDMA_EQE_HWC_SOC_RECONFIG_DATA=133, GDMA_EQE_HWC_SOC_SERVICE=134, GDMA_EQE_HWC_RESET_REQUEST=135, GDMA_EQE_RNIC_QP_FATAL=176 }
pub const GDMA_DEVICE_NONE:u32=0; pub const GDMA_DEVICE_HWC:u32=1; pub const GDMA_DEVICE_MANA:u32=2; pub const GDMA_DEVICE_MANA_IB:u32=3;
#[repr(u32)] pub enum gdma_service_type { GDMA_SERVICE_TYPE_NONE=0, GDMA_SERVICE_TYPE_RDMA_SUSPEND=1, GDMA_SERVICE_TYPE_RDMA_RESUME=2 }

#[repr(C)] pub struct mana_service_work { pub work: work_struct, pub gdma_dev: *mut gdma_dev, pub event: gdma_service_type }
#[repr(C)] pub struct gdma_resource { pub lock: spinlock_t, pub size:u32, pub map:*mut ::std::os::raw::c_ulong }
#[repr(C)] pub union gdma_doorbell_entry { pub as_uint64:u64, pub cq: gdma_db_cq, pub rq:gdma_db_rq, pub sq:gdma_db_sq, pub eq:gdma_db_eq, pub dim:gdma_db_dim }
#[repr(C)] pub struct gdma_db_cq { pub id:u64, pub reserved:u64, pub tail_ptr:u64, pub arm:u64 }
#[repr(C)] pub struct gdma_db_rq { pub id:u64, pub wqe_cnt:u64, pub tail_ptr:u64 }
#[repr(C)] pub struct gdma_db_sq { pub id:u64, pub reserved:u64, pub tail_ptr:u64 }
#[repr(C)] pub struct gdma_db_eq { pub id:u64, pub reserved:u64, pub tail_ptr:u64, pub arm:u64 }
#[repr(C)] pub struct gdma_db_dim { pub id:u64, pub reserved:u64, pub mod_usec:u64, pub reserve1:u64, pub mod_usec_vld:u64, pub mod_comps:u64, pub reserve2:u64, pub mod_comps_vld:u64 }
#[repr(C)] pub struct gdma_msg_hdr { pub hdr_type:u32,pub msg_type:u32,pub msg_version:u16,pub hwc_msg_id:u16,pub msg_size:u32 }
#[repr(C)] pub union gdma_dev_id_union { pub fields: gdma_dev_id_fields, pub as_uint32:u32 }
#[repr(C)] pub struct gdma_dev_id_fields { pub r#type:u16, pub instance:u16 }
#[repr(C)] pub struct gdma_dev_id { pub value:gdma_dev_id_union }
#[repr(C)] pub struct gdma_req_hdr { pub req:gdma_msg_hdr,pub resp:gdma_msg_hdr,pub dev_id:gdma_dev_id,pub activity_id:u32 }
#[repr(C)] pub struct gdma_resp_hdr { pub response:gdma_msg_hdr,pub dev_id:gdma_dev_id,pub activity_id:u32,pub status:u32,pub reserved:u32 }
#[repr(C)] pub struct gdma_general_req { pub hdr:gdma_req_hdr }
pub const GDMA_MESSAGE_V1:u16=1; pub const GDMA_MESSAGE_V2:u16=2; pub const GDMA_MESSAGE_V3:u16=3; pub const GDMA_MESSAGE_V4:u16=4; pub const GDMA_MESSAGE_V5:u16=5;
#[repr(C)] pub struct gdma_general_resp { pub hdr:gdma_resp_hdr }
pub const GDMA_STANDARD_HEADER_TYPE:u32=0;
pub unsafe fn mana_gd_init_req_hdr(hdr:*mut gdma_req_hdr,code:u32,req_size:u32,resp_size:u32){ (*hdr).req.hdr_type=0;(*hdr).req.msg_type=code;(*hdr).req.msg_version=1;(*hdr).req.msg_size=req_size;(*hdr).resp.hdr_type=0;(*hdr).resp.msg_type=code;(*hdr).resp.msg_version=1;(*hdr).resp.msg_size=resp_size; }
#[repr(C)] pub struct gdma_sge { pub address:u64,pub mem_key:u32,pub size:u32 }
#[repr(C)] pub struct gdma_wqe_request { pub sgl:*mut gdma_sge,pub num_sge:u32,pub inline_oob_size:u32,pub inline_oob_data:*const ::std::ffi::c_void,pub flags:u32,pub client_data_unit:u32 }
#[repr(u32)] pub enum gdma_page_type { GDMA_PAGE_TYPE_4K }
pub const GDMA_INVALID_DMA_REGION:u32=0;
#[repr(C)] pub struct mana_serv_work { pub serv_work:work_struct,pub pdev:*mut pci_dev,pub r#type:gdma_eqe_type }
#[repr(C)] pub struct gdma_mem_info { pub dev:*mut device,pub dma_handle:dma_addr_t,pub virt_addr:*mut ::std::ffi::c_void,pub length:u64,pub pages_va:*mut *mut ::std::ffi::c_void,pub pages_dma:*mut dma_addr_t,pub nr_pages:u32,pub dma_region_handle:u64 }
pub const REGISTER_ATB_MST_MKEY_LOWER_SIZE:u32=8;
#[repr(C)] pub struct gdma_dev { pub gdma_context:*mut gdma_context,pub dev_id:gdma_dev_id,pub pdid:u32,pub doorbell:u32,pub gpa_mkey:u32,pub driver_data:*mut ::std::ffi::c_void,pub adev:*mut auxiliary_device,pub is_suspended:bool,pub rdma_teardown:bool }
pub const MANA_PAGE_SHIFT:u32=12; pub const MANA_PAGE_SIZE:u32=1<<MANA_PAGE_SHIFT; pub const MANA_MIN_QSIZE:u32=MANA_PAGE_SIZE;
pub const GDMA_CQE_SIZE:u32=64; pub const GDMA_EQE_SIZE:u32=16; pub const GDMA_MAX_SQE_SIZE:u32=512; pub const GDMA_MAX_RQE_SIZE:u32=256; pub const GDMA_COMP_DATA_SIZE:u32=0x3c; pub const GDMA_EVENT_DATA_SIZE:u32=0xc; pub const GDMA_WQE_BU_SIZE:u32=32;
pub const INVALID_PDID:u32=u32::MAX; pub const INVALID_DOORBELL:u32=u32::MAX; pub const INVALID_MEM_KEY:u32=u32::MAX; pub const INVALID_QUEUE_ID:u32=u32::MAX; pub const INVALID_PCI_MSIX_INDEX:u32=u32::MAX;
#[repr(C)] pub struct gdma_comp { pub cqe_data:[u32;15],pub wq_num:u32,pub is_sq:bool }
#[repr(C)] pub struct gdma_event { pub details:[u32;3],pub r#type:u8 }
pub struct gdma_queue;
pub type gdma_eq_callback=unsafe extern "C" fn(*mut ::std::ffi::c_void,*mut gdma_queue,*mut gdma_event);
pub type gdma_cq_callback=unsafe extern "C" fn(*mut ::std::ffi::c_void,*mut gdma_queue);
#[repr(C)] pub struct mana_eq { pub eq:*mut gdma_queue,pub mana_eq_debugfs:*mut dentry }
#[repr(C)] pub struct gdma_queue { pub gdma_dev:*mut gdma_dev,pub r#type:gdma_queue_type,pub id:u32,pub mem_info:gdma_mem_info,pub queue_mem_ptr:*mut ::std::ffi::c_void,pub queue_size:u32,pub monitor_avl_buf:bool,pub head:u32,pub tail:u32,pub entry:list_head }
#[repr(C)] pub struct gdma_queue_spec { pub r#type:gdma_queue_type,pub monitor_avl_buf:bool,pub queue_size:u32 }
pub const MANA_IRQ_NAME_SZ:usize=32;
#[repr(C)] pub struct gdma_irq_context { pub handler:Option<unsafe extern "C" fn(*mut ::std::ffi::c_void)>,pub lock:spinlock_t,pub eq_list:list_head,pub name:[i8;32],pub msi:u32,pub irq:u32,pub refcount:refcount_t,pub bitmap_refs:u32,pub dyn_msix:bool }
#[repr(C)] pub struct gdma_context { pub dev:*mut device,pub mana_pci_debugfs:*mut dentry,pub max_num_queues:u32,pub max_num_queues_vport:u32,pub max_num_msix:u32,pub num_msix_usable:u32,pub irq_contexts:xarray,pub adapter_mtu:u16,pub cqe8_coalescing_sup:bool,pub max_num_cqs:u32,pub cq_table:*mut *mut gdma_queue,pub eq_test_event_mutex:mutex,pub eq_test_event:completion,pub test_event_eq_id:u32,pub is_pf:bool,pub is_pf2:bool,pub bar0_pa:phys_addr_t,pub bar0_va:*mut ::std::ffi::c_void,pub bar0_size:resource_size_t,pub shm_base:*mut ::std::ffi::c_void,pub db_page_base:*mut ::std::ffi::c_void,pub phys_db_page_base:phys_addr_t,pub db_page_off:u64,pub db_page_size:u64,pub numa_node:i32,pub shm_channel:shm_channel,pub hwc:gdma_dev,pub mana:gdma_dev,pub mana_ib:gdma_dev,pub pf_cap_flags1:u64,pub gdma_protocol_ver:u64,pub service_wq:*mut workqueue_struct,pub flags:usize,pub gic_mutex:mutex,pub msi_sharing:bool,pub msi_bitmap:*mut usize }
pub unsafe fn mana_gd_is_mana(gd:*mut gdma_dev)->bool{(*gd).dev_id.value.as_uint32==GDMA_DEVICE_MANA} pub unsafe fn mana_gd_is_hwc(gd:*mut gdma_dev)->bool{(*gd).dev_id.value.as_uint32==GDMA_DEVICE_HWC}

extern "C" { pub fn mana_gd_get_wqe_ptr(wq:*const gdma_queue,wqe_offset:u32)->*mut u8; pub fn mana_gd_wq_avail_space(wq:*mut gdma_queue)->u32; pub fn mana_gd_test_eq(gc:*mut gdma_context,eq:*mut gdma_queue)->i32; pub fn mana_gd_create_hwc_queue(gd:*mut gdma_dev,spec:*const gdma_queue_spec,queue_ptr:*mut *mut gdma_queue)->i32; pub fn mana_gd_create_mana_eq(gd:*mut gdma_dev,spec:*const gdma_queue_spec,queue_ptr:*mut *mut gdma_queue)->i32; pub fn mana_gd_create_mana_wq_cq(gd:*mut gdma_dev,spec:*const gdma_queue_spec,queue_ptr:*mut *mut gdma_queue)->i32; pub fn mana_gd_destroy_queue(gc:*mut gdma_context,q:*mut gdma_queue); pub fn mana_gd_poll_cq(cq:*mut gdma_queue,comp:*mut gdma_comp,num_cqe:i32)->i32; pub fn mana_gd_ring_cq(cq:*mut gdma_queue,arm_bit:u8); pub fn mana_gd_read_ring(q:*mut gdma_queue,buf:*mut i8,count:usize,pos:*mut i64)->isize; pub fn mana_schedule_serv_work(gc:*mut gdma_context,r#type:gdma_eqe_type)->i32; pub fn mana_gd_ring_dim(cq:*mut gdma_queue,mod_usec:u32,mod_usec_vld:bool,mod_comps:u32,mod_comps_vld:bool); }
#[repr(C)] pub struct gdma_wqe { pub reserved:u32,pub last_vbytes:u32,pub flags:u32 }
pub const INLINE_OOB_SMALL_SIZE:u32=8; pub const INLINE_OOB_LARGE_SIZE:u32=24; pub const MANA_MAX_TX_WQE_SGL_ENTRIES:u32=30; pub const MAX_TX_WQE_SIZE:u32=512; pub const MAX_RX_WQE_SIZE:u32=256;
pub const MAX_TX_WQE_SGL_ENTRIES:u32=(GDMA_MAX_SQE_SIZE-16-INLINE_OOB_SMALL_SIZE)/16; pub const MAX_RX_WQE_SGL_ENTRIES:u32=(GDMA_MAX_RQE_SIZE-16)/16;
#[repr(C)] pub struct gdma_cqe { pub cqe_data:[u32;15],pub cqe_info:u32 }
pub const GDMA_CQE_OWNER_BITS:u32=3; pub const GDMA_CQE_OWNER_MASK:u32=(1<<GDMA_CQE_OWNER_BITS)-1; pub const SET_ARM_BIT:u32=1; pub const GDMA_EQE_OWNER_BITS:u32=3;
#[repr(C)] pub union gdma_eqe_info { pub as_uint32:u32,pub fields:gdma_eqe_info_fields }
#[repr(C)] pub struct gdma_eqe_info_fields { pub r#type:u32,pub reserved1:u32,pub client_id:u32,pub reserved2:u32,pub owner_bits:u32 }
pub const GDMA_EQE_OWNER_MASK:u32=(1<<GDMA_EQE_OWNER_BITS)-1;
#[inline] pub const fn INITIALIZED_OWNER_BIT(log2_num_entries:u32)->usize{1usize<<log2_num_entries}
#[repr(C)] pub struct gdma_eqe { pub details:[u32;3],pub eqe_info:u32 }
pub const GDMA_REG_DB_PAGE_OFFSET:u32=8; pub const GDMA_REG_DB_PAGE_SIZE:u32=0x10; pub const GDMA_REG_SHM_OFFSET:u32=0x18; pub const GDMA_PF_REG_DB_PAGE_SIZE:u32=0xd0; pub const GDMA_PF_REG_DB_PAGE_OFF:u32=0xc8; pub const GDMA_PF_REG_SHM_OFF:u32=0x70; pub const GDMA_SRIOV_REG_CFG_BASE_OFF:u32=0x108; pub const MANA_PF_DEVICE_ID:u32=0xb9; pub const MANA_PF2_DEVICE_ID:u32=0xc1; pub const MANA_VF_DEVICE_ID:u32=0xba;
#[repr(C)] pub struct gdma_posted_wqe_info { pub wqe_size_in_bu:u32 }
#[repr(C)] pub struct gdma_generate_test_event_req { pub hdr:gdma_req_hdr,pub queue_index:u32 }
pub const GDMA_PROTOCOL_V1:u32=1; pub const GDMA_PROTOCOL_FIRST:u32=1; pub const GDMA_PROTOCOL_LAST:u32=1; pub const GDMA_DRV_CAP_FLAG_1_EQ_SHARING_MULTI_VPORT:u64=1<<0; pub const GDMA_DRV_CAP_FLAG_1_NAPI_WKDONE_FIX:u64=1<<2; pub const GDMA_DRV_CAP_FLAG_1_HWC_TIMEOUT_RECONFIG:u64=1<<3; pub const GDMA_DRV_CAP_FLAG_1_GDMA_PAGES_4MB_1GB_2GB:u64=1<<4; pub const GDMA_DRV_CAP_FLAG_1_VARIABLE_INDIRECTION_TABLE_SUPPORT:u64=1<<5; pub const GDMA_DRV_CAP_FLAG_1_HW_VPORT_LINK_AWARE:u64=1<<6; pub const GDMA_DRV_CAP_FLAG_1_DEV_LIST_HOLES_SUP:u64=1<<11; pub const GDMA_DRV_CAP_FLAG_1_DYNAMIC_IRQ_ALLOC_SUPPORT:u64=1<<13; pub const GDMA_DRV_CAP_FLAG_1_SELF_RESET_ON_EQE:u64=1<<14; pub const GDMA_DRV_CAP_FLAG_1_HANDLE_RECONFIG_EQE:u64=1<<17; pub const GDMA_DRV_CAP_FLAG_1_HANDLE_STALL_SQ_RECOVERY:u64=1<<18; pub const GDMA_DRV_CAP_FLAG_1_EQ_MSI_UNSHARE_MULTI_VPORT:u64=1<<19; pub const GDMA_DRV_CAP_FLAG_1_SKB_LINEARIZE:u64=1<<20; pub const GDMA_DRV_CAP_FLAG_1_PERIODIC_STATS_QUERY:u64=1<<21; pub const GDMA_DRV_CAP_FLAG_1_PROBE_RECOVERY:u64=1<<22; pub const GDMA_DRV_CAP_FLAG_1_HWC_TIMEOUT_RECOVERY:u64=1<<25; pub const GDMA_DRV_CAP_FLAG_1_DYN_INTERRUPT_MODERATION:u64=1<<28; pub const GDMA_DRV_CAP_FLAG_1_NON_CONTIGUOUS_BUFFERS:u64=1<<30;
pub const GDMA_DRV_CAP_FLAGS1:u64=GDMA_DRV_CAP_FLAG_1_EQ_SHARING_MULTI_VPORT|GDMA_DRV_CAP_FLAG_1_NAPI_WKDONE_FIX|GDMA_DRV_CAP_FLAG_1_HWC_TIMEOUT_RECONFIG|GDMA_DRV_CAP_FLAG_1_VARIABLE_INDIRECTION_TABLE_SUPPORT|GDMA_DRV_CAP_FLAG_1_DEV_LIST_HOLES_SUP|GDMA_DRV_CAP_FLAG_1_DYNAMIC_IRQ_ALLOC_SUPPORT|GDMA_DRV_CAP_FLAG_1_SELF_RESET_ON_EQE|GDMA_DRV_CAP_FLAG_1_HANDLE_RECONFIG_EQE|GDMA_DRV_CAP_FLAG_1_HW_VPORT_LINK_AWARE|GDMA_DRV_CAP_FLAG_1_PERIODIC_STATS_QUERY|GDMA_DRV_CAP_FLAG_1_SKB_LINEARIZE|GDMA_DRV_CAP_FLAG_1_PROBE_RECOVERY|GDMA_DRV_CAP_FLAG_1_HANDLE_STALL_SQ_RECOVERY|GDMA_DRV_CAP_FLAG_1_HWC_TIMEOUT_RECOVERY|GDMA_DRV_CAP_FLAG_1_EQ_MSI_UNSHARE_MULTI_VPORT|GDMA_DRV_CAP_FLAG_1_DYN_INTERRUPT_MODERATION|GDMA_DRV_CAP_FLAG_1_NON_CONTIGUOUS_BUFFERS;
#[repr(C)] pub struct gdma_verify_ver_req { pub hdr:gdma_req_hdr,pub protocol_ver_min:u64,pub protocol_ver_max:u64,pub gd_drv_cap_flags1:u64,pub gd_drv_cap_flags2:u64,pub gd_drv_cap_flags3:u64,pub gd_drv_cap_flags4:u64,pub drv_ver:u64,pub os_type:u32,pub reserved:u32,pub os_ver_major:u32,pub os_ver_minor:u32,pub os_ver_build:u32,pub os_ver_platform:u32,pub reserved_2:u64,pub os_ver_str1:[u8;128],pub os_ver_str2:[u8;128],pub os_ver_str3:[u8;128],pub os_ver_str4:[u8;128] }
pub const GDMA_PF_CAP_FLAG_1_DYN_INTERRUPT_MODERATION:u64=1<<15;
#[repr(C)] pub struct gdma_verify_ver_resp { pub hdr:gdma_resp_hdr,pub gdma_protocol_ver:u64,pub pf_cap_flags1:u64,pub pf_cap_flags2:u64,pub pf_cap_flags3:u64,pub pf_cap_flags4:u64 }
#[repr(C)] pub struct gdma_query_max_resources_resp { pub hdr:gdma_resp_hdr,pub status:u32,pub max_sq:u32,pub max_rq:u32,pub max_cq:u32,pub max_eq:u32,pub max_db:u32,pub max_mst:u32,pub max_cq_mod_ctx:u32,pub max_mod_cq:u32,pub max_msix:u32 }
pub const GDMA_DEV_LIST_SIZE:usize=64; #[repr(C)] pub struct gdma_list_devices_resp { pub hdr:gdma_resp_hdr,pub num_of_devs:u32,pub reserved:u32,pub devs:[gdma_dev_id;64] }
#[repr(C)] pub struct gdma_register_device_resp { pub hdr:gdma_resp_hdr,pub pdid:u32,pub gpa_mkey:u32,pub db_id:u32 }
#[repr(C)] pub struct gdma_allocate_resource_range_req { pub hdr:gdma_req_hdr,pub resource_type:u32,pub num_resources:u32,pub alignment:u32,pub allocated_resources:u32 }
#[repr(C)] pub struct gdma_allocate_resource_range_resp { pub hdr:gdma_resp_hdr,pub allocated_resources:u32 }
#[repr(C)] pub struct gdma_destroy_resource_range_req { pub hdr:gdma_req_hdr,pub resource_type:u32,pub num_resources:u32,pub allocated_resources:u32 }
#[repr(C)] pub struct gdma_create_queue_req { pub hdr:gdma_req_hdr,pub r#type:u32,pub reserved1:u32,pub pdid:u32,pub doolbell_id:u32,pub gdma_region:u64,pub reserved2:u32,pub queue_size:u32,pub log2_throttle_limit:u32,pub eq_pci_msix_index:u32,pub cq_mod_ctx_id:u32,pub cq_parent_eq_id:u32,pub rq_drop_on_overrun:u8,pub rq_err_on_wqe_overflow:u8,pub rq_chain_rec_wqes:u8,pub sq_hw_db:u8,pub reserved3:u32 }
#[repr(C)] pub struct gdma_create_queue_resp { pub hdr:gdma_resp_hdr,pub queue_index:u32 }
#[repr(C)] pub struct gdma_disable_queue_req { pub hdr:gdma_req_hdr,pub r#type:u32,pub queue_index:u32,pub alloc_res_id_on_creation:u32 }
#[repr(C)] pub struct gdma_query_hwc_timeout_req { pub hdr:gdma_req_hdr,pub timeout_ms:u32,pub reserved:u32 } #[repr(C)] pub struct gdma_query_hwc_timeout_resp { pub hdr:gdma_resp_hdr,pub timeout_ms:u32,pub reserved:u32 }
#[repr(u64)] pub enum gdma_mr_access_flags { GDMA_ACCESS_FLAG_LOCAL_READ=1,GDMA_ACCESS_FLAG_LOCAL_WRITE=2,GDMA_ACCESS_FLAG_REMOTE_READ=4,GDMA_ACCESS_FLAG_REMOTE_WRITE=8,GDMA_ACCESS_FLAG_REMOTE_ATOMIC=16,GDMA_ACCESS_FLAG_BIND_MW=32 }
#[repr(C)] pub struct gdma_create_dma_region_req { pub hdr:gdma_req_hdr,pub length:u64,pub offset_in_page:u32,pub gdma_page_type:u32,pub page_count:u32,pub page_addr_list_len:u32,pub page_addr_list:[u64;0] }
#[repr(C)] pub struct gdma_create_dma_region_resp { pub hdr:gdma_resp_hdr,pub dma_region_handle:u64 } #[repr(C)] pub struct gdma_dma_region_add_pages_req { pub hdr:gdma_req_hdr,pub dma_region_handle:u64,pub page_addr_list_len:u32,pub reserved3:u32,pub page_addr_list:[u64;0] } #[repr(C)] pub struct gdma_destroy_dma_region_req { pub hdr:gdma_req_hdr,pub dma_region_handle:u64 }
#[repr(u32)] pub enum gdma_pd_flags { GDMA_PD_FLAG_ALLOW_GPA_MR=1,GDMA_PD_FLAG_SHORT_PDN=4 }
#[repr(C)] pub struct gdma_create_pd_req { pub hdr:gdma_req_hdr,pub flags:gdma_pd_flags,pub reserved:u32 } #[repr(C)] pub struct gdma_create_pd_resp { pub hdr:gdma_resp_hdr,pub pd_handle:u64,pub pd_id:u32,pub reserved:u32 } #[repr(C)] pub struct gdma_destroy_pd_req { pub hdr:gdma_req_hdr,pub pd_handle:u64 } #[repr(C)] pub struct gdma_destroy_pd_resp { pub hdr:gdma_resp_hdr }
#[repr(u32)] pub enum gdma_mr_type { GDMA_MR_TYPE_GPA=1,GDMA_MR_TYPE_GVA=2,GDMA_MR_TYPE_ZBVA=4,GDMA_MR_TYPE_DM=5,GDMA_MR_TYPE_MW1=6,GDMA_MR_TYPE_MW2=7 }
#[repr(C)] pub struct gdma_create_mr_params { pub pd_handle:u64,pub mr_type:gdma_mr_type,pub data:[u64;4] }
#[repr(C)] pub struct gdma_create_mr_request { pub hdr:gdma_req_hdr,pub pd_handle:u64,pub mr_type:gdma_mr_type,pub reserved_1:u32,pub data:[u64;4],pub reserved_2:u32,pub length:u64 }
#[repr(C)] pub struct gdma_create_mr_response { pub hdr:gdma_resp_hdr,pub mr_handle:u64,pub lkey:u32,pub rkey:u32 } #[repr(C)] pub struct gdma_destroy_mr_request { pub hdr:gdma_req_hdr,pub mr_handle:u64 } #[repr(C)] pub struct gdma_destroy_mr_response { pub hdr:gdma_resp_hdr }
#[repr(C)] pub struct gdma_alloc_dm_req { pub hdr:gdma_req_hdr,pub length:u64,pub alignment:u32,pub flags:u32 } #[repr(C)] pub struct gdma_alloc_dm_resp { pub hdr:gdma_resp_hdr,pub dm_handle:u64 } #[repr(C)] pub struct gdma_destroy_dm_req { pub hdr:gdma_req_hdr,pub dm_handle:u64 } #[repr(C)] pub struct gdma_destroy_dm_resp { pub hdr:gdma_resp_hdr }
extern "C" { pub fn mana_gd_verify_vf_version(pdev:*mut pci_dev)->i32; pub fn mana_gd_register_device(gd:*mut gdma_dev)->i32; pub fn mana_gd_deregister_device(gd:*mut gdma_dev)->i32; pub fn mana_gd_post_work_request(wq:*mut gdma_queue,wqe_req:*const gdma_wqe_request,wqe_info:*mut gdma_posted_wqe_info)->i32; pub fn mana_gd_post_and_ring(queue:*mut gdma_queue,wqe:*const gdma_wqe_request,wqe_info:*mut gdma_posted_wqe_info)->i32; pub fn mana_gd_alloc_res_map(res_avail:u32,r:*mut gdma_resource)->i32; pub fn mana_gd_free_res_map(r:*mut gdma_resource); pub fn mana_gd_wq_ring_doorbell(gc:*mut gdma_context,queue:*mut gdma_queue); pub fn mana_gd_alloc_memory(gc:*mut gdma_context,length:u32,gmi:*mut gdma_mem_info,allow_scatter:bool)->i32; pub fn mana_gd_free_memory(gmi:*mut gdma_mem_info); pub fn mana_gd_send_request(gc:*mut gdma_context,req_len:u32,req:*const ::std::ffi::c_void,resp_len:u32,resp:*mut ::std::ffi::c_void)->i32; pub fn mana_gd_destroy_dma_region(gc:*mut gdma_context,dma_region_handle:u64)->i32; pub fn mana_register_debugfs(); pub fn mana_unregister_debugfs(); pub fn mana_rdma_service_event(gc:*mut gdma_context,event:gdma_service_type)->i32; pub fn mana_gd_suspend(pdev:*mut pci_dev,state:pm_message_t)->i32; pub fn mana_gd_resume(pdev:*mut pci_dev)->i32; pub fn mana_need_log(gc:*mut gdma_context,err:i32)->bool; pub fn mana_gd_get_gic(gc:*mut gdma_context,use_msi_bitmap:bool,msi_requested:*mut i32)->*mut gdma_irq_context; pub fn mana_gd_put_gic(gc:*mut gdma_context,use_msi_bitmap:bool,msi:i32); pub fn mana_gd_query_device_cfg(gc:*mut gdma_context,proto_major_ver:u32,proto_minor_ver:u32,proto_micro_ver:u32,max_num_vports:*mut u16,bm_hostmode:*mut u8)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
