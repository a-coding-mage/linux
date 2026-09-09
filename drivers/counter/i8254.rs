// SPDX-License-Identifier: GPL-2.0
/*
 * Intel 8254 Programmable Interval Timer
 * Copyright (C) William Breathitt Gray
 */
// Linux dependencies are supplied by the surrounding translation unit.

const I8254_CONTROL_REG: u8 = 0x3;
const I8254_SC: u8 = 0xc0;
const I8254_RW: u8 = 0x30;
const I8254_M: u8 = 0x0e;
const I8254_RW_TWO_BYTE: u8 = 0x3;
const I8254_MODE_INTERRUPT_ON_TERMINAL_COUNT: u8 = 0;
const I8254_MODE_HARDWARE_RETRIGGERABLE_ONESHOT: u8 = 1;
const I8254_MODE_RATE_GENERATOR: u8 = 2;
const I8254_MODE_SQUARE_WAVE_MODE: u8 = 3;
const I8254_MODE_SOFTWARE_TRIGGERED_STROBE: u8 = 4;
const I8254_MODE_HARDWARE_TRIGGERED_STROBE: u8 = 5;
const I8254_NUM_COUNTERS: usize = 3;

const fn i8254_control(sc: u8, rw: u8, m: u8) -> u8 {
    ((sc << 6) & I8254_SC) | ((rw << 4) & I8254_RW) | ((m << 1) & I8254_M)
}

const fn i8254_counter_latch(counter: u8) -> u8 { i8254_control(counter, 0, 0) }
const fn i8254_program_counter(counter: u8, mode: u8) -> u8 {
    i8254_control(counter, I8254_RW_TWO_BYTE, mode)
}

#[repr(C)]
struct I8254 {
    lock: mutex,
    preset: [u16; I8254_NUM_COUNTERS],
    out_mode: [u8; I8254_NUM_COUNTERS],
    map: *mut regmap,
}

unsafe fn i8254_count_read(counter: *mut counter_device, count: *mut counter_count, val: *mut u64) -> c_int {
    let priv_ = counter_priv(counter);
    let mut value = [0u8; 2];
    mutex_lock(&mut (*priv_).lock);
    let mut ret = regmap_write((*priv_).map, I8254_CONTROL_REG, i8254_counter_latch((*count).id as u8));
    if ret != 0 { mutex_unlock(&mut (*priv_).lock); return ret; }
    ret = regmap_noinc_read((*priv_).map, (*count).id as u8, value.as_mut_ptr(), value.len());
    if ret != 0 { mutex_unlock(&mut (*priv_).lock); return ret; }
    mutex_unlock(&mut (*priv_).lock);
    *val = u16::from_le_bytes(value) as u64;
    ret
}

unsafe fn i8254_function_read(_: *mut counter_device, _: *mut counter_count, function: *mut counter_function) -> c_int {
    *function = COUNTER_FUNCTION_DECREASE; 0
}

const I8254_SYNAPSES_PER_COUNT: usize = 2;
const I8254_SIGNAL_ID_CLK: usize = 0;
const I8254_SIGNAL_ID_GATE: usize = 1;

unsafe fn i8254_action_read(counter: *mut counter_device, count: *mut counter_count, synapse: *mut counter_synapse, action: *mut counter_synapse_action) -> c_int {
    let priv_ = counter_priv(counter);
    match ((*(*synapse).signal).id as usize) % I8254_SYNAPSES_PER_COUNT {
        I8254_SIGNAL_ID_CLK => { *action = COUNTER_SYNAPSE_ACTION_FALLING_EDGE; 0 }
        I8254_SIGNAL_ID_GATE => {
            match (*priv_).out_mode[(*count).id as usize] {
                I8254_MODE_HARDWARE_RETRIGGERABLE_ONESHOT | I8254_MODE_RATE_GENERATOR | I8254_MODE_SQUARE_WAVE_MODE | I8254_MODE_HARDWARE_TRIGGERED_STROBE => { *action = COUNTER_SYNAPSE_ACTION_RISING_EDGE; 0 }
                _ => { *action = COUNTER_SYNAPSE_ACTION_NONE; 0 }
            }
        }
        _ => -EINVAL,
    }
}

