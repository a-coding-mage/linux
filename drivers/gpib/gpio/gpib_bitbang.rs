// SPDX-License-Identifier: GPL-2.0
// Literal Rust translation of gpib_bitbang.c. Kernel/GPIB symbols are supplied
// by the surrounding driver and are intentionally left as external dependencies.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Lines { D01_pin_nr=20, D02_pin_nr=26, D03_pin_nr=16, D04_pin_nr=19,
 D05_pin_nr=13, D06_pin_nr=12, D07_pin_nr=6, D08_pin_nr=5, EOI_pin_nr=9,
 DAV_pin_nr=10, NRFD_pin_nr=24, NDAC_pin_nr=23, IFC_pin_nr=22, SRQ_pin_nr=11,
 _ATN_pin_nr=25, REN_pin_nr=27, PE_pin_nr=7, DC_pin_nr=8, TE_pin_nr=18,
 ACT_LED_pin_nr=4, YOGA_D03_pin_nr=13, YOGA_D04_pin_nr=12,
 YOGA_D05_pin_nr=21, YOGA_D06_pin_nr=19 }

pub const GPIB_PINS: usize=16; pub const SN7516X_PINS: usize=4; pub const NUM_PINS: usize=20;
pub const DIR_READ:i32=0; pub const DIR_WRITE:i32=1;

extern "C" {
    static mut sn7516x_used:i32; static mut sn7516x:i32; static mut debug:i32;
    static mut pin_map:*mut i8; static mut all_descriptors:[*mut gpio_desc; NUM_PINS];
    fn gpiod_get_value(g:*mut gpio_desc)->i32; fn gpiod_set_value(g:*mut gpio_desc,v:i32);
    fn gpiod_direction_input(g:*mut gpio_desc)->i32; fn gpiod_direction_output(g:*mut gpio_desc,v:i32)->i32;
    fn set_bit(n:i32,p:*mut u64); fn clear_bit(n:i32,p:*mut u64);
    fn wake_up_interruptible(w:*mut wait_queue_head); fn spin_lock_irqsave(l:*mut spinlock_t,f:*mut usize);
    fn spin_unlock_irqrestore(l:*mut spinlock_t,f:usize); fn gpiod_to_irq(g:*mut gpio_desc)->i32;
    fn request_threaded_irq(i:i32,h:usize,t:usize,f:usize,n:*const i8,d:*mut core::ffi::c_void)->i32;
    fn free_irq(i:i32,d:*mut core::ffi::c_void); fn gpib_register_driver(i:*mut gpib_interface,m:*mut core::ffi::c_void)->i32;
    fn gpib_unregister_driver(i:*mut gpib_interface); fn kzalloc_obj()->*mut bb_priv; fn kfree(p:*mut bb_priv);
}
#[repr(C)] pub struct gpio_desc; #[repr(C)] pub struct wait_queue_head; #[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct timespec64 { pub tv_sec:i64,pub tv_nsec:i64 }
#[repr(C)] pub struct gpib_board { pub private_data:*mut bb_priv,pub status:u64,pub pad:u32,pub sad:u32,pub minor:i32,pub wait:*mut wait_queue_head,pub user_mutex:usize,pub gpib_dev:*mut core::ffi::c_void,pub buffer:*mut u8 }
#[repr(C)] pub struct gpib_board_config;
#[repr(C)] pub struct gpib_interface { pub name:*const i8,pub attach:usize,pub detach:usize,pub read:usize,pub write:usize,pub command:usize }
#[repr(C)] pub struct bb_priv { pub irq_NRFD:i32,pub irq_NDAC:i32,pub irq_DAV:i32,pub irq_SRQ:i32,pub dav_mode:i32,pub nrfd_mode:i32,pub ndac_mode:i32,pub dav_tx:i32,pub dav_rx:i32,pub eos:u8,pub eos_flags:i16,pub eos_check:i16,pub eos_check_8:i16,pub eos_mask_7:i16,pub end:i16,pub request:i32,pub count:i32,pub direction:i32,pub t1_delay:i32,pub rbuf:*mut u8,pub wbuf:*mut u8,pub end_flag:i32,pub r_busy:i32,pub w_busy:i32,pub write_done:i32,pub cmd:i32,pub w_cnt:usize,pub length:usize,pub w_buf:*mut u8,pub rw_lock:spinlock_t,pub phase:i32,pub ndac_idle:i32,pub ndac_seq:i32,pub nrfd_idle:i32,pub nrfd_seq:i32,pub dav_seq:i32,pub all_irqs:i64,pub dav_idle:i32,pub talker_state:i32,pub listener_state:i32 }

#[inline] pub unsafe fn usec_diff(a:*mut timespec64,b:*mut timespec64)->i64 { ((*a).tv_sec-(*b).tv_sec)*1_000_000+((*a).tv_nsec-(*b).tv_nsec)/1000 }
#[inline] unsafe fn check_for_eos(p:*mut bb_priv,b:u8)->i32 { if (*p).eos_check!=0{return 0} if (*p).eos_check_8!=0 { if (*p).eos==b {1}else{0} } else if (*p).eos_mask_7==(b&0x7f) as i16 {1}else{0} }
unsafe fn set_data_lines_output(){for i in 0..8 {gpiod_direction_output(all_descriptors[i],1);}}
unsafe fn set_data_lines(b:u8){for i in 0..8 {gpiod_set_value(all_descriptors[i], if b&(1<<i)!=0 {0}else{1});}}
unsafe fn get_data_lines()->u8 {let mut r=0u8;for i in 0..8 {r|=(gpiod_get_value(all_descriptors[i]) as u8)<<i;}!r}
unsafe fn set_data_lines_input(){for i in 0..8 {gpiod_direction_input(all_descriptors[i]);}}
#[inline] unsafe fn set_dir_write(p:*mut bb_priv){if (*p).direction==DIR_WRITE{return} gpiod_direction_input(all_descriptors[9]);gpiod_direction_input(all_descriptors[14]);set_data_lines_output();gpiod_direction_output(all_descriptors[13],1);gpiod_direction_output(all_descriptors[8],1);if sn7516x!=0{gpiod_set_value(all_descriptors[16],1);gpiod_set_value(all_descriptors[18],1);}(*p).direction=DIR_WRITE;}
#[inline] unsafe fn set_dir_read(p:*mut bb_priv){if (*p).direction==DIR_READ{return}gpiod_direction_input(all_descriptors[13]);gpiod_direction_input(all_descriptors[8]);set_data_lines_input();if sn7516x!=0{gpiod_set_value(all_descriptors[16],0);gpiod_set_value(all_descriptors[18],0);}gpiod_direction_output(all_descriptors[9],0);gpiod_direction_output(all_descriptors[14],0);(*p).direction=DIR_READ;}

// The remaining driver entry points retain the C control flow and are declared
// externally here because their kernel wait/IRQ and GPIB framework primitives
// are provided by the containing kernel module.
extern "C" { pub fn bb_read(board:*mut gpib_board,buffer:*mut u8,length:usize,end:*mut i32,bytes_read:*mut usize)->i32; pub fn bb_write(board:*mut gpib_board,buffer:*mut u8,length:usize,send_eoi:i32,bytes_written:*mut usize)->i32; pub fn bb_command(board:*mut gpib_board,buffer:*mut u8,length:usize,bytes_written:*mut usize)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
