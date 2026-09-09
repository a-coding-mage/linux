// SPDX-License-Identifier: GPL-2.0
/* UEFI Common Platform Error Record (CPER) support. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Types, constants, macros, and external functions are supplied by the kernel headers. */
extern "C" {
    fn ktime_get_real_seconds() -> i64;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> c_int;
    fn dmi_memdev_name(handle: u16, bank: *mut *const c_char, device: *mut *const c_char);
    fn acpi_hest_get_payload(gdata: *mut acpi_hest_generic_data) -> *mut c_void;
    fn acpi_hest_get_version(gdata: *mut acpi_hest_generic_data) -> u16;
    fn print_hex_dump(prefix: *const c_char, unused: *const c_char, typ: c_int,
                      rowsize: usize, groupsize: usize, buf: *const c_void,
                      len: usize, ascii: bool);
    fn trace_seq_buffer_ptr(p: *mut trace_seq) -> *const c_char;
    fn trace_seq_printf(p: *mut trace_seq, fmt: *const c_char, ...);
    fn trace_seq_putc(p: *mut trace_seq, c: c_char);
    fn cxl_cper_print_prot_err(pfx: *const c_char, err: *const cxl_cper_sec_prot_err);
    fn bcd2bin(x: u8) -> u8;
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool;
    fn apei_estatus_for_each_section(estatus: *const acpi_hest_generic_status,
                                     gdata: *mut *mut acpi_hest_generic_data) -> bool;
}

#[repr(C)] pub struct guid_t { pub b: [u8; 16] }
#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct acpi_hest_generic_data { pub section_type: [u8;16], pub error_severity:u16, pub validation_bits:u8, pub fru_id:guid_t, pub fru_text:[c_char;20], pub error_data_length:u32 }
#[repr(C)] pub struct acpi_hest_generic_data_v300 { pub base: acpi_hest_generic_data, pub time_stamp: [u8;8] }
#[repr(C)] pub struct acpi_hest_generic_status { pub error_severity:u16, pub data_length:u32, pub raw_data_length:u32, pub raw_data_offset:u32 }
#[repr(C)] pub struct cper_sec_proc_generic { pub validation_bits:u64, pub proc_type:u8, pub proc_isa:u8, pub proc_error_type:u16, pub operation:u16, pub flags:u16, pub level:u8, pub cpu_version:u64, pub proc_id:u64, pub target_addr:u64, pub requestor_id:u64, pub responder_id:u64, pub ip:u64 }
#[repr(C)] pub struct cper_sec_mem_err { pub validation_bits:u64, pub error_status:u64, pub physical_addr:u64, pub physical_addr_mask:u64, pub node:u16, pub card:u16, pub module:u16, pub bank:u16, pub device:u16, pub row:u32, pub column:u16, pub bit_pos:u16, pub requestor_id:u64, pub responder_id:u64, pub target_id:u64, pub error_type:u16, pub extended:u64, pub rank:u16, pub mem_array_handle:u16, pub mem_dev_handle:u16 }
#[repr(C)] pub struct cper_sec_mem_err_old { _private: [u8;0] }
#[repr(C)] pub struct cper_mem_err_compact { pub validation_bits:u64, pub node:u16,pub card:u16,pub module:u16,pub bank:u16,pub device:u16,pub row:u32,pub column:u16,pub bit_pos:u16,pub requestor_id:u64,pub responder_id:u64,pub target_id:u64,pub extended:u64,pub rank:u16,pub mem_array_handle:u16,pub mem_dev_handle:u16 }
#[repr(C)] pub struct cper_sec_pcie { pub validation_bits:u64, pub port_type:u8, pub version: PcieVersion, pub command:u16,pub status:u16,pub device_id:PcieDeviceId,pub serial_number:PcieSerial,pub bridge:PcieBridge,pub aer_info:[u8;0] }
#[repr(C)] pub struct PcieVersion { pub major:u8,pub minor:u8 }
#[repr(C)] pub struct PcieDeviceId { pub segment:u16,pub bus:u8,pub device:u8,pub function:u8,pub slot:u16,pub secondary_bus:u8,pub vendor_id:u16,pub device_id:u16,pub class_code:[u8;3] }
#[repr(C)] pub struct PcieSerial { pub lower:u32,pub upper:u32 }
#[repr(C)] pub struct PcieBridge { pub secondary_status:u16,pub control:u16 }
#[repr(C)] pub struct cper_sec_fw_err_rec_ref { pub record_type:u16,pub revision:u16,pub record_identifier:u64,pub record_identifier_guid:guid_t }
#[repr(C)] pub struct cxl_cper_sec_prot_err { _private:[u8;0] }

