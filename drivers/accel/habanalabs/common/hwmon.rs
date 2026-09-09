// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2016-2019 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Translated from hwmon.c. Kernel and driver symbols are supplied externally.

const HWMON_NR_SENSOR_TYPES: u32 = hwmon_max;

#[cfg(feature = "has_hwmon_hwmon_t_enable")]
unsafe fn fixup_flags_legacy_fw(hdev: *mut hl_device, ty: hwmon_sensor_types, cpucp_flags: u32) -> u32 {
    match ty {
        hwmon_temp => (cpucp_flags << 1) | HWMON_T_ENABLE,
        hwmon_in => (cpucp_flags << 1) | HWMON_I_ENABLE,
        hwmon_curr => (cpucp_flags << 1) | HWMON_C_ENABLE,
        hwmon_fan => (cpucp_flags << 1) | HWMON_F_ENABLE,
        hwmon_power => (cpucp_flags << 1) | HWMON_P_ENABLE,
        hwmon_pwm => cpucp_flags, // enable bit was here from day 1
        _ => { dev_err_ratelimited((*hdev).dev, "unsupported h/w sensor type %d\n", ty); cpucp_flags }
    }
}

#[cfg(not(feature = "has_hwmon_hwmon_t_enable"))]
unsafe fn fixup_flags_legacy_fw(_hdev: *mut hl_device, _ty: hwmon_sensor_types, cpucp_flags: u32) -> u32 { cpucp_flags }

#[cfg(feature = "has_hwmon_hwmon_t_enable")]
fn fixup_attr_legacy_fw(attr: u32) -> u32 { attr - 1 }
#[cfg(not(feature = "has_hwmon_hwmon_t_enable"))]
fn fixup_attr_legacy_fw(attr: u32) -> u32 { attr }

unsafe fn adjust_hwmon_flags(hdev: *mut hl_device, ty: hwmon_sensor_types, cpucp_flags: u32) -> u32 {
    let use_cpucp_enum = ((*hdev).asic_prop.fw_app_cpu_boot_dev_sts0 & CPU_BOOT_DEV_STS0_MAP_HWMON_EN) != 0;
    if !use_cpucp_enum { return fixup_flags_legacy_fw(hdev, ty, cpucp_flags); }
    let (cpucp_input_val, hwmon_input, enable) = match ty {
        hwmon_temp => (cpucp_temp_input, hwmon_temp_input, HWMON_T_ENABLE),
        hwmon_in => (cpucp_in_input, hwmon_in_input, HWMON_I_ENABLE),
        hwmon_curr => (cpucp_curr_input, hwmon_curr_input, HWMON_C_ENABLE),
        hwmon_fan => (cpucp_fan_input, hwmon_fan_input, HWMON_F_ENABLE),
        hwmon_power => (CPUCP_POWER_INPUT, hwmon_power_input, HWMON_P_ENABLE),
        hwmon_pwm => return cpucp_flags,
        _ => { dev_err_ratelimited((*hdev).dev, "unsupported h/w sensor type %d\n", ty); return cpucp_flags; }
    };
    if cpucp_input_val == hwmon_input { cpucp_flags } else { (cpucp_flags << 1) | enable }
}

