// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regcache_drop_region(map: *mut regmap, min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_enum_val_to_item(e: *mut soc_enum, val: c_uint) -> c_uint;
    fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_soc_component_get_kcontrol(component: *mut snd_soc_component, name: *const c_char) -> *mut snd_kcontrol;
    fn sdca_selector_find_range(
        dev: *mut device,
        entity: *mut sdca_entity,
        selector: c_uint,
        ncols: c_uint,
        row: c_uint,
    ) -> *mut sdca_control_range;
    fn sdca_range(range: *mut sdca_control_range, col: c_uint, row: c_int) -> sdca_terminal_type;
    fn sdca_range_search(
        range: *mut sdca_control_range,
        key_col: c_uint,
        key: c_uint,
        val_col: c_uint,
    ) -> sdca_terminal_type;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
}

/**
 * sdca_jack_process - Process an SDCA jack event
 * @interrupt: SDCA interrupt structure
 *
 * Return: Zero on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_process(interrupt: *mut sdca_interrupt) -> c_int {
    unsafe {
        let dev = (*interrupt).dev;
        let component = (*interrupt).component;
        let card = (*component).card;
        let rwsem = &mut (*(*card).snd_card).controls_rwsem as *mut rw_semaphore;
        let state = (*interrupt).priv as *mut jack_state;
        let kctl = (*state).kctl;
        let mut ucontrol: *mut snd_ctl_elem_value = ptr::null_mut();
        let mut soc_enum: *mut soc_enum;
        let mut reg: c_uint;
        let mut val: c_uint = 0;
        let mut ret: c_int;

        // guard(rwsem_write)(rwsem);
        rwsem_write_lock(rwsem);

        reg = SDW_SDCA_CTL(
            (*(*(*interrupt).function).desc).adr,
            (*(*interrupt).entity).id,
            (*(*interrupt).control).sel,
            0,
        );

        ret = regmap_read((*interrupt).function_regmap, reg, &mut val);
        if ret < 0 {
            dev_err(dev, c"failed to read detected mode: %d\n".as_ptr(), ret);
            rwsem_write_unlock(rwsem);
            return ret;
        }

        reg = SDW_SDCA_CTL(
            (*(*(*interrupt).function).desc).adr,
            (*(*interrupt).entity).id,
            SDCA_CTL_GE_SELECTED_MODE,
            0,
        );

        match val {
            SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS | SDCA_DETECTED_MODE_JACK_UNKNOWN => {
                /*
                 * Selected mode is not normally marked as volatile register
                 * (RW), but here force a read from the hardware. If the
                 * detected mode is unknown we need to see what the device
                 * selected as a "safe" option.
                 */
                regcache_drop_region((*interrupt).function_regmap, reg, reg);

                ret = regmap_read((*interrupt).function_regmap, reg, &mut val);
                if ret != 0 {
                    dev_err(dev, c"failed to re-check selected mode: %d\n".as_ptr(), ret);
                    rwsem_write_unlock(rwsem);
                    return ret;
                }
            }
            _ => {}
        }

        dev_dbg(dev, c"%s: %#x\n".as_ptr(), (*interrupt).name, val);

        ucontrol = kzalloc(size_of::<snd_ctl_elem_value>(), GFP_KERNEL) as *mut snd_ctl_elem_value;
        if ucontrol.is_null() {
            rwsem_write_unlock(rwsem);
            return -ENOMEM;
        }

        soc_enum = (*kctl).private_value as *mut soc_enum;
        (*ucontrol).value.enumerated.item[0] = snd_soc_enum_val_to_item(soc_enum, val);

        ret = snd_soc_dapm_put_enum_double(kctl, ucontrol);
        if ret < 0 {
            dev_err(dev, c"failed to update selected mode: %d\n".as_ptr(), ret);
            kfree(ucontrol as *const c_void);
            rwsem_write_unlock(rwsem);
            return ret;
        }

        snd_ctl_notify((*card).snd_card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id);

        kfree(ucontrol as *const c_void);
        ret = sdca_jack_report(interrupt);
        rwsem_write_unlock(rwsem);
        ret
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_process, "SND_SOC_SDCA");

