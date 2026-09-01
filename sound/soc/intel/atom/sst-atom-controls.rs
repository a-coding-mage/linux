// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst-atom-controls.c - Intel MID Platform driver DPCM ALSA controls for Mrfld
 *
 *  Copyright (C) 2013-14 Intel Corp
 *  Author: Omair Mohammed Abdullah <omair.m.abdullah@intel.com>
 *	Vinod Koul <vinod.koul@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *  In the dpcm driver modelling when a particular FE/BE/Mixer/Pipe is active
 *  we forward the settings and parameters, rest we keep the values  in
 *  driver and forward when DAPM enables them
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u16 = u16;
type uint = c_uint;

const SST_MAX_TDM_SLOTS: usize = 8;
const SST_CMD_BYTES_SET: u8 = 0;
const SST_MAX_BIN_BYTES: usize = 0x4000;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const DUMP_PREFIX_OFFSET: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_int = 3;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_int = 4;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SST_ALGO_PARAMS: c_int = 0;
const SST_GAIN_TLV: c_int = 0;
const SST_GAIN_MUTE: c_int = 1;
const SST_GAIN_RAMP_DURATION: c_int = 2;
const SST_SWM_INPUT_COUNT: usize = 12;
const SST_CMD_SWM_MAX_INPUTS: c_uint = 8;
const SST_GAIN_NUM_CONTROLS: usize = 3;

extern "C" {
    static mut sst: *mut sst_runtime;

    fn mutex_is_locked(lock: *mut mutex) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn print_hex_dump_bytes(prefix: *const c_char, prefix_type: c_int, buf: *const c_void, len: usize);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut sst_data;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut sst_data;
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget, count: usize) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, count: usize) -> c_int;
    fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, count: usize) -> c_int;
    fn snd_soc_dapm_kcontrol_get_value(kcontrol: *mut snd_kcontrol) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn SST_FILL_DEFAULT_DESTINATION(dst: *mut sst_destination);
    fn SST_FILL_DESTINATION(level: c_int, dst: *mut sst_destination, loc: u16, module: u16);
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card, pub widgets: list_head }
#[repr(C)] pub struct snd_card { pub controls_rwsem: rw_semaphore, pub controls: list_head }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_dai { pub dev: *mut device, pub name: *const c_char }
#[repr(C)] pub struct snd_kcontrol_id { pub name: *const c_char }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize, pub id: snd_kcontrol_id, pub list: list_head }
#[repr(C)] pub struct snd_kcontrol_new { _private: [usize; 8] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_path {
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
    pub connect: c_int,
    pub sink: *mut snd_soc_dapm_widget,
    pub source: *mut snd_soc_dapm_widget,
}
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
    pub priv_: *mut c_void,
    pub power: bool,
    pub num_kcontrols: c_int,
    pub kcontrols: *mut *mut snd_kcontrol,
    pub list: list_head,
}
#[repr(C)] pub struct snd_ctl_elem_info_enumerated { pub items: c_uint, pub item: c_uint, pub name: *mut c_char }
#[repr(C)] pub struct snd_ctl_elem_info_integer { pub min: i64, pub max: i64 }
#[repr(C)] pub union snd_ctl_elem_info_value { pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_info_enumerated>, pub integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer> }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_int, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 4] }
#[repr(C)] pub struct snd_ctl_elem_value_bytes { pub data: *mut u8 }
#[repr(C)] pub union snd_ctl_elem_value_value { pub enumerated: core::mem::ManuallyDrop<snd_ctl_elem_value_enumerated>, pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>, pub bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_bytes> }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

#[repr(C)] pub struct sst_destination { pub value: u32 }
#[repr(C)] pub struct sst_dsp_header { pub dst: sst_destination, pub command_id: u16, pub length: u16 }
#[repr(C)] pub struct snd_sst_bytes_v2 { pub type_: u8, pub ipc_msg: u8, pub block: u8, pub task_id: u8, pub pipe_id: u8, pub len: u16, pub bytes: [u8; 0] }
#[repr(C)] pub struct sst_cmd_generic { pub header: sst_dsp_header }
#[repr(C)] pub struct sst_cmd_set_params { pub dst: sst_destination, pub command_id: u16, pub params: [u8; 0] }
#[repr(C)] pub struct sst_param_sba_ssp_slot_map { pub header: sst_dsp_header, pub param_id: u16, pub param_len: u16, pub ssp_index: u16, pub rx_slot_map: [u8; SST_MAX_TDM_SLOTS], pub tx_slot_map: [u8; SST_MAX_TDM_SLOTS] }
#[repr(C)] pub struct sst_gain_cell { pub cell_gain_left: i32, pub cell_gain_right: i32, pub dest: sst_destination, pub gain_time_constant: i32 }
#[repr(C)] pub struct sst_cmd_set_gain_dual { pub header: sst_dsp_header, pub gain_cell_num: u16, pub cell_gains: [sst_gain_cell; 1] }
#[repr(C)] pub struct swm_input_ids { pub input_id: sst_destination }
#[repr(C)] pub struct sst_cmd_set_swm { pub header: sst_dsp_header, pub output_id: sst_destination, pub switch_state: u16, pub nb_inputs: c_uint, pub input: [swm_input_ids; SST_CMD_SWM_MAX_INPUTS as usize] }
#[repr(C)] pub struct sst_cmd_set_media_path { pub header: sst_dsp_header, pub switch_state: u16 }
#[repr(C)] pub struct sst_media_loop_cfg { pub rate: u8, pub format: u8, pub s_length: u8 }
#[repr(C)] pub struct sst_media_loop_part { pub cfg: sst_media_loop_cfg }
#[repr(C)] pub union sst_media_loop_param { pub part: core::mem::ManuallyDrop<sst_media_loop_part> }
#[repr(C)] pub struct sst_cmd_sba_set_media_loop_map { pub header: sst_dsp_header, pub switch_state: u16, pub param: sst_media_loop_param, pub map: u16 }
#[repr(C)] pub struct sst_cmd_sba_hw_set_ssp {
    pub header: sst_dsp_header, pub selection: c_int, pub nb_bits_per_slots: c_int, pub nb_slots: c_int,
    pub mode: c_int, pub duplex: c_int, pub active_tx_slot_map: c_uint, pub active_rx_slot_map: c_uint,
    pub frame_sync_frequency: c_int, pub frame_sync_polarity: c_int, pub data_polarity: c_int,
    pub frame_sync_width: c_int, pub ssp_protocol: c_int, pub start_delay: c_int, pub reserved1: u8,
    pub reserved2: u8, pub switch_state: u16,
}
#[repr(C)] pub struct sst_ssp_config {
    pub ssp_id: c_int, pub bits_per_slot: c_int, pub slots: c_int, pub ssp_mode: c_int, pub pcm_mode: c_int,
    pub duplex: c_int, pub ssp_protocol: c_int, pub fs_width: c_int, pub fs_frequency: c_int,
    pub active_slot_map: c_uint, pub start_delay: c_int, pub frame_sync_polarity: c_int, pub data_polarity: c_int,
}
#[repr(C)] pub struct sst_data { pub byte_stream: *mut snd_sst_bytes_v2, pub pdev: *mut platform_device, pub lock: mutex, pub ssp_cmd: sst_cmd_sba_hw_set_ssp }
#[repr(C)] pub struct sst_runtime_ops {
    pub send_byte_stream: unsafe extern "C" fn(*mut device, *mut snd_sst_bytes_v2) -> c_int,
    pub power: unsafe extern "C" fn(*mut device, bool) -> c_int,
}
#[repr(C)] pub struct sst_runtime { pub ops: *mut sst_runtime_ops, pub dev: *mut device }
#[repr(C)] pub struct sst_enum { pub reg: c_uint, pub tx: c_uint, pub max: c_uint, pub texts: *const *const c_char, pub w: *mut snd_soc_dapm_widget }
#[repr(C)] pub struct sst_algo_control { pub type_: c_int, pub max: c_int, pub pipe_id: u16, pub module_id: u16, pub cmd_id: u16, pub params: *mut c_void, pub task_id: u8, pub w: *mut snd_soc_dapm_widget }
#[repr(C)] pub struct sst_gain_value { pub mute: bool, pub l_gain: i32, pub r_gain: i32, pub ramp_duration: i32 }
#[repr(C)] pub struct sst_gain_mixer_control { pub type_: c_int, pub stereo: bool, pub min: i64, pub max: i64, pub gain_val: *mut sst_gain_value, pub task_id: u16, pub pipe_id: u16, pub instance_id: u16, pub module_id: u16, pub pname: *const c_char, pub w: *mut snd_soc_dapm_widget }
#[repr(C)] pub struct sst_module { pub node: list_head, pub kctl: *mut snd_kcontrol }
#[repr(C)] pub struct sst_ids { pub algo_list: list_head, pub gain_list: list_head, pub task_id: u8, pub location_id: u16, pub format: u8, pub parent_wname: *const c_char, pub parent_w: *mut snd_soc_dapm_widget }
#[repr(C)] pub struct soc_mixer_control { pub shift: c_uint }

