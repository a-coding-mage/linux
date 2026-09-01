// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

// C dependencies removed:
// linux/bitmap.h, linux/bits.h, linux/cleanup.h, linux/device.h,
// linux/dev_printk.h, linux/interrupt.h, linux/pm_runtime.h, linux/regmap.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_registers.h, sound/sdca.h,
// sound/sdca_fdl.h, sound/sdca_function.h, sound/sdca_hid.h,
// sound/sdca_interrupts.h, sound/sdca_jack.h, sound/sdca_ump.h,
// sound/soc-component.h, sound/soc.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

pub const BITS_PER_BYTE: c_uint = 8;
pub const SDCA_MAX_INTERRUPTS: usize = 31;
pub const SDCA_NO_INTERRUPT: c_int = -1;
pub const IRQF_ONESHOT: c_ulong = 0x00002000;
pub const GFP_KERNEL: c_uint = 0;
pub const EINVAL: c_int = 22;
pub const ENODEV: c_int = 19;
pub const ENOMEM: c_int = 12;

pub type irqreturn_t = c_uint;
pub type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>;

pub const IRQ_NONE: irqreturn_t = 0;
pub const IRQ_HANDLED: irqreturn_t = 1;

#[repr(C)]
pub struct device_power {
    pub completion: c_int,
}

