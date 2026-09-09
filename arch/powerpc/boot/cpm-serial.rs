// SPDX-License-Identifier: GPL-2.0
/*
 * CPM serial console support.
 *
 * Copyright 2007 Freescale Semiconductor, Inc.
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * It is assumed that the firmware (or the platform file) has already set
 * up the port.
 */

// Dependencies supplied by the surrounding bootwrapper sources:
// types.h, io.h, ops.h, and page.h.

#[repr(C)]
struct CpmScc {
    gsmrl: u32,
    gsmrh: u32,
    psmr: u16,
    res1: [u8; 2],
    todr: u16,
    dsr: u16,
    scce: u16,
    res2: [u8; 2],
    sccm: u16,
    res3: u8,
    sccs: u8,
    res4: [u8; 8],
}

#[repr(C)]
struct CpmSmc {
    res1: [u8; 2],
    smcmr: u16,
    res2: [u8; 2],
    smce: u8,
    res3: [u8; 3],
    smcm: u8,
    res4: [u8; 5],
}

#[repr(C)]
struct CpmParam {
    rbase: u16,
    tbase: u16,
    rfcr: u8,
    tfcr: u8,
    mrblr: u16,
    rstate: u32,
    res1: [u8; 4],
    rbptr: u16,
    res2: [u8; 6],
    tstate: u32,
    res3: [u8; 4],
    tbptr: u16,
    res4: [u8; 6],
    maxidl: u16,
    idlc: u16,
    brkln: u16,
    brkec: u16,
    brkcr: u16,
    rmask: u16,
    res5: [u8; 4],
}

#[repr(C)]
struct CpmBd {
    sc: u16,   // Status and Control
    len: u16,  // Data length in buffer
    addr: *mut u8, // Buffer address in host memory
}

static mut CPCR: *mut core::ffi::c_void = core::ptr::null_mut();
static mut PARAM: *mut CpmParam = core::ptr::null_mut();
static mut SMC: *mut CpmSmc = core::ptr::null_mut();
static mut SCC: *mut CpmScc = core::ptr::null_mut();
static mut TBDF: *mut CpmBd = core::ptr::null_mut();
static mut RBDF: *mut CpmBd = core::ptr::null_mut();
static mut CPM_CMD: u32 = 0;
static mut CBD_ADDR: *mut core::ffi::c_void = core::ptr::null_mut();
static mut CBD_OFFSET: u32 = 0;

static mut DO_CMD: Option<unsafe extern "C" fn(i32)> = None;
static mut ENABLE_PORT: Option<unsafe extern "C" fn()> = None;
static mut DISABLE_PORT: Option<unsafe extern "C" fn()> = None;

const CPM_CMD_STOP_TX: i32 = 4;
const CPM_CMD_RESTART_TX: i32 = 6;
const CPM_CMD_INIT_RX_TX: i32 = 0;

