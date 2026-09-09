/* Rust translation of pnfs.h. External kernel types and symbols are supplied by dependencies. */

use core::ffi::c_void;

#[repr(C)] pub struct sockaddr_storage { _priv: [u8; 128] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct refcount_t { pub value: i32 }
#[repr(C)] pub struct atomic_t { pub value: i32 }
#[repr(C)] pub struct rcu_head { _priv: [u8; 0] }

pub type u32 = std::os::raw::c_uint; pub type u64 = std::os::raw::c_ulonglong;
pub type loff_t = i64; pub type size_t = usize; pub type gfp_t = u32;
pub type nfs4_stateid = [u8; 16];
pub enum nfs4_exception {} pub enum nfs4_opendata {} pub enum net {} pub enum nfs_client {}
pub enum nfs_server {} pub enum nfs_page {} pub enum inode {} pub enum nfs_inode {}
pub enum nfs_fh {} pub enum nfs_fsinfo {} pub enum nfs_pgio_header {}
pub enum nfs_pageio_descriptor {} pub enum nfs_commit_info {} pub enum nfs_commit_data {}
pub enum nfs_open_context {} pub enum rpc_task {} pub enum cred {} pub enum page {}
pub enum xdr_stream {} pub enum nfs4_layoutget {} pub enum nfs4_layoutreturn {}
pub enum nfs4_layoutreturn_args {} pub enum nfs4_layoutreturn_res {}
pub enum nfs4_layoutcommit_data {} pub enum nfs4_layoutcommit_args {}
pub enum nfs42_layoutstat_args {} pub enum nfs4_threshold {}
pub enum module {} pub enum nfs_pageio_ops {}

#[repr(C)] pub struct nfs4_deviceid { pub data: [u8; 16] }
#[repr(C)] pub struct pnfs_layout_range { pub offset: u64, pub length: u64, pub iomode: i32, pub type_: i32 }
#[repr(C)] pub struct pnfs_ds_commit_info { pub commits: list_head, pub ops: *const pnfs_commit_ops, pub ncommitting: u32, pub nwritten: u32 }

pub const NFS_LSEG_VALID: u32=0; pub const NFS_LSEG_ROC:u32=1; pub const NFS_LSEG_LAYOUTCOMMIT:u32=2; pub const NFS_LSEG_LAYOUTRETURN:u32=3; pub const NFS_LSEG_UNAVAILABLE:u32=4;
pub const PNFS_ATTEMPTED:i32=0; pub const PNFS_NOT_ATTEMPTED:i32=1; pub const PNFS_TRY_AGAIN:i32=2;
pub const NFS4_DEF_DS_TIMEO:u32=600; pub const NFS4_DEF_DS_RETRANS:u32=5; pub const PNFS_DEVICE_RETRY_TIMEOUT:u32=120;
pub const PNFS_LAYOUTRET_ON_SETATTR:u32=1<<0; pub const PNFS_LAYOUTRET_ON_ERROR:u32=1<<1; pub const PNFS_READ_WHOLE_PAGE:u32=1<<2; pub const PNFS_LAYOUTGET_ON_OPEN:u32=1<<3;
pub const NFS4_PNFS_GETDEVLIST_MAXNUM:usize=16; pub const PNFS_FL_LAYOUTRETURN_ASYNC:u32=1; pub const PNFS_FL_LAYOUTRETURN_PRIVILEGED:u32=2;
pub const NFS4_MAX_UINT64:u64=u64::MAX;

