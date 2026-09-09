// SPDX-License-Identifier: GPL-2.0-or-later
/* Parisc performance counters; translated from perf.c. */

const MAX_RDR_WORDS: usize = 24;
const PERF_VERSION: i32 = 2;

#[repr(C)]
struct RdrTblEnt { width: u16, num_words: u8, write_control: u8 }

static mut perf_processor_interface: i32 = UNKNOWN_INTF;
static mut perf_enabled: i32 = 0;
static mut cpu_device: *mut parisc_device = core::ptr::null_mut();

static perf_rdrs_W: [i32; 16] = [0,1,4,5,6,15,16,17,18,20,21,22,23,24,25,-1];
static perf_rdrs_U: [i32; 16] = [0,1,4,5,6,7,16,17,18,20,21,22,23,24,25,-1];

static perf_rdr_tbl_W: [RdrTblEnt; 32] = [
    RdrTblEnt{width:19,num_words:1,write_control:8}, RdrTblEnt{width:16,num_words:1,write_control:16}, RdrTblEnt{width:72,num_words:2,write_control:0}, RdrTblEnt{width:81,num_words:2,write_control:0},
    RdrTblEnt{width:328,num_words:6,write_control:0}, RdrTblEnt{width:160,num_words:3,write_control:0}, RdrTblEnt{width:336,num_words:6,write_control:0}, RdrTblEnt{width:164,num_words:3,write_control:0},
    RdrTblEnt{width:0,num_words:0,write_control:0}, RdrTblEnt{width:35,num_words:1,write_control:0}, RdrTblEnt{width:6,num_words:1,write_control:0}, RdrTblEnt{width:18,num_words:1,write_control:0},
    RdrTblEnt{width:13,num_words:1,write_control:0}, RdrTblEnt{width:8,num_words:1,write_control:0}, RdrTblEnt{width:8,num_words:1,write_control:0}, RdrTblEnt{width:8,num_words:1,write_control:0},
    RdrTblEnt{width:1530,num_words:24,write_control:0}, RdrTblEnt{width:16,num_words:1,write_control:0}, RdrTblEnt{width:4,num_words:1,write_control:0}, RdrTblEnt{width:0,num_words:0,write_control:0},
    RdrTblEnt{width:152,num_words:3,write_control:24}, RdrTblEnt{width:152,num_words:3,write_control:24}, RdrTblEnt{width:233,num_words:4,write_control:48}, RdrTblEnt{width:233,num_words:4,write_control:48},
    RdrTblEnt{width:71,num_words:2,write_control:0}, RdrTblEnt{width:71,num_words:2,write_control:0}, RdrTblEnt{width:11,num_words:1,write_control:0}, RdrTblEnt{width:18,num_words:1,write_control:0},
    RdrTblEnt{width:128,num_words:2,write_control:0}, RdrTblEnt{width:0,num_words:0,write_control:0}, RdrTblEnt{width:16,num_words:1,write_control:0}, RdrTblEnt{width:16,num_words:1,write_control:0},
];
static perf_rdr_tbl_U: [RdrTblEnt; 32] = [
    RdrTblEnt{width:19,num_words:1,write_control:8},RdrTblEnt{width:32,num_words:1,write_control:16},RdrTblEnt{width:20,num_words:1,write_control:0},RdrTblEnt{width:0,num_words:0,write_control:0},
    RdrTblEnt{width:344,num_words:6,write_control:0},RdrTblEnt{width:176,num_words:3,write_control:0},RdrTblEnt{width:336,num_words:6,write_control:0},RdrTblEnt{width:0,num_words:0,write_control:0},
    RdrTblEnt{width:0,num_words:0,write_control:0},RdrTblEnt{width:0,num_words:0,write_control:0},RdrTblEnt{width:28,num_words:1,write_control:0},RdrTblEnt{width:33,num_words:1,write_control:0},
    RdrTblEnt{width:0,num_words:0,write_control:0},RdrTblEnt{width:230,num_words:4,write_control:0},RdrTblEnt{width:32,num_words:1,write_control:0},RdrTblEnt{width:128,num_words:2,write_control:0},
    RdrTblEnt{width:1494,num_words:24,write_control:0},RdrTblEnt{width:18,num_words:1,write_control:0},RdrTblEnt{width:4,num_words:1,write_control:0},RdrTblEnt{width:0,num_words:0,write_control:0},
    RdrTblEnt{width:158,num_words:3,write_control:24},RdrTblEnt{width:158,num_words:3,write_control:24},RdrTblEnt{width:194,num_words:4,write_control:48},RdrTblEnt{width:194,num_words:4,write_control:48},
    RdrTblEnt{width:71,num_words:2,write_control:0},RdrTblEnt{width:71,num_words:2,write_control:0},RdrTblEnt{width:28,num_words:1,write_control:0},RdrTblEnt{width:33,num_words:1,write_control:0},
    RdrTblEnt{width:88,num_words:2,write_control:0},RdrTblEnt{width:32,num_words:1,write_control:0},RdrTblEnt{width:24,num_words:1,write_control:0},RdrTblEnt{width:16,num_words:1,write_control:0},
];
static perf_bitmasks: [u64; 10] = [0,0xfdffe00000000000,0x003f000000000000,0x00ffffffffffffff,0xffffffffffffffff,0xfffffffc00000000,0xffffffffffffffff,0xffffffffffffffff,0xfffffffffffffffcu64,0xff00000000000000];
static perf_bitmasks_piranha: [u64; 10] = [0,0xfdffe00000000000,0x003f000000000000,0x00ffffffffffffff,0xffffffffffffffff,0xfffffffc00000000,0xffffffffffffffff,0xffffffffffffffff,0xffffffffffffffff,0xfffc000000000000];
static mut bitmask_array: *const u64 = core::ptr::null();