extern "C" {
    fn in_be16(addr: *const core::ffi::c_void) -> u16;
    fn out_be16(addr: *mut core::ffi::c_void, value: u16);
    fn in_be32(addr: *const core::ffi::c_void) -> u32;
    fn out_be32(addr: *mut core::ffi::c_void, value: u32);
    fn out_8(addr: *mut u8, value: u8);
    fn sync();
    fn barrier();
    fn eieio();
    fn dt_is_compatible(devp: *mut core::ffi::c_void, compatible: *const u8) -> i32;
    fn getprop(devp: *mut core::ffi::c_void, name: *const u8, value: *mut core::ffi::c_void, len: i32) -> i32;
    fn dt_get_virtual_reg(devp: *mut core::ffi::c_void, regs: *mut *mut core::ffi::c_void, n: i32) -> i32;
    fn get_parent(devp: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn finddevice(path: *const u8) -> *mut core::ffi::c_void;
    fn _ALIGN_DOWN(value: u16, align: u16) -> u16;
}

unsafe extern "C" fn cpm1_cmd(op: i32) {
    while in_be16(CPCR) & 1 != 0 {}
    out_be16(CPCR, ((op << 8) as u16) | CPM_CMD as u16 | 1);
    while in_be16(CPCR) & 1 != 0 {}
}

unsafe extern "C" fn cpm2_cmd(op: i32) {
    while in_be32(CPCR) & 0x10000 != 0 {}
    out_be32(CPCR, op as u32 | CPM_CMD | 0x10000);
    while in_be32(CPCR) & 0x10000 != 0 {}
}

unsafe extern "C" fn smc_disable_port() {
    DO_CMD.unwrap()(CPM_CMD_STOP_TX);
    let p = core::ptr::addr_of_mut!((*SMC).smcmr) as *mut core::ffi::c_void;
    out_be16(p, in_be16(p) & !3);
}

unsafe extern "C" fn scc_disable_port() {
    DO_CMD.unwrap()(CPM_CMD_STOP_TX);
    let p = core::ptr::addr_of_mut!((*SCC).gsmrl) as *mut core::ffi::c_void;
    out_be32(p, in_be32(p) & !0x30);
}

unsafe extern "C" fn smc_enable_port() {
    let p = core::ptr::addr_of_mut!((*SMC).smcmr) as *mut core::ffi::c_void;
    out_be16(p, in_be16(p) | 3);
    DO_CMD.unwrap()(CPM_CMD_RESTART_TX);
}

unsafe extern "C" fn scc_enable_port() {
    let p = core::ptr::addr_of_mut!((*SCC).gsmrl) as *mut core::ffi::c_void;
    out_be32(p, in_be32(p) | 0x30);
    DO_CMD.unwrap()(CPM_CMD_RESTART_TX);
}

unsafe extern "C" fn cpm_serial_open() -> i32 {
    DISABLE_PORT.unwrap()();
    out_8(&mut (*PARAM).rfcr, 0x10);
    out_8(&mut (*PARAM).tfcr, 0x10);
    out_be16(&mut (*PARAM).mrblr as *mut _ as *mut _, 1);
    out_be16(&mut (*PARAM).maxidl as *mut _ as *mut _, 0);
    out_be16(&mut (*PARAM).brkec as *mut _ as *mut _, 0);
    out_be16(&mut (*PARAM).brkln as *mut _ as *mut _, 0);
    out_be16(&mut (*PARAM).brkcr as *mut _ as *mut _, 0);

    RBDF = CBD_ADDR as *mut CpmBd;
    (*RBDF).addr = (RBDF as *mut u8).offset(-1);
    (*RBDF).sc = 0xa000;
    (*RBDF).len = 1;
    TBDF = RBDF.add(1);
    (*TBDF).addr = (RBDF as *mut u8).offset(-2);
    (*TBDF).sc = 0x2000;
    (*TBDF).len = 1;
    sync();
    out_be16(&mut (*PARAM).rbase as *mut _ as *mut _, CBD_OFFSET as u16);
    out_be16(&mut (*PARAM).tbase as *mut _ as *mut _, (CBD_OFFSET + core::mem::size_of::<CpmBd>() as u32) as u16);
    DO_CMD.unwrap()(CPM_CMD_INIT_RX_TX);
    ENABLE_PORT.unwrap()();
    0
}

unsafe extern "C" fn cpm_serial_putc(c: u8) {
    while (*TBDF).sc & 0x8000 != 0 { barrier(); }
    sync();
    (*TBDF).addr.write(c);
    eieio();
    (*TBDF).sc |= 0x8000;
}

unsafe extern "C" fn cpm_serial_tstc() -> u8 {
    barrier();
    (!((*RBDF).sc & 0x8000 != 0)) as u8
}

unsafe extern "C" fn cpm_serial_getc() -> u8 {
    while cpm_serial_tstc() == 0 {}
    sync();
    let c = (*RBDF).addr.read();
    eieio();
    (*RBDF).sc |= 0x8000;
    c
}

#[no_mangle]
pub unsafe extern "C" fn cpm_console_init(devp: *mut core::ffi::c_void, scdp: *mut SerialConsoleData) -> i32 {
    let mut vreg = [core::ptr::null_mut(); 2];
    let mut reg = [0u32; 2];
    let mut is_smc = false;
    let mut is_cpm2 = false;
    let parent;
    let muram;
    let mut muram_addr = core::ptr::null_mut();
    let muram_offset: u32;
    let muram_size: u32;

    if dt_is_compatible(devp, b"fsl,cpm1-smc-uart\0".as_ptr()) != 0 { is_smc = true; }
    else if dt_is_compatible(devp, b"fsl,cpm2-scc-uart\0".as_ptr()) != 0 { is_cpm2 = true; }
    else if dt_is_compatible(devp, b"fsl,cpm2-smc-uart\0".as_ptr()) != 0 { is_cpm2 = true; is_smc = true; }
    if is_smc { ENABLE_PORT = Some(smc_enable_port); DISABLE_PORT = Some(smc_disable_port); }
    else { ENABLE_PORT = Some(scc_enable_port); DISABLE_PORT = Some(scc_disable_port); }
    DO_CMD = if is_cpm2 { Some(cpm2_cmd) } else { Some(cpm1_cmd) };
    if getprop(devp, b"fsl,cpm-command\0".as_ptr(), &mut CPM_CMD as *mut _ as *mut _, 4) < 4 { return -1; }
    if dt_get_virtual_reg(devp, vreg.as_mut_ptr(), 2) < 2 { return -1; }
    if is_smc { SMC = vreg[0] as *mut CpmSmc; } else { SCC = vreg[0] as *mut CpmScc; }
    PARAM = vreg[1] as *mut CpmParam;
    parent = get_parent(devp); if parent.is_null() { return -1; }
    if dt_get_virtual_reg(parent, &mut CPCR, 1) < 1 { return -1; }
    muram = finddevice(b"/soc/cpm/muram/data\0".as_ptr()); if muram.is_null() { return -1; }
    if dt_get_virtual_reg(muram, &mut muram_addr, 1) < 1 { return -1; }
    if getprop(muram, b"reg\0".as_ptr(), reg.as_mut_ptr() as *mut _, 8) < 8 { return -1; }
    muram_offset = reg[0]; muram_size = reg[1];
    CBD_OFFSET = muram_offset + muram_size - 2 * core::mem::size_of::<CpmBd>() as u32;
    if is_cpm2 && is_smc {
        let smc_base = PARAM as *mut u16;
        let mut pram_offset = (CBD_OFFSET - 64) as u16;
        pram_offset = _ALIGN_DOWN(pram_offset, 64);
        DISABLE_PORT.unwrap()();
        out_be16(smc_base as *mut _, pram_offset);
        PARAM = (muram_addr as *mut u8).offset((pram_offset as u32 - muram_offset) as isize) as *mut CpmParam;
    }
    CBD_ADDR = (muram_addr as *mut u8).offset((CBD_OFFSET - muram_offset) as isize) as *mut _;
    (*scdp).open = Some(cpm_serial_open); (*scdp).putc = Some(cpm_serial_putc); (*scdp).getc = Some(cpm_serial_getc); (*scdp).tstc = Some(cpm_serial_tstc);
    0
}

// Supplied by the serial console definitions in the surrounding sources.
#[repr(C)]
struct SerialConsoleData {
    open: Option<unsafe extern "C" fn() -> i32>,
    putc: Option<unsafe extern "C" fn(u8)>,
    getc: Option<unsafe extern "C" fn() -> u8>,
    tstc: Option<unsafe extern "C" fn() -> u8>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