static SEVERITY_STRS: [&[u8]; 4] = [b"recoverable\0", b"fatal\0", b"corrected\0", b"info\0"];
static PROC_TYPE_STRS: [&[u8];3] = [b"IA32/X64\0",b"IA64\0",b"ARM\0"];
static PROC_ISA_STRS: [&[u8];5] = [b"IA32\0",b"IA64\0",b"X64\0",b"ARM A32/T32\0",b"ARM A64\0"];
static PROC_ERROR_TYPE_STRS: [&[u8];4] = [b"cache error\0",b"TLB error\0",b"bus error\0",b"micro-architectural error\0"];
static PROC_OP_STRS: [&[u8];4] = [b"unknown or generic\0",b"data read\0",b"data write\0",b"instruction execution\0"];
static PROC_FLAG_STRS: [&[u8];4] = [b"restartable\0",b"precise IP\0",b"overflow\0",b"corrected\0"];
static MEM_ERR_TYPE_STRS: [&[u8];16] = [b"unknown\0",b"no error\0",b"single-bit ECC\0",b"multi-bit ECC\0",b"single-symbol chipkill ECC\0",b"multi-symbol chipkill ECC\0",b"master abort\0",b"target abort\0",b"parity error\0",b"watchdog timeout\0",b"invalid address\0",b"mirror Broken\0",b"memory sparing\0",b"scrub corrected error\0",b"scrub uncorrected error\0",b"physical memory map-out event\0"];

pub unsafe fn cper_next_record_id() -> u64 { static mut SEQ:u64=0; if SEQ==0 { let t=ktime_get_real_seconds(); SEQ=if t<0x80000000 {(ktime_get_real_seconds() as u64)<<32} else {0x8000000000000000u64|((ktime_get_real_seconds() as u64)<<24)}; } SEQ=SEQ.wrapping_add(1); SEQ }
pub unsafe fn cper_severity_str(severity:u32)->*const c_char { if severity<4 { SEVERITY_STRS[severity as usize].as_ptr() as *const c_char } else { b"unknown\0".as_ptr() as *const c_char } }

pub unsafe fn cper_print_bits(pfx:*const c_char,bits:u32,strs:*const *const c_char,strs_size:u32) { let mut len=0usize; let mut buf=[0i8;84]; for i in 0..strs_size { if bits&(1u32<<i)==0 {continue;} let s=*strs.add(i as usize); if s.is_null(){continue;} if len!=0 && len+strlen(s)+2>80 { printk(b"%s\n\0".as_ptr() as _,buf.as_ptr()); len=0; } if len==0 {len=snprintf(buf.as_mut_ptr(),84,b"%s%s\0".as_ptr() as _,pfx,s) as usize;} else {len+=scnprintf(buf.as_mut_ptr().add(len),84-len,b", %s\0".as_ptr() as _,s) as usize;} } if len!=0 {printk(b"%s\n\0".as_ptr() as _,buf.as_ptr());} }

pub unsafe fn cper_bits_to_str(buf:*mut c_char,buf_size:c_int,bits:c_ulong,strs:*const *const c_char,strs_size:u32)->c_int { let mut len=buf_size; let mut out=buf; *buf=0; for i in 0..strs_size { if bits&(1usize<<i)==0 {continue;} if *buf!=0 && len>0 {*out=b'|' as c_char;len-=1;out=out.add(1);} let size=strscpy(out,*strs.add(i as usize),len as usize); if size<0{return size;} len-=size;out=out.add(size as usize);} buf_size-len }

pub unsafe fn cper_mem_err_type_str(etype:u32)->*const c_char {if etype<16 {MEM_ERR_TYPE_STRS[etype as usize].as_ptr() as _}else{b"unknown\0".as_ptr() as _}}
pub unsafe fn cper_mem_err_status_str(status:u64)->*const c_char {let s=match (status>>8)&0xff {1=>b"Error detected internal to the component\0",4=>b"Storage error in DRAM memory\0",5=>b"Storage error in TLB\0",6=>b"Storage error in cache\0",7=>b"Error in one or more functional units\0",8=>b"Component failed self test\0",9=>b"Overflow or undervalue of internal queue\0",16=>b"Error detected in the bus\0",17=>b"Virtual address not found on IO-TLB or IO-PDIR\0",18=>b"Improper access error\0",19=>b"Access to a memory address which is not mapped to any component\0",20=>b"Loss of Lockstep\0",21=>b"Response not associated with a request\0",22=>b"Bus parity error - must also set the A, C, or D Bits\0",23=>b"Detection of a protocol error\0",24=>b"Detection of a PATH_ERROR\0",25=>b"Bus operation timeout\0",26=>b"A read was issued to data that has been poisoned\0",_=>b"Reserved\0"};s.as_ptr() as _}