extern "C" {
    fn perf_rdr_shift_in_W(rdr_num:u32,width:u16)->u64; fn perf_rdr_shift_in_U(rdr_num:u32,width:u16)->u64;
    fn perf_rdr_shift_out_W(rdr_num:u32,buffer:u64); fn perf_rdr_shift_out_U(rdr_num:u32,buffer:u64);
    fn perf_intrigue_enable_perf_counters(); fn perf_intrigue_disable_perf_counters();
}

unsafe fn perf_start_counters() { perf_intrigue_enable_perf_counters(); }

unsafe fn perf_rdr_get_entry(rdr_num:u32) -> *const RdrTblEnt {
    if perf_processor_interface == ONYX_INTF { &perf_rdr_tbl_U[rdr_num as usize] } else { &perf_rdr_tbl_W[rdr_num as usize] }
}

unsafe fn perf_rdr_read_ubuf(rdr_num:u32, buffer:*mut u64) -> i32 {
    let t = &*perf_rdr_get_entry(rdr_num); if t.width == 0 { return 0; }
    for i in 0..t.num_words as usize { *buffer.add(i)=0; }
    let xbits = t.width & 0x3f; let data_mask = if xbits != 0 { (1u64 << (64-xbits))-1 } else { 0 };
    let mut i=t.num_words as usize; while i != 0 { i-=1; let data=if perf_processor_interface==ONYX_INTF {perf_rdr_shift_in_U(rdr_num,t.width)} else {perf_rdr_shift_in_W(rdr_num,t.width)}; if xbits!=0 { *buffer.add(i) |= data << (64-xbits); if i!=0 { *buffer.add(i-1) |= (data >> xbits) & data_mask; } } else {*buffer.add(i)=data;} } 1
}

unsafe fn perf_rdr_clear(rdr_num:u32)->i32 { let t=&*perf_rdr_get_entry(rdr_num); if t.width==0{return -1;} let mut i=t.num_words; while i!=0 {i-=1; if perf_processor_interface==ONYX_INTF{perf_rdr_shift_out_U(rdr_num,0)}else{perf_rdr_shift_out_W(rdr_num,0)}} 0 }

unsafe fn perf_rdr_write(rdr_num:u32, buffer:*mut u64) { let t=&*perf_rdr_get_entry(rdr_num); if t.width==0{return;} let mut i=t.num_words; while i!=0 {i-=1;if perf_processor_interface==ONYX_INTF{perf_rdr_shift_out_U(rdr_num,*buffer.add(i as usize))}else{perf_rdr_shift_out_W(rdr_num,*buffer.add(i as usize))}} }

unsafe fn perf_stop_counters(raddr:*mut u32)->i32 {
    let mut userbuf=[0u64;MAX_RDR_WORDS]; perf_intrigue_disable_perf_counters();
    if perf_processor_interface==ONYX_INTF { if perf_rdr_read_ubuf(16,userbuf.as_mut_ptr())==0{return -13;} let mut t=(userbuf[21]<<22)&0xffc00000; t|=(userbuf[22]>>42)&0x3fffff; t|=(userbuf[22]>>10)&0x80000000;*raddr=t as u32; t=(userbuf[22]>>9)&0xffffffff;t|=(userbuf[22]<<23)&0x80000000;*raddr.add(1)=t as u32;t=(userbuf[22]<<24)&0xff000000;t|=(userbuf[23]>>40)&0xffffff;t|=(userbuf[23]>>8)&0x80000000;*raddr.add(2)=t as u32;t=(userbuf[23]>>7)&0xffffffff;t|=(userbuf[23]<<25)&0x80000000;*raddr.add(3)=t as u32;userbuf[21]&=0xfffffffffffffc00;userbuf[22]=0;userbuf[23]=0;perf_rdr_write(16,userbuf.as_mut_ptr()); } else { if perf_rdr_read_ubuf(15,userbuf.as_mut_ptr())==0{return -13;}perf_rdr_clear(15);*raddr=(userbuf[0]>>32) as u32;*raddr.add(1)=userbuf[0] as u32;*raddr.add(2)=(userbuf[1]>>32) as u32;*raddr.add(3)=userbuf[1] as u32; } 0
}

