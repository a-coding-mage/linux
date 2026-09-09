/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of comedidev.h; dependencies are supplied externally. */

pub const COMEDI_NUM_BOARD_MINORS: u32 = 0x30;
pub const COMEDI_TIMEOUT_MS: u32 = 1000;
pub const fn range(a: f64,b:f64,flags:i32)->(f64,f64,i32){(a*1e6,b*1e6,flags)}
extern "C" {
    pub static range_bipolar10: comedi_lrange; pub static range_bipolar5: comedi_lrange; pub static range_bipolar2_5: comedi_lrange;
    pub static range_unipolar10: comedi_lrange; pub static range_unipolar5: comedi_lrange; pub static range_unipolar2_5: comedi_lrange;
    pub static range_0_20mA: comedi_lrange; pub static range_4_20mA: comedi_lrange; pub static range_0_32mA: comedi_lrange; pub static range_unknown: comedi_lrange;
}

#[inline] pub const fn comedi_version(a: u32, b: u32, c: u32) -> u32 { (a << 16) + (b << 8) + c }

#[repr(C)]
pub struct comedi_subdevice {
    pub device: *mut comedi_device, pub index: i32, pub type_: i32, pub n_chan: i32,
    pub subdev_flags: i32, pub len_chanlist: i32, pub private: *mut core::ffi::c_void,
    pub async_: *mut comedi_async, pub lock: *mut core::ffi::c_void, pub busy: *mut core::ffi::c_void,
    pub runflags: u32, pub spin_lock: spinlock_t, pub io_bits: u32, pub maxdata: u32,
    pub maxdata_list: *const u32, pub range_table: *const comedi_lrange,
    pub range_table_list: *const *const comedi_lrange, pub chanlist: *mut u32,
    pub insn_read: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>,
    pub insn_write: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>,
    pub insn_bits: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>,
    pub insn_config: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32>,
    pub do_cmd: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32>,
    pub do_cmdtest: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_cmd)->i32>,
    pub poll: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32>,
    pub cancel: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32>,
    pub buf_change: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice)->i32>,
    pub munge: Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut core::ffi::c_void,u32,u32)>,
    pub async_dma_dir: dma_data_direction, pub state: u32, pub class_dev: *mut device, pub minor: i32,
    pub readback: *mut u32,
}

#[repr(C)] pub struct comedi_buf_page { pub virt_addr: *mut core::ffi::c_void, pub dma_addr: dma_addr_t }
#[repr(C)] pub struct comedi_buf_map { pub dma_hw_dev: *mut device, pub page_list: *mut comedi_buf_page, pub n_pages: u32, pub dma_dir: dma_data_direction, pub refcount: kref }
#[repr(C)] pub struct comedi_async {
    pub prealloc_bufsz:u32, pub buf_map:*mut comedi_buf_map, pub max_bufsize:u32,
    pub buf_write_count:u32, pub buf_write_alloc_count:u32, pub buf_read_count:u32, pub buf_read_alloc_count:u32,
    pub buf_write_ptr:u32, pub buf_read_ptr:u32, pub cur_chan:u32, pub scans_done:u32, pub scan_progress:u32,
    pub munge_chan:u32, pub munge_count:u32, pub munge_ptr:u32, pub events:u32, pub cmd:comedi_cmd,
    pub wait_head:wait_queue_head_t, pub run_complete:completion, pub run_active:refcount_t, pub cb_mask:u32,
    pub inttrig:Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,u32)->i32>,
}

#[repr(C)] pub struct comedi_driver { pub next:*mut comedi_driver, pub driver_name:*const i8, pub module:*mut module,
    pub attach:Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_devconfig)->i32>, pub detach:Option<unsafe extern "C" fn(*mut comedi_device)>,
    pub auto_attach:Option<unsafe extern "C" fn(*mut comedi_device,usize)->i32>, pub num_names:u32, pub board_name:*const *const i8, pub offset:i32 }