#[repr(C)] pub struct nfs4_pnfs_ds_addr { pub da_addr:sockaddr_storage, pub da_addrlen:size_t, pub da_node:list_head, pub da_remotestr:*mut i8, pub da_netid:*const i8, pub da_transport:i32 }
#[repr(C)] pub struct nfs4_pnfs_ds { pub ds_node:list_head, pub ds_remotestr:*mut i8, pub ds_addrs:list_head, pub ds_net:*const net, pub ds_clp:*mut nfs_client, pub ds_count:refcount_t, pub ds_version:u32, pub ds_state:usize }
#[repr(C)] pub struct pnfs_layout_segment { pub pls_list:list_head, pub pls_lc_list:list_head, pub pls_commits:list_head, pub pls_range:pnfs_layout_range, pub pls_refcount:refcount_t, pub pls_seq:u32, pub pls_flags:usize, pub pls_layout:*mut pnfs_layout_hdr }
#[repr(C)] pub struct pnfs_layout_hdr { pub plh_refcount:refcount_t, pub plh_outstanding:atomic_t, pub plh_layouts:list_head, pub plh_bulk_destroy:list_head, pub plh_segs:list_head, pub plh_return_segs:list_head, pub plh_block_lgets:usize, pub plh_retry_timestamp:usize, pub plh_flags:usize, pub plh_stateid:nfs4_stateid, pub plh_barrier:u32, pub plh_return_seq:u32, pub plh_return_iomode:i32, pub plh_lwb:loff_t, pub plh_lc_cred:*const cred, pub plh_inode:*mut inode, pub plh_rcu:rcu_head }
#[repr(C)] pub struct pnfs_device { pub dev_id:nfs4_deviceid, pub layout_type:u32, pub mincount:u32, pub maxcount:u32, pub pages:*mut *mut page, pub pgbase:u32, pub pglen:u32, pub nocache:u8 }
#[repr(C)] pub struct pnfs_devicelist { pub eof:u32, pub num_devs:u32, pub dev_id:[nfs4_deviceid;16] }
#[repr(C)] pub struct nfs4_deviceid_node { pub node:hlist_node, pub tmpnode:hlist_node, pub ld:*const pnfs_layoutdriver_type, pub nfs_client:*const nfs_client, pub flags:usize, pub timestamp_unavailable:usize, pub deviceid:nfs4_deviceid, pub rcu:rcu_head, pub ref_:atomic_t }

#[repr(C)] pub struct pnfs_layoutdriver_type { pub pnfs_tblid:list_head, pub id:u32, pub name:*const i8, pub owner:*mut module, pub flags:u32, pub max_layoutget_response:u32 }
#[repr(C)] pub struct pnfs_commit_ops { pub setup_ds_info:Option<unsafe extern "C" fn(*mut pnfs_ds_commit_info,*mut pnfs_layout_segment)>, pub release_ds_info:Option<unsafe extern "C" fn(*mut pnfs_ds_commit_info,*mut inode)>, pub commit_pagelist:Option<unsafe extern "C" fn(*mut inode,*mut list_head,i32,*mut nfs_commit_info)->i32> }

