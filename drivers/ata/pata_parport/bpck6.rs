// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 2001 Micro Solutions Inc.
 *
 * backpack.c is a low-level protocol driver for the Micro Solutions
 * "BACKPACK" parallel port IDE adapter (works on Series 6 drives).
 *
 * Written by: Ken Hahn (linux-dev@micro-solutions.com)
 *             Clive Turvey (linux-dev@micro-solutions.com)
 */

// Linux kernel dependencies supplied externally.

const ACCESS_REG: u8 = 0x00;
const ACCESS_PORT: u8 = 0x40;
const ACCESS_READ: u8 = 0x00;
const ACCESS_WRITE: u8 = 0x20;
const CMD_PREFIX_SET: u8 = 0xe0;
const CMD_PREFIX_RESET: u8 = 0xc0;
const PREFIX_IO16: u8 = 0x01;
const PREFIX_FASTWR: u8 = 0x04;
const PREFIX_BLK: u8 = 0x08;
const REG_STATUS: u8 = 0x00;
const STATUS_IRQA: u8 = 0x01;
const STATUS_EEPROM_DO: u8 = 0x40;
const REG_VERSION: u8 = 0x01;
const REG_HWCFG: u8 = 0x02;
const REG_RAMSIZE: u8 = 0x03;
const RAMSIZE_128K: u8 = 0x02;
const REG_EEPROM: u8 = 0x06;
const EEPROM_SK: u8 = 0x01;
const EEPROM_DI: u8 = 0x02;
const EEPROM_CS: u8 = 0x04;
const EEPROM_EN: u8 = 0x08;
const REG_BLKSIZE: u8 = 0x08;
const fifo_wait: u32 = 0x10;

const PPCMODE_UNI_SW: i32 = 0;
const PPCMODE_UNI_FW: i32 = 1;
const PPCMODE_BI_SW: i32 = 2;
const PPCMODE_BI_FW: i32 = 3;
const PPCMODE_EPP_BYTE: i32 = 4;
const PPCMODE_EPP_WORD: i32 = 5;
const PPCMODE_EPP_DWORD: i32 = 6;

static mut mode_map: [i32; 5] = [PPCMODE_UNI_FW, PPCMODE_BI_FW, PPCMODE_EPP_BYTE,
    PPCMODE_EPP_WORD, PPCMODE_EPP_DWORD];

unsafe fn bpck6_send_cmd(pi: *mut pi_adapter, cmd: u8) {
    match mode_map[(*pi).mode as usize] {
        PPCMODE_UNI_SW | PPCMODE_UNI_FW | PPCMODE_BI_SW | PPCMODE_BI_FW => {
            parport_write_data((*pi).pardev.port, cmd);
            parport_frob_control((*pi).pardev.port, 0, PARPORT_CONTROL_AUTOFD);
        }
        PPCMODE_EPP_BYTE | PPCMODE_EPP_WORD | PPCMODE_EPP_DWORD => {
            ((*pi).pardev.port.ops.epp_write_addr)((*pi).pardev.port, &cmd as *const u8, 1, 0);
        }
        _ => {}
    }
}

unsafe fn bpck6_rd_data_byte(pi: *mut pi_adapter) -> u8 {
    let mut data: u8 = 0;
    match mode_map[(*pi).mode as usize] {
        PPCMODE_UNI_SW | PPCMODE_UNI_FW => {
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_INIT);
            data = parport_read_status((*pi).pardev.port);
            data = ((data & 0x80) >> 1) | ((data & 0x38) >> 3);
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_STROBE);
            data |= parport_read_status((*pi).pardev.port) & 0xB8;
        }
        PPCMODE_BI_SW | PPCMODE_BI_FW => {
            parport_data_reverse((*pi).pardev.port);
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE,
                PARPORT_CONTROL_STROBE | PARPORT_CONTROL_INIT);
            data = parport_read_data((*pi).pardev.port);
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, 0);
            parport_data_forward((*pi).pardev.port);
        }
        PPCMODE_EPP_BYTE | PPCMODE_EPP_WORD | PPCMODE_EPP_DWORD => {
            ((*pi).pardev.port.ops.epp_read_data)((*pi).pardev.port, &mut data as *mut u8, 1, 0);
        }
        _ => {}
    }
    data
}

unsafe fn bpck6_wr_data_byte(pi: *mut pi_adapter, data: u8) {
    match mode_map[(*pi).mode as usize] {
        PPCMODE_UNI_SW | PPCMODE_UNI_FW | PPCMODE_BI_SW | PPCMODE_BI_FW => {
            parport_write_data((*pi).pardev.port, data);
            parport_frob_control((*pi).pardev.port, 0, PARPORT_CONTROL_INIT);
        }
        PPCMODE_EPP_BYTE | PPCMODE_EPP_WORD | PPCMODE_EPP_DWORD => {
            ((*pi).pardev.port.ops.epp_write_data)((*pi).pardev.port, &data as *const u8, 1, 0);
        }
        _ => {}
    }
}

