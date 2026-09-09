// SPDX-License-Identifier: GPL-2.0
/* RTAS (Runtime Abstraction Services) stuff. */

const KEY_SWITCH: u32 = 0x0001;
const ENCLOSURE_SWITCH: u32 = 0x0002;
const THERMAL_SENSOR: u32 = 0x0003;
const LID_STATUS: u32 = 0x0004;
const POWER_SOURCE: u32 = 0x0005;
const BATTERY_VOLTAGE: u32 = 0x0006;
const BATTERY_REMAINING: u32 = 0x0007;
const BATTERY_PERCENTAGE: u32 = 0x0008;
const EPOW_SENSOR: u32 = 0x0009;
const BATTERY_CYCLESTATE: u32 = 0x000a;
const BATTERY_CHARGING: u32 = 0x000b;
const IBM_SURVEILLANCE: u32 = 0x2328;
const IBM_FANRPM: u32 = 0x2329;
const IBM_VOLTAGE: u32 = 0x232a;
const IBM_DRCONNECTOR: u32 = 0x232b;
const IBM_POWERSUPPLY: u32 = 0x232c;
const SENSOR_CRITICAL_HIGH: i32 = 13;
const SENSOR_WARNING_HIGH: i32 = 12;
const SENSOR_NORMAL: i32 = 11;
const SENSOR_WARNING_LOW: i32 = 10;
const SENSOR_CRITICAL_LOW: i32 = 9;
const SENSOR_SUCCESS: i32 = 0;
const SENSOR_HW_ERROR: i32 = -1;
const SENSOR_BUSY: i32 = -2;
const SENSOR_NOT_EXIST: i32 = -3;
const SENSOR_DR_ENTITY: i32 = -9000;
const LOC_PLANAR: u8 = b'P';
const LOC_CPU: u8 = b'C';
const LOC_FAN: u8 = b'F';
const LOC_RACKMOUNTED: u8 = b'U';
const LOC_VOLTAGE: u8 = b'V';
const LOC_LCD: u8 = b'L';
const TONE_FREQUENCY: u32 = 1;
const TONE_VOLUME: u32 = 2;
const MAX_SENSORS: usize = 17;
const MAX_LINELENGTH: usize = 256;
const SENSOR_PREFIX: &str = "ibm,sensor-";

#[repr(C)]
struct individual_sensor { token: u32, quant: u32 }
#[repr(C)]
struct rtas_sensors { sensor: [individual_sensor; MAX_SENSORS], quant: u32 }

extern "C" {
    static mut sensors: rtas_sensors;
    static mut rtas_node: *mut device_node;
    static mut power_on_time: u64;
    static mut progress_led: [u8; MAX_LINELENGTH];
    static mut rtas_tone_frequency: u64;
    static mut rtas_tone_volume: u64;
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn seq_read(); fn seq_lseek(); fn single_release();
    fn proc_create(*const u8, u32, *mut inode, *const proc_ops) -> *mut inode;
    fn proc_create_single(*const u8, u32, *mut inode, unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32) -> *mut inode;
    fn machine_is(x: i32) -> bool; fn of_find_node_by_name(*mut device_node, *const u8) -> *mut device_node;
    fn copy_from_user(*mut u8, *const u8, usize) -> usize;
    fn kstrtoull(*const u8, u32, *mut u64) -> i32;
    fn rtc_time64_to_tm(i64, *mut rtc_time);
    fn rtas_function_token(i32) -> i32;
    fn rtas_call(i32, i32, i32, *mut i32, ... ) -> i32;
    fn printk(*const u8, ...);
    fn rtas_progress(*const u8, u64);
    fn mktime64(u32,u32,u32,u32,u32,u32) -> i64;
    fn of_get_property(*mut device_node, *const u8, *mut i32) -> *const u8;
    fn seq_printf(*mut seq_file, *const u8, ...); fn seq_putc(*mut seq_file, i32);
    fn simple_strtoul(*const u8, *mut *const u8, u32) -> u64;
}
#[repr(C)] struct device_node;
#[repr(C)] struct seq_file;
#[repr(C)] struct file;
#[repr(C)] struct inode;
#[repr(C)] struct proc_ops { proc_open: Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, proc_read: Option<unsafe extern "C" fn()>, proc_lseek: Option<unsafe extern "C" fn()>, proc_write: Option<unsafe extern "C" fn()>, proc_release: Option<unsafe extern "C" fn()> }
#[repr(C)] struct rtc_time { tm_sec:i32, tm_min:i32, tm_hour:i32, tm_mday:i32, tm_mon:i32, tm_year:i32, tm_wday:i32, tm_yday:i32, tm_isdst:i32 }
extern "C" { static RTAS_FN_SET_TIME_FOR_POWER_ON:i32; static RTAS_FN_SET_TIME_OF_DAY:i32; static RTAS_FN_GET_TIME_OF_DAY:i32; static RTAS_FN_GET_SENSOR_STATE:i32; static RTAS_FN_SET_INDICATOR:i32; static rtas_rmo_buf:u64; static RTAS_USER_REGION_SIZE:u32; }

