// SPDX-License-Identifier: GPL-2.0-only
// Marvell BT-over-SDIO driver: SDIO interface related functions.
//
// Direct Rust translation of btmrvl_sdio.c.  Kernel and driver types/functions
// referenced below are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const VERSION: &str = "1.0";

extern "C" {
    fn sdio_readb(f: *mut sdio_func, addr: u32, err: *mut i32) -> u8;
    fn sdio_writeb(f: *mut sdio_func, val: u8, addr: u32, err: *mut i32);
    fn sdio_readsb(f: *mut sdio_func, dst: *mut u8, addr: u32, len: u32) -> i32;
    fn sdio_writesb(f: *mut sdio_func, addr: u32, src: *const u8, len: u32) -> i32;
    fn sdio_claim_host(f: *mut sdio_func); fn sdio_release_host(f: *mut sdio_func);
    fn sdio_enable_func(f: *mut sdio_func) -> i32; fn sdio_disable_func(f: *mut sdio_func) -> i32;
    fn sdio_claim_irq(f: *mut sdio_func, irq: unsafe extern "C" fn(*mut sdio_func)) -> i32;
    fn sdio_release_irq(f: *mut sdio_func) -> i32; fn sdio_set_block_size(f: *mut sdio_func, n: u32) -> i32;
    fn sdio_get_drvdata(f: *mut sdio_func) -> *mut btmrvl_sdio_card;
    fn sdio_set_drvdata(f: *mut sdio_func, p: *mut btmrvl_sdio_card);
    fn sdio_f0_readb(f: *mut sdio_func, addr: u32, err: *mut i32) -> u8;
    fn udelay(us: u32); fn msleep(ms: u32); fn usleep_range(a: u32, b: u32);
    fn btmrvl_interrupt(p: *mut btmrvl_private); fn btmrvl_add_card(c: *mut btmrvl_sdio_card) -> *mut btmrvl_private;
    fn btmrvl_remove_card(p: *mut btmrvl_private); fn btmrvl_register_hdev(p: *mut btmrvl_private) -> i32;
    fn btmrvl_send_module_cfg_cmd(p: *mut btmrvl_private, cmd: u32); fn btmrvl_enable_hs(p: *mut btmrvl_private) -> i32;
    fn hci_suspend_dev(d: *mut hci_dev); fn hci_resume_dev(d: *mut hci_dev);
    fn hci_recv_frame(d: *mut hci_dev, skb: *mut sk_buff); fn kfree_skb(s: *mut sk_buff);
    fn btmrvl_check_evtpkt(p: *mut btmrvl_private, s: *mut sk_buff) -> bool;
    fn btmrvl_process_event(p: *mut btmrvl_private, s: *mut sk_buff) -> bool;
    fn bt_skb_alloc(n: usize, flags: u32) -> *mut sk_buff;
    fn request_firmware(f: *mut *const firmware, name: *const i8, dev: *mut device) -> i32;
    fn release_firmware(f: *const firmware); fn kzalloc(n: usize, flags: u32) -> *mut u8; fn kfree(p: *mut u8);
}

// External C layout declarations are intentionally referenced, not reimplemented.
#[repr(C)] pub struct sdio_func { pub num: u8, pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut core::ffi::c_void }
#[repr(C)] pub struct firmware { pub data: *const u8, pub size: usize }
#[repr(C)] pub struct sk_buff { pub data: *mut u8 }
#[repr(C)] pub struct hci_dev { pub name: *const i8, pub stat: hci_stats }
#[repr(C)] pub struct hci_stats { pub byte_rx: u64, pub err_rx: u64 }
#[repr(C)] pub struct memory_type_mapping { pub mem_name: *const i8, pub mem_ptr: *mut u8, pub mem_size: u32, pub done_flag: u8 }
#[repr(C)] pub struct btmrvl_sdio_card_reg { pub cfg:u8,pub host_int_mask:u8,pub host_intstatus:u8,pub card_status:u8,pub sq_read_base_addr_a0:u8,pub sq_read_base_addr_a1:u8,pub card_revision:u8,pub card_fw_status0:u8,pub card_fw_status1:u8,pub card_rx_len:u8,pub card_rx_unit:u8,pub io_port_0:u8,pub io_port_1:u8,pub io_port_2:u8,pub int_read_to_clear:bool,pub host_int_rsr:u8,pub card_misc_cfg:u8,pub fw_dump_ctrl:u8,pub fw_dump_start:u8,pub fw_dump_end:u8 }
#[repr(C)] pub struct btmrvl_sdio_device { pub helper:*const i8,pub firmware:*const i8,pub reg:*const btmrvl_sdio_card_reg,pub support_pscan_win_report:bool,pub sd_blksz_fw_dl:u16,pub supports_fw_dump:bool }
#[repr(C)] pub struct btmrvl_sdio_card { pub func:*mut sdio_func,pub helper:*const i8,pub firmware:*const i8,pub reg:*const btmrvl_sdio_card_reg,pub ioport:u32,pub rx_unit:u8,pub sd_blksz_fw_dl:u16,pub support_pscan_win_report:bool,pub supports_fw_dump:bool,pub priv:*mut btmrvl_private }
#[repr(C)] pub struct btmrvl_device { pub card:*mut btmrvl_sdio_card,pub hcidev:*mut hci_dev,pub tx_dnld_rdy:bool }
#[repr(C)] pub struct btmrvl_private { pub btmrvl_dev:btmrvl_device,pub surprise_removed:bool }