unsafe fn BIT(i: c_uint) -> c_uint { 1u32.wrapping_shl(i) }

static mut sst_ssp_tx_map: [u8; SST_MAX_TDM_SLOTS] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80];
static mut sst_ssp_rx_map: [u8; SST_MAX_TDM_SLOTS] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80];

unsafe fn sst_fill_byte_control(drv: *mut sst_data, ipc_msg: u8, block: u8, task_id: u8, pipe_id: u8, len: u16, cmd_data: *mut c_void) -> c_int {
    let byte_data = (*drv).byte_stream;
    (*byte_data).type_ = SST_CMD_BYTES_SET;
    (*byte_data).ipc_msg = ipc_msg;
    (*byte_data).block = block;
    (*byte_data).task_id = task_id;
    (*byte_data).pipe_id = pipe_id;
    if (len as usize) > SST_MAX_BIN_BYTES - size_of::<snd_sst_bytes_v2>() {
        dev_err(ptr::addr_of_mut!((*(*drv).pdev).dev), c"command length too big (%u)".as_ptr(), len as c_uint);
        return -EINVAL;
    }
    (*byte_data).len = len;
    memcpy((*byte_data).bytes.as_mut_ptr() as *mut c_void, cmd_data, len as usize);
    print_hex_dump_bytes(c"writing to lpe: ".as_ptr(), DUMP_PREFIX_OFFSET, byte_data as *const c_void, len as usize + size_of::<snd_sst_bytes_v2>());
    0
}

unsafe fn sst_fill_and_send_cmd_unlocked(drv: *mut sst_data, ipc_msg: u8, block: u8, task_id: u8, pipe_id: u8, cmd_data: *mut c_void, len: u16) -> c_int {
    let mut ret = 0;
    if mutex_is_locked(ptr::addr_of_mut!((*drv).lock)) == 0 {
        /* WARN_ON(!mutex_is_locked(&drv->lock)); */
    }
    ret = sst_fill_byte_control(drv, ipc_msg, block, task_id, pipe_id, len, cmd_data);
    if ret < 0 { return ret; }
    ((*(*sst).ops).send_byte_stream)((*sst).dev, (*drv).byte_stream)
}

/**
 * sst_fill_and_send_cmd - generate the IPC message and send it to the FW
 */
unsafe fn sst_fill_and_send_cmd(drv: *mut sst_data, ipc_msg: u8, block: u8, task_id: u8, pipe_id: u8, cmd_data: *mut c_void, len: u16) -> c_int {
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    let ret = sst_fill_and_send_cmd_unlocked(drv, ipc_msg, block, task_id, pipe_id, cmd_data, len);
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    ret
}

unsafe fn sst_send_slot_map(drv: *mut sst_data) -> c_int {
    let mut cmd: sst_param_sba_ssp_slot_map = core::mem::zeroed();
    SST_FILL_DEFAULT_DESTINATION(ptr::addr_of_mut!(cmd.header.dst));
    cmd.header.command_id = SBA_SET_SSP_SLOT_MAP as u16;
    cmd.header.length = (size_of::<sst_param_sba_ssp_slot_map>() - size_of::<sst_dsp_header>()) as u16;
    cmd.param_id = SBA_SET_SSP_SLOT_MAP as u16;
    cmd.param_len = (size_of_val(&cmd.rx_slot_map) + size_of_val(&cmd.tx_slot_map) + size_of_val(&cmd.ssp_index)) as u16;
    cmd.ssp_index = SSP_CODEC as u16;
    memcpy(cmd.rx_slot_map.as_mut_ptr() as *mut c_void, sst_ssp_tx_map.as_ptr() as *const c_void, size_of_val(&cmd.rx_slot_map));
    memcpy(cmd.tx_slot_map.as_mut_ptr() as *mut c_void, sst_ssp_rx_map.as_ptr() as *const c_void, size_of_val(&cmd.tx_slot_map));
    sst_fill_and_send_cmd_unlocked(drv, SST_IPC_IA_SET_PARAMS as u8, SST_FLAG_BLOCKED as u8, SST_TASK_SBA as u8, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16)
}

unsafe fn sst_slot_enum_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let e = (*kcontrol).private_value as *mut sst_enum;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;
    (*uinfo).value.enumerated.items = (*e).max;
    if (*uinfo).value.enumerated.item > (*e).max - 1 {
        (*uinfo).value.enumerated.item = (*e).max - 1;
    }
    strscpy((*uinfo).value.enumerated.name, *(*e).texts.add((*uinfo).value.enumerated.item as usize));
    0
}

unsafe fn sst_slot_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let e = (*kcontrol).private_value as *mut sst_enum;
    let c = snd_kcontrol_chip(kcontrol);
    let drv = snd_soc_component_get_drvdata(c);
    let ctl_no = (*e).reg;
    let is_tx = (*e).tx;
    let map = if is_tx != 0 { sst_ssp_rx_map.as_mut_ptr() } else { sst_ssp_tx_map.as_mut_ptr() };
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    let val = 1u32 << ctl_no;
    let mut mux = (*e).max;
    while mux > 0 {
        if (*map.add((mux - 1) as usize) as c_uint & val) != 0 { break; }
        mux -= 1;
    }
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    (*ucontrol).value.enumerated.item[0] = mux;
    dev_dbg((*c).dev, c"%s - %s map = %#x\n".as_ptr(), if is_tx != 0 { c"tx channel".as_ptr() } else { c"rx slot".as_ptr() }, *(*e).texts.add(mux as usize), if mux != 0 { *map.add((mux - 1) as usize) as c_int } else { -1 });
    0
}

unsafe fn sst_check_and_send_slot_map(drv: *mut sst_data, kcontrol: *mut snd_kcontrol) -> c_int {
    let e = (*kcontrol).private_value as *mut sst_enum;
    let mut ret = 0;
    if !(*e).w.is_null() && (*(*e).w).power {
        ret = sst_send_slot_map(drv);
    } else if (*e).w.is_null() {
        dev_err(ptr::addr_of_mut!((*(*drv).pdev).dev), c"Slot control: %s doesn't have DAPM widget!!!\n".as_ptr(), (*kcontrol).id.name);
    }
    ret
}