#[repr(C)]
pub struct device {
    pub power: device_power,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_irq {
    pub reg_offset: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct regmap_irq_chip {
    pub name: *const c_char,
    pub status_base: c_uint,
    pub unmask_base: c_uint,
    pub ack_base: c_uint,
    pub num_regs: c_uint,
    pub irqs: *const regmap_irq,
    pub num_irqs: c_uint,
    pub runtime_pm: bool,
}

#[repr(C)]
pub struct regmap_irq_chip_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct sdca_function_desc {
    pub adr: c_uint,
}

#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
    pub num_entities: c_int,
    pub entities: *mut sdca_entity,
}

#[repr(C)]
pub struct sdca_entity {
    pub id: c_uint,
    pub label: *const c_char,
    pub r#type: c_uint,
    pub num_controls: c_int,
    pub controls: *mut sdca_control,
}

#[repr(C)]
pub struct sdca_control {
    pub label: *const c_char,
    pub sel: c_uint,
    pub interrupt_position: c_int,
}

#[repr(C)]
pub struct sdca_interrupt {
    pub name: *const c_char,
    pub dev: *mut device,
    pub function_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub function: *mut sdca_function_data,
    pub entity: *mut sdca_entity,
    pub control: *mut sdca_control,
    pub handler: irq_handler_t,
    pub free_priv: Option<unsafe extern "C" fn(*mut sdca_interrupt)>,
    pub early_request: bool,
    pub irq: c_int,
    pub device_regmap: *mut regmap,
}

#[repr(C)]
pub struct sdca_interrupt_info {
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irqs: [sdca_interrupt; SDCA_MAX_INTERRUPTS],
    pub irq_lock: mutex,
}

unsafe extern "C" {
    static SDW_SCP_SDCA_INT1: c_uint;
    static SDW_SCP_SDCA_INTMASK1: c_uint;
    static SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION: c_uint;
    static SDCA_CTL_ENTITY_0_FUNCTION_FAULT: c_uint;
    static SDCA_CTL_ENTITY_0_UMP_SEQUENCE_FAULT: c_uint;
    static SDCA_CTL_ENTITY_0_FUNCTION_BUSY: c_uint;
    static SDCA_CTL_ENTITY_0_DEVICE_NEWLY_ATTACHED: c_uint;
    static SDCA_CTL_ENTITY_0_INTS_DISABLED_ABNORMALLY: c_uint;
    static SDCA_CTL_ENTITY_0_STREAMING_STOPPED_ABNORMALLY: c_uint;
    static SDCA_CTL_ENTITY_0_FUNCTION_HAS_BEEN_RESET: c_uint;
    static ENTITY_0: c_uint;
    static FUNCTION_STATUS: c_uint;
    static GE: c_uint;
    static DETECTED_MODE: c_uint;
    static XU: c_uint;
    static FDL_CURRENTOWNER: c_uint;
    static HIDE: c_uint;
    static HIDTX_CURRENTOWNER: c_uint;

    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_irq_get_virq(data: *mut regmap_irq_chip_data, irq: c_int) -> c_int;
    fn request_threaded_irq(
        irq: c_int,
        handler: irq_handler_t,
        thread_fn: irq_handler_t,
        flags: c_ulong,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, data: *mut c_void);
    fn enable_irq(irq: c_int);
    fn disable_irq(irq: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> c_int;
    fn devm_regmap_add_irq_chip(
        dev: *mut device,
        map: *mut regmap,
        irq: c_int,
        irq_flags: c_ulong,
        irq_base: c_int,
        chip: *const regmap_irq_chip,
        data: *mut *mut regmap_irq_chip_data,
    ) -> c_int;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn kfree(ptr: *const c_void);
    fn completion_done(completion: *mut c_int) -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn sdca_jack_process(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_hid_process_report(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_fdl_process(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_jack_free_state(interrupt: *mut sdca_interrupt);
    fn sdca_jack_alloc_state(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_jack_init_state(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_fdl_free_state(interrupt: *mut sdca_interrupt);
    fn sdca_fdl_alloc_state(interrupt: *mut sdca_interrupt) -> c_int;
    fn sdca_destroy_hid_device(interrupt: *mut sdca_interrupt);
    fn sdca_add_hid_device(interrupt: *mut sdca_interrupt) -> c_int;
    fn SDW_SDCA_CTL(adr: c_uint, entity_id: c_uint, sel: c_uint, offset: c_uint) -> c_uint;
    fn SDCA_CTL_TYPE(entity_type: c_uint, sel: c_uint) -> c_uint;
    fn SDCA_CTL_TYPE_S(entity_type: c_uint, sel: c_uint) -> c_uint;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const fn irq_sdca(number: c_uint) -> regmap_irq {
    regmap_irq {
        reg_offset: number / BITS_PER_BYTE,
        mask: 1u32 << (number % BITS_PER_BYTE),
    }
}

static REGMAP_IRQS: [regmap_irq; SDCA_MAX_INTERRUPTS] = [
    irq_sdca(0), irq_sdca(1), irq_sdca(2), irq_sdca(3), irq_sdca(4), irq_sdca(5), irq_sdca(6),
    irq_sdca(7), irq_sdca(8), irq_sdca(9), irq_sdca(10), irq_sdca(11), irq_sdca(12),
    irq_sdca(13), irq_sdca(14), irq_sdca(15), irq_sdca(16), irq_sdca(17), irq_sdca(18),
    irq_sdca(19), irq_sdca(20), irq_sdca(21), irq_sdca(22), irq_sdca(23), irq_sdca(24),
    irq_sdca(25), irq_sdca(26), irq_sdca(27), irq_sdca(28), irq_sdca(29), irq_sdca(30),
];

static SDCA_IRQ_NAME: &[u8] = b"sdca_irq\0";

static mut SDCA_IRQ_CHIP: regmap_irq_chip = regmap_irq_chip {
    name: SDCA_IRQ_NAME.as_ptr() as *const c_char,
    status_base: 0,
    unmask_base: 0,
    ack_base: 0,
    num_regs: 4,
    irqs: REGMAP_IRQS.as_ptr(),
    num_irqs: SDCA_MAX_INTERRUPTS as c_uint,
    runtime_pm: true,
};

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

unsafe fn bit(mask: c_uint) -> c_uint {
    1u32 << mask
}

unsafe extern "C" fn base_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let dev = unsafe { (*interrupt).dev };

    unsafe { dev_info(dev, c"%s irq without full handling\n".as_ptr(), (*interrupt).name) };

    IRQ_HANDLED
}

unsafe extern "C" fn function_status_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let dev = unsafe { (*interrupt).dev };
    let mut irqret = IRQ_NONE;
    let mut val: c_uint = 0;
    let reg: c_uint;
    let mut ret: c_int;

    ret = unsafe { pm_runtime_get_sync(dev) };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to resume for function status: %d\n".as_ptr(), ret) };
        unsafe { pm_runtime_put(dev) };
        return irqret;
    }

    unsafe {
        reg = SDW_SDCA_CTL(
            (*(*(*interrupt).function).desc).adr,
            (*(*interrupt).entity).id,
            (*(*interrupt).control).sel,
            0,
        );
    }

    ret = unsafe { regmap_read((*interrupt).function_regmap, reg, &mut val) };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to read function status: %d\n".as_ptr(), ret) };
        unsafe { pm_runtime_put(dev) };
        return irqret;
    }

    unsafe { dev_dbg(dev, c"function status: %#x\n".as_ptr(), val) };

    let status: c_ulong = val as c_ulong;
    for mask in 0..BITS_PER_BYTE {
        if (status & (1u64 << mask) as c_ulong) == 0 {
            continue;
        }

        let b = unsafe { bit(mask) };
        unsafe {
            if b == SDCA_CTL_ENTITY_0_FUNCTION_NEEDS_INITIALIZATION {
                /*
                 * FIXME: Should this do init writes?
                 *
                 * Currently init writes/cache sync are done from the suspend/resume
                 * infrastructure. It is unclear in what situations one would receive this
                 * IRQ outside of that flow. Presumably it would be something like the chip
                 * crashing. In that case however doing the init writes and a cache sync might
                 * not be sufficient, for example if the failure was during audio playback
                 * there could be ordering constraints on the register writes to restore the
                 * state that are not handled by a simple cache sync.
                 */
            } else if b == SDCA_CTL_ENTITY_0_FUNCTION_FAULT {
                dev_err(dev, c"function fault\n".as_ptr());
            } else if b == SDCA_CTL_ENTITY_0_UMP_SEQUENCE_FAULT {
                dev_err(dev, c"ump sequence fault\n".as_ptr());
            } else if b == SDCA_CTL_ENTITY_0_FUNCTION_BUSY {
                dev_info(dev, c"unexpected function busy\n".as_ptr());
            } else if b == SDCA_CTL_ENTITY_0_DEVICE_NEWLY_ATTACHED
                || b == SDCA_CTL_ENTITY_0_INTS_DISABLED_ABNORMALLY
                || b == SDCA_CTL_ENTITY_0_STREAMING_STOPPED_ABNORMALLY
                || b == SDCA_CTL_ENTITY_0_FUNCTION_HAS_BEEN_RESET
            {
            }
        }
    }

    ret = unsafe { regmap_write((*interrupt).function_regmap, reg, val & 0x7f) };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to clear function status: %d\n".as_ptr(), ret) };
        unsafe { pm_runtime_put(dev) };
        return irqret;
    }

    irqret = IRQ_HANDLED;
    unsafe { pm_runtime_put(dev) };
    irqret
}

unsafe extern "C" fn detected_mode_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let dev = unsafe { (*interrupt).dev };
    let mut irqret = IRQ_NONE;

    let ret = unsafe { pm_runtime_get_sync(dev) };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to resume for detected mode: %d\n".as_ptr(), ret) };
        unsafe { pm_runtime_put(dev) };
        return irqret;
    }

    if unsafe { sdca_jack_process(interrupt) } == 0 {
        irqret = IRQ_HANDLED;
    }

    unsafe { pm_runtime_put(dev) };
    irqret
}

unsafe extern "C" fn hid_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let dev = unsafe { (*interrupt).dev };
    let mut irqret = IRQ_NONE;

