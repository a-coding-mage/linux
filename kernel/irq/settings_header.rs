/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Internal header to deal with irq_desc->status which will be renamed
 * to irq_desc->settings.
 */

pub const _IRQ_DEFAULT_INIT_FLAGS: u32 = IRQ_DEFAULT_INIT_FLAGS;
pub const _IRQ_PER_CPU: u32 = IRQ_PER_CPU;
pub const _IRQ_LEVEL: u32 = IRQ_LEVEL;
pub const _IRQ_NOPROBE: u32 = IRQ_NOPROBE;
pub const _IRQ_NOREQUEST: u32 = IRQ_NOREQUEST;
pub const _IRQ_NOTHREAD: u32 = IRQ_NOTHREAD;
pub const _IRQ_NOAUTOEN: u32 = IRQ_NOAUTOEN;
pub const _IRQ_NO_BALANCING: u32 = IRQ_NO_BALANCING;
pub const _IRQ_NESTED_THREAD: u32 = IRQ_NESTED_THREAD;
pub const _IRQ_PER_CPU_DEVID: u32 = IRQ_PER_CPU_DEVID;
pub const _IRQ_IS_POLLED: u32 = IRQ_IS_POLLED;
pub const _IRQ_DISABLE_UNLAZY: u32 = IRQ_DISABLE_UNLAZY;
pub const _IRQ_HIDDEN: u32 = IRQ_HIDDEN;
pub const _IRQ_NO_DEBUG: u32 = IRQ_NO_DEBUG;
pub const _IRQ_PROC_VALID: u32 = IRQ_RESERVED;
pub const _IRQF_MODIFY_MASK: u32 = IRQF_MODIFY_MASK;

// C preprocessor poison macros; these names intentionally cannot be used here.
// #define IRQ_PER_CPU GOT_YOU_MORON
// #define IRQ_NO_BALANCING GOT_YOU_MORON
// #define IRQ_LEVEL GOT_YOU_MORON
// #define IRQ_NOPROBE GOT_YOU_MORON
// #define IRQ_NOREQUEST GOT_YOU_MORON
// #define IRQ_NOTHREAD GOT_YOU_MORON
// #define IRQ_NOAUTOEN GOT_YOU_MORON
// #define IRQ_NESTED_THREAD GOT_YOU_MORON
// #define IRQ_PER_CPU_DEVID GOT_YOU_MORON
// #define IRQ_IS_POLLED GOT_YOU_MORON
// #define IRQ_DISABLE_UNLAZY GOT_YOU_MORON
// #define IRQ_HIDDEN GOT_YOU_MORON
// #define IRQ_NO_DEBUG GOT_YOU_MORON
// #define IRQ_RESERVED GOT_YOU_MORON
// #undef IRQF_MODIFY_MASK
// #define IRQF_MODIFY_MASK GOT_YOU_MORON

pub unsafe fn irq_settings_clr_and_set(desc: *mut irq_desc, clr: u32, set: u32) {
    (*desc).status_use_accessors &= !(clr & _IRQF_MODIFY_MASK);
    (*desc).status_use_accessors |= set & _IRQF_MODIFY_MASK;
}

pub unsafe fn irq_settings_is_per_cpu(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_PER_CPU) != 0
}

pub unsafe fn irq_settings_is_per_cpu_devid(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_PER_CPU_DEVID) != 0
}

pub unsafe fn irq_settings_set_per_cpu(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_PER_CPU;
}

pub unsafe fn irq_settings_set_no_balancing(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_NO_BALANCING;
}

pub unsafe fn irq_settings_has_no_balance_set(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NO_BALANCING) != 0
}

pub unsafe fn irq_settings_get_trigger_mask(desc: *mut irq_desc) -> u32 {
    (*desc).status_use_accessors & IRQ_TYPE_SENSE_MASK
}

pub unsafe fn irq_settings_set_trigger_mask(desc: *mut irq_desc, mask: u32) {
    (*desc).status_use_accessors &= !IRQ_TYPE_SENSE_MASK;
    (*desc).status_use_accessors |= mask & IRQ_TYPE_SENSE_MASK;
}

pub unsafe fn irq_settings_is_level(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_LEVEL) != 0
}

pub unsafe fn irq_settings_clr_level(desc: *mut irq_desc) {
    (*desc).status_use_accessors &= !_IRQ_LEVEL;
}

pub unsafe fn irq_settings_set_level(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_LEVEL;
}

pub unsafe fn irq_settings_can_request(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NOREQUEST) == 0
}

pub unsafe fn irq_settings_clr_norequest(desc: *mut irq_desc) {
    (*desc).status_use_accessors &= !_IRQ_NOREQUEST;
}

pub unsafe fn irq_settings_set_norequest(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_NOREQUEST;
}

pub unsafe fn irq_settings_can_thread(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NOTHREAD) == 0
}

pub unsafe fn irq_settings_clr_nothread(desc: *mut irq_desc) {
    (*desc).status_use_accessors &= !_IRQ_NOTHREAD;
}

pub unsafe fn irq_settings_set_nothread(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_NOTHREAD;
}

pub unsafe fn irq_settings_can_probe(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NOPROBE) == 0
}

pub unsafe fn irq_settings_clr_noprobe(desc: *mut irq_desc) {
    (*desc).status_use_accessors &= !_IRQ_NOPROBE;
}

pub unsafe fn irq_settings_set_noprobe(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_NOPROBE;
}

pub unsafe fn irq_settings_can_autoenable(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NOAUTOEN) == 0
}

pub unsafe fn irq_settings_is_nested_thread(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NESTED_THREAD) != 0
}

pub unsafe fn irq_settings_is_polled(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_IS_POLLED) != 0
}

pub unsafe fn irq_settings_disable_unlazy(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_DISABLE_UNLAZY) != 0
}

pub unsafe fn irq_settings_clr_disable_unlazy(desc: *mut irq_desc) {
    (*desc).status_use_accessors &= !_IRQ_DISABLE_UNLAZY;
}

pub unsafe fn irq_settings_is_hidden(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_HIDDEN) != 0
}

pub unsafe fn irq_settings_set_no_debug(desc: *mut irq_desc) {
    (*desc).status_use_accessors |= _IRQ_NO_DEBUG;
}

pub unsafe fn irq_settings_no_debug(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_NO_DEBUG) != 0
}

pub unsafe fn irq_settings_proc_valid(desc: *mut irq_desc) -> bool {
    ((*desc).status_use_accessors & _IRQ_PROC_VALID) != 0
}

pub unsafe fn irq_settings_update_proc_valid(desc: *mut irq_desc, set: u32) {
    (*desc).status_use_accessors &= !_IRQ_PROC_VALID;
    (*desc).status_use_accessors |= set & _IRQ_PROC_VALID;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
