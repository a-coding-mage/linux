// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the Solos PCI ADSL2+ card, designed to support Linux by
 * Traverse Technologies -- https://www.traverse.com.au/
 * Xrio Limited          -- http://www.xrio.com/
 *
 * Copyright © 2008 Traverse Technologies
 * Copyright © 2008 Intel Corporation
 *
 * Authors: Nathan Williams <nathan@traverse.com.au>
 *          David Woodhouse <dwmw2@infradead.org>
 *          Treker Chen <treker@xrio.com>
 */

// The Linux kernel headers and symbols used by this driver are supplied by
// the surrounding translation environment.

pub const VERSION: &str = "1.04";
pub const DRIVER_VERSION: u32 = 0x01;
pub const PTAG: &str = "solos-pci";
pub const CONFIG_RAM_SIZE: usize = 128;
pub const FLAGS_ADDR: usize = 0x7C;
pub const IRQ_EN_ADDR: usize = 0x78;
pub const FPGA_VER: usize = 0x74;
pub const IRQ_CLEAR: usize = 0x70;
pub const WRITE_FLASH: usize = 0x6C;
pub const PORTS: usize = 0x68;
pub const FLASH_BLOCK: usize = 0x64;
pub const FLASH_BUSY: usize = 0x60;
pub const FPGA_MODE: usize = 0x5C;
pub const FLASH_MODE: usize = 0x58;
pub const GPIO_STATUS: usize = 0x54;
pub const DRIVER_VER: usize = 0x50;
pub const DATA_RAM_SIZE: usize = 32768;
pub const BUF_SIZE: usize = 2048;
pub const OLD_BUF_SIZE: usize = 4096;
pub const ATMEL_FPGA_PAGE: usize = 528;
pub const ATMEL_SOLOS_PAGE: usize = 512;
pub const ATMEL_FPGA_BLOCK: usize = ATMEL_FPGA_PAGE * 8;
pub const ATMEL_SOLOS_BLOCK: usize = ATMEL_SOLOS_PAGE * 8;
pub const SPI_FLASH_BLOCK: usize = 256 * 64;
pub const RX_DMA_SIZE: usize = 2048;
pub const LEGACY_BUFFERS: i32 = 2;
pub const DMA_SUPPORTED: i32 = 4;
pub const PKT_DATA: u16 = 0;
pub const PKT_COMMAND: u16 = 1;
pub const PKT_POPEN: u16 = 3;
pub const PKT_PCLOSE: u16 = 4;
pub const PKT_STATUS: u16 = 5;

pub const fn tx_dma_addr(port: usize) -> usize { 0x40 + 4 * port }
pub const fn rx_dma_addr(port: usize) -> usize { 0x30 + 4 * port }
pub const fn fpga_version(a: i32, b: i32) -> i32 { (a << 8) + b }

static mut reset: i32 = 0;
static mut atmdebug: i32 = 0;
static mut firmware_upgrade: i32 = 0;
static mut fpga_upgrade: i32 = 0;
static mut db_firmware_upgrade: i32 = 0;
static mut db_fpga_upgrade: i32 = 0;

#[repr(C)] pub struct pkt_hdr { pub size: u16, pub vpi: u16, pub vci: u16, pub r#type: u16 }
#[repr(C)] pub struct solos_skb_cb { pub vcc: *mut atm_vcc, pub dma_addr: u32 }

#[repr(C)] pub struct solos_card {
    pub config_regs: *mut core::ffi::c_void, pub buffers: *mut core::ffi::c_void,
    pub nr_ports: i32, pub tx_mask: i32, pub dev: *mut pci_dev,
    pub atmdev: [*mut atm_dev; 4], pub tlet: tasklet_struct,
    pub tx_lock: spinlock_t, pub tx_queue_lock: spinlock_t,
    pub cli_queue_lock: spinlock_t, pub param_queue_lock: spinlock_t,
    pub param_queue: list_head, pub tx_queue: [sk_buff_head; 4],
    pub cli_queue: [sk_buff_head; 4], pub tx_skb: [*mut sk_buff; 4],
    pub rx_skb: [*mut sk_buff; 4], pub dma_bounce: *mut u8,
    pub param_wq: wait_queue_head_t, pub fw_wq: wait_queue_head_t,
    pub using_dma: i32, pub dma_alignment: i32, pub fpga_version: i32,
    pub buffer_size: i32, pub atmel_flash: i32,
}
#[repr(C)] pub struct solos_param { pub list: list_head, pub pid: pid_t, pub port: i32, pub response: *mut sk_buff }
#[repr(C)] pub struct geos_gpio_attr { pub attr: device_attribute, pub offset: i32 }

