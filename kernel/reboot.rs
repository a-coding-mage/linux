// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of reboot.c. Kernel dependencies are external.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,c_ulong,*mut c_void)->c_int>, pub priority: c_int }
#[repr(C)] pub struct sys_off_data { pub cb_data:*mut c_void, pub mode:c_ulong, pub cmd:*mut c_void, pub dev:*mut device }
#[repr(C)] pub struct sys_off_handler { pub nb:notifier_block, pub sys_off_cb:Option<unsafe extern "C" fn(*mut sys_off_data)->c_int>, pub cb_data:*mut c_void, pub mode:c_ulong, pub blocking:bool, pub list:*mut c_void, pub dev:*mut device }

type RebootMode = c_int;
type HwAction = c_int;
extern "C" {
    static mut system_state:c_int; static mut reboot_mode:RebootMode; static mut panic_reboot_mode:RebootMode;
    static mut reboot_default:c_int; static mut reboot_cpu:c_int; static mut reboot_type:c_int; static mut reboot_force:c_int;
    static mut pm_power_off:Option<unsafe extern "C" fn()>;
    fn kmsg_dump(c_int); fn machine_emergency_restart(); fn machine_restart(*mut c_char); fn machine_halt(); fn machine_power_off();
    fn blocking_notifier_call_chain(*mut c_void,c_ulong,*mut c_void)->c_int; fn atomic_notifier_call_chain(*mut c_void,c_ulong,*mut c_void)->c_int;
    fn blocking_notifier_chain_register(*mut c_void,*mut notifier_block)->c_int; fn blocking_notifier_chain_unregister(*mut c_void,*mut notifier_block)->c_int;
    fn atomic_notifier_chain_register(*mut c_void,*mut notifier_block)->c_int; fn atomic_notifier_chain_unregister(*mut c_void,*mut notifier_block)->c_int;
    fn atomic_notifier_chain_call_chain_is_empty(*mut c_void)->bool;
    fn usermodehelper_disable(); fn device_shutdown(); fn syscore_shutdown(); fn cpu_hotplug_disable(); fn cpu_online(c_int)->bool; fn cpumask_first(*mut c_void)->c_int;
    fn set_cpus_allowed_ptr(*mut c_void,*mut c_void); static mut current:*mut c_void; static mut cpu_online_mask:*mut c_void;
    fn kernel_kexec()->c_int; fn hibernate()->c_int; fn reboot_pid_ns(*mut pid_namespace,c_uint)->c_int; fn do_exit(c_int);
    fn ns_capable(*mut c_void,c_int)->bool; fn task_active_pid_ns(*mut c_void)->*mut pid_namespace; fn strncpy_from_user(*mut c_char,*mut c_void,usize)->isize;
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); static mut system_transition_mutex:*mut c_void;
    fn schedule_work(*mut work_struct); fn kill_cad_pid(c_int,c_int); fn argv_split(*mut c_void,*const c_char,*mut c_void)->*mut *mut c_char; fn argv_free(*mut *mut c_char); fn call_usermodehelper(*mut c_char,*mut *mut c_char,*mut *mut c_char,c_int)->c_int; fn emergency_sync(); fn pr_flush(c_int,bool);
    fn schedule_delayed_work(*mut c_void,c_ulong); fn msecs_to_jiffies(c_int)->c_ulong; fn sysfs_streq(*const c_char,*const c_char)->bool; fn capable(c_int)->bool;
    fn num_possible_cpus()->c_int; fn simple_strtoul(*const c_char,*mut *mut c_char,c_uint)->c_ulong; fn strchr(*const c_char,c_int)->*mut c_char; fn isdigit(c_int)->c_int;
    fn sysfs_emit(*mut c_char,*const c_char,...)->isize; fn kstrtobool(*const c_char,*mut bool)->c_int; fn kstrtouint(*const c_char,c_uint,*mut c_uint)->c_int;
    fn register_sysctl_init(*const c_char,*const c_void); fn kobject_create_and_add(*const c_char,*mut kobject)->*mut kobject; fn sysfs_create_group(*mut kobject,*const c_void)->c_int; fn kobject_put(*mut kobject);
}

static mut C_A_D:c_int=1; pub static mut cad_pid:*mut pid=core::ptr::null_mut();
static mut hw_protection_action:HwAction=0; static mut poweroff_fallback_to_halt:bool=false;
static mut poweroff_cmd:[c_char;256]=[0;256]; static reboot_cmd:&[u8]=b"/sbin/reboot\0"; static mut poweroff_force:bool=false; static mut hw_failure_emergency_action:HwAction=0;