extern "C" { pub fn pnfs_register_layoutdriver(p:*mut pnfs_layoutdriver_type)->i32; pub fn pnfs_unregister_layoutdriver(p:*mut pnfs_layoutdriver_type); pub fn pnfs_find_layoutdriver(id:u32)->*const pnfs_layoutdriver_type; pub fn pnfs_put_layoutdriver(p:*const pnfs_layoutdriver_type); }
extern "C" { pub fn pnfs_get_layout_hdr(p:*mut pnfs_layout_hdr); pub fn pnfs_put_lseg(p:*mut pnfs_layout_segment); pub fn _pnfs_return_layout(p:*mut inode)->i32; }
extern "C" {
 pub fn pnfs_destroy_layout(p:*mut nfs_inode); pub fn pnfs_destroy_layout_final(p:*mut nfs_inode); pub fn pnfs_destroy_all_layouts(p:*mut nfs_client);
 pub fn pnfs_layout_destroy_byfsid(p:*mut nfs_client,fsid:*mut c_void,mode:i32)->i32; pub fn pnfs_layout_destroy_byclid(p:*mut nfs_client,mode:i32)->i32;
 pub fn pnfs_layout_process(p:*mut nfs4_layoutget)->*mut pnfs_layout_segment; pub fn pnfs_layoutget_free(p:*mut nfs4_layoutget); pub fn pnfs_free_lseg_list(p:*mut list_head);
 pub fn pnfs_put_layout_hdr(p:*mut pnfs_layout_hdr); pub fn pnfs_layoutcommit_inode(p:*mut inode,sync:bool)->i32; pub fn pnfs_generic_sync(p:*mut inode,datasync:bool)->i32;
 pub fn pnfs_nfs_generic_sync(p:*mut inode,datasync:bool)->i32; pub fn pnfs_commit_and_return_layout(p:*mut inode)->i32;
 pub fn pnfs_update_layout(ino:*mut inode,ctx:*mut nfs_open_context,pos:loff_t,count:u64,iomode:i32,strict:bool,gfp:gfp_t)->*mut pnfs_layout_segment;
 pub fn nfs4_deviceid_mark_client_invalid(p:*mut nfs_client); pub fn pnfs_read_done_resend_to_mds(p:*mut nfs_pgio_header)->i32; pub fn pnfs_write_done_resend_to_mds(p:*mut nfs_pgio_header)->i32;
 pub fn pnfs_mdsthreshold_alloc()->*mut nfs4_threshold; pub fn pnfs_layout_handle_reboot(p:*mut nfs_client)->i32;
 pub fn nfs4_find_get_deviceid(s:*mut nfs_server,id:*const nfs4_deviceid,c:*const cred,gfp:gfp_t)->*mut nfs4_deviceid_node;
 pub fn nfs4_delete_deviceid(ld:*const pnfs_layoutdriver_type,cl:*const nfs_client,id:*const nfs4_deviceid); pub fn nfs4_init_deviceid_node(n:*mut nfs4_deviceid_node,s:*mut nfs_server,id:*const nfs4_deviceid);
 pub fn nfs4_put_deviceid_node(n:*mut nfs4_deviceid_node)->bool; pub fn nfs4_mark_deviceid_available(n:*mut nfs4_deviceid_node); pub fn nfs4_mark_deviceid_unavailable(n:*mut nfs4_deviceid_node); pub fn nfs4_test_deviceid_unavailable(n:*mut nfs4_deviceid_node)->bool;
 pub fn nfs4_deviceid_purge_client(c:*const nfs_client); pub fn pnfs_alloc_commit_array(n:size_t,gfp:gfp_t)->*mut c_void; pub fn pnfs_free_commit_array(p:*mut c_void);
 pub fn nfs4_pnfs_ds_put(ds:*mut nfs4_pnfs_ds); pub fn nfs4_pnfs_ds_add(net:*const net,addrs:*mut list_head,version:u32,gfp:gfp_t)->*mut nfs4_pnfs_ds;
 pub fn nfs4_pnfs_v3_ds_connect_unload(); pub fn nfs4_pnfs_ds_connect(mds:*mut nfs_server,ds:*mut nfs4_pnfs_ds,devid:*mut nfs4_deviceid_node,timeo:u32,retrans:u32,version:u32,minor:u32,tightly:bool)->i32;
 pub fn nfs4_decode_mp_ds_addr(net:*mut net,xdr:*mut xdr_stream,gfp:gfp_t)->*mut nfs4_pnfs_ds_addr; pub fn pnfs_layout_mark_request_commit(req:*mut nfs_page,lseg:*mut pnfs_layout_segment,cinfo:*mut nfs_commit_info,idx:u32);
 pub fn pnfs_lgopen_prepare(data:*mut nfs4_opendata,ctx:*mut nfs_open_context); pub fn pnfs_parse_lgopen(ino:*mut inode,lgp:*mut nfs4_layoutget,ctx:*mut nfs_open_context); pub fn nfs4_lgopen_release(lgp:*mut nfs4_layoutget);
}

#[inline] pub unsafe fn pnfs_calc_offset_end(offset:u64,len:u64)->u64 { if len==u64::MAX || len>=u64::MAX-offset {u64::MAX} else {offset+len-1} }
#[inline] pub unsafe fn pnfs_calc_offset_length(offset:u64,end:u64)->u64 { if end==u64::MAX || end<=offset {u64::MAX} else {1+end-offset} }
#[inline] pub unsafe fn pnfs_end_offset(start:u64,len:u64)->u64 { if u64::MAX-start<=len {u64::MAX} else {start+len} }
#[inline] pub unsafe fn pnfs_is_range_intersecting(start1:u64,end1:u64,start2:u64,end2:u64)->bool { (end1==u64::MAX||start2<end1)&&(end2==u64::MAX||start1<end2) }
#[inline] pub unsafe fn pnfs_lseg_range_intersecting(l1:*const pnfs_layout_range,l2:*const pnfs_layout_range)->bool { let a=pnfs_end_offset((*l1).offset,(*l1).length); let b=pnfs_end_offset((*l2).offset,(*l2).length); pnfs_is_range_intersecting((*l1).offset,a,(*l2).offset,b) }

#[cfg(not(feature="CONFIG_NFS_V4_2"))] #[inline] pub unsafe fn pnfs_report_layoutstat(_inode:*mut inode,_gfp:gfp_t)->i32 {0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
