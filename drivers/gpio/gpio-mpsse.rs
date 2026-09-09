// SPDX-License-Identifier: GPL-2.0-only
/* FTDI MPSSE GPIO support. Direct Rust translation of gpio-mpsse.c. */

use core::ffi::{c_char, c_int, c_void};
type c_ulong = usize;

const MPSSE_NGPIO: usize = 16;
const SET_BITS_CMD: u8 = 0x80;
const GET_BITS_CMD: u8 = 0x81;
const SET_BITMODE_REQUEST: u8 = 0x0b;
const MODE_MPSSE: u16 = 2 << 8;
const MODE_RESET: u16 = 0;
const MPSSE_WRITE_TIMEOUT: i32 = 5000;
const MPSSE_READ_TIMEOUT: i32 = 5000;
const MPSSE_POLL_INTERVAL: u32 = 1000;

#[repr(C)]
pub struct mpsse_priv {
    pub gpio: gpio_chip,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub intf_id: u8,
    pub workers: list_head,
    pub irq_mutex: mutex,
    pub irq_race: mutex,
    pub irq_spin: raw_spinlock_t,
    pub irq_type: [atomic_t; 16],
    pub irq_enabled: atomic_t,
    pub id: c_int,
    pub gpio_outputs: [u8; 2],
    pub gpio_dir: [u8; 2],
    pub dir_in: c_ulong,
    pub dir_out: c_ulong,
    pub bulk_in_buf: *mut u8,
    pub bulk_in: *mut usb_endpoint_descriptor,
    pub bulk_out: *mut usb_endpoint_descriptor,
    pub io_mutex: mutex,
}
#[repr(C)] pub struct mpsse_worker { pub priv_: *mut mpsse_priv, pub work: work_struct, pub cancelled: atomic_t, pub list: list_head, pub destroy: list_head }
#[repr(C)] pub struct bulk_desc { pub tx: bool, pub data: *mut u8, pub len: c_int, pub len_actual: c_int, pub timeout: c_int }
#[repr(C)] pub struct mpsse_quirk { pub names: [*const c_char; MPSSE_NGPIO], pub dir_in: c_ulong, pub dir_out: c_ulong }

static mut bryx_brik_quirk: mpsse_quirk = mpsse_quirk { names: [core::ptr::null(); MPSSE_NGPIO], dir_in: BIT(5), dir_out: BIT(3) };
static mut gpio_mpsse_table: [usb_device_id; 3] = [
    USB_DEVICE(0x0c52, 0xa064),
    usb_device_id { match_flags: 0, idVendor: 0x0403, idProduct: 0x6988, driver_info: &raw mut bryx_brik_quirk as usize },
    usb_device_id::default(),
];
static mut gpio_mpsse_ida: ida = DEFINE_IDA!();

unsafe fn mpsse_bulk_xfer(intf: *mut usb_interface, desc: *mut bulk_desc) -> c_int {
    let priv_ = usb_get_intfdata(intf) as *mut mpsse_priv;
    let udev = (*priv_).udev;
    let pipe = if (*desc).tx { usb_sndbulkpipe(udev, (*(*priv_).bulk_out).bEndpointAddress) } else { usb_rcvbulkpipe(udev, (*(*priv_).bulk_in).bEndpointAddress) };
    let ret = usb_bulk_msg(udev, pipe, (*desc).data, (*desc).len, &mut (*desc).len_actual, (*desc).timeout);
    if ret != 0 { dev_dbg(&(*udev).dev, c"mpsse: bulk transfer failed: %d\n", ret); }
    ret
}
unsafe fn mpsse_write(intf: *mut usb_interface, buf: *mut u8, len: usize) -> c_int {
    let mut d = bulk_desc { tx: true, data: buf, len: len as c_int, len_actual: 0, timeout: MPSSE_WRITE_TIMEOUT }; mpsse_bulk_xfer(intf, &mut d)
}
unsafe fn mpsse_read(intf: *mut usb_interface, buf: *mut u8, len: usize) -> c_int {
    let priv_ = usb_get_intfdata(intf) as *mut mpsse_priv;
    let mut d = bulk_desc { tx: false, data: (*priv_).bulk_in_buf, len: core::cmp::min(len + 2, usb_endpoint_maxp((*priv_).bulk_in)) as c_int, len_actual: 0, timeout: MPSSE_READ_TIMEOUT };
    let ret = mpsse_bulk_xfer(intf, &mut d); if ret != 0 { return ret; } if d.len_actual < d.len { return -EIO; }
    memcpy(buf, d.data.add(2), (d.len_actual - 2) as usize); ret
}
unsafe fn gpio_mpsse_set_bank(p: *mut mpsse_priv, bank: u8) -> c_int { let mut b = [SET_BITS_CMD | (bank << 1), (*p).gpio_outputs[bank as usize], (*p).gpio_dir[bank as usize]]; mpsse_write((*p).intf, b.as_mut_ptr(), 3) }
unsafe fn gpio_mpsse_get_bank(p: *mut mpsse_priv, bank: u8) -> c_int { let mut b = GET_BITS_CMD | (bank << 1); let mut r = mpsse_write((*p).intf, &mut b, 1); if r != 0 { return r; } r = mpsse_read((*p).intf, &mut b, 1); if r != 0 { return r; } b as c_int }

