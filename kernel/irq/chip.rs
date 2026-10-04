// SPDX-License-Identifier: GPL-2.0
// Chip and flow-handler policy from chip.c at be59db382996.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_pub,
    clippy::all
)]
include!("irq_core.rs");

unsafe extern "C" fn bad_chained_irq(irq: c_int, _: *mut c_void) -> irqreturn_t {
    lupos_irq_warn_chained(irq);
    IRQ_NONE
}
#[no_mangle]
pub static mut chained_action: irqaction = irqaction {
    handler: Some(bad_chained_irq),
    ..unsafe { zeroed() }
};
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip(irq: c_uint, chip: *const irq_chip) -> c_int {
    let mut ret = -(EINVAL as c_int);
    if let Some(lock) = BusLock::get(irq, 0, false) {
        (*lock.desc).irq_data.chip = if chip.is_null() {
            addr_of_mut!(no_irq_chip)
        } else {
            chip.cast_mut()
        };
        ret = 0;
    }
    if ret == 0 {
        #[cfg(not(CONFIG_SPARSE_IRQ))]
        irq_mark_irq(irq);
        lupos_irq_proc_chip(chip);
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_irq_type(irq: c_uint, ty: c_uint) -> c_int {
    if let Some(lock) = BusLock::get(irq, IRQ_GET_DESC_CHECK_GLOBAL, true) {
        return __irq_set_trigger(lock.desc, ty as c_ulong);
    }
    -(EINVAL as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_handler_data(irq: c_uint, value: *mut c_void) -> c_int {
    if let Some(lock) = BusLock::get(irq, 0, false) {
        (*lock.desc).irq_common_data.handler_data = value;
        return 0;
    }
    -(EINVAL as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_msi_desc_off(
    base: c_uint,
    offset: c_uint,
    entry: *mut msi_desc,
) -> c_int {
    if let Some(lock) = BusLock::get(base.wrapping_add(offset), IRQ_GET_DESC_CHECK_GLOBAL, false) {
        (*lock.desc).irq_common_data.msi_desc = entry;
        if !entry.is_null() && offset == 0 {
            (*entry).irq = base;
        }
        return 0;
    }
    -(EINVAL as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_msi_desc(irq: c_uint, entry: *mut msi_desc) -> c_int {
    irq_set_msi_desc_off(irq, 0, entry)
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip_data(irq: c_uint, value: *mut c_void) -> c_int {
    if let Some(lock) = BusLock::get(irq, 0, false) {
        (*lock.desc).irq_data.chip_data = value;
        return 0;
    }
    -(EINVAL as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn irq_get_irq_data(irq: c_uint) -> *mut irq_data {
    let desc = irq_to_desc(irq);
    if desc.is_null() {
        null_mut()
    } else {
        data(desc)
    }
}
unsafe fn irq_state_clr_disabled(desc: *mut irq_desc) {
    lupos_irq_data_clear(data(desc), IRQD_IRQ_DISABLED);
}
unsafe fn irq_state_clr_masked(desc: *mut irq_desc) {
    lupos_irq_data_clear(data(desc), IRQD_IRQ_MASKED);
}
unsafe fn irq_state_clr_started(desc: *mut irq_desc) {
    lupos_irq_data_clear(data(desc), IRQD_IRQ_STARTED);
}
unsafe fn irq_state_set_started(desc: *mut irq_desc) {
    lupos_irq_data_set(data(desc), IRQD_IRQ_STARTED);
}
unsafe fn irq_state_set_disabled(desc: *mut irq_desc) {
    lupos_irq_data_set(data(desc), IRQD_IRQ_DISABLED);
}
unsafe fn irq_state_set_masked(desc: *mut irq_desc) {
    lupos_irq_data_set(data(desc), IRQD_IRQ_MASKED);
}
const IRQ_STARTUP_NORMAL: c_int = 0;
const IRQ_STARTUP_MANAGED: c_int = 1;
const IRQ_STARTUP_ABORT: c_int = 2;
#[cfg(CONFIG_SMP)]
unsafe fn __irq_startup_managed(desc: *mut irq_desc, aff: *const cpumask, force: bool) -> c_int {
    let d = data(desc);
    if !lupos_irq_data_has(d, IRQD_AFFINITY_MANAGED) {
        return IRQ_STARTUP_NORMAL;
    }
    lupos_irq_data_clear(d, IRQD_MANAGED_SHUTDOWN);
    if !lupos_irq_mask_intersects(aff, lupos_irq_online_mask()) {
        lupos_irq_warn_managed_force(force);
        return IRQ_STARTUP_ABORT;
    }
    if lupos_irq_warn_managed_activate(lupos_irq_activate(d, false)) {
        return IRQ_STARTUP_ABORT;
    }
    IRQ_STARTUP_MANAGED
}
#[cfg(not(CONFIG_SMP))]
unsafe fn __irq_startup_managed(_: *mut irq_desc, _: *const cpumask, _: bool) -> c_int {
    IRQ_STARTUP_NORMAL
}
#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn irq_startup_managed(desc: *mut irq_desc) {
    lupos_irq_data_clear(data(desc), IRQD_MANAGED_SHUTDOWN);
    (*desc).depth = (*desc).depth.wrapping_sub(1);
    if (*desc).depth == 0 {
        irq_startup(desc, true, false);
    }
}
unsafe fn irq_enable(desc: *mut irq_desc) {
    if !disabled(desc) {
        unmask_irq(desc);
    } else {
        irq_state_clr_disabled(desc);
        if let Some(enable) = (*(*desc).irq_data.chip).irq_enable {
            enable(data(desc));
            irq_state_clr_masked(desc);
        } else {
            unmask_irq(desc);
        }
    }
}
unsafe fn __irq_startup(desc: *mut irq_desc) -> c_int {
    let d = data(desc);
    let mut ret = 0;
    lupos_irq_warn_not_activated(!lupos_irq_data_has(d, IRQD_ACTIVATED));
    if let Some(startup) = (*(*d).chip).irq_startup {
        ret = startup(d) as c_int;
        irq_state_clr_disabled(desc);
        irq_state_clr_masked(desc);
    } else {
        irq_enable(desc);
    }
    irq_state_set_started(desc);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn irq_startup(desc: *mut irq_desc, resend: bool, force: bool) -> c_int {
    let d = data(desc);
    let aff = lupos_irq_affinity(d);
    let mut ret = 0;
    (*desc).depth = 0;
    if lupos_irq_data_has(d, IRQD_IRQ_STARTED) {
        irq_enable(desc);
    } else {
        match __irq_startup_managed(desc, aff, force) {
            IRQ_STARTUP_NORMAL => {
                if (*(*d).chip).flags & IRQCHIP_AFFINITY_PRE_STARTUP as c_ulong != 0 {
                    lupos_irq_setup_affinity(desc);
                }
                ret = __irq_startup(desc);
                if (*(*d).chip).flags & IRQCHIP_AFFINITY_PRE_STARTUP as c_ulong == 0 {
                    lupos_irq_setup_affinity(desc);
                }
            }
            IRQ_STARTUP_MANAGED => {
                irq_do_set_affinity(d, aff, false);
                ret = __irq_startup(desc);
            }
            IRQ_STARTUP_ABORT => {
                (*desc).depth = 1;
                lupos_irq_data_set(d, IRQD_MANAGED_SHUTDOWN);
                return 0;
            }
            _ => {}
        }
    }
    if resend {
        check_irq_resend(desc, false);
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn irq_activate(desc: *mut irq_desc) -> c_int {
    let d = data(desc);
    if !lupos_irq_data_has(d, IRQD_AFFINITY_MANAGED) {
        lupos_irq_activate(d, false)
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_activate_and_startup(desc: *mut irq_desc, resend: bool) -> c_int {
    if lupos_irq_warn_activate(irq_activate(desc)) {
        return 0;
    }
    irq_startup(desc, resend, true)
}
#[no_mangle]
pub unsafe extern "C" fn irq_shutdown(desc: *mut irq_desc) {
    if lupos_irq_data_has(data(desc), IRQD_IRQ_STARTED) {
        clear_irq_resend(desc);
        (*desc).depth = (*desc).depth.wrapping_add(1);
        if let Some(shutdown) = (*(*desc).irq_data.chip).irq_shutdown {
            shutdown(data(desc));
            irq_state_set_disabled(desc);
            irq_state_set_masked(desc);
        } else {
            __irq_disable(desc, true);
        }
        irq_state_clr_started(desc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_shutdown_and_deactivate(desc: *mut irq_desc) {
    irq_shutdown(desc);
    lupos_irq_deactivate(data(desc));
}
unsafe fn __irq_disable(desc: *mut irq_desc, mask: bool) {
    if disabled(desc) {
        if mask {
            mask_irq(desc);
        }
    } else {
        irq_state_set_disabled(desc);
        if let Some(disable) = (*(*desc).irq_data.chip).irq_disable {
            disable(data(desc));
            irq_state_set_masked(desc);
        } else if mask {
            mask_irq(desc);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_disable(desc: *mut irq_desc) {
    __irq_disable(desc, setting(desc, _IRQ_DISABLE_UNLAZY));
}
#[no_mangle]
pub unsafe extern "C" fn irq_percpu_enable(desc: *mut irq_desc, cpu: c_uint) {
    let chip = (*desc).irq_data.chip;
    if let Some(enable) = (*chip).irq_enable {
        enable(data(desc));
    } else {
        ((*chip).irq_unmask.unwrap_unchecked())(data(desc));
    }
    lupos_irq_mask_set_cpu(cpu, (*desc).percpu_enabled);
}
#[no_mangle]
pub unsafe extern "C" fn irq_percpu_disable(desc: *mut irq_desc, cpu: c_uint) {
    let chip = (*desc).irq_data.chip;
    if let Some(disable) = (*chip).irq_disable {
        disable(data(desc));
    } else {
        ((*chip).irq_mask.unwrap_unchecked())(data(desc));
    }
    lupos_irq_mask_clear_cpu(cpu, (*desc).percpu_enabled);
}
unsafe fn mask_ack_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    if let Some(mask_ack) = (*chip).irq_mask_ack {
        mask_ack(data(desc));
        irq_state_set_masked(desc);
    } else {
        mask_irq(desc);
        if let Some(ack) = (*chip).irq_ack {
            ack(data(desc));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mask_irq(desc: *mut irq_desc) {
    if masked(desc) {
        return;
    }
    if let Some(mask) = (*(*desc).irq_data.chip).irq_mask {
        mask(data(desc));
        irq_state_set_masked(desc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unmask_irq(desc: *mut irq_desc) {
    if !masked(desc) {
        return;
    }
    if let Some(unmask) = (*(*desc).irq_data.chip).irq_unmask {
        unmask(data(desc));
        irq_state_clr_masked(desc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unmask_threaded_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    if (*chip).flags & IRQCHIP_EOI_THREADED as c_ulong != 0 {
        ((*chip).irq_eoi.unwrap_unchecked())(data(desc));
    }
    unmask_irq(desc);
}
unsafe fn irq_wait_on_inprogress(desc: *mut irq_desc) -> bool {
    #[cfg(CONFIG_SMP)]
    {
        loop {
            lupos_irq_raw_unlock(addr_of_mut!((*desc).lock));
            while lupos_irq_data_has(data(desc), IRQD_IRQ_INPROGRESS) {
                lupos_irq_relax();
            }
            lupos_irq_raw_lock(addr_of_mut!((*desc).lock));
            if !lupos_irq_data_has(data(desc), IRQD_IRQ_INPROGRESS) {
                break;
            }
        }
        return !disabled(desc) && !(*desc).action.is_null();
    }
    #[cfg(not(CONFIG_SMP))]
    {
        false
    }
}
unsafe fn irq_can_handle_pm(desc: *mut irq_desc) -> bool {
    let d = data(desc);
    if !lupos_irq_data_has(d, IRQD_IRQ_INPROGRESS | IRQD_WAKEUP_ARMED) {
        return true;
    }
    if lupos_irq_data_has(d, IRQD_WAKEUP_ARMED) {
        lupos_irq_pm_wakeup(desc);
        return false;
    }
    if *state(desc) & IRQS_POLL_INPROGRESS != 0 {
        let cpu = lupos_irq_cpu();
        if lupos_irq_warn_poll(irq_poll_cpu == cpu as c_int, cpu, (*d).irq) {
            return false;
        }
        return irq_wait_on_inprogress(desc);
    }
    if !cfg!(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)
        || !lupos_irq_data_has(d, IRQD_SINGLE_TARGET)
        || (*desc).handle_irq.map(|f| f as usize) != Some(handle_edge_irq as *const () as usize)
    {
        return false;
    }
    if lupos_irq_mask_first(lupos_irq_effective_affinity(d)) != lupos_irq_cpu() {
        return false;
    }
    irq_wait_on_inprogress(desc)
}
unsafe fn irq_can_handle_actions(desc: *mut irq_desc) -> bool {
    *state(desc) &= !(IRQS_REPLAY | IRQS_WAITING);
    if (*desc).action.is_null() || disabled(desc) {
        *state(desc) |= IRQS_PENDING;
        return false;
    }
    true
}
unsafe fn irq_can_handle(desc: *mut irq_desc) -> bool {
    irq_can_handle_pm(desc) && irq_can_handle_actions(desc)
}
#[no_mangle]
pub unsafe extern "C" fn handle_nested_irq(irq: c_uint) {
    let desc = irq_to_desc(irq);
    lupos_irq_might_sleep();
    {
        let _lock = DescLock::irq(desc);
        if !irq_can_handle_actions(desc) {
            return;
        }
        lupos_irq_kstat_incr(desc);
        lupos_irq_atomic_inc(addr_of_mut!((*desc).threads_active));
    }
    let mut action_ret = IRQ_NONE;
    let mut action = (*desc).action;
    while !action.is_null() {
        action_ret.0 |=
            ((*action).thread_fn.unwrap_unchecked())((*action).irq as c_int, (*action).dev_id).0;
        action = (*action).next;
    }
    if !lupos_irq_no_debug(desc) {
        note_interrupt(desc, action_ret);
    }
    wake_threads_waitq(desc);
}
#[no_mangle]
pub unsafe extern "C" fn handle_simple_irq(desc: *mut irq_desc) {
    let _lock = DescLock::raw(desc);
    if !irq_can_handle_pm(desc) {
        if lupos_irq_data_has(data(desc), IRQD_RESEND_WHEN_IN_PROGRESS) {
            *state(desc) |= IRQS_PENDING;
        }
        return;
    }
    if !irq_can_handle_actions(desc) {
        return;
    }
    lupos_irq_kstat_incr(desc);
    handle_irq_event(desc);
}
#[no_mangle]
pub unsafe extern "C" fn handle_untracked_irq(desc: *mut irq_desc) {
    {
        let _lock = DescLock::raw(desc);
        if !irq_can_handle(desc) {
            return;
        }
        *state(desc) &= !IRQS_PENDING;
        lupos_irq_data_set(data(desc), IRQD_IRQ_INPROGRESS);
    }
    __handle_irq_event_percpu(desc);
    let _lock = DescLock::raw(desc);
    lupos_irq_data_clear(data(desc), IRQD_IRQ_INPROGRESS);
}
unsafe fn cond_unmask_irq(desc: *mut irq_desc) {
    if !disabled(desc) && masked(desc) && (*desc).threads_oneshot == 0 {
        unmask_irq(desc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_level_irq(desc: *mut irq_desc) {
    let _lock = DescLock::raw(desc);
    mask_ack_irq(desc);
    if !irq_can_handle(desc) {
        return;
    }
    lupos_irq_kstat_incr(desc);
    handle_irq_event(desc);
    cond_unmask_irq(desc);
}
unsafe fn cond_unmask_eoi_irq(desc: *mut irq_desc, chip: *mut irq_chip) {
    if *state(desc) & IRQS_ONESHOT == 0 {
        ((*chip).irq_eoi.unwrap_unchecked())(data(desc));
        return;
    }
    if !disabled(desc) && masked(desc) && (*desc).threads_oneshot == 0 {
        ((*chip).irq_eoi.unwrap_unchecked())(data(desc));
        unmask_irq(desc);
    } else if (*chip).flags & IRQCHIP_EOI_THREADED as c_ulong == 0 {
        ((*chip).irq_eoi.unwrap_unchecked())(data(desc));
    }
}
unsafe fn cond_eoi_irq(chip: *mut irq_chip, d: *mut irq_data) {
    if (*chip).flags & IRQCHIP_EOI_IF_HANDLED as c_ulong == 0 {
        ((*chip).irq_eoi.unwrap_unchecked())(d);
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    let _lock = DescLock::raw(desc);
    if !irq_can_handle_pm(desc) {
        if lupos_irq_data_has(data(desc), IRQD_RESEND_WHEN_IN_PROGRESS) {
            *state(desc) |= IRQS_PENDING;
        }
        cond_eoi_irq(chip, data(desc));
        return;
    }
    if !irq_can_handle_actions(desc) {
        mask_irq(desc);
        cond_eoi_irq(chip, data(desc));
        return;
    }
    lupos_irq_kstat_incr(desc);
    if *state(desc) & IRQS_ONESHOT != 0 {
        mask_irq(desc);
    }
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
    if *state(desc) & IRQS_PENDING != 0 {
        check_irq_resend(desc, false);
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_nmi(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    let action = (*desc).action;
    let irq = (*desc).irq_data.irq;
    lupos_irq_kstat_incr_raw(desc);
    lupos_irq_trace_entry(irq, action);
    let res = ((*action).handler.unwrap_unchecked())(irq as c_int, (*action).dev_id);
    lupos_irq_trace_exit(irq, action, res);
    if let Some(eoi) = (*chip).irq_eoi {
        eoi(data(desc));
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_edge_irq(desc: *mut irq_desc) {
    let _lock = DescLock::raw(desc);
    if !irq_can_handle(desc) {
        *state(desc) |= IRQS_PENDING;
        mask_ack_irq(desc);
        return;
    }
    lupos_irq_kstat_incr(desc);
    ((*(*desc).irq_data.chip).irq_ack.unwrap_unchecked())(data(desc));
    loop {
        if (*desc).action.is_null() {
            mask_irq(desc);
            return;
        }
        if *state(desc) & IRQS_PENDING != 0 && !disabled(desc) && masked(desc) {
            unmask_irq(desc);
        }
        handle_irq_event(desc);
        if *state(desc) & IRQS_PENDING == 0 || disabled(desc) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_percpu_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    lupos_irq_kstat_incr_raw(desc);
    if let Some(ack) = (*chip).irq_ack {
        ack(data(desc));
    }
    handle_irq_event_percpu(desc);
    if let Some(eoi) = (*chip).irq_eoi {
        eoi(data(desc));
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_percpu_devid_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    let irq = (*desc).irq_data.irq;
    let cpu = lupos_irq_cpu();
    lupos_irq_kstat_incr_raw(desc);
    if let Some(ack) = (*chip).irq_ack {
        ack(data(desc));
    }
    let mut action = (*desc).action;
    while !action.is_null() {
        if lupos_irq_mask_test(cpu, (*action).affinity) {
            break;
        }
        action = (*action).next;
    }
    if !action.is_null() {
        lupos_irq_trace_entry(irq, action);
        let res = ((*action).handler.unwrap_unchecked())(
            irq as c_int,
            lupos_irq_raw_cpu_ptr((*action).percpu_dev_id),
        );
        lupos_irq_trace_exit(irq, action, res);
    } else {
        let enabled = lupos_irq_mask_test(cpu, (*desc).percpu_enabled);
        if enabled {
            irq_percpu_disable(desc, cpu);
        }
        lupos_irq_warn_spurious_percpu(enabled, irq, cpu);
    }
    if !lupos_irq_in_nmi() {
        add_interrupt_randomness(irq as c_int);
    }
    if let Some(eoi) = (*chip).irq_eoi {
        eoi(data(desc));
    }
}
unsafe fn __irq_do_set_handler(
    desc: *mut irq_desc,
    mut handle: irq_flow_handler_t,
    chained: c_int,
    name: *const c_char,
) {
    if handle.is_none() {
        handle = Some(handle_bad_irq);
    } else {
        let mut d = data(desc);
        #[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
        while !d.is_null() {
            if (*d).chip != addr_of_mut!(no_irq_chip) {
                break;
            }
            if chained != 0 {
                lupos_irq_warn_chained_outer(true);
                return;
            }
            d = (*d).parent_data;
        }
        if d.is_null() || (*d).chip == addr_of_mut!(no_irq_chip) {
            lupos_irq_warn_no_chip(true);
            return;
        }
    }
    let bad = handle.map(|f| f as usize) == Some(handle_bad_irq as *const () as usize);
    if bad {
        if (*desc).irq_data.chip != addr_of_mut!(no_irq_chip) {
            mask_ack_irq(desc);
        }
        irq_state_set_disabled(desc);
        if chained != 0 {
            (*desc).action = null_mut();
            irq_chip_pm_put(data(desc));
        }
        (*desc).depth = 1;
    }
    (*desc).handle_irq = handle;
    (*desc).name = name;
    if !bad && chained != 0 {
        let ty = (*(*data(desc)).common).state_use_accessors & IRQD_TRIGGER_MASK;
        if ty != IRQ_TYPE_NONE {
            __irq_set_trigger(desc, ty as c_ulong);
            (*desc).handle_irq = handle;
        }
        (*desc).status_use_accessors |= _IRQ_NOPROBE | _IRQ_NOREQUEST | _IRQ_NOTHREAD;
        (*desc).action = addr_of_mut!(chained_action);
        lupos_irq_warn_pm_get(irq_chip_pm_get(data(desc)));
        irq_activate_and_startup(desc, true);
    }
    lupos_irq_proc_valid(desc);
}
#[no_mangle]
pub unsafe extern "C" fn __irq_set_handler(
    irq: c_uint,
    handle: irq_flow_handler_t,
    chained: c_int,
    name: *const c_char,
) {
    if let Some(lock) = BusLock::get(irq, 0, true) {
        __irq_do_set_handler(lock.desc, handle, chained, name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_chained_handler_and_data(
    irq: c_uint,
    handle: irq_flow_handler_t,
    value: *mut c_void,
) {
    if let Some(lock) = BusLock::get(irq, 0, true) {
        (*lock.desc).irq_common_data.handler_data = value;
        __irq_do_set_handler(lock.desc, handle, 1, null());
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip_and_handler_name(
    irq: c_uint,
    chip: *const irq_chip,
    handle: irq_flow_handler_t,
    name: *const c_char,
) {
    irq_set_chip(irq, chip);
    __irq_set_handler(irq, handle, 0, name);
}
#[no_mangle]
pub unsafe extern "C" fn irq_modify_status(irq: c_uint, clr: c_ulong, set: c_ulong) {
    if let Some(lock) = BusLock::get(irq, 0, false) {
        let desc = lock.desc;
        let d = data(desc);
        lupos_irq_warn_autoenable((*desc).depth == 0 && set & _IRQ_NOAUTOEN as c_ulong != 0);
        settings(desc, clr as c_uint, set as c_uint);
        let mut trigger = (*(*d).common).state_use_accessors & IRQD_TRIGGER_MASK;
        lupos_irq_data_clear(
            d,
            IRQD_NO_BALANCING | IRQD_PER_CPU | IRQD_TRIGGER_MASK | IRQD_LEVEL,
        );
        if setting(desc, _IRQ_NO_BALANCING) {
            lupos_irq_data_set(d, IRQD_NO_BALANCING);
        }
        if setting(desc, _IRQ_PER_CPU) {
            lupos_irq_data_set(d, IRQD_PER_CPU);
        }
        if setting(desc, _IRQ_LEVEL) {
            lupos_irq_data_set(d, IRQD_LEVEL);
        }
        let tmp = (*desc).status_use_accessors & IRQ_TYPE_SENSE_MASK;
        if tmp != IRQ_TYPE_NONE {
            trigger = tmp;
        }
        lupos_irq_data_set(d, trigger);
        lupos_irq_proc_valid(desc);
    }
}
#[cfg(CONFIG_DEPRECATED_IRQ_CPU_ONOFFLINE)]
#[no_mangle]
pub unsafe extern "C" fn irq_cpu_online() {
    let count = irq_get_nr_irqs();
    let mut irq = irq_get_next_irq(0);
    while irq < count {
        let desc = irq_to_desc(irq);
        if !desc.is_null() {
            let _lock = DescLock::save(desc);
            let chip = (*desc).irq_data.chip;
            if !chip.is_null()
                && ((*chip).flags & IRQCHIP_ONOFFLINE_ENABLED as c_ulong == 0 || !disabled(desc))
            {
                if let Some(online) = (*chip).irq_cpu_online {
                    online(data(desc));
                }
            }
        }
        irq = irq_get_next_irq(irq.wrapping_add(1));
    }
}
#[cfg(CONFIG_DEPRECATED_IRQ_CPU_ONOFFLINE)]
#[no_mangle]
pub unsafe extern "C" fn irq_cpu_offline() {
    let count = irq_get_nr_irqs();
    let mut irq = irq_get_next_irq(0);
    while irq < count {
        let desc = irq_to_desc(irq);
        if !desc.is_null() {
            let _lock = DescLock::save(desc);
            let chip = (*desc).irq_data.chip;
            if !chip.is_null()
                && ((*chip).flags & IRQCHIP_ONOFFLINE_ENABLED as c_ulong == 0 || !disabled(desc))
            {
                if let Some(offline) = (*chip).irq_cpu_offline {
                    offline(data(desc));
                }
            }
        }
        irq = irq_get_next_irq(irq.wrapping_add(1));
    }
}
#[cfg(all(CONFIG_IRQ_DOMAIN_HIERARCHY, CONFIG_IRQ_FASTEOI_HIERARCHY_HANDLERS))]
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_ack_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    let _lock = DescLock::raw(desc);
    if !irq_can_handle_pm(desc) {
        cond_eoi_irq(chip, data(desc));
        return;
    }
    if !irq_can_handle_actions(desc) {
        mask_irq(desc);
        cond_eoi_irq(chip, data(desc));
        return;
    }
    lupos_irq_kstat_incr(desc);
    if *state(desc) & IRQS_ONESHOT != 0 {
        mask_irq(desc);
    }
    ((*chip).irq_ack.unwrap_unchecked())(data(desc));
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
}
#[cfg(all(CONFIG_IRQ_DOMAIN_HIERARCHY, CONFIG_IRQ_FASTEOI_HIERARCHY_HANDLERS))]
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_mask_irq(desc: *mut irq_desc) {
    let chip = (*desc).irq_data.chip;
    let _lock = DescLock::raw(desc);
    mask_ack_irq(desc);
    if !irq_can_handle(desc) {
        cond_eoi_irq(chip, data(desc));
        return;
    }
    lupos_irq_kstat_incr(desc);
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
}
#[cfg(all(CONFIG_IRQ_DOMAIN_HIERARCHY, CONFIG_SMP))]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_pre_redirect_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_pre_redirect.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_parent_state(
    d: *mut irq_data,
    which: irqchip_irq_state,
    value: bool,
) -> c_int {
    let parent = (*d).parent_data;
    if parent.is_null() {
        return 0;
    }
    if let Some(set) = (*(*parent).chip).irq_set_irqchip_state {
        set(parent, which, value)
    } else {
        0
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_get_parent_state(
    d: *mut irq_data,
    which: irqchip_irq_state,
    state: *mut bool,
) -> c_int {
    let parent = (*d).parent_data;
    if parent.is_null() {
        return 0;
    }
    if let Some(get) = (*(*parent).chip).irq_get_irqchip_state {
        get(parent, which, state)
    } else {
        0
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_shutdown_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    if let Some(shutdown) = (*(*parent).chip).irq_shutdown {
        shutdown(parent);
    } else {
        irq_chip_disable_parent(d);
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_startup_parent(d: *mut irq_data) -> c_uint {
    let parent = (*d).parent_data;
    if let Some(startup) = (*(*parent).chip).irq_startup {
        return startup(parent);
    }
    irq_chip_enable_parent(d);
    0
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_enable_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    if let Some(enable) = (*(*parent).chip).irq_enable {
        enable(parent);
    } else {
        ((*(*parent).chip).irq_unmask.unwrap_unchecked())(parent);
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_disable_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    if let Some(disable) = (*(*parent).chip).irq_disable {
        disable(parent);
    } else {
        ((*(*parent).chip).irq_mask.unwrap_unchecked())(parent);
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_ack_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_ack.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_mask_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_mask.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_mask_ack_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_mask_ack.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_unmask_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_unmask.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_eoi_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    ((*(*parent).chip).irq_eoi.unwrap_unchecked())(parent);
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_affinity_parent(
    d: *mut irq_data,
    dest: *const cpumask,
    force: bool,
) -> c_int {
    let parent = (*d).parent_data;
    if let Some(set) = (*(*parent).chip).irq_set_affinity {
        set(parent, dest, force)
    } else {
        -(ENOSYS as c_int)
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_type_parent(d: *mut irq_data, ty: c_uint) -> c_int {
    let parent = (*d).parent_data;
    if let Some(set) = (*(*parent).chip).irq_set_type {
        set(parent, ty)
    } else {
        -(ENOSYS as c_int)
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_retrigger_hierarchy(d: *mut irq_data) -> c_int {
    let mut parent = (*d).parent_data;
    while !parent.is_null() {
        if !(*parent).chip.is_null() {
            if let Some(retrigger) = (*(*parent).chip).irq_retrigger {
                return retrigger(parent);
            }
        }
        parent = (*parent).parent_data;
    }
    0
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_vcpu_affinity_parent(
    d: *mut irq_data,
    info: *mut c_void,
) -> c_int {
    let parent = (*d).parent_data;
    if let Some(set) = (*(*parent).chip).irq_set_vcpu_affinity {
        set(parent, info)
    } else {
        -(ENOSYS as c_int)
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_wake_parent(d: *mut irq_data, on: c_uint) -> c_int {
    let parent = (*d).parent_data;
    if (*(*parent).chip).flags & IRQCHIP_SKIP_SET_WAKE as c_ulong != 0 {
        return 0;
    }
    if let Some(set) = (*(*parent).chip).irq_set_wake {
        set(parent, on)
    } else {
        -(ENOSYS as c_int)
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_request_resources_parent(d: *mut irq_data) -> c_int {
    let parent = (*d).parent_data;
    if let Some(request) = (*(*parent).chip).irq_request_resources {
        request(parent)
    } else {
        0
    }
}
#[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_release_resources_parent(d: *mut irq_data) {
    let parent = (*d).parent_data;
    if let Some(release) = (*(*parent).chip).irq_release_resources {
        release(parent);
    }
}
#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn irq_chip_redirect_set_affinity(
    d: *mut irq_data,
    dest: *const cpumask,
    _: bool,
) -> c_int {
    let desc: *mut irq_desc = (*d)
        .common
        .cast::<u8>()
        .sub(offset_of!(irq_desc, irq_common_data))
        .cast();
    lupos_irq_target_write(
        addr_of_mut!((*desc).redirect.target_cpu),
        lupos_irq_mask_first(dest),
    );
    lupos_irq_effective_update(d, dest);
    IRQ_SET_MASK_OK_DONE as c_int
}
#[no_mangle]
pub unsafe extern "C" fn irq_chip_compose_msi_msg(
    mut d: *mut irq_data,
    msg: *mut msi_msg,
) -> c_int {
    let mut pos: *mut irq_data = null_mut();
    while pos.is_null() && !d.is_null() {
        if !(*d).chip.is_null() && (*(*d).chip).irq_compose_msi_msg.is_some() {
            pos = d;
        }
        d = lupos_irq_parent(d);
    }
    if pos.is_null() {
        return -(ENOSYS as c_int);
    }
    ((*(*pos).chip).irq_compose_msi_msg.unwrap_unchecked())(pos, msg);
    0
}
unsafe fn irq_get_pm_device(d: *mut irq_data) -> *mut device {
    if !(*d).domain.is_null() {
        (*(*d).domain).pm_dev
    } else {
        null_mut()
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_chip_pm_get(d: *mut irq_data) -> c_int {
    let dev = irq_get_pm_device(d);
    if cfg!(CONFIG_PM) && !dev.is_null() {
        lupos_irq_pm_resume_get(dev)
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn irq_chip_pm_put(d: *mut irq_data) {
    let dev = irq_get_pm_device(d);
    if !dev.is_null() {
        lupos_irq_pm_put(dev);
    }
}
