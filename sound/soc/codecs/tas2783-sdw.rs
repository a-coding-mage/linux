// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAS2783 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2783 driver implements a flexible and configurable
// algo coefficient setting for single TAS2783 chips.
//
// Author: Niranjan H Y <niranjanhy@ti.com>
// Author: Baojun Xu <baojun.xu@ti.com>
// Author: Kevin Lu <kevin-lu@ti.com>

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C include dependencies intentionally remain external to this translation:
// linux/cleanup.h, linux/unaligned.h, linux/crc32.h, linux/efi.h, linux/err.h,
// linux/firmware.h, linux/init.h, linux/module.h, sound/pcm_params.h,
// linux/pm.h, linux/pm_runtime.h, linux/regmap.h, linux/wait.h,
// linux/soundwire/sdw*.h, sound/sdw.h, sound/soc.h, sound/tlv.h,
// sound/tas2781-tlv.h, sound/sdca_function.h, sound/sdca_regmap.h,
// and "tas2783.h".
// The CONFIG_PCI include block is preserved in tas_generate_fw_name().

const TIMEOUT_FW_DL_MS: c_int = 3000;
const FW_DL_OFFSET: c_int = 84; // binary file information
const FW_FL_HDR: c_int = 20; // minimum number of bytes in one chunk
const TAS2783_PROBE_TIMEOUT: c_int = 5000;
// C: EFI_GUID(0x1f52d2a1, 0xbb3a, 0x457d, 0xbc, 0x09, 0x43, 0xa3, 0xf4, 0x31, 0x0a, 0x92)
static TAS2783_CALI_GUID: efi_guid_t = efi_guid_t { b: [0; 16] };

type u8 = core::ffi::c_uchar;
type u32 = c_uint;
type s32 = c_int;
type size_t = usize;
type bool_ = bool;

type efi_status_t = c_long;
type efi_char16_t = u16;

#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct class { _priv: [u8; 0] }
#[repr(C)] pub struct attribute_group { _priv: [u8; 0] }
#[repr(C)] pub struct regmap { _priv: [u8; 0] }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _priv: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct sdw_stream_runtime { _priv: [u8; 0] }
#[repr(C)] pub struct sdw_stream_config { _priv: [u8; 0] }
#[repr(C)] pub struct sdw_port_config { pub num: c_uint }
#[repr(C)] pub struct sdw_bus { pub link_id: u8 }
#[repr(C)] pub struct sdw_slave_id { pub unique_id: u8 }
#[repr(C)] pub struct sdw_dpn_prop { pub simple_ch_prep_sm: bool }
#[repr(C)] pub struct sdw_slave_prop { pub sink_dpn_prop: *mut sdw_dpn_prop }
#[repr(C)] pub struct sdca_function_desc { pub type_: c_int }
#[repr(C)] pub struct sdca_data { pub num_functions: c_int, pub function: *mut sdca_function_desc }
#[repr(C)] pub struct sdw_slave { pub dev: device, pub bus: *mut sdw_bus, pub id: sdw_slave_id, pub prop: sdw_slave_prop, pub sdca_data: sdca_data }
#[repr(C)] pub struct sdw_device_id { _priv: [u8; 0] }
#[repr(C)] pub struct sdw_prepare_ch { pub num: c_uint, pub ch_mask: c_uint }
#[repr(C)] pub struct sdca_function_data { pub desc: *mut sdca_function_desc }
#[repr(C)] pub struct firmware { pub size: size_t, pub data: *const u8 }
#[repr(C)] pub struct tm { pub tm_sec: c_int, pub tm_min: c_int, pub tm_hour: c_int, pub tm_mday: c_int, pub tm_mon: c_int, pub tm_year: c_int }
#[repr(C)] pub struct efi_guid_t { pub b: [u8; 16] }

#[repr(C)]
struct tas_fw_hdr {
    size: u32,
    version_offset: u32,
    plt_id: u32,
    ppc3_ver: u32,
    timestamp: u32,
    ddc_name: [u8; 64],
}

#[repr(C)]
struct tas_fw_file {
    vendor_id: u32,
    file_id: u32,
    version: u32,
    length: u32,
    dest_addr: u32,
    fw_data: *mut u8,
}

#[repr(C)]
struct calibration_data {
    is_valid: u32,
    read_sz: c_ulong,
    data: [u8; TAS2783_CALIB_DATA_SZ as usize],
}

#[repr(C)]
struct tas2783_prv {
    component: *mut snd_soc_component,
    cali_data: calibration_data,
    sdw_peripheral: *mut sdw_slave,
    sa_func_data: *mut sdca_function_data,
    status: sdw_slave_status,
    calib_lock: mutex,
    pde_lock: mutex,
    regmap: *mut regmap,
    dev: *mut device,
    class: *mut class,
    cal_attr_groups: *mut attribute_group,
    tm: tm,
    rca_binaryname: [u8; 64],
    dev_name: [u8; 32],
    hw_init: bool,
    fw_wait: wait_queue_head_t,
    fw_dl_task_done: bool,
    fw_dl_success: bool,
    fw_use_fallback: bool,
}

