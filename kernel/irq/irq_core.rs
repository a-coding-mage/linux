// SPDX-License-Identifier: GPL-2.0
// Production owners for irqdesc.c, handle.c and chip.c. Native configured types
// are generated from irq_core_bindings.h; no hand-written kernel layouts.
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/irq_core_generated.rs"
    ));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[inline(always)]
unsafe fn data(desc: *mut irq_desc) -> *mut irq_data {
    addr_of_mut!((*desc).irq_data)
}
#[inline(always)]
unsafe fn state(desc: *mut irq_desc) -> *mut c_uint {
    addr_of_mut!((*desc).core_internal_state__do_not_mess_with_it)
}
// Scoped C guards translated to Rust RAII retain early-return unlock behavior.
struct DescLock {
    desc: *mut irq_desc,
    flags: c_ulong,
    mode: u8,
}
impl DescLock {
    unsafe fn raw(desc: *mut irq_desc) -> Self {
        lupos_irq_raw_lock(addr_of_mut!((*desc).lock));
        Self {
            desc,
            flags: 0,
            mode: 0,
        }
    }
    unsafe fn irq(desc: *mut irq_desc) -> Self {
        lupos_irq_raw_lock_irq(addr_of_mut!((*desc).lock));
        Self {
            desc,
            flags: 0,
            mode: 1,
        }
    }
    unsafe fn save(desc: *mut irq_desc) -> Self {
        let flags = lupos_irq_raw_lock_save(addr_of_mut!((*desc).lock));
        Self {
            desc,
            flags,
            mode: 2,
        }
    }
}
impl Drop for DescLock {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                0 => lupos_irq_raw_unlock(addr_of_mut!((*self.desc).lock)),
                1 => lupos_irq_raw_unlock_irq(addr_of_mut!((*self.desc).lock)),
                _ => lupos_irq_raw_unlock_restore(addr_of_mut!((*self.desc).lock), self.flags),
            }
        }
    }
}
struct BusLock {
    desc: *mut irq_desc,
    flags: c_ulong,
    bus: bool,
}
impl BusLock {
    unsafe fn get(irq: c_uint, check: c_uint, bus: bool) -> Option<Self> {
        let mut flags = 0;
        let desc = __irq_get_desc_lock(irq, &mut flags, bus, check);
        if desc.is_null() {
            None
        } else {
            Some(Self { desc, flags, bus })
        }
    }
}
impl Drop for BusLock {
    fn drop(&mut self) {
        unsafe { __irq_put_desc_unlock(self.desc, self.flags, self.bus) }
    }
}
struct SparseLock;
impl SparseLock {
    unsafe fn new() -> Self {
        lupos_irq_sparse_lock();
        Self
    }
}
impl Drop for SparseLock {
    fn drop(&mut self) {
        unsafe { lupos_irq_sparse_unlock() }
    }
}
struct RcuLock;
impl RcuLock {
    unsafe fn new() -> Self {
        lupos_irq_rcu_lock();
        Self
    }
}
impl Drop for RcuLock {
    fn drop(&mut self) {
        unsafe { lupos_irq_rcu_unlock() }
    }
}

// Header-only state accessors, expressed over canonical configured layouts.
#[inline(always)]
unsafe fn lupos_irq_data_has(d: *mut irq_data, bits: c_uint) -> bool {
    (*(*d).common).state_use_accessors & bits != 0
}
#[inline(always)]
unsafe fn lupos_irq_data_set(d: *mut irq_data, bits: c_uint) {
    (*(*d).common).state_use_accessors |= bits;
}
#[inline(always)]
unsafe fn lupos_irq_data_clear(d: *mut irq_data, bits: c_uint) {
    (*(*d).common).state_use_accessors &= !bits;
}
#[inline(always)]
unsafe fn settings(desc: *mut irq_desc, clear: c_uint, set: c_uint) {
    (*desc).status_use_accessors &= !(clear & _IRQF_MODIFY_MASK);
    (*desc).status_use_accessors |= set & _IRQF_MODIFY_MASK;
}
#[inline(always)]
unsafe fn setting(desc: *mut irq_desc, flag: c_uint) -> bool {
    (*desc).status_use_accessors & flag != 0
}
#[inline(always)]
unsafe fn lupos_irq_can_thread(desc: *mut irq_desc) -> bool {
    !setting(desc, _IRQ_NOTHREAD)
}
#[inline(always)]
unsafe fn lupos_irq_no_debug(desc: *mut irq_desc) -> bool {
    setting(desc, _IRQ_NO_DEBUG)
}
#[inline(always)]
unsafe fn disabled(desc: *mut irq_desc) -> bool {
    lupos_irq_data_has(data(desc), IRQD_IRQ_DISABLED)
}
#[inline(always)]
unsafe fn masked(desc: *mut irq_desc) -> bool {
    lupos_irq_data_has(data(desc), IRQD_IRQ_MASKED)
}
#[cfg(CONFIG_IRQ_DOMAIN)]
#[inline(always)]
unsafe fn irq_resolve_mapping(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> *mut irq_desc {
    __irq_resolve_mapping(domain, hwirq, null_mut())
}
