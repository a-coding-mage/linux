// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel-provided types, globals, constants, and functions are supplied by
// the surrounding translation unit.
extern "C" {
    static mut synth: *mut spk_synth;
    static mut spk_pitch_buff: [c_char; 32];
    static mut spk_quiet_boot: bool;
    static mut speakup_info: speakup_info_t;
    static mut speakup_event: c_void;
    static mut speakup_task: *mut c_void;
    static mut spk_pitch_shift: c_int;
    static mut synth_flags: c_uint;
    static mut spk_mutex: c_void;
    static mut ioport_resource: resource;
    static mut jiffies: c_ulong;
    fn spk_get_var(id: c_int) -> *mut var_t;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn kthread_should_stop() -> c_int;
    fn set_current_state(state: c_int);
    fn schedule_timeout(timeout: c_ulong);
    fn time_after_eq(a: c_ulong, b: c_ulong) -> bool;
    fn msecs_to_jiffies(ms: c_int) -> c_ulong;
    fn synth_buffer_skip_nonlatin1();
    fn synth_buffer_empty() -> bool;
    fn synth_buffer_peek() -> u16;
    fn synth_buffer_getc();
    fn synth_buffer_clear();
    fn synth_buffer_add(ch: u16);
    fn wake_up_interruptible_all(event: *mut c_void);
    fn wake_up_process(task: *mut c_void);
    fn timer_pending(timer: *mut timer_list) -> bool;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong);
    fn timer_delete(timer: *mut timer_list);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn request_resource(parent: *mut resource, child: *mut resource) -> c_int;
    fn release_resource(resource: *mut resource) -> c_int;
    fn sysfs_create_group(kobj: *mut c_void, group: *mut c_void) -> c_int;
    fn sysfs_remove_group(kobj: *mut c_void, group: *mut c_void);
    fn speakup_register_var(var: *mut var_t);
    fn speakup_unregister_var(id: c_int);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn vsnprintf(buf: *mut u8, size: usize, fmt: *const c_char, args: *mut c_void) -> c_int;
    fn __va_start(args: *mut c_void, last: *const c_void);
    fn __va_end(args: *mut c_void);
}

#[repr(C)] pub struct spk_synth { pub node: c_void, pub name: *const c_char, pub long_name: *const c_char, pub init: *const c_char, pub clear: u16, pub procspeech: u16, pub alive: c_int, pub delay: c_int, pub trigger: c_int, pub jiffies: c_int, pub full: c_int, pub flush_time: c_int, pub checkval: c_int, pub flags: c_uint, pub startup: bool, pub vars: *mut var_t, pub attributes: attributes, pub indexing: indexing, pub io_ops: *mut io_ops, pub probe: unsafe extern "C" fn(*mut spk_synth) -> c_int, pub release: unsafe extern "C" fn(*mut spk_synth), pub flush: unsafe extern "C" fn(*mut spk_synth), pub get_index: unsafe extern "C" fn(*mut spk_synth) -> c_int }
#[repr(C)] pub struct io_ops { pub synth_out_unicode: unsafe extern "C" fn(*mut spk_synth, u16) -> c_int, pub synth_out: unsafe extern "C" fn(*mut spk_synth, u16) -> c_int, pub flush_buffer: unsafe extern "C" fn(*mut spk_synth), pub synth_in_nowait: unsafe extern "C" fn(*mut spk_synth) -> u8, pub wait_for_xmitr: unsafe extern "C" fn(*mut spk_synth) -> c_int }
#[repr(C)] pub struct var_t { pub var_id: c_int, pub u: var_union }
#[repr(C)] pub union var_union { pub n: number_var }
#[repr(C)] pub struct number_var { pub data: *mut c_void, pub value: c_int, pub default_val: c_int, pub max: c_int, pub min: c_int, pub step: c_int, pub setter: *mut c_void }
#[repr(C)] pub struct indexing { pub currindex: c_int, pub highindex: c_int, pub lowindex: c_int, pub command: *const c_char }
#[repr(C)] pub struct attributes { pub name: *const c_char }
#[repr(C)] pub struct resource { pub name: *const c_char, pub start: c_ulong, pub end: c_ulong, pub flags: c_ulong }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct speakup_info_t { pub spinlock: c_void, pub flushing: c_int }