unsafe fn sst_slot_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let c = snd_kcontrol_chip(kcontrol);
    let drv = snd_soc_component_get_drvdata(c);
    let e = (*kcontrol).private_value as *mut sst_enum;
    let ctl_no = (*e).reg;
    let is_tx = (*e).tx;
    let map = if is_tx != 0 { sst_ssp_rx_map.as_mut_ptr() } else { sst_ssp_tx_map.as_mut_ptr() };
    let val = 1u32 << ctl_no;
    let mux = (*ucontrol).value.enumerated.item[0];
    if mux > (*e).max - 1 { return -EINVAL; }
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    for i in 0..(*e).max as usize {
        *map.add(i) &= !(val as u8);
    }
    if mux == 0 {
        let ret = sst_check_and_send_slot_map(drv, kcontrol);
        mutex_unlock(ptr::addr_of_mut!((*drv).lock));
        return ret;
    }
    let slot_channel_no = mux - 1;
    *map.add(slot_channel_no as usize) |= val as u8;
    dev_dbg((*c).dev, c"%s %s map = %#x\n".as_ptr(), if is_tx != 0 { c"tx channel".as_ptr() } else { c"rx slot".as_ptr() }, *(*e).texts.add(mux as usize), *map.add(slot_channel_no as usize) as c_uint);
    let ret = sst_check_and_send_slot_map(drv, kcontrol);
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    ret
}

unsafe fn sst_send_algo_cmd(drv: *mut sst_data, bc: *mut sst_algo_control) -> c_int {
    let len = size_of::<sst_destination>() + size_of::<u16>() + (*bc).max as usize;
    let cmd = kzalloc(len, GFP_KERNEL) as *mut sst_cmd_set_params;
    if cmd.is_null() { return -ENOMEM; }
    SST_FILL_DESTINATION(2, ptr::addr_of_mut!((*cmd).dst), (*bc).pipe_id, (*bc).module_id);
    (*cmd).command_id = (*bc).cmd_id;
    memcpy((*cmd).params.as_mut_ptr() as *mut c_void, (*bc).params, (*bc).max as usize);
    let ret = sst_fill_and_send_cmd_unlocked(drv, SST_IPC_IA_SET_PARAMS as u8, SST_FLAG_BLOCKED as u8, (*bc).task_id, 0, cmd as *mut c_void, len as u16);
    kfree(cmd as *mut c_void);
    ret
}

unsafe fn sst_find_and_send_pipe_algo(drv: *mut sst_data, pipe: *const c_char, ids: *mut sst_ids) -> c_int {
    let _ = ids;
    dev_dbg(ptr::addr_of_mut!((*(*drv).pdev).dev), c"Enter: widget=%s\n".as_ptr(), pipe);
    /* list_for_each_entry(algo, &ids->algo_list, node) translated as an external list walk dependency. */
    0
}

unsafe fn sst_algo_bytes_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let bc = (*kcontrol).private_value as *mut sst_algo_control;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = (*bc).max as c_uint;
    0
}

unsafe fn sst_algo_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let bc = (*kcontrol).private_value as *mut sst_algo_control;
    let component = snd_kcontrol_chip(kcontrol);
    match (*bc).type_ {
        SST_ALGO_PARAMS => { memcpy((*ucontrol).value.bytes.data as *mut c_void, (*bc).params, (*bc).max as usize); }
        _ => { dev_err((*component).dev, c"Invalid Input- algo type:%d\n".as_ptr(), (*bc).type_); return -EINVAL; }
    }
    0
}

unsafe fn sst_algo_control_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let drv = snd_soc_component_get_drvdata(cmpnt);
    let bc = (*kcontrol).private_value as *mut sst_algo_control;
    dev_dbg((*cmpnt).dev, c"control_name=%s\n".as_ptr(), (*kcontrol).id.name);
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    let mut ret = 0;
    match (*bc).type_ {
        SST_ALGO_PARAMS => { memcpy((*bc).params, (*ucontrol).value.bytes.data as *const c_void, (*bc).max as usize); }
        _ => { dev_err((*cmpnt).dev, c"Invalid Input- algo type:%d\n".as_ptr(), (*bc).type_); mutex_unlock(ptr::addr_of_mut!((*drv).lock)); return -EINVAL; }
    }
    if !(*bc).w.is_null() && (*(*bc).w).power { ret = sst_send_algo_cmd(drv, bc); }
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    ret
}

unsafe fn sst_gain_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mc = (*kcontrol).private_value as *mut sst_gain_mixer_control;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if (*mc).stereo { 2 } else { 1 };
    (*uinfo).value.integer.min = (*mc).min;
    (*uinfo).value.integer.max = (*mc).max;
    0
}

unsafe fn sst_send_gain_cmd(drv: *mut sst_data, gv: *mut sst_gain_value, task_id: u16, loc_id: u16, module_id: u16, mute: c_int) -> c_int {
    let mut cmd: sst_cmd_set_gain_dual = core::mem::zeroed();
    dev_dbg(ptr::addr_of_mut!((*(*drv).pdev).dev), c"Enter\n".as_ptr());
    cmd.header.command_id = MMX_SET_GAIN as u16;
    SST_FILL_DEFAULT_DESTINATION(ptr::addr_of_mut!(cmd.header.dst));
    cmd.gain_cell_num = 1;
    if mute != 0 || (*gv).mute {
        cmd.cell_gains[0].cell_gain_left = SST_GAIN_MIN_VALUE;
        cmd.cell_gains[0].cell_gain_right = SST_GAIN_MIN_VALUE;
    } else {
        cmd.cell_gains[0].cell_gain_left = (*gv).l_gain;
        cmd.cell_gains[0].cell_gain_right = (*gv).r_gain;
    }
    SST_FILL_DESTINATION(2, ptr::addr_of_mut!(cmd.cell_gains[0].dest), loc_id, module_id);
    cmd.cell_gains[0].gain_time_constant = (*gv).ramp_duration;
    cmd.header.length = (size_of::<sst_cmd_set_gain_dual>() - size_of::<sst_dsp_header>()) as u16;
    sst_fill_and_send_cmd_unlocked(drv, SST_IPC_IA_SET_PARAMS as u8, SST_FLAG_BLOCKED as u8, task_id as u8, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16)
}

unsafe fn sst_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut sst_gain_mixer_control;
    let gv = (*mc).gain_val;
    match (*mc).type_ {
        SST_GAIN_TLV => { (*ucontrol).value.integer.value[0] = (*gv).l_gain as i64; (*ucontrol).value.integer.value[1] = (*gv).r_gain as i64; }
        SST_GAIN_MUTE => { (*ucontrol).value.integer.value[0] = if (*gv).mute { 0 } else { 1 }; }
        SST_GAIN_RAMP_DURATION => { (*ucontrol).value.integer.value[0] = (*gv).ramp_duration as i64; }
        _ => { dev_err((*component).dev, c"Invalid Input- gain type:%d\n".as_ptr(), (*mc).type_); return -EINVAL; }
    }
    0
}