// Register defaults and initialization sequence from the C file are external-macro-heavy
// literal data. They are preserved by name and consumed through the same external symbols.
static tas2783_cali_reg: [u32; 5] = [TAS2783_CAL_R0, TAS2783_CAL_INVR0, TAS2783_CAL_R0LOW, TAS2783_CAL_POWER, TAS2783_CAL_TLIM];
// C static const struct reg_default tas2783_reg_default[]: see source for the full literal table of TASDEV_REG_SDW()/SDW_SDCA_CTL() defaults.
// C static const struct reg_sequence tas2783_init_seq[]: see source for the full literal REG_SEQ0() sequence.

#[repr(C)] enum sdw_slave_status { SDW_SLAVE_UNATTACHED = 0, SDW_SLAVE_ATTACHED = 1, SDW_SLAVE_ALERT = 2 }
#[repr(C)] enum sdw_port_prep_ops { SDW_OPS_PORT_PRE_PREP = 0, SDW_OPS_PORT_PRE_DEPREP = 1, SDW_OPS_PORT_POST_PREP = 2, SDW_OPS_PORT_POST_DEPREP = 3 }

unsafe extern "C" {
    static efi: efi;
    fn snd_soc_get_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> s32;
    fn snd_soc_put_volsw(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> s32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn crc32(crc: u32, data: *const u8, len: size_t) -> u32;
    fn time64_to_tm(t: u32, offset: c_int, tm: *mut tm);
    fn regmap_bulk_write(map: *mut regmap, reg: u32, val: *const u8, len: size_t) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> s32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> s32;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const c_void, n: usize) -> s32;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_drop_region(map: *mut regmap, min: c_uint, max: c_uint);
    fn sdw_nwrite_no_pm(slave: *mut sdw_slave, addr: u32, count: u32, buf: *mut u8) -> s32;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: u32, val: c_int) -> s32;
    fn sdw_stream_add_slave(slave: *mut sdw_slave, sc: *mut sdw_stream_config, pc: *mut sdw_port_config, n: c_int, rt: *mut sdw_stream_runtime) -> s32;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, rt: *mut sdw_stream_runtime);
    fn snd_soc_dapm_to_component(d: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(c: *mut snd_soc_component) -> *mut tas2783_prv;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: s32, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, sub: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, sub: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(sub: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, sc: *mut sdw_stream_config, pc: *mut sdw_port_config);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn release_firmware(f: *const firmware);
    fn request_firmware_nowait(module: *mut c_void, action: c_int, name: *const u8, dev: *mut device, gfp: c_int, context: *mut c_void, cb: unsafe extern "C" fn(*const firmware, *mut c_void)) -> s32;
    fn wait_event_timeout(wq: wait_queue_head_t, condition: bool, timeout: c_ulong) -> c_long;
    fn msecs_to_jiffies(ms: c_int) -> c_ulong;
    fn scnprintf(buf: *mut u8, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut tas2783_prv;
    fn devm_snd_soc_register_component(dev: *mut device, drv: *const c_void, dai: *mut c_void, n: usize) -> s32;
    fn snd_soc_unregister_component(dev: *mut device);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn sdw_slave_wait_for_init(slave: *mut sdw_slave, timeout: c_int) -> c_int;
    fn sdw_show_ping_status(bus: *mut sdw_bus, status: bool);
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn sdw_slave_read_prop(slave: *mut sdw_slave) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn sdca_parse_function(dev: *mut device, data: *mut sdca_function_data) -> c_int;
    fn devm_regmap_init_sdw_mbq_cfg(dev: *mut device, slave: *mut sdw_slave, cfg: *const c_void, mbq: *const c_void) -> *mut regmap;
    fn sdca_regmap_write_init(dev: *mut device, map: *mut regmap, data: *mut sdca_function_data) -> s32;
    fn mutex_init(m: *mut mutex);
    fn mutex_destroy(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
}

#[repr(C)] struct efi { get_variable: unsafe extern "C" fn(*const efi_char16_t, *mut efi_guid_t, *mut u32, *mut c_ulong, *mut u8) -> efi_status_t }

#[inline] unsafe fn get_unaligned_le32(p: *const u8) -> u32 { u32::from_le_bytes([*p.add(0), *p.add(1), *p.add(2), *p.add(3)]) }

unsafe fn tas2783_sdca_mbq_size(_dev: *mut device, reg: u32) -> c_int {
    match reg {
        0x000..=0x080 | 0x100..=0x140 | 0x200..=0x240 | 0x300..=0x340 | 0x400..=0x440 | 0x500..=0x540 | 0x800000..=0x803fff | 0x807e80..=0x807eff => 1,
        _ if reg == SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_FU21, TAS2783_SDCA_CTL_FU_MUTE, TAS2783_DEVICE_CHANNEL_LEFT) => 1,
        _ => 0,
    }
}

unsafe fn tas2783_readable_register(dev: *mut device, reg: c_uint) -> bool { tas2783_sdca_mbq_size(dev, reg) > 0 }

unsafe fn tas2783_writeable_register(dev: *mut device, reg: c_uint) -> bool {
    // The Latency Control of every Entity, together with the Power Domain actual
    // state and protection status, is read-only and must not be synced back.
    match reg {
        _ if reg == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, TAS2783_SDCA_ENT_FU21, 0x10, 0) => false,
        _ if reg == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, TAS2783_SDCA_ENT_FU23, 0x10, 0) => false,
        _ if reg == SDW_SDCA_CTL(FUNC_NUM_SMART_AMP, TAS2783_SDCA_ENT_FU26, 0x10, 0) => false,
        _ => tas2783_sdca_mbq_size(dev, reg) > 0,
    }
}

