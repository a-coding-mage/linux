/* Telecom Clock driver for Intel NetStructure(tm) MPCBL0010 */

/* Kernel dependencies are supplied by the surrounding kernel/Rust bindings. */
use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type irqreturn_t = c_int;

const RESET_ON: u8 = 0x00; const RESET_OFF: u8 = 0x01;
const NORMAL_MODE: u8 = 0x00; const HOLDOVER_MODE: u8 = 0x10; const FREERUN_MODE: u8 = 0x20;
const FILTER_6HZ: u8 = 0x04; const FILTER_12HZ: u8 = 0x00;
const REF_CLK1_8KHZ: u8 = 0x00; const REF_CLK2_19_44MHZ: u8 = 0x02;
const PRIMARY_CLOCK: u8 = 0x00; const SECONDARY_CLOCK: u8 = 0x01;
const CLK_8KHZ: u8 = 0xff; const CLK_16_384MHZ: u8 = 0xfb;
const CLK_1_544MHZ: u8 = 0x00; const CLK_2_048MHZ: u8 = 0x01; const CLK_4_096MHZ: u8 = 0x02;
const CLK_6_312MHZ: u8 = 0x03; const CLK_8_192MHZ: u8 = 0x04; const CLK_19_440MHZ: u8 = 0x06;
const CLK_8_592MHZ: u8 = 0x08; const CLK_11_184MHZ: u8 = 0x09; const CLK_34_368MHZ: u8 = 0x0b; const CLK_44_736MHZ: u8 = 0x0a;
const AMC_B1: u8 = 0; const AMC_B2: u8 = 1;
const HW_ENABLE: u8 = 0x80; const HW_DISABLE: u8 = 0x00; const PLL_HOLDOVER: u8 = 0x40; const LOST_CLOCK: u8 = 0x00;
const UNLOCK_MASK: u8 = 0x10; const HOLDOVER_MASK: u8 = 0x20; const SEC_LOST_MASK: u8 = 0x40; const PRI_LOST_MASK: u8 = 0x80;
const PRI_LOS_01_MASK: u8 = 0x01; const PRI_LOS_10_MASK: u8 = 0x02; const SEC_LOS_01_MASK: u8 = 0x04; const SEC_LOS_10_MASK: u8 = 0x08;
const HOLDOVER_01_MASK: u8 = 0x10; const HOLDOVER_10_MASK: u8 = 0x20; const UNLOCK_01_MASK: u8 = 0x40; const UNLOCK_10_MASK: u8 = 0x80;

#[repr(C)]
pub struct tlclk_alarms { pub lost_clocks: __u32, pub lost_primary_clock: __u32, pub lost_secondary_clock: __u32, pub primary_clock_back: __u32, pub secondary_clock_back: __u32, pub switchover_primary: __u32, pub switchover_secondary: __u32, pub pll_holdover: __u32, pub pll_end_holdover: __u32, pub pll_lost_sync: __u32, pub pll_sync: __u32 }

const TLCLK_BASE: u16 = 0xa08; const TLCLK_REG0: u16 = TLCLK_BASE; const TLCLK_REG1: u16 = TLCLK_BASE + 1; const TLCLK_REG2: u16 = TLCLK_BASE + 2; const TLCLK_REG3: u16 = TLCLK_BASE + 3; const TLCLK_REG4: u16 = TLCLK_BASE + 4; const TLCLK_REG5: u16 = TLCLK_BASE + 5; const TLCLK_REG6: u16 = TLCLK_BASE + 6; const TLCLK_REG7: u16 = TLCLK_BASE + 7;
const TLCLK_MAJOR: c_int = 0;