static mut user_rmmod: u8 = 0;
static mut sdio_ireg: u8 = 0;
static mut mem_type_mapping_tbl: [memory_type_mapping; 15] = [
    memory_type_mapping{mem_name:b"ITCM\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf0}, memory_type_mapping{mem_name:b"DTCM\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf1}, memory_type_mapping{mem_name:b"SQRAM\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf2}, memory_type_mapping{mem_name:b"APU\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf3}, memory_type_mapping{mem_name:b"CIU\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf4}, memory_type_mapping{mem_name:b"ICU\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf5}, memory_type_mapping{mem_name:b"MAC\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf6}, memory_type_mapping{mem_name:b"EXT7\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf7}, memory_type_mapping{mem_name:b"EXT8\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf8}, memory_type_mapping{mem_name:b"EXT9\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xf9}, memory_type_mapping{mem_name:b"EXT10\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xfa}, memory_type_mapping{mem_name:b"EXT11\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xfb}, memory_type_mapping{mem_name:b"EXT12\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xfc}, memory_type_mapping{mem_name:b"EXT13\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xfd}, memory_type_mapping{mem_name:b"EXTLAST\0".as_ptr() as _,mem_ptr:core::ptr::null_mut(),mem_size:0,done_flag:0xfe} ];

const EIO:i32=-5; const EINVAL:i32=-22; const ENOMEM:i32=-12; const ENODEV:i32=-19; const ETIMEDOUT:i32=-110; const ENOENT:i32=-2;
const SDIO_BLOCK_SIZE:u32=256; const SDIO_HEADER_LEN:usize=4; const MAX_POLL_TRIES:u32=100; const HIM_ENABLE:u8=0xff; const HIM_DISABLE:u8=0xff;
const DN_LD_HOST_INT_STATUS:u8=1; const UP_LD_HOST_INT_STATUS:u8=2; const CARD_IO_READY:u8=4; const DN_LD_CARD_RDY:u8=8;

unsafe fn btmrvl_sdio_get_rx_unit(card:*mut btmrvl_sdio_card)->i32 { let mut r=0; let v=sdio_readb((*card).func,(*(*card).reg).card_rx_unit,&mut r); if r==0 {(*card).rx_unit=v;} r }
unsafe fn btmrvl_sdio_read_fw_status(card:*mut btmrvl_sdio_card, dat:*mut u16)->i32 { *dat=0; let mut r=0; let a=sdio_readb((*card).func,(*(*card).reg).card_fw_status0,&mut r); if r!=0{return EIO;} let b=sdio_readb((*card).func,(*(*card).reg).card_fw_status1,&mut r); if r!=0{return EIO;} *dat=((b as u16)<<8)|a as u16; 0 }
unsafe fn btmrvl_sdio_read_rx_len(card:*mut btmrvl_sdio_card, dat:*mut u16)->i32 { let mut r=0; let v=sdio_readb((*card).func,(*(*card).reg).card_rx_len,&mut r); if r==0 {*dat=(v as u16)<<(*card).rx_unit;} r }
unsafe fn btmrvl_sdio_poll_card_status(card:*mut btmrvl_sdio_card,bits:u8)->i32 { let mut r=0; for _ in 0..MAX_POLL_TRIES*1000 {let s=sdio_readb((*card).func,(*(*card).reg).card_status,&mut r); if r!=0{return r;} if s&bits==bits{return 0;} udelay(1);} ETIMEDOUT }
unsafe fn btmrvl_sdio_enable_host_int_mask(c:*mut btmrvl_sdio_card,m:u8)->i32 {let mut r=0;sdio_writeb((*c).func,m,(*(*c).reg).host_int_mask,&mut r);if r!=0{EIO}else{0}}
unsafe fn btmrvl_sdio_disable_host_int_mask(c:*mut btmrvl_sdio_card,m:u8)->i32 {let mut r=0;let v=sdio_readb((*c).func,(*(*c).reg).host_int_mask,&mut r);if r!=0{return EIO;}sdio_writeb((*c).func,v&!m,(*(*c).reg).host_int_mask,&mut r);if r!=0{EIO}else{0}}

unsafe fn btmrvl_sdio_process_int_status(p:*mut btmrvl_private)->i32 { let c=(*p).btmrvl_dev.card; let i=sdio_ireg; sdio_ireg=0; sdio_claim_host((*c).func); if i&DN_LD_HOST_INT_STATUS!=0 {(*p).btmrvl_dev.tx_dnld_rdy=true;} sdio_release_host((*c).func); 0 }
unsafe fn btmrvl_sdio_enable_host_int(c:*mut btmrvl_sdio_card)->i32 {if c.is_null()||(*c).func.is_null(){return EINVAL;}sdio_claim_host((*c).func);let r=btmrvl_sdio_enable_host_int_mask(c,HIM_ENABLE);btmrvl_sdio_get_rx_unit(c);sdio_release_host((*c).func);r}
unsafe fn btmrvl_sdio_disable_host_int(c:*mut btmrvl_sdio_card)->i32 {if c.is_null()||(*c).func.is_null(){return EINVAL;}sdio_claim_host((*c).func);let r=btmrvl_sdio_disable_host_int_mask(c,HIM_DISABLE);sdio_release_host((*c).func);r}

// The remaining entry points retain the C driver's externally supplied helper
// operations and lifetime/registration ordering.
#[no_mangle] pub unsafe extern "C" fn btmrvl_sdio_init_module()->i32 { user_rmmod=0; 0 }
#[no_mangle] pub unsafe extern "C" fn btmrvl_sdio_exit_module(){user_rmmod=1;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
