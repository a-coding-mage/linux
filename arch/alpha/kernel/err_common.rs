// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/arch/alpha/kernel/err_common.c. */

use core::ffi::c_void;

// Types, constants, globals, and functions below are supplied by the kernel
// headers and companion implementation files.
extern "C" {
    static mut hwrpb: *mut hwrpb;
    fn printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct el_common { pub size: i32, pub retry: i32, pub err2: i32, pub code: i32, pub frame_rev: i32, pub proc_offset: u32, pub sys_offset: u32 }
#[repr(C)] pub struct el_timestamp_bits { pub month: i32, pub day: i32, pub year: i32, pub hour: i32, pub minute: i32, pub second: i32 }
#[repr(C)] pub union el_timestamp { pub as_int: i32, pub b: el_timestamp_bits }
#[repr(C)] pub struct sys_err { pub frame_length: i32, pub frame_packet_count: i32 }
#[repr(C)] pub struct sys_event { pub frame_length: i32, pub frame_packet_count: i32, pub timestamp: el_timestamp }
#[repr(C)] pub struct err_halt { pub frame_length: i32, pub frame_packet_count: i32, pub timestamp: el_timestamp }
#[repr(C)] pub struct logout_header { pub frame_length: i32 }
#[repr(C)] pub union el_subpacket_by_type { pub sys_err: sys_err, pub sys_event: sys_event, pub err_halt: err_halt, pub logout_header: logout_header }
#[repr(C)] pub struct el_subpacket { pub class: i32, pub type_: i32, pub revision: i32, pub length: usize, pub by_type: el_subpacket_by_type }
#[repr(C)] pub struct el_subpacket_handler { pub class: i32, pub handler: Option<unsafe extern "C" fn(*mut el_subpacket) -> *mut el_subpacket>, pub next: *mut el_subpacket_handler }
#[repr(C)] pub struct el_subpacket_annotation { pub class: i32, pub type_: i32, pub revision: i32, pub annotation: *mut *mut u8, pub description: *const u8, pub next: *mut el_subpacket_annotation }
#[repr(C)] pub struct percpu_struct { pub console_data_log_pa: usize }
#[repr(C)] pub struct hwrpb { pub nr_processors: usize, pub processor_offset: usize, pub processor_size: usize }

const EINVAL: i32 = 22;
const EL_CLASS__HEADER: i32 = 0;
const EL_CLASS__TERMINATION: i32 = 0xffff;
const EL_TYPE__HEADER__SYSTEM_ERROR_FRAME: i32 = 0;
const EL_TYPE__HEADER__SYSTEM_EVENT_FRAME: i32 = 1;
const EL_TYPE__HEADER__HALT_FRAME: i32 = 2;
const EL_TYPE__HEADER__LOGOUT_FRAME: i32 = 3;
const IDENT_ADDR: usize = 0;

#[no_mangle]
pub static mut err_print_prefix: *mut u8 = b"< notice >\0" as *const u8 as *mut u8;
static mut subpacket_handler_list: *mut el_subpacket_handler = core::ptr::null_mut();
static mut subpacket_annotation_list: *mut el_subpacket_annotation = core::ptr::null_mut();

pub unsafe fn mchk_dump_mem(data: *mut c_void, length: usize, mut annotation: *mut *mut u8) {
    let ldata = data as *mut usize;
    let mut i = 0usize;
    while i.wrapping_mul(core::mem::size_of::<usize>()) < length {
        if !annotation.is_null() && (*annotation.add(i)).is_null() { annotation = core::ptr::null_mut(); }
        printk(b"%s    %08x: %016lx    %s\n\0".as_ptr(), err_print_prefix, (i * core::mem::size_of::<usize>()) as u32, *ldata.add(i), if !annotation.is_null() { *annotation.add(i) } else { b"\0".as_ptr() });
        i += 1;
    }
}

pub unsafe fn mchk_dump_logout_frame(mchk_header: *mut el_common) {
    printk(b"%s  -- Frame Header --\n    Frame Size:   %d (0x%x) bytes\n    Flags:        %s%s\n    MCHK Code:    0x%x\n    Frame Rev:    %d\n    Proc Offset:  0x%08x\n    Sys Offset:   0x%08x\n  -- Processor Region --\n\0".as_ptr(), err_print_prefix, (*mchk_header).size, (*mchk_header).size, if (*mchk_header).retry != 0 { b"RETRY \0".as_ptr() } else { b"\0".as_ptr() }, if (*mchk_header).err2 != 0 { b"SECOND_ERR \0".as_ptr() } else { b"\0".as_ptr() }, (*mchk_header).code, (*mchk_header).frame_rev, (*mchk_header).proc_offset, (*mchk_header).sys_offset);
    mchk_dump_mem((mchk_header as usize + (*mchk_header).proc_offset as usize) as *mut c_void, ((*mchk_header).sys_offset - (*mchk_header).proc_offset) as usize, core::ptr::null_mut());
    printk(b"%s  -- System Region --\n\0".as_ptr(), err_print_prefix);
    mchk_dump_mem((mchk_header as usize + (*mchk_header).sys_offset as usize) as *mut c_void, ((*mchk_header).size as u32 - (*mchk_header).sys_offset) as usize, core::ptr::null_mut());
    printk(b"%s  -- End of Frame --\n\0".as_ptr(), err_print_prefix);
}

unsafe fn el_process_header_subpacket(mut header: *mut el_subpacket) -> *mut el_subpacket {
    let mut timestamp = el_timestamp { as_int: 0 }; let mut name = b"UNKNOWN EVENT\0".as_ptr(); let mut packet_count = 0; let mut length = 0;
    if (*header).class != EL_CLASS__HEADER { printk(b"%s** Unexpected header CLASS %d TYPE %d, aborting\n\0".as_ptr(), err_print_prefix, (*header).class, (*header).type_); return core::ptr::null_mut(); }
    match (*header).type_ {
        EL_TYPE__HEADER__SYSTEM_ERROR_FRAME => { name=b"SYSTEM ERROR\0".as_ptr(); length=(*header).by_type.sys_err.frame_length; packet_count=(*header).by_type.sys_err.frame_packet_count; }
        EL_TYPE__HEADER__SYSTEM_EVENT_FRAME => { name=b"SYSTEM EVENT\0".as_ptr(); length=(*header).by_type.sys_event.frame_length; packet_count=(*header).by_type.sys_event.frame_packet_count; timestamp=(*header).by_type.sys_event.timestamp; }
        EL_TYPE__HEADER__HALT_FRAME => { name=b"ERROR HALT\0".as_ptr(); length=(*header).by_type.err_halt.frame_length; packet_count=(*header).by_type.err_halt.frame_packet_count; timestamp=(*header).by_type.err_halt.timestamp; }
        EL_TYPE__HEADER__LOGOUT_FRAME => { name=b"LOGOUT FRAME\0".as_ptr(); length=(*header).by_type.logout_header.frame_length; packet_count=1; }
        _ => { printk(b"%s** Unknown header - CLASS %d TYPE %d, aborting\n\0".as_ptr(), err_print_prefix, (*header).class, (*header).type_); return core::ptr::null_mut(); }
    }
    printk(b"%s*** %s:\n  CLASS %d, TYPE %d\n\0".as_ptr(), err_print_prefix, name, (*header).class, (*header).type_); el_print_timestamp(&mut timestamp); el_process_subpackets(header, packet_count);
    header = (header as usize + (*header).length + length as usize) as *mut el_subpacket; header
}

unsafe fn el_process_subpacket_reg(header: *mut el_subpacket) -> *mut el_subpacket { let mut h=subpacket_handler_list; while !h.is_null() && (*h).class != (*header).class { h=(*h).next; } if !h.is_null() { if let Some(f)=(*h).handler { f(header) } else { core::ptr::null_mut() } } else { core::ptr::null_mut() } }
pub unsafe fn el_print_timestamp(timestamp: *mut el_timestamp) { if (*timestamp).as_int != 0 { printk(b"%s  TIMESTAMP: %d/%d/%02d %d:%02d:%0d\n\0".as_ptr(), err_print_prefix, (*timestamp).b.month, (*timestamp).b.day, (*timestamp).b.year, (*timestamp).b.hour, (*timestamp).b.minute, (*timestamp).b.second); } }
pub unsafe fn el_process_subpackets(header: *mut el_subpacket, packet_count: i32) { let mut subpacket=(header as usize+(*header).length) as *mut el_subpacket; let mut i=0; while !subpacket.is_null() && i<packet_count { printk(b"%sPROCESSING SUBPACKET %d\n\0".as_ptr(),err_print_prefix,i); subpacket=el_process_subpacket(subpacket); i+=1; } }
pub unsafe fn el_process_subpacket(header: *mut el_subpacket) -> *mut el_subpacket { match (*header).class { EL_CLASS__TERMINATION=>core::ptr::null_mut(), EL_CLASS__HEADER=>el_process_header_subpacket(header), _=>{let n=el_process_subpacket_reg(header); if n.is_null(){printk(b"%s** Unexpected header CLASS %d TYPE %d -- aborting.\n\0".as_ptr(),err_print_prefix,(*header).class,(*header).type_);} n} } }
pub unsafe fn el_annotate_subpacket(header: *mut el_subpacket) { let mut a=subpacket_annotation_list; let mut annotation=core::ptr::null_mut(); while !a.is_null(){if (*a).class==(*header).class&&(*a).type_==(*header).type_&&(*a).revision==(*header).revision{annotation=(*a).annotation; printk(b"%s  %s\n\0".as_ptr(),err_print_prefix,(*a).description);break;}a=(*a).next;} mchk_dump_mem(header as *mut c_void,(*header).length,annotation); }

unsafe fn cdl_process_console_data_log(cpu: i32, pcpu: *mut percpu_struct) { let mut header=(IDENT_ADDR|(*pcpu).console_data_log_pa) as *mut el_subpacket; printk(b"%s******* CONSOLE DATA LOG FOR CPU %d. *******\n*** Error(s) were logged on a previous boot\n\0".as_ptr(),err_print_prefix,cpu); let mut err=0; while !header.is_null()&&(*header).class!=EL_CLASS__TERMINATION{err+=1;header=el_process_subpacket(header);} (*pcpu).console_data_log_pa=0; printk(b"%s*** %d total error(s) logged\n**** END OF CONSOLE DATA LOG FOR CPU %d ****\n\0".as_ptr(),err_print_prefix,err,cpu); }
pub unsafe fn cdl_check_console_data_log() { let mut cpu=0; while cpu<(*hwrpb).nr_processors { let pcpu=((*hwrpb as usize)+(*hwrpb).processor_offset+cpu*(*hwrpb).processor_size) as *mut percpu_struct; if (*pcpu).console_data_log_pa!=0{cdl_process_console_data_log(cpu as i32,pcpu);} cpu+=1; } }
pub unsafe fn cdl_register_subpacket_annotation(new: *mut el_subpacket_annotation) -> i32 { let mut a=subpacket_annotation_list; if a.is_null(){subpacket_annotation_list=new;}else{while !(*a).next.is_null(){if ((*a).class==(*new).class&&(*a).type_==(*new).type_)||a==new{printk(b"Attempted to re-register subpacket annotation\n\0".as_ptr());return -EINVAL;}a=(*a).next;}(*a).next=new;}(*new).next=core::ptr::null_mut();0 }
pub unsafe fn cdl_register_subpacket_handler(new: *mut el_subpacket_handler) -> i32 { let mut h=subpacket_handler_list; if h.is_null(){subpacket_handler_list=new;}else{while !(*h).next.is_null(){if (*h).class==(*new).class||h==new{printk(b"Attempted to re-register subpacket handler\n\0".as_ptr());return -EINVAL;}h=(*h).next;}(*h).next=new;}(*new).next=core::ptr::null_mut();0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
