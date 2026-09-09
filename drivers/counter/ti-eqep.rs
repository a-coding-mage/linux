// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2019 David Lechner <david@lechnology.com>
 *
 * Counter driver for Texas Instruments Enhanced Quadrature Encoder Pulse (eQEP)
 */

// Linux kernel dependencies supplied by the surrounding translation.

const QPOSCNT: u32 = 0x0;
const QPOSINIT: u32 = 0x4;
const QPOSMAX: u32 = 0x8;
const QPOSCMP: u32 = 0xc;
const QPOSILAT: u32 = 0x10;
const QPOSSLAT: u32 = 0x14;
const QPOSLAT: u32 = 0x18;
const QUTMR: u32 = 0x1c;
const QUPRD: u32 = 0x20;
const QWDTMR: u32 = 0x0;
const QWDPRD: u32 = 0x2;
const QDECCTL: u32 = 0x4;
const QEPCTL: u32 = 0x6;
const QCAPCTL: u32 = 0x8;
const QPOSCTL: u32 = 0xa;
const QEINT: u32 = 0xc;
const QFLG: u32 = 0xe;
const QCLR: u32 = 0x10;
const QFRC: u32 = 0x12;
const QEPSTS: u32 = 0x14;
const QCTMR: u32 = 0x16;
const QCPRD: u32 = 0x18;
const QCTMRLAT: u32 = 0x1a;
const QCPRDLAT: u32 = 0x1c;

const QDECCTL_QSRC_SHIFT: u32 = 14;
const QDECCTL_QSRC: u32 = 0x3 << 14;
const QDECCTL_SOEN: u32 = 1 << 13;
const QDECCTL_SPSEL: u32 = 1 << 12;
const QDECCTL_XCR: u32 = 1 << 11;
const QDECCTL_SWAP: u32 = 1 << 10;
const QDECCTL_IGATE: u32 = 1 << 9;
const QDECCTL_QAP: u32 = 1 << 8;
const QDECCTL_QBP: u32 = 1 << 7;
const QDECCTL_QIP: u32 = 1 << 6;
const QDECCTL_QSP: u32 = 1 << 5;
const QEPCTL_FREE_SOFT: u32 = 0x3 << 14;
const QEPCTL_PCRM: u32 = 0x3 << 12;
const QEPCTL_SEI: u32 = 0x3 << 10;
const QEPCTL_IEI: u32 = 0x3 << 8;
const QEPCTL_SWI: u32 = 1 << 7;
const QEPCTL_SEL: u32 = 1 << 6;
const QEPCTL_IEL: u32 = 0x3 << 4;
const QEPCTL_PHEN: u32 = 1 << 3;
const QEPCTL_QCLM: u32 = 1 << 2;
const QEPCTL_UTE: u32 = 1 << 1;
const QEPCTL_WDE: u32 = 1;

const QEINT_PCO: u32 = 1 << 6;
const QEINT_PCU: u32 = 1 << 5;
const QEINT_QDC: u32 = 1 << 3;
const QFLG_PCO: u32 = 1 << 6;
const QFLG_PCU: u32 = 1 << 5;
const QFLG_QDC: u32 = 1 << 3;
const QEPSTS_QDF: u32 = 1 << 5;

#[repr(u32)]
enum TiEqepSignal { Qepa, Qepb }
#[repr(u32)]
enum TiEqepCountFunc { QuadCount, DirCount, UpCount, DownCount }

#[repr(C)]
struct TiEqepCnt { regmap32: *mut Regmap, regmap16: *mut Regmap }

unsafe fn ti_eqep_count_read(counter: *mut CounterDevice, _count: *mut CounterCount, val: *mut u64) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter);
    let mut cnt: u32 = 0;
    regmap_read((*priv_).regmap32, QPOSCNT, &mut cnt);
    *val = cnt as u64;
    0
}

unsafe fn ti_eqep_count_write(counter: *mut CounterDevice, _count: *mut CounterCount, val: u64) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter);
    let mut max: u32 = 0;
    regmap_read((*priv_).regmap32, QPOSMAX, &mut max);
    if val > max as u64 { return -22; }
    regmap_write((*priv_).regmap32, QPOSCNT, val as u32)
}

unsafe fn ti_eqep_function_read(counter: *mut CounterDevice, count: *mut CounterCount, function: *mut CounterFunction) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter);
    let mut qdecctl = 0u32;
    regmap_read((*priv_).regmap16, QDECCTL, &mut qdecctl);
    match ((qdecctl & QDECCTL_QSRC) >> QDECCTL_QSRC_SHIFT) {
        0 => *function = COUNTER_FUNCTION_QUADRATURE_X4,
        1 => *function = COUNTER_FUNCTION_PULSE_DIRECTION,
        2 => *function = COUNTER_FUNCTION_INCREASE,
        3 => *function = COUNTER_FUNCTION_DECREASE,
        _ => {}
    }
    0
}

unsafe fn ti_eqep_function_write(counter: *mut CounterDevice, _count: *mut CounterCount, function: CounterFunction) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter);
    let qsrc = match function {
        COUNTER_FUNCTION_QUADRATURE_X4 => 0,
        COUNTER_FUNCTION_PULSE_DIRECTION => 1,
        COUNTER_FUNCTION_INCREASE => 2,
        COUNTER_FUNCTION_DECREASE => 3,
        _ => return -22,
    };
    regmap_write_bits((*priv_).regmap16, QDECCTL, QDECCTL_QSRC, qsrc << QDECCTL_QSRC_SHIFT)
}

