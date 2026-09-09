/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/fb.h. C preprocessor configuration is represented by
// conditional comments where it cannot be selected from this file alone.

pub const FBIO_CURSOR: u32 = 0; // _IOWR('F', 0x08, struct fb_cursor_user)
pub const FB_DPMS_ACTIVE_OFF: u32 = 1;
pub const FB_DPMS_SUSPEND: u32 = 2;
pub const FB_DPMS_STANDBY: u32 = 4;
pub const FB_DISP_DDI: u32 = 1; pub const FB_DISP_ANA_700_300: u32 = 2;
pub const FB_DISP_ANA_714_286: u32 = 4; pub const FB_DISP_ANA_1000_400: u32 = 8;
pub const FB_DISP_ANA_700_000: u32 = 16; pub const FB_DISP_MONO: u32 = 32;
pub const FB_DISP_RGB: u32 = 64; pub const FB_DISP_MULTI: u32 = 128;
pub const FB_DISP_UNKNOWN: u32 = 256;
pub const FB_SIGNAL_NONE: u32 = 0; pub const FB_SIGNAL_BLANK_BLANK: u32 = 1;
pub const FB_SIGNAL_SEPARATE: u32 = 2; pub const FB_SIGNAL_COMPOSITE: u32 = 4;
pub const FB_SIGNAL_SYNC_ON_GREEN: u32 = 8; pub const FB_SIGNAL_SERRATION_ON: u32 = 16;
pub const FB_MISC_PRIM_COLOR: u32 = 1; pub const FB_MISC_1ST_DETAIL: u32 = 2;
pub const FB_MISC_HDMI: u32 = 4;

#[repr(C)] pub struct fb_chroma { pub redx:u32,pub greenx:u32,pub bluex:u32,pub whitex:u32,pub redy:u32,pub greeny:u32,pub bluey:u32,pub whitey:u32 }
#[repr(C)] pub struct fb_monspecs { pub chroma:fb_chroma, pub modedb:*mut fb_videomode, pub manufacturer:[u8;4], pub monitor:[u8;14], pub serial_no:[u8;14], pub ascii:[u8;14], pub modedb_len:u32,pub model:u32,pub serial:u32,pub year:u32,pub week:u32,pub hfmin:u32,pub hfmax:u32,pub dclkmin:u32,pub dclkmax:u32,pub input:u16,pub dpms:u16,pub signal:u16,pub vfmin:u16,pub vfmax:u16,pub gamma:u16,pub gtf:u16,pub misc:u16,pub version:u8,pub revision:u8,pub max_x:u8,pub max_y:u8 }
#[repr(C)] pub struct fb_cmap_user { pub start:u32,pub len:u32,pub red:*mut u16,pub green:*mut u16,pub blue:*mut u16,pub transp:*mut u16 }
#[repr(C)] pub struct fb_image_user { pub dx:u32,pub dy:u32,pub width:u32,pub height:u32,pub fg_color:u32,pub bg_color:u32,pub depth:u8,pub data:*const i8,pub cmap:fb_cmap_user }
#[repr(C)] pub struct fb_cursor_user { pub set:u16,pub enable:u16,pub rop:u16,pub mask:*const i8,pub hot:fbcurpos,pub image:fb_image_user }

pub const FB_MAX_BLIT_WIDTH:usize=64; pub const FB_MAX_BLIT_HEIGHT:usize=128;
#[repr(C)] pub struct fb_event { pub info:*mut fb_info,pub data:*mut core::ffi::c_void }
#[repr(C)] pub struct fb_blit_caps { pub x:[usize;1],pub y:[usize;2],pub len:u32,pub flags:u32 }

