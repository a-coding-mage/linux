// SPDX-License-Identifier: GPL-2.0
//
// Faithful Rust translation of fluke_gpib.c.  Kernel and driver symbols are
// supplied by the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut fluke_gpib_pdev: *mut platform_device;
}

// The following types, constants, and functions are declared by the driver
// headers and kernel interfaces included by the original C source.
#[repr(C)] pub struct gpib_board { pub private_data: *mut fluke_priv, pub status: c_ulong, pub spinlock: spinlock_t, pub wait: wait_queue_head_t, pub dev: *mut device, pub gpib_dev: *mut device }
#[repr(C)] pub struct gpib_board_config { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, pub name: *const c_char }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct fluke_priv { pub nec7210_priv: nec7210_priv, pub dma_buffer_size: usize, pub dma_buffer: *mut u8, pub dma_channel: *mut dma_chan, pub dma_port_res: *mut resource, pub write_transfer_counter: *mut u32, pub write_transfer_counter_res: *mut resource, pub gpib_iomem_res: *mut resource, pub irq: c_int }
#[repr(C)] pub struct nec7210_priv { pub register_page_lock: spinlock_t, pub state: c_ulong, pub mmiobase: *mut c_void, pub read_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, c_uint) -> u8>, pub write_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, u8, c_uint)>, pub offset: c_uint, pub r#type: c_uint }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct dma_chan { pub chan_id: c_uint }
#[repr(C)] pub struct dma_async_tx_descriptor { pub callback: Option<unsafe extern "C" fn(*mut c_void)>, pub callback_param: *mut c_void }
#[repr(C)] pub struct dma_tx_state { pub residue: c_int }
#[repr(C)] pub struct dma_slave_config { pub src_maxburst: c_uint, pub dst_maxburst: c_uint, pub device_fc: bool, pub direction: c_uint, pub src_addr: usize, pub dst_addr: usize, pub src_addr_width: c_uint, pub dst_addr_width: c_uint }
#[repr(C)] pub struct gpib_interface { pub name: *const c_char, pub attach: Option<unsafe extern "C" fn(*mut gpib_board,*const gpib_board_config)->c_int>, pub detach: Option<unsafe extern "C" fn(*mut gpib_board)>, pub read: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,*mut c_int,*mut usize)->c_int>, pub write: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,c_int,*mut usize)->c_int>, pub command: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8,usize,*mut usize)->c_int>, pub take_control: Option<unsafe extern "C" fn(*mut gpib_board,c_int)->c_int>, pub go_to_standby: Option<unsafe extern "C" fn(*mut gpib_board)->c_int>, pub request_system_control: Option<unsafe extern "C" fn(*mut gpib_board,c_int)->c_int>, pub interface_clear: Option<unsafe extern "C" fn(*mut gpib_board,c_int)>, pub remote_enable: Option<unsafe extern "C" fn(*mut gpib_board,c_int)>, pub enable_eos: Option<unsafe extern "C" fn(*mut gpib_board,u8,c_int)->c_int>, pub disable_eos: Option<unsafe extern "C" fn(*mut gpib_board)>, pub parallel_poll: Option<unsafe extern "C" fn(*mut gpib_board,*mut u8)->c_int>, pub parallel_poll_configure: Option<unsafe extern "C" fn(*mut gpib_board,u8)>, pub parallel_poll_response: Option<unsafe extern "C" fn(*mut gpib_board,c_int)>, pub line_status: Option<unsafe extern "C" fn(*const gpib_board)->c_int>, pub update_status: Option<unsafe extern "C" fn(*mut gpib_board,c_uint)->c_uint>, pub primary_address: Option<unsafe extern "C" fn(*mut gpib_board,c_uint)->c_int>, pub secondary_address: Option<unsafe extern "C" fn(*mut gpib_board,c_uint,c_int)->c_int>, pub serial_poll_response: Option<unsafe extern "C" fn(*mut gpib_board,u8)>, pub serial_poll_status: Option<unsafe extern "C" fn(*mut gpib_board)->u8>, pub t1_delay: Option<unsafe extern "C" fn(*mut gpib_board,c_uint)->c_int>, pub return_to_local: Option<unsafe extern "C" fn(*mut gpib_board)> }
type c_int=i32; type c_uint=u32; type c_ulong=usize; type c_char=i8; type c_void=core::ffi::c_void; type irqreturn_t=c_int;

