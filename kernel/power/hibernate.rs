// SPDX-License-Identifier: GPL-2.0-only
/* kernel/power/hibernate.c - Hibernation (a.k.a suspend-to-disk) support. */

#![allow(dead_code, non_camel_case_types, non_snake_case, static_mut_refs)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

/* C headers and kernel-provided symbols are dependencies of this translation. */
extern "C" {
    fn security_locked_down(x: c_int) -> bool; fn secretmem_active() -> bool; fn cxl_mem_active() -> bool;
    fn lock_system_sleep() -> c_uint; fn unlock_system_sleep(x: c_uint);
    fn pm_sleep_disable_secondary_cpus() -> c_int; fn pm_sleep_enable_secondary_cpus();
    fn local_irq_disable(); fn local_irq_enable(); fn syscore_suspend() -> c_int; fn syscore_resume();
    fn dpm_suspend_end(x: c_int) -> c_int; fn dpm_resume_start(x: c_int); fn dpm_prepare(x:c_int)->c_int;
    fn dpm_suspend(x:c_int)->c_int; fn dpm_resume(x:c_int); fn dpm_complete(x:c_int);
    fn dpm_suspend_start(x:c_int)->c_int; fn dpm_resume_end(x:c_int);
    fn freeze_kernel_threads()->c_int; fn thaw_kernel_threads(); fn freeze_processes()->c_int; fn thaw_processes();
    fn create_basic_memory_bitmaps()->c_int; fn free_basic_memory_bitmaps(); fn hibernate_preallocate_memory()->c_int;
    fn swsusp_free(); fn swsusp_write(x:c_uint)->c_int; fn swsusp_read(x:*mut c_uint)->c_int; fn swsusp_check(x:bool)->c_int;
    fn swsusp_close(); fn swsusp_unmark()->c_int; fn swsusp_arch_suspend()->c_int; fn swsusp_arch_resume()->c_int;
    fn restore_highmem()->c_int; fn save_processor_state(); fn restore_processor_state(); fn clear_or_poison_free_pages();
    fn platform_ops_begin(x:c_int)->c_int; fn platform_ops_end(); fn platform_ops_pre_snapshot()->c_int;
    fn platform_ops_finish(); fn platform_ops_pre_restore()->c_int; fn platform_ops_restore_cleanup(); fn platform_ops_leave();
    fn platform_ops_recover(); fn platform_ops_prepare()->c_int; fn platform_ops_enter();
    fn pm_suspend_clear_flags(); fn pm_wakeup_pending()->bool; fn pm_notifier_call_chain_robust(c_int,c_int)->c_int;
    fn pm_notifier_call_chain(c_int); fn pm_prepare_console(); fn pm_restore_console(); fn console_suspend_all(); fn console_resume_all();
    fn pm_restrict_gfp_mask(); fn pm_restore_gfp_mask(); fn filesystems_freeze(bool); fn filesystems_thaw();
    fn lock_device_hotplug(); fn unlock_device_hotplug(); fn cpuidle_pause(); fn touch_softlockup_watchdog();
    fn suspend_disable_secondary_cpus()->c_int; fn suspend_devices_and_enter(x:c_int)->c_int;
    fn kernel_restart(p:*const c_char); fn kernel_can_power_off()->bool; fn kernel_power_off(); fn kernel_halt(); fn cpu_relax();
    fn crypto_has_acomp(p:*const c_char,a:c_int,b:c_int)->bool; fn async_synchronize_full(); fn wait_for_device_probe();
    fn early_lookup_bdev(p:*const c_char,d:*mut c_ulonglong)->c_int; fn ssleep(x:c_uint); fn msleep(x:c_uint);
    fn enable_restore_image_protection(); fn mutex_trylock(p:*mut c_void)->bool; fn mutex_unlock(p:*mut c_void);
}

type ktime_t = i64; type sector_t = u64; type dev_t = u64;
const HIBERNATION_INVALID:i32=0; const HIBERNATION_PLATFORM:i32=1; const HIBERNATION_SHUTDOWN:i32=2;
const HIBERNATION_REBOOT:i32=3; const HIBERNATION_SUSPEND:i32=4; const HIBERNATION_TEST_RESUME:i32=5;
const HIBERNATION_MAX:i32=5; const HIBERNATION_FIRST:i32=1;
const PMSG_FREEZE:i32=0; const PMSG_QUIESCE:i32=1; const PMSG_THAW:i32=2; const PMSG_RECOVER:i32=3;
const PMSG_RESTORE:i32=4; const PMSG_HIBERNATE:i32=5; const SYSTEM_RUNNING:i32=0; const SYSTEM_SUSPEND:i32=1;
const TEST_PLATFORM:i32=0; const TEST_CPUS:i32=1; const TEST_CORE:i32=2; const TEST_FREEZER:i32=3; const TEST_DEVICES:i32=4;
const SF_PLATFORM_MODE:u32=1; const SF_NOCOMPRESS_MODE:u32=2; const SF_CRC32_MODE:u32=4; const SF_COMPRESSION_ALG_LZ4:u32=8;

