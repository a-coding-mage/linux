// SPDX-License-Identifier: GPL-2.0
/* Rust translation of the ACCES 104-QUAD-8 counter driver. */

// Linux kernel interfaces referenced by this translation are supplied by the
// surrounding kernel/Rust bindings.
use core::mem::MaybeUninit;

const QUAD8_EXTENT: usize = 32;
const QUAD8_NUM_COUNTERS: usize = 8;
const QUAD8_INTERRUPT_STATUS: u8 = 0x10;
const QUAD8_CHANNEL_OPERATION: u8 = 0x11;
const QUAD8_INDEX_INTERRUPT: u8 = 0x12;
const QUAD8_INDEX_INPUT_LEVELS: u8 = 0x16;
const QUAD8_CABLE_STATUS: u8 = 0x17;
const LS7267_CNTR_MAX: u32 = 0x00ff_ffff;

const fn quad8_data(channel: usize) -> u8 { (channel as u8).wrapping_mul(2) }
const fn quad8_control(channel: usize) -> u8 { quad8_data(channel).wrapping_add(1) }
const fn bit(n: u32) -> u8 { 1u8 << n }
const fn genmask(high: u32, low: u32) -> u8 { (((1u16 << (high-low+1)) - 1) << low) as u8 }
const fn encode(value: u8, mask: u8, shift: u32) -> u8 { (value << shift) & mask }

const FLAG_E: u8 = bit(4);
const FLAG_UD: u8 = bit(5);
const UP: u8 = 1;
const REGISTER_SELECTION: u8 = genmask(6, 5);
const SELECT_RLD: u8 = encode(0, REGISTER_SELECTION, 5);
const SELECT_CMR: u8 = encode(1, REGISTER_SELECTION, 5);
const SELECT_IOR: u8 = encode(2, REGISTER_SELECTION, 5);
const SELECT_IDR: u8 = encode(3, REGISTER_SELECTION, 5);
const RESETS: u8 = genmask(2, 1);
const LOADS: u8 = genmask(4, 3);
const RESET_BP: u8 = bit(0);
const RESET_BT_CT_CPT_S_IDX: u8 = encode(2, RESETS, 1);
const RESET_E: u8 = encode(3, RESETS, 1);
const TRANSFER_PR_TO_CNTR: u8 = encode(1, LOADS, 3);
const TRANSFER_CNTR_TO_OL: u8 = encode(2, LOADS, 3);
const TRANSFER_PR0_TO_PSC: u8 = encode(3, LOADS, 3);
const COUNT_ENCODING: u8 = bit(0);
const COUNT_MODE: u8 = genmask(2, 1);
const QUADRATURE_MODE: u8 = genmask(4, 3);
const BINARY: u8 = encode(0, COUNT_ENCODING, 0);
const NORMAL_COUNT: u8 = 0;
const RANGE_LIMIT: u8 = 1;
const NON_RECYCLE_COUNT: u8 = 2;
const MODULO_N: u8 = 3;
const NON_QUADRATURE: u8 = 0;
const QUADRATURE_X1: u8 = 1;
const QUADRATURE_X2: u8 = 2;
const QUADRATURE_X4: u8 = 3;
const AB_GATE: u8 = bit(0);
const LOAD_PIN: u8 = bit(1);
const FLG_PINS: u8 = genmask(4, 3);
const DISABLE_AB: u8 = encode(0, AB_GATE, 0);
const LOAD_CNTR: u8 = 0;
const FLG1_CARRY_FLG2_BORROW: u8 = 0;
const FLG1_COMPARE_FLG2_BORROW: u8 = 1;
const FLG1_CARRYBORROW_FLG2_UD: u8 = 2;
const FLG1_INDX_FLG2_E: u8 = 3;
const INDEX_MODE: u8 = bit(0);
const INDEX_POLARITY: u8 = bit(1);
const DISABLE_INDEX_MODE: u8 = 0;
const ENABLE_INDEX_MODE: u8 = 1;
const NEGATIVE_INDEX_POLARITY: u8 = 0;
const POSITIVE_INDEX_POLARITY: u8 = 1;
const COUNTERS_OPERATION: u8 = bit(0);
const INTERRUPT_FUNCTION: u8 = bit(2);
const ENABLE_COUNTERS: u8 = encode(0, COUNTERS_OPERATION, 0);
const RESET_COUNTERS: u8 = encode(1, COUNTERS_OPERATION, 0);
const DISABLE_INTERRUPT_FUNCTION: u8 = encode(0, INTERRUPT_FUNCTION, 2);
const ENABLE_INTERRUPT_FUNCTION: u8 = encode(1, INTERRUPT_FUNCTION, 2);
const CLEAR_PENDING_INTERRUPTS: u8 = ENABLE_COUNTERS | ENABLE_INTERRUPT_FUNCTION;