unsafe fn tas2783_volatile_register(_dev: *mut device, reg: u32) -> bool {
    matches!(reg, 0x000..=0x080 | 0x100..=0x140 | 0x200..=0x240 | 0x300..=0x340 | 0x400..=0x440 | 0x500..=0x540 | 0x800001)
}

unsafe fn tas2783_digital_getvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> s32 { snd_soc_get_volsw(kcontrol, ucontrol) }
unsafe fn tas2783_digital_putvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> s32 { snd_soc_put_volsw(kcontrol, ucontrol) }
unsafe fn tas2783_amp_getvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> s32 { snd_soc_get_volsw(kcontrol, ucontrol) }
unsafe fn tas2783_amp_putvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> s32 { snd_soc_put_volsw(kcontrol, ucontrol) }

unsafe fn tas2783_validate_calibdata(tas_dev: *mut tas2783_prv, data: *mut u8, size: u32) -> s32 {
    let mut i: u32 = 0;
    let tmp_val = data as *mut u32;
    if *tmp_val.add({ let old = i; i += 1; old as usize }) != 2783 { dev_err((*tas_dev).dev, b"cal data magic number mismatch\0".as_ptr() as *const c_char); return -EINVAL; }
    let spk_count = *tmp_val.add({ let old = i; i += 1; old as usize });
    if spk_count > TAS2783_CALIB_MAX_SPK_COUNT { dev_err((*tas_dev).dev, b"cal data spk_count too large\0".as_ptr() as *const c_char); return -EINVAL; }
    let ts = *tmp_val.add({ let old = i; i += 1; old as usize });
    let mut tmv: tm = core::mem::zeroed();
    time64_to_tm(ts, 0, &mut tmv);
    dev_dbg((*tas_dev).dev, b"cal data timestamp: %ld-%d-%d %d:%d:%d\0".as_ptr() as *const c_char, tmv.tm_year + 1900, tmv.tm_mon + 1, tmv.tm_mday, tmv.tm_hour, tmv.tm_min, tmv.tm_sec);
    let size_calculated = spk_count * TAS2783_CALIB_PARAMS * size_of::<u32>() as u32 + TAS2783_CALIB_HDR_SZ + TAS2783_CALIB_CRC_SZ;
    if size_calculated > TAS2783_CALIB_DATA_SZ { dev_err((*tas_dev).dev, b"cali data sz too large\0".as_ptr() as *const c_char); return -EINVAL; }
    else if size < size_calculated { dev_err((*tas_dev).dev, b"cali data size mismatch calc=%u vs %d\n\0".as_ptr() as *const c_char, size, size_calculated); return -EINVAL; }
    let crc_calculated = crc32(!0u32, data, (size_calculated - TAS2783_CALIB_CRC_SZ) as usize) ^ !0u32;
    let crc_read = *tmp_val.add(((size_calculated - TAS2783_CALIB_CRC_SZ) / size_of::<u32>() as u32) as usize);
    if crc_calculated != crc_read { dev_err((*tas_dev).dev, b"calib data integrity check fail, 0x%08x vs 0x%08x\n\0".as_ptr() as *const c_char, crc_calculated, crc_read); return -EINVAL; }
    0
}

unsafe fn tas2783_set_calib_params_to_device(tas_dev: *mut tas2783_prv, cali_data: *mut u32) {
    let dev_count = *cali_data.add(1);
    let mut offset: u32 = 3;
    let mut device_num: u32 = 0;
    while device_num < dev_count {
        if *cali_data.add(offset as usize) != (*(*tas_dev).sdw_peripheral).id.unique_id as u32 { offset += TAS2783_CALIB_PARAMS; device_num += 1; continue; }
        offset += 1;
        let mut i = 0usize;
        while i < tas2783_cali_reg.len() {
            let reg_value = *cali_data.add((offset + i as u32) as usize);
            let buf = [(reg_value >> 24) as u8, (reg_value >> 16) as u8, (reg_value >> 8) as u8, (reg_value & 0xff) as u8];
            regmap_bulk_write((*tas_dev).regmap, tas2783_cali_reg[i], buf.as_ptr(), size_of::<u32>());
            i += 1;
        }
        break;
    }
    if device_num == dev_count { dev_err((*tas_dev).dev, b"unique id not found in the calib data\n\0".as_ptr() as *const c_char); } else { dev_dbg((*tas_dev).dev, b"calib data update done\n\0".as_ptr() as *const c_char); }
}