unsafe fn bpck6_read_regr(pi: *mut pi_adapter, cont: i32, reg: i32) -> i32 {
    let port = if cont != 0 { (reg | 8) as u8 } else { reg as u8 };
    bpck6_send_cmd(pi, port | ACCESS_PORT | ACCESS_READ);
    bpck6_rd_data_byte(pi) as i32
}

unsafe fn bpck6_write_regr(pi: *mut pi_adapter, cont: i32, reg: i32, val: i32) {
    let port = if cont != 0 { (reg | 8) as u8 } else { reg as u8 };
    bpck6_send_cmd(pi, port | ACCESS_PORT | ACCESS_WRITE);
    bpck6_wr_data_byte(pi, val as u8);
}

unsafe fn bpck6_wait_for_fifo(pi: *mut pi_adapter) {
    if ((*pi).private & fifo_wait as _) != 0 {
        for _ in 0..20 { parport_read_status((*pi).pardev.port); }
    }
}

unsafe fn bpck6_write_block(pi: *mut pi_adapter, mut buf: *mut i8, mut len: i32) {
    bpck6_send_cmd(pi, REG_BLKSIZE | ACCESS_REG | ACCESS_WRITE);
    bpck6_wr_data_byte(pi, len as u8); bpck6_wr_data_byte(pi, (len >> 8) as u8); bpck6_wr_data_byte(pi, 0);
    bpck6_send_cmd(pi, CMD_PREFIX_SET | PREFIX_IO16 | PREFIX_BLK);
    bpck6_send_cmd(pi, ATA_REG_DATA | ACCESS_PORT | ACCESS_WRITE);
    match mode_map[(*pi).mode as usize] {
        PPCMODE_UNI_SW | PPCMODE_BI_SW => while len != 0 { parport_write_data((*pi).pardev.port, *buf as u8); buf = buf.add(1); parport_frob_control((*pi).pardev.port, 0, PARPORT_CONTROL_INIT); len -= 1; },
        PPCMODE_UNI_FW | PPCMODE_BI_FW => {
            bpck6_send_cmd(pi, CMD_PREFIX_SET | PREFIX_FASTWR);
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_STROBE);
            let mut last = *buf as u8; parport_write_data((*pi).pardev.port, last);
            while len != 0 { let this = *buf as u8; buf = buf.add(1); len -= 1; if this == last { parport_frob_control((*pi).pardev.port, 0, PARPORT_CONTROL_INIT); } else { parport_write_data((*pi).pardev.port, this); last = this; } }
            parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, 0); bpck6_send_cmd(pi, CMD_PREFIX_RESET | PREFIX_FASTWR);
        }
        PPCMODE_EPP_BYTE => { ((*pi).pardev.port.ops.epp_write_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_8); bpck6_wait_for_fifo(pi); }
        PPCMODE_EPP_WORD => { ((*pi).pardev.port.ops.epp_write_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_16); bpck6_wait_for_fifo(pi); }
        PPCMODE_EPP_DWORD => { ((*pi).pardev.port.ops.epp_write_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_32); bpck6_wait_for_fifo(pi); }
        _ => {}
    }
    bpck6_send_cmd(pi, CMD_PREFIX_RESET | PREFIX_IO16 | PREFIX_BLK);
}

unsafe fn bpck6_read_block(pi: *mut pi_adapter, mut buf: *mut i8, mut len: i32) {
    bpck6_send_cmd(pi, REG_BLKSIZE | ACCESS_REG | ACCESS_WRITE);
    bpck6_wr_data_byte(pi, len as u8); bpck6_wr_data_byte(pi, (len >> 8) as u8); bpck6_wr_data_byte(pi, 0);
    bpck6_send_cmd(pi, CMD_PREFIX_SET | PREFIX_IO16 | PREFIX_BLK); bpck6_send_cmd(pi, ATA_REG_DATA | ACCESS_PORT | ACCESS_READ);
    match mode_map[(*pi).mode as usize] {
        PPCMODE_UNI_SW | PPCMODE_UNI_FW => while len != 0 { parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_INIT); let mut d=parport_read_status((*pi).pardev.port); d=((d&0x80)>>1)|((d&0x38)>>3); parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_STROBE); d|=parport_read_status((*pi).pardev.port)&0xB8; *buf=d as i8; buf=buf.add(1); len-=1; },
        PPCMODE_BI_SW | PPCMODE_BI_FW => { parport_data_reverse((*pi).pardev.port); while len != 0 { parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, PARPORT_CONTROL_STROBE|PARPORT_CONTROL_INIT); *buf=parport_read_data((*pi).pardev.port) as i8; buf=buf.add(1); len-=1; } parport_frob_control((*pi).pardev.port, PARPORT_CONTROL_STROBE, 0); parport_data_forward((*pi).pardev.port); }
        PPCMODE_EPP_BYTE => ((*pi).pardev.port.ops.epp_read_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_8),
        PPCMODE_EPP_WORD => ((*pi).pardev.port.ops.epp_read_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_16),
        PPCMODE_EPP_DWORD => ((*pi).pardev.port.ops.epp_read_data)((*pi).pardev.port, buf, len, PARPORT_EPP_FAST_32),
        _ => {}
    }
    bpck6_send_cmd(pi, CMD_PREFIX_RESET | PREFIX_IO16 | PREFIX_BLK);
}