#[repr(C)] pub struct comedi_device {
    pub use_count:i32, pub driver:*mut comedi_driver, pub pacer:*mut comedi_8254, pub private:*mut core::ffi::c_void,
    pub class_dev:*mut device, pub minor:i32, pub detach_count:u32, pub hw_dev:*mut device, pub board_name:*const i8, pub board_ptr:*const core::ffi::c_void,
    pub attached:u32, pub ioenabled:u32, pub spinlock:spinlock_t, pub mutex:mutex, pub attach_lock:rw_semaphore, pub refcount:kref,
    pub n_subdevices:i32, pub subdevices:*mut comedi_subdevice, pub mmio:*mut core::ffi::c_void, pub iobase:usize, pub iolen:usize, pub irq:u32,
    pub read_subdev:*mut comedi_subdevice, pub write_subdev:*mut comedi_subdevice, pub async_queue:*mut fasync_struct,
    pub open:Option<unsafe extern "C" fn(*mut comedi_device)->i32>, pub close:Option<unsafe extern "C" fn(*mut comedi_device)>,
    pub insn_device_config:Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_insn,*mut u32)->i32>,
    pub get_valid_routes:Option<unsafe extern "C" fn(*mut comedi_device,u32,*mut u32)->u32>,
}

#[repr(C)] pub struct comedi_lrange { pub length:i32, pub range:[comedi_krange;0] }
#[repr(C)] pub enum comedi_cb { EOS=1, EOA=2, BLOCK=4, EOBUF=8, ERROR=16, OVERFLOW=32 }
pub const COMEDI_CB_ERROR_MASK:u32 = 16|32; pub const COMEDI_CB_CANCEL_MASK:u32 = 2|16|32;

extern "C" {
    pub fn comedi_event(*mut comedi_device,*mut comedi_subdevice); pub fn comedi_dev_get_from_minor(u32)->*mut comedi_device; pub fn comedi_dev_put(*mut comedi_device)->bool;
    pub fn comedi_is_subdevice_running(*mut comedi_subdevice)->bool; pub fn comedi_get_is_subdevice_running(*mut comedi_subdevice)->bool; pub fn comedi_put_is_subdevice_running(*mut comedi_subdevice);
    pub fn comedi_alloc_spriv(*mut comedi_subdevice,usize)->*mut core::ffi::c_void; pub fn comedi_set_spriv_auto_free(*mut comedi_subdevice);
    pub fn comedi_check_chanlist(*mut comedi_subdevice,i32,*mut u32)->i32; pub fn comedi_set_hw_dev(*mut comedi_device,*mut device)->i32;
    pub fn comedi_buf_write_alloc(*mut comedi_subdevice,u32)->u32; pub fn comedi_buf_write_free(*mut comedi_subdevice,u32)->u32; pub fn comedi_buf_read_n_available(*mut comedi_subdevice)->u32;
    pub fn comedi_buf_read_alloc(*mut comedi_subdevice,u32)->u32; pub fn comedi_buf_read_free(*mut comedi_subdevice,u32)->u32;
    pub fn comedi_buf_write_samples(*mut comedi_subdevice,*const core::ffi::c_void,u32)->u32; pub fn comedi_buf_read_samples(*mut comedi_subdevice,*mut core::ffi::c_void,u32)->u32;
    pub fn comedi_driver_register(*mut comedi_driver)->i32; pub fn comedi_driver_unregister(*mut comedi_driver);
    pub fn comedi_timeout(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,Option<unsafe extern "C" fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32>,usize)->i32;
    pub fn comedi_handle_events(*mut comedi_device,*mut comedi_subdevice)->u32;
    pub fn comedi_dio_insn_config(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32,u32)->i32;
    pub fn comedi_dio_update_state(*mut comedi_subdevice,*mut u32)->u32; pub fn comedi_bytes_per_scan_cmd(*mut comedi_subdevice,*mut comedi_cmd)->u32;
    pub fn comedi_bytes_per_scan(*mut comedi_subdevice)->u32; pub fn comedi_nscans_left(*mut comedi_subdevice,u32)->u32; pub fn comedi_nsamples_left(*mut comedi_subdevice,u32)->u32;
    pub fn comedi_inc_scan_progress(*mut comedi_subdevice,u32); pub fn comedi_alloc_devpriv(*mut comedi_device,usize)->*mut core::ffi::c_void;
    pub fn comedi_alloc_subdevices(*mut comedi_device,i32)->i32; pub fn comedi_alloc_subdev_readback(*mut comedi_subdevice)->i32;
    pub fn comedi_readback_insn_read(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,*mut u32)->i32;
    pub fn comedi_load_firmware(*mut comedi_device,*mut device,*const i8,Option<unsafe extern "C" fn(*mut comedi_device,*const u8,usize,usize)->i32>,usize)->i32;
    pub fn __comedi_check_request_region(*mut comedi_device,usize,usize,usize,usize,usize)->i32; pub fn comedi_check_request_region(*mut comedi_device,usize,usize,usize,usize,usize)->i32;
    pub fn comedi_legacy_detach(*mut comedi_device); pub fn comedi_auto_config(*mut device,*mut comedi_driver,usize)->i32; pub fn comedi_auto_unconfig(*mut device);
}