pub const FB_PIXMAP_DEFAULT:u32=1; pub const FB_PIXMAP_SYSTEM:u32=2; pub const FB_PIXMAP_IO:u32=4; pub const FB_PIXMAP_SYNC:u32=256;
#[repr(C)] pub struct fb_pixmap { pub addr:*mut u8,pub size:u32,pub offset:u32,pub buf_align:u32,pub scan_align:u32,pub access_align:u32,pub flags:u32,pub blit_x:[usize;1],pub blit_y:[usize;2],pub writeio:Option<unsafe extern "C" fn(*mut fb_info,*mut core::ffi::c_void,*mut core::ffi::c_void,u32)>,pub readio:Option<unsafe extern "C" fn(*mut fb_info,*mut core::ffi::c_void,*mut core::ffi::c_void,u32)> }

#[cfg(any())] pub struct fb_deferred_io_pageref { pub page:*mut page,pub offset:usize,pub list:list_head }
#[cfg(any())] pub struct fb_deferred_io { pub delay:usize,pub sort_pagereflist:bool,pub get_page:Option<unsafe extern "C" fn(*mut fb_info,usize)->*mut page>,pub deferred_io:Option<unsafe extern "C" fn(*mut fb_info,*mut list_head)> }
#[repr(C)] pub struct fb_ops { pub owner:*mut module,pub fb_open:Option<unsafe extern "C" fn(*mut fb_info,i32)->i32>,pub fb_release:Option<unsafe extern "C" fn(*mut fb_info,i32)->i32>,pub fb_read:Option<unsafe extern "C" fn(*mut fb_info,*mut i8,usize,*mut loff_t)->isize>,pub fb_write:Option<unsafe extern "C" fn(*mut fb_info,*const i8,usize,*mut loff_t)->isize>,pub fb_check_var:Option<unsafe extern "C" fn(*mut fb_var_screeninfo,*mut fb_info)->i32>,pub fb_set_par:Option<unsafe extern "C" fn(*mut fb_info)->i32>,pub fb_setcolreg:Option<unsafe extern "C" fn(u32,u32,u32,u32,u32,*mut fb_info)->i32>,pub fb_setcmap:Option<unsafe extern "C" fn(*mut fb_cmap,*mut fb_info)->i32>,pub fb_blank:Option<unsafe extern "C" fn(i32,*mut fb_info)->i32>,pub fb_pan_display:Option<unsafe extern "C" fn(*mut fb_var_screeninfo,*mut fb_info)->i32>,pub fb_fillrect:Option<unsafe extern "C" fn(*mut fb_info,*const fb_fillrect)>,pub fb_copyarea:Option<unsafe extern "C" fn(*mut fb_info,*const fb_copyarea)>,pub fb_imageblit:Option<unsafe extern "C" fn(*mut fb_info,*const fb_image)>,pub fb_cursor:Option<unsafe extern "C" fn(*mut fb_info,*mut fb_cursor)->i32>,pub fb_sync:Option<unsafe extern "C" fn(*mut fb_info)->i32>,pub fb_ioctl:Option<unsafe extern "C" fn(*mut fb_info,u32,usize)->i32>,pub fb_compat_ioctl:Option<unsafe extern "C" fn(*mut fb_info,u32,usize)->i32>,pub fb_mmap:Option<unsafe extern "C" fn(*mut fb_info,*mut vm_area_struct)->i32>,pub fb_get_caps:Option<unsafe extern "C" fn(*mut fb_info,*mut fb_blit_caps,*mut fb_var_screeninfo)>,pub fb_destroy:Option<unsafe extern "C" fn(*mut fb_info)> }

pub const FBINFO_HWACCEL_DISABLED:u32=0x0002; pub const FBINFO_VIRTFB:u32=0x0004; pub const FBINFO_PARTIAL_PAN_OK:u32=0x0040; pub const FBINFO_READS_FAST:u32=0x0080;
pub const FBINFO_HWACCEL_NONE:u32=0; pub const FBINFO_HWACCEL_COPYAREA:u32=0x100; pub const FBINFO_HWACCEL_FILLRECT:u32=0x200; pub const FBINFO_HWACCEL_IMAGEBLIT:u32=0x400; pub const FBINFO_HWACCEL_ROTATE:u32=0x800; pub const FBINFO_HWACCEL_XPAN:u32=0x1000; pub const FBINFO_HWACCEL_YPAN:u32=0x2000; pub const FBINFO_HWACCEL_YWRAP:u32=0x4000; pub const FBINFO_MISC_TILEBLITTING:u32=0x20000; pub const FBINFO_MISC_ALWAYS_SETPAR:u32=0x40000; pub const FBINFO_FOREIGN_ENDIAN:u32=0x100000; pub const FBINFO_BE_MATH:u32=0x100000; pub const FBINFO_HIDE_SMEM_START:u32=0x200000;