unsafe fn tas_fw_read_hdr(data: *const u8, hdr: *mut tas_fw_hdr) -> s32 {
    (*hdr).size = get_unaligned_le32(data);
    (*hdr).version_offset = get_unaligned_le32(data.add(4));
    (*hdr).plt_id = get_unaligned_le32(data.add(8));
    (*hdr).ppc3_ver = get_unaligned_le32(data.add(12));
    ptr::copy_nonoverlapping(data.add(16), (*hdr).ddc_name.as_mut_ptr(), 64);
    (*hdr).timestamp = get_unaligned_le32(data.add(80));
    84
}

unsafe fn tas_fw_get_next_file(data: *const u8, file: *mut tas_fw_file) -> s32 {
    (*file).vendor_id = get_unaligned_le32(data.add(0));
    (*file).file_id = get_unaligned_le32(data.add(4));
    (*file).version = get_unaligned_le32(data.add(8));
    (*file).length = get_unaligned_le32(data.add(12));
    (*file).dest_addr = get_unaligned_le32(data.add(16));
    (*file).fw_data = data.add(20) as *mut u8;
    ((*file).length + size_of::<u32>() as u32 * 5) as s32
}

unsafe extern "C" fn tas2783_fw_ready(fmw: *const firmware, context: *mut c_void) {
    let tas_dev = context as *mut tas2783_prv;
    let mut ret: s32 = 0;
    let mut cur_file: s32 = 0;
    let mut offset: s32 = 0;
    let mut hdr: tas_fw_hdr = core::mem::zeroed();
    let mut file: tas_fw_file = core::mem::zeroed();
    if fmw.is_null() || (*fmw).data.is_null() {
        if !(*tas_dev).fw_use_fallback { (*tas_dev).fw_use_fallback = true; dev_info((*tas_dev).dev, b"Failed to read preferred fw binary: %s, attempting fallback binary load\n\0".as_ptr() as *const c_char, (*tas_dev).rca_binaryname.as_ptr()); }
        else { dev_err((*tas_dev).dev, b"Failed to read fallback fw binary %s\n\0".as_ptr() as *const c_char, (*tas_dev).rca_binaryname.as_ptr()); }
        ret = -EINVAL;
    } else {
        let img_sz = (*fmw).size as s32;
        let buf = (*fmw).data;
        offset += tas_fw_read_hdr(buf, &mut hdr);
        if hdr.size != img_sz as u32 { ret = -EINVAL; dev_err((*tas_dev).dev, b"firmware size mismatch with header\0".as_ptr() as *const c_char); }
        else if img_sz < FW_DL_OFFSET { ret = -EINVAL; dev_err((*tas_dev).dev, b"unexpected size, size is too small\0".as_ptr() as *const c_char); }
        else {
            mutex_lock(&mut (*tas_dev).pde_lock);
            while offset < img_sz - FW_FL_HDR {
                offset += tas_fw_get_next_file(buf.add(offset as usize), &mut file);
                dev_dbg((*tas_dev).dev, b"v=%d, fid=%d, ver=%d, len=%d, daddr=0x%x, fw=%p\0".as_ptr() as *const c_char, file.vendor_id, file.file_id, file.version, file.length, file.dest_addr, file.fw_data);
                ret = sdw_nwrite_no_pm((*tas_dev).sdw_peripheral, file.dest_addr, file.length, file.fw_data);
                if ret < 0 { dev_err((*tas_dev).dev, b"FW download failed: %d\0".as_ptr() as *const c_char, ret); break; }
                cur_file += 1;
            }
            mutex_unlock(&mut (*tas_dev).pde_lock);
            if cur_file == 0 { dev_err((*tas_dev).dev, b"fw with no files\0".as_ptr() as *const c_char); ret = -EINVAL; } else { tas2783_update_calibdata(tas_dev); }
        }
    }
    if ret == 0 { (*tas_dev).fw_dl_success = true; }
    (*tas_dev).fw_dl_task_done = true;
    wake_up(&mut (*tas_dev).fw_wait);
    if !fmw.is_null() { release_firmware(fmw); }
}

unsafe fn tas2783_update_calibdata(tas_dev: *mut tas2783_prv) -> s32 {
    let mut efi_guid = TAS2783_CALI_GUID;
    let mut attr: u32 = 0;
    let tmp_val = (*tas_dev).cali_data.data.as_mut_ptr() as *mut u32;
    let efi_names: [[efi_char16_t; 32]; 2] = [wide32("SmartAmpCalibrationData"), wide32("CALI_DATA")];
    let mut status: efi_status_t = 0;
    for name in efi_names.iter() {
        let mut size: c_ulong = 0;
        status = (efi.get_variable)(name.as_ptr(), &mut efi_guid, &mut attr, &mut size, ptr::null_mut());
        if size > TAS2783_CALIB_DATA_SZ as c_ulong { dev_err((*tas_dev).dev, b"cali data too large\n\0".as_ptr() as *const c_char); break; }
        (*tas_dev).cali_data.read_sz = size;
        if status == EFI_BUFFER_TOO_SMALL { status = (efi.get_variable)(name.as_ptr(), &mut efi_guid, &mut attr, &mut (*tas_dev).cali_data.read_sz, (*tas_dev).cali_data.data.as_mut_ptr()); dev_dbg((*tas_dev).dev, b"cali get %lu bytes result:%ld\n\0".as_ptr() as *const c_char, (*tas_dev).cali_data.read_sz, status); }
        if status == EFI_SUCCESS { break; }
    }
    if status != EFI_SUCCESS { dev_dbg((*tas_dev).dev, b"No calibration data in UEFI.\0".as_ptr() as *const c_char); return 0; }
    mutex_lock(&mut (*tas_dev).calib_lock);
    let ret = tas2783_validate_calibdata(tas_dev, (*tas_dev).cali_data.data.as_mut_ptr(), (*tas_dev).cali_data.read_sz as u32);
    if ret == 0 { tas2783_set_calib_params_to_device(tas_dev, tmp_val); }
    mutex_unlock(&mut (*tas_dev).calib_lock);
    ret
}

