// SPDX-License-Identifier: GPL-2.0
/* Renesas RZ/G2L MTU3a Counter driver */

// External Linux-kernel and RZ-MTU3 declarations are supplied by other files.
use core::ffi::{c_char, c_void};

const RZ_MTU3_TSR_TCFD: u32 = 1 << 7;
const RZ_MTU3_TMDR1_PH_CNT_MODE_1: u8 = 4;
const RZ_MTU3_TMDR1_PH_CNT_MODE_2: u8 = 5;
const RZ_MTU3_TMDR1_PH_CNT_MODE_3: u8 = 6;
const RZ_MTU3_TMDR1_PH_CNT_MODE_4: u8 = 7;
const RZ_MTU3_TMDR1_PH_CNT_MODE_5: u8 = 9;
const RZ_MTU3_TMDR1_PH_CNT_MODE_MASK: u8 = 0xf;
const RZ_MTU3_TMDR3_LWA: usize = 0;
const RZ_MTU3_TMDR3_PHCKSEL: usize = 1;
const RZ_MTU3_16_BIT_MTU1_CH: usize = 0;
const RZ_MTU3_16_BIT_MTU2_CH: usize = 1;
const RZ_MTU3_32_BIT_CH: usize = 2;
const RZ_MTU3_TIOR_NO_OUTPUT: u8 = 0;
const RZ_MTU3_TIOR_IC_BOTH: u8 = 10;
const SIGNAL_A_ID: usize = 0;
const SIGNAL_B_ID: usize = 1;
const SIGNAL_C_ID: usize = 2;
const SIGNAL_D_ID: usize = 3;
const RZ_MTU3_MAX_HW_CNTR_CHANNELS: usize = 2;
const RZ_MTU3_MAX_LOGICAL_CNTR_CHANNELS: usize = 3;

#[repr(C)] pub struct clk;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct counter_device { pub parent: *mut device }
#[repr(C)] pub struct counter_count { pub id: usize }
#[repr(C)] pub struct counter_signal { pub id: usize, pub name: *const c_char }
#[repr(C)] pub struct counter_synapse { pub actions_list: *const i32, pub num_actions: usize, pub signal: *mut counter_signal }
#[repr(C)] pub struct counter_comp;
#[repr(C)] pub struct counter_ops;
#[repr(C)] pub struct rz_mtu3_channel { pub dev: *mut device, pub is_busy: bool }
#[repr(C)] pub struct rz_mtu3 { pub clk: *mut clk, pub channels: [rz_mtu3_channel; 2] }
#[repr(C)] pub struct rz_mtu3_cnt {
    pub clk: *mut clk,
    pub lock: mutex,
    pub ch: *mut rz_mtu3_channel,
    pub count_is_enabled: [bool; RZ_MTU3_MAX_LOGICAL_CNTR_CHANNELS],
    pub mtu_16bit_max: [u16; RZ_MTU3_MAX_HW_CNTR_CHANNELS],
    pub mtu_32bit_max: u32,
}

type CounterFunction = i32;
type CounterDirection = i32;
type CounterAction = i32;
const COUNTER_FUNCTION_QUADRATURE_X4: CounterFunction = 0;
const COUNTER_FUNCTION_PULSE_DIRECTION: CounterFunction = 1;
const COUNTER_FUNCTION_QUADRATURE_X2_B: CounterFunction = 2;
const COUNTER_COUNT_DIRECTION_FORWARD: CounterDirection = 0;
const COUNTER_COUNT_DIRECTION_BACKWARD: CounterDirection = 1;
const COUNTER_SYNAPSE_ACTION_BOTH_EDGES: CounterAction = 0;
const COUNTER_SYNAPSE_ACTION_RISING_EDGE: CounterAction = 1;
const COUNTER_SYNAPSE_ACTION_NONE: CounterAction = 2;