#[repr(C)] pub struct timer_list { pub expires: usize }
#[repr(C)] pub struct inode; #[repr(C)] pub struct file; #[repr(C)] pub struct device; #[repr(C)] pub struct device_attribute; #[repr(C)] pub struct faux_device; #[repr(C)] pub struct file_operations; #[repr(C)] pub struct miscdevice; #[repr(C)] pub struct wait_queue_head; #[repr(C)] pub struct mutex; #[repr(C)] pub struct spinlock;
extern "C" { fn inb(port: u16) -> u8; fn outb(value: u8, port: u16); fn jiffies() -> usize; fn msecs_to_jiffies(v: usize) -> usize; fn printk(fmt: *const c_char, ...); fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int; fn strnlen(s: *const c_char, n: usize) -> usize; fn memset(p: *mut c_void, v: c_int, n: usize); fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_int; fn request_irq(irq: u32, handler: unsafe extern "C" fn(c_int,*mut c_void)->irqreturn_t, flags: u64, name: *const c_char, dev: *mut c_void) -> c_int; fn free_irq(irq:u32, dev:*mut c_void); fn register_chrdev(major:c_int,name:*const c_char,fops:*const file_operations)->c_int; fn unregister_chrdev(major:c_int,name:*const c_char); fn request_region(base:u16,n:usize,name:*const c_char)->*mut c_void; fn release_region(base:u16,n:usize); fn misc_register(d:*mut miscdevice)->c_int; fn misc_deregister(d:*mut miscdevice); fn faux_device_create_with_groups(name:*const c_char,a:*mut c_void,b:*mut c_void,g:*mut *mut c_void)->*mut faux_device; fn faux_device_destroy(d:*mut faux_device); fn kzalloc(n:usize,flags:u32)->*mut c_void; fn kfree(p:*mut c_void); fn wake_up(q:*mut wait_queue_head); fn wake_up_all(q:*mut wait_queue_head); fn timer_setup(t:*mut timer_list, f:unsafe extern "C" fn(*mut timer_list), flags:u32); fn mod_timer(t:*mut timer_list,e:usize); fn timer_delete(t:*mut timer_list); fn timer_delete_sync(t:*mut timer_list); }

static mut telclk_interrupt: u32 = 0; static mut int_events: c_int = 0; static mut got_event: c_int = 0; static mut switchover_timer: timer_list = timer_list{expires:0}; static mut tlclk_timer_data: usize = 0; static mut alarm_events: *mut tlclk_alarms = core::ptr::null_mut(); static mut tlclk_major: c_int = TLCLK_MAJOR; static mut useflags: usize = 0; static mut wq: wait_queue_head = wait_queue_head; static mut tlclk_mutex: mutex = mutex; static mut event_lock: spinlock = spinlock;
extern "C" { fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn mutex_lock_interruptible(m:*mut mutex)->c_int; fn test_and_set_bit(n:usize,p:*mut usize)->c_int; fn clear_bit(n:usize,p:*mut usize); fn spin_lock_irqsave(l:*mut spinlock,f:*mut usize); fn spin_unlock_irqrestore(l:*mut spinlock,f:usize); }

unsafe fn set_port_bits(port:u16, mask:u8, val:u8) { outb((inb(port) & mask) | val, port); }

#[no_mangle] pub unsafe extern "C" fn tlclk_open(_: *mut inode, _: *mut file) -> c_int { mutex_lock(&mut tlclk_mutex); let mut r=0; if test_and_set_bit(0,&mut useflags)!=0 { r=-16; } else { inb(TLCLK_REG6); r=request_irq(telclk_interrupt,tlclk_interrupt_handler,0,b"telco_clock\0".as_ptr() as _,tlclk_interrupt_handler as _); if r== -16 { printk(b"tlclk: Interrupt can't be reserved.\n\0".as_ptr() as _); } else { inb(TLCLK_REG6); } } mutex_unlock(&mut tlclk_mutex); r }
#[no_mangle] pub unsafe extern "C" fn tlclk_release(_: *mut inode, _: *mut file)->c_int { free_irq(telclk_interrupt,tlclk_interrupt_handler as _); clear_bit(0,&mut useflags); 0 }
#[no_mangle] pub unsafe extern "C" fn tlclk_read(_: *mut file, buf:*mut c_char, count:usize, _: *mut loff_t)->ssize_t { if count<core::mem::size_of::<tlclk_alarms>() {return -5} if mutex_lock_interruptible(&mut tlclk_mutex)!=0{return -4} while got_event==0 {} if copy_to_user(buf as _,alarm_events as _,core::mem::size_of::<tlclk_alarms>())!=0 {mutex_unlock(&mut tlclk_mutex);return -14} memset(alarm_events as _,0,core::mem::size_of::<tlclk_alarms>());got_event=0;mutex_unlock(&mut tlclk_mutex);core::mem::size_of::<tlclk_alarms>() as ssize_t }

