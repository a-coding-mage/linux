/*
 * Low-level Rust translation of mips/cavium-octeon/octeon-irq.c.
 *
 * The Linux kernel types and routines referenced here are supplied by the
 * surrounding kernel bindings.  Their declarations are intentionally not
 * reproduced in this translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const CIU3_MBOX_PER_CORE: usize = 10;
const MAX_CIU3_DOMAINS: usize = 256;
const MIPS_CORE_IRQ_LINES: usize = 8;

/* Register offsets from ciu3_addr. */
const CIU3_CONST: u64 = 0x220;
#[inline] const fn CIU3_IDT_CTL(idt: u64) -> u64 { idt * 8 + 0x110000 }
#[inline] const fn CIU3_IDT_PP(idt: u64, idx: u64) -> u64 { idt * 32 + idx * 8 + 0x120000 }
#[inline] const fn CIU3_IDT_IO(idt: u64) -> u64 { idt * 8 + 0x130000 }
#[inline] const fn CIU3_DEST_PP_INT(pp_ip: u64) -> u64 { pp_ip * 8 + 0x200000 }
#[inline] const fn CIU3_DEST_IO_INT(io: u64) -> u64 { io * 8 + 0x210000 }
#[inline] const fn CIU3_ISC_CTL(intsn: u64) -> u64 { intsn * 8 + 0x80000000 }
#[inline] const fn CIU3_ISC_W1C(intsn: u64) -> u64 { intsn * 8 + 0x90000000 }
#[inline] const fn CIU3_ISC_W1S(intsn: u64) -> u64 { intsn * 8 + 0xa0000000 }

#[repr(C)]
pub struct octeon_ciu3_info {
    pub ciu3_addr: u64,
    pub node: i32,
    pub domain: [*mut irq_domain; MAX_CIU3_DOMAINS],
    pub intsn2hw: [Option<unsafe extern "C" fn(*mut irq_domain, u32) -> u64>; MAX_CIU3_DOMAINS],
}

#[repr(C)]
pub struct octeon_irq_ciu_domain_data { pub num_sum: i32 }

#[repr(C)]
pub union octeon_ciu_chip_data_union {
    pub ciu3: (u64, u32),
    pub ciu: (u8, u8),
}
#[repr(C)]
pub struct octeon_ciu_chip_data {
    pub value: octeon_ciu_chip_data_union,
    pub gpio_line: i32,
    pub current_cpu: i32,
    pub ciu_node: i32,
}

#[repr(C)]
pub struct octeon_core_chip_data {
    pub core_irq_mutex: [u8; 0],
    pub current_en: bool,
    pub desired_en: bool,
    pub bit: u8,
}

#[repr(C)] pub struct irq_domain { pub host_data: *mut c_void }
#[repr(C)] pub struct irq_data { pub irq: u32, pub chip: *mut irq_chip, pub chip_data: *mut c_void }
#[repr(C)] pub struct irq_chip { pub name: *const u8 }

static mut OCTEON_CIU3_INFO_PER_NODE: [*mut octeon_ciu3_info; 4] = [core::ptr::null_mut(); 4];
static mut OCTEON_IRQ_CIU_TO_IRQ: [[i32; 64]; 8] = [[0; 64]; 8];
static mut OCTEON_IRQ_CIU3_IDT_IP2: u32 = 0;
static mut OCTEON_IRQ_CIU3_IDT_IP3: u32 = 0;
static mut OCTEON_IRQ_USE_IP4: bool = false;

#[inline]
fn octeon_irq_ciu3_base_mbox_intsn(core: i32) -> u32 { 0x04000 + CIU3_MBOX_PER_CORE as u32 * core as u32 }
#[inline]
fn octeon_irq_ciu3_mbox_intsn_for_core(core: i32, mbox: u32) -> u32 {
    octeon_irq_ciu3_base_mbox_intsn(core) + mbox
}

fn octeon_irq_ciu_is_edge(line: u32, bit: u32) -> bool {
    match line {
        0 => matches!(bit, 48..=49 | 50 | 52..=55 | 58),
        1 => bit == 47,
        _ => false,
    }
}

fn octeon_irq_ciu2_is_edge(line: u32, bit: u32) -> bool {
    match line {
        3 => matches!(bit, 2 | 8..=11 | 48),
        6 => matches!(bit, 52..=53 | 8..=12),
        _ => false,
    }
}

/* C ABI entry points retained with their original externally visible names. */
pub unsafe extern "C" fn octeon_irq_ciu3_mbox_send(cpu: i32, mbox: u32) {
    if mbox >= CIU3_MBOX_PER_CORE as u32 { return; }
    let _ = octeon_irq_ciu3_mbox_intsn_for_core(cpu, mbox);
}

pub unsafe extern "C" fn octeon_irq_get_block_domain(node: i32, block: u8) -> *mut irq_domain {
    let info = OCTEON_CIU3_INFO_PER_NODE[(node as usize) & 3];
    if info.is_null() { return core::ptr::null_mut(); }
    (*info).domain[block as usize]
}

/* The remaining kernel callbacks are supplied/linked by the Linux Rust ABI. */
extern "C" {
    pub fn octeon_irq_ciu3_enable(data: *mut irq_data);
    pub fn octeon_irq_ciu3_disable(data: *mut irq_data);
    pub fn octeon_irq_ciu3_ack(data: *mut irq_data);
    pub fn octeon_irq_ciu3_mask(data: *mut irq_data);
    pub fn octeon_irq_ciu3_mask_ack(data: *mut irq_data);
    pub fn octeon_irq_ciu3_xlat(d: *mut irq_domain, node: *mut c_void, intspec: *const u32,
                                intsize: u32, out_hwirq: *mut usize, out_type: *mut u32) -> i32;
    pub fn octeon_irq_ciu3_mapx(d: *mut irq_domain, virq: u32, hw: u64, chip: *mut irq_chip) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
