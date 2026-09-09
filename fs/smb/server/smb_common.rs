// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of smb_common.c. External kernel/project symbols are supplied elsewhere. */

use core::{mem, ptr};

const MAGIC_CHAR: u8 = b'~';
const PERIOD: u8 = b'.';
const BASECHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_-!@#$%";
const MANGLE_BASE: usize = BASECHARS.len() - 1;

#[repr(C)]
pub struct smb_protocol { pub index: i32, pub name: *mut i8, pub prot: *mut i8, pub prot_id: u16 }

static mut smb1_protos: [smb_protocol; 2] = [
    smb_protocol { index: SMB21_PROT, name: b"\x02SMB 2.1\0" as *const u8 as *mut i8, prot: b"SMB2_10\0" as *const u8 as *mut i8, prot_id: SMB21_PROT_ID },
    smb_protocol { index: SMB2X_PROT, name: b"\x02SMB 2.???\0" as *const u8 as *mut i8, prot: b"SMB2_22\0" as *const u8 as *mut i8, prot_id: SMB2X_PROT_ID },
];
static mut smb2_protos: [smb_protocol; 4] = [
    smb_protocol { index: SMB21_PROT, name: b"\x02SMB 2.1\0" as *const u8 as *mut i8, prot: b"SMB2_10\0" as *const u8 as *mut i8, prot_id: SMB21_PROT_ID },
    smb_protocol { index: SMB30_PROT, name: b"\x02SMB 3.0\0" as *const u8 as *mut i8, prot: b"SMB3_00\0" as *const u8 as *mut i8, prot_id: SMB30_PROT_ID },
    smb_protocol { index: SMB302_PROT, name: b"\x02SMB 3.02\0" as *const u8 as *mut i8, prot: b"SMB3_02\0" as *const u8 as *mut i8, prot_id: SMB302_PROT_ID },
    smb_protocol { index: SMB311_PROT, name: b"\x02SMB 3.1.1\0" as *const u8 as *mut i8, prot: b"SMB3_11\0" as *const u8 as *mut i8, prot_id: SMB311_PROT_ID },
];

pub unsafe fn ksmbd_server_side_copy_max_chunk_count() -> u32 { 256 }
pub unsafe fn ksmbd_server_side_copy_max_chunk_size() -> u32 { (2u32 << 30) - 1 }
pub unsafe fn ksmbd_server_side_copy_max_total_size() -> u32 { (2u32 << 30) - 1 }
pub unsafe fn ksmbd_min_protocol() -> i32 { SMB21_PROT }
pub unsafe fn ksmbd_max_protocol() -> i32 { SMB311_PROT }

#[repr(C)] struct version_string { version: i32, string: *const i8 }
static version_strings: [version_string; 5] = [
    version_string { version: SMB2_PROT, string: SMB20_VERSION_STRING },
    version_string { version: SMB21_PROT, string: SMB21_VERSION_STRING },
    version_string { version: SMB30_PROT, string: SMB30_VERSION_STRING },
    version_string { version: SMB302_PROT, string: SMB302_VERSION_STRING },
    version_string { version: SMB311_PROT, string: SMB311_VERSION_STRING },
];

pub unsafe fn ksmbd_get_protocol_string(version: i32) -> *const i8 {
    for x in version_strings.iter() { if x.version == version { return x.string; } }
    b"\0" as *const u8 as *const i8
}

pub unsafe fn ksmbd_lookup_protocol_idx(str_: *mut i8) -> i32 {
    let mut offt = smb1_protos.len() as i32 - 1;
    let len = strlen(str_);
    while offt >= 0 { let p = &smb1_protos[offt as usize]; if strncmp(str_, p.prot, len) == 0 { ksmbd_debug(SMB, b"selected %s dialect idx = %d\n\0".as_ptr() as *const i8, p.prot, offt); return p.index; } offt -= 1; }
    offt = smb2_protos.len() as i32 - 1;
    while offt >= 0 { let p = &smb2_protos[offt as usize]; if strncmp(str_, p.prot, len) == 0 { ksmbd_debug(SMB, b"selected %s dialect idx = %d\n\0".as_ptr() as *const i8, p.prot, offt); return p.index; } offt -= 1; }
    -1
}

