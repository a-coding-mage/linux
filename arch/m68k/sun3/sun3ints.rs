/*
 * linux/arch/m68k/sun3/sun3ints.c -- Sun-3(x) Linux interrupt handling code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    fn sun3_disable_irq(irq: u32);
    fn sun3_enable_irq(irq: u32);
    fn sun3_leds(value: u8);
    fn kstat_irqs_cpu(irq: i32, cpu: i32) -> u32;
    fn local_irq_save(flags: *mut u64);
    fn local_irq_restore(flags: u64);
    fn legacy_timer_tick(ticks: u32);
    fn m68k_setup_user_interrupt(vector: u32, count: u32);
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: u32,
        name: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn pr_err(format: *const core::ffi::c_char, ...);
    #[cfg(feature = "CONFIG_SUN3")]
    fn intersil_clear();
}

// These types and constants are provided by the corresponding kernel headers.
type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
extern "C" {
    static mut sun3_intreg: *mut u8;
}

const VEC_USER: u32 = 0;
const IRQ_AUTO_5: u32 = 0;
const IRQ_AUTO_7: u32 = 0;
const IRQ_USER: u32 = 0;

pub unsafe extern "C" fn sun3_disable_interrupts() {
    sun3_disable_irq(0);
}

pub unsafe extern "C" fn sun3_enable_interrupts() {
    sun3_enable_irq(0);
}

static mut led_pattern: [u8; 8] = [
    !(0x80u8), !(0x01u8), !(0x40u8), !(0x02u8),
    !(0x20u8), !(0x04u8), !(0x10u8), !(0x08u8),
];

pub unsafe fn sun3_enable_irq(irq: u32) {
    let value = core::ptr::read_volatile(sun3_intreg);
    core::ptr::write_volatile(sun3_intreg, value | (1u8 << irq));
}

pub unsafe fn sun3_disable_irq(irq: u32) {
    let value = core::ptr::read_volatile(sun3_intreg);
    core::ptr::write_volatile(sun3_intreg, value & !(1u8 << irq));
}

unsafe extern "C" fn sun3_int7(irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let cnt = kstat_irqs_cpu(irq, 0);
    if cnt % 2000 == 0 {
        sun3_leds(led_pattern[(cnt % 16000 / 2000) as usize]);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn sun3_int5(irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut flags: u64 = 0;
    local_irq_save(&mut flags);
    #[cfg(feature = "CONFIG_SUN3")]
    intersil_clear();
    sun3_disable_irq(5);
    sun3_enable_irq(5);
    #[cfg(feature = "CONFIG_SUN3")]
    intersil_clear();
    legacy_timer_tick(1);
    let cnt = kstat_irqs_cpu(irq, 0);
    if cnt % 20 == 0 {
        sun3_leds(led_pattern[(cnt % 160 / 20) as usize]);
    }
    local_irq_restore(flags);
    IRQ_HANDLED
}

unsafe extern "C" fn sun3_vec255(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    IRQ_HANDLED
}

pub unsafe extern "C" fn sun3_init_IRQ() {
    core::ptr::write_volatile(sun3_intreg, 1);

    m68k_setup_user_interrupt(VEC_USER, 128);

    if request_irq(IRQ_AUTO_5, sun3_int5, 0, b"clock\0".as_ptr() as *const _, core::ptr::null_mut()) != 0 {
        pr_err(b"Couldn't register %s interrupt\n\0".as_ptr() as *const _, b"int5\0".as_ptr());
    }
    if request_irq(IRQ_AUTO_7, sun3_int7, 0, b"nmi\0".as_ptr() as *const _, core::ptr::null_mut()) != 0 {
        pr_err(b"Couldn't register %s interrupt\n\0".as_ptr() as *const _, b"int7\0".as_ptr());
    }
    if request_irq(IRQ_USER + 127, sun3_vec255, 0, b"vec255\0".as_ptr() as *const _, core::ptr::null_mut()) != 0 {
        pr_err(b"Couldn't register %s interrupt\n\0".as_ptr() as *const _, b"vec255\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