unsafe fn i8254_count_ceiling_read(counter: *mut counter_device, count: *mut counter_count, ceiling: *mut u64) -> c_int {
    let priv_ = counter_priv(counter); let id = (*count).id as usize;
    mutex_lock(&mut (*priv_).lock);
    *ceiling = match (*priv_).out_mode[id] {
        I8254_MODE_RATE_GENERATOR => if (*priv_).preset[id] == 0 { u16::MAX } else { (*priv_).preset[id] },
        I8254_MODE_SQUARE_WAVE_MODE => if (*priv_).preset[id] % 2 != 0 { (*priv_).preset[id] - 1 } else if (*priv_).preset[id] == 0 { u16::MAX - 1 } else { (*priv_).preset[id] },
        _ => u16::MAX,
    } as u64;
    mutex_unlock(&mut (*priv_).lock); 0
}

unsafe fn i8254_count_mode_read(counter: *mut counter_device, count: *mut counter_count, mode: *mut counter_count_mode) -> c_int {
    let priv_ = counter_priv(counter);
    *mode = match (*priv_).out_mode[(*count).id as usize] {
        I8254_MODE_INTERRUPT_ON_TERMINAL_COUNT => COUNTER_COUNT_MODE_INTERRUPT_ON_TERMINAL_COUNT,
        I8254_MODE_HARDWARE_RETRIGGERABLE_ONESHOT => COUNTER_COUNT_MODE_HARDWARE_RETRIGGERABLE_ONESHOT,
        I8254_MODE_RATE_GENERATOR => COUNTER_COUNT_MODE_RATE_GENERATOR,
        I8254_MODE_SQUARE_WAVE_MODE => COUNTER_COUNT_MODE_SQUARE_WAVE_MODE,
        I8254_MODE_SOFTWARE_TRIGGERED_STROBE => COUNTER_COUNT_MODE_SOFTWARE_TRIGGERED_STROBE,
        I8254_MODE_HARDWARE_TRIGGERED_STROBE => COUNTER_COUNT_MODE_HARDWARE_TRIGGERED_STROBE,
        _ => return -EINVAL,
    }; 0
}

unsafe fn i8254_count_mode_write(counter: *mut counter_device, count: *mut counter_count, count_mode: counter_count_mode) -> c_int {
    let priv_ = counter_priv(counter);
    let out_mode = match count_mode {
        COUNTER_COUNT_MODE_INTERRUPT_ON_TERMINAL_COUNT => I8254_MODE_INTERRUPT_ON_TERMINAL_COUNT,
        COUNTER_COUNT_MODE_HARDWARE_RETRIGGERABLE_ONESHOT => I8254_MODE_HARDWARE_RETRIGGERABLE_ONESHOT,
        COUNTER_COUNT_MODE_RATE_GENERATOR => I8254_MODE_RATE_GENERATOR,
        COUNTER_COUNT_MODE_SQUARE_WAVE_MODE => I8254_MODE_SQUARE_WAVE_MODE,
        COUNTER_COUNT_MODE_SOFTWARE_TRIGGERED_STROBE => I8254_MODE_SOFTWARE_TRIGGERED_STROBE,
        COUNTER_COUNT_MODE_HARDWARE_TRIGGERED_STROBE => I8254_MODE_HARDWARE_TRIGGERED_STROBE,
        _ => return -EINVAL,
    };
    let id = (*count).id as usize;
    mutex_lock(&mut (*priv_).lock); (*priv_).preset[id] = 0; (*priv_).out_mode[id] = out_mode;
    let ret = regmap_write((*priv_).map, I8254_CONTROL_REG, i8254_program_counter((*count).id as u8, out_mode));
    mutex_unlock(&mut (*priv_).lock); ret
}

unsafe fn i8254_count_floor_read(counter: *mut counter_device, count: *mut counter_count, floor: *mut u64) -> c_int {
    let priv_ = counter_priv(counter); let id = (*count).id as usize;
    mutex_lock(&mut (*priv_).lock);
    *floor = match (*priv_).out_mode[id] {
        I8254_MODE_RATE_GENERATOR => if (*priv_).preset[id] == 0 { 0 } else { 1 },
        I8254_MODE_SQUARE_WAVE_MODE => if (*priv_).preset[id] % 2 != 0 || (*priv_).preset[id] == 0 { 0 } else { 2 },
        _ => 0,
    };
    mutex_unlock(&mut (*priv_).lock); 0
}

unsafe fn i8254_count_preset_read(counter: *mut counter_device, count: *mut counter_count, preset: *mut u64) -> c_int {
    let priv_ = counter_priv(counter); *preset = (*priv_).preset[(*count).id as usize] as u64; 0
}