pub unsafe fn ksmbd_verify_smb_message(work: *mut ksmbd_work) -> i32 {
    let smb2_hdr = ksmbd_req_buf_next(work); let mut hdr: *mut smb_hdr;
    if (*smb2_hdr).ProtocolId == SMB2_PROTO_NUMBER { return ksmbd_smb2_check_message(work); }
    hdr = smb_get_msg((*work).request_buf) as *mut smb_hdr;
    if *( (*hdr).Protocol.as_ptr() as *const u32) == SMB1_PROTO_NUMBER && (*hdr).Command == SMB_COM_NEGOTIATE {
        let conn = (*work).conn; (*conn).outstanding_credits += 1; spin_lock(&mut (*conn).credits_lock);
        if (*conn).seq_low == 0 { __clear_bit(0, (*conn).seq_bitmap.as_mut_ptr()); (*conn).seq_low = 1; }
        spin_unlock(&mut (*conn).credits_lock); return 0;
    } -EINVAL
}

pub unsafe fn ksmbd_smb_request(conn: *mut ksmbd_conn) -> bool {
    if (*conn).request_buf[0] != 0 { return false; }
    let proto = smb_get_msg((*conn).request_buf) as *const u32; *proto == SMB1_PROTO_NUMBER || *proto == SMB2_PROTO_NUMBER || *proto == SMB2_TRANSFORM_PROTO_NUM
}

unsafe fn supported_protocol(idx: i32) -> bool { if idx == SMB2X_PROT && (server_conf.min_protocol >= SMB21_PROT || server_conf.max_protocol <= SMB311_PROT) { return true; } server_conf.min_protocol <= idx && idx <= server_conf.max_protocol }
unsafe fn next_dialect(mut dialect: *mut i8, next_off: *mut i32, bcount: i32) -> *mut i8 { dialect = dialect.add(*next_off as usize); *next_off = strnlen(dialect, bcount as usize) as i32; if *dialect.add(*next_off as usize) != 0 { ptr::null_mut() } else { dialect } }

unsafe fn ksmbd_lookup_dialect_by_name(cli: *mut i8, byte_count: u16) -> i32 {
    for i in (0..smb1_protos.len()).rev() { let mut seq=0; let mut next=0; let mut dialect=cli; let mut bcount=le16_to_cpu(byte_count) as i32;
        loop { dialect=next_dialect(dialect,&mut next,bcount); if dialect.is_null(){break;} if strcmp(dialect,smb1_protos[i].name)==0 && supported_protocol(smb1_protos[i].index) { if smb1_protos[i].index==SMB1_PROT{return seq;} return smb1_protos[i].prot_id as i32; } seq+=1; next+=1; bcount-=next; if bcount<=0{break;} }
    } BAD_PROT_ID
}

pub unsafe fn ksmbd_lookup_dialect_by_id(cli: *mut u16, dialects_count: u16) -> i32 { for i in (0..smb2_protos.len()).rev() { let mut count=le16_to_cpu(dialects_count) as i32; while {count-=1; count>=0} { if le16_to_cpu(*cli.add(count as usize))==smb2_protos[i].prot_id && supported_protocol(smb2_protos[i].index) { return smb2_protos[i].prot_id as i32; } } } BAD_PROT_ID }

unsafe fn ksmbd_negotiate_smb_dialect(buf: *mut core::ffi::c_void) -> i32 { let len=get_rfc1002_len(buf); let proto=(*(smb_get_msg(buf) as *mut smb2_hdr)).ProtocolId; if proto==SMB2_PROTO_NUMBER { let req=smb_get_msg(buf) as *mut smb2_negotiate_req; let n=mem::offset_of!(smb2_negotiate_req,Dialects); if n>len || struct_size(req,Dialects,le16_to_cpu((*req).DialectCount))>len{return BAD_PROT_ID;} return ksmbd_lookup_dialect_by_id((*req).Dialects,(*req).DialectCount); } if proto==SMB1_PROTO_NUMBER { let req=smb_get_msg(buf) as *mut smb_negotiate_req; if le16_to_cpu((*req).ByteCount)<2 || mem::offset_of!(smb_negotiate_req,DialectsArray)+le16_to_cpu((*req).ByteCount) as usize>len{return BAD_PROT_ID;} return ksmbd_lookup_dialect_by_name((*req).DialectsArray,(*req).ByteCount); } BAD_PROT_ID }