pub unsafe fn cper_mem_err_pack(mem:*const cper_sec_mem_err,cmem:*mut cper_mem_err_compact){ (*cmem).validation_bits=(*mem).validation_bits;(*cmem).node=(*mem).node;(*cmem).card=(*mem).card;(*cmem).module=(*mem).module;(*cmem).bank=(*mem).bank;(*cmem).device=(*mem).device;(*cmem).row=(*mem).row;(*cmem).column=(*mem).column;(*cmem).bit_pos=(*mem).bit_pos;(*cmem).requestor_id=(*mem).requestor_id;(*cmem).responder_id=(*mem).responder_id;(*cmem).target_id=(*mem).target_id;(*cmem).extended=(*mem).extended;(*cmem).rank=(*mem).rank;(*cmem).mem_array_handle=(*mem).mem_array_handle;(*cmem).mem_dev_handle=(*mem).mem_dev_handle; }

pub unsafe fn cper_mem_err_location(mem:*mut cper_mem_err_compact,msg:*mut c_char)->u32 { if msg.is_null(){return 0;} let mut n=0u32; let len=256u32; macro_rules! put {($cond:expr,$fmt:expr,$v:expr)=>{if $cond {n+=scnprintf(msg.add(n as usize), (len-n) as usize, $fmt.as_ptr() as _, $v) as u32;}}} put!((*mem).validation_bits&1!=0,b"node:%d \0",(*mem).node as c_int);put!((*mem).validation_bits&2!=0,b"card:%d \0",(*mem).card as c_int);put!((*mem).validation_bits&4!=0,b"module:%d \0",(*mem).module as c_int);put!((*mem).validation_bits&8!=0,b"rank:%d \0",(*mem).rank as c_int);put!((*mem).validation_bits&16!=0,b"bank:%d \0",(*mem).bank as c_int);put!((*mem).validation_bits&32!=0,b"device:%d \0",(*mem).device as c_int);put!((*mem).validation_bits&64!=0,b"row:%d \0",(*mem).row as c_int);put!((*mem).validation_bits&128!=0,b"column:%d \0",(*mem).column as c_int); n }

pub unsafe fn cper_dimm_err_location(mem:*mut cper_mem_err_compact,msg:*mut c_char)->u32 { if msg.is_null(){return 0;} let mut bank=core::ptr::null();let mut device=core::ptr::null();dmi_memdev_name((*mem).mem_dev_handle,&mut bank,&mut device);if !bank.is_null()&&!device.is_null(){snprintf(msg,256,b"DIMM location: %s %s \0".as_ptr() as _,bank,device) as u32}else{snprintf(msg,256,b"DIMM location: not present. DMI handle: 0x%.4x \0".as_ptr() as _,(*mem).mem_dev_handle) as u32} }

pub unsafe fn cper_mem_err_unpack(p:*mut trace_seq,cmem:*mut cper_mem_err_compact)->*const c_char {let ret=trace_seq_buffer_ptr(p);let mut b=[0i8;256];if cper_mem_err_location(cmem,b.as_mut_ptr())!=0{trace_seq_printf(p,b"%s\0".as_ptr() as _,b.as_ptr());}if cper_dimm_err_location(cmem,b.as_mut_ptr())!=0{trace_seq_printf(p,b"%s\0".as_ptr() as _,b.as_ptr());}trace_seq_putc(p,0);ret}

pub unsafe fn cper_estatus_check_header(e:*const acpi_hest_generic_status)->c_int {if (*e).data_length!=0&&(*e).data_length<core::mem::size_of::<acpi_hest_generic_data>() as u32{return -22;}if (*e).raw_data_length!=0&&(*e).raw_data_offset<core::mem::size_of::<acpi_hest_generic_status>() as u32+(*e).data_length{return -22;}0}
pub unsafe fn cper_estatus_check(e:*const acpi_hest_generic_status)->c_int {let rc=cper_estatus_check_header(e);if rc!=0{return rc;}0}

// The remaining section printers retain their external kernel ABI; their declarations
// are kept here because the corresponding platform section layouts are external.
extern "C" { fn cper_print_proc_generic(pfx:*const c_char, proc:*const cper_sec_proc_generic); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