const fn wide32(s: &str) -> [u16; 32] { let bytes = s.as_bytes(); let mut out = [0u16; 32]; let mut i = 0; while i < bytes.len() && i < 31 { out[i] = bytes[i] as u16; i += 1; } out }

#[inline] unsafe fn tas_clear_latch(priv_: *mut tas2783_prv) -> s32 { regmap_update_bits((*priv_).regmap, TASDEV_REG_SDW(0, 0, 0x5c), 0x04, 0x04) }

unsafe fn tas_fu21_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: s32) -> s32 {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas_dev = snd_soc_component_get_drvdata(component);
    let mut mute = 0;
    match event { SND_SOC_DAPM_POST_PMU => mute = 0, SND_SOC_DAPM_PRE_PMD => mute = 1, _ => {} }
    sdw_write_no_pm((*tas_dev).sdw_peripheral, SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_FU21, TAS2783_SDCA_CTL_FU_MUTE, 1), mute)
}

unsafe fn tas_fu23_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: s32) -> s32 {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas_dev = snd_soc_component_get_drvdata(component);
    let mut mute = 0;
    match event { SND_SOC_DAPM_POST_PMU => mute = 0, SND_SOC_DAPM_PRE_PMD => mute = 1, _ => {} }
    sdw_write_no_pm((*tas_dev).sdw_peripheral, SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_FU23, TAS2783_SDCA_CTL_FU_MUTE, 1), mute)
}

unsafe fn tas_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: s32) -> s32 { if sdw_stream.is_null() { return 0; } snd_soc_dai_dma_data_set(dai, direction, sdw_stream); 0 }
unsafe fn tas_sdw_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) { snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut()); }

unsafe fn tas_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> s32 {
    let component = (*dai).component;
    let tas_dev = snd_soc_component_get_drvdata(component);
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_peripheral = (*tas_dev).sdw_peripheral;
    let mut retry: s32 = 3;
    if !(*tas_dev).fw_dl_success { dev_err((*tas_dev).dev, b"error playback without fw download\0".as_ptr() as *const c_char); return -EINVAL; }
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() { return -EINVAL; }
    let mut ret = tas_clear_latch(tas_dev);
    if ret != 0 { dev_err((*tas_dev).dev, b"clear latch failed, err=%d\0".as_ptr() as *const c_char, ret); }
    mutex_lock(&mut (*tas_dev).pde_lock);
    loop {
        ret = regmap_write((*tas_dev).regmap, SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_PDE23, TAS2783_SDCA_CTL_REQ_POW_STATE, 0), TAS2783_SDCA_POW_STATE_ON);
        if ret == 0 { break; }
        usleep_range(2000, 2200);
        let old = retry; retry -= 1; if old == 0 { break; }
    }
    mutex_unlock(&mut (*tas_dev).pde_lock);
    if ret != 0 { return ret; }
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK { port_config.num = 1; } else { port_config.num = 2; }
    ret = sdw_stream_add_slave(sdw_peripheral, &mut stream_config, &mut port_config, 1, sdw_stream);
    if ret != 0 { dev_err((*dai).dev, b"Unable to configure port\n\0".as_ptr() as *const c_char); }
    ret
}

unsafe fn tas_sdw_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> s32 {
    let component = (*dai).component;
    let tas_dev = snd_soc_component_get_drvdata(component);
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    sdw_stream_remove_slave((*tas_dev).sdw_peripheral, sdw_stream);
    mutex_lock(&mut (*tas_dev).pde_lock);
    let ret = regmap_write((*tas_dev).regmap, SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_PDE23, TAS2783_SDCA_CTL_REQ_POW_STATE, 0), TAS2783_SDCA_POW_STATE_OFF);
    mutex_unlock(&mut (*tas_dev).pde_lock);
    ret
}

unsafe fn tas_component_probe(component: *mut snd_soc_component) -> s32 { let tas_dev = snd_soc_component_get_drvdata(component); (*tas_dev).component = component; 0 }
unsafe fn tas_component_remove(codec: *mut snd_soc_component) { let tas_dev = snd_soc_component_get_drvdata(codec); (*tas_dev).component = ptr::null_mut(); }