    let ret = unsafe { pm_runtime_get_sync(dev) };
    if ret < 0 {
        unsafe { dev_err(dev, c"failed to resume for hid: %d\n".as_ptr(), ret) };
        unsafe { pm_runtime_put(dev) };
        return irqret;
    }

    if unsafe { sdca_hid_process_report(interrupt) } == 0 {
        irqret = IRQ_HANDLED;
    }

    unsafe { pm_runtime_put(dev) };
    irqret
}

// CONFIG_PM_SLEEP: when enabled, no_pm_in_progress checks dev->power.completion;
// otherwise the C implementation returns true unconditionally.
unsafe fn no_pm_in_progress(dev: *mut device) -> bool {
    unsafe { completion_done(&mut (*dev).power.completion) }
}

unsafe extern "C" fn fdl_owner_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let interrupt = data as *mut sdca_interrupt;
    let dev = unsafe { (*interrupt).dev };
    let mut irqret = IRQ_NONE;

    /*
     * FDL has to run from the system resume handler, at which point
     * we can't wait for the pm runtime.
     */
    if unsafe { no_pm_in_progress(dev) } {
        let ret = unsafe { pm_runtime_get_sync(dev) };
        if ret < 0 {
            unsafe { dev_err(dev, c"failed to resume for fdl: %d\n".as_ptr(), ret) };
            if unsafe { no_pm_in_progress(dev) } {
                unsafe { pm_runtime_put(dev) };
            }
            return irqret;
        }
    }

    if unsafe { sdca_fdl_process(interrupt) } == 0 {
        irqret = IRQ_HANDLED;
    }

    if unsafe { no_pm_in_progress(dev) } {
        unsafe { pm_runtime_put(dev) };
    }
    irqret
}