/**
 * sdca_jack_alloc_state - allocate state for a jack interrupt
 * @interrupt: SDCA interrupt structure.
 *
 * Return: Zero on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_alloc_state(interrupt: *mut sdca_interrupt) -> c_int {
    unsafe {
        let jack_state = kzalloc(size_of::<jack_state>(), GFP_KERNEL) as *mut jack_state;

        if jack_state.is_null() {
            return -ENOMEM;
        }

        (*interrupt).priv = jack_state as *mut c_void;

        0
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_alloc_state, "SND_SOC_SDCA");

/**
 * sdca_jack_free_state - free state for a jack interrupt
 * @interrupt: SDCA interrupt structure.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_free_state(interrupt: *mut sdca_interrupt) {
    unsafe {
        kfree((*interrupt).priv as *const c_void);
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_free_state, "SND_SOC_SDCA");

/**
 * sdca_jack_init_state - Initialise transient state for a jack interrupt
 * @interrupt: SDCA interrupt structure.
 *
 * Return: Zero on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_init_state(interrupt: *mut sdca_interrupt) -> c_int {
    unsafe {
        let jack_state = (*interrupt).priv as *mut jack_state;
        let name = kasprintf(
            GFP_KERNEL,
            c"%s %s".as_ptr(),
            (*(*interrupt).entity).label,
            SDCA_CTL_SELECTED_MODE_NAME,
        );

        if name.is_null() {
            return -ENOMEM;
        }

        (*jack_state).kctl = snd_soc_component_get_kcontrol((*interrupt).component, name);
        if (*jack_state).kctl.is_null() {
            dev_err((*interrupt).dev, c"control not found: %s\n".as_ptr(), name);
            kfree(name as *const c_void);
            return -ENODEV;
        }

        kfree(name as *const c_void);
        0
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_init_state, "SND_SOC_SDCA");

unsafe fn type_get_mask(type_: sdca_terminal_type) -> c_int {
    match type_ {
        SDCA_TERM_TYPE_LINEIN_STEREO
        | SDCA_TERM_TYPE_LINEIN_FRONT_LR
        | SDCA_TERM_TYPE_LINEIN_CENTER_LFE
        | SDCA_TERM_TYPE_LINEIN_SURROUND_LR
        | SDCA_TERM_TYPE_LINEIN_REAR_LR => SND_JACK_LINEIN,
        SDCA_TERM_TYPE_LINEOUT_STEREO
        | SDCA_TERM_TYPE_LINEOUT_FRONT_LR
        | SDCA_TERM_TYPE_LINEOUT_CENTER_LFE
        | SDCA_TERM_TYPE_LINEOUT_SURROUND_LR
        | SDCA_TERM_TYPE_LINEOUT_REAR_LR => SND_JACK_LINEOUT,
        SDCA_TERM_TYPE_MIC_JACK => SND_JACK_MICROPHONE,
        SDCA_TERM_TYPE_HEADPHONE_JACK => SND_JACK_HEADPHONE,
        SDCA_TERM_TYPE_HEADSET_JACK => SND_JACK_HEADSET,
        _ => 0,
    }
}

/**
 * sdca_jack_set_jack - attach an ASoC jack to SDCA
 * @info: SDCA interrupt information.
 * @jack: ASoC jack to be attached.
 *
 * Return: Zero on success or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_set_jack(
    info: *mut sdca_interrupt_info,
    jack: *mut snd_soc_jack,
) -> c_int {
    unsafe {
        let mut i: c_int;
        let mut j: c_int;
        let mut ret: c_int;

        // guard(mutex)(&info->irq_lock);
        mutex_lock(&mut (*info).irq_lock);

        i = 0;
        while i < SDCA_MAX_INTERRUPTS {
            let interrupt = &mut (*info).irqs[i as usize] as *mut sdca_interrupt;
            let control = (*interrupt).control;
            let entity = (*interrupt).entity;
            let mut range: *mut sdca_control_range;
            let jack_state: *mut jack_state;

            if (*interrupt).dev.is_null() {
                i += 1;
                continue;
            }

            match SDCA_CTL_TYPE((*entity).type_, (*control).sel) {
                x if x == SDCA_CTL_TYPE_S(GE, DETECTED_MODE) => {
                    range = sdca_selector_find_range(
                        (*interrupt).dev,
                        entity,
                        SDCA_CTL_GE_SELECTED_MODE,
                        SDCA_SELECTED_MODE_NCOLS,
                        0,
                    );
                    if range.is_null() {
                        mutex_unlock(&mut (*info).irq_lock);
                        return -EINVAL;
                    }

                    jack_state = (*interrupt).priv as *mut jack_state;

                    j = 0;
                    while j < (*range).rows {
                        let type_: sdca_terminal_type;

                        type_ = sdca_range(range, SDCA_SELECTED_MODE_TERM_TYPE, j);

                        (*jack_state).mask |= type_get_mask(type_);
                        j += 1;
                    }

                    (*jack_state).jack = jack;

                    /* Report initial state in case IRQ was already handled */
                    ret = sdca_jack_report(interrupt);
                    if ret != 0 {
                        mutex_unlock(&mut (*info).irq_lock);
                        return ret;
                    }
                }
                _ => {}
            }

            i += 1;
        }

        mutex_unlock(&mut (*info).irq_lock);
        0
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_set_jack, "SND_SOC_SDCA");

#[no_mangle]
pub unsafe extern "C" fn sdca_jack_report(interrupt: *mut sdca_interrupt) -> c_int {
    unsafe {
        let jack_state = (*interrupt).priv as *mut jack_state;
        let mut range: *mut sdca_control_range;
        let mut type_: sdca_terminal_type;
        let mut reg: c_uint;
        let mut val: c_uint = 0;
        let mut ret: c_int;

        reg = SDW_SDCA_CTL(
            (*(*(*interrupt).function).desc).adr,
            (*(*interrupt).entity).id,
            SDCA_CTL_GE_SELECTED_MODE,
            0,
        );

        ret = regmap_read((*interrupt).function_regmap, reg, &mut val);
        if ret != 0 {
            dev_err((*interrupt).dev, c"failed to read selected mode: %d\n".as_ptr(), ret);
            return ret;
        }

        range = sdca_selector_find_range(
            (*interrupt).dev,
            (*interrupt).entity,
            SDCA_CTL_GE_SELECTED_MODE,
            SDCA_SELECTED_MODE_NCOLS,
            0,
        );
        if range.is_null() {
            return -EINVAL;
        }

        type_ = sdca_range_search(
            range,
            SDCA_SELECTED_MODE_INDEX,
            val,
            SDCA_SELECTED_MODE_TERM_TYPE,
        );

        snd_soc_jack_report((*jack_state).jack, type_get_mask(type_), (*jack_state).mask);

        0
    }
}
// EXPORT_SYMBOL_NS_GPL(sdca_jack_report, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