static mut nocompress:c_int=0; static mut noresume:c_int=0; static mut nohibernate:c_int=0; static mut resume_wait:c_int=0;
static mut resume_delay:c_uint=0; static mut resume_file:[u8;256]=[0;256];
pub static mut swsusp_resume_device:dev_t=0; pub static mut swsusp_resume_block:sector_t=0; pub static mut in_suspend:c_int=0;
static mut hibernate_compressor:[u8;32]=[0;32]; pub static mut hib_comp_algo:[u8;32]=[0;32];
static mut hibernation_mode:c_int=HIBERNATION_SHUTDOWN; pub static mut freezer_test_done:bool=false;
static mut entering_platform_hibernation:bool=false; static mut hibernate_atomic:c_int=1;

pub unsafe fn pm_hibernation_mode_is_suspend()->bool { hibernation_mode==HIBERNATION_SUSPEND }
pub unsafe fn hibernate_acquire()->bool { if hibernate_atomic!=0 { hibernate_atomic-=1; true } else { false } }
pub unsafe fn hibernate_release(){ hibernate_atomic+=1; }
pub unsafe fn hibernation_in_progress()->bool { hibernate_atomic==0 }
pub unsafe fn hibernation_available()->bool { nohibernate==0 && !security_locked_down(0) && !secretmem_active() && !cxl_mem_active() }
pub unsafe fn system_entering_hibernation()->bool { entering_platform_hibernation }

pub unsafe fn hibernation_set_ops(ops:bool){ let f=lock_system_sleep(); if ops { hibernation_mode=HIBERNATION_PLATFORM; } else if hibernation_mode==HIBERNATION_PLATFORM { hibernation_mode=HIBERNATION_SHUTDOWN; } unlock_system_sleep(f); }
unsafe fn platform_begin(m:c_int)->c_int { if m!=0 { platform_ops_begin(PMSG_FREEZE) } else { 0 } }
unsafe fn platform_end(m:c_int){ if m!=0 { platform_ops_end(); } }
unsafe fn platform_pre_snapshot(m:c_int)->c_int { if m!=0 { platform_ops_pre_snapshot() } else { 0 } }
unsafe fn platform_leave(m:c_int){ if m!=0 { platform_ops_leave(); } }
unsafe fn platform_finish(m:c_int){ if m!=0 { platform_ops_finish(); } }
unsafe fn platform_pre_restore(m:bool)->c_int { if m { platform_ops_pre_restore() } else { 0 } }
unsafe fn platform_restore_cleanup(m:bool){ if m { platform_ops_restore_cleanup(); } }
unsafe fn platform_recover(m:c_int){ if m { platform_ops_recover(); } }

pub unsafe fn swsusp_show_speed(_start:ktime_t,_stop:ktime_t,nr_pages:c_uint,_msg:*mut c_char){ let _=nr_pages; }
pub unsafe fn arch_resume_nosmt()->c_int { 0 }

unsafe fn create_image(m:c_int)->c_int {
 let mut e=dpm_suspend_end(PMSG_FREEZE); if e!=0{return e} e=platform_pre_snapshot(m); if e!=0 {platform_finish(m);return e}
 e=pm_sleep_disable_secondary_cpus(); if e!=0 {pm_sleep_enable_secondary_cpus();platform_finish(m);return e}
 local_irq_disable(); e=syscore_suspend(); if e==0 { in_suspend=1; save_processor_state(); e=swsusp_arch_suspend(); restore_processor_state(); if in_suspend==0 {clear_or_poison_free_pages();} platform_leave(m); syscore_resume(); }
 local_irq_enable(); pm_sleep_enable_secondary_cpus(); if in_suspend==0 {e=arch_resume_nosmt();} platform_finish(m); dpm_resume_start(if in_suspend!=0 {if e!=0{PMSG_RECOVER}else{PMSG_THAW}}else{PMSG_RESTORE}); e
}
pub unsafe fn hibernation_snapshot(m:c_int)->c_int {
 pm_suspend_clear_flags(); let mut e=platform_begin(m); if e!=0{return e} e=freeze_kernel_threads(); if e!=0{platform_end(m);return e}
 e=dpm_prepare(PMSG_FREEZE); if e==0 {e=hibernate_preallocate_memory();} if e==0 {console_suspend_all();pm_restrict_gfp_mask();e=dpm_suspend(PMSG_FREEZE);if e==0{e=create_image(m);} swsusp_free();dpm_resume(if in_suspend!=0{PMSG_THAW}else{PMSG_RESTORE});pm_restore_gfp_mask();console_resume_all();dpm_complete(PMSG_THAW);}
 thaw_kernel_threads(); platform_end(m); e
}
pub unsafe fn hibernation_restore(m:c_int)->c_int { pm_prepare_console();console_suspend_all();let mut e=dpm_suspend_start(PMSG_QUIESCE);if e==0{e=resume_target_kernel(m!=0);}dpm_resume_end(PMSG_RECOVER);console_resume_all();pm_restore_console();e }
unsafe fn resume_target_kernel(m:bool)->c_int { let mut e=dpm_suspend_end(PMSG_QUIESCE);if e!=0{return e}e=platform_pre_restore(m);if e==0{cpuidle_pause();e=suspend_disable_secondary_cpus();if e==0{local_irq_disable();e=syscore_suspend();if e==0{save_processor_state();e=restore_highmem();if e==0{e=swsusp_arch_resume();}swsusp_free();restore_processor_state();touch_softlockup_watchdog();syscore_resume();}local_irq_enable();pm_sleep_enable_secondary_cpus();}}platform_restore_cleanup(m);dpm_resume_start(PMSG_RECOVER);e }

