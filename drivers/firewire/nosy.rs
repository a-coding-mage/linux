// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * nosy - Snoop mode driver for TI PCILynx 1394 controllers
 * Copyright (C) 2002-2007 Kristian Høgsberg
 */

// Linux kernel headers and local headers are supplied by the surrounding build.

const TCODE_PHY_PACKET: u32 = 0x10;
const PCI_DEVICE_ID_TI_PCILYNX: u32 = 0x8000;
static mut DRIVER_NAME: [u8; 5] = *b"nosy\0";
const RCV_BUFFER_SIZE: usize = 16 * 1024;

#[repr(C)]
pub struct pcl {
    pub next: u32,
    pub async_error_next: u32,
    pub user_data: u32,
    pub pcl_status: u32,
    pub remaining_transfer_count: u32,
    pub next_data_buffer: u32,
    pub buffer: [pcl_buffer; 13],
}
#[repr(C)] pub struct pcl_buffer { pub control: u32, pub pointer: u32 }
#[repr(C)] pub struct packet { pub length: u32 }
#[repr(C)] pub struct packet_buffer {
    pub data: *mut i8, pub capacity: usize,
    pub total_packet_count: i64, pub lost_packet_count: i64,
    pub size: atomic_t, pub head: *mut packet, pub tail: *mut packet,
    pub wait: wait_queue_head_t,
}
#[repr(C)] pub struct pcilynx {
    pub pci_device: *mut pci_dev, pub registers: *mut i8,
    pub rcv_start_pcl: *mut pcl, pub rcv_pcl: *mut pcl, pub rcv_buffer: *mut u32,
    pub rcv_start_pcl_bus: dma_addr_t, pub rcv_pcl_bus: dma_addr_t, pub rcv_buffer_bus: dma_addr_t,
    pub client_list_lock: spinlock_t, pub client_list: list_head,
    pub misc: miscdevice, pub link: list_head, pub kref: kref,
}
#[repr(C)] pub struct client { pub lynx: *mut pcilynx, pub tcode_mask: u32, pub buffer: packet_buffer, pub link: list_head }

extern "C" {
    fn kref_get(k: *mut kref); fn kref_put(k: *mut kref, f: unsafe extern "C" fn(*mut kref));
    fn kfree(p: *mut core::ffi::c_void); fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn atomic_set(a: *mut atomic_t, v: i32); fn atomic_read(a: *const atomic_t) -> i32;
    fn atomic_add(v: i32, a: *mut atomic_t); fn atomic_sub(v: i32, a: *mut atomic_t);
    fn init_waitqueue_head(w: *mut wait_queue_head_t); fn wake_up_interruptible(w: *mut wait_queue_head_t);
    fn wait_event_interruptible(w: *mut wait_queue_head_t, condition: bool) -> i32;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn memcpy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
    fn writel(v: u32, p: *mut i8); fn readl(p: *mut i8) -> u32;
    fn dev_err(dev: *mut device, fmt: *const i8, ...); fn dev_info(dev: *mut device, fmt: *const i8, ...);
    fn ktime_get_real_ts64(ts: *mut timespec64); fn iounmap(p: *mut i8);
    fn pci_get_drvdata(d: *mut pci_dev) -> *mut pcilynx; fn pci_set_drvdata(d: *mut pci_dev, p: *mut pcilynx);
    fn pci_enable_device(d: *mut pci_dev) -> i32; fn pci_disable_device(d: *mut pci_dev);
    fn pci_set_master(d: *mut pci_dev); fn pci_resource_start(d: *mut pci_dev, n: i32) -> dma_addr_t;
    fn free_irq(i: i32, p: *mut core::ffi::c_void); fn request_irq(i: i32, f: unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t, flags: u32, n: *const u8, d: *mut core::ffi::c_void) -> i32;
    fn dma_set_mask(d: *mut device, m: u64) -> i32; fn dma_alloc_coherent(d: *mut device, n: usize, bus: *mut dma_addr_t, f: u32) -> *mut core::ffi::c_void;
    fn dma_free_coherent(d: *mut device, n: usize, p: *mut core::ffi::c_void, bus: dma_addr_t);
    fn misc_register(m: *mut miscdevice) -> i32; fn misc_deregister(m: *mut miscdevice);
    fn stream_open(i: *mut inode, f: *mut file) -> i32;
}

// External kernel types, constants, list/mutex/spinlock helpers, ioctl values,
// register definitions, and byte-order helpers are supplied by the kernel port.
pub unsafe fn lynx_get(lynx: *mut pcilynx) -> *mut pcilynx { kref_get(&mut (*lynx).kref); lynx }
unsafe extern "C" fn lynx_release(k: *mut kref) { kfree(k as *mut _); }
pub unsafe fn lynx_put(lynx: *mut pcilynx) { kref_put(&mut (*lynx).kref, lynx_release); }

pub unsafe fn packet_buffer_init(buffer: *mut packet_buffer, capacity: usize) -> i32 {
    (*buffer).data = kmalloc(capacity, GFP_KERNEL) as *mut i8;
    if (*buffer).data.is_null() { return -ENOMEM; }
    (*buffer).head = (*buffer).data as *mut packet; (*buffer).tail = (*buffer).data as *mut packet;
    (*buffer).capacity = capacity; (*buffer).lost_packet_count = 0; atomic_set(&mut (*buffer).size, 0); init_waitqueue_head(&mut (*buffer).wait); 0
}
pub unsafe fn packet_buffer_destroy(buffer: *mut packet_buffer) { kfree((*buffer).data as *mut _); }