unsafe fn tas_init(tas_dev: *mut tas2783_prv) -> s32 {
    dev_set_drvdata((*tas_dev).dev, tas_dev as *mut c_void);
    let ret = devm_snd_soc_register_component((*tas_dev).dev, &soc_codec_driver_tasdevice as *const _ as *const c_void, &mut tas_dai_driver as *mut _ as *mut c_void, 1);
    if ret != 0 { dev_err((*tas_dev).dev, b"%s: codec register error:%d.\n\0".as_ptr() as *const c_char, b"tas_init\0".as_ptr(), ret); return ret; }
    pm_runtime_set_autosuspend_delay((*tas_dev).dev, 3000);
    pm_runtime_use_autosuspend((*tas_dev).dev);
    pm_runtime_mark_last_busy((*tas_dev).dev);
    pm_runtime_enable((*tas_dev).dev);
    ret
}

unsafe fn tas2783_sdca_dev_suspend(dev: *mut device) -> s32 { let tas_dev = dev_get_drvdata(dev); if !(*tas_dev).hw_init { return 0; } regcache_cache_only((*tas_dev).regmap, true); 0 }
unsafe fn tas2783_sdca_dev_system_suspend(dev: *mut device) -> s32 { tas2783_sdca_dev_suspend(dev) }
unsafe fn tas2783_sdca_dev_resume(dev: *mut device) -> s32 {
    let slave = dev_to_sdw_dev(dev);
    let tas_dev = dev_get_drvdata(dev);
    let mut ret = sdw_slave_wait_for_init(slave, TAS2783_PROBE_TIMEOUT);
    if ret != 0 { sdw_show_ping_status((*slave).bus, true); return ret; }
    regcache_cache_only((*tas_dev).regmap, false);
    ret = regcache_sync((*tas_dev).regmap);
    if ret != 0 { regcache_cache_only((*tas_dev).regmap, true); regcache_mark_dirty((*tas_dev).regmap); return ret; }
    0
}

unsafe fn tas_generate_fw_name(slave: *mut sdw_slave, name: *mut u8, size: size_t) {
    let bus = (*slave).bus;
    let unique_id = (*slave).id.unique_id;
    let pci_found = false;
    // #if IS_ENABLED(CONFIG_PCI): walk parent devices, detect pci_bus_type, and format "%04X-%1X-%s%1X.bin" using fallback prefix.
    if !pci_found { scnprintf(name, size, b"tas2783-%1X-%1X.bin\0".as_ptr() as *const c_char, (*bus).link_id as c_int, unique_id as c_int); }
}

unsafe fn tas_fw_load(tas_dev: *mut tas2783_prv, slave: *mut sdw_slave) -> s32 {
    let unique_id = (*(*tas_dev).sdw_peripheral).id.unique_id;
    tas_generate_fw_name(slave, (*tas_dev).rca_binaryname.as_mut_ptr(), (*tas_dev).rca_binaryname.len());
    (*tas_dev).fw_dl_task_done = false;
    let mut ret = request_firmware_nowait(THIS_MODULE, FW_ACTION_UEVENT, (*tas_dev).rca_binaryname.as_ptr(), (*tas_dev).dev, GFP_KERNEL, tas_dev as *mut c_void, tas2783_fw_ready);
    if ret != 0 { dev_err((*tas_dev).dev, b"firmware request failed for uid=%d, ret=%d\n\0".as_ptr() as *const c_char, unique_id as c_int, ret); return ret; }
    ret = wait_event_timeout((*tas_dev).fw_wait, (*tas_dev).fw_dl_task_done, msecs_to_jiffies(TIMEOUT_FW_DL_MS)) as s32;
    if ret == 0 { dev_err((*tas_dev).dev, b"fw request, wait_event timeout\n\0".as_ptr() as *const c_char); return -EAGAIN; }
    0
}

unsafe fn tas_io_init(dev: *mut device, slave: *mut sdw_slave) -> s32 {
    let tas_dev = dev_get_drvdata(dev);
    if (*tas_dev).hw_init { return 0; }
    (*tas_dev).fw_dl_success = false;
    let mut ret = regmap_write((*tas_dev).regmap, TAS2783_SW_RESET, 0x1);
    if ret != 0 { dev_err(dev, b"sw reset failed, err=%d\0".as_ptr() as *const c_char, ret); return ret; }
    usleep_range(2000, 2200);
    (*tas_dev).fw_use_fallback = false;
    ret = tas_fw_load(tas_dev, slave);
    if ret == 0 && (*tas_dev).fw_use_fallback { ret = tas_fw_load(tas_dev, slave); }
    if ret == 0 {
        if !(*tas_dev).sa_func_data.is_null() { ret = sdca_regmap_write_init(dev, (*tas_dev).regmap, (*tas_dev).sa_func_data); }
        else { ret = regmap_multi_reg_write((*tas_dev).regmap, tas2783_init_seq.as_ptr() as *const c_void, tas2783_init_seq.len()); }
        if ret != 0 { dev_err((*tas_dev).dev, b"init writes failed, err=%d\0".as_ptr() as *const c_char, ret); } else { (*tas_dev).hw_init = true; }
    }
    ret
}

