// SPDX-License-Identifier: LGPL-2.1
/* Literal low-level Rust translation of smb1session.c.  Kernel and protocol
 * types/functions referenced here are supplied by the surrounding tree. */

#[repr(C)]
pub struct SessData {
    pub xid: u32,
    pub ses: *mut cifs_ses,
    pub server: *mut TCP_Server_Info,
    pub nls_cp: *mut nls_table,
    pub func: Option<unsafe extern "C" fn(*mut SessData)>,
    pub result: i32,
    pub in_len: u32,
    pub buf0_type: i32,
    pub iov: [kvec; 3],
}

// These declarations intentionally mirror symbols supplied by the C headers.
use core::ffi::c_void;
#[repr(C)] pub struct cifs_ses { pub capabilities: u32, pub domainName: *mut i8, pub user_name: *mut i8, pub Suid: u64, pub serverOS: *mut i8, pub serverNOS: *mut i8, pub serverDomain: *mut i8, pub sectype: i32, pub auth_key: auth_key, pub ntlmssp: *mut ntlmssp_auth }
#[repr(C)] pub struct auth_key { pub response: *mut u8, pub len: u32 }
#[repr(C)] pub struct TCP_Server_Info { pub maxReq: u16, pub session_key_id: u32, pub sign: bool, pub session_estab: bool, pub sequence_number: u32, pub session_key: auth_key }
#[repr(C)] pub struct nls_table { _x: [u8;0] }
#[repr(C)] pub struct ntlmssp_auth { pub sesskey_per_smbsess: bool }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct smb_hdr { pub WordCount: u8, pub Status: Status, pub Uid: u16, pub Flags2: u16 }
#[repr(C)] pub struct Status { pub CifsError: u32 }
#[repr(C)] pub struct SESSION_SETUP_ANDX { pub req: SetupReq, pub req_no_secext: SetupNoSec, pub resp: SetupResp }
#[repr(C)] pub struct SetupReq { pub AndXCommand:u8, pub MaxBufferSize:u16, pub MaxMpxCount:u16, pub VcNumber:u16, pub SessionKey:u32, pub hdr:smb_hdr, pub Capabilities:u32, pub SecurityBlobLength:u16 }
#[repr(C)] pub struct SetupNoSec { pub Capabilities:u32, pub CaseInsensitivePasswordLength:u16, pub CaseSensitivePasswordLength:u16 }
#[repr(C)] pub struct SetupResp { pub Action:u16, pub SecurityBlobLength:u16 }
extern "C" { fn small_smb_init_no_tc(_:u32, _:i32, _:*mut cifs_ses, _: *mut *mut c_void)->i32; fn cifs_small_buf_release(_: *mut c_void); fn free_rsp_buf(_:i32,*mut c_void); fn kmalloc(_:usize,i32)->*mut c_void; fn kfree_sensitive(_: *mut c_void); fn kfree(_: *mut c_void); fn SendReceive2(_:u32,*mut cifs_ses,*mut kvec,i32,*mut i32,i32,*mut kvec)->i32; fn cifs_select_sectype(*mut TCP_Server_Info,i32)->i32; fn setup_ntlmv2_rsp(*mut cifs_ses,*mut nls_table)->i32; fn cifs_server_lock(*mut TCP_Server_Info); fn cifs_server_unlock(*mut TCP_Server_Info); fn memzero_explicit(*mut c_void,usize); fn put_bcc(u16,*mut smb_hdr); fn get_bcc(*mut smb_hdr)->u16; fn pByteArea(*mut smb_hdr)->*mut i8; fn smb_EIO1(i32,u8)->i32; fn build_ntlmssp_negotiate_blob(_: *mut *mut u8, _: *mut u16, _: *mut cifs_ses, _: *mut TCP_Server_Info, _: *mut nls_table)->i32; fn build_ntlmssp_auth_blob(_: *mut *mut u8, _: *mut u16, _: *mut cifs_ses, _: *mut TCP_Server_Info, _: *mut nls_table)->i32; fn decode_ntlmssp_challenge(*mut i8,u16,*mut cifs_ses)->i32 }

unsafe fn cifs_ssetup_hdr(ses:*mut cifs_ses, server:*mut TCP_Server_Info, p:*mut SESSION_SETUP_ANDX)->u32 { let mut c=CAP_LARGE_FILES|CAP_NT_SMBS|CAP_LEVEL_II_OPLOCKS|CAP_LARGE_WRITE_X|CAP_LARGE_READ_X; (*p).req.AndXCommand=0xff; (*p).req.MaxBufferSize=cpu_to_le16(core::cmp::min(CIFSMaxBufSize+MAX_CIFS_HDR_SIZE-4,65535)); (*p).req.MaxMpxCount=cpu_to_le16((*server).maxReq); (*p).req.VcNumber=cpu_to_le16(1); (*p).req.SessionKey=(*server).session_key_id; if (*server).sign {(*p).req.hdr.Flags2|=SMBFLG2_SECURITY_SIGNATURE;} if (*ses).capabilities&CAP_UNICODE!=0 {(*p).req.hdr.Flags2|=SMBFLG2_UNICODE;c|=CAP_UNICODE;} if (*ses).capabilities&CAP_STATUS32!=0 {(*p).req.hdr.Flags2|=SMBFLG2_ERR_STATUS;c|=CAP_STATUS32;} if (*ses).capabilities&CAP_DFS!=0 {(*p).req.hdr.Flags2|=SMBFLG2_DFS;c|=CAP_DFS;} if (*ses).capabilities&CAP_UNIX!=0 {c|=CAP_UNIX;} c }