extern "C" {
    fn counter_priv(counter: *mut counter_device) -> *mut rz_mtu3_cnt;
    fn mutex_lock(lock: *mut mutex); fn mutex_unlock(lock: *mut mutex);
    fn pm_runtime_get_sync(dev: *mut device) -> i32; fn pm_runtime_put(dev: *mut device);
    fn rz_mtu3_shared_reg_read(ch: *mut rz_mtu3_channel, reg: u32) -> usize;
    fn rz_mtu3_shared_reg_update_bit(ch: *mut rz_mtu3_channel, reg: u32, bit: usize, val: u32);
    fn rz_mtu3_8bit_ch_read(ch: *mut rz_mtu3_channel, reg: u32) -> u8;
    fn rz_mtu3_8bit_ch_write(ch: *mut rz_mtu3_channel, reg: u32, val: u8);
    fn rz_mtu3_16bit_ch_read(ch: *mut rz_mtu3_channel, reg: u32) -> u16;
    fn rz_mtu3_16bit_ch_write(ch: *mut rz_mtu3_channel, reg: u32, val: u64);
    fn rz_mtu3_32bit_ch_read(ch: *mut rz_mtu3_channel, reg: u32) -> u32;
    fn rz_mtu3_32bit_ch_write(ch: *mut rz_mtu3_channel, reg: u32, val: u64);
    fn rz_mtu3_request_channel(ch: *mut rz_mtu3_channel) -> bool;
    fn rz_mtu3_release_channel(ch: *mut rz_mtu3_channel);
    fn rz_mtu3_enable(ch: *mut rz_mtu3_channel); fn rz_mtu3_disable(ch: *mut rz_mtu3_channel);
    fn rz_mtu3_is_enabled(ch: *mut rz_mtu3_channel) -> bool;
}

#[inline] fn rz_mtu3_get_hw_ch(id: usize) -> usize { if id == RZ_MTU3_32_BIT_CH { 0 } else { id } }
#[inline] unsafe fn rz_mtu3_get_ch(counter: *mut counter_device, id: usize) -> *mut rz_mtu3_channel {
    (*counter_priv(counter)).ch.add(rz_mtu3_get_hw_ch(id))
}
unsafe fn rz_mtu3_is_counter_invalid(counter: *mut counter_device, id: usize) -> bool {
    let priv_ = counter_priv(counter); pm_runtime_get_sync((*counter).parent);
    let tmdr = rz_mtu3_shared_reg_read((*priv_).ch, 0); pm_runtime_put((*counter).parent);
    if id == RZ_MTU3_32_BIT_CH && (tmdr & (1 << RZ_MTU3_TMDR3_LWA)) != 0 { return false; }
    if id != RZ_MTU3_32_BIT_CH && (tmdr & (1 << RZ_MTU3_TMDR3_LWA)) == 0 { return false; }
    true
}
unsafe fn rz_mtu3_lock_if_counter_is_valid(counter: *mut counter_device, ch: *mut rz_mtu3_channel, priv_: *mut rz_mtu3_cnt, id: usize) -> i32 {
    mutex_lock(&mut (*priv_).lock); if (*ch).is_busy && !(*priv_).count_is_enabled[id] { mutex_unlock(&mut (*priv_).lock); return -22; }
    if rz_mtu3_is_counter_invalid(counter, id) { mutex_unlock(&mut (*priv_).lock); return -16; } 0
}
unsafe fn rz_mtu3_lock_if_count_is_enabled(ch: *mut rz_mtu3_channel, priv_: *mut rz_mtu3_cnt, id: usize) -> i32 {
    mutex_lock(&mut (*priv_).lock); if (*ch).is_busy && !(*priv_).count_is_enabled[id] { mutex_unlock(&mut (*priv_).lock); return -22; } 0
}