#[inline] pub unsafe fn comedi_range_is_bipolar(s:*mut comedi_subdevice,r:usize)->bool { (*(*s).range_table).range[r].min < 0 }
#[inline] pub unsafe fn comedi_range_is_unipolar(s:*mut comedi_subdevice,r:usize)->bool { (*(*s).range_table).range[r].min >= 0 }
#[inline] pub unsafe fn comedi_range_is_external(s:*mut comedi_subdevice,r:usize)->bool { ((*(*s).range_table).range[r].flags & RF_EXTERNAL) != 0 }
#[inline] pub unsafe fn comedi_chan_range_is_bipolar(s:*mut comedi_subdevice,c:usize,r:usize)->bool { (*(*(*s).range_table_list.add(c))).range[r].min < 0 }
#[inline] pub unsafe fn comedi_chan_range_is_unipolar(s:*mut comedi_subdevice,c:usize,r:usize)->bool { (*(*(*s).range_table_list.add(c))).range[r].min >= 0 }
#[inline] pub unsafe fn comedi_chan_range_is_external(s:*mut comedi_subdevice,c:usize,r:usize)->bool { ((*(*(*s).range_table_list.add(c))).range[r].flags & RF_EXTERNAL) != 0 }
#[inline] pub unsafe fn comedi_offset_munge(s:*mut comedi_subdevice,v:u32)->u32 { v ^ (*s).maxdata ^ ((*s).maxdata >> 1) }
#[inline] pub unsafe fn comedi_bytes_per_sample(s:*mut comedi_subdevice)->u32 { if (*s).subdev_flags & SDF_LSAMPL != 0 {4} else {2} }
#[inline] pub unsafe fn comedi_sample_shift(s:*mut comedi_subdevice)->u32 { if (*s).subdev_flags & SDF_LSAMPL != 0 {2} else {1} }
#[inline] pub unsafe fn comedi_bytes_to_samples(s:*mut comedi_subdevice,n:u32)->u32 { n >> comedi_sample_shift(s) }
#[inline] pub unsafe fn comedi_samples_to_bytes(s:*mut comedi_subdevice,n:u32)->u32 { n << comedi_sample_shift(s) }
#[inline] pub unsafe fn comedi_buf_n_bytes_ready(s:*mut comedi_subdevice)->u32 { (*(*s).async_).buf_write_count.wrapping_sub((*(*s).async_).buf_read_count) }
#[inline] pub unsafe fn comedi_check_trigger_src(src:*mut u32,flags:u32)->i32 { let old=*src; *src=old&flags; if *src==TRIG_INVALID || *src!=old {-EINVAL} else {0} }
#[inline] pub fn comedi_check_trigger_is_unique(src:u32)->i32 { if src & src.wrapping_sub(1) != 0 {-EINVAL} else {0} }
#[inline] pub unsafe fn comedi_check_trigger_arg_is(a:*mut u32,v:u32)->i32 { if *a!=v {*a=v;-EINVAL} else {0} }
#[inline] pub unsafe fn comedi_check_trigger_arg_min(a:*mut u32,v:u32)->i32 { if *a<v {*a=v;-EINVAL} else {0} }
#[inline] pub unsafe fn comedi_check_trigger_arg_max(a:*mut u32,v:u32)->i32 { if *a>v {*a=v;-EINVAL} else {0} }
#[inline] pub unsafe fn __comedi_request_region(d:*mut comedi_device,start:usize,len:usize)->i32 { __comedi_check_request_region(d,start,len,0,usize::MAX,1) }
#[inline] pub unsafe fn comedi_request_region(d:*mut comedi_device,start:usize,len:usize)->i32 { comedi_check_request_region(d,start,len,0,usize::MAX,1) }

/* External kernel/comedi types and constants referenced above are supplied by other translated headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