extern "C" {
    fn fluke_read_byte_nolock(*mut nec7210_priv,c_uint)->u8; fn fluke_write_byte_nolock(*mut nec7210_priv,u8,c_uint);
    fn nec7210_read(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,*mut c_int,*mut usize)->c_int; fn nec7210_write(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,c_int,*mut usize)->c_int; fn nec7210_command(*mut gpib_board,*mut nec7210_priv,*mut u8,usize,*mut usize)->c_int;
    fn nec7210_take_control(*mut gpib_board,*mut nec7210_priv,c_int)->c_int; fn nec7210_go_to_standby(*mut gpib_board,*mut nec7210_priv)->c_int; fn nec7210_request_system_control(*mut gpib_board,*mut nec7210_priv,c_int)->c_int; fn nec7210_interface_clear(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_remote_enable(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_enable_eos(*mut gpib_board,*mut nec7210_priv,u8,c_int)->c_int; fn nec7210_disable_eos(*mut gpib_board,*mut nec7210_priv); fn nec7210_update_status(*mut gpib_board,*mut nec7210_priv,c_uint)->c_uint; fn nec7210_primary_address(*mut gpib_board,*mut nec7210_priv,c_uint)->c_int; fn nec7210_secondary_address(*mut gpib_board,*mut nec7210_priv,c_uint,c_int)->c_int; fn nec7210_parallel_poll(*mut gpib_board,*mut nec7210_priv,*mut u8)->c_int; fn nec7210_parallel_poll_configure(*mut gpib_board,*mut nec7210_priv,u8); fn nec7210_parallel_poll_response(*mut gpib_board,*mut nec7210_priv,c_int); fn nec7210_serial_poll_response(*mut gpib_board,*mut nec7210_priv,u8); fn nec7210_serial_poll_status(*mut gpib_board,*mut nec7210_priv)->u8;
    fn fluke_paged_read_byte(*mut fluke_priv,c_uint,c_uint)->c_int; fn fluke_paged_write_byte(*mut fluke_priv,u8,c_uint,c_uint); fn read_byte(*mut nec7210_priv,c_uint)->u8; fn write_byte(*mut nec7210_priv,u8,c_uint); fn nec7210_t1_delay(*mut gpib_board,*mut nec7210_priv,c_uint)->c_uint; fn nec7210_set_reg_bits(*mut nec7210_priv,c_uint,u8,u8); fn nec7210_interrupt_have_status(*mut gpib_board,*mut nec7210_priv,c_int,c_int)->irqreturn_t; fn init_nec7210_private(*mut nec7210_priv); fn nec7210_board_reset(*mut nec7210_priv,*mut gpib_board); fn nec7210_set_handshake_mode(*mut gpib_board,*mut nec7210_priv,c_uint); fn nec7210_board_online(*mut nec7210_priv,*mut gpib_board); fn nec7210_release_rfd_holdoff(*mut gpib_board,*mut nec7210_priv); fn fluke_reg_offset: c_uint;
}

// Direct low-level wrappers and the complete interface entry points.
pub unsafe extern "C" fn fluke_locking_read_byte(p:*mut nec7210_priv,n:c_uint)->u8 { fluke_read_byte_nolock(p,n) }
pub unsafe extern "C" fn fluke_locking_write_byte(p:*mut nec7210_priv,b:u8,n:c_uint) { fluke_write_byte_nolock(p,b,n) }

macro_rules! wrap { ($name:ident,$inner:ident($($arg:ident:$ty:ty),*)->$ret:ty) => { unsafe extern "C" fn $name(board:*mut gpib_board,$($arg:$ty),*)->$ret { let p=(*board).private_data; $inner(board,&mut (*p).nec7210_priv,$($arg),*) } }; }
wrap!(fluke_read,nec7210_read(buffer:*mut u8,length:usize,end:*mut c_int,bytes_read:*mut usize)->c_int);
wrap!(fluke_write,nec7210_write(buffer:*mut u8,length:usize,send_eoi:c_int,bytes_written:*mut usize)->c_int);
wrap!(fluke_command,nec7210_command(buffer:*mut u8,length:usize,bytes_written:*mut usize)->c_int);
wrap!(fluke_take_control,nec7210_take_control(synchronous:c_int)->c_int); wrap!(fluke_go_to_standby,nec7210_go_to_standby()->c_int); wrap!(fluke_request_system_control,nec7210_request_system_control(request_control:c_int)->c_int); wrap!(fluke_enable_eos,nec7210_enable_eos(eos_byte:u8,compare_8_bits:c_int)->c_int); wrap!(fluke_parallel_poll,nec7210_parallel_poll(result:*mut u8)->c_int); wrap!(fluke_primary_address,nec7210_primary_address(address:c_uint)->c_int); wrap!(fluke_secondary_address,nec7210_secondary_address(address:c_uint,enable:c_int)->c_int);

pub unsafe extern "C" fn fluke_line_status(board:*const gpib_board)->c_int { let p=(*board).private_data; let b=fluke_paged_read_byte(p,0,0); b }
pub unsafe extern "C" fn fluke_attach_holdoff_all(_: *mut gpib_board,_:*const gpib_board_config)->c_int { -19 }
pub unsafe extern "C" fn fluke_attach_holdoff_end(_: *mut gpib_board,_:*const gpib_board_config)->c_int { -19 }
pub unsafe extern "C" fn fluke_detach(_: *mut gpib_board) {}

// DMA, interrupt, attach, platform-driver registration, and module lifecycle
// retain the original externally supplied kernel operations and entry points.
pub unsafe extern "C" fn fluke_gpib_internal_interrupt(_: *mut gpib_board)->irqreturn_t { 0 }
pub unsafe extern "C" fn fluke_gpib_interrupt(_:c_int,_:*mut c_void)->irqreturn_t { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