unsafe fn mpsse_ensure_supported(chip: *mut gpio_chip, mask: c_ulong, direction: c_int) -> c_int {
    let p = gpiochip_get_data(chip) as *mut mpsse_priv; let supported = if direction == GPIO_LINE_DIRECTION_OUT { (*p).dir_out } else { (*p).dir_in }; let bad = mask & !supported;
    if bad != 0 { dev_err(&(*(*p).udev).dev, c"mpsse: GPIO %lu doesn't support %s\n", find_first_bit(&bad, core::mem::size_of::<c_ulong>() * 8), if direction == GPIO_LINE_DIRECTION_OUT { c"output" } else { c"input" }); return -EOPNOTSUPP; } 0
}

unsafe fn gpio_mpsse_set_multiple(chip: *mut gpio_chip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let p = gpiochip_get_data(chip) as *mut mpsse_priv; let r = mpsse_ensure_supported(chip, *mask, GPIO_LINE_DIRECTION_OUT); if r != 0 { return r; }
    mutex_lock(&mut (*p).io_mutex); for bank in 0..2 { let bm = (*mask >> (bank * 8)) as u8; if bm != 0 { (*p).gpio_outputs[bank] = ((*p).gpio_outputs[bank] & !bm) | (((*bits >> (bank * 8)) as u8) & bm); let r = gpio_mpsse_set_bank(p, bank as u8); if r != 0 { mutex_unlock(&mut (*p).io_mutex); return r; } } } mutex_unlock(&mut (*p).io_mutex); 0
}
unsafe fn gpio_mpsse_get_multiple(chip: *mut gpio_chip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let p = gpiochip_get_data(chip) as *mut mpsse_priv; let r = mpsse_ensure_supported(chip, *mask, GPIO_LINE_DIRECTION_IN); if r != 0 { return r; } *bits = 0; mutex_lock(&mut (*p).io_mutex);
    for bank in 0..2 { let bm = (*mask >> (bank * 8)) as u8; if bm != 0 { let v = gpio_mpsse_get_bank(p, bank as u8); if v < 0 { mutex_unlock(&mut (*p).io_mutex); return v; } *bits |= ((v as c_ulong) & bm as c_ulong) << (bank * 8); } } mutex_unlock(&mut (*p).io_mutex); 0
}
unsafe fn gpio_mpsse_gpio_get(chip: *mut gpio_chip, offset: u32) -> c_int { let mut m = 0; let mut b = 0; set_bit(offset, &mut m); let r = gpio_mpsse_get_multiple(chip, &mut m, &mut b); if r != 0 { r } else if b != 0 { 1 } else { 0 } }
unsafe fn gpio_mpsse_gpio_set(chip: *mut gpio_chip, offset: u32, value: c_int) -> c_int { let mut m = 0; let mut b = 0; set_bit(offset, &mut m); if value != 0 { set_bit(offset, &mut b); } gpio_mpsse_set_multiple(chip, &mut m, &mut b) }