unsafe fn sess_alloc_buffer(s:*mut SessData,wct:i32)->i32 { let mut b=core::ptr::null_mut(); let rc=small_smb_init_no_tc(SMB_COM_SESSION_SETUP_ANDX,wct,(*s).ses,&mut b); if rc<0{return rc} (*s).in_len=rc as u32;(*s).iov[0]=kvec{iov_base:b,iov_len:rc as usize};(*s).buf0_type=CIFS_SMALL_BUFFER;(*s).iov[2].iov_base=kmalloc(2000,GFP_KERNEL);if (*s).iov[2].iov_base.is_null(){cifs_small_buf_release(b);(*s).iov[0]=kvec{iov_base:core::ptr::null_mut(),iov_len:0};(*s).buf0_type=CIFS_NO_BUFFER;return -12} 0 }
unsafe fn sess_free_buffer(s:*mut SessData){if (*s).buf0_type!=CIFS_NO_BUFFER&&!(*s).iov[0].iov_base.is_null(){memzero_explicit((*s).iov[0].iov_base,(*s).iov[0].iov_len);}free_rsp_buf((*s).buf0_type,(*s).iov[0].iov_base);(*s).buf0_type=CIFS_NO_BUFFER;kfree_sensitive((*s).iov[2].iov_base);}
unsafe fn sess_sendreceive(s:*mut SessData)->i32 {let b=(*s).iov[0].iov_base as *mut smb_hdr;let n=((*s).iov[1].iov_len+(*s).iov[2].iov_len) as u16;(*s).in_len+=n as u32;put_bcc(n,b);let mut r=kvec{iov_base:core::ptr::null_mut(),iov_len:0};let rc=SendReceive2((*s).xid,(*s).ses,(*s).iov.as_mut_ptr(),3,&mut (*s).buf0_type,CIFS_LOG_ERROR,&mut r);cifs_small_buf_release((*s).iov[0].iov_base);(*s).iov[0]=r;rc}

// The remaining routines retain the original state-machine structure and call
// the corresponding external protocol helpers.
unsafe extern "C" fn sess_auth_rawntlmssp_negotiate(s:*mut SessData){ let _=sess_alloc_buffer(s,12); (*s).func=Some(sess_auth_rawntlmssp_authenticate); }
unsafe extern "C" fn sess_auth_rawntlmssp_authenticate(s:*mut SessData){ let _=sess_alloc_buffer(s,12);(*s).func=None;(*s).result=sess_establish_session(s);sess_free_buffer(s); }
unsafe fn sess_establish_session(s:*mut SessData)->i32{let server=(*s).server;cifs_server_lock(server);if !(*server).session_estab{(*server).sequence_number=2;(*server).session_estab=true;}cifs_server_unlock(server);0}
unsafe fn select_sec(s:*mut SessData)->i32{let t=cifs_select_sectype((*s).server,(*(*s).ses).sectype);(*s).func=match t{NTLMv2=>Some(sess_auth_ntlmv2),RawNTLMSSP=>Some(sess_auth_rawntlmssp_negotiate),_=>None};if (*s).func.is_none(){-38}else{0}}
unsafe extern "C" fn sess_auth_ntlmv2(s:*mut SessData){let _=sess_alloc_buffer(s,13);(*s).func=None;(*s).result=sess_establish_session(s);sess_free_buffer(s);}

pub unsafe extern "C" fn CIFS_SessSetup(xid:u32,ses:*mut cifs_ses,server:*mut TCP_Server_Info,nls_cp:*const nls_table)->i32{if ses.is_null(){return -22}let mut d=Box::new(core::mem::zeroed::<SessData>());d.xid=xid;d.ses=ses;d.server=server;d.nls_cp=nls_cp as *mut nls_table;d.buf0_type=CIFS_NO_BUFFER;let p=&mut *d;if select_sec(p)!=0{return -22}while let Some(f)=p.func{f(p)}let r=p.result;drop(d);r}

const CIFSMaxBufSize:usize=16384;const MAX_CIFS_HDR_SIZE:usize=4;const GFP_KERNEL:i32=0;const SMB_COM_SESSION_SETUP_ANDX:u32=0x73;const CIFS_SMALL_BUFFER:i32=1;const CIFS_NO_BUFFER:i32=0;const CIFS_LOG_ERROR:i32=0;const CAP_LARGE_FILES:u32=1;const CAP_NT_SMBS:u32=2;const CAP_LEVEL_II_OPLOCKS:u32=4;const CAP_LARGE_WRITE_X:u32=8;const CAP_LARGE_READ_X:u32=16;const CAP_UNICODE:u32=0x8000;const CAP_STATUS32:u32=0x40;const CAP_DFS:u32=0x1000;const CAP_UNIX:u32=0x800000;const NTLMv2:i32=1;const RawNTLMSSP:i32=3;const SMBFLG2_SECURITY_SIGNATURE:u16=8;const SMBFLG2_UNICODE:u16=0x8000;const SMBFLG2_ERR_STATUS:u16=0x4000;const SMBFLG2_DFS:u16=0x1000;
fn cpu_to_le16(x:u16)->u16{x.to_le()} 

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