pub unsafe extern "C" fn emergency_restart(){ kmsg_dump(0); system_state=1; machine_emergency_restart(); }
pub unsafe extern "C" fn kernel_restart_prepare(cmd:*mut c_char){ blocking_notifier_call_chain(core::ptr::null_mut(),0,cmd.cast()); system_state=1; usermodehelper_disable(); device_shutdown(); }
pub unsafe extern "C" fn register_reboot_notifier(nb:*mut notifier_block)->c_int{ blocking_notifier_chain_register(core::ptr::null_mut(),nb) }
pub unsafe extern "C" fn unregister_reboot_notifier(nb:*mut notifier_block)->c_int{ blocking_notifier_chain_unregister(core::ptr::null_mut(),nb) }
pub unsafe extern "C" fn register_restart_handler(nb:*mut notifier_block)->c_int{ atomic_notifier_chain_register(core::ptr::null_mut(),nb) }
pub unsafe extern "C" fn unregister_restart_handler(nb:*mut notifier_block)->c_int{ atomic_notifier_chain_unregister(core::ptr::null_mut(),nb) }
pub unsafe extern "C" fn do_kernel_restart(cmd:*mut c_char){ atomic_notifier_call_chain(core::ptr::null_mut(),reboot_mode as c_ulong,cmd.cast()); }
pub unsafe extern "C" fn migrate_to_reboot_cpu(){ cpu_hotplug_disable(); let mut cpu=reboot_cpu; if !cpu_online(cpu){cpu=cpumask_first(cpu_online_mask);} set_cpus_allowed_ptr(current,core::ptr::null_mut()); }
pub unsafe extern "C" fn kernel_restart(cmd:*mut c_char){ kernel_restart_prepare(cmd); migrate_to_reboot_cpu(); syscore_shutdown(); kmsg_dump(0); machine_restart(cmd); }
pub unsafe extern "C" fn kernel_halt(){ system_state=2; usermodehelper_disable(); device_shutdown(); migrate_to_reboot_cpu(); syscore_shutdown(); kmsg_dump(0); machine_halt(); }
pub unsafe extern "C" fn kernel_can_power_off()->bool{ !atomic_notifier_chain_call_chain_is_empty(core::ptr::null_mut()) || pm_power_off.is_some() }
pub unsafe extern "C" fn kernel_power_off(){ system_state=3; usermodehelper_disable(); device_shutdown(); migrate_to_reboot_cpu(); syscore_shutdown(); pr_flush(1000,true); kmsg_dump(0); machine_power_off(); }

pub unsafe extern "C" fn orderly_reboot(){ schedule_work(core::ptr::null_mut()); }
pub unsafe extern "C" fn orderly_poweroff(force:bool){ if force{poweroff_force=true;} schedule_work(core::ptr::null_mut()); }
pub unsafe extern "C" fn __hw_protection_trigger(_reason:*const c_char,_ms:c_int,_action:HwAction){ if hw_failure_emergency_action==0 { hw_failure_emergency_action=hw_protection_action; } if hw_failure_emergency_action==1 { orderly_reboot(); } else { orderly_poweroff(true); } }

pub unsafe extern "C" fn register_sys_off_handler(mode:c_ulong, priority:c_int, callback:Option<unsafe extern "C" fn(*mut sys_off_data)->c_int>, data:*mut c_void)->*mut sys_off_handler {
    let h=Box::into_raw(Box::new(sys_off_handler{nb:notifier_block{notifier_call:None,priority},sys_off_cb:callback,cb_data:data,mode,blocking:mode==0||mode==2,list:core::ptr::null_mut(),dev:core::ptr::null_mut()})); h
}
pub unsafe extern "C" fn unregister_sys_off_handler(handler:*mut sys_off_handler){ if !handler.is_null(){drop(Box::from_raw(handler));} }
pub unsafe extern "C" fn devm_register_sys_off_handler(_dev:*mut device,mode:c_ulong,priority:c_int,callback:Option<unsafe extern "C" fn(*mut sys_off_data)->c_int>,data:*mut c_void)->c_int { let h=register_sys_off_handler(mode,priority,callback,data); if h.is_null(){-12}else{0} }
pub unsafe extern "C" fn devm_register_power_off_handler(dev:*mut device,cb:Option<unsafe extern "C" fn(*mut sys_off_data)->c_int>,data:*mut c_void)->c_int { devm_register_sys_off_handler(dev,1,128,cb,data) }
pub unsafe extern "C" fn devm_register_restart_handler(dev:*mut device,cb:Option<unsafe extern "C" fn(*mut sys_off_data)->c_int>,data:*mut c_void)->c_int { devm_register_sys_off_handler(dev,3,128,cb,data) }
static mut platform_power_off_handler:*mut sys_off_handler=core::ptr::null_mut();
pub unsafe extern "C" fn register_platform_power_off(cb:Option<unsafe extern "C" fn()>)->c_int { platform_power_off_handler=register_sys_off_handler(1,0, None, cb.map(|f|f as *mut c_void).unwrap_or(core::ptr::null_mut())); if platform_power_off_handler.is_null(){-12}else{0} }
pub unsafe extern "C" fn unregister_platform_power_off(_cb:Option<unsafe extern "C" fn()>){ if !platform_power_off_handler.is_null(){unregister_sys_off_handler(platform_power_off_handler);platform_power_off_handler=core::ptr::null_mut();} }
pub unsafe extern "C" fn do_kernel_power_off(){ atomic_notifier_call_chain(core::ptr::null_mut(),0,core::ptr::null_mut()); }
pub unsafe extern "C" fn ctrl_alt_del(){ if C_A_D!=0 {schedule_work(core::ptr::null_mut());} else {kill_cad_pid(2,1);} }
pub unsafe extern "C" fn reboot_setup(_str:*mut c_char)->c_int { reboot_default=0; 1 }
pub unsafe extern "C" fn hw_protection_setup(_str:*mut c_char)->c_int {1}
pub unsafe extern "C" fn reboot_ksysfs_init()->c_int {0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
