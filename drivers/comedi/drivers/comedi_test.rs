// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of comedi/drivers/comedi_test.c. */

use core::ptr;

const N_CHANS: usize = 8;
const DEV_NAME: &[u8] = b"comedi_testd\0";
const CLASS_NAME: &[u8] = b"comedi_test\0";

static mut CONFIG_MODE: bool = false;
static mut SET_AMPLITUDE: u32 = 0;
static mut SET_PERIOD: u32 = 0;
static mut CTDEV: *mut device = ptr::null_mut();

#[repr(C)]
struct waveform_private {
    ai_timer: timer_list,
    ai_convert_time: u64,
    wf_amplitude: u32,
    wf_period: u32,
    wf_current: u32,
    ai_scan_period: u32,
    ai_convert_period: u32,
    ao_timer: timer_list,
    dev: *mut comedi_device,
    ao_last_scan_time: u64,
    ao_scan_period: u32,
    ai_timer_enable: bool,
    ao_timer_enable: bool,
    ao_loopbacks: [u16; N_CHANS],
}

// Kernel and Comedi declarations are supplied by the surrounding translation unit.
extern "C" {
    static mut waveform_ai_ranges: comedi_lrange;
    static mut range_digital: comedi_lrange;
    fn ktime_get() -> u64;
    fn ktime_to_us(v: u64) -> u64;
    fn comedi_nsamples_left(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_nscans_left(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_buf_write_samples(s: *mut comedi_subdevice, p: *const u16, n: u32) -> u32;
    fn comedi_buf_read_samples(s: *mut comedi_subdevice, p: *mut u16, n: u32) -> u32;
    fn comedi_buf_read_alloc(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_buf_read_free(s: *mut comedi_subdevice, n: u32);
    fn comedi_samples_to_bytes(s: *mut comedi_subdevice, n: u32) -> u32;
    fn comedi_inc_scan_progress(s: *mut comedi_subdevice, n: u32);
    fn comedi_handle_events(d: *mut comedi_device, s: *mut comedi_subdevice);
    fn comedi_alloc_devpriv(d: *mut comedi_device, n: usize) -> *mut waveform_private;
    fn comedi_alloc_subdevices(d: *mut comedi_device, n: u32) -> i32;
    fn comedi_check_trigger_src(v: *mut u32, mask: u32) -> i32;
    fn comedi_check_trigger_is_unique(v: u32) -> i32;
    fn comedi_check_trigger_arg_is(v: *mut u32, x: u32) -> i32;
    fn comedi_check_trigger_arg_min(v: *mut u32, x: u32) -> i32;
    fn comedi_dio_update_state(s: *mut comedi_subdevice, d: *mut u32);
    fn comedi_dio_insn_config(d: *mut comedi_device, s: *mut comedi_subdevice, i: *mut comedi_insn, data: *mut u32, x: u32) -> i32;
}

unsafe fn fake_sawtooth(dev: *mut comedi_device, range_index: u32, current_time: u32) -> u16 {
    let p = (*dev).private as *mut waveform_private;
    let s = (*dev).read_subdev;
    let offset = (*s).maxdata / 2;
    let kr = &(*(*s).range_table).range[range_index as usize];
    let amplitude = ((*s).maxdata as u64 * (*p).wf_amplitude as u64) / (kr.max - kr.min) as u64;
    let mut value = (current_time as u64 * amplitude * 2) / (*p).wf_period as u64 + offset as u64;
    if value < amplitude { value = 0; } else { value -= amplitude; if value > (*s).maxdata as u64 { value = (*s).maxdata as u64; } }
    value as u16
}

unsafe fn fake_squarewave(dev: *mut comedi_device, range_index: u32, current_time: u32) -> u16 {
    let p = (*dev).private as *mut waveform_private;
    let s = (*dev).read_subdev;
    let offset = (*s).maxdata / 2;
    let kr = &(*(*s).range_table).range[range_index as usize];
    let mut value = ((*s).maxdata as u64 * (*p).wf_amplitude as u64) / (kr.max - kr.min) as u64;
    if current_time < (*p).wf_period / 2 { value = if offset as u64 < value { 0 } else { offset as u64 - value }; }
    else { value += offset as u64; if value > (*s).maxdata as u64 { value = (*s).maxdata as u64; } }
    value as u16
}

unsafe fn fake_flatline(dev: *mut comedi_device, _: u32, _: u32) -> u16 { ((*(*dev).read_subdev).maxdata) / 2 as u32 as u16 }

unsafe fn fake_waveform(dev: *mut comedi_device, channel: u32, range: u32, time: u32) -> u16 {
    match channel { 0 => fake_sawtooth(dev, range, time), 1 => fake_squarewave(dev, range, time), _ => fake_flatline(dev, range, time) }
}

unsafe extern "C" fn waveform_ai_insn_read(_: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*(*s).dev).private as *mut waveform_private;
    let chan = CR_CHAN((*insn).chanspec);
    for i in 0..(*insn).n as isize { *data.offset(i) = (*p).ao_loopbacks[chan as usize] as u32; }
    (*insn).n as i32
}

unsafe extern "C" fn waveform_ao_insn_write(_: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let p = (*(*s).dev).private as *mut waveform_private;
    let chan = CR_CHAN((*insn).chanspec) as usize;
    for i in 0..(*insn).n as isize { (*p).ao_loopbacks[chan] = *data.offset(i) as u16; }
    (*insn).n as i32
}

unsafe extern "C" fn waveform_ai_insn_config(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, data: *mut u32) -> i32 {
    if *data == INSN_CONFIG_GET_CMD_TIMING_CONSTRAINTS { if *data.add(1) == TRIG_FOLLOW { *data.add(1)=0; *data.add(2)=NSEC_PER_USEC; } else { *data.add(1)=NSEC_PER_USEC; *data.add(2)=if *data.add(2)&TRIG_TIMER != 0 {NSEC_PER_USEC} else {0}; } 0 } else { -EINVAL }
}

unsafe extern "C" fn waveform_ao_insn_config(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, data: *mut u32) -> i32 {
    if *data == INSN_CONFIG_GET_CMD_TIMING_CONSTRAINTS { *data.add(1)=NSEC_PER_USEC; *data.add(2)=0; 0 } else { -EINVAL }
}

unsafe extern "C" fn waveform_dio_insn_bits(_: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    comedi_dio_update_state(s, data);
    let driven_low = (*s).io_bits & !(*s).state;
    let wires = 0xffff & !driven_low & !(driven_low >> 16);
    *data.add(1) = wires | (wires << 16);
    (*insn).n as i32
}

unsafe extern "C" fn waveform_dio_insn_config(d: *mut comedi_device, s: *mut comedi_subdevice, i: *mut comedi_insn, data: *mut u32) -> i32 { comedi_dio_insn_config(d,s,i,data,0) }

unsafe extern "C" fn waveform_ai_timer(t: *mut timer_list) {
    let p = container_of_waveform(t, true); let d=(*p).dev; let s=(*d).read_subdev;
    let now=ktime_to_us(ktime_get()); let a=(*s).async_; let c=&mut (*a).cmd;
    while comedi_nsamples_left(s, u32::MAX)!=0 && (*p).ai_convert_time < now {
        let cs=c.chanlist[(*a).cur_chan as usize]; let sample=fake_waveform(d,CR_CHAN(cs),CR_RANGE(cs),(*p).wf_current);
        if comedi_buf_write_samples(s,&sample,1)==0 { break; }
        let mut inc=(*p).ai_convert_period; if (*a).scan_progress==0 { inc += (*p).ai_scan_period-(*p).ai_convert_period*c.scan_end_arg; }
        (*p).wf_current=(*p).wf_current.wrapping_add(inc)%(*p).wf_period; (*p).ai_convert_time+=inc as u64;
    }
    comedi_handle_events(d,s);
}
unsafe extern "C" fn waveform_ao_timer(t: *mut timer_list) { let p=container_of_waveform(t,false); comedi_handle_events((*p).dev,(*p).dev.write_subdev); }
unsafe extern "C" fn waveform_ai_cmdtest(_: *mut comedi_device, _: *mut comedi_subdevice, c: *mut comedi_cmd) -> i32 {
    let mut e=0; e|=comedi_check_trigger_src(&mut (*c).start_src,TRIG_NOW); e|=comedi_check_trigger_src(&mut (*c).scan_begin_src,TRIG_FOLLOW|TRIG_TIMER); e|=comedi_check_trigger_src(&mut (*c).convert_src,TRIG_NOW|TRIG_TIMER); e|=comedi_check_trigger_src(&mut (*c).scan_end_src,TRIG_COUNT); e|=comedi_check_trigger_src(&mut (*c).stop_src,TRIG_COUNT|TRIG_NONE); if e!=0 {1} else {0}
}
unsafe extern "C" fn waveform_ao_cmdtest(_: *mut comedi_device, _: *mut comedi_subdevice, c: *mut comedi_cmd) -> i32 { let mut e=0; e|=comedi_check_trigger_src(&mut (*c).start_src,TRIG_INT); e|=comedi_check_trigger_src(&mut (*c).scan_begin_src,TRIG_TIMER); e|=comedi_check_trigger_src(&mut (*c).convert_src,TRIG_NOW); e|=comedi_check_trigger_src(&mut (*c).scan_end_src,TRIG_COUNT); if e!=0 {1} else {0} }
unsafe extern "C" fn waveform_ai_cmd(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe extern "C" fn waveform_ao_cmd(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe extern "C" fn waveform_ai_cancel(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe extern "C" fn waveform_ao_cancel(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { 0 }
unsafe fn container_of_waveform(t:*mut timer_list, ai:bool)->*mut waveform_private { let p=t as *mut u8; p.offset(-(if ai {0} else {core::mem::offset_of!(waveform_private,ao_timer)}) as isize) as *mut waveform_private }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