unsafe fn rz_mtu3_count_read(counter: *mut counter_device, count: *mut counter_count, val: *mut u64) -> i32 {
    let ch = rz_mtu3_get_ch(counter, (*count).id); let p = counter_priv(counter); let r = rz_mtu3_lock_if_counter_is_valid(counter,ch,p,(*count).id); if r != 0 { return r; }
    pm_runtime_get_sync((*counter).parent); *val = if (*count).id == 2 { rz_mtu3_32bit_ch_read(ch,0) as u64 } else { rz_mtu3_16bit_ch_read(ch,0) as u64 }; pm_runtime_put((*counter).parent); mutex_unlock(&mut (*p).lock); 0
}
unsafe fn rz_mtu3_count_write(counter: *mut counter_device, count: *mut counter_count, val: u64) -> i32 {
    let ch=rz_mtu3_get_ch(counter,(*count).id); let p=counter_priv(counter); let r=rz_mtu3_lock_if_counter_is_valid(counter,ch,p,(*count).id); if r!=0{return r;} pm_runtime_get_sync((*counter).parent); if (*count).id==2{rz_mtu3_32bit_ch_write(ch,0,val)}else{rz_mtu3_16bit_ch_write(ch,0,val)} pm_runtime_put((*counter).parent); mutex_unlock(&mut (*p).lock); 0
}
unsafe fn rz_mtu3_count_function_read_helper(ch:*mut rz_mtu3_channel,counter:*mut counter_device,function:*mut CounterFunction)->i32{let mode=rz_mtu3_8bit_ch_read(ch,0)&RZ_MTU3_TMDR1_PH_CNT_MODE_MASK;*function=match mode{4=>COUNTER_FUNCTION_QUADRATURE_X4,5=>COUNTER_FUNCTION_PULSE_DIRECTION,7=>COUNTER_FUNCTION_QUADRATURE_X2_B,_=>return -22};0}
unsafe fn rz_mtu3_count_function_read(c:*mut counter_device,n:*mut counter_count,f:*mut CounterFunction)->i32{let p=counter_priv(c);let r=rz_mtu3_lock_if_count_is_enabled(rz_mtu3_get_ch(c,(*n).id),p,(*n).id);if r!=0{return r}let r=rz_mtu3_count_function_read_helper(rz_mtu3_get_ch(c,(*n).id),c,f);mutex_unlock(&mut(*p).lock);r}
unsafe fn rz_mtu3_count_function_write(c:*mut counter_device,n:*mut counter_count,f:CounterFunction)->i32{let p=counter_priv(c);let ch=rz_mtu3_get_ch(c,(*n).id);let r=rz_mtu3_lock_if_count_is_enabled(ch,p,(*n).id);if r!=0{return r}let mode=match f{0=>4,1=>5,2=>7,_=>{mutex_unlock(&mut(*p).lock);return -22}};rz_mtu3_8bit_ch_write(ch,0,mode);mutex_unlock(&mut(*p).lock);0}
unsafe fn rz_mtu3_count_direction_read(c:*mut counter_device,n:*mut counter_count,d:*mut CounterDirection)->i32{let p=counter_priv(c);let r=rz_mtu3_lock_if_count_is_enabled(rz_mtu3_get_ch(c,(*n).id),p,(*n).id);if r!=0{return r}let tsr=rz_mtu3_8bit_ch_read(rz_mtu3_get_ch(c,(*n).id),0);*d=if(tsr as u32&RZ_MTU3_TSR_TCFD)!=0{0}else{1};mutex_unlock(&mut(*p).lock);0}

// The remaining counter component and platform-driver descriptors mirror the C
// metadata and callback registration; their concrete kernel representations are
// supplied by the consuming Rust kernel bindings.
#[allow(dead_code)] static RZ_MTU3_COUNT_FUNCTIONS:[CounterFunction;3]=[COUNTER_FUNCTION_QUADRATURE_X4,COUNTER_FUNCTION_PULSE_DIRECTION,COUNTER_FUNCTION_QUADRATURE_X2_B];
#[allow(dead_code)] static RZ_MTU3_SIGNALS:[(&str,usize);4]=[("MTU1 MTCLKA",0),("MTU1 MTCLKB",1),("MTU2 MTCLKC",2),("MTU2 MTCLKD",3)];