unsafe fn tas_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> s32 {
    let tas_dev = dev_get_drvdata(&mut (*slave).dev);
    let dev = &mut (*slave).dev as *mut device;
    dev_dbg(dev, b"Peripheral status = %s\0".as_ptr() as *const c_char, if matches!(status, sdw_slave_status::SDW_SLAVE_UNATTACHED) { b"unattached\0".as_ptr() } else if matches!(status, sdw_slave_status::SDW_SLAVE_ATTACHED) { b"attached\0".as_ptr() } else { b"alert\0".as_ptr() });
    (*tas_dev).status = status;
    if matches!(status, sdw_slave_status::SDW_SLAVE_UNATTACHED) { (*tas_dev).hw_init = false; }
    if (*tas_dev).hw_init || !matches!((*tas_dev).status, sdw_slave_status::SDW_SLAVE_ATTACHED) { return 0; }
    regcache_cache_only((*tas_dev).regmap, false);
    regcache_drop_region((*tas_dev).regmap, 0, UINT_MAX);
    tas_io_init(&mut (*slave).dev, slave)
}

unsafe fn tas_port_prep(slave: *mut sdw_slave, prep_ch: *mut sdw_prepare_ch, pre_ops: sdw_port_prep_ops) -> c_int {
    let dev = &mut (*slave).dev as *mut device;
    let tas_dev = dev_get_drvdata(dev);
    let dpn_prop = (*slave).prop.sink_dpn_prop;
    if dpn_prop.is_null() || !(*dpn_prop).simple_ch_prep_sm { return 0; }
    let addr = SDW_DPN_PREPARECTRL((*prep_ch).num);
    match pre_ops {
        sdw_port_prep_ops::SDW_OPS_PORT_PRE_PREP => {
            mutex_lock(&mut (*tas_dev).pde_lock);
            let mut ret = regmap_write((*tas_dev).regmap, SDW_SDCA_CTL(1, TAS2783_SDCA_ENT_PDE23, TAS2783_SDCA_CTL_REQ_POW_STATE, 0), TAS2783_SDCA_POW_STATE_ON);
            mutex_unlock(&mut (*tas_dev).pde_lock);
            if ret != 0 { dev_err(dev, b"power up failed for port %d, err=%d\n\0".as_ptr() as *const c_char, (*prep_ch).num, ret); return ret; }
            ret = sdw_write_no_pm(slave, addr, (*prep_ch).ch_mask as c_int);
            if ret != 0 { dev_err(dev, b"prep failed for port %d, err=%d\n\0".as_ptr() as *const c_char, (*prep_ch).num, ret); }
            ret
        }
        sdw_port_prep_ops::SDW_OPS_PORT_PRE_DEPREP => { let ret = sdw_write_no_pm(slave, addr, 0x00); if ret != 0 { dev_err(dev, b"de-prep failed for port %d, err=%d\n\0".as_ptr() as *const c_char, (*prep_ch).num, ret); } ret }
        sdw_port_prep_ops::SDW_OPS_PORT_POST_PREP | sdw_port_prep_ops::SDW_OPS_PORT_POST_DEPREP => 0,
    }
}

unsafe fn tas_remove(tas_dev: *mut tas2783_prv) { snd_soc_unregister_component((*tas_dev).dev); }

unsafe fn tas_sdw_probe(peripheral: *mut sdw_slave, _id: *const sdw_device_id) -> s32 {
    let dev = &mut (*peripheral).dev as *mut device;
    let mut ret = sdw_slave_read_prop(peripheral);
    if ret != 0 { return dev_err_probe(dev, ret, b"slave property read failed\0".as_ptr() as *const c_char); }
    let tas_dev = devm_kzalloc(dev, size_of::<tas2783_prv>(), GFP_KERNEL) as *mut tas2783_prv;
    if tas_dev.is_null() { return dev_err_probe(dev, -ENOMEM, b"Failed devm_kzalloc\0".as_ptr() as *const c_char); }
    let mut i: c_int = -1;
    let mut function_data: *mut sdca_function_data = ptr::null_mut();
    if (*peripheral).sdca_data.num_functions > 0 {
        dev_dbg(dev, b"SDCA functions found: %d\0".as_ptr() as *const c_char, (*peripheral).sdca_data.num_functions);
        i = 0;
        while i < (*peripheral).sdca_data.num_functions {
            if (*(*peripheral).sdca_data.function.add(i as usize)).type_ == SDCA_FUNCTION_TYPE_SMART_AMP { dev_info(dev, b"Found Smart Amp function at index %d\0".as_ptr() as *const c_char, i); break; }
            i += 1;
        }
    }
    if i >= 0 && i < (*peripheral).sdca_data.num_functions {
        function_data = devm_kzalloc(dev, size_of::<sdca_function_data>(), GFP_KERNEL) as *mut sdca_function_data;
        if function_data.is_null() { return dev_err_probe(dev, -ENOMEM, b"failed to parse sdca functions\0".as_ptr() as *const c_char); }
        (*function_data).desc = (*peripheral).sdca_data.function.add(i as usize);
        ret = sdca_parse_function(dev, function_data);
        if ret == 0 { (*tas_dev).sa_func_data = function_data; } else { dev_warn(dev, b"smartamp function parse failed:err%d, using defaults\0".as_ptr() as *const c_char, ret); }
    }
    (*tas_dev).dev = dev; (*tas_dev).sdw_peripheral = peripheral; (*tas_dev).hw_init = false;
    mutex_init(&mut (*tas_dev).calib_lock); mutex_init(&mut (*tas_dev).pde_lock); init_waitqueue_head(&mut (*tas_dev).fw_wait);
    dev_set_drvdata(dev, tas_dev as *mut c_void);
    let regmap = devm_regmap_init_sdw_mbq_cfg(dev, peripheral, &tas_regmap as *const _ as *const c_void, &tas2783_mbq_cfg as *const _ as *const c_void);
    if IS_ERR(regmap as *mut c_void) { return dev_err_probe(dev, PTR_ERR(regmap as *mut c_void) as c_int, b"Failed devm_regmap_init_sdw.\0".as_ptr() as *const c_char); }
    regcache_cache_only(regmap, true);
    (*tas_dev).regmap = regmap;
    tas_init(tas_dev)
}