unsafe extern "C" fn parse_number(p:*const u8,count:usize,val:*mut u64)->i32 { let mut buf=[0u8;40]; if count>39{return -22;} if copy_from_user(buf.as_mut_ptr(),p,count)!=0{return -14;} buf[count]=0; kstrtoull(buf.as_ptr(),10,val) }
unsafe extern "C" fn ppc_rtas_poweron_write(_: *mut file,buf:*const u8,count:usize,_:*mut i64)->isize { let mut now=0u64; let e=parse_number(buf,count,&mut now); if e!=0{return e as isize;} power_on_time=now; let mut tm=core::mem::zeroed::<rtc_time>(); rtc_time64_to_tm(now as i64,&mut tm); let e=rtas_call(rtas_function_token(RTAS_FN_SET_TIME_FOR_POWER_ON),7,1,core::ptr::null_mut(),tm.tm_year+1900,tm.tm_mon+1,tm.tm_mday,tm.tm_hour,tm.tm_min,tm.tm_sec,0); if e!=0 { printk(b"error: setting poweron time returned: %s\0".as_ptr(), ppc_rtas_process_error(e)); } count as isize }
unsafe extern "C" fn ppc_rtas_poweron_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32 { if power_on_time==0 {seq_printf(m,b"Power on time not set\n\0".as_ptr());} else {seq_printf(m,b"%lu\n\0".as_ptr(),power_on_time);} 0 }
unsafe extern "C" fn ppc_rtas_progress_write(_: *mut file,buf:*const u8,count:usize,_:*mut i64)->isize { let n=if count>=MAX_LINELENGTH {MAX_LINELENGTH-1}else{count}; if copy_from_user(progress_led.as_mut_ptr(),buf,n)!=0{return -14;} progress_led[n]=0; let h=simple_strtoul(progress_led.as_ptr(),core::ptr::null_mut(),10); rtas_progress(progress_led.as_ptr(),h); n as isize }
unsafe extern "C" fn ppc_rtas_progress_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32 { if progress_led[0]!=0 {seq_printf(m,b"%s\n\0".as_ptr(),progress_led.as_ptr());} 0 }
unsafe extern "C" fn ppc_rtas_clock_write(_: *mut file,buf:*const u8,count:usize,_:*mut i64)->isize { let mut now=0u64; let e=parse_number(buf,count,&mut now); if e!=0{return e as isize;} let mut tm=core::mem::zeroed::<rtc_time>(); rtc_time64_to_tm(now as i64,&mut tm); let e=rtas_call(rtas_function_token(RTAS_FN_SET_TIME_OF_DAY),7,1,core::ptr::null_mut(),tm.tm_year+1900,tm.tm_mon+1,tm.tm_mday,tm.tm_hour,tm.tm_min,tm.tm_sec,0); if e!=0 {printk(b"error: setting the clock returned: %s\0".as_ptr(),ppc_rtas_process_error(e));} count as isize }
unsafe extern "C" fn ppc_rtas_clock_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32 { let mut r=[0i32;8]; let e=rtas_call(rtas_function_token(RTAS_FN_GET_TIME_OF_DAY),0,8,r.as_mut_ptr()); if e!=0 {seq_printf(m,b"0\0".as_ptr());} else {seq_printf(m,b"%lld\n\0".as_ptr(),mktime64(r[0] as u32,r[1] as u32,r[2] as u32,r[3] as u32,r[4] as u32,r[5] as u32));} 0 }
unsafe extern "C" fn ppc_rtas_process_error(e:i32)->*const u8 { match e {13=>b"(critical high)\0",12=>b"(warning high)\0",11=>b"(normal)\0",10=>b"(warning low)\0",9=>b"(critical low)\0",0=>b"(read ok)\0",-1=>b"(hardware error)\0",-2=>b"(busy)\0",-3=>b"(non existent)\0",-9000=>b"(dr entity removed)\0",_=>b"(UNKNOWN)\0"}.as_ptr() }
unsafe extern "C" fn ppc_rtas_find_all_sensors()->i32 { let mut len=0; let mut p=of_get_property(rtas_node,b"rtas-sensors\0".as_ptr(),&mut len) as *const u32; if p.is_null(){return 1;} sensors.quant=(len/8) as u32; for i in 0..sensors.quant as usize {sensors.sensor[i].token=*p;p=p.add(1);sensors.sensor[i].quant=*p;p=p.add(1);} 0 }
unsafe extern "C" fn ppc_rtas_sensors_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32 { seq_printf(m,b"RTAS (RunTime Abstraction Services) Sensor Information\nSensor\t\tValue\t\tCondition\tLocation\n********************************************************\n\0".as_ptr()); if ppc_rtas_find_all_sensors()!=0 {seq_printf(m,b"\nNo sensors are available\n\0".as_ptr());} 0 }
unsafe extern "C" fn ppc_rtas_process_sensor(_: *mut seq_file,_:*mut individual_sensor,_:i32,_:i32,_:*const u8) {}
unsafe extern "C" fn get_location_code(_: *mut seq_file,_:*mut individual_sensor,_:*const u8) {}
unsafe extern "C" fn check_location_string(_: *mut seq_file,_:*const u8) {}
unsafe extern "C" fn check_location(_: *mut seq_file,_:*const u8) {}
unsafe extern "C" fn ppc_rtas_tone_freq_write(_: *mut file,buf:*const u8,count:usize,_:*mut i64)->isize {let mut x=0;let e=parse_number(buf,count,&mut x);if e!=0{return e as isize;}rtas_tone_frequency=x;rtas_call(rtas_function_token(RTAS_FN_SET_INDICATOR),3,1,core::ptr::null_mut(),TONE_FREQUENCY,0,x);count as isize}
unsafe extern "C" fn ppc_rtas_tone_freq_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32{seq_printf(m,b"%lu\n\0".as_ptr(),rtas_tone_frequency);0}
unsafe extern "C" fn ppc_rtas_tone_volume_write(_: *mut file,buf:*const u8,count:usize,_:*mut i64)->isize {let mut x=0;let e=parse_number(buf,count,&mut x);if e!=0{return e as isize;}if x>100{x=100;}rtas_tone_volume=x;rtas_call(rtas_function_token(RTAS_FN_SET_INDICATOR),3,1,core::ptr::null_mut(),TONE_VOLUME,0,x);count as isize}
unsafe extern "C" fn ppc_rtas_tone_volume_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32{seq_printf(m,b"%lu\n\0".as_ptr(),rtas_tone_volume);0}
unsafe extern "C" fn ppc_rtas_rmo_buf_show(m:*mut seq_file,_:*mut core::ffi::c_void)->i32{seq_printf(m,b"%016lx %x\n\0".as_ptr(),rtas_rmo_buf,RTAS_USER_REGION_SIZE);0}

