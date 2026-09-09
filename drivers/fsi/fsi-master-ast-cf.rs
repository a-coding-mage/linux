// SPDX-License-Identifier: GPL-2.0+
// Copyright 2018 IBM Corp
/* A FSI master based on Aspeed ColdFire coprocessor. */

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

/* Kernel interfaces and constants supplied by the surrounding translation unit. */
use core::ffi::c_void;

const FW_FILE_NAME: &[u8] = b"cf-fsi-fw.bin\0";
const SCU_COPRO_CTRL: u32 = 0x100;
const SCU_COPRO_RESET: u32 = 0x00000002;
const SCU_COPRO_CLK_EN: u32 = 0x00000001;
const SCU_2500_COPRO_SEG0: u32 = 0x104;
const SCU_2500_COPRO_SEG2: u32 = 0x10c;
const SCU_2500_COPRO_SEG3: u32 = 0x110;
const SCU_2500_COPRO_SEG6: u32 = 0x11c;
const SCU_2500_COPRO_SEG7: u32 = 0x120;
const SCU_2500_COPRO_SEG_SWAP: u32 = 1;
const SCU_2500_COPRO_CACHE_CTL: u32 = 0x128;
const SCU_2500_COPRO_CACHE_EN: u32 = 1;
const SCU_2500_COPRO_SEG0_CACHE_EN: u32 = 2;
const SCU_2400_COPRO_SEG0: u32 = 0x104;
const SCU_2400_COPRO_SEG2: u32 = 0x108;
const SCU_2400_COPRO_SEG6: u32 = 0x110;
const SCU_2400_COPRO_SEG_SWAP: u32 = 0x80000000;
const SCU_2400_COPRO_CACHE_CTL: u32 = 0x118;
const SCU_2400_COPRO_CACHE_EN: u32 = 1;
const SCU_2400_COPRO_SEG0_CACHE_EN: u32 = 2;
const CVIC_EN_REG: usize = 0x10;
const CVIC_TRIG_REG: usize = 0x18;
const SYSREG_BASE: u32 = 0x1e600000;
const SRAM_SIZE: usize = 0x1000;
const LAST_ADDR_INVALID: u32 = 1;

#[repr(C)]
pub struct fsi_master { pub dev: *mut device, pub n_links: u32, pub flags: u32 }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct gen_pool { _private: [u8; 0] }

#[repr(C)]
pub struct fsi_master_acf {
    pub master: fsi_master, pub dev: *mut device, pub scu: *mut regmap, pub lock: mutex,
    pub gpio_clk: *mut gpio_desc, pub gpio_data: *mut gpio_desc, pub gpio_trans: *mut gpio_desc,
    pub gpio_enable: *mut gpio_desc, pub gpio_mux: *mut gpio_desc,
    pub gpio_clk_vreg: u16, pub gpio_clk_dreg: u16, pub gpio_dat_vreg: u16, pub gpio_dat_dreg: u16,
    pub gpio_tra_vreg: u16, pub gpio_tra_dreg: u16, pub gpio_clk_bit: u8, pub gpio_dat_bit: u8,
    pub gpio_tra_bit: u8, pub cf_mem_addr: u32, pub cf_mem_size: usize, pub cf_mem: *mut u8,
    pub cvic: *mut u8, pub sram_pool: *mut gen_pool, pub sram: *mut u8, pub is_ast2500: bool,
    pub external_mode: bool, pub trace_enabled: bool, pub last_addr: u32, pub t_send_delay: u8,
    pub t_echo_delay: u8, pub cvic_sw_irq: u32,
}
#[repr(C)] pub struct fsi_msg { pub msg: u64, pub bits: u8 }

extern "C" {
    fn crc4(crc: u8, data: u64, bits: i32) -> u8;
    fn trace_fsi_master_acf_cmd_same_addr(m: *mut fsi_master_acf);
    fn trace_fsi_master_acf_cmd_rel_addr(m: *mut fsi_master_acf, a: u32);
    fn trace_fsi_master_acf_cmd_abs_addr(m: *mut fsi_master_acf, a: u32);
}