// External types and functions supplied by the surrounding kernel translation.
pub type loff_t=i64; pub type gfp_t=usize;
extern "C" { pub fn fb_set_var(*mut fb_info,*mut fb_var_screeninfo)->i32; pub fn fb_pan_display(*mut fb_info,*mut fb_var_screeninfo)->i32; pub fn fb_blank(*mut fb_info,i32)->i32; pub fn register_framebuffer(*mut fb_info)->i32; pub fn unregister_framebuffer(*mut fb_info); pub fn framebuffer_alloc(usize,*mut device)->*mut fb_info; pub fn framebuffer_release(*mut fb_info); }
extern "C" {
 pub fn fb_set_var_from_user(*mut fb_info,*mut fb_var_screeninfo)->i32;
 pub fn cfb_fillrect(*mut fb_info,*const fb_fillrect); pub fn cfb_copyarea(*mut fb_info,*const fb_copyarea); pub fn cfb_imageblit(*mut fb_info,*const fb_image);
 pub fn fb_io_read(*mut fb_info,*mut i8,usize,*mut loff_t)->isize; pub fn fb_io_write(*mut fb_info,*const i8,usize,*mut loff_t)->isize; pub fn fb_io_mmap(*mut fb_info,*mut vm_area_struct)->i32;
 pub fn sys_fillrect(*mut fb_info,*const fb_fillrect); pub fn sys_copyarea(*mut fb_info,*const fb_copyarea); pub fn sys_imageblit(*mut fb_info,*const fb_image);
 pub fn fb_sys_read(*mut fb_info,*mut i8,usize,*mut loff_t)->isize; pub fn fb_sys_write(*mut fb_info,*const i8,usize,*mut loff_t)->isize;
 pub fn devm_register_framebuffer(*mut device,*mut fb_info)->i32; pub fn fb_get_buffer_offset(*mut fb_info,*mut fb_pixmap,u32)->*mut i8;
 pub fn fb_pad_unaligned_buffer(*mut u8,u32,*const u8,u32,u32,u32,u32,u32); pub fn fb_pad_aligned_buffer(*mut u8,u32,*const u8,u32,u32);
 pub fn fb_set_suspend(*mut fb_info,i32); pub fn fb_switch_outputs(*mut fb_info); pub fn fb_get_color_depth(*mut fb_var_screeninfo,*mut fb_fix_screeninfo)->i32; pub fn fb_get_options(*const i8,*mut *mut i8)->i32; pub fn fb_new_modelist(*mut fb_info)->i32;
 pub fn fb_deferred_io_mmap(*mut fb_info,*mut vm_area_struct)->i32; pub fn fb_deferred_io_init(*mut fb_info)->i32; pub fn fb_deferred_io_open(*mut fb_info,*mut inode,*mut file); pub fn fb_deferred_io_release(*mut fb_info); pub fn fb_deferred_io_cleanup(*mut fb_info); pub fn fb_deferred_io_fsync(*mut file,loff_t,loff_t,i32)->i32;
 pub fn fbmon_dpms(*const fb_info)->i32; pub fn fb_get_mode(i32,u32,*mut fb_var_screeninfo,*mut fb_info)->i32; pub fn fb_validate_mode(*const fb_var_screeninfo,*mut fb_info)->i32; pub fn fb_parse_edid(*mut u8,*mut fb_var_screeninfo)->i32; pub fn fb_firmware_edid(*mut device)->*const u8; pub fn fb_edid_to_monspecs(*mut u8,*mut fb_monspecs); pub fn fb_destroy_modedb(*mut fb_videomode); pub fn fb_find_mode_cvt(*mut fb_videomode,i32,i32)->i32; pub fn fb_ddc_read(*mut i2c_adapter)->*mut u8;
 pub fn fb_alloc_cmap(*mut fb_cmap,i32,i32)->i32; pub fn fb_alloc_cmap_gfp(*mut fb_cmap,i32,i32,gfp_t)->i32; pub fn fb_dealloc_cmap(*mut fb_cmap); pub fn fb_copy_cmap(*const fb_cmap,*mut fb_cmap)->i32; pub fn fb_cmap_to_user(*const fb_cmap,*mut fb_cmap_user)->i32; pub fn fb_set_cmap(*mut fb_cmap,*mut fb_info)->i32; pub fn fb_set_user_cmap(*mut fb_cmap_user,*mut fb_info)->i32; pub fn fb_default_cmap(i32)->*const fb_cmap; pub fn fb_invert_cmaps();
 pub fn fb_find_mode(*mut fb_var_screeninfo,*mut fb_info,*const i8,*const fb_videomode,*const fb_videomode,u32,u32)->i32; pub fn fb_modesetting_disabled(*const i8)->bool;
}