unsafe fn bpck6_open(pi: *mut pi_adapter) -> i32 {
    let mut i: u8; let mut j: u8; let mut k: u8;
    (*pi).saved_r0=parport_read_data((*pi).pardev.port); (*pi).saved_r2=parport_read_control((*pi).pardev.port)&0x5F;
    parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT,PARPORT_CONTROL_SELECT);
    if (*pi).saved_r0==b'b' { parport_write_data((*pi).pardev.port,b'x'); }
    for c in [b'b',b'p',(*pi).unit, !(*pi).unit] { parport_write_data((*pi).pardev.port,c); }
    parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT,0); parport_write_control((*pi).pardev.port,PARPORT_CONTROL_INIT);
    i=(mode_map[(*pi).mode as usize]&0x0C) as u8; if i==0 { i=((mode_map[(*pi).mode as usize]&2)|1) as u8; }
    parport_write_data((*pi).pardev.port,i); parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT,PARPORT_CONTROL_SELECT); parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_AUTOFD,PARPORT_CONTROL_AUTOFD);
    j=((i&8)<<4)|((i&7)<<3); k=parport_read_status((*pi).pardev.port)&0xB8; if j!=k { parport_write_control((*pi).pardev.port,(*pi).saved_r2); parport_write_data((*pi).pardev.port,(*pi).saved_r0); return 0; }
    parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_AUTOFD,0); k=(parport_read_status((*pi).pardev.port)&0xB8)^0xB8; if j!=k { parport_write_control((*pi).pardev.port,(*pi).saved_r2); parport_write_data((*pi).pardev.port,(*pi).saved_r0); return 0; }
    if i&4!=0 { parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT|PARPORT_CONTROL_INIT,0); } else { parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT,0); }
    (*pi).private=0; bpck6_send_cmd(pi,ACCESS_REG|ACCESS_WRITE|REG_RAMSIZE); bpck6_wr_data_byte(pi,RAMSIZE_128K); bpck6_send_cmd(pi,ACCESS_REG|ACCESS_READ|REG_VERSION); if bpck6_rd_data_byte(pi)&0x3F==0x0C { (*pi).private|=fifo_wait as _; } 1
}
unsafe fn bpck6_deselect(pi: *mut pi_adapter) { if mode_map[(*pi).mode as usize]&4!=0 { parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_INIT,PARPORT_CONTROL_INIT); } else { parport_frob_control((*pi).pardev.port,PARPORT_CONTROL_SELECT,PARPORT_CONTROL_SELECT); } parport_write_data((*pi).pardev.port,(*pi).saved_r0); parport_write_control((*pi).pardev.port,(*pi).saved_r2|PARPORT_CONTROL_SELECT); parport_write_control((*pi).pardev.port,(*pi).saved_r2); }
unsafe fn bpck6_connect(pi: *mut pi_adapter) { bpck6_open(pi); bpck6_wr_extout(pi,3); }
unsafe fn bpck6_disconnect(pi: *mut pi_adapter) { bpck6_wr_extout(pi,0); bpck6_deselect(pi); }
unsafe fn bpck6_wr_extout(pi: *mut pi_adapter, regdata: u8) { bpck6_send_cmd(pi,REG_VERSION|ACCESS_REG|ACCESS_WRITE); bpck6_wr_data_byte(pi,(regdata&3)<<6); }
unsafe fn bpck6_test_port(pi: *mut pi_adapter) -> i32 { if (*pi).pardev.port.modes&PARPORT_MODE_EPP!=0 {5} else if (*pi).pardev.port.modes&PARPORT_MODE_TRISTATE!=0 {2} else {1} }
unsafe fn bpck6_probe_unit(pi: *mut pi_adapter) -> i32 { let saved=(*pi).mode; (*pi).mode=0; let out=bpck6_open(pi); if out!=0 { bpck6_deselect(pi); (*pi).mode=saved; 1 } else { (*pi).mode=saved; 0 } }
unsafe fn bpck6_log_adapter(pi: *mut pi_adapter) { let mode_string=["4-bit","8-bit","EPP-8","EPP-16","EPP-32"]; dev_info!(&(*pi).dev,"Micro Solutions BACKPACK Drive unit %d at 0x%x, mode:%d (%s), delay %d\n",(*pi).unit,(*pi).port,(*pi).mode,mode_string[(*pi).mode as usize],(*pi).delay); }

// Original module metadata and module_pata_parport_driver(bpck6) are build-time
// kernel integration supplied by the surrounding Rust translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