static mut synths: c_void = c_void;
static mut module_status: c_int = 0;
static mut thread_timer: *mut timer_list = core::ptr::null_mut();
static mut index_count: c_int = 0;
static mut sentence_count: c_int = 0;
static mut synth_res: resource = resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 };
unsafe extern "C" fn thread_wake_up(_unused:*mut timer_list){wake_up_interruptible_all(&mut speakup_event)}

#[repr(C)] pub struct timer_var { pub id:c_int, pub n:number_var }
#[no_mangle] pub static mut synth_time_vars:[var_t;5]=[
    var_t{var_id:DELAY,u:var_union{n:number_var{data:core::ptr::null_mut(),value:100,default_val:100,max:2000,min:0,step:0,setter:core::ptr::null_mut()}}},
    var_t{var_id:TRIGGER,u:var_union{n:number_var{data:core::ptr::null_mut(),value:20,default_val:10,max:2000,min:0,step:0,setter:core::ptr::null_mut()}}},
    var_t{var_id:JIFFY,u:var_union{n:number_var{data:core::ptr::null_mut(),value:50,default_val:20,max:200,min:0,step:0,setter:core::ptr::null_mut()}}},
    var_t{var_id:FULL,u:var_union{n:number_var{data:core::ptr::null_mut(),value:400,default_val:200,max:60000,min:0,step:0,setter:core::ptr::null_mut()}}},
    var_t{var_id:FLUSH,u:var_union{n:number_var{data:core::ptr::null_mut(),value:4000,default_val:10,max:4000,min:0,step:0,setter:core::ptr::null_mut()}}},
];

unsafe fn do_synth_init(in_synth:*mut spk_synth)->c_int { synth_release(); if (*in_synth).checkval!=SYNTH_CHECK{return -EINVAL;} synth=in_synth;(*synth).alive=0;pr_warn(b"synth probe\n\0".as_ptr() as _);if ((*synth).probe)(synth)<0{pr_warn(b"%s: device probe failed\n\0".as_ptr() as _,(*in_synth).name);synth=core::ptr::null_mut();return -ENODEV;} synth_time_vars[0].u.n.value=(*synth).delay;synth_time_vars[0].u.n.default_val=(*synth).delay;synth_time_vars[1].u.n.value=(*synth).trigger;synth_time_vars[1].u.n.default_val=(*synth).trigger;synth_time_vars[2].u.n.value=(*synth).jiffies;synth_time_vars[2].u.n.default_val=(*synth).jiffies;synth_time_vars[3].u.n.value=(*synth).full;synth_time_vars[3].u.n.default_val=(*synth).full;synth_time_vars[4].u.n.value=(*synth).flush_time;synth_time_vars[4].u.n.default_val=(*synth).flush_time;synth_printf((*synth).init);let mut v=(*synth).vars;while (*v).var_id>=0&&(*v).var_id<MAXVARS{speakup_register_var(v);v=v.add(1)};if !spk_quiet_boot{synth_printf(b"%s found\n\0".as_ptr() as _,(*synth).long_name);}synth_flags=(*synth).flags;wake_up_interruptible_all(&mut speakup_event);if !speakup_task.is_null(){wake_up_process(speakup_task)};0}
pub unsafe extern "C" fn synth_init(name:*mut c_char)->c_int{if name.is_null(){return 0} if strcmp(name,b"none\0".as_ptr() as _)==0{mutex_lock(&mut spk_mutex);synth_release();mutex_unlock(&mut spk_mutex);return 0} mutex_lock(&mut spk_mutex);let r=if !synth.is_null()&&strcmp((*synth).name,name)==0{do_synth_init(synth)}else{-ENODEV};mutex_unlock(&mut spk_mutex);r}
pub unsafe extern "C" fn synth_release(){if synth.is_null(){return}let mut flags=0;spin_lock_irqsave(&mut speakup_info.spinlock,&mut flags);pr_info(b"releasing synth %s\n\0".as_ptr() as _,(*synth).name);(*synth).alive=0;timer_delete(thread_timer);spin_unlock_irqrestore(&mut speakup_info.spinlock,flags);let mut v=(*synth).vars;while (*v).var_id!=MAXVARS{speakup_unregister_var((*v).var_id);v=v.add(1)};((*synth).release)(synth);synth=core::ptr::null_mut()}
pub unsafe extern "C" fn synth_add(in_synth:*mut spk_synth)->c_int{mutex_lock(&mut spk_mutex);let status=if (*in_synth).startup{do_synth_init(in_synth)}else{0};mutex_unlock(&mut spk_mutex);status}
pub unsafe extern "C" fn synth_remove(in_synth:*mut spk_synth){mutex_lock(&mut spk_mutex);if synth==in_synth{synth_release()}module_status=0;mutex_unlock(&mut spk_mutex)}
pub unsafe extern "C" fn synth_current()->*mut spk_synth{synth}