#[no_mangle] pub unsafe extern "C" fn switchover_timeout(_: *mut timer_list) { let f=tlclk_timer_data; if f&1 !=0 {if (inb(TLCLK_REG1)&8) as usize != f&8 {(*alarm_events).switchover_primary+=1}} else if (inb(TLCLK_REG1)&8) as usize != f&8 {(*alarm_events).switchover_secondary+=1} timer_delete(&mut switchover_timer);got_event=1;wake_up(&mut wq); }
#[no_mangle] pub unsafe extern "C" fn tlclk_interrupt_handler(_:c_int,_:*mut c_void)->irqreturn_t { let mut f=0;spin_lock_irqsave(&mut event_lock,&mut f);int_events=inb(TLCLK_REG6) as c_int;if int_events&1!=0 {if inb(TLCLK_REG2)&SEC_LOST_MASK!=0{(*alarm_events).lost_clocks+=1}else{(*alarm_events).lost_primary_clock+=1}}if int_events&2!=0{(*alarm_events).primary_clock_back+=1;set_port_bits(TLCLK_REG1,0xfe,1)}if int_events&4!=0{if inb(TLCLK_REG2)&PRI_LOST_MASK!=0{(*alarm_events).lost_clocks+=1}else{(*alarm_events).lost_secondary_clock+=1}}if int_events&8!=0{(*alarm_events).secondary_clock_back+=1;set_port_bits(TLCLK_REG1,0xfe,0)}if int_events&0x20!=0{(*alarm_events).pll_end_holdover+=1}if int_events&0x40!=0{(*alarm_events).pll_lost_sync+=1}if int_events&0x80!=0{(*alarm_events).pll_sync+=1}if int_events&0x10!=0{(*alarm_events).pll_holdover+=1;switchover_timer.expires=jiffies()+msecs_to_jiffies(10);tlclk_timer_data=inb(TLCLK_REG1) as usize;mod_timer(&mut switchover_timer,switchover_timer.expires)}else{got_event=1;wake_up(&mut wq)}spin_unlock_irqrestore(&mut event_lock,f);1 }

#[no_mangle] pub unsafe extern "C" fn tlclk_init()->c_int { telclk_interrupt=(inb(TLCLK_REG7)&0xf) as u32;alarm_events=kzalloc(core::mem::size_of::<tlclk_alarms>(),0) as _;if alarm_events.is_null(){return -12}let r=register_chrdev(tlclk_major,b"telco_clock\0".as_ptr() as _,core::ptr::null());if r<0{kfree(alarm_events as _);return r}tlclk_major=r;if request_region(TLCLK_BASE,8,b"telco_clock\0".as_ptr() as _).is_null(){kfree(alarm_events as _);unregister_chrdev(tlclk_major,b"telco_clock\0".as_ptr() as _);return -16}if telclk_interrupt==0xf{release_region(TLCLK_BASE,8);kfree(alarm_events as _);unregister_chrdev(tlclk_major,b"telco_clock\0".as_ptr() as _);return -6}timer_setup(&mut switchover_timer,switchover_timeout,0);0 }
#[no_mangle] pub unsafe extern "C" fn tlclk_cleanup(){got_event=1;wake_up_all(&mut wq);release_region(TLCLK_BASE,8);timer_delete_sync(&mut switchover_timer);kfree(alarm_events as _);}

/* Sysfs handlers retain the original register masks and write ordering. */
unsafe fn store_reg(port:u16, mask:u8, val:u8)->ssize_t { let mut f=0;spin_lock_irqsave(&mut event_lock,&mut f);set_port_bits(port,mask,val);spin_unlock_irqrestore(&mut event_lock,f);0 }
#[no_mangle] pub unsafe extern "C" fn store_received_ref_clk3a(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG1,0xef,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_received_ref_clk3b(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG1,0xdf,2);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clk3b_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG3,0x7f,0x80);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clk3a_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG3,0xbf,0x40);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clkb1_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG2,0xf7,8);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clka1_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG2,0xfb,4);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clkb0_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG2,0xfd,2);n as _}
#[no_mangle] pub unsafe extern "C" fn store_enable_clka0_output(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG2,0xfe,1);n as _}
#[no_mangle] pub unsafe extern "C" fn store_select_redundant_clock(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG1,0xfe,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_select_ref_frequency(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG1,0xfd,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_filter_select(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG0,0xfb,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_hardware_switching_mode(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG0,0xbf,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_hardware_switching(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG0,0x7f,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_refalign(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG0,0xf7,0);store_reg(TLCLK_REG0,0xf7,8);store_reg(TLCLK_REG0,0xf7,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_mode_select(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG0,0xcf,0);n as _}
#[no_mangle] pub unsafe extern "C" fn store_reset(_: *mut device,_:*mut device_attribute,_:*const c_char,n:usize)->ssize_t{store_reg(TLCLK_REG4,0xfd,0);n as _}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