unsafe fn sdca_irq_request_locked(
    dev: *mut device,
    info: *mut sdca_interrupt_info,
    sdca_irq: c_int,
    name: *const c_char,
    handler: irq_handler_t,
    data: *mut c_void,
) -> c_int {
    let irq = unsafe { regmap_irq_get_virq((*info).irq_data, sdca_irq) };
    if irq < 0 {
        return irq;
    }

    let ret = unsafe { request_threaded_irq(irq, None, handler, IRQF_ONESHOT, name, data) };
    if ret != 0 {
        return ret;
    }

    unsafe { (*info).irqs[sdca_irq as usize].irq = irq };

    unsafe { dev_dbg(dev, c"requested irq %d for %s\n".as_ptr(), irq, name) };

    0
}

unsafe fn sdca_irq_free_locked(
    dev: *mut device,
    info: *mut sdca_interrupt_info,
    sdca_irq: c_int,
    name: *const c_char,
    data: *mut c_void,
) {
    let irq = unsafe { regmap_irq_get_virq((*info).irq_data, sdca_irq) };
    if irq < 0 {
        return;
    }

    unsafe { free_irq(irq, data) };

    unsafe { (*info).irqs[sdca_irq as usize].irq = 0 };

    unsafe { dev_dbg(dev, c"freed irq %d for %s\n".as_ptr(), irq, name) };
}