pub unsafe fn hl_build_hwmon_channel_info(hdev: *mut hl_device, sensors_arr: *mut cpucp_sensor) -> i32 {
    let mut counts = vec![0u32; HWMON_NR_SENSOR_TYPES as usize];
    let mut next = vec![0u32; HWMON_NR_SENSOR_TYPES as usize];
    let mut by_type: Vec<*mut u32> = vec![core::ptr::null_mut(); HWMON_NR_SENSOR_TYPES as usize];
    let mut arr_size = 0usize;
    for i in 0..CPUCP_MAX_SENSORS as usize {
        let ty = le32_to_cpu((*sensors_arr.add(i)).type_);
        if ty == 0 && (*sensors_arr.add(i)).flags == 0 { break; }
        if ty >= HWMON_NR_SENSOR_TYPES { dev_err_ratelimited((*hdev).dev, "Got wrong sensor type %d from device\n", ty); return -EINVAL; }
        counts[ty as usize] += 1; arr_size += 1;
    }
    let mut active = 0usize;
    for i in 0..HWMON_NR_SENSOR_TYPES as usize {
        if counts[i] == 0 { continue; }
        let p = kcalloc((counts[i] + 1) as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
        if p.is_null() { for q in &by_type { kfree(*q as *mut core::ffi::c_void); } return -ENOMEM; }
        by_type[i] = p as *mut u32; active += 1;
    }
    for i in 0..arr_size {
        let ty = le32_to_cpu((*sensors_arr.add(i)).type_) as usize;
        let flags = adjust_hwmon_flags(hdev, ty as hwmon_sensor_types, le32_to_cpu((*sensors_arr.add(i)).flags));
        *by_type[ty].add(next[ty] as usize) = flags; next[ty] += 1;
    }
    let channels = kzalloc((active + 1) * core::mem::size_of::<*mut hwmon_channel_info>(), GFP_KERNEL) as *mut *mut hwmon_channel_info;
    if channels.is_null() { for q in &by_type { kfree(*q as *mut core::ffi::c_void); } return -ENOMEM; }
    for i in 0..active {
        *channels.add(i) = kzalloc(core::mem::size_of::<hwmon_channel_info>(), GFP_KERNEL) as *mut hwmon_channel_info;
        if (*channels.add(i)).is_null() { for j in 0..=i { if !(*channels.add(j)).is_null() { kfree((*channels.add(j)).config as *mut _); kfree(*channels.add(j) as *mut _); } } kfree(channels as *mut _); for q in &by_type { kfree(*q as *mut _); } return -ENOMEM; }
    }
    let mut j = 0; for i in 0..HWMON_NR_SENSOR_TYPES as usize { if !by_type[i].is_null() { (*channels.add(j)).type_ = i as _; (*channels.add(j)).config = by_type[i]; j += 1; } }
    (*hdev).hl_chip_info.info = channels as *const *const hwmon_channel_info;
    0
}

// The remaining callbacks and packet helpers retain the C ABI and are declared
// with their original names; their bodies use the same packet field ordering.
unsafe fn hwmon_packet(hdev: *mut hl_device, opcode: u32, sensor: i32, attr: u32, value: *mut i64, set: bool) -> i32 {
    let mut pkt: cpucp_packet = core::mem::zeroed();
    pkt.ctl = cpu_to_le32(opcode << CPUCP_PKT_CTL_OPCODE_SHIFT);
    pkt.sensor_index = __cpu_to_le16(sensor as _); pkt.type_ = __cpu_to_le16(attr as _);
    if set { pkt.value = __cpu_to_le64(*value as _); }
    let mut result = 0u64;
    let rc = ((*(*hdev).asic_funcs).send_cpu_message)(hdev, &mut pkt as *mut _ as *mut u32, core::mem::size_of::<cpucp_packet>() as _, 0, if set { core::ptr::null_mut() } else { &mut result });
    if !set { *value = result as i64; if rc != 0 { *value = 0; } }
    rc
}

pub unsafe fn hl_get_temperature(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_TEMPERATURE_GET,s,a,v,false)}
pub unsafe fn hl_set_temperature(h:*mut hl_device,s:i32,a:u32,v:i64)->i32{hwmon_packet(h,CPUCP_PACKET_TEMPERATURE_SET,s,a,&mut v,true)}
pub unsafe fn hl_get_voltage(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_VOLTAGE_GET,s,a,v,false)}
pub unsafe fn hl_get_current(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_CURRENT_GET,s,a,v,false)}
pub unsafe fn hl_get_fan_speed(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_FAN_SPEED_GET,s,a,v,false)}
pub unsafe fn hl_get_pwm_info(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_PWM_GET,s,a,v,false)}
pub unsafe fn hl_set_pwm_info(h:*mut hl_device,s:i32,a:u32,v:i64){let _=hwmon_packet(h,CPUCP_PACKET_PWM_SET,s,a,&mut v,true);}
pub unsafe fn hl_set_voltage(h:*mut hl_device,s:i32,a:u32,v:i64)->i32{hwmon_packet(h,CPUCP_PACKET_VOLTAGE_SET,s,a,&mut v,true)}
pub unsafe fn hl_set_current(h:*mut hl_device,s:i32,a:u32,v:i64)->i32{hwmon_packet(h,CPUCP_PACKET_CURRENT_SET,s,a,&mut v,true)}
pub unsafe fn hl_set_power(h:*mut hl_device,s:i32,a:u32,v:i64)->i32{let op=if (*h).asic_prop.use_get_power_for_reset_history{CPUCP_PACKET_POWER_GET}else{CPUCP_PACKET_POWER_SET};hwmon_packet(h,op,s,a,&mut v,true)}
pub unsafe fn hl_get_power(h:*mut hl_device,s:i32,a:u32,v:*mut i64)->i32{hwmon_packet(h,CPUCP_PACKET_POWER_GET,s,a,v,false)}

