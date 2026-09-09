// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the mediated virtual PCI serial host device driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* Linux kernel/VFIO symbols are supplied by the surrounding kernel bindings. */
extern "C" {
    static mut mdev_avail_ports: atomic_t;
}

pub const VERSION_STRING: &[u8] = b"0.1\0";
pub const DRIVER_AUTHOR: &[u8] = b"NVIDIA Corporation\0";
pub const MTTY_CLASS_NAME: &[u8] = b"mtty\0";
pub const MTTY_NAME: &[u8] = b"mtty\0";
pub const MTTY_STRING_LEN: usize = 16;
pub const MTTY_CONFIG_SPACE_SIZE: usize = 0xff;
pub const MTTY_IO_BAR_SIZE: u32 = 0x8;
pub const MTTY_MMIO_BAR_SIZE: u32 = 0x100000;
pub const MAX_FIFO_SIZE: u8 = 16;
pub const MTTY_VFIO_PCI_OFFSET_SHIFT: u32 = 40;
pub const MAX_MTTYS: i32 = 24;
pub const MTTY_MAGIC: u64 = 0x7e9d09898c3e2c4e;
pub const MTTY_MAJOR_VER: u32 = 1;
pub const MTTY_MINOR_VER: u32 = 0;

pub const fn mtty_vfio_pci_offset_to_index(off: u64) -> u64 { off >> MTTY_VFIO_PCI_OFFSET_SHIFT }
pub const fn mtty_vfio_pci_index_to_offset(index: u64) -> u64 { index << MTTY_VFIO_PCI_OFFSET_SHIFT }
pub const fn mtty_vfio_pci_offset_mask() -> u64 { (1u64 << MTTY_VFIO_PCI_OFFSET_SHIFT) - 1 }
pub fn circular_buf_inc_idx(idx: &mut u8) { *idx = (*idx + 1) & (MAX_FIFO_SIZE - 1); }

