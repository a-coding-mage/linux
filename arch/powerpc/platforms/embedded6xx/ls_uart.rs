/*
 * AVR power-management chip interface for the Buffalo Linkstation /
 * Kurobox Platform.
 *
 * Author: 2006 (c) G. Liakhovetski
 *	 g.liakhovetski@gmx.de
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty
 * of any kind, whether express or implied.
 */

use core::ffi::c_void;

// Kernel and platform declarations supplied by the surrounding translation.
extern "C" {
    fn in_8(addr: *const c_void) -> u8;
    fn out_8(addr: *mut c_void, value: u8);
    fn msleep(milliseconds: u32);
    fn printk(format: *const u8, ...);
    fn local_irq_disable();
    fn local_irq_enable();
    fn of_find_node_by_path(path: *const u8) -> *mut device_node;
    fn of_get_property(node: *mut device_node, name: *const u8, len: *mut i32) -> *const c_void;
    fn of_address_to_resource(node: *mut device_node, index: i32, resource: *mut resource) -> i32;
    fn of_node_put(node: *mut device_node);
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn schedule_work(work: *mut work_struct) -> i32;
    fn init_work(work: *mut work_struct, function: unsafe extern "C" fn(*mut work_struct));
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    _private: [u8; 0],
}

static mut avr_addr: *mut c_void = core::ptr::null_mut();
static mut avr_clock: u64 = 0;

static mut WD_WORK: work_struct = work_struct { _private: [] };

unsafe extern "C" fn wd_stop(_unused: *mut work_struct) {
    let string = b"AAAAFFFFJJJJ>>>>VVVV>>>>ZZZZVVVVKKKK\0";
    let mut i: i32 = 0;
    let mut rescue: i32 = 8;
    let len: i32 = string.len() as i32 - 1;

    while rescue != 0 {
        let mut j: i32;
        let lsr = in_8(avr_addr.add(UART_LSR as usize));

        if (lsr & (UART_LSR_THRE | UART_LSR_TEMT)) != 0 {
            j = 0;
            while j < 16 && i < len {
                out_8(avr_addr.add(UART_TX as usize), string[i as usize]);
                j += 1;
                i += 1;
            }
            if i == len {
                /* Read "OK" back: 4ms for the last "KKKK"
                   plus a couple bytes back */
                msleep(7);
                printk(b"linkstation: disarming the AVR watchdog: \0".as_ptr());
                while (in_8(avr_addr.add(UART_LSR as usize)) & UART_LSR_DR) != 0 {
                    printk(b"%c\0".as_ptr(), in_8(avr_addr.add(UART_RX as usize)) as i32);
                }
                break;
            }
        }
        msleep(17);
        rescue -= 1;
    }
    printk(b"\n\0".as_ptr());
}

#[inline]
const fn avr_quot(clock: u64) -> u64 {
    (clock + 8 * 9600) / (16 * 9600)
}

#[no_mangle]
pub unsafe extern "C" fn avr_uart_configure() {
    let mut cval: u8 = UART_LCR_WLEN8;
    let quot: u64 = avr_quot(avr_clock);

    if avr_addr.is_null() || avr_clock == 0 {
        return;
    }

    out_8(avr_addr.add(UART_LCR as usize), cval); // initialise UART
    out_8(avr_addr.add(UART_MCR as usize), 0);
    out_8(avr_addr.add(UART_IER as usize), 0);

    cval |= UART_LCR_STOP | UART_LCR_PARITY | UART_LCR_EPAR;
    out_8(avr_addr.add(UART_LCR as usize), cval); // Set character format
    out_8(avr_addr.add(UART_LCR as usize), cval | UART_LCR_DLAB); // set DLAB
    out_8(avr_addr.add(UART_DLL as usize), (quot & 0xff) as u8); // LS of divisor
    out_8(avr_addr.add(UART_DLM as usize), (quot >> 8) as u8); // MS of divisor
    out_8(avr_addr.add(UART_LCR as usize), cval); // reset DLAB
    out_8(avr_addr.add(UART_FCR as usize), UART_FCR_ENABLE_FIFO); // enable FIFO
}

#[no_mangle]
pub unsafe extern "C" fn avr_uart_send(c: u8) {
    if avr_addr.is_null() || avr_clock == 0 {
        return;
    }
    out_8(avr_addr.add(UART_TX as usize), c);
    out_8(avr_addr.add(UART_TX as usize), c);
    out_8(avr_addr.add(UART_TX as usize), c);
    out_8(avr_addr.add(UART_TX as usize), c);
}

unsafe extern "C" fn ls_uart_init() {
    local_irq_disable();

    // CONFIG_SERIAL_8250 conditional: declarations are preserved; this block
    // is active when the 8250 serial driver is not configured.
    #[cfg(not(CONFIG_SERIAL_8250))]
    {
        out_8(avr_addr.add(UART_FCR as usize), UART_FCR_ENABLE_FIFO); // enable FIFO
        out_8(avr_addr.add(UART_FCR as usize), UART_FCR_ENABLE_FIFO | UART_FCR_CLEAR_RCVR | UART_FCR_CLEAR_XMIT); // clear FIFOs
        out_8(avr_addr.add(UART_FCR as usize), 0);
        out_8(avr_addr.add(UART_IER as usize), 0);
        let _ = in_8(avr_addr.add(UART_LSR as usize));
        let _ = in_8(avr_addr.add(UART_RX as usize));
        let _ = in_8(avr_addr.add(UART_IIR as usize));
        let _ = in_8(avr_addr.add(UART_MSR as usize));
    }
    avr_uart_configure();
    local_irq_enable();
}

unsafe extern "C" fn ls_uarts_init() -> i32 {
    let avr = of_find_node_by_path(b"/soc10x/serial@80004500\0".as_ptr());
    if avr.is_null() { return -EINVAL; }

    let mut len: i32 = 0;
    avr_clock = *(of_get_property(avr, b"clock-frequency\0".as_ptr(), &mut len) as *const u32) as u64;
    if avr_clock == 0 { return -EINVAL; }

    let mut res = resource { start: 0, _private: [] };
    let ret = of_address_to_resource(avr, 0, &mut res);
    if ret != 0 { return ret; }
    of_node_put(avr);

    avr_addr = ioremap(res.start, 32);
    if avr_addr.is_null() { return -EFAULT; }
    ls_uart_init();
    init_work(&mut WD_WORK, wd_stop);
    schedule_work(&mut WD_WORK);
    0
}

// machine_late_initcall(linkstation, ls_uarts_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
