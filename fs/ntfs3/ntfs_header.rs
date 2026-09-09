// SPDX-License-Identifier: GPL-2.0
// On-disk ntfs structs. Literal translation of ntfs.h.

// Linux dependency types and helpers are supplied by other translated files.
type u8 = std::primitive::u8;
type u16 = std::primitive::u16;
type u32 = std::primitive::u32;
type u64 = std::primitive::u64;
type s8 = std::primitive::i8;
type le16 = u16;
type le32 = u32;
type le64 = u64;
type CLST = u32;

pub const NTFS_NAME_LEN: usize = 255;
pub const NTFS_LINK_MAX: usize = 4000;
pub const NTFS_LZNT_MAX_CLUSTER: u32 = 4096;
pub const NTFS_LZNT_CUNIT: u32 = 4;
pub const NTFS_LZNT_CLUSTERS: u32 = 1u32 << NTFS_LZNT_CUNIT;
pub const SPARSE_LCN64: u64 = u64::MAX;
pub const SPARSE_LCN: CLST = CLST::MAX;
pub const RESIDENT_LCN: CLST = CLST::MAX - 1;
pub const COMPRESSED_LCN: CLST = CLST::MAX - 2;
pub const EOF_LCN: CLST = CLST::MAX - 3;
pub const DELALLOC_LCN: CLST = CLST::MAX - 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GUID { pub Data1: le32, pub Data2: le16, pub Data3: le16, pub Data4: [u8; 8] }
#[repr(C)]
pub struct cpu_str { pub len: u8, pub ads_len: u8, pub name: [u16; 0] }
#[repr(C)]
pub struct le_str { pub len: u8, pub unused: u8, pub name: [le16; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MFT_REF { pub low: le32, pub high: le16, pub seq: le16 }

#[repr(C)]
pub struct NTFS_BOOT {
    pub jump_code: [u8; 3], pub system_id: [u8; 8], pub bytes_per_sector: [u8; 2],
    pub sectors_per_clusters: u8, pub unused1: [u8; 7], pub media_type: u8,
    pub unused2: [u8; 2], pub sct_per_track: le16, pub heads: le16,
    pub hidden_sectors: le32, pub unused3: [u8; 4], pub bios_drive_num: u8,
    pub unused4: u8, pub signature_ex: u8, pub unused5: u8,
    pub sectors_per_volume: le64, pub mft_clst: le64, pub mft2_clst: le64,
    pub record_size: s8, pub unused6: [u8; 3], pub index_size: s8,
    pub unused7: [u8; 3], pub serial_num: le64, pub check_sum: le32,
    pub boot_code: [u8; 0x200 - 0x50 - 2 - 4], pub boot_magic: [u8; 2],
}

#[repr(u32)]
pub enum RECORD_NUM { MFT_REC_MFT=0, MFT_REC_MIRR=1, MFT_REC_LOG=2, MFT_REC_VOL=3, MFT_REC_ATTR=4, MFT_REC_ROOT=5, MFT_REC_BITMAP=6, MFT_REC_BOOT=7, MFT_REC_BADCLUST=8, MFT_REC_SECURE=9, MFT_REC_UPCASE=10, MFT_REC_EXTEND=11, MFT_REC_RESERVED=12, MFT_REC_FREE=16, MFT_REC_USER=24 }
#[repr(u32)]
pub enum ATTR_TYPE { ATTR_ZERO=0x00, ATTR_STD=0x10, ATTR_LIST=0x20, ATTR_NAME=0x30, ATTR_ID=0x40, ATTR_SECURE=0x50, ATTR_LABEL=0x60, ATTR_VOL_INFO=0x70, ATTR_DATA=0x80, ATTR_ROOT=0x90, ATTR_ALLOC=0xA0, ATTR_BITMAP=0xB0, ATTR_REPARSE=0xC0, ATTR_EA_INFO=0xD0, ATTR_EA=0xE0, ATTR_PROPERTYSET=0xF0, ATTR_LOGGED_UTILITY_STREAM=0x100, ATTR_END=0xffff_ffff }
#[repr(u32)]
pub enum FILE_ATTRIBUTE { FILE_ATTRIBUTE_READONLY=1, FILE_ATTRIBUTE_HIDDEN=2, FILE_ATTRIBUTE_SYSTEM=4, FILE_ATTRIBUTE_ARCHIVE=0x20, FILE_ATTRIBUTE_DEVICE=0x40, FILE_ATTRIBUTE_TEMPORARY=0x100, FILE_ATTRIBUTE_SPARSE_FILE=0x200, FILE_ATTRIBUTE_REPARSE_POINT=0x400, FILE_ATTRIBUTE_COMPRESSED=0x800, FILE_ATTRIBUTE_OFFLINE=0x1000, FILE_ATTRIBUTE_NOT_CONTENT_INDEXED=0x2000, FILE_ATTRIBUTE_ENCRYPTED=0x4000, FILE_ATTRIBUTE_VALID_FLAGS=0x7fb7, FILE_ATTRIBUTE_DIRECTORY=0x1000_0000, FILE_ATTRIBUTE_INDEX=0x2000_0000 }

extern "C" {
    pub static NAME_MFT: cpu_str; pub static NAME_MIRROR: cpu_str; pub static NAME_LOGFILE: cpu_str; pub static NAME_VOLUME: cpu_str; pub static NAME_ATTRDEF: cpu_str; pub static NAME_ROOT: cpu_str; pub static NAME_BITMAP: cpu_str; pub static NAME_BOOT: cpu_str; pub static NAME_BADCLUS: cpu_str; pub static NAME_QUOTA: cpu_str; pub static NAME_SECURE: cpu_str; pub static NAME_UPCASE: cpu_str; pub static NAME_EXTEND: cpu_str; pub static NAME_OBJID: cpu_str; pub static NAME_REPARSE: cpu_str; pub static NAME_USNJRNL: cpu_str;
    pub static I30_NAME: [le16; 4]; pub static SII_NAME: [le16; 4]; pub static SDH_NAME: [le16; 4]; pub static SO_NAME: [le16; 2]; pub static SQ_NAME: [le16; 2]; pub static SR_NAME: [le16; 2]; pub static QUERY_STREAMS: [le16; 13]; pub static BAD_NAME: [le16; 4]; pub static SDS_NAME: [le16; 4]; pub static WOF_NAME: [le16; 17];
}

#[inline] pub unsafe fn ino_get(ref_: *const MFT_REF) -> CLST { u32::from_le((*ref_).low) }

#[repr(u32)] pub enum NTFS_SIGNATURE { NTFS_FILE_SIGNATURE=0x454c4946, NTFS_INDX_SIGNATURE=0x58444e49, NTFS_CHKD_SIGNATURE=0x444b4843, NTFS_RSTR_SIGNATURE=0x52545352, NTFS_RCRD_SIGNATURE=0x44524352, NTFS_BAAD_SIGNATURE=0x44414142, NTFS_HOLE_SIGNATURE=0x454c4f48, NTFS_FFFF_SIGNATURE=0xffff_ffff }
#[repr(C)] pub struct NTFS_RECORD_HEADER { pub sign: NTFS_SIGNATURE, pub fix_off: le16, pub fix_num: le16, pub lsn: le64 }
#[inline] pub unsafe fn is_baad(hdr: *const NTFS_RECORD_HEADER) -> bool { (*hdr).sign as u32 == NTFS_SIGNATURE::NTFS_BAAD_SIGNATURE as u32 }
#[repr(u16)] pub enum RECORD_FLAG { RECORD_FLAG_IN_USE=1, RECORD_FLAG_DIR=2, RECORD_FLAG_SYSTEM=4, RECORD_FLAG_INDEX=8 }
#[repr(C)] pub struct MFT_REC { pub rhdr: NTFS_RECORD_HEADER, pub seq: le16, pub hard_links: le16, pub attr_off: le16, pub flags: le16, pub used: le32, pub total: le32, pub parent_ref: MFT_REF, pub next_attr_id: le16, pub res: le16, pub mft_record: le32, pub fixups: [le16; 0] }
pub const MFTRECORD_FIXUP_OFFSET_1: usize = 0x2a;
pub const MFTRECORD_FIXUP_OFFSET_3: usize = 0x30;
pub const MFTRECORD_FIXUP_OFFSET: usize = MFTRECORD_FIXUP_OFFSET_1;
#[inline] pub unsafe fn is_rec_base(rec: *const MFT_REC) -> bool { (*rec).parent_ref.low == 0 && (*rec).parent_ref.high == 0 && (*rec).parent_ref.seq == 0 }
#[inline] pub unsafe fn is_mft_rec5(rec: *const MFT_REC) -> bool { u16::from_le((*rec).rhdr.fix_off) >= MFTRECORD_FIXUP_OFFSET_3 as u16 }
#[inline] pub unsafe fn is_rec_inuse(rec: *const MFT_REC) -> bool { (*rec).flags & RECORD_FLAG::RECORD_FLAG_IN_USE as u16 != 0 }
#[inline] pub unsafe fn clear_rec_inuse(rec: *mut MFT_REC) -> bool { (*rec).flags &= !(RECORD_FLAG::RECORD_FLAG_IN_USE as u16); (*rec).flags != 0 }

#[repr(C)] pub struct ATTR_RESIDENT { pub data_size: le32, pub data_off: le16, pub flags: u8, pub res: u8 }
#[repr(C)] pub struct ATTR_NONRESIDENT { pub svcn: le64, pub evcn: le64, pub run_off: le16, pub c_unit: u8, pub res1: [u8; 5], pub alloc_size: le64, pub data_size: le64, pub valid_size: le64, pub total_size: le64 }
#[repr(C)] pub union ATTRIB_DATA { pub res: ATTR_RESIDENT, pub nres: ATTR_NONRESIDENT }
#[repr(C)] pub struct ATTRIB { pub type_: ATTR_TYPE, pub size: le32, pub non_res: u8, pub name_len: u8, pub name_off: le16, pub flags: le16, pub id: le16, pub data: ATTRIB_DATA }
pub const RESIDENT_FLAG_INDEXED: u8 = 1;
pub const ATTR_FLAG_COMPRESSED: le16 = 1; pub const ATTR_FLAG_COMPRESSED_MASK: le16 = 0xff; pub const ATTR_FLAG_ENCRYPTED: le16 = 0x4000; pub const ATTR_FLAG_SPARSED: le16 = 0x8000;
pub const SIZEOF_RESIDENT: usize=0x18; pub const SIZEOF_NONRESIDENT_EX: usize=0x48; pub const SIZEOF_NONRESIDENT: usize=0x40;
pub const SIZEOF_RESIDENT_LE: le16=0x18; pub const SIZEOF_NONRESIDENT_EX_LE: le16=0x48; pub const SIZEOF_NONRESIDENT_LE: le16=0x40;

#[repr(C)] pub struct ATTR_STD_INFO { pub cr_time:le64,pub m_time:le64,pub c_time:le64,pub a_time:le64,pub fa:FILE_ATTRIBUTE,pub max_ver_num:le32,pub ver_num:le32,pub class_id:le32 }
#[repr(C)] pub struct ATTR_STD_INFO5 { pub base:ATTR_STD_INFO,pub owner_id:le32,pub security_id:le32,pub quota_charge:le64,pub usn:le64 }
pub const SECURITY_ID_INVALID:u32=0; pub const SECURITY_ID_FIRST:u32=0x100;
#[repr(C)] pub struct ATTR_LIST_ENTRY { pub type_:ATTR_TYPE,pub size:le16,pub name_len:u8,pub name_off:u8,pub vcn:le64,pub ref_:MFT_REF,pub id:le16,pub name:[le16;0] }
#[repr(C)] pub struct NTFS_DUP_INFO { pub cr_time:le64,pub m_time:le64,pub c_time:le64,pub a_time:le64,pub alloc_size:le64,pub data_size:le64,pub fa:FILE_ATTRIBUTE,pub extend_data:le32 }
#[repr(C)] pub struct ATTR_FILE_NAME { pub home:MFT_REF,pub dup:NTFS_DUP_INFO,pub name_len:u8,pub type_:u8,pub name:[le16;0] }
pub const FILE_NAME_POSIX:u8=0; pub const FILE_NAME_UNICODE:u8=1; pub const FILE_NAME_DOS:u8=2; pub const FILE_NAME_UNICODE_AND_DOS:u8=3;
pub const SIZEOF_ATTRIBUTE_FILENAME:usize=0x44; pub const SIZEOF_ATTRIBUTE_FILENAME_MAX:usize=0x42+255*2;

#[repr(C)] pub union NTFS_DE_HEAD { pub ref_:MFT_REF, pub view: NTFS_DE_VIEW }
#[repr(C)] pub struct NTFS_DE_VIEW { pub data_off:le16,pub data_size:le16,pub res:le32 }
#[repr(C)] pub struct NTFS_DE { pub head:NTFS_DE_HEAD,pub size:le16,pub key_size:le16,pub flags:le16,pub res:le16 }
pub const NTFS_IE_HAS_SUBNODES:le16=1; pub const NTFS_IE_LAST:le16=2;
#[repr(C)] pub struct INDEX_HDR { pub de_off:le32,pub used:le32,pub total:le32,pub flags:le32 }
pub const NTFS_INDEX_HDR_HAS_SUBNODES:le32=1;
#[repr(C)] pub struct INDEX_BUFFER { pub rhdr:NTFS_RECORD_HEADER,pub vbn:le64,pub ihdr:INDEX_HDR }
#[repr(u32)] pub enum COLLATION_RULE { NTFS_COLLATION_TYPE_BINARY=0, NTFS_COLLATION_TYPE_FILENAME=1, NTFS_COLLATION_TYPE_UINT=0x10, NTFS_COLLATION_TYPE_SID=0x11, NTFS_COLLATION_TYPE_SECURITY_HASH=0x12, NTFS_COLLATION_TYPE_UINTS=0x13 }
#[repr(C)] pub struct INDEX_ROOT { pub type_:ATTR_TYPE,pub rule:COLLATION_RULE,pub index_block_size:le32,pub index_block_clst:u8,pub res:[u8;3],pub ihdr:INDEX_HDR }
#[repr(C)] pub struct VOLUME_INFO { pub res1:le64,pub major_ver:u8,pub minor_ver:u8,pub flags:le16 }
pub const VOLUME_FLAG_DIRTY:le16=1; pub const VOLUME_FLAG_RESIZE_LOG_FILE:le16=2; pub const SIZEOF_ATTRIBUTE_VOLUME_INFO:usize=0xc;
pub const NTFS_LABEL_MAX_LENGTH:usize=0x100/2; pub const NTFS_ATTR_INDEXABLE:le32=2; pub const NTFS_ATTR_DUPALLOWED:le32=4; pub const NTFS_ATTR_MUST_BE_INDEXED:le32=0x10; pub const NTFS_ATTR_MUST_BE_NAMED:le32=0x20; pub const NTFS_ATTR_MUST_BE_RESIDENT:le32=0x40; pub const NTFS_ATTR_LOG_ALWAYS:le32=0x80;
#[repr(C)] pub struct ATTR_DEF_ENTRY { pub name:[le16;0x40],pub type_:ATTR_TYPE,pub res:le32,pub rule:COLLATION_RULE,pub flags:le32,pub min_sz:le64,pub max_sz:le64 }
#[repr(C)] pub struct OBJECT_ID { pub ObjId:GUID,pub BirthVolumeId:GUID,pub BirthObjectId:GUID,pub DomainId:GUID }
#[repr(C)] pub struct NTFS_DE_O { pub de:NTFS_DE,pub ObjId:GUID,pub ref_:MFT_REF,pub BirthVolumeId:GUID,pub BirthObjectId:GUID,pub BirthDomainId:GUID }
#[repr(C,packed)] pub struct NTFS_DE_Q { pub de:NTFS_DE,pub owner_id:le32,pub Version:le32,pub Flags:le32,pub BytesUsed:le64,pub ChangeTime:le64,pub WarningLimit:le64,pub HardLimit:le64,pub ExceededTime:le64 }
pub const SecurityDescriptorsBlockSize:u32=0x40000; pub const SecurityDescriptorMaxSize:u32=0x20000; pub const Log2OfSecurityDescriptorsBlockSize:u32=18;
#[repr(C)] pub struct SECURITY_KEY { pub hash:le32,pub sec_id:le32 }
#[repr(C,packed)] pub struct SECURITY_HDR { pub key:SECURITY_KEY,pub off:le64,pub size:le32 }
#[repr(C,packed)] pub struct NTFS_DE_SII { pub de:NTFS_DE,pub sec_id:le32,pub sec_hdr:SECURITY_HDR }
#[repr(C)] pub struct NTFS_DE_SDH { pub de:NTFS_DE,pub key:SECURITY_KEY,pub sec_hdr:SECURITY_HDR,pub magic:[le16;2] }
pub const SIZEOF_SDH_DIRENTRY:usize=0x30;
#[repr(C)] pub struct REPARSE_KEY { pub ReparseTag:le32,pub ref_:MFT_REF }
pub const SIZEOF_REPARSE_KEY:usize=0x0c;

#[inline] pub unsafe fn attr_from_name(fname:*mut ATTR_FILE_NAME)->*mut ATTRIB { (fname as *mut u8).sub(SIZEOF_RESIDENT) as *mut ATTRIB }
#[inline] pub unsafe fn fname_full_size(fname:*const ATTR_FILE_NAME)->u16 { (0x42 + (*fname).name_len as usize * 2) as u16 }
#[inline] pub fn paired_name(t:u8)->u8 { if t==FILE_NAME_UNICODE {FILE_NAME_DOS} else if t==FILE_NAME_DOS {FILE_NAME_UNICODE} else {FILE_NAME_POSIX} }
#[inline] pub unsafe fn le_size(n:u8)->u32 { (((0x1a + n as usize*2)+7)&!7) as u32 }
#[inline] pub unsafe fn de_set_vbn_le(e:*mut NTFS_DE,v:le64) { *((e as *mut u8).add(u16::from_le((*e).size) as usize-8) as *mut le64)=v }
#[inline] pub unsafe fn de_set_vbn(e:*mut NTFS_DE,v:CLST) { de_set_vbn_le(e,v as le64) }
#[inline] pub unsafe fn de_get_vbn_le(e:*const NTFS_DE)->le64 { *((e as *const u8).add(u16::from_le((*e).size) as usize-8) as *const le64) }
#[inline] pub unsafe fn de_get_vbn(e:*const NTFS_DE)->CLST { u64::from_le(de_get_vbn_le(e)) as CLST }
#[inline] pub unsafe fn de_get_next(e:*const NTFS_DE)->*mut NTFS_DE { (e as *const u8).add(u16::from_le((*e).size) as usize) as *mut NTFS_DE }
#[inline] pub unsafe fn de_get_fname(e:*const NTFS_DE)->*mut ATTR_FILE_NAME { if u16::from_le((*e).key_size)>=SIZEOF_ATTRIBUTE_FILENAME as u16 {(e as *const u8).add(0x10) as *mut ATTR_FILE_NAME} else {std::ptr::null_mut()} }
#[inline] pub unsafe fn de_is_last(e:*const NTFS_DE)->bool { (*e).flags & NTFS_IE_LAST != 0 }
#[inline] pub unsafe fn de_has_vcn(e:*const NTFS_DE)->bool { (*e).flags & NTFS_IE_HAS_SUBNODES != 0 }
#[inline] pub unsafe fn de_has_vcn_ex(e:*const NTFS_DE)->bool { de_has_vcn(e) && de_get_vbn_le(e)!=u64::MAX }
#[inline] pub unsafe fn hdr_first_de(h:*const INDEX_HDR)->*mut NTFS_DE { let o=u32::from_le((*h).de_off) as usize; let u=u32::from_le((*h).used) as usize; if o>=u || o+0x10>u{return std::ptr::null_mut()} let e=(h as *const u8).add(o) as *mut NTFS_DE; let s=u16::from_le((*e).size) as usize; if s<0x10||o+s>u{std::ptr::null_mut()}else{e} }
#[inline] pub unsafe fn hdr_next_de(h:*const INDEX_HDR,e:*const NTFS_DE)->*mut NTFS_DE { let o=e as usize-h as usize; let u=u32::from_le((*h).used) as usize; if o>=u{return std::ptr::null_mut()} let s=u16::from_le((*e).size) as usize; if s<0x10||o+s+0x10>u{return std::ptr::null_mut()} (e as *const u8).add(s) as *mut NTFS_DE }
#[inline] pub unsafe fn hdr_has_subnode(h:*const INDEX_HDR)->bool { (*h).flags & NTFS_INDEX_HDR_HAS_SUBNODES != 0 }
#[inline] pub unsafe fn ib_is_empty(ib:*const INDEX_BUFFER)->bool { let e=hdr_first_de(&(*ib).ihdr); e.is_null()||de_is_last(e) }
#[inline] pub unsafe fn ib_is_leaf(ib:*const INDEX_BUFFER)->bool { (*ib).ihdr.flags & NTFS_INDEX_HDR_HAS_SUBNODES == 0 }
#[inline] pub unsafe fn attr_size(a:*const ATTRIB)->u64 { if (*a).non_res!=0 {u64::from_le((*a).data.nres.data_size)} else {u32::from_le((*a).data.res.data_size) as u64} }
#[inline] pub unsafe fn attr_svcn(a:*const ATTRIB)->u64 { if (*a).non_res!=0 {u64::from_le((*a).data.nres.svcn)} else {0} }
#[inline] pub unsafe fn is_attr_encrypted(a:*const ATTRIB)->bool { (*a).flags&ATTR_FLAG_ENCRYPTED!=0 }
#[inline] pub unsafe fn is_attr_sparsed(a:*const ATTRIB)->bool { (*a).flags&ATTR_FLAG_SPARSED!=0 }
#[inline] pub unsafe fn is_attr_compressed(a:*const ATTRIB)->bool { (*a).flags&ATTR_FLAG_COMPRESSED!=0 }
#[inline] pub unsafe fn is_attr_ext(a:*const ATTRIB)->bool { (*a).flags&(ATTR_FLAG_SPARSED|ATTR_FLAG_COMPRESSED)!=0 }
#[inline] pub unsafe fn is_attr_indexed(a:*const ATTRIB)->bool { (*a).non_res==0 && (*a).data.res.flags&RESIDENT_FLAG_INDEXED!=0 }
#[inline] pub unsafe fn resident_data(a:*const ATTRIB)->*mut u8 { (a as *const u8).add(u16::from_le((*a).data.res.data_off) as usize) as *mut u8 }
#[inline] pub unsafe fn attr_run(a:*const ATTRIB)->*mut u8 { (a as *const u8).add(u16::from_le((*a).data.nres.run_off) as usize) as *mut u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
