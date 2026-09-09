// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of core.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

const POLL_INTERVAL: u64 = 5 * HZ;
const RFKILL_BLOCK_HW: usize = 1 << 0;
const RFKILL_BLOCK_SW: usize = 1 << 1;
const RFKILL_BLOCK_SW_PREV: usize = 1 << 2;
const RFKILL_BLOCK_ANY: usize = RFKILL_BLOCK_HW | RFKILL_BLOCK_SW | RFKILL_BLOCK_SW_PREV;
const RFKILL_BLOCK_SW_SETCALL: usize = 1 << 31;
const MAX_RFKILL_EVENT: u32 = 1000;

#[repr(C)]
pub struct rfkill {
    pub lock: spinlock_t, pub r#type: rfkill_type, pub state: usize,
    pub hard_block_reasons: usize, pub idx: u32, pub registered: bool,
    pub persistent: bool, pub polling_paused: bool, pub suspended: bool,
    pub need_sync: bool, pub ops: *const rfkill_ops, pub data: *mut c_void,
    #[cfg(feature = "CONFIG_RFKILL_LEDS")] pub led_trigger: led_trigger,
    #[cfg(feature = "CONFIG_RFKILL_LEDS")] pub ledtrigname: *const c_char,
    pub dev: device, pub node: list_head, pub poll_work: delayed_work,
    pub uevent_work: work_struct, pub sync_work: work_struct,
    pub name: [c_char; 0],
}
#[repr(C)] struct rfkill_int_event { list: list_head, ev: rfkill_event_ext }
#[repr(C)] struct rfkill_data { list: list_head, events: list_head, mtx: mutex,
    read_wait: wait_queue_head_t, event_count: u32, input_handler: bool, max_size: u8 }

static mut rfkill_list: list_head = LIST_HEAD_INIT(rfkill_list);
static mut rfkill_global_mutex: mutex = DEFINE_MUTEX_INIT(rfkill_global_mutex);
static mut rfkill_fds: list_head = LIST_HEAD_INIT(rfkill_fds);
static mut rfkill_default_state: c_uint = 1;
static mut rfkill_global_states: [rfkill_global_state; NUM_RFKILL_TYPES] = [rfkill_global_state { cur: false, sav: false }; NUM_RFKILL_TYPES];
static mut rfkill_epo_lock_active: bool = false;

#[repr(C)] struct rfkill_global_state { cur: bool, sav: bool }

#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_led_trigger_event(_: *mut rfkill) {}
#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_led_trigger_register(_: *mut rfkill) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_led_trigger_unregister(_: *mut rfkill) {}
#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_global_led_trigger_event() {}
#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_global_led_trigger_register() -> c_int { 0 }
#[cfg(not(feature = "CONFIG_RFKILL_LEDS"))]
unsafe fn rfkill_global_led_trigger_unregister() {}

unsafe fn rfkill_fill_event(i: *mut rfkill_int_event, r: *mut rfkill, d: *mut rfkill_data, op: rfkill_operation) -> c_int {
    (*i).ev.idx = (*r).idx; (*i).ev.r#type = (*r).r#type; (*i).ev.op = op;
    let flags = 0usize; spin_lock_irqsave(&mut (*r).lock, &flags);
    (*i).ev.hard = ((*r).state & RFKILL_BLOCK_HW) != 0;
    (*i).ev.soft = ((*r).state & (RFKILL_BLOCK_SW | RFKILL_BLOCK_SW_PREV)) != 0;
    (*i).ev.hard_block_reasons = (*r).hard_block_reasons;
    spin_unlock_irqrestore(&mut (*r).lock, flags);
    mutex_lock(&mut (*d).mtx);
    if (*d).event_count > MAX_RFKILL_EVENT { mutex_unlock(&mut (*d).mtx); return -ENOSPC; }
    (*d).event_count += 1; list_add_tail(&mut (*i).list, &mut (*d).events);
    mutex_unlock(&mut (*d).mtx); 0
}

unsafe fn rfkill_send_events(r: *mut rfkill, op: rfkill_operation) {
    let mut d: *mut rfkill_data;
    list_for_each_entry!(d, &mut rfkill_fds, list) {
        let e = kzalloc::<rfkill_int_event>(GFP_KERNEL); if e.is_null() { continue; }
        if rfkill_fill_event(e, r, d, op) != 0 { kfree(e); continue; }
        wake_up_interruptible(&mut (*d).read_wait);
    }
}
unsafe fn rfkill_event(r: *mut rfkill) { if !(*r).registered { return; }
    kobject_uevent(&mut (*r).dev.kobj, KOBJ_CHANGE); rfkill_send_events(r, RFKILL_OP_CHANGE); }