/**
 * sdca_irq_request - request an individual SDCA interrupt
 * @dev: Pointer to the struct device against which things should be allocated.
 * @info: Pointer to the interrupt information structure.
 * @sdca_irq: SDCA interrupt position.
 * @name: Name to be given to the IRQ.
 * @handler: A callback thread function to be called for the IRQ.
 * @data: Private data pointer that will be passed to the handler.
 *
 * Typically this is handled internally by sdca_irq_populate, however if
 * a device requires custom IRQ handling this can be called manually before
 * calling sdca_irq_populate, which will then skip that IRQ whilst processing.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_request(
    dev: *mut device,
    info: *mut sdca_interrupt_info,
    sdca_irq: c_int,
    name: *const c_char,
    handler: irq_handler_t,
    data: *mut c_void,
) -> c_int {
    if sdca_irq < 0 || sdca_irq >= SDCA_MAX_INTERRUPTS as c_int {
        unsafe { dev_err(dev, c"bad irq request: %d\n".as_ptr(), sdca_irq) };
        return -EINVAL;
    }

    let _guard = unsafe { MutexGuard::new(&mut (*info).irq_lock) };

    let ret = unsafe { sdca_irq_request_locked(dev, info, sdca_irq, name, handler, data) };
    if ret != 0 {
        unsafe { dev_err(dev, c"failed to request irq %s: %d\n".as_ptr(), name, ret) };
        return ret;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_request, "SND_SOC_SDCA");

/**
 * sdca_irq_free - free an individual SDCA interrupt
 * @dev: Pointer to the struct device.
 * @info: Pointer to the interrupt information structure.
 * @sdca_irq: SDCA interrupt position.
 * @name: Name to be given to the IRQ.
 * @data: Private data pointer that will be passed to the handler.
 *
 * Typically this is handled internally by sdca_irq_cleanup, however if
 * a device requires custom IRQ handling this can be called manually before
 * calling sdca_irq_cleanup, which will then skip that IRQ whilst processing.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_free(
    dev: *mut device,
    info: *mut sdca_interrupt_info,
    sdca_irq: c_int,
    name: *const c_char,
    data: *mut c_void,
) {
    if sdca_irq < 0 || sdca_irq >= SDCA_MAX_INTERRUPTS as c_int {
        return;
    }

    let _guard = unsafe { MutexGuard::new(&mut (*info).irq_lock) };

    unsafe { sdca_irq_free_locked(dev, info, sdca_irq, name, data) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_free, "SND_SOC_SDCA");

/**
 * sdca_irq_data_populate - Populate common interrupt data
 * @dev: Pointer to the Function device.
 * @regmap: Pointer to the Function regmap.
 * @component: Pointer to the ASoC component for the Function.
 * @function: Pointer to the SDCA Function.
 * @entity: Pointer to the SDCA Entity.
 * @control: Pointer to the SDCA Control.
 * @interrupt: Pointer to the SDCA interrupt for this IRQ.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_data_populate(
    mut dev: *mut device,
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    function: *mut sdca_function_data,
    entity: *mut sdca_entity,
    control: *mut sdca_control,
    interrupt: *mut sdca_interrupt,
) -> c_int {
    if dev.is_null() && !component.is_null() {
        dev = unsafe { (*component).dev };
    }
    if dev.is_null() {
        return -ENODEV;
    }

    let name = unsafe {
        kasprintf(
            GFP_KERNEL,
            c"%s %s".as_ptr(),
            (*entity).label,
            (*control).label,
        )
    };
    if name.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*interrupt).name = name;
        (*interrupt).dev = dev;
        if regmap.is_null() && !component.is_null() {
            (*interrupt).function_regmap = (*component).regmap;
        } else {
            (*interrupt).function_regmap = regmap;
        }
        (*interrupt).component = component;
        (*interrupt).function = function;
        (*interrupt).entity = entity;
        (*interrupt).control = control;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_data_populate, "SND_SOC_SDCA");

unsafe fn err_ptr(err: c_int) -> *mut sdca_interrupt {
    err as isize as *mut sdca_interrupt
}

unsafe fn is_err(ptr: *mut sdca_interrupt) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err(ptr: *mut sdca_interrupt) -> c_int {
    ptr as isize as c_int
}

unsafe fn get_interrupt_data(
    dev: *mut device,
    irq: c_int,
    info: *mut sdca_interrupt_info,
) -> *mut sdca_interrupt {
    if irq == SDCA_NO_INTERRUPT {
        return ptr::null_mut();
    } else if irq < 0 || irq >= SDCA_MAX_INTERRUPTS as c_int {
        unsafe { dev_err(dev, c"bad irq position: %d\n".as_ptr(), irq) };
        return unsafe { err_ptr(-EINVAL) };
    }

    if unsafe { (*info).irqs[irq as usize].irq != 0 } {
        unsafe { dev_dbg(dev, c"skipping irq %d, already requested\n".as_ptr(), irq) };
        return ptr::null_mut();
    }

    unsafe { &mut (*info).irqs[irq as usize] }
}

/**
 * sdca_irq_populate_early - process pre-audio card IRQ registrations
 * @dev: Device pointer for SDCA Function.
 * @regmap: Regmap pointer for the SDCA Function.
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * This is intended to be used as part of the Function boot process. It
 * can be called before the soundcard is registered (ie. doesn't depend
 * on component) and will populate all the required IRQ data, as well as
 * registering the FDL interrupts to start booting the device.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_populate_early(
    dev: *mut device,
    regmap: *mut regmap,
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) -> c_int {
    let _guard = unsafe { MutexGuard::new(&mut (*info).irq_lock) };

    for i in 0..unsafe { (*function).num_entities } {
        let entity = unsafe { (*function).entities.add(i as usize) };

        for j in 0..unsafe { (*entity).num_controls } {
            let control = unsafe { (*entity).controls.add(j as usize) };
            let irq = unsafe { (*control).interrupt_position };
            let interrupt = unsafe { get_interrupt_data(dev, irq, info) };
            if unsafe { is_err(interrupt) } {
                return unsafe { ptr_err(interrupt) };
            } else if interrupt.is_null() {
                continue;
            }

            let mut ret = unsafe {
                sdca_irq_data_populate(dev, regmap, ptr::null_mut(), function, entity, control, interrupt)
            };
            if ret != 0 {
                return ret;
            }

            let ctl_type = unsafe { SDCA_CTL_TYPE((*entity).r#type, (*control).sel) };
            if ctl_type == unsafe { SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_STATUS) } {
                unsafe { (*interrupt).handler = Some(function_status_handler) };
            } else if ctl_type == unsafe { SDCA_CTL_TYPE_S(GE, DETECTED_MODE) } {
                unsafe {
                    (*interrupt).handler = Some(detected_mode_handler);
                    (*interrupt).free_priv = Some(sdca_jack_free_state);
                }

                ret = unsafe { sdca_jack_alloc_state(interrupt) };
                if ret != 0 {
                    return ret;
                }
            } else if ctl_type == unsafe { SDCA_CTL_TYPE_S(XU, FDL_CURRENTOWNER) } {
                unsafe {
                    (*interrupt).handler = Some(fdl_owner_handler);
                    (*interrupt).free_priv = Some(sdca_fdl_free_state);
                }

                ret = unsafe { sdca_fdl_alloc_state(interrupt) };
                if ret != 0 {
                    return ret;
                }

                unsafe { (*interrupt).early_request = true };

                ret = unsafe {
                    sdca_irq_request_locked(
                        dev,
                        info,
                        irq,
                        (*interrupt).name,
                        (*interrupt).handler,
                        interrupt as *mut c_void,
                    )
                };
                if ret != 0 {
                    unsafe {
                        dev_err(
                            dev,
                            c"failed to request irq %s: %d\n".as_ptr(),
                            (*interrupt).name,
                            ret,
                        )
                    };
                    return ret;
                }
            } else if ctl_type == unsafe { SDCA_CTL_TYPE_S(HIDE, HIDTX_CURRENTOWNER) } {
                unsafe { (*interrupt).free_priv = Some(sdca_destroy_hid_device) };

                ret = unsafe { sdca_add_hid_device(interrupt) };
                if ret != 0 {
                    return ret;
                }

                unsafe { (*interrupt).handler = Some(hid_handler) };
            } else {
                unsafe { (*interrupt).handler = Some(base_handler) };
            }
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_populate_early, "SND_SOC_SDCA");

/**
 * sdca_irq_populate - Request all the individual IRQs for an SDCA Function
 * @function: Pointer to the SDCA Function.
 * @component: Pointer to the ASoC component for the Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * Typically this would be called from the driver for a single SDCA Function.
 *
 * Return: Zero on success, and a negative error code on failure.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_populate(
    function: *mut sdca_function_data,
    component: *mut snd_soc_component,
    info: *mut sdca_interrupt_info,
) -> c_int {
    let dev = unsafe { (*component).dev };

    let _guard = unsafe { MutexGuard::new(&mut (*info).irq_lock) };

    for i in 0..SDCA_MAX_INTERRUPTS {
        let interrupt = unsafe { &mut (*info).irqs[i] as *mut sdca_interrupt };
        let control = unsafe { (*interrupt).control };
        let entity = unsafe { (*interrupt).entity };

        if unsafe { (*interrupt).function != function || (*interrupt).irq != 0 } {
            continue;
        }

        unsafe { (*interrupt).component = component };

        let ctl_type = unsafe { SDCA_CTL_TYPE((*entity).r#type, (*control).sel) };
        if ctl_type == unsafe { SDCA_CTL_TYPE_S(GE, DETECTED_MODE) } {
            let ret = unsafe { sdca_jack_init_state(interrupt) };
            if ret != 0 {
                return ret;
            }
        }

        let irq = unsafe { (*(*interrupt).control).interrupt_position };
        let ret = unsafe {
            sdca_irq_request_locked(
                dev,
                info,
                irq,
                (*interrupt).name,
                (*interrupt).handler,
                interrupt as *mut c_void,
            )
        };
        if ret != 0 {
            unsafe {
                dev_err(
                    dev,
                    c"failed to request irq %s: %d\n".as_ptr(),
                    (*interrupt).name,
                    ret,
                )
            };
            return ret;
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_populate, "SND_SOC_SDCA");

unsafe fn sdca_irq_cleanup_flags(
    dev: *mut device,
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
    late_cleanup: bool,
) {
    let _guard = unsafe { MutexGuard::new(&mut (*info).irq_lock) };

    for i in 0..SDCA_MAX_INTERRUPTS {
        let interrupt = unsafe { &mut (*info).irqs[i] as *mut sdca_interrupt };

        if unsafe { (*interrupt).function != function || ((*interrupt).early_request && !late_cleanup) } {
            continue;
        }

        if unsafe { (*interrupt).irq != 0 } {
            unsafe {
                sdca_irq_free_locked(
                    dev,
                    info,
                    i as c_int,
                    (*interrupt).name,
                    interrupt as *mut c_void,
                )
            };
        }

        if !late_cleanup {
            continue;
        }

        if let Some(free_priv) = unsafe { (*interrupt).free_priv } {
            unsafe { free_priv(interrupt) };
        }

        unsafe { kfree((*interrupt).name as *const c_void) };
    }
}

/**
 * sdca_irq_cleanup - Free the regular IRQs for an SDCA Function
 * @dev: Device pointer against which the sdca_interrupt_info was allocated.
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * Typically this would be called from the driver for a single SDCA Function
 * from component remove.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_cleanup(
    dev: *mut device,
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) {
    unsafe { sdca_irq_cleanup_flags(dev, function, info, false) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_cleanup, "SND_SOC_SDCA");

/**
 * sdca_irq_cleanup_late - Free the early IRQs for an SDCA Function
 * @dev: Device pointer against which the sdca_interrupt_info was allocated.
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * Typically this would be called from the driver for a single SDCA Function
 * from bus remove.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_cleanup_late(
    dev: *mut device,
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) {
    unsafe { sdca_irq_cleanup_flags(dev, function, info, true) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_cleanup_late, "SND_SOC_SDCA");

/**
 * devm_sdca_irq_allocate - allocate an SDCA interrupt structure for a device
 * @sdev: Device pointer against which things should be allocated.
 * @regmap: regmap to be used for accessing the SDCA IRQ registers.
 * @irq: The interrupt number.
 *
 * Typically this would be called from the top level driver for the whole
 * SDCA device, as only a single instance is required across all Functions
 * on the device.
 *
 * Return: A pointer to the allocated sdca_interrupt_info struct, or an
 * error code.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_sdca_irq_allocate(
    sdev: *mut device,
    regmap: *mut regmap,
    irq: c_int,
) -> *mut sdca_interrupt_info {
    let info = unsafe {
        devm_kzalloc(sdev, core::mem::size_of::<sdca_interrupt_info>(), GFP_KERNEL)
            as *mut sdca_interrupt_info
    };
    if info.is_null() {
        return unsafe { err_ptr(-ENOMEM) as *mut sdca_interrupt_info };
    }

    unsafe {
        SDCA_IRQ_CHIP.status_base = SDW_SCP_SDCA_INT1;
        SDCA_IRQ_CHIP.unmask_base = SDW_SCP_SDCA_INTMASK1;
        SDCA_IRQ_CHIP.ack_base = SDW_SCP_SDCA_INT1;
        (*info).irq_chip = SDCA_IRQ_CHIP;
    }

    for i in 0..SDCA_MAX_INTERRUPTS {
        unsafe { (*info).irqs[i].device_regmap = regmap };
    }

    let mut ret = unsafe { devm_mutex_init(sdev, &mut (*info).irq_lock) };
    if ret != 0 {
        return unsafe { err_ptr(ret) as *mut sdca_interrupt_info };
    }

    ret = unsafe {
        devm_regmap_add_irq_chip(
            sdev,
            regmap,
            irq,
            IRQF_ONESHOT,
            0,
            &raw const (*info).irq_chip,
            &mut (*info).irq_data,
        )
    };
    if ret != 0 {
        unsafe { dev_err(sdev, c"failed to register irq chip: %d\n".as_ptr(), ret) };
        return unsafe { err_ptr(ret) as *mut sdca_interrupt_info };
    }

    unsafe { dev_dbg(sdev, c"registered on irq %d\n".as_ptr(), irq) };

    info
}
// EXPORT_SYMBOL_NS_GPL(devm_sdca_irq_allocate, "SND_SOC_SDCA");

unsafe fn irq_enable_flags(
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
    early: bool,
) {
    for i in 0..SDCA_MAX_INTERRUPTS {
        let interrupt = unsafe { &mut (*info).irqs[i] as *mut sdca_interrupt };

        if unsafe { (*interrupt).irq == 0 || (*interrupt).function != function } {
            continue;
        }

        let ctl_type = unsafe {
            SDCA_CTL_TYPE(
                (*(*interrupt).entity).r#type,
                (*(*interrupt).control).sel,
            )
        };
        if ctl_type == unsafe { SDCA_CTL_TYPE_S(XU, FDL_CURRENTOWNER) } {
            if early {
                unsafe { enable_irq((*interrupt).irq) };
            }
        } else if !early {
            unsafe { enable_irq((*interrupt).irq) };
        }
    }
}

/**
 * sdca_irq_enable_early - Re-enable early SDCA IRQs for a given function
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 *
 * The early version of the IRQ enable allows enabling IRQs which may be
 * necessary to bootstrap functionality for other IRQs, such as the FDL
 * process.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_enable_early(
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) {
    unsafe { irq_enable_flags(function, info, true) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_enable_early, "SND_SOC_SDCA");

/**
 * sdca_irq_enable - Re-enable SDCA IRQs for a given function
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_enable(
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) {
    unsafe { irq_enable_flags(function, info, false) };
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_enable, "SND_SOC_SDCA");

/**
 * sdca_irq_disable - Disable SDCA IRQs for a given function
 * @function: Pointer to the SDCA Function.
 * @info: Pointer to the SDCA interrupt info for this device.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sdca_irq_disable(
    function: *mut sdca_function_data,
    info: *mut sdca_interrupt_info,
) {
    for i in 0..SDCA_MAX_INTERRUPTS {
        let interrupt = unsafe { &mut (*info).irqs[i] as *mut sdca_interrupt };

        if unsafe { (*interrupt).irq == 0 || (*interrupt).function != function } {
            continue;
        }

        unsafe { disable_irq((*interrupt).irq) };
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_irq_disable, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