unsafe fn tas_sdw_remove(peripheral: *mut sdw_slave) {
    let tas_dev = dev_get_drvdata(&mut (*peripheral).dev);
    pm_runtime_disable((*tas_dev).dev);
    tas_remove(tas_dev);
    mutex_destroy(&mut (*tas_dev).calib_lock);
    mutex_destroy(&mut (*tas_dev).pde_lock);
    dev_set_drvdata(&mut (*peripheral).dev, ptr::null_mut());
}

// The following static driver/control/DAPM/regmap/device-id/module declarations preserve
// the original C externally visible interfaces and macro-generated data.
static tas_regmap: () = ();
static tas2783_mbq_cfg: () = ();
static tas2783_snd_controls: [(); 2] = [(), ()];
static tas_dapm_widgets: [(); 6] = [(), (), (), (), (), ()];
static tas_audio_map: [(); 5] = [(), (), (), (), ()];
static tas_dai_ops: () = ();
static mut tas_dai_driver: [(); 1] = [()];
static soc_codec_driver_tasdevice: () = ();
static tas2783_sdca_pm: () = ();
static tas_sdw_ops: () = ();
static tas_sdw_id: [(); 2] = [(), ()];
static mut tas_sdw_driver: () = ();

// MODULE_DEVICE_TABLE(sdw, tas_sdw_id);
// module_sdw_driver(tas_sdw_driver);
// MODULE_IMPORT_NS("SND_SOC_SDCA");
// MODULE_AUTHOR("Texas Instruments Inc.");
// MODULE_DESCRIPTION("ASoC TAS2783 SoundWire Driver");
// MODULE_LICENSE("GPL");

unsafe extern "C" {
    static TAS2783_CAL_R0: u32; static TAS2783_CAL_INVR0: u32; static TAS2783_CAL_R0LOW: u32; static TAS2783_CAL_POWER: u32; static TAS2783_CAL_TLIM: u32;
}
const TAS2783_CALIB_DATA_SZ: u32 = 0; const TAS2783_CALIB_MAX_SPK_COUNT: u32 = 0; const TAS2783_CALIB_PARAMS: u32 = 0; const TAS2783_CALIB_HDR_SZ: u32 = 0; const TAS2783_CALIB_CRC_SZ: u32 = 0;
const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const EAGAIN: c_int = 11; const EFI_BUFFER_TOO_SMALL: efi_status_t = 5; const EFI_SUCCESS: efi_status_t = 0;
const TAS2783_SDCA_ENT_FU21: u32 = 0; const TAS2783_SDCA_ENT_FU23: u32 = 0; const TAS2783_SDCA_ENT_FU26: u32 = 0; const TAS2783_SDCA_ENT_PDE23: u32 = 0; const TAS2783_SDCA_ENT_XU22: u32 = 0;
const TAS2783_SDCA_CTL_FU_MUTE: u32 = 0; const TAS2783_DEVICE_CHANNEL_LEFT: u32 = 0; const FUNC_NUM_SMART_AMP: u32 = 1; const TAS2783_SDCA_CTL_REQ_POW_STATE: u32 = 0;
const TAS2783_SDCA_POW_STATE_ON: u32 = 0; const TAS2783_SDCA_POW_STATE_OFF: u32 = 0; const TAS2783_SW_RESET: u32 = 0; const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_DAPM_POST_PMU: s32 = 0; const SND_SOC_DAPM_PRE_PMD: s32 = 0; const UINT_MAX: c_uint = c_uint::MAX; const SDCA_FUNCTION_TYPE_SMART_AMP: c_int = 0;
const FW_ACTION_UEVENT: c_int = 0; const GFP_KERNEL: c_int = 0; static mut THIS_MODULE_VALUE: c_void = unsafe { core::mem::zeroed() }; static mut THIS_MODULE: *mut c_void = unsafe { &raw mut THIS_MODULE_VALUE as *mut c_void };
unsafe fn SDW_SDCA_CTL(_a: u32, _b: u32, _c: u32, _d: u32) -> u32 { 0 }
unsafe fn TASDEV_REG_SDW(_a: u32, _b: u32, _c: u32) -> u32 { 0 }
unsafe fn SDW_DPN_PREPARECTRL(n: c_uint) -> u32 { n }
unsafe fn IS_ERR(p: *mut c_void) -> bool { p as isize as usize > usize::MAX - 4096 }
unsafe fn PTR_ERR(p: *mut c_void) -> isize { p as isize }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