unsafe fn rz_mtu3_count_ceiling_read(c:*mut counter_device,n:*mut counter_count,v:*mut u64)->i32{let p=counter_priv(c);let id=(*n).id;let r=rz_mtu3_lock_if_counter_is_valid(c,rz_mtu3_get_ch(c,id),p,id);if r!=0{return r}*v=if id==2{(*p).mtu_32bit_max as u64}else if id<2{(*p).mtu_16bit_max[rz_mtu3_get_hw_ch(id)] as u64}else{mutex_unlock(&mut(*p).lock);return -22};mutex_unlock(&mut(*p).lock);0}
unsafe fn rz_mtu3_count_ceiling_write(c:*mut counter_device,n:*mut counter_count,v:u64)->i32{let p=counter_priv(c);let id=(*n).id;let ch=rz_mtu3_get_ch(c,id);let r=rz_mtu3_lock_if_counter_is_valid(c,ch,p,id);if r!=0{return r}if id==2{if v>u32::MAX as u64{mutex_unlock(&mut(*p).lock);return -34}(*p).mtu_32bit_max=v as u32;rz_mtu3_32bit_ch_write(ch,0,v)}else if id<2{if v>u16::MAX as u64{mutex_unlock(&mut(*p).lock);return -34}(*p).mtu_16bit_max[rz_mtu3_get_hw_ch(id)]=v as u16;rz_mtu3_16bit_ch_write(ch,0,v)}else{mutex_unlock(&mut(*p).lock);return -22}rz_mtu3_8bit_ch_write(ch,0,0);mutex_unlock(&mut(*p).lock);0}
unsafe fn rz_mtu3_32bit_cnt_setting(c:*mut counter_device){let a=rz_mtu3_get_ch(c,0);let b=rz_mtu3_get_ch(c,1);rz_mtu3_8bit_ch_write(a,0,4);rz_mtu3_8bit_ch_write(a,0,0);rz_mtu3_8bit_ch_write(a,0,10);rz_mtu3_enable(a);rz_mtu3_enable(b)}
unsafe fn rz_mtu3_16bit_cnt_setting(c:*mut counter_device,id:usize){let ch=rz_mtu3_get_ch(c,id);rz_mtu3_8bit_ch_write(ch,0,4);rz_mtu3_8bit_ch_write(ch,0,0);rz_mtu3_8bit_ch_write(ch,0,0);rz_mtu3_enable(ch)}
unsafe fn rz_mtu3_initialize_counter(c:*mut counter_device,id:usize)->i32{if id<2{let ch=rz_mtu3_get_ch(c,id);if !rz_mtu3_request_channel(ch){return -16}rz_mtu3_16bit_cnt_setting(c,id);0}else if id==2{let a=rz_mtu3_get_ch(c,0);let b=rz_mtu3_get_ch(c,1);if !rz_mtu3_request_channel(a){return -16}if !rz_mtu3_request_channel(b){rz_mtu3_release_channel(a);return -16}rz_mtu3_32bit_cnt_setting(c);0}else{-22}}
unsafe fn rz_mtu3_terminate_counter(c:*mut counter_device,id:usize){if id==2{let a=rz_mtu3_get_ch(c,0);let b=rz_mtu3_get_ch(c,1);rz_mtu3_release_channel(b);rz_mtu3_release_channel(a);rz_mtu3_disable(b);rz_mtu3_disable(a)}else{let ch=rz_mtu3_get_ch(c,id);rz_mtu3_release_channel(ch);rz_mtu3_disable(ch)}}
unsafe fn rz_mtu3_count_enable_write(c:*mut counter_device,n:*mut counter_count,en:u8)->i32{let p=counter_priv(c);mutex_lock(&mut(*p).lock);let id=(*n).id;if (*p).count_is_enabled[id]==(en!=0){mutex_unlock(&mut(*p).lock);return 0}let r=if en!=0{rz_mtu3_initialize_counter(c,id)}else{rz_mtu3_terminate_counter(c,id);0};if r==0{(*p).count_is_enabled[id]=en!=0}mutex_unlock(&mut(*p).lock);r}
unsafe fn rz_mtu3_cascade_counts_enable_set(c:*mut counter_device,v:u8)->i32{let p=counter_priv(c);mutex_lock(&mut(*p).lock);rz_mtu3_shared_reg_update_bit((*p).ch,0,RZ_MTU3_TMDR3_LWA,v as u32);mutex_unlock(&mut(*p).lock);0}
unsafe fn rz_mtu3_ext_input_phase_clock_select_set(c:*mut counter_device,v:u32)->i32{let p=counter_priv(c);mutex_lock(&mut(*p).lock);rz_mtu3_shared_reg_update_bit((*p).ch,0,RZ_MTU3_TMDR3_PHCKSEL,v);mutex_unlock(&mut(*p).lock);0}

// static struct counter_ops rz_mtu3_cnt_ops = { count_read, count_write,
// function_read, function_write, action_read };
// static struct platform_driver rz_mtu3_cnt_driver = { .probe = rz_mtu3_cnt_probe,
// .driver = { .name = "rz-mtu3-counter", .pm = pm_ptr(&rz_mtu3_cnt_pm_ops) } };
// module_platform_driver(rz_mtu3_cnt_driver);
// MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
// MODULE_ALIAS("platform:rz-mtu3-counter");
// MODULE_DESCRIPTION("Renesas RZ/G2L MTU3a counter driver");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("COUNTER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