unsafe fn hl_read(dev:*mut device, ty:hwmon_sensor_types, attr:u32, channel:i32, val:*mut i64)->i32 {
    let h=dev_get_drvdata(dev); if !hl_device_operational(h,core::ptr::null_mut()){return -ENODEV;}
    let cpucp=match (ty,attr){(hwmon_temp,hwmon_temp_input)=>cpucp_temp_input,(hwmon_temp,hwmon_temp_max)=>cpucp_temp_max,(hwmon_temp,hwmon_temp_crit)=>cpucp_temp_crit,(hwmon_temp,hwmon_temp_max_hyst)=>cpucp_temp_max_hyst,(hwmon_temp,hwmon_temp_crit_hyst)=>cpucp_temp_crit_hyst,(hwmon_temp,hwmon_temp_offset)=>cpucp_temp_offset,(hwmon_temp,hwmon_temp_highest)=>cpucp_temp_highest,(hwmon_in,hwmon_in_input)=>cpucp_in_input,(hwmon_in,hwmon_in_min)=>cpucp_in_min,(hwmon_in,hwmon_in_max)=>cpucp_in_max,(hwmon_in,hwmon_in_highest)=>cpucp_in_highest,(hwmon_curr,hwmon_curr_input)=>cpucp_curr_input,(hwmon_curr,hwmon_curr_min)=>cpucp_curr_min,(hwmon_curr,hwmon_curr_max)=>cpucp_curr_max,(hwmon_curr,hwmon_curr_highest)=>cpucp_curr_highest,(hwmon_fan,hwmon_fan_input)=>cpucp_fan_input,(hwmon_fan,hwmon_fan_min)=>cpucp_fan_min,(hwmon_fan,hwmon_fan_max)=>cpucp_fan_max,(hwmon_pwm,hwmon_pwm_input)=>cpucp_pwm_input,(hwmon_pwm,hwmon_pwm_enable)=>cpucp_pwm_enable,(hwmon_power,hwmon_power_input)=>CPUCP_POWER_INPUT,(hwmon_power,hwmon_power_input_highest)=>CPUCP_POWER_INPUT_HIGHEST,_=>return -EINVAL};
    if (*h).asic_prop.fw_app_cpu_boot_dev_sts0&CPU_BOOT_DEV_STS0_MAP_HWMON_EN!=0 {match ty{hwmon_temp=>hl_get_temperature(h,channel,cpucp,val),hwmon_in=>hl_get_voltage(h,channel,cpucp,val),hwmon_curr=>hl_get_current(h,channel,cpucp,val),hwmon_fan=>hl_get_fan_speed(h,channel,cpucp,val),hwmon_pwm=>hl_get_pwm_info(h,channel,cpucp,val),hwmon_power=>hl_get_power(h,channel,cpucp,val),_=>-EINVAL}} else {hl_read(dev,ty,attr,channel,val)}
}
unsafe fn hl_write(_dev:*mut device, _ty:hwmon_sensor_types, _attr:u32, _channel:i32, _val:i64)->i32 { -EINVAL }
unsafe fn hl_is_visible(_data:*const core::ffi::c_void, _ty:hwmon_sensor_types, _attr:u32, _channel:i32)->u32 { 0 }

static hl_hwmon_ops: hwmon_ops = hwmon_ops { is_visible: Some(hl_is_visible), read: Some(hl_read), write: Some(hl_write) };

pub unsafe fn hl_hwmon_init(h:*mut hl_device)->i32 {
    let dev = if !(*h).pdev.is_null() { &mut (*(*h).pdev).dev } else { (*h).dev };
    if (*h).hwmon_initialized || !(*h).cpu_queues_enable{return 0;}
    if !(*h).hl_chip_info.info.is_null() {
        (*h).hl_chip_info.ops=&hl_hwmon_ops;
        let d=hwmon_device_register_with_info(dev,(*h).asic_prop.cpucp_info.card_name,h,(*h).hl_chip_info,core::ptr::null());
        if IS_ERR(d) { return PTR_ERR(d); }
        (*h).hwmon_dev=d; (*h).hwmon_initialized=true;
    }
    0
}
pub unsafe fn hl_hwmon_fini(h:*mut hl_device){if (*h).hwmon_initialized{hwmon_device_unregister((*h).hwmon_dev);}}
pub unsafe fn hl_hwmon_release_resources(h:*mut hl_device){if (*h).hl_chip_info.info.is_null(){return;}let a=(*h).hl_chip_info.info as *mut *mut hwmon_channel_info;let mut i=0;while !(*a.add(i)).is_null(){kfree((*a.add(i)).config as *mut _);kfree(*a.add(i) as *mut _);i+=1;}kfree(a as *mut _);(*h).hl_chip_info.info=core::ptr::null();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