unsafe fn ti_eqep_position_ceiling_read(counter: *mut CounterDevice, _count: *mut CounterCount, ceiling: *mut u64) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); let mut qposmax = 0u32;
    regmap_read((*priv_).regmap32, QPOSMAX, &mut qposmax); *ceiling = qposmax as u64; 0
}
unsafe fn ti_eqep_position_ceiling_write(counter: *mut CounterDevice, _count: *mut CounterCount, ceiling: u64) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); if ceiling != ceiling as u32 as u64 { return -34; }
    regmap_write((*priv_).regmap32, QPOSMAX, ceiling as u32); 0
}
unsafe fn ti_eqep_position_enable_read(counter: *mut CounterDevice, _count: *mut CounterCount, enable: *mut u8) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); let mut qepctl = 0u32;
    regmap_read((*priv_).regmap16, QEPCTL, &mut qepctl); *enable = ((qepctl & QEPCTL_PHEN) != 0) as u8; 0
}
unsafe fn ti_eqep_position_enable_write(counter: *mut CounterDevice, _count: *mut CounterCount, enable: u8) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); regmap_write_bits((*priv_).regmap16, QEPCTL, QEPCTL_PHEN, if enable != 0 { u32::MAX } else { 0 }); 0
}
unsafe fn ti_eqep_direction_read(counter: *mut CounterDevice, _count: *mut CounterCount, direction: *mut CounterCountDirection) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); let mut qepsts = 0u32;
    regmap_read((*priv_).regmap16, QEPSTS, &mut qepsts); *direction = if qepsts & QEPSTS_QDF != 0 { COUNTER_COUNT_DIRECTION_FORWARD } else { COUNTER_COUNT_DIRECTION_BACKWARD }; 0
}

unsafe fn ti_eqep_action_read(counter: *mut CounterDevice, count: *mut CounterCount, synapse: *mut CounterSynapse, action: *mut CounterSynapseAction) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); let mut function = COUNTER_FUNCTION_QUADRATURE_X4;
    let err = ti_eqep_function_read(counter, count, &mut function); if err != 0 { return err; }
    match function {
        COUNTER_FUNCTION_QUADRATURE_X4 => { *action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES; 0 }
        COUNTER_FUNCTION_PULSE_DIRECTION => match (*(*synapse).signal).id {
            0 => { *action = COUNTER_SYNAPSE_ACTION_RISING_EDGE; 0 }, 1 => { *action = COUNTER_SYNAPSE_ACTION_NONE; 0 }, _ => -22
        },
        COUNTER_FUNCTION_INCREASE | COUNTER_FUNCTION_DECREASE => match (*(*synapse).signal).id {
            0 => { let mut qdecctl = 0; let err = regmap_read((*priv_).regmap16, QDECCTL, &mut qdecctl); if err != 0 { return err; } *action = if qdecctl & QDECCTL_XCR != 0 { COUNTER_SYNAPSE_ACTION_BOTH_EDGES } else { COUNTER_SYNAPSE_ACTION_RISING_EDGE }; 0 },
            1 => { *action = COUNTER_SYNAPSE_ACTION_NONE; 0 }, _ => -22
        }, _ => -22
    }
}
unsafe fn ti_eqep_events_configure(counter: *mut CounterDevice) -> i32 {
    let priv_: *mut TiEqepCnt = counter_priv(counter); let mut qeint = 0u32;
    for event_node in counter_events(counter) { qeint |= match (*event_node).event { COUNTER_EVENT_OVERFLOW => QEINT_PCO, COUNTER_EVENT_UNDERFLOW => QEINT_PCU, COUNTER_EVENT_DIRECTION_CHANGE => QEINT_QDC, _ => 0 }; }
    regmap_write((*priv_).regmap16, QEINT, qeint)
}
unsafe fn ti_eqep_watch_validate(_counter: *mut CounterDevice, watch: *const CounterWatch) -> i32 {
    match (*watch).event { COUNTER_EVENT_OVERFLOW | COUNTER_EVENT_UNDERFLOW | COUNTER_EVENT_DIRECTION_CHANGE => if (*watch).channel == 0 { 0 } else { -22 }, _ => -22 }
}
unsafe fn ti_eqep_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let counter = dev_id as *mut CounterDevice; let priv_: *mut TiEqepCnt = counter_priv(counter); let mut qflg = 0u32;
    regmap_read((*priv_).regmap16, QFLG, &mut qflg);
    if qflg & QFLG_PCO != 0 { counter_push_event(counter, COUNTER_EVENT_OVERFLOW, 0); }
    if qflg & QFLG_PCU != 0 { counter_push_event(counter, COUNTER_EVENT_UNDERFLOW, 0); }
    if qflg & QFLG_QDC != 0 { counter_push_event(counter, COUNTER_EVENT_DIRECTION_CHANGE, 0); }
    regmap_write((*priv_).regmap16, QCLR, qflg); IRQ_HANDLED
}

// MODULE_DEVICE_TABLE(of, ti_eqep_of_match);
// module_platform_driver(ti_eqep_driver);
// MODULE_AUTHOR("David Lechner <david@lechnology.com>");
// MODULE_DESCRIPTION("TI eQEP counter driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("COUNTER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
