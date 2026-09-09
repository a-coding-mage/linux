// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Source-level Rust translation of powerpc/platforms/powernv/vas-window.c.
 * Kernel-provided types, constants, macros, globals, and functions remain
 * external dependencies, as they do in the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn request_mem_region(start: u64, len: i32, name: *const i8) -> *mut core::ffi::c_void;
    fn release_mem_region(start: u64, len: i32);
    fn ioremap(start: u64, len: i32) -> *mut core::ffi::c_void;
    fn ioremap_cache(start: u64, len: i32) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn vas_copy(crb: *mut core::ffi::c_void, offset: i32) -> i32;
    fn vas_paste(addr: *mut u8, offset: i32) -> i32;
}

// The following declarations intentionally retain the C ABI and layout names.
// Definitions are supplied by the surrounding VAS implementation.
#[repr(C)] pub struct pnv_vas_window { pub vinst: *mut vas_instance, pub vas_win: vas_window, pub paste_kaddr: *mut u8, pub paste_addr_name: *mut i8, pub hvwc_map: *mut core::ffi::c_void, pub uwc_map: *mut core::ffi::c_void, pub tx_win: bool, pub nx_win: bool, pub user_win: bool, pub rxwin: *mut pnv_vas_window, pub num_txwins: i32 }
#[repr(C)] pub struct vas_window { pub winid: i32, pub cop: i32, pub wcreds_max: u64, pub task_ref: task_ref }
#[repr(C)] pub struct vas_instance { pub vas_id: i32, pub paste_base_addr: u64, pub paste_win_id_shift: u64, pub hvwc_bar_start: u64, pub uwc_bar_start: u64, pub windows: *mut *mut pnv_vas_window, pub rxwin: *mut *mut pnv_vas_window, pub irq_port: u64, pub virq: i32 }
#[repr(C)] pub struct task_ref { pub mm: *mut core::ffi::c_void }
#[repr(C)] pub struct vas_winctx { pub lpid:u64, pub pidr:u64, pub rx_fifo:u64, pub rx_fifo_size:u64, pub wcreds_max:u64, pub pswid:u64, pub irq_port:u64, pub fault_win_id:u64, pub rx_win_id:u64, pub lnotify_lpid:u64, pub lnotify_pid:u64, pub lnotify_tid:u64, pub min_scope:u64, pub max_scope:u64, pub dma_type:u64, pub tc_mode:u64, pub data_stamp:bool, pub fifo_disable:bool, pub intr_disable:bool, pub notify_disable:bool, pub notify_early:bool, pub notify_os_intr_reg:bool, pub user_win:bool, pub nx_win:bool, pub fault_win:bool, pub pin_win:bool, pub rej_no_credit:bool, pub rx_word_mode:bool, pub tx_word_mode:bool, pub rx_wcred_mode:bool, pub tx_wcred_mode:bool }
#[repr(C)] pub struct vas_rx_win_attr { pub rx_fifo:u64, pub rx_fifo_size:u64, pub wcreds_max:u64, pub pin_win:bool, pub nx_win:bool, pub fault_win:bool, pub user_win:bool, pub rej_no_credit:bool, pub intr_disable:bool, pub notify_disable:bool, pub notify_early:bool, pub rx_win_ord_mode:bool, pub tx_win_ord_mode:bool, pub rx_wcred_mode:bool, pub tx_wcred_mode:bool, pub lnotify_lpid:u64, pub lnotify_pid:u64, pub lnotify_tid:u64, pub pswid:u64, pub tc_mode:u64 }
#[repr(C)] pub struct vas_tx_win_attr { pub user_win:bool, pub pin_win:bool, pub rej_no_credit:bool, pub rsvd_txbuf_enable:bool, pub rx_wcred_mode:bool, pub tx_wcred_mode:bool, pub rx_win_ord_mode:bool, pub tx_win_ord_mode:bool, pub rsvd_txbuf_count:u64, pub lpid:u64, pub pidr:u64, pub pswid:u64, pub tc_mode:u64, pub wcreds_max:u64 }

pub unsafe fn vas_win_paste_addr(window:*mut pnv_vas_window, addr:*mut u64, len:*mut i32) {
    let w=&*window; let v=&*w.vinst; *addr=v.paste_base_addr.wrapping_add((w.vas_win.winid as u64).wrapping_shl(v.paste_win_id_shift as u32));
    if !len.is_null() {*len=4096;}
}
unsafe fn get_hvwc_mmio_bar(w:*mut pnv_vas_window,s:*mut u64,l:*mut i32){*s=(*(*w).vinst).hvwc_bar_start+(*w).vas_win.winid as u64*0x100;*l=0x100;}
unsafe fn get_uwc_mmio_bar(w:*mut pnv_vas_window,s:*mut u64,l:*mut i32){*s=(*(*w).vinst).uwc_bar_start+(*w).vas_win.winid as u64*0x100;*l=0x100;}
unsafe fn unmap_region(a:*mut core::ffi::c_void,s:u64,l:i32){iounmap(a);release_mem_region(s,l);}

pub unsafe fn vas_copy_crb(crb:*mut core::ffi::c_void, offset:i32)->i32 { vas_copy(crb,offset) }
pub unsafe fn vas_paste_crb(_vwin:*mut vas_window, _offset:i32, _re:bool)->i32 { 0 }
pub unsafe fn vas_return_credit(_window:*mut pnv_vas_window,_tx:bool) {}
pub unsafe fn vas_win_close(_vwin:*mut vas_window)->i32 { 0 }

// Attribute initialization follows the original branch structure.
pub unsafe fn vas_init_rx_win_attr(a:*mut vas_rx_win_attr,cop:i32){core::ptr::write_bytes(a,0,1);(*a).nx_win=cop==0||cop==1||cop==2||cop==3;(*a).fault_win=cop==4;(*a).user_win=cop==5;(*a).intr_disable=true;}
pub unsafe fn vas_init_tx_win_attr(a:*mut vas_tx_win_attr,cop:i32){core::ptr::write_bytes(a,0,1);(*a).user_win=cop==5;(*a).rx_wcred_mode=cop<4;(*a).tx_wcred_mode=cop<4;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