unsafe fn i8254_count_preset_write(counter: *mut counter_device, count: *mut counter_count, preset: u64) -> c_int {
    let priv_ = counter_priv(counter); if preset > u16::MAX as u64 { return -ERANGE; }
    let id = (*count).id as usize; mutex_lock(&mut (*priv_).lock);
    if ((*priv_).out_mode[id] == I8254_MODE_RATE_GENERATOR || (*priv_).out_mode[id] == I8254_MODE_SQUARE_WAVE_MODE) && preset == 1 { mutex_unlock(&mut (*priv_).lock); return -EINVAL; }
    (*priv_).preset[id] = preset as u16; let value = (preset as u16).to_le_bytes();
    let ret = regmap_noinc_write((*priv_).map, (*count).id as u8, value.as_ptr(), 2);
    mutex_unlock(&mut (*priv_).lock); ret
}

unsafe fn i8254_init_hw(map: *mut regmap) -> c_int {
    for i in 0..I8254_NUM_COUNTERS { let ret = regmap_write(map, I8254_CONTROL_REG, i8254_program_counter(i as u8, I8254_MODE_INTERRUPT_ON_TERMINAL_COUNT)); if ret != 0 { return ret; } }
    0
}

static I8254_OPS: counter_ops = counter_ops { count_read: Some(i8254_count_read), function_read: Some(i8254_function_read), action_read: Some(i8254_action_read) };

static mut I8254_SIGNALS: [counter_signal; 6] = [
    counter_signal { id: 0, name: "CLK 0" }, counter_signal { id: 1, name: "GATE 0" },
    counter_signal { id: 2, name: "CLK 1" }, counter_signal { id: 3, name: "GATE 1" },
    counter_signal { id: 4, name: "CLK 2" }, counter_signal { id: 5, name: "GATE 2" },
];
static I8254_CLK_ACTIONS: [counter_synapse_action; 1] = [COUNTER_SYNAPSE_ACTION_FALLING_EDGE];
static I8254_GATE_ACTIONS: [counter_synapse_action; 2] = [COUNTER_SYNAPSE_ACTION_NONE, COUNTER_SYNAPSE_ACTION_RISING_EDGE];

static I8254_FUNCTIONS_LIST: [counter_function; 1] = [COUNTER_FUNCTION_DECREASE];
static I8254_COUNT_MODES: [counter_count_mode; 6] = [
    COUNTER_COUNT_MODE_INTERRUPT_ON_TERMINAL_COUNT, COUNTER_COUNT_MODE_HARDWARE_RETRIGGERABLE_ONESHOT,
    COUNTER_COUNT_MODE_RATE_GENERATOR, COUNTER_COUNT_MODE_SQUARE_WAVE_MODE,
    COUNTER_COUNT_MODE_SOFTWARE_TRIGGERED_STROBE, COUNTER_COUNT_MODE_HARDWARE_TRIGGERED_STROBE,
];
static I8254_COUNT_MODES_AVAILABLE: counter_available = counter_available { list: I8254_COUNT_MODES.as_ptr(), num: I8254_COUNT_MODES.len() };

// C macro-generated synapse/count tables; pointers and array sizes preserve the original layout.
static mut I8254_SYNAPSES: [counter_synapse; 6] = [counter_synapse::ZERO; 6];
static mut I8254_COUNT_EXT: [counter_comp; 4] = [counter_comp::ZERO; 4];
static mut I8254_COUNTS: [counter_count; I8254_NUM_COUNTERS] = [counter_count::ZERO; I8254_NUM_COUNTERS];

pub unsafe fn devm_i8254_regmap_register(dev: *const device, config: *const i8254_regmap_config) -> c_int {
    if (*config).parent.is_null() || (*config).map.is_null() { return -EINVAL; }
    let counter = devm_counter_alloc(dev, core::mem::size_of::<I8254>());
    if counter.is_null() { return -ENOMEM; }
    let priv_ = counter_priv(counter); (*priv_).map = (*config).map;
    (*counter).name = dev_name((*config).parent); (*counter).parent = (*config).parent;
    (*counter).ops = &I8254_OPS; (*counter).counts = I8254_COUNTS.as_mut_ptr(); (*counter).num_counts = I8254_COUNTS.len();
    (*counter).signals = I8254_SIGNALS.as_mut_ptr(); (*counter).num_signals = I8254_SIGNALS.len();
    mutex_init(&mut (*priv_).lock);
    let err = i8254_init_hw((*priv_).map); if err != 0 { return err; }
    let err = devm_counter_add(dev, counter); if err < 0 { return dev_err_probe(dev, err, "Failed to add counter\n"); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