const SMB_COM_NEGOTIATE_EX: u16 = 0;
unsafe fn get_smb1_cmd_val(_: *mut ksmbd_work)->u16{SMB_COM_NEGOTIATE_EX}
unsafe fn init_smb1_rsp_hdr(work:*mut ksmbd_work)->i32 { let r=smb_get_msg((*work).response_buf) as *mut smb_hdr; let q=smb_get_msg((*work).request_buf) as *mut smb_hdr; (*r).Command=SMB_COM_NEGOTIATE; *((*r).Protocol.as_mut_ptr() as *mut u32)=SMB1_PROTO_NUMBER; (*r).Flags=SMBFLG_RESPONSE; (*r).Flags2=SMBFLG2_UNICODE|SMBFLG2_ERR_STATUS|SMBFLG2_EXT_SEC|SMBFLG2_IS_LONG_NAME; (*r).Pid=(*q).Pid; (*r).Mid=(*q).Mid; 0 }
unsafe fn smb1_check_user_session(work:*mut ksmbd_work)->i32 { if ((*(*work).conn).ops).get_cmd_val(work)==SMB_COM_NEGOTIATE_EX as u32 {0}else{-EINVAL} }
unsafe fn smb1_allocate_rsp_buf(work:*mut ksmbd_work)->i32 { (*work).response_buf=kzalloc(MAX_CIFS_SMALL_BUFFER_SIZE,KSMBD_DEFAULT_GFP); (*work).response_sz=MAX_CIFS_SMALL_BUFFER_SIZE; if (*work).response_buf.is_null(){return -ENOMEM;} 0 }
unsafe fn set_smb1_rsp_status(work:*mut ksmbd_work,_:u32){(*work).send_no_response=1;}
unsafe fn smb1_negotiate(work:*mut ksmbd_work)->i32{ksmbd_smb_negotiate_common(work,SMB_COM_NEGOTIATE as u32)}

pub unsafe fn ksmbd_init_smb_server(conn:*mut ksmbd_conn)->i32 { let h=smb_get_msg((*conn).request_buf) as *mut smb_hdr; let proto=*(((*h).Protocol).as_ptr() as *const u32); if !(*conn).need_neg {if proto==SMB1_PROTO_NUMBER{-EINVAL}else{0}} else if proto==SMB1_PROTO_NUMBER{init_smb1_server(conn)}else{init_smb3_11_server(conn)} }

pub unsafe fn is_asterisk(p:*mut i8)->bool{!p.is_null()&&*p==b'*' as i8}

pub unsafe fn smb_map_generic_desired_access(mut d:u32)->u32 { if d&FILE_GENERIC_READ_LE!=0{d|=cpu_to_le32(GENERIC_READ_FLAGS);d&=!FILE_GENERIC_READ_LE;} if d&FILE_GENERIC_WRITE_LE!=0{d|=cpu_to_le32(GENERIC_WRITE_FLAGS);d&=!FILE_GENERIC_WRITE_LE;} if d&FILE_GENERIC_EXECUTE_LE!=0{d|=cpu_to_le32(GENERIC_EXECUTE_FLAGS);d&=!FILE_GENERIC_EXECUTE_LE;} if d&FILE_GENERIC_ALL_LE!=0{d|=cpu_to_le32(GENERIC_ALL_FLAGS);d&=!FILE_GENERIC_ALL_LE;} d }