/* The following callback/open tables mirror the C procfs registration layer. */
unsafe extern "C" fn poweron_open(i:*mut inode,f:*mut file)->i32{single_open(f,ppc_rtas_poweron_show,core::ptr::null_mut())}
unsafe extern "C" fn progress_open(_: *mut inode,f:*mut file)->i32{single_open(f,ppc_rtas_progress_show,core::ptr::null_mut())}
unsafe extern "C" fn clock_open(_: *mut inode,f:*mut file)->i32{single_open(f,ppc_rtas_clock_show,core::ptr::null_mut())}
unsafe extern "C" fn tone_freq_open(_: *mut inode,f:*mut file)->i32{single_open(f,ppc_rtas_tone_freq_show,core::ptr::null_mut())}
unsafe extern "C" fn tone_volume_open(_: *mut inode,f:*mut file)->i32{single_open(f,ppc_rtas_tone_volume_show,core::ptr::null_mut())}

unsafe extern "C" fn proc_rtas_init()->i32 {
    rtas_node=of_find_node_by_name(core::ptr::null_mut(),b"rtas\0".as_ptr());
    if rtas_node.is_null(){return -19;}
    proc_create(b"powerpc/rtas/progress\0".as_ptr(),0o644,core::ptr::null_mut(),core::ptr::null());
    proc_create(b"powerpc/rtas/clock\0".as_ptr(),0o644,core::ptr::null_mut(),core::ptr::null());
    proc_create(b"powerpc/rtas/poweron\0".as_ptr(),0o644,core::ptr::null_mut(),core::ptr::null());
    proc_create_single(b"powerpc/rtas/sensors\0".as_ptr(),0o444,core::ptr::null_mut(),ppc_rtas_sensors_show);
    proc_create(b"powerpc/rtas/frequency\0".as_ptr(),0o644,core::ptr::null_mut(),core::ptr::null());
    proc_create(b"powerpc/rtas/volume\0".as_ptr(),0o644,core::ptr::null_mut(),core::ptr::null());
    proc_create_single(b"powerpc/rtas/rmo_buffer\0".as_ptr(),0o400,core::ptr::null_mut(),ppc_rtas_rmo_buf_show); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