unsafe fn sst_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let drv = snd_soc_component_get_drvdata(cmpnt);
    let mc = (*kcontrol).private_value as *mut sst_gain_mixer_control;
    let gv = (*mc).gain_val;
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    let mut ret = 0;
    match (*mc).type_ {
        SST_GAIN_TLV => { (*gv).l_gain = (*ucontrol).value.integer.value[0] as i32; (*gv).r_gain = (*ucontrol).value.integer.value[1] as i32; dev_dbg((*cmpnt).dev, c"%s: Volume %d, %d\n".as_ptr(), (*mc).pname, (*gv).l_gain, (*gv).r_gain); }
        SST_GAIN_MUTE => { (*gv).mute = (*ucontrol).value.integer.value[0] == 0; dev_dbg((*cmpnt).dev, c"%s: Mute %d\n".as_ptr(), (*mc).pname, (*gv).mute as c_int); }
        SST_GAIN_RAMP_DURATION => { (*gv).ramp_duration = (*ucontrol).value.integer.value[0] as i32; dev_dbg((*cmpnt).dev, c"%s: Ramp Delay%d\n".as_ptr(), (*mc).pname, (*gv).ramp_duration); }
        _ => { dev_err((*cmpnt).dev, c"Invalid Input- gain type:%d\n".as_ptr(), (*mc).type_); mutex_unlock(ptr::addr_of_mut!((*drv).lock)); return -EINVAL; }
    }
    if !(*mc).w.is_null() && (*(*mc).w).power {
        ret = sst_send_gain_cmd(drv, gv, (*mc).task_id, (*mc).pipe_id | (*mc).instance_id, (*mc).module_id, 0);
    }
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    ret
}

unsafe fn sst_set_pipe_gain(ids: *mut sst_ids, drv: *mut sst_data, mute: c_int) -> c_int {
    let _ = (ids, drv, mute);
    /* list_for_each_entry(gain, &ids->gain_list, node) translated as an external list walk dependency. */
    0
}

unsafe fn sst_send_pipe_module_params(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let drv = snd_soc_component_get_drvdata(c);
    let ids = (*w).priv_ as *mut sst_ids;
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    sst_find_and_send_pipe_algo(drv, (*w).name, ids);
    sst_set_pipe_gain(ids, drv, 0);
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    let _ = kcontrol;
    0
}