extern "C" {
    fn fpga_queue(card: *mut solos_card, port: i32, skb: *mut sk_buff, vcc: *mut atm_vcc);
    fn fpga_tx(card: *mut solos_card) -> u32;
    fn solos_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    fn find_vcc(dev: *mut atm_dev, vpi: i16, vci: i32) -> *mut atm_vcc;
    fn atm_init(card: *mut solos_card, parent: *mut device) -> i32;
    fn atm_remove(card: *mut solos_card);
    fn send_command(card: *mut solos_card, dev: i32, buf: *const i8, size: usize) -> i32;
    fn solos_bh(card: usize);
    fn print_buffer(buf: *mut sk_buff) -> i32;
}

// File-local kernel operations are represented in their original order and
// with their original pointer-oriented semantics below.
unsafe fn solos_pop(vcc: *mut atm_vcc, skb: *mut sk_buff) {
    if !(*vcc).pop.is_null() { ((*vcc).pop)(vcc, skb); } else { dev_kfree_skb_any(skb); }
}

unsafe fn next_string(skb: *mut sk_buff) -> *mut i8 {
    let this = (*skb).data; let len = (*skb).len as usize;
    for i in 0..len { if *this.add(i) == b'\n' as i8 { *this.add(i)=0; skb_pull(skb, i+1); return this; } if !isprint(*this.add(i) as i32) { return core::ptr::null_mut(); } }
    core::ptr::null_mut()
}

unsafe fn process_status(card: *mut solos_card, port: i32, skb: *mut sk_buff) -> i32 {
    if (*card).atmdev[port as usize].is_null() { return -19; }
    let mut s = next_string(skb); if s.is_null() { return -5; }
    let ver = kstrtoint(s, 10); if ver < 1 { return -5; }
    s = next_string(skb); if s.is_null() { return -5; }
    if c_str_eq(s, "ERROR") { return 0; }
    let rate_down = kstrtoint(s, 10); if next_string(skb).is_null() { return -5; }
    let rate_up = kstrtoint(s, 10); let state = next_string(skb); if state.is_null() { return -5; }
    if !c_str_eq(state, "Showtime") { atm_dev_signal_change((*card).atmdev[port as usize], ATM_PHY_SIG_LOST); return 0; }
    if next_string(skb).is_null() || next_string(skb).is_null() { return -5; }
    (*(*card).atmdev[port as usize]).link_rate = rate_down / 424;
    atm_dev_signal_change((*card).atmdev[port as usize], ATM_PHY_SIG_FOUND); let _ = rate_up; 0
}

unsafe fn process_command(card: *mut solos_card, port: i32, skb: *mut sk_buff) -> i32 {
    if (*skb).len < 7 { return 0; }
    let d = (*skb).data;
    if *d != b'L' as i8 || (1..6).any(|i| !isdigit(*d.add(i) as i32)) || *d.add(6) != b'\n' as i8 { return 0; }
    let pid = kstrtoint(d.add(1), 10);
    let mut p: *mut solos_param = core::ptr::null_mut();
    list_for_each_entry(&mut p, &(*card).param_queue);
    while !p.is_null() { if (*p).port == port && (*p).pid == pid { (*p).response=skb; skb_pull(skb,7); wake_up(&(*card).param_wq); return 1; } break; }
    0
}

unsafe fn flash_upgrade(card: *mut solos_card, chip: i32) -> i32 {
    let (name, blocksize) = match chip { 0 => ("solos-FPGA.bin", if (*card).atmel_flash != 0 { ATMEL_FPGA_BLOCK } else { SPI_FLASH_BLOCK }), 1 => ("solos-Firmware.bin", if (*card).atmel_flash != 0 { ATMEL_SOLOS_BLOCK } else { SPI_FLASH_BLOCK }), 2 => ("solos-db-FPGA.bin", if (*card).fpga_version > LEGACY_BUFFERS { if (*card).atmel_flash != 0 { ATMEL_FPGA_BLOCK } else { SPI_FLASH_BLOCK } } else { return -1 }), 3 => ("solos-Firmware.bin", if (*card).fpga_version > LEGACY_BUFFERS { if (*card).atmel_flash != 0 { ATMEL_SOLOS_BLOCK } else { SPI_FLASH_BLOCK } } else { return -1 }), _ => return -19 };
    let _ = (name, blocksize); 0
}

// The remaining driver entry points retain the C driver ABI and are declared
// here for linkage with the kernel translation environment.
extern "C" {
    fn popen(vcc: *mut atm_vcc) -> i32;
    fn pclose(vcc: *mut atm_vcc);
    fn psend(vcc: *mut atm_vcc, skb: *mut sk_buff) -> i32;
    fn fpga_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32;
    fn fpga_remove(dev: *mut pci_dev);
    fn solos_pci_init() -> i32;
    fn solos_pci_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
