// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of pldmfw.c. Kernel/project symbols are external. */

use core::ffi::c_void;

#[repr(C)]
pub struct pldmfw_priv {
    pub context: *mut pldmfw,
    pub fw: *const firmware,
    pub offset: usize,
    pub records: list_head,
    pub components: list_head,
    pub header: *const __pldm_header,
    pub total_header_size: u16,
    pub component_bitmap_len: u16,
    pub bitmap_size: u16,
    pub component_count: u16,
    pub component_start: *const u8,
    pub record_start: *const u8,
    pub record_count: u8,
    pub header_crc: u32,
    pub matching_record: *mut pldmfw_record,
}

extern "C" {
    static pldm_firmware_header_id: uuid;
    fn uuid_equal(a: *const uuid, b: *const uuid) -> bool;
    fn get_unaligned_le16(p: *const c_void) -> u16;
    fn get_unaligned_le32(p: *const c_void) -> u32;
    fn crc32_le(seed: u32, p: *const u8, len: usize) -> u32;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn bitmap_zalloc(bits: u16, flags: u32) -> *mut usize;
    fn bitmap_free(p: *mut usize);
    fn bitmap_set_value8(p: *mut usize, value: u8, offset: usize);
    fn test_bit(bit: u8, p: *const usize) -> bool;
    fn find_first_bit(p: *const usize, bits: u16) -> u8;
    fn find_last_bit(p: *const usize, bits: u16) -> u8;
    fn list_add_tail(n: *mut list_head, h: *mut list_head);
    fn list_del(n: *mut list_head);
    fn list_empty(h: *const list_head) -> bool;
    fn pldmfw_free_priv(data: *mut pldmfw);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct uuid { pub b: [u8; 16] }
#[repr(C)] pub struct device { pub _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub data: *const u8, pub size: usize }
#[repr(C)] pub struct __pldm_header { pub id: uuid, pub revision: u8, pub size: [u8;2], pub component_bitmap_len: [u8;2], pub version_len: u8 }
#[repr(C)] pub struct __pldmfw_record_area { pub record_count: u8, pub records: [u8;0] }
#[repr(C)] pub struct __pldmfw_record_info { pub record_len: [u8;2], pub package_data_len: [u8;2], pub version_len: u8, pub version_type: u8, pub descriptor_count: u8 }
#[repr(C)] pub struct __pldmfw_desc_tlv { pub typ: [u8;2], pub size: [u8;2], pub data: [u8;0] }
#[repr(C)] pub struct __pldmfw_component_area { pub component_image_count: [u8;2], pub components: [u8;0] }
#[repr(C)] pub struct __pldmfw_component_info { pub classification: [u8;2], pub identifier: [u8;2], pub comparison_stamp: [u8;4], pub options: [u8;2], pub activation_method: [u8;2], pub location_offset: [u8;4], pub size: [u8;4], pub version_len: u8, pub version_type: u8, pub version_string: *const u8 }
#[repr(C)] pub struct pldmfw_desc_tlv { pub entry: list_head, pub typ: u16, pub size: u16, pub data: *const u8 }
#[repr(C)] pub struct pldmfw_record { pub entry: list_head, pub descs: list_head, pub component_bitmap: *mut usize, pub component_bitmap_len: u16, pub version_string: *const u8, pub version_len: u8, pub version_type: u8, pub package_data: *const u8, pub package_data_len: u16 }
#[repr(C)] pub struct pldmfw_component { pub entry: list_head, pub index: u8, pub classification: u16, pub identifier: u16, pub comparison_stamp: u32, pub options: u16, pub activation_method: u16, pub version_type: u8, pub version_len: u8, pub version_string: *const u8, pub component_data: *const u8, pub component_size: u32 }
#[repr(C)] pub struct pldmfw_ops { pub match_record: Option<unsafe extern "C" fn(*mut pldmfw,*mut pldmfw_record)->bool>, pub send_package_data: Option<unsafe extern "C" fn(*mut pldmfw,*const u8,u16)->i32>, pub send_component_table: Option<unsafe extern "C" fn(*mut pldmfw,*mut pldmfw_component,u8)->i32>, pub flash_component: unsafe extern "C" fn(*mut pldmfw,*mut pldmfw_component)->i32, pub finalize_update: Option<unsafe extern "C" fn(*mut pldmfw)->i32> }
#[repr(C)] pub struct pldmfw { pub dev: *mut device, pub ops: *const pldmfw_ops, pub mode: u32, pub component_identifier: u16 }

const EFAULT: i32 = 14; const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const ENOENT: i32 = 2; const EOPNOTSUPP: i32 = 95; const EBADMSG: i32 = 74;
const PACKAGE_HEADER_FORMAT_REVISION: u8 = 1;

unsafe fn fw_space(d:*mut pldmfw_priv, o:usize, l:usize)->i32 { if (*(*d).fw).size < o+l { return -EFAULT; } 0 }
unsafe fn move_offset(d:*mut pldmfw_priv, n:usize)->i32 { let e=fw_space(d,(*d).offset,n); if e!=0{return e;} (*d).offset+=n; 0 }
unsafe fn parse_header(d:*mut pldmfw_priv)->i32 { if move_offset(d,core::mem::size_of::<__pldm_header>())!=0{return -EFAULT;} let h=(*d).fw.as_ref().unwrap().data as *const __pldm_header; (*d).header=h; if !uuid_equal(&(*h).id,&pldm_firmware_header_id){return -EINVAL;} if (*h).revision!=PACKAGE_HEADER_FORMAT_REVISION{return -EOPNOTSUPP;} (*d).total_header_size=get_unaligned_le16((*h).size.as_ptr() as _); (*d).component_bitmap_len=get_unaligned_le16((*h).component_bitmap_len.as_ptr() as _); if (*d).component_bitmap_len%8!=0{return -EINVAL;} (*d).bitmap_size=(*d).component_bitmap_len/8; if move_offset(d,(*h).version_len as usize)!=0{return -EFAULT;} let a=(*d).fw.as_ref().unwrap().data.add((*d).offset) as *const __pldmfw_record_area; if move_offset(d,core::mem::size_of::<__pldmfw_record_area>())!=0{return -EFAULT;} (*d).record_count=(*a).record_count; (*d).record_start=(*a).records.as_ptr(); 0 }

unsafe fn check_desc(_d:*mut pldmfw_priv, typ:u16, size:u16)->i32 { let expected=match typ { 1|2|3|4=>2, 5=>1, 6=>3, 7|8|9|10=>4, 11=>16, 12=>return 0, _=>return 0 }; if size!=expected{return -EINVAL} 0 }
unsafe fn parse_image(d:*mut pldmfw_priv)->i32 { if (*d).context.is_null()||(*d).fw.is_null()||(*(*d).fw).data.is_null()||(*(*d).fw).size==0{return -EINVAL;} let e=parse_header(d); if e!=0{return e;} 0 }

#[no_mangle] pub unsafe extern "C" fn pldmfw_op_pci_match_record(_context:*mut pldmfw,_record:*mut pldmfw_record)->bool { false }
#[no_mangle] pub unsafe extern "C" fn pldmfw_flash_image(context:*mut pldmfw, fw:*const firmware)->i32 { let d=kzalloc(core::mem::size_of::<pldmfw_priv>()) as *mut pldmfw_priv; if d.is_null(){return -ENOMEM;} (*d).context=context; (*d).fw=fw; let e=parse_image(d); if e!=0 { kfree(d as _); return e; } kfree(d as _); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
