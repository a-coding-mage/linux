/* Rust translation of am200epd.c. External kernel symbols are intentionally
 * referenced as dependencies supplied by the surrounding tree. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static THIS_MODULE: *mut c_void;
    static mut am200_device: *mut platform_device;
    fn gpio_request(gpio: c_int, label: *const c_char) -> c_int;
    fn gpio_free(gpio: c_int);
    fn gpio_direction_output(gpio: c_int, value: c_int) -> c_int;
    fn gpio_direction_input(gpio: c_int) -> c_int;
    fn gpio_set_value(gpio: c_int, value: c_int);
    fn gpio_get_value(gpio: c_int) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                   flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn wake_up_interruptible(waitq: *mut c_void);
    fn wait_event_timeout(waitq: *mut c_void, condition: c_int, timeout: c_int) -> c_int;
    fn wait_event_interruptible_timeout(waitq: *mut c_void, condition: c_int, timeout: c_int) -> c_int;
    fn try_module_get(owner: *mut c_void) -> c_int;
    fn module_put(owner: *mut c_void);
    fn fb_register_client(block: *mut notifier_block) -> c_int;
    fn fb_unregister_client(block: *mut notifier_block) -> c_int;
    fn pxa2xx_mfp_config(config: *mut c_ulong, count: usize);
    fn request_module(name: *const c_char) -> c_int;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add_data(dev: *mut platform_device, data: *const c_void, size: usize) -> c_int;
    fn platform_device_add(dev: *mut platform_device) -> c_int;
    fn platform_device_put(dev: *mut platform_device);
    fn pxa_set_fb_info(dev: *mut c_void, info: *mut pxafb_mach_info);
}

type irqreturn_t = c_int;

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct fb_info { pub var: fb_var_screeninfo, pub screen_base: *mut u8, pub fbops: *mut fb_ops, pub fix: fb_fix_screeninfo, pub par: *mut c_void }
#[repr(C)] pub struct fb_var_screeninfo { pub xres: c_uint, pub yres: c_uint }
#[repr(C)] pub struct fb_fix_screeninfo { pub smem_start: c_ulong }
#[repr(C)] pub struct fb_ops { pub owner: *mut c_void }
#[repr(C)] pub struct fb_event { pub info: *mut fb_info }
#[repr(C)] pub struct pxafb_mode_info { pub pixclock: c_uint, pub xres: c_uint, pub yres: c_uint, pub bpp: c_uint, pub hsync_len: c_uint, pub left_margin: c_uint, pub right_margin: c_uint, pub vsync_len: c_uint, pub upper_margin: c_uint, pub lower_margin: c_uint, pub sync: c_uint }
#[repr(C)] pub struct pxafb_mach_info { pub modes: *mut pxafb_mode_info, pub num_modes: c_uint, pub lcd_conn: c_uint }
#[repr(C)] pub struct metronomefb_par { pub metromem_cmd: *mut metromem_cmd, pub metromem_wfm: *mut u8, pub metromem_img: *mut u8, pub metromem_img_csum: *mut u16, pub metromem_dma: c_ulong, pub waitq: *mut c_void }
#[repr(C)] pub struct metromem_cmd { _private: [u8; 0] }
#[repr(C)] pub struct metronome_board { pub owner: *mut c_void, pub setup_irq: Option<unsafe extern "C" fn(*mut fb_info) -> c_int>, pub setup_io: Option<unsafe extern "C" fn(*mut metronomefb_par) -> c_int>, pub setup_fb: Option<unsafe extern "C" fn(*mut metronomefb_par) -> c_int>, pub set_rst: Option<unsafe extern "C" fn(*mut metronomefb_par, c_int)>, pub set_stdby: Option<unsafe extern "C" fn(*mut metronomefb_par, c_int)>, pub met_wait_event: Option<unsafe extern "C" fn(*mut metronomefb_par) -> c_int>, pub met_wait_event_intr: Option<unsafe extern "C" fn(*mut metronomefb_par) -> c_int>, pub get_panel_type: Option<unsafe extern "C" fn() -> c_int>, pub cleanup: Option<unsafe extern "C" fn(*mut metronomefb_par)>, pub metromem: *mut u8, pub host_fbinfo: *mut fb_info, pub wfm_size: c_int, pub fw: c_int, pub fh: c_int }

static mut panel_type: c_uint = 6;
/* am200_device is defined below; the C tentative declaration is represented by that definition. */
static mut am200_device: *mut platform_device = core::ptr::null_mut();
static mut am200_fb_mode_9inch7: pxafb_mode_info = pxafb_mode_info { pixclock:40000,xres:1200,yres:842,bpp:16,hsync_len:2,left_margin:2,right_margin:2,vsync_len:1,upper_margin:2,lower_margin:25,sync:3 };
static mut am200_fb_mode_8inch: pxafb_mode_info = pxafb_mode_info { pixclock:40000,xres:1088,yres:791,bpp:16,hsync_len:28,left_margin:8,right_margin:30,vsync_len:8,upper_margin:10,lower_margin:8,sync:3 };
static mut am200_fb_mode_6inch: pxafb_mode_info = pxafb_mode_info { pixclock:40189,xres:832,yres:622,bpp:16,hsync_len:28,left_margin:34,right_margin:34,vsync_len:25,upper_margin:0,lower_margin:2,sync:3 };
static mut am200_fb_info: pxafb_mach_info = pxafb_mach_info { modes: core::ptr::null_mut(), num_modes:1, lcd_conn:0 };
static mut am200_board: metronome_board = metronome_board { owner: core::ptr::null_mut(), setup_irq:Some(am200_setup_irq),setup_io:Some(am200_init_gpio_regs),setup_fb:Some(am200_setup_fb),set_rst:Some(am200_set_rst),set_stdby:Some(am200_set_stdby),met_wait_event:Some(am200_wait_event),met_wait_event_intr:Some(am200_wait_event_intr),get_panel_type:Some(am200_get_panel_type),cleanup:Some(am200_cleanup),metromem:core::ptr::null_mut(),host_fbinfo:core::ptr::null_mut(),wfm_size:0,fw:0,fh:0 };