unsafe fn _spk_do_catch_up(s: *mut spk_synth, unicode: c_int) {
    let jiffy_delta = spk_get_var(JIFFY); let full_time = spk_get_var(FULL); let delay_time = spk_get_var(DELAY);
    let mut flags = 0; spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags); let mut jd = (*jiffy_delta).u.n.value; spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    let mut jiff_max = jiffies + jd as c_ulong;
    while kthread_should_stop() == 0 {
        spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
        if speakup_info.flushing != 0 { speakup_info.flushing = 0; spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); ((*s).flush)(s); continue; }
        if unicode == 0 { synth_buffer_skip_nonlatin1(); }
        if synth_buffer_empty() { spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); break; }
        let mut ch = synth_buffer_peek(); set_current_state(TASK_INTERRUPTIBLE); let full = (*full_time).u.n.value; spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
        if ch == b'\n' as u16 { ch = (*s).procspeech; }
        let ret = if unicode != 0 { ((*(*s).io_ops).synth_out_unicode)(s, ch) } else { ((*(*s).io_ops).synth_out)(s, ch) };
        if ret == 0 { schedule_timeout(msecs_to_jiffies(full)); continue; }
        if time_after_eq(jiffies, jiff_max) && ch == SPACE { spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags); jd = (*jiffy_delta).u.n.value; let delay = (*delay_time).u.n.value; let full = (*full_time).u.n.value; spin_unlock_irqrestore(&mut speakup_info.spinlock, flags); let r = ((*(*s).io_ops).synth_out)(s, (*s).procspeech); schedule_timeout(msecs_to_jiffies(if r != 0 { delay } else { full })); jiff_max = jiffies + jd as c_ulong; }
        set_current_state(TASK_RUNNING); spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags); synth_buffer_getc(); spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    }
    ((*(*s).io_ops).synth_out)(s, (*s).procspeech);
}

pub unsafe extern "C" fn spk_do_catch_up(s: *mut spk_synth) { _spk_do_catch_up(s, 0); }
pub unsafe extern "C" fn spk_do_catch_up_unicode(s: *mut spk_synth) { _spk_do_catch_up(s, 1); }
pub unsafe extern "C" fn spk_synth_flush(s: *mut spk_synth) { ((*(*s).io_ops).flush_buffer)(s); ((*(*s).io_ops).synth_out)(s, (*s).clear); }
pub unsafe extern "C" fn spk_synth_get_index(s: *mut spk_synth) -> u8 { ((*(*s).io_ops).synth_in_nowait)(s) }
pub unsafe extern "C" fn spk_synth_is_alive_nop(s: *mut spk_synth) -> c_int { (*s).alive = 1; 1 }
pub unsafe extern "C" fn spk_synth_is_alive_restart(s: *mut spk_synth) -> c_int { if (*s).alive != 0 { return 1; } if ((*(*s).io_ops).wait_for_xmitr)(s) > 0 { (*s).alive = 1; synth_printf((*s).init); return 2; } pr_warn(b"%s: can't restart synth\n\0".as_ptr() as _, (*s).long_name); 0 }

pub unsafe extern "C" fn synth_start() { if synth.is_null() || (*synth).alive == 0 { synth_buffer_clear(); return; } let t = spk_get_var(TRIGGER); if !timer_pending(thread_timer) { mod_timer(thread_timer, jiffies + msecs_to_jiffies((*t).u.n.value)); } }
pub unsafe extern "C" fn spk_do_flush() { if synth.is_null() { return; } speakup_info.flushing = 1; synth_buffer_clear(); if (*synth).alive != 0 && spk_pitch_shift != 0 { synth_printf(spk_pitch_buff.as_ptr()); spk_pitch_shift = 0; } wake_up_interruptible_all(&mut speakup_event); wake_up_process(speakup_task); }
pub unsafe extern "C" fn synth_write(buf: *const c_char, mut count: usize) { let mut p = buf as *const u8; while count != 0 { synth_buffer_add(*p as u16); p = p.add(1); count -= 1; } synth_start(); }