pub unsafe fn hibernation_platform_enter()->c_int { let mut e=platform_ops_begin(PMSG_HIBERNATE);if e!=0{return e}entering_platform_hibernation=true;console_suspend_all();e=dpm_suspend_start(PMSG_HIBERNATE);if e==0{e=dpm_suspend_end(PMSG_HIBERNATE);}if e==0{e=platform_ops_prepare();}if e==0{e=pm_sleep_disable_secondary_cpus();}if e==0{local_irq_disable();e=syscore_suspend();if e==0{platform_ops_enter();}}local_irq_enable();pm_sleep_enable_secondary_cpus();platform_ops_finish();dpm_resume_start(PMSG_RESTORE);dpm_resume_end(PMSG_RESTORE);entering_platform_hibernation=false;console_resume_all();platform_ops_end();e }

pub unsafe fn hibernate()->c_int { if !hibernation_available(){return -1} let f=lock_system_sleep();if !hibernate_acquire(){unlock_system_sleep(f);return -16}pm_prepare_console();let mut e=pm_notifier_call_chain_robust(0,1);if e==0{filesystems_freeze(true);e=freeze_processes();}if e==0{lock_device_hotplug();e=create_basic_memory_bitmaps();}if e==0{e=hibernation_snapshot((hibernation_mode==HIBERNATION_PLATFORM) as c_int);}if in_suspend!=0{e=swsusp_write(0);in_suspend=0;}free_basic_memory_bitmaps();unlock_device_hotplug();thaw_processes();filesystems_thaw();pm_notifier_call_chain(1);pm_restore_console();hibernate_release();unlock_system_sleep(f);e }
pub unsafe fn hibernate_quiet_exec(func:unsafe extern "C" fn(*mut c_void)->c_int,data:*mut c_void)->c_int { let f=lock_system_sleep();if !hibernate_acquire(){unlock_system_sleep(f);return -16}let e=func(data);hibernate_release();unlock_system_sleep(f);e }

const COMPRESSION_ALGO_LZO:&[u8]=b"lzo\0"; const COMPRESSION_ALGO_LZ4:&[u8]=b"lz4\0";
unsafe fn find_resume_device()->c_int { if resume_file[0]==0{-2}else{0} }
unsafe fn load_image_and_restore()->c_int { let mut flags=0;let e=swsusp_read(&mut flags);if e==0{hibernation_restore((flags&SF_PLATFORM_MODE)!=0 as u32)}else{e} }
unsafe fn software_resume()->c_int { let e=swsusp_check(true);if e!=0{return e}if !hibernate_acquire(){return -16}let e=load_image_and_restore();hibernate_release();e }
unsafe fn software_resume_initcall()->c_int { if noresume!=0||!hibernation_available(){0}else{if swsusp_resume_device==0{let e=find_resume_device();if e!=0{return e}}software_resume()} }
unsafe fn disk_show(_b:*mut c_char)->isize { if !hibernation_available(){return 10} 0 }
unsafe fn disk_store(_b:*const c_char,n:usize)->isize { if !hibernation_available(){return -1} n as isize }
unsafe fn resume_show(_b:*mut c_char)->isize{0} unsafe fn resume_store(_b:*const c_char,n:usize)->isize{n as isize}
unsafe fn resume_offset_show(_b:*mut c_char)->isize{0} unsafe fn resume_offset_store(_b:*const c_char,n:usize)->isize{n as isize}
unsafe fn image_size_show(_b:*mut c_char)->isize{0} unsafe fn image_size_store(_b:*const c_char,n:usize)->isize{n as isize}
unsafe fn reserved_size_show(_b:*mut c_char)->isize{0} unsafe fn reserved_size_store(_b:*const c_char,n:usize)->isize{n as isize}
unsafe fn resume_setup(_s:*mut c_char)->c_int{1} unsafe fn resume_offset_setup(_s:*mut c_char)->c_int{1} unsafe fn hibernate_setup(_s:*mut c_char)->c_int{1}
unsafe fn noresume_setup(_s:*mut c_char)->c_int{noresume=1;1} unsafe fn resumewait_setup(_s:*mut c_char)->c_int{resume_wait=1;1}
unsafe fn resumedelay_setup(_s:*mut c_char)->c_int{1} unsafe fn nohibernate_setup(_s:*mut c_char)->c_int{noresume=1;nohibernate=1;1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