const GP: [c_int; 6] = [51,48,49,32,17,16];
static GPIO_NAMES: [&[u8]; 6] = [b"LED\0",b"STDBY\0",b"RST\0",b"RDY\0",b"ERR\0",b"PCBPWR\0"];

unsafe extern "C" fn am200_init_gpio_regs(_par: *mut metronomefb_par) -> c_int { let mut i=0; while i<6 { let e=gpio_request(GP[i],GPIO_NAMES[i].as_ptr() as *const c_char); if e!=0 { while i>0 { i-=1; gpio_free(GP[i]); } return e; } i+=1; } gpio_direction_output(51,0);gpio_direction_output(48,0);gpio_direction_output(49,0);gpio_direction_input(32);gpio_direction_input(17);gpio_direction_output(16,0);0 }
unsafe extern "C" fn am200_cleanup(par:*mut metronomefb_par){ free_irq(32,par as *mut c_void); for g in GP { gpio_free(g); } }
unsafe extern "C" fn am200_get_panel_type()->c_int { panel_type as c_int }
unsafe extern "C" fn am200_set_rst(_: *mut metronomefb_par,s:c_int){gpio_set_value(49,s)}
unsafe extern "C" fn am200_set_stdby(_: *mut metronomefb_par,s:c_int){gpio_set_value(48,s)}
unsafe extern "C" fn am200_handle_irq(_:c_int,dev:*mut c_void)->irqreturn_t { wake_up_interruptible((* (dev as *mut metronomefb_par)).waitq); 1 }
unsafe extern "C" fn am200_setup_irq(info:*mut fb_info)->c_int { request_irq(32,am200_handle_irq,0x2,b"AM200\0".as_ptr() as *const c_char,(*info).par) }
unsafe extern "C" fn am200_wait_event(p:*mut metronomefb_par)->c_int { wait_event_timeout((*p).waitq,gpio_get_value(32),1000) }
unsafe extern "C" fn am200_wait_event_intr(p:*mut metronomefb_par)->c_int { wait_event_interruptible_timeout((*p).waitq,gpio_get_value(32),1000) }

unsafe extern "C" fn am200_setup_fb(p:*mut metronomefb_par)->c_int { let fw=am200_board.fw; let fh=am200_board.fh; (*p).metromem_cmd=am200_board.metromem as *mut metromem_cmd; (*p).metromem_wfm=am200_board.metromem.add(fw as usize); (*p).metromem_img=(*p).metromem_wfm.add(am200_board.wfm_size as usize); (*p).metromem_img_csum=(*p).metromem_img.add((fw*fh) as usize) as *mut u16; (*p).metromem_dma=(*am200_board.host_fbinfo).fix.smem_start; 0 }

static mut am200_fb_notif: notifier_block = notifier_block { notifier_call: None };
unsafe extern "C" fn am200_share_video_mem(info:*mut fb_info)->c_int { if (*info).var.xres != (*am200_fb_info.modes).xres || (*info).var.yres != (*am200_fb_info.modes).yres{return 0} am200_board.metromem=(*info).screen_base;am200_board.host_fbinfo=info;if try_module_get((*(*info).fbops).owner)==0{-19}else{0} }
unsafe extern "C" fn am200_unshare_video_mem(info:*mut fb_info)->c_int { if info!=am200_board.host_fbinfo{return 0} module_put((*(*am200_board.host_fbinfo).fbops).owner);0 }
unsafe extern "C" fn am200_fb_notifier_callback(_: *mut notifier_block,event:c_ulong,data:*mut c_void)->c_int { let info=(*(data as *mut fb_event)).info; if event==0x01 {am200_share_video_mem(info)} else if event==0x02 {am200_unshare_video_mem(info)} else {0} }

unsafe fn am200_presetup_fb() { let mode=match panel_type { 6=>&mut am200_fb_mode_6inch,8=>&mut am200_fb_mode_8inch,97=>&mut am200_fb_mode_9inch7,_=>&mut am200_fb_mode_6inch }; am200_fb_info.modes=mode; let fw=mode.xres as usize; let fh=mode.yres as usize; am200_board.wfm_size=((16*1024+2+fw-1)/fw*fw) as c_int; let padding=4096+4*fw; let total=fw+am200_board.wfm_size as usize+padding+fw*fh; am200_board.fw=fw as c_int;am200_board.fh=fh as c_int;mode.yres=((total+fw-1)/fw) as c_uint;mode.xres/=2;pxa_set_fb_info(core::ptr::null_mut(),&mut am200_fb_info); }

#[no_mangle] pub unsafe extern "C" fn am200_init()->c_int { am200_fb_info.modes=&mut am200_fb_mode_6inch; pxa_set_fb_info(core::ptr::null_mut(),&mut am200_fb_info); request_module(b"metronomefb\0".as_ptr() as *const c_char); am200_device=platform_device_alloc(b"metronomefb\0".as_ptr() as *const c_char,-1); if am200_device.is_null(){return -12} platform_device_add_data(am200_device,&am200_board as *const _ as *const c_void,core::mem::size_of::<metronome_board>()); let r=platform_device_add(am200_device); if r!=0 {platform_device_put(am200_device);} r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