#[repr(C)]
pub struct Quad8 {
    pub lock: SpinLock,
    pub cmr: [u8; QUAD8_NUM_COUNTERS],
    pub ior: [u8; QUAD8_NUM_COUNTERS],
    pub idr: [u8; QUAD8_NUM_COUNTERS],
    pub fck_prescaler: [u32; QUAD8_NUM_COUNTERS],
    pub preset: [u32; QUAD8_NUM_COUNTERS],
    pub cable_fault_enable: u32,
    pub map: *mut Regmap,
}

// External kernel objects and operations.
pub enum SpinLock {}
pub enum Regmap {}
pub enum CounterDevice {}
pub enum CounterSignal {}
pub enum CounterCount {}
pub enum CounterSynapse {}
pub enum CounterWatch {}
pub enum CounterEventNode {}
extern "C" {
    fn regmap_write(map: *mut Regmap, reg: u8, val: u8) -> i32;
    fn regmap_read(map: *mut Regmap, reg: u8, val: *mut u32) -> i32;
    fn regmap_noinc_read(map: *mut Regmap, reg: u8, val: *mut u8, len: usize) -> i32;
    fn regmap_noinc_write(map: *mut Regmap, reg: u8, val: *const u8, len: usize) -> i32;
    fn regmap_test_bits(map: *mut Regmap, reg: u8, mask: u8) -> i32;
}

#[inline]
unsafe fn control_register_update(map: *mut Regmap, buf: *mut u8, channel: usize, val: u8, field: u8) -> i32 {
    let shift = field.trailing_zeros();
    let p = buf.add(channel);
    *p = (*p & !field) | ((val << shift) & field);
    regmap_write(map, quad8_control(channel), *p)
}

unsafe fn preset_register_set(priv_: *mut Quad8, id: usize, preset: u32) -> i32 {
    let value = [(preset & 0xff) as u8, ((preset >> 8) & 0xff) as u8, ((preset >> 16) & 0xff) as u8];
    let ret = regmap_write((*priv_).map, quad8_control(id), SELECT_RLD | RESET_BP);
    if ret != 0 { return ret; }
    regmap_noinc_write((*priv_).map, quad8_data(id), value.as_ptr(), 3)
}

unsafe fn flag_register_reset(priv_: *mut Quad8, id: usize) -> i32 {
    let ret = regmap_write((*priv_).map, quad8_control(id), SELECT_RLD | RESET_BT_CT_CPT_S_IDX);
    if ret != 0 { return ret; }
    regmap_write((*priv_).map, quad8_control(id), SELECT_RLD | RESET_E)
}

unsafe fn filter_clock_prescaler_set(priv_: *mut Quad8, id: usize, prescaler: u8) -> i32 {
    let mut ret = regmap_write((*priv_).map, quad8_control(id), SELECT_RLD | RESET_BP);
    if ret != 0 { return ret; }
    ret = regmap_write((*priv_).map, quad8_data(id), prescaler);
    if ret != 0 { return ret; }
    regmap_write((*priv_).map, quad8_control(id), SELECT_RLD | TRANSFER_PR0_TO_PSC)
}

// The remaining driver callbacks and the device registration retain the C
// driver's externally supplied counter-framework types and are declared here
// as ABI-compatible entry points for the surrounding kernel bindings.
extern "C" {
    fn quad8_signal_read(counter: *mut CounterDevice, signal: *mut CounterSignal, level: *mut u32) -> i32;
    fn quad8_count_read(counter: *mut CounterDevice, count: *mut CounterCount, value: *mut u64) -> i32;
    fn quad8_count_write(counter: *mut CounterDevice, count: *mut CounterCount, value: u64) -> i32;
    fn quad8_function_read(counter: *mut CounterDevice, count: *mut CounterCount, function: *mut u32) -> i32;
    fn quad8_function_write(counter: *mut CounterDevice, count: *mut CounterCount, function: u32) -> i32;
    fn quad8_action_read(counter: *mut CounterDevice, count: *mut CounterCount, synapse: *mut CounterSynapse, action: *mut u32) -> i32;
    fn quad8_events_configure(counter: *mut CounterDevice) -> i32;
    fn quad8_watch_validate(counter: *mut CounterDevice, watch: *const CounterWatch) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