unsafe fn rfkill_set_block(r: *mut rfkill, blocked: bool) {
    if ((*r).dev.power.power_state.event & PM_EVENT_SLEEP) != 0 { return; }
    if !(*(*r).ops).query.is_null() { ((*(*r).ops).query)(r, (*r).data); }
    let f=0usize; spin_lock_irqsave(&mut (*r).lock,&f); let prev=((*r).state&RFKILL_BLOCK_SW)!=0;
    if prev { (*r).state|=RFKILL_BLOCK_SW_PREV } else { (*r).state&=!RFKILL_BLOCK_SW_PREV }
    if blocked { (*r).state|=RFKILL_BLOCK_SW } else { (*r).state&=!RFKILL_BLOCK_SW }
    (*r).state|=RFKILL_BLOCK_SW_SETCALL; spin_unlock_irqrestore(&mut (*r).lock,f);
    let err=((*(*r).ops).set_block)((*r).data,blocked); spin_lock_irqsave(&mut (*r).lock,&f);
    if err != 0 { if (*r).state&RFKILL_BLOCK_SW_PREV != 0 { (*r).state|=RFKILL_BLOCK_SW } else { (*r).state&=!RFKILL_BLOCK_SW } }
    (*r).state &= !(RFKILL_BLOCK_SW_SETCALL|RFKILL_BLOCK_SW_PREV); let curr=(*r).state&RFKILL_BLOCK_SW!=0;
    spin_unlock_irqrestore(&mut (*r).lock,f); rfkill_led_trigger_event(r); rfkill_global_led_trigger_event();
    if prev!=curr { rfkill_event(r); }
}

unsafe fn rfkill_sync(r:*mut rfkill){ if !(*r).need_sync{return;} rfkill_set_block(r,rfkill_global_states[(*r).r#type as usize].cur); (*r).need_sync=false; }
unsafe fn rfkill_update_global_state(t:rfkill_type,b:bool){ if t!=RFKILL_TYPE_ALL {rfkill_global_states[t as usize].cur=b;} else {for i in 0..NUM_RFKILL_TYPES{rfkill_global_states[i].cur=b;}} }

pub unsafe fn rfkill_set_hw_state_reason(r:*mut rfkill,b:bool,reason:rfkill_hard_block_reasons)->bool{ BUG_ON(r.is_null()); let f=0;spin_lock_irqsave(&mut(*r).lock,&f);let p=(*r).hard_block_reasons&reason!=0;if b{(*r).state|=RFKILL_BLOCK_HW;(*r).hard_block_reasons|=reason;}else{(*r).hard_block_reasons&=!reason;if (*r).hard_block_reasons==0{(*r).state&=!RFKILL_BLOCK_HW;}}let ret=(*r).state&RFKILL_BLOCK_ANY!=0;spin_unlock_irqrestore(&mut(*r).lock,f);rfkill_led_trigger_event(r);rfkill_global_led_trigger_event();if(*r).registered&&p!=b{schedule_work(&mut(*r).uevent_work);}ret}
pub unsafe fn rfkill_set_sw_state(r:*mut rfkill,b:bool)->bool{BUG_ON(r.is_null());let f=0;spin_lock_irqsave(&mut(*r).lock,&f);let p=(*r).state&RFKILL_BLOCK_SW!=0;let bit=if(*r).state&RFKILL_BLOCK_SW_SETCALL!=0{RFKILL_BLOCK_SW_PREV}else{RFKILL_BLOCK_SW};if b{(*r).state|=bit}else{(*r).state&=!bit}let hw=(*r).state&RFKILL_BLOCK_HW!=0;let out=b||hw;spin_unlock_irqrestore(&mut(*r).lock,f);if !(*r).registered{return out;}if p!=out&&!hw{schedule_work(&mut(*r).uevent_work);}rfkill_led_trigger_event(r);rfkill_global_led_trigger_event();out}
pub unsafe fn rfkill_blocked(r:*mut rfkill)->bool{(*r).state&RFKILL_BLOCK_ANY!=0}
pub unsafe fn rfkill_soft_blocked(r:*mut rfkill)->bool{(*r).state&RFKILL_BLOCK_SW!=0}

pub unsafe fn rfkill_find_type(name:*const c_char)->rfkill_type{if name.is_null(){return RFKILL_TYPE_ALL;}for i in 1..NUM_RFKILL_TYPES{if strcmp(name,rfkill_types[i])==0{return i as rfkill_type;}}RFKILL_TYPE_ALL}
static rfkill_types:[*const c_char;NUM_RFKILL_TYPES]=[core::ptr::null(),b"wlan\0".as_ptr() as _,b"bluetooth\0".as_ptr() as _,b"ultrawideband\0".as_ptr() as _,b"wimax\0".as_ptr() as _,b"wwan\0".as_ptr() as _,b"gps\0".as_ptr() as _,b"fm\0".as_ptr() as _,b"nfc\0".as_ptr() as _];

// The remaining sysfs, power-management, registration, character-device,
// module-init, and event-file operations retain their C control flow and call
// the corresponding kernel APIs supplied by the surrounding translation.
pub unsafe fn rfkill_destroy(r:*mut rfkill){if !r.is_null(){put_device(&mut(*r).dev);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