pub unsafe extern "C" fn synth_utf8_get(buf: *const c_char, count: usize, consumed: *mut usize, want: *mut usize) -> i32 { let c = *buf as u8; let nbytes = 8 - ((c ^ 0xff).leading_zeros() as i32); let mut value: u32; if nbytes == 8 || nbytes == 7 || nbytes == 1 { *consumed=1; *want=1; return -1; } if nbytes == 0 { *consumed=1; *want=1; return c as i32; } if count < nbytes as usize { *consumed=0; *want=nbytes as usize; return -1; } value = (c as u32) & ((1u32 << (7 - nbytes)) - 1); for i in 1..nbytes { let d=*buf.add(i as usize) as u8; if d & 0xc0 != 0x80 { *consumed=i as usize; *want=1; return -1; } value=(value<<6)|(d&0x3f) as u32; } *consumed=nbytes as usize; *want=1; value as i32 }
pub unsafe extern "C" fn synth_writeu(buf: *const c_char, mut count: usize) { let mut i=0; while i<count { let mut consumed=0; let mut want=0; let value=synth_utf8_get(buf.add(i), count-i, &mut consumed, &mut want); if value == -1 { if want > count-i { count=i; } i += consumed; continue; } if value < 0x10000 { synth_buffer_add(value as u16); } i += consumed; } synth_start(); }

pub unsafe extern "C" fn synth_printf(fmt: *const c_char, ...) { let mut buf=[0u8;160]; let mut args=[0u8;64]; __va_start(args.as_mut_ptr() as _, fmt as _); let mut r=vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, args.as_mut_ptr() as _); __va_end(args.as_mut_ptr() as _); if r > 159 { r=159; } synth_writeu(buf.as_ptr() as _, r as usize); }
pub unsafe extern "C" fn synth_putwc(wc:u16){synth_buffer_add(wc)}
pub unsafe extern "C" fn synth_putwc_s(wc:u16){synth_buffer_add(wc);synth_start()}
pub unsafe extern "C" fn synth_putws(buf:*const u16){let mut p=buf;while *p != 0{synth_buffer_add(*p);p=p.add(1)}}
pub unsafe extern "C" fn synth_putws_s(buf:*const u16){synth_putws(buf);synth_start()}
pub unsafe extern "C" fn spk_reset_index_count(sc:c_int){static mut FIRST:c_int=1;if FIRST!=0{FIRST=0}else{((*synth).get_index)(synth)};index_count=0;sentence_count=sc}
pub unsafe extern "C" fn synth_supports_indexing()->c_int{if (*synth).get_index as usize != 0{1}else{0}}
pub unsafe extern "C" fn synth_insert_next_index(sent_num:c_int){if (*synth).alive!=0{if sent_num==0{(*synth).indexing.currindex+=1;index_count+=1;if (*synth).indexing.currindex>(*synth).indexing.highindex{(*synth).indexing.currindex=(*synth).indexing.lowindex}}let out=(*synth).indexing.currindex*10+sent_num;synth_printf((*synth).indexing.command,out,out)}}
pub unsafe extern "C" fn spk_get_index_count(linecount:*mut c_int,sentcount:*mut c_int){let ind=((*synth).get_index)(synth);if ind!=0{sentence_count=ind%10;if ind/10<=(*synth).indexing.currindex{index_count=(*synth).indexing.currindex-ind/10}else{index_count=(*synth).indexing.currindex-(*synth).indexing.lowindex+(*synth).indexing.highindex-ind/10+1}};*sentcount=sentence_count;*linecount=index_count}
pub unsafe extern "C" fn synth_request_region(start:c_ulong,n:c_ulong)->c_int{memset(&mut synth_res as *mut _ as _,0,core::mem::size_of::<resource>());synth_res.name=(*synth).name;synth_res.start=start;synth_res.end=start+n-1;synth_res.flags=IORESOURCE_BUSY;request_resource(&mut ioport_resource,&mut synth_res)}
pub unsafe extern "C" fn synth_release_region(_start:c_ulong,_n:c_ulong)->c_int{release_resource(&mut synth_res)}

#[no_mangle] pub static mut spk_punc_masks:[u16;5]=[0,SOME,MOST,PUNC,PUNC|B_SYM];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