// The remaining device-facing definitions retain the C interfaces and external kernel dependencies.
// The disabled image patching block is intentionally preserved as a conditional comment.
unsafe fn perf_config(image_ptr:*mut u32)->i32 { let mut r=[0u32;4]; if perf_stop_counters(r.as_mut_ptr())!=0{return -EINVAL;} if perf_write_image(image_ptr as *mut u64)!=0{return -EINVAL;} perf_start_counters(); core::mem::size_of::<u32>() as i32 }

unsafe fn perf_write_image(memaddr:*mut u64)->i32 {
    let mut buffer=[0u64;MAX_RDR_WORDS]; let mut list=if perf_processor_interface==ONYX_INTF{perf_rdrs_U.as_ptr()}else{perf_rdrs_W.as_ptr()};
    if perf_processor_interface==ONYX_INTF{perf_rdr_clear(16);perf_intrigue_enable_perf_counters();perf_intrigue_disable_perf_counters();}else{perf_rdr_clear(15);}
    while *list != -1 { let n=*list as u32; let t=&*perf_rdr_get_entry(n); perf_rdr_read_ubuf(n,buffer.as_mut_ptr()); let mut d=t.num_words as usize; if t.write_control!=0 { let masks=bitmask_array.add((t.write_control>>3) as usize); while d!=0{d-=1;let m=*masks.add(t.num_words as usize-d-1);buffer[d]=(m & *memaddr)+(!m & buffer[d]);memaddr=memaddr.add(1);} } else {while d!=0{d-=1;buffer[d]=*memaddr;memaddr=memaddr.add(1);}} perf_rdr_write(n,buffer.as_mut_ptr());list=list.add(1); }
    0
}

unsafe fn perf_open(_inode:*mut inode,_file:*mut file)->i32 { if perf_enabled != 0 { return -EBUSY; } perf_enabled=1; 0 }
unsafe fn perf_release(_inode:*mut inode,_file:*mut file)->i32 { perf_enabled=0; 0 }
unsafe fn perf_read(_file:*mut file,_buf:*mut u8,_cnt:usize,_ppos:*mut loff_t)->isize { 0 }
unsafe fn perf_write(_file:*mut file,buf:*const u8,count:usize,_ppos:*mut loff_t)->isize {
    let image_size = if perf_processor_interface==ONYX_INTF {PCXU_IMAGE_SIZE} else if perf_processor_interface==CUDA_INTF {PCXW_IMAGE_SIZE} else{return -EFAULT as isize};
    let _ = image_size; if !perfmon_capable(){return -EACCES as isize;} if count != 4{return -EIO as isize;}
    let image_type=*(buf as *const u32); let interface_type=(image_type>>16)&0xffff; let test=image_type&0xffff;
    if (perf_processor_interface==CUDA_INTF && interface_type!=CUDA_INTF)||(perf_processor_interface==ONYX_INTF&&interface_type!=ONYX_INTF){return -EINVAL as isize;}
    if (interface_type==CUDA_INTF&&test>=MAX_CUDA_IMAGES)||(interface_type==ONYX_INTF&&test>=MAX_ONYX_IMAGES){return -EINVAL as isize;}
    if interface_type==CUDA_INTF {perf_config(cuda_images[test as usize] as *mut u32) as isize} else {perf_config(onyx_images[test as usize] as *mut u32) as isize}
}
unsafe fn perf_ioctl(_file:*mut file,cmd:u32,arg:usize)->isize { match cmd { PA_PERF_ON=>{perf_start_counters();0}, PA_PERF_OFF=>{let mut r=[0u32;4];if perf_stop_counters(r.as_mut_ptr())!=0{-EFAULT as isize}else{core::ptr::copy_nonoverlapping(r.as_ptr(),arg as *mut u32,4);0}}, PA_PERF_VERSION=>{*(arg as *mut i32)=PERF_VERSION;0}, _=>-ENOTTY as isize} }

unsafe fn perf_patch_images() { /* #if 0 FIXME: source image IVA patching block intentionally disabled. */ }
unsafe fn perf_init()->i32 { bitmask_array=perf_bitmasks.as_ptr(); if boot_cpu_data.cpu_type==pcxu||boot_cpu_data.cpu_type==pcxu_{perf_processor_interface=ONYX_INTF;}else if boot_cpu_data.cpu_type==pcxw||boot_cpu_data.cpu_type==pcxw_||boot_cpu_data.cpu_type==pcxw2||boot_cpu_data.cpu_type==mako||boot_cpu_data.cpu_type==mako2{perf_processor_interface=CUDA_INTF;if boot_cpu_data.cpu_type==pcxw2||boot_cpu_data.cpu_type==mako||boot_cpu_data.cpu_type==mako2{bitmask_array=perf_bitmasks_piranha.as_ptr();}}else{perf_processor_interface=UNKNOWN_INTF;return -ENODEV;}perf_patch_images();0 }

// Kernel-provided declarations and constants referenced above.
extern "C" { fn perfmon_capable()->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