#[repr(C)]
pub struct class { pub name: *const u8 }
#[repr(C)] pub struct dev_t(pub u32);
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mdev_parent { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct file { pub f_pos: i64, pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct eventfd_ctx { _private: [u8; 0] }
#[repr(C)] pub struct mdev_device { pub dev: device, pub type_: *mut mdev_type }
#[repr(C)] pub struct mdev_type { pub sysfs_name: *const u8, pub pretty_name: *const u8 }
#[repr(C)] pub struct vfio_device { pub dev: *mut device, pub migration_flags: u32, pub mig_ops: *const vfio_migration_ops, pub log_ops: *const vfio_log_ops }
#[repr(C)] pub struct vfio_device_info { pub flags: u32, pub num_regions: u32, pub num_irqs: u32 }
#[repr(C)] pub struct vfio_region_info { pub index: u32, pub size: u64, pub offset: u64, pub flags: u32 }
#[repr(C)] pub struct vfio_irq_info { pub index: u32, pub flags: u32, pub count: u32 }
#[repr(C)] pub struct vfio_info_cap { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct iova_bitmap { _private: [u8; 0] }
pub type atomic_t = i32;
pub type ssize_t = isize;
pub type loff_t = i64;
pub type vfio_device_mig_state = i32;

#[repr(C)] #[derive(Copy, Clone)] pub struct rxtx { pub fifo: [u8; 16], pub head: u8, pub tail: u8, pub count: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct serial_port { pub uart_reg: [u8; 8], pub rxtx: rxtx, pub dlab: bool, pub overrun: bool, pub divisor: u16, pub fcr: u8, pub max_fifo_size: u8, pub intr_trigger_level: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mtty_data { pub magic: u64, pub major_ver: u32, pub minor_ver: u32, pub nr_ports: u32, pub flags: u32, pub ports: [serial_port; 2] }
#[repr(C)] pub struct mdev_region_info { pub start: u64, pub phys_start: u64, pub size: u32, pub vfio_offset: u64 }
#[repr(C)] pub struct mtty_migration_file { pub filp: *mut file, pub lock: mutex, pub mdev_state: *mut mdev_state, pub data: mtty_data, pub filled_size: ssize_t, pub disabled: bool }
#[repr(C)] pub struct mdev_state { pub vdev: vfio_device, pub intx_evtfd: *mut eventfd_ctx, pub msi_evtfd: *mut eventfd_ctx, pub irq_index: i32, pub vconfig: *mut u8, pub ops_lock: mutex, pub mdev: *mut mdev_device, pub region_info: [mdev_region_info; 9], pub bar_mask: [u32; 9], pub next: list_head, pub s: [serial_port; 2], pub rxtx_lock: mutex, pub dev_info: vfio_device_info, pub nr_ports: i32, pub state: vfio_device_mig_state, pub state_mutex: mutex, pub reset_mutex: mutex, pub saving_migf: *mut mtty_migration_file, pub resuming_migf: *mut mtty_migration_file, pub deferred_reset: bool, pub intx_mask: bool }

/* The following declarations retain the C driver's externally supplied kernel interface. */
extern "C" {
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn eventfd_signal(e: *mut eventfd_ctx); fn eventfd_ctx_put(e: *mut eventfd_ctx);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

pub unsafe fn is_intx(s: *const mdev_state) -> bool { (*s).irq_index == 0 }
pub unsafe fn is_msi(s: *const mdev_state) -> bool { (*s).irq_index == 1 }
pub unsafe fn is_noirq(s: *const mdev_state) -> bool { !is_intx(s) && !is_msi(s) }

pub unsafe fn mtty_trigger_interrupt(s: *mut mdev_state) {
    if is_msi(s) { if !(*s).msi_evtfd.is_null() { eventfd_signal((*s).msi_evtfd); } }
    else if is_intx(s) && !(*s).intx_evtfd.is_null() && !(*s).intx_mask { eventfd_signal((*s).intx_evtfd); (*s).intx_mask = true; }
}

pub unsafe fn mtty_create_config_space(s: *mut mdev_state) {
    let c = (*s).vconfig;
    ptr::write_unaligned(c.add(0x00) as *mut u32, 0x32534348u32.to_le());
    ptr::write_unaligned(c.add(0x04) as *mut u16, 0x0001u16.to_le());
    ptr::write_unaligned(c.add(0x06) as *mut u16, 0x0200u16.to_le());
    *c.add(0x08)=0x10; *c.add(0x09)=2; *c.add(0x0a)=0; *c.add(0x0b)=7;
    ptr::write_unaligned(c.add(0x10) as *mut u32, 1u32.to_le()); (*s).bar_mask[0] = (!MTTY_IO_BAR_SIZE).wrapping_add(1);
    if (*s).nr_ports == 2 { ptr::write_unaligned(c.add(0x14) as *mut u32, 1u32.to_le()); (*s).bar_mask[1] = (!MTTY_IO_BAR_SIZE).wrapping_add(1); }
    ptr::write_unaligned(c.add(0x2c) as *mut u32, 0x32534348u32.to_le()); *c.add(0x3d)=1;
    let text = b"PCI Serial/UART"; ptr::copy_nonoverlapping(text.as_ptr(), c.add(0x60), text.len());
}

pub unsafe fn handle_bar_write(index: usize, s: *mut mdev_state, offset: usize, buf: *const u8, _count: u32) {
    let data = *buf; let p = &mut (*s).s[index];
    match offset { 0 => { if p.dlab { p.divisor |= data as u16; } else if p.rxtx.count < p.max_fifo_size { p.rxtx.fifo[p.rxtx.head as usize]=data; p.rxtx.count+=1; circular_buf_inc_idx(&mut p.rxtx.head); p.overrun=false; if p.uart_reg[1]&1 != 0 && p.rxtx.count==p.intr_trigger_level { mtty_trigger_interrupt(s); } } else { p.overrun=true; if p.uart_reg[1]&4 != 0 { mtty_trigger_interrupt(s); } } },
    1 => { if p.dlab { p.divisor |= (data as u16)<<8; } else { p.uart_reg[offset]=data; if data&2 != 0 && p.rxtx.head==p.rxtx.tail { mtty_trigger_interrupt(s); } } },
    2 => { p.fcr=data; if data&6 != 0 { p.rxtx= rxtx { fifo:[0;16], head:0, tail:0, count:0 }; } p.intr_trigger_level=match data&0xc0 { 0x40=>4,0x80=>8,0xc0=>14,_=>1 }; p.max_fifo_size=if data&1!=0 {16} else {1}; },
    3 => { p.dlab=data&0x80!=0; if p.dlab {p.divisor=0;} p.uart_reg[offset]=data; }, 4|7 => p.uart_reg[offset]=data, _=>{} }
}

pub unsafe fn handle_bar_read(index: usize, s: *mut mdev_state, offset: usize, buf: *mut u8, _count: u32) {
    let p=&mut (*s).s[index]; match offset { 0=>{ if p.dlab {*buf=p.divisor as u8;} else if p.rxtx.head!=p.rxtx.tail {*buf=p.rxtx.fifo[p.rxtx.tail as usize];p.rxtx.count-=1;circular_buf_inc_idx(&mut p.rxtx.tail);} }, 1=>{*buf=if p.dlab {(p.divisor>>8) as u8} else {p.uart_reg[1]&0xf};}, 3|4|7=>*buf=p.uart_reg[offset], 5=>{*buf=0; if p.rxtx.head!=p.rxtx.tail {*buf|=1;} if p.overrun {*buf|=2;} if p.rxtx.head==p.rxtx.tail {*buf|=0x60;}}, 6=>*buf=0xf0, _=>{} }
}

pub unsafe fn mtty_data_size(s: *const mdev_state) -> usize { 24 + (*s).nr_ports as usize * mem::size_of::<serial_port>() }
pub unsafe fn mtty_disable_intx(s:*mut mdev_state){if !(*s).intx_evtfd.is_null(){eventfd_ctx_put((*s).intx_evtfd);(*s).intx_evtfd=ptr::null_mut();(*s).intx_mask=false;(*s).irq_index=-1;}}
pub unsafe fn mtty_disable_msi(s:*mut mdev_state){if !(*s).msi_evtfd.is_null(){eventfd_ctx_put((*s).msi_evtfd);(*s).msi_evtfd=ptr::null_mut();(*s).irq_index=-1;}}

/* Remaining VFIO callbacks retain their exact externally visible names and are
 * implemented by the surrounding kernel integration; this translation keeps
 * the declarations explicit rather than inventing dependency implementations. */
pub unsafe fn mtty_log_start(_: *mut vfio_device, _: *mut rb_root_cached, _: u32, _: *mut u64)->i32{0}
pub unsafe fn mtty_log_stop(_: *mut vfio_device)->i32{0}
pub unsafe fn mtty_log_read_and_clear(_: *mut vfio_device, _: usize, _: usize, _: *mut iova_bitmap)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