pub unsafe fn packet_buffer_get(client: *mut client, data: *mut i8, user_length: usize) -> isize {
    let b = &mut (*client).buffer;
    if wait_event_interruptible(&mut b.wait, atomic_read(&b.size) > 0) != 0 || list_empty(&(*(*client).lynx).link) { return -ERESTARTSYS as isize; }
    if atomic_read(&b.size) == 0 { return -ENODEV as isize; }
    let length = (*b.head).length as usize; if length > user_length { return 0; }
    let end = b.data.add(b.capacity);
    if ((*b.head).data().add(length)) < end { if copy_to_user(data as *mut _, (*b.head).data() as *const _, length) != 0 { return -EFAULT as isize; } b.head = (*b.head).data().add(length) as *mut packet; }
    else { let split = end.offset_from((*b.head).data()) as usize; if copy_to_user(data as *mut _, (*b.head).data() as *const _, split)!=0 || copy_to_user(data.add(split) as *mut _, b.data as *const _, length-split)!=0 { return -EFAULT as isize; } b.head = b.data.add(length-split) as *mut packet; }
    atomic_sub((core::mem::size_of::<packet>() + length) as i32, &mut b.size); length as isize
}

pub unsafe fn packet_buffer_put(buffer: *mut packet_buffer, data: *mut core::ffi::c_void, length: usize) {
    let b=&mut *buffer; b.total_packet_count+=1;
    if b.capacity < atomic_read(&b.size) as usize + core::mem::size_of::<packet>() + length { b.lost_packet_count+=1; return; }
    let end=b.data.add(b.capacity); (*b.tail).length=length as u32;
    if (*b.tail).data().add(length)<end { memcpy((*b.tail).data() as *mut _,data,length); b.tail=(*b.tail).data().add(length) as *mut packet; }
    else { let split=end.offset_from((*b.tail).data()) as usize; memcpy((*b.tail).data() as *mut _,data,split); memcpy(b.data,data.add(split),length-split); b.tail=b.data.add(length-split) as *mut packet; }
    atomic_add((core::mem::size_of::<packet>()+length) as i32,&mut b.size); wake_up_interruptible(&mut b.wait);
}

impl packet { unsafe fn data(&self)->*mut i8 { (self as *const _ as *mut u8).add(core::mem::size_of::<packet>()) as *mut i8 } }
pub unsafe fn reg_write(l:*mut pcilynx,o:i32,d:u32){ writel(d,(*l).registers.offset(o as isize)); }
pub unsafe fn reg_read(l:*mut pcilynx,o:i32)->u32{ readl((*l).registers.offset(o as isize)) }
pub unsafe fn reg_set_bits(l:*mut pcilynx,o:i32,m:u32){reg_write(l,o,reg_read(l,o)|m)}
pub unsafe fn run_pcl(l:*mut pcilynx,b:dma_addr_t,c:i32){reg_write(l,DMA0_CURRENT_PCL+c*0x20,b as u32);reg_write(l,DMA0_CHAN_CTRL+c*0x20,DMA_CHAN_CTRL_ENABLE|DMA_CHAN_CTRL_LINK)}

pub unsafe fn set_phy_reg(l:*mut pcilynx,addr:i32,val:i32)->i32 { if addr>15||val>0xff { return -1; } reg_write(l,LINK_PHY,LINK_PHY_WRITE|LINK_PHY_ADDR(addr)|LINK_PHY_WDATA(val)); 0 }

pub unsafe extern "C" fn packet_irq_handler(l:*mut pcilynx) {
    let length=(u32::from_le((*(*l).rcv_pcl).pcl_status)&0x1fff) as usize;
    let tcode=(u32::from_le(*(*l).rcv_buffer.add(1))>>4)&0xf;
    let mut ts=timespec64{tv_sec:0,tv_nsec:0}; ktime_get_real_ts64(&mut ts);
    *(*l).rcv_buffer=(ts.tv_nsec as u32/NSEC_PER_USEC) as u32;
    let mask=if length==12 {1<<TCODE_PHY_PACKET} else {1<<tcode};
    spin_lock_irq(&mut (*l).client_list_lock);
    let mut c=(*l).client_list.next as *mut client;
    while c != &mut (*l).client_list as *mut _ as *mut client { if (*c).tcode_mask&mask!=0 {packet_buffer_put(&mut (*c).buffer,(*l).rcv_buffer as *mut _,length+4);} c=(*c).link.next as *mut client; }
    spin_unlock_irq(&mut (*l).client_list_lock);
}
pub unsafe extern "C" fn bus_reset_irq_handler(l:*mut pcilynx){let mut ts=timespec64{tv_sec:0,tv_nsec:0};ktime_get_real_ts64(&mut ts);spin_lock_irq(&mut (*l).client_list_lock);let mut c=(*l).client_list.next as *mut client;while c!=&mut (*l).client_list as *mut _ as *mut client{packet_buffer_put(&mut (*c).buffer,&mut ts.tv_nsec as *mut _ as *mut _,4);c=(*c).link.next as *mut client;}spin_unlock_irq(&mut (*l).client_list_lock);}

// Device open/release, poll, read, ioctl, IRQ registration, PCI probe/remove,
// file operations, PCI ID table, and module registration use the corresponding
// kernel declarations and constants supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