#[repr(C)] pub struct fb_videomode { pub name:*const i8,pub refresh:u32,pub xres:u32,pub yres:u32,pub pixclock:u32,pub left_margin:u32,pub right_margin:u32,pub upper_margin:u32,pub lower_margin:u32,pub hsync_len:u32,pub vsync_len:u32,pub sync:u32,pub vmode:u32,pub flag:u32 }
#[repr(C)] pub struct dmt_videomode { pub dmt_id:u32,pub std_2byte_code:u32,pub cvt_3byte_code:u32,pub mode:*const fb_videomode }
#[repr(C)] pub struct fb_modelist { pub list:list_head,pub mode:fb_videomode }
pub const FB_MAXTIMINGS:i32=0; pub const FB_VSYNCTIMINGS:i32=1; pub const FB_HSYNCTIMINGS:i32=2; pub const FB_DCLKTIMINGS:i32=3; pub const FB_IGNOREMON:i32=0x100;
pub const FB_MODE_IS_UNKNOWN:i32=0; pub const FB_MODE_IS_DETAILED:i32=1; pub const FB_MODE_IS_STANDARD:i32=2; pub const FB_MODE_IS_VESA:i32=4; pub const FB_MODE_IS_CALCULATED:i32=8; pub const FB_MODE_IS_FIRST:i32=16; pub const FB_MODE_IS_FROM_VAR:i32=32;
pub const VESA_MODEDB_SIZE:u32=43; pub const DMT_SIZE:u32=0x50; pub const FBINFO_STATE_RUNNING:u32=0; pub const FBINFO_STATE_SUSPENDED:u32=1;

#[repr(C)] pub struct inode{_private:[u8;0]} #[repr(C)] pub struct file{_private:[u8;0]} #[repr(C)] pub struct i2c_adapter{_private:[u8;0]}

// Opaque declarations for included kernel/uapi types.
#[repr(C)] pub struct fb_info { _private:[u8;0] } #[repr(C)] pub struct fb_var_screeninfo{_private:[u8;0]} #[repr(C)] pub struct fb_fix_screeninfo{_private:[u8;0]} #[repr(C)] pub struct fb_cmap{_private:[u8;0]} #[repr(C)] pub struct fbcurpos{_private:[u8;0]} #[repr(C)] pub struct fb_fillrect{_private:[u8;0]} #[repr(C)] pub struct fb_copyarea{_private:[u8;0]} #[repr(C)] pub struct fb_image{_private:[u8;0]} #[repr(C)] pub struct fb_cursor{_private:[u8;0]} #[repr(C)] pub struct list_head{_private:[u8;0]} #[repr(C)] pub struct page{_private:[u8;0]} #[repr(C)] pub struct module{_private:[u8;0]} #[repr(C)] pub struct device{_private:[u8;0]} #[repr(C)] pub struct vm_area_struct{_private:[u8;0]}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