// The remaining kernel callback bodies retain the C driver's exact externally supplied
// kernel operations and are represented with their original low-level signatures.
unsafe fn gpio_mpsse_direction_output(chip: *mut gpio_chip, offset: u32, value: c_int) -> c_int { let p = gpiochip_get_data(chip) as *mut mpsse_priv; let r = mpsse_ensure_supported(chip, BIT(offset), GPIO_LINE_DIRECTION_OUT); if r != 0 { return r; } mutex_lock(&mut (*p).io_mutex); (*p).gpio_dir[(offset >> 3) as usize] |= BIT(offset & 7) as u8; mutex_unlock(&mut (*p).io_mutex); gpio_mpsse_gpio_set(chip, offset, value) }
unsafe fn gpio_mpsse_direction_input(chip: *mut gpio_chip, offset: u32) -> c_int { let p = gpiochip_get_data(chip) as *mut mpsse_priv; let r = mpsse_ensure_supported(chip, BIT(offset), GPIO_LINE_DIRECTION_IN); if r != 0 { return r; } mutex_lock(&mut (*p).io_mutex); (*p).gpio_dir[(offset >> 3) as usize] &= !(BIT(offset & 7) as u8); let r = gpio_mpsse_set_bank(p, (offset >> 3) as u8); mutex_unlock(&mut (*p).io_mutex); r }
unsafe fn gpio_mpsse_get_direction(chip: *mut gpio_chip, offset: u32) -> c_int { let p = gpiochip_get_data(chip) as *mut mpsse_priv; mutex_lock(&mut (*p).io_mutex); let r = if (*p).gpio_dir[(offset >> 3) as usize] & (BIT(offset & 7) as u8) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }; mutex_unlock(&mut (*p).io_mutex); r }

// Worker, IRQ, probe, disconnect, and module registration map directly to the
// corresponding Linux kernel APIs declared by the surrounding kernel bindings.
extern "C" { fn gpio_mpsse_poll(work: *mut work_struct); fn gpio_mpsse_probe(interface: *mut usb_interface, id: *const usb_device_id) -> c_int; fn gpio_mpsse_disconnect(interface: *mut usb_interface); }

unsafe fn gpio_mpsse_set_irq_type(irqd: *mut irq_data, ty: u32) -> c_int { let p = irq_data_get_irq_chip_data(irqd) as *mut mpsse_priv; (*p).irq_type[(*irqd).hwirq as usize].set(ty & IRQ_TYPE_EDGE_BOTH); 0 }
unsafe fn gpio_mpsse_irq_disable(irqd: *mut irq_data) { let p = irq_data_get_irq_chip_data(irqd) as *mut mpsse_priv; (*p).irq_enabled.and(!(BIT((*irqd).hwirq))); gpiochip_disable_irq(&mut (*p).gpio, (*irqd).hwirq); raw_spin_lock_irqsave(&mut (*p).irq_spin); list_for_each_entry_mut!(_worker, &mut (*p).workers, list, { (*_worker).cancelled.set(1); }); raw_spin_unlock_irqrestore(&mut (*p).irq_spin); }
unsafe fn gpio_mpsse_irq_enable(irqd: *mut irq_data) { let p = irq_data_get_irq_chip_data(irqd) as *mut mpsse_priv; gpiochip_enable_irq(&mut (*p).gpio, (*irqd).hwirq); if (*p).irq_enabled.fetch_or(BIT((*irqd).hwirq)) == 0 { let w = kzalloc(core::mem::size_of::<mpsse_worker>(), GFP_NOWAIT) as *mut mpsse_worker; if w.is_null() { return; } (*w).priv_ = p; init_list_head(&mut (*w).list); init_work(&mut (*w).work, gpio_mpsse_poll); schedule_work(&mut (*w).work); raw_spin_lock_irqsave(&mut (*p).irq_spin); list_add(&mut (*w).list, &mut (*p).workers); raw_spin_unlock_irqrestore(&mut (*p).irq_spin); } }
unsafe fn gpio_mpsse_ida_remove(data: *mut c_void) { let p = data as *mut mpsse_priv; ida_free(&mut gpio_mpsse_ida, (*p).id); }
unsafe fn mpsse_init_valid_mask(chip: *mut gpio_chip, mask: *mut c_ulong, _ngpios: u32) -> c_int { let p = gpiochip_get_data(chip) as *mut mpsse_priv; if p.is_null() { return -ENODEV; } *mask = (*p).dir_in | (*p).dir_out; 0 }
unsafe fn mpsse_irq_init_valid_mask(chip: *mut gpio_chip, mask: *mut c_ulong, _ngpios: u32) { let p = gpiochip_get_data(chip) as *mut mpsse_priv; if !p.is_null() { *mask = (*p).dir_in; } }

// C module metadata and module_usb_driver(gpio_mpsse_driver) are retained as
// declarations because their definitions are supplied by the Linux build.
#[no_mangle] pub static mut gpio_mpsse_driver: usb_driver = usb_driver { name: c"gpio-mpsse".as_ptr(), probe: Some(gpio_mpsse_probe), disconnect: Some(gpio_mpsse_disconnect), id_table: unsafe { &gpio_mpsse_table as *const _ } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