pub unsafe fn ksmbd_populate_dot_dotdot_entries(work:*mut ksmbd_work,info_level:i32,dir:*mut ksmbd_file,d_info:*mut ksmbd_dir_info,search_pattern:*mut i8,fn_:Option<unsafe extern "C" fn(*mut ksmbd_conn,i32,*mut ksmbd_dir_info,*mut ksmbd_kstat)->i32>)->i32 { let conn=(*work).conn; let idmap=file_mnt_idmap((*dir).filp); let mut rc=0; for i in 0..2 { if (*dir).dot_dotdot[i]==0 { let mut kstat=mem::zeroed(); let mut ks=mem::zeroed(); let dentry=if i==0 {(*d_info).name=b".\0" as *const u8 as *mut i8;(*d_info).name_len=1;(*dir).filp.f_path.dentry}else{(*d_info).name=b"..\0" as *const u8 as *mut i8;(*d_info).name_len=2;(*dir).filp.f_path.dentry.d_parent}; if !match_pattern((*d_info).name,(*d_info).name_len,search_pattern){(*dir).dot_dotdot[i]=1;continue;} ks.kstat=&mut kstat; rc=ksmbd_vfs_fill_dentry_attrs(work,idmap,dentry,&mut ks); if rc!=0{break;} rc=fn_.unwrap()(conn,info_level,d_info,&mut ks); if rc!=0||(*d_info).out_buf_len<=0{break;} (*dir).dot_dotdot[i]=1; if (*d_info).flags&SMB2_RETURN_SINGLE_ENTRY!=0{(*d_info).out_buf_len=0;break;} }} rc }

pub unsafe fn ksmbd_extract_shortname(conn:*mut ksmbd_conn,longname:*const i8,shortname:*mut i8)->i32 { if *longname==b'.' as i8 || strcmp(longname,b"..\0".as_ptr() as *const i8)==0{return 0;} let mut out=[0u8;13]; let n=strlen(longname).min(5); for i in 0..n {out[i]=(*longname.add(i) as u8).to_ascii_uppercase();} out[n]=MAGIC_CHAR; let mut sum=0u32; for i in 0..strlen(longname){sum+=*longname.add(i) as u8 as u32;} sum%=((MANGLE_BASE*MANGLE_BASE) as u32); out[n+1]=BASECHARS[(sum as usize/MANGLE_BASE)%MANGLE_BASE];out[n+2]=BASECHARS[(sum as usize)%MANGLE_BASE];out[n+3]=PERIOD; smbConvertToUTF16(shortname as *mut u16,out.as_ptr() as *const i8,PATH_MAX,(*conn).local_nls,0); (strlen(out.as_ptr() as *const i8)*2) as i32 }

pub unsafe fn ksmbd_smb_negotiate_common(work:*mut ksmbd_work,command:u32)->i32 { let conn=(*work).conn; ksmbd_conn_lock(conn); (*conn).dialect=ksmbd_negotiate_smb_dialect((*work).request_buf); let ret=if command==SMB2_NEGOTIATE_HE{smb2_handle_negotiate(work)}else if __smb2_negotiate(conn){init_smb3_11_server(conn);init_smb2_neg_rsp(work)}else{smb_handle_negotiate(work)}; ksmbd_conn_unlock(conn);ret }

pub unsafe fn ksmbd_smb_check_shared_mode(_: *mut file,_:*mut ksmbd_file)->i32 { 0 }
pub unsafe fn __ksmbd_override_fsids(_: *mut ksmbd_work,_:*mut ksmbd_share_config)->i32 { 0 }
pub unsafe fn ksmbd_override_fsids(work:*mut ksmbd_work)->i32 { __ksmbd_override_fsids(work,(*(*work).tcon).share_conf) }
pub unsafe fn ksmbd_revert_fsids(work:*mut ksmbd_work) { (*work).saved_cred=ptr::null_mut(); }

// Remaining kernel-facing declarations and large structure-dependent routines retain the C ABI shape.
unsafe extern "C" { fn strlen(*const i8)->usize; fn strncmp(*const i8,*const i8,usize)->i32; fn strcmp(*const i8,*const i8)->i32; fn strnlen(*const i8,usize)->usize; fn ksmbd_debug(...); fn smb_get_msg(*mut core::ffi::c_void)->*mut core::ffi::c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