unsafe fn msg_push_bits(msg: *mut fsi_msg, data: u64, bits: u8) {
    (*msg).msg = (*msg).msg.wrapping_shl(bits as u32) | (data & ((1u64 << bits) - 1));
    (*msg).bits = (*msg).bits.wrapping_add(bits);
}
unsafe fn msg_push_crc(msg: *mut fsi_msg) {
    let top = (*msg).bits & 3;
    let mut crc = crc4(0, (1u64 << top) | ((*msg).msg >> ((*msg).bits - top)), (top + 1) as i32);
    crc = crc4(crc, (*msg).msg, ((*msg).bits - top) as i32);
    msg_push_bits(msg, crc as u64, 4);
}
unsafe fn msg_finish_cmd(cmd: *mut fsi_msg) { (*cmd).msg <<= 64 - (*cmd).bits; }

unsafe fn check_same_address(master: *mut fsi_master_acf, id: i32, addr: u32) -> bool {
    (*master).last_addr == (((id as u32 & 3) << 21) | (addr & !3))
}
unsafe fn check_relative_address(master: *mut fsi_master_acf, id: i32, addr: u32, out: *mut u32) -> bool {
    let mut last = (*master).last_addr;
    if last == LAST_ADDR_INVALID || ((last >> 21) & 3) != id as u32 { return false; }
    last &= (1 << 21) - 1;
    let rel = addr as i32 - last as i32;
    if !(-256..=255).contains(&rel) { return false; }
    *out = rel as u32; true
}
unsafe fn last_address_update(master: *mut fsi_master_acf, id: i32, valid: bool, addr: u32) {
    (*master).last_addr = if valid { ((id as u32 & 3) << 21) | (addr & !3) } else { LAST_ADDR_INVALID };
}

/* FSI command opcodes are supplied by fsi-master.h. */
unsafe fn build_ar_command(master: *mut fsi_master_acf, cmd: *mut fsi_msg, id: u8, mut addr: u32, size: usize, data: *const u8) {
    (*cmd).bits = 0; (*cmd).msg = 0; addr &= (1 << 21) - 1;
    let mut opcode_bits = 3; let mut addr_bits = 21; let mut opcode = FSI_CMD_ABS_AR; let mut rel = 0;
    if check_same_address(master, id as i32, addr) { addr_bits = 2; opcode_bits = 2; opcode = FSI_CMD_SAME_AR; trace_fsi_master_acf_cmd_same_addr(master); }
    else if check_relative_address(master, id as i32, addr, &mut rel) { addr_bits = 9; addr = rel; opcode = FSI_CMD_REL_AR; trace_fsi_master_acf_cmd_rel_addr(master, rel); }
    else { trace_fsi_master_acf_cmd_abs_addr(master, addr); }
    let write = !data.is_null(); let ds = (size > 1) as u64; addr &= !(size as u32 - 1); if size == 4 { addr |= 1; }
    msg_push_bits(cmd, id as u64, 2); msg_push_bits(cmd, opcode as u64, opcode_bits); msg_push_bits(cmd, (!write) as u64, 1);
    msg_push_bits(cmd, addr as u64, addr_bits); msg_push_bits(cmd, ds, 1);
    for i in 0..size { if write { msg_push_bits(cmd, *data.add(i) as u64, 8); } }
    msg_push_crc(cmd); msg_finish_cmd(cmd);
}

/* External kernel operations and the remaining driver entry points retain C ABI declarations. */
extern "C" {
    static FSI_CMD_ABS_AR: u32; static FSI_CMD_REL_AR: u32; static FSI_CMD_SAME_AR: u32;
    fn fsi_master_acf_probe(pdev: *mut c_void) -> i32;
    fn fsi_master_acf_remove(pdev: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