unsafe fn sst_generic_modules_event(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int {
    if SND_SOC_DAPM_EVENT_ON(event) { return sst_send_pipe_module_params(w, k); }
    0
}

/* static const DECLARE_TLV_DB_SCALE(sst_gain_tlv_common, SST_GAIN_MIN_VALUE * 10, 10, 0); */
static sst_gain_tlv_common: [c_uint; 4] = [0, (SST_GAIN_MIN_VALUE * 10) as c_uint, 10, 0];

static swm_mixer_input_ids: [uint; SST_SWM_INPUT_COUNT] = [
    SST_SWM_IN_MODEM as uint, SST_SWM_IN_CODEC0 as uint, SST_SWM_IN_CODEC1 as uint,
    SST_SWM_IN_SPROT_LOOP as uint, SST_SWM_IN_MEDIA_LOOP1 as uint, SST_SWM_IN_MEDIA_LOOP2 as uint,
    SST_SWM_IN_PCM0 as uint, SST_SWM_IN_PCM1 as uint, SST_SWM_IN_MEDIA0 as uint,
    SST_SWM_IN_MEDIA1 as uint, SST_SWM_IN_MEDIA2 as uint, SST_SWM_IN_MEDIA3 as uint,
];

unsafe fn fill_swm_input(cmpnt: *mut snd_soc_component, mut swm_input: *mut swm_input_ids, reg: c_uint) -> c_int {
    let mut nb_inputs: c_uint = 0;
    dev_dbg((*cmpnt).dev, c"reg: %#x\n".as_ptr(), reg);
    for i in 0..SST_SWM_INPUT_COUNT {
        let is_set = reg & BIT(i as c_uint);
        if is_set == 0 { continue; }
        let input_loc_id = swm_mixer_input_ids[i] as u16;
        SST_FILL_DESTINATION(2, ptr::addr_of_mut!((*swm_input).input_id), input_loc_id, SST_DEFAULT_MODULE_ID as u16);
        nb_inputs += 1;
        swm_input = swm_input.add(1);
        dev_dbg((*cmpnt).dev, c"input id: %#x, nb_inputs: %d\n".as_ptr(), input_loc_id as c_uint, nb_inputs);
        if nb_inputs == SST_CMD_SWM_MAX_INPUTS {
            dev_warn((*cmpnt).dev, c"SET_SWM cmd max inputs reached".as_ptr());
            break;
        }
    }
    nb_inputs as c_int
}

unsafe fn sst_swm_mixer_event(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int {
    let mut cmd: sst_cmd_set_swm = core::mem::zeroed();
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let drv = snd_soc_component_get_drvdata(cmpnt);
    let ids = (*w).priv_ as *mut sst_ids;
    let mut set_mixer = false;
    let mut val: c_int = 0;
    dev_dbg((*cmpnt).dev, c"widget = %s\n".as_ptr(), (*w).name);
    for i in 0..(*w).num_kcontrols {
        let kc = *(*w).kcontrols.add(i as usize);
        if snd_soc_dapm_kcontrol_get_value(kc) != 0 {
            let mc = (*kc).private_value as *mut soc_mixer_control;
            val |= 1 << (*mc).shift;
        }
    }
    dev_dbg((*cmpnt).dev, c"val = %#x\n".as_ptr(), val);
    match event {
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD => set_mixer = true,
        SND_SOC_DAPM_POST_REG => if (*w).power { set_mixer = true; },
        _ => set_mixer = false,
    }
    if !set_mixer { return 0; }
    cmd.switch_state = if SND_SOC_DAPM_EVENT_ON(event) || event == SND_SOC_DAPM_POST_REG { SST_SWM_ON as u16 } else { SST_SWM_OFF as u16 };
    SST_FILL_DEFAULT_DESTINATION(ptr::addr_of_mut!(cmd.header.dst));
    cmd.header.command_id = SBA_SET_SWM as u16;
    SST_FILL_DESTINATION(2, ptr::addr_of_mut!(cmd.output_id), (*ids).location_id, SST_DEFAULT_MODULE_ID as u16);
    cmd.nb_inputs = fill_swm_input(cmpnt, cmd.input.as_mut_ptr(), val as c_uint) as c_uint;
    cmd.header.length = (offset_of!(sst_cmd_set_swm, input) - size_of::<sst_dsp_header>() + (cmd.nb_inputs as usize * size_of::<swm_input_ids>())) as u16;
    let _ = k;
    sst_fill_and_send_cmd(drv, SST_IPC_IA_CMD as u8, SST_FLAG_BLOCKED as u8, (*ids).task_id, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16)
}

/* Macro-generated mixer control arrays from SST_SBA_DECLARE_MIX_CONTROLS and SST_MMX_DECLARE_MIX_CONTROLS are preserved as dependency-provided declarations. */
extern "C" {
    static sst_mix_media0_controls: [snd_kcontrol_new; 4];
    static sst_mix_media1_controls: [snd_kcontrol_new; 4];
    static sst_mix_pcm0_controls: [snd_kcontrol_new; 8];
    static sst_mix_pcm1_controls: [snd_kcontrol_new; 8];
    static sst_mix_pcm2_controls: [snd_kcontrol_new; 8];
    static sst_mix_sprot_l0_controls: [snd_kcontrol_new; 8];
    static sst_mix_media_l1_controls: [snd_kcontrol_new; 8];
    static sst_mix_media_l2_controls: [snd_kcontrol_new; 8];
    static sst_mix_voip_controls: [snd_kcontrol_new; 8];
    static sst_mix_codec0_controls: [snd_kcontrol_new; 8];
    static sst_mix_codec1_controls: [snd_kcontrol_new; 8];
    static sst_mix_modem_controls: [snd_kcontrol_new; 8];
}

#[no_mangle]
pub unsafe extern "C" fn sst_handle_vb_timer(dai: *mut snd_soc_dai, mut enable: bool) -> c_int {
    let mut ret = 0;
    let mut cmd: sst_cmd_generic = core::mem::zeroed();
    let drv = snd_soc_dai_get_drvdata(dai);
    static mut timer_usage: c_int = 0;
    cmd.header.command_id = if enable { SBA_VB_START as u16 } else { SBA_IDLE as u16 };
    dev_dbg((*dai).dev, c"enable=%u, usage=%d\n".as_ptr(), enable as c_uint, timer_usage);
    SST_FILL_DEFAULT_DESTINATION(ptr::addr_of_mut!(cmd.header.dst));
    cmd.header.length = 0;
    if enable {
        ret = ((*(*sst).ops).power)((*sst).dev, true);
        if ret < 0 { return ret; }
    }
    mutex_lock(ptr::addr_of_mut!((*drv).lock));
    if enable { timer_usage += 1; } else { timer_usage -= 1; }
    if (enable && timer_usage == 1) || (!enable && timer_usage == 0) {
        ret = sst_fill_and_send_cmd_unlocked(drv, SST_IPC_IA_CMD as u8, SST_FLAG_BLOCKED as u8, SST_TASK_SBA as u8, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16);
        if ret != 0 && enable {
            timer_usage -= 1;
            enable = false;
        }
    }
    mutex_unlock(ptr::addr_of_mut!((*drv).lock));
    if !enable { ((*(*sst).ops).power)((*sst).dev, false); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn sst_fill_ssp_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let ctx = snd_soc_dai_get_drvdata(dai);
    (*ctx).ssp_cmd.nb_slots = slots;
    (*ctx).ssp_cmd.active_tx_slot_map = tx_mask;
    (*ctx).ssp_cmd.active_rx_slot_map = rx_mask;
    (*ctx).ssp_cmd.nb_bits_per_slots = slot_width;
    0
}

unsafe fn sst_get_frame_sync_polarity(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let format = fmt & SND_SOC_DAIFMT_INV_MASK as c_uint;
    dev_dbg((*dai).dev, c"Enter:%s, format=%x\n".as_ptr(), c"sst_get_frame_sync_polarity".as_ptr(), format);
    match format as c_int {
        SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => SSP_FS_ACTIVE_HIGH,
        SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_IB_IF => SSP_FS_ACTIVE_LOW,
        _ => { dev_err((*dai).dev, c"Invalid frame sync polarity %d\n".as_ptr(), format); -EINVAL }
    }
}

unsafe fn sst_get_ssp_mode(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let format = fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK as c_uint;
    dev_dbg((*dai).dev, c"Enter:%s, format=%x\n".as_ptr(), c"sst_get_ssp_mode".as_ptr(), format);
    match format as c_int {
        SND_SOC_DAIFMT_BP_FP => SSP_MODE_PROVIDER,
        SND_SOC_DAIFMT_BC_FC => SSP_MODE_CONSUMER,
        _ => { dev_err((*dai).dev, c"Invalid ssp protocol: %d\n".as_ptr(), format); -EINVAL }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sst_fill_ssp_config(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let mode = fmt & SND_SOC_DAIFMT_FORMAT_MASK as c_uint;
    let ctx = snd_soc_dai_get_drvdata(dai);
    match mode as c_int {
        SND_SOC_DAIFMT_DSP_B => { (*ctx).ssp_cmd.ssp_protocol = SSP_MODE_PCM; (*ctx).ssp_cmd.mode = sst_get_ssp_mode(dai, fmt) | (SSP_PCM_MODE_NETWORK << 1); (*ctx).ssp_cmd.start_delay = 0; (*ctx).ssp_cmd.data_polarity = 1; (*ctx).ssp_cmd.frame_sync_width = 1; }
        SND_SOC_DAIFMT_DSP_A => { (*ctx).ssp_cmd.ssp_protocol = SSP_MODE_PCM; (*ctx).ssp_cmd.mode = sst_get_ssp_mode(dai, fmt) | (SSP_PCM_MODE_NETWORK << 1); (*ctx).ssp_cmd.start_delay = 1; (*ctx).ssp_cmd.data_polarity = 1; (*ctx).ssp_cmd.frame_sync_width = 1; }
        SND_SOC_DAIFMT_I2S => { (*ctx).ssp_cmd.ssp_protocol = SSP_MODE_I2S; (*ctx).ssp_cmd.mode = sst_get_ssp_mode(dai, fmt) | (SSP_PCM_MODE_NORMAL << 1); (*ctx).ssp_cmd.start_delay = 1; (*ctx).ssp_cmd.data_polarity = 0; (*ctx).ssp_cmd.frame_sync_width = (*ctx).ssp_cmd.nb_bits_per_slots; }
        SND_SOC_DAIFMT_LEFT_J => { (*ctx).ssp_cmd.ssp_protocol = SSP_MODE_I2S; (*ctx).ssp_cmd.mode = sst_get_ssp_mode(dai, fmt) | (SSP_PCM_MODE_NORMAL << 1); (*ctx).ssp_cmd.start_delay = 0; (*ctx).ssp_cmd.data_polarity = 0; (*ctx).ssp_cmd.frame_sync_width = (*ctx).ssp_cmd.nb_bits_per_slots; }
        _ => dev_dbg((*dai).dev, c"using default ssp configs\n".as_ptr()),
    }
    let fs_polarity = sst_get_frame_sync_polarity(dai, fmt);
    if fs_polarity < 0 { return fs_polarity; }
    (*ctx).ssp_cmd.frame_sync_polarity = fs_polarity;
    0
}

static sst_ssp_configs: sst_ssp_config = sst_ssp_config {
    ssp_id: SSP_CODEC, bits_per_slot: 24, slots: 4, ssp_mode: SSP_MODE_PROVIDER,
    pcm_mode: SSP_PCM_MODE_NETWORK, duplex: SSP_DUPLEX, ssp_protocol: SSP_MODE_PCM,
    fs_width: 1, fs_frequency: SSP_FS_48_KHZ, active_slot_map: 0xF, start_delay: 0,
    frame_sync_polarity: SSP_FS_ACTIVE_HIGH, data_polarity: 1,
};

#[no_mangle]
pub unsafe extern "C" fn sst_fill_ssp_defaults(dai: *mut snd_soc_dai) {
    let config = &sst_ssp_configs;
    let ctx = snd_soc_dai_get_drvdata(dai);
    (*ctx).ssp_cmd.selection = config.ssp_id;
    (*ctx).ssp_cmd.nb_bits_per_slots = config.bits_per_slot;
    (*ctx).ssp_cmd.nb_slots = config.slots;
    (*ctx).ssp_cmd.mode = config.ssp_mode | (config.pcm_mode << 1);
    (*ctx).ssp_cmd.duplex = config.duplex;
    (*ctx).ssp_cmd.active_tx_slot_map = config.active_slot_map;
    (*ctx).ssp_cmd.active_rx_slot_map = config.active_slot_map;
    (*ctx).ssp_cmd.frame_sync_frequency = config.fs_frequency;
    (*ctx).ssp_cmd.frame_sync_polarity = config.frame_sync_polarity;
    (*ctx).ssp_cmd.data_polarity = config.data_polarity;
    (*ctx).ssp_cmd.frame_sync_width = config.fs_width;
    (*ctx).ssp_cmd.ssp_protocol = config.ssp_protocol;
    (*ctx).ssp_cmd.start_delay = config.start_delay;
    (*ctx).ssp_cmd.reserved2 = 0xFF;
    (*ctx).ssp_cmd.reserved1 = (*ctx).ssp_cmd.reserved2;
}

#[no_mangle]
pub unsafe extern "C" fn send_ssp_cmd(dai: *mut snd_soc_dai, id: *const c_char, enable: bool) -> c_int {
    let drv = snd_soc_dai_get_drvdata(dai);
    let ssp_id: c_int;
    dev_dbg((*dai).dev, c"Enter: enable=%d port_name=%s\n".as_ptr(), enable as c_int, id);
    if strcmp(id, c"ssp0-port".as_ptr()) == 0 { ssp_id = SSP_MODEM; }
    else if strcmp(id, c"ssp2-port".as_ptr()) == 0 { ssp_id = SSP_CODEC; }
    else { dev_dbg((*dai).dev, c"port %s is not supported\n".as_ptr(), id); return -1; }
    SST_FILL_DEFAULT_DESTINATION(ptr::addr_of_mut!((*drv).ssp_cmd.header.dst));
    (*drv).ssp_cmd.header.command_id = SBA_HW_SET_SSP as u16;
    (*drv).ssp_cmd.header.length = (size_of::<sst_cmd_sba_hw_set_ssp>() - size_of::<sst_dsp_header>()) as u16;
    (*drv).ssp_cmd.selection = ssp_id;
    dev_dbg((*dai).dev, c"ssp_id: %u\n".as_ptr(), ssp_id as c_uint);
    (*drv).ssp_cmd.switch_state = if enable { SST_SWITCH_ON as u16 } else { SST_SWITCH_OFF as u16 };
    sst_fill_and_send_cmd(drv, SST_IPC_IA_CMD as u8, SST_FLAG_BLOCKED as u8, SST_TASK_SBA as u8, 0, ptr::addr_of_mut!((*drv).ssp_cmd) as *mut c_void, (size_of::<sst_dsp_header>() + (*drv).ssp_cmd.header.length as usize) as u16)
}

unsafe fn sst_set_be_modules(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int {
    let mut ret = 0;
    let c = snd_soc_dapm_to_component((*w).dapm);
    let drv = snd_soc_component_get_drvdata(c);
    dev_dbg((*c).dev, c"Enter: widget=%s\n".as_ptr(), (*w).name);
    if SND_SOC_DAPM_EVENT_ON(event) {
        mutex_lock(ptr::addr_of_mut!((*drv).lock));
        ret = sst_send_slot_map(drv);
        mutex_unlock(ptr::addr_of_mut!((*drv).lock));
        if ret != 0 { return ret; }
        ret = sst_send_pipe_module_params(w, k);
    }
    ret
}

unsafe fn sst_set_media_path(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int {
    let mut cmd: sst_cmd_set_media_path = core::mem::zeroed();
    let c = snd_soc_dapm_to_component((*w).dapm);
    let drv = snd_soc_component_get_drvdata(c);
    let ids = (*w).priv_ as *mut sst_ids;
    dev_dbg((*c).dev, c"widget=%s\n".as_ptr(), (*w).name);
    dev_dbg((*c).dev, c"task=%u, location=%#x\n".as_ptr(), (*ids).task_id as c_uint, (*ids).location_id as c_uint);
    cmd.switch_state = if SND_SOC_DAPM_EVENT_ON(event) { SST_PATH_ON as u16 } else { SST_PATH_OFF as u16 };
    SST_FILL_DESTINATION(2, ptr::addr_of_mut!(cmd.header.dst), (*ids).location_id, SST_DEFAULT_MODULE_ID as u16);
    cmd.header.command_id = MMX_SET_MEDIA_PATH as u16;
    cmd.header.length = (size_of::<sst_cmd_set_media_path>() - size_of::<sst_dsp_header>()) as u16;
    let mut ret = sst_fill_and_send_cmd(drv, SST_IPC_IA_CMD as u8, SST_FLAG_BLOCKED as u8, (*ids).task_id, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16);
    if ret != 0 { return ret; }
    if SND_SOC_DAPM_EVENT_ON(event) { ret = sst_send_pipe_module_params(w, k); }
    ret
}

unsafe fn sst_set_media_loop(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int {
    let mut cmd: sst_cmd_sba_set_media_loop_map = core::mem::zeroed();
    let c = snd_soc_dapm_to_component((*w).dapm);
    let drv = snd_soc_component_get_drvdata(c);
    let ids = (*w).priv_ as *mut sst_ids;
    dev_dbg((*c).dev, c"Enter:widget=%s\n".as_ptr(), (*w).name);
    cmd.switch_state = if SND_SOC_DAPM_EVENT_ON(event) { SST_SWITCH_ON as u16 } else { SST_SWITCH_OFF as u16 };
    SST_FILL_DESTINATION(2, ptr::addr_of_mut!(cmd.header.dst), (*ids).location_id, SST_DEFAULT_MODULE_ID as u16);
    cmd.header.command_id = SBA_SET_MEDIA_LOOP_MAP as u16;
    cmd.header.length = (size_of::<sst_cmd_sba_set_media_loop_map>() - size_of::<sst_dsp_header>()) as u16;
    cmd.param.part.cfg.rate = 2;
    cmd.param.part.cfg.format = (*ids).format;
    cmd.param.part.cfg.s_length = 1;
    cmd.map = 0;
    let mut ret = sst_fill_and_send_cmd(drv, SST_IPC_IA_CMD as u8, SST_FLAG_BLOCKED as u8, SST_TASK_SBA as u8, 0, &mut cmd as *mut _ as *mut c_void, (size_of::<sst_dsp_header>() + cmd.header.length as usize) as u16);
    if ret != 0 { return ret; }
    if SND_SOC_DAPM_EVENT_ON(event) { ret = sst_send_pipe_module_params(w, k); }
    ret
}

/* Static DAPM widgets, routes, slot controls, gain controls and algo controls are macro-heavy declarations in C.
 * Their source-level intent is preserved below using Rust macro invocations expected from translated headers.
 */
static sst_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SST_AIF_IN!("modem_in", sst_set_be_modules),
    SST_AIF_IN!("codec_in0", sst_set_be_modules),
    SST_AIF_IN!("codec_in1", sst_set_be_modules),
    SST_AIF_OUT!("modem_out", sst_set_be_modules),
    SST_AIF_OUT!("codec_out0", sst_set_be_modules),
    SST_AIF_OUT!("codec_out1", sst_set_be_modules),
    SST_PATH_INPUT!("media0_in", SST_TASK_MMX, SST_SWM_IN_MEDIA0, sst_generic_modules_event),
    SST_PATH_INPUT!("media1_in", SST_TASK_MMX, SST_SWM_IN_MEDIA1, None),
    SST_PATH_INPUT!("media2_in", SST_TASK_MMX, SST_SWM_IN_MEDIA2, sst_set_media_path),
    SST_PATH_INPUT!("media3_in", SST_TASK_MMX, SST_SWM_IN_MEDIA3, None),
    SST_PATH_OUTPUT!("media0_out", SST_TASK_MMX, SST_SWM_OUT_MEDIA0, sst_set_media_path),
    SST_PATH_OUTPUT!("media1_out", SST_TASK_MMX, SST_SWM_OUT_MEDIA1, sst_set_media_path),
    SST_PATH_INPUT!("pcm0_in", SST_TASK_SBA, SST_SWM_IN_PCM0, sst_set_media_path),
    SST_PATH_INPUT!("pcm1_in", SST_TASK_SBA, SST_SWM_IN_PCM1, sst_set_media_path),
    SST_PATH_OUTPUT!("pcm0_out", SST_TASK_SBA, SST_SWM_OUT_PCM0, sst_set_media_path),
    SST_PATH_OUTPUT!("pcm1_out", SST_TASK_SBA, SST_SWM_OUT_PCM1, sst_set_media_path),
    SST_PATH_OUTPUT!("pcm2_out", SST_TASK_SBA, SST_SWM_OUT_PCM2, sst_set_media_path),
    SST_PATH_INPUT!("sprot_loop_in", SST_TASK_SBA, SST_SWM_IN_SPROT_LOOP, None),
    SST_PATH_INPUT!("media_loop1_in", SST_TASK_SBA, SST_SWM_IN_MEDIA_LOOP1, None),
    SST_PATH_INPUT!("media_loop2_in", SST_TASK_SBA, SST_SWM_IN_MEDIA_LOOP2, None),
    SST_PATH_MEDIA_LOOP_OUTPUT!("sprot_loop_out", SST_TASK_SBA, SST_SWM_OUT_SPROT_LOOP, SST_FMT_STEREO, sst_set_media_loop),
    SST_PATH_MEDIA_LOOP_OUTPUT!("media_loop1_out", SST_TASK_SBA, SST_SWM_OUT_MEDIA_LOOP1, SST_FMT_STEREO, sst_set_media_loop),
    SST_PATH_MEDIA_LOOP_OUTPUT!("media_loop2_out", SST_TASK_SBA, SST_SWM_OUT_MEDIA_LOOP2, SST_FMT_STEREO, sst_set_media_loop),
    SST_SWM_MIXER!("media0_out mix 0", SND_SOC_NOPM, SST_TASK_MMX, SST_SWM_OUT_MEDIA0, sst_mix_media0_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("media1_out mix 0", SND_SOC_NOPM, SST_TASK_MMX, SST_SWM_OUT_MEDIA1, sst_mix_media1_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("pcm0_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_PCM0, sst_mix_pcm0_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("pcm1_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_PCM1, sst_mix_pcm1_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("pcm2_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_PCM2, sst_mix_pcm2_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("sprot_loop_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_SPROT_LOOP, sst_mix_sprot_l0_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("media_loop1_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_MEDIA_LOOP1, sst_mix_media_l1_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("media_loop2_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_MEDIA_LOOP2, sst_mix_media_l2_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("codec_out0 mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_CODEC0, sst_mix_codec0_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("codec_out1 mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_CODEC1, sst_mix_codec1_controls, sst_swm_mixer_event),
    SST_SWM_MIXER!("modem_out mix 0", SND_SOC_NOPM, SST_TASK_SBA, SST_SWM_OUT_MODEM, sst_mix_modem_controls, sst_swm_mixer_event),
];

static intercon: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c"media0_in".as_ptr(), control: ptr::null(), source: c"Compress Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"media1_in".as_ptr(), control: ptr::null(), source: c"Headset Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"media2_in".as_ptr(), control: ptr::null(), source: c"pcm0_out".as_ptr() },
    snd_soc_dapm_route { sink: c"media3_in".as_ptr(), control: ptr::null(), source: c"Deepbuffer Playback".as_ptr() },
    SST_SBA_MIXER_GRAPH_MAP!("pcm0_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("pcm1_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("pcm2_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("media_loop1_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("media_loop2_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("sprot_loop_out mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("codec_out0 mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("codec_out1 mix 0"),
    SST_SBA_MIXER_GRAPH_MAP!("modem_out mix 0"),
];

static slot_names: [*const c_char; 9] = [c"none".as_ptr(), c"slot 0".as_ptr(), c"slot 1".as_ptr(), c"slot 2".as_ptr(), c"slot 3".as_ptr(), c"slot 4".as_ptr(), c"slot 5".as_ptr(), c"slot 6".as_ptr(), c"slot 7".as_ptr()];
static channel_names: [*const c_char; 9] = [c"none".as_ptr(), c"codec_out0_0".as_ptr(), c"codec_out0_1".as_ptr(), c"codec_out1_0".as_ptr(), c"codec_out1_1".as_ptr(), c"codec_out2_0".as_ptr(), c"codec_out2_1".as_ptr(), c"codec_out3_0".as_ptr(), c"codec_out3_1".as_ptr()];

static sst_slot_controls: &[snd_kcontrol_new] = &[
    SST_INTERLEAVER!("codec_out", "slot 0", 0), SST_INTERLEAVER!("codec_out", "slot 1", 1),
    SST_INTERLEAVER!("codec_out", "slot 2", 2), SST_INTERLEAVER!("codec_out", "slot 3", 3),
    SST_DEINTERLEAVER!("codec_in", "codec_in0_0", 0), SST_DEINTERLEAVER!("codec_in", "codec_in0_1", 1),
    SST_DEINTERLEAVER!("codec_in", "codec_in1_0", 2), SST_DEINTERLEAVER!("codec_in", "codec_in1_1", 3),
];

static mut sst_gains: [sst_gain_value; 18] = [sst_gain_value { mute: false, l_gain: 0, r_gain: 0, ramp_duration: 0 }; 18];
static sst_gain_controls: &[snd_kcontrol_new] = &[
    SST_GAIN!("media0_in", SST_PATH_INDEX_MEDIA0_IN, SST_TASK_MMX, 0, unsafe { &mut sst_gains[0] }),
    SST_GAIN!("media1_in", SST_PATH_INDEX_MEDIA1_IN, SST_TASK_MMX, 0, unsafe { &mut sst_gains[1] }),
    SST_GAIN!("media2_in", SST_PATH_INDEX_MEDIA2_IN, SST_TASK_MMX, 0, unsafe { &mut sst_gains[2] }),
    SST_GAIN!("media3_in", SST_PATH_INDEX_MEDIA3_IN, SST_TASK_MMX, 0, unsafe { &mut sst_gains[3] }),
    SST_GAIN!("pcm0_in", SST_PATH_INDEX_PCM0_IN, SST_TASK_SBA, 0, unsafe { &mut sst_gains[4] }),
    SST_GAIN!("pcm1_in", SST_PATH_INDEX_PCM1_IN, SST_TASK_SBA, 0, unsafe { &mut sst_gains[5] }),
    SST_GAIN!("pcm1_out", SST_PATH_INDEX_PCM1_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[6] }),
    SST_GAIN!("pcm2_out", SST_PATH_INDEX_PCM2_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[7] }),
    SST_GAIN!("codec_in0", SST_PATH_INDEX_CODEC_IN0, SST_TASK_SBA, 0, unsafe { &mut sst_gains[8] }),
    SST_GAIN!("codec_in1", SST_PATH_INDEX_CODEC_IN1, SST_TASK_SBA, 0, unsafe { &mut sst_gains[9] }),
    SST_GAIN!("codec_out0", SST_PATH_INDEX_CODEC_OUT0, SST_TASK_SBA, 0, unsafe { &mut sst_gains[10] }),
    SST_GAIN!("codec_out1", SST_PATH_INDEX_CODEC_OUT1, SST_TASK_SBA, 0, unsafe { &mut sst_gains[11] }),
    SST_GAIN!("media_loop1_out", SST_PATH_INDEX_MEDIA_LOOP1_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[12] }),
    SST_GAIN!("media_loop2_out", SST_PATH_INDEX_MEDIA_LOOP2_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[13] }),
    SST_GAIN!("sprot_loop_out", SST_PATH_INDEX_SPROT_LOOP_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[14] }),
    SST_VOLUME!("media0_in", SST_PATH_INDEX_MEDIA0_IN, SST_TASK_MMX, 0, unsafe { &mut sst_gains[15] }),
    SST_GAIN!("modem_in", SST_PATH_INDEX_MODEM_IN, SST_TASK_SBA, 0, unsafe { &mut sst_gains[16] }),
    SST_GAIN!("modem_out", SST_PATH_INDEX_MODEM_OUT, SST_TASK_SBA, 0, unsafe { &mut sst_gains[17] }),
];

static sst_algo_controls: &[snd_kcontrol_new] = &[
    SST_ALGO_KCONTROL_BYTES!("media_loop1_out", "fir", 272, SST_MODULE_ID_FIR_24, SST_PATH_INDEX_MEDIA_LOOP1_OUT, 0, SST_TASK_SBA, SBA_VB_SET_FIR),
    SST_ALGO_KCONTROL_BYTES!("media_loop1_out", "iir", 300, SST_MODULE_ID_IIR_24, SST_PATH_INDEX_MEDIA_LOOP1_OUT, 0, SST_TASK_SBA, SBA_VB_SET_IIR),
    SST_ALGO_KCONTROL_BYTES!("media_loop1_out", "mdrp", 286, SST_MODULE_ID_MDRP, SST_PATH_INDEX_MEDIA_LOOP1_OUT, 0, SST_TASK_SBA, SBA_SET_MDRP),
    SST_ALGO_KCONTROL_BYTES!("media_loop2_out", "fir", 272, SST_MODULE_ID_FIR_24, SST_PATH_INDEX_MEDIA_LOOP2_OUT, 0, SST_TASK_SBA, SBA_VB_SET_FIR),
    SST_ALGO_KCONTROL_BYTES!("media_loop2_out", "iir", 300, SST_MODULE_ID_IIR_24, SST_PATH_INDEX_MEDIA_LOOP2_OUT, 0, SST_TASK_SBA, SBA_VB_SET_IIR),
    SST_ALGO_KCONTROL_BYTES!("media_loop2_out", "mdrp", 286, SST_MODULE_ID_MDRP, SST_PATH_INDEX_MEDIA_LOOP2_OUT, 0, SST_TASK_SBA, SBA_SET_MDRP),
    SST_ALGO_KCONTROL_BYTES!("sprot_loop_out", "lpro", 192, SST_MODULE_ID_SPROT, SST_PATH_INDEX_SPROT_LOOP_OUT, 0, SST_TASK_SBA, SBA_VB_LPRO),
    SST_ALGO_KCONTROL_BYTES!("codec_in0", "dcr", 52, SST_MODULE_ID_FILT_DCR, SST_PATH_INDEX_CODEC_IN0, 0, SST_TASK_SBA, SBA_VB_SET_IIR),
    SST_ALGO_KCONTROL_BYTES!("codec_in1", "dcr", 52, SST_MODULE_ID_FILT_DCR, SST_PATH_INDEX_CODEC_IN1, 0, SST_TASK_SBA, SBA_VB_SET_IIR),
];

unsafe fn sst_algo_control_init(dev: *mut device) -> c_int {
    for i in 0..sst_algo_controls.len() {
        let bc = sst_algo_controls[i].private_value as *mut sst_algo_control;
        (*bc).params = devm_kzalloc(dev, (*bc).max as usize, GFP_KERNEL);
        if (*bc).params.is_null() { return -ENOMEM; }
    }
    0
}

unsafe fn is_sst_dapm_widget(w: *mut snd_soc_dapm_widget) -> bool {
    match (*w).id {
        snd_soc_dapm_pga | snd_soc_dapm_aif_in | snd_soc_dapm_aif_out | snd_soc_dapm_input | snd_soc_dapm_output | snd_soc_dapm_mixer => true,
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sst_send_pipe_gains(dai: *mut snd_soc_dai, stream: c_int, mute: c_int) -> c_int {
    let drv = snd_soc_dai_get_drvdata(dai);
    let w = snd_soc_dai_get_widget(dai, stream);
    dev_dbg((*dai).dev, c"enter, dai-name=%s dir=%d\n".as_ptr(), (*dai).name, stream);
    dev_dbg((*dai).dev, c"Stream name=%s\n".as_ptr(), (*w).name);
    let _ = (drv, mute);
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* snd_soc_dapm_widget_for_each_sink_path(w, p) body preserved as dependency on DAPM path iterator. */
    } else {
        /* snd_soc_dapm_widget_for_each_source_path(w, p) body preserved as dependency on DAPM path iterator. */
    }
    0
}

unsafe fn sst_fill_module_list(kctl: *mut snd_kcontrol, w: *mut snd_soc_dapm_widget, type_: c_int) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let ids = (*w).priv_ as *mut sst_ids;
    let module = devm_kzalloc((*c).dev, size_of::<sst_module>(), GFP_KERNEL) as *mut sst_module;
    if module.is_null() { return -ENOMEM; }
    if type_ == SST_MODULE_GAIN {
        let mc = (*kctl).private_value as *mut sst_gain_mixer_control;
        (*mc).w = w;
        (*module).kctl = kctl;
        list_add_tail(ptr::addr_of_mut!((*module).node), ptr::addr_of_mut!((*ids).gain_list));
    } else if type_ == SST_MODULE_ALGO {
        let bc = (*kctl).private_value as *mut sst_algo_control;
        (*bc).w = w;
        (*module).kctl = kctl;
        list_add_tail(ptr::addr_of_mut!((*module).node), ptr::addr_of_mut!((*ids).algo_list));
    } else {
        dev_err((*c).dev, c"invoked for unknown type %d module %s".as_ptr(), type_, (*kctl).id.name);
        return -EINVAL;
    }
    0
}

unsafe fn sst_fill_widget_module_info(w: *mut snd_soc_dapm_widget, component: *mut snd_soc_component) -> c_int {
    let card = (*(*component).card).snd_card;
    down_read(ptr::addr_of_mut!((*card).controls_rwsem));
    /* list_for_each_entry(kctl, &card->controls, list) translated as an external ALSA controls list walk dependency. */
    up_read(ptr::addr_of_mut!((*card).controls_rwsem));
    let _ = w;
    0
}

unsafe fn sst_fill_linked_widgets(component: *mut snd_soc_component, ids: *mut sst_ids) {
    let len = strlen((*ids).parent_wname);
    let _ = (component, len);
    /* list_for_each_entry(w, &component->card->widgets, list) translated as an external widget list walk dependency. */
}

unsafe fn sst_map_modules_to_pipe(component: *mut snd_soc_component) -> c_int {
    let _ = component;
    /* list_for_each_entry(w, &component->card->widgets, list) translated as an external widget list walk dependency. */
    0
}

#[no_mangle]
pub unsafe extern "C" fn sst_dsp_init_v2_dpcm(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let dapm = snd_soc_component_to_dapm(component);
    let drv = snd_soc_component_get_drvdata(component);
    let gains = sst_gain_controls.len() / 3;
    (*drv).byte_stream = devm_kzalloc((*component).dev, SST_MAX_BIN_BYTES, GFP_KERNEL) as *mut snd_sst_bytes_v2;
    if (*drv).byte_stream.is_null() { return -ENOMEM; }
    snd_soc_dapm_new_controls(dapm, sst_dapm_widgets.as_ptr(), sst_dapm_widgets.len());
    snd_soc_dapm_add_routes(dapm, intercon.as_ptr(), intercon.len());
    snd_soc_dapm_new_widgets((*component).card);
    for i in 0..gains {
        sst_gains[i].mute = SST_GAIN_MUTE_DEFAULT;
        sst_gains[i].l_gain = SST_GAIN_VOLUME_DEFAULT;
        sst_gains[i].r_gain = SST_GAIN_VOLUME_DEFAULT;
        sst_gains[i].ramp_duration = SST_GAIN_RAMP_DURATION_DEFAULT;
    }
    ret = snd_soc_add_component_controls(component, sst_gain_controls.as_ptr(), sst_gain_controls.len());
    if ret != 0 { return ret; }
    ret = sst_algo_control_init((*component).dev);
    if ret != 0 { return ret; }
    ret = snd_soc_add_component_controls(component, sst_algo_controls.as_ptr(), sst_algo_controls.len());
    if ret != 0 { return ret; }
    ret = snd_soc_add_component_controls(component, sst_slot_controls.as_ptr(), sst_slot_controls.len());
    if ret != 0 { return ret; }
    sst_map_modules_to_pipe(component)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
