/* SPDX-License-Identifier: GPL-2.0 */

// The original header requires ARMv6 or newer SMP support.
// Dependencies supplied by the surrounding translation unit:
// linux/prefetch.h, asm/barrier.h, and asm/processor.h.

#[cfg(thumb2_kernel)]
macro_rules! WFE {
    ($cond:expr) => { __ALT_SMP_ASM!(concat!("it ", $cond, "\n\t", "wfe", $cond, ".n"), "nop.w") };
}

#[cfg(not(thumb2_kernel))]
macro_rules! WFE {
    ($cond:expr) => { __ALT_SMP_ASM!(concat!("wfe", $cond), "nop") };
}

macro_rules! SEV {
    () => { __ALT_SMP_ASM!(WASM!("sev"), WASM!("nop")) };
}

#[inline]
pub unsafe fn dsb_sev() {
    dsb(ishst);
    core::arch::asm!("sev");
}

/* ARMv6 ticket-based spin-locking. */

#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    let mut tmp: usize;
    let mut newval: u32;
    let mut lockval: arch_spinlock_t;

    prefetchw(core::ptr::addr_of_mut!((*lock).slock));
    core::arch::asm!(
        "1: ldrex {lockval}, [{slock}]\n\tadd {newval}, {lockval}, {shift}\n\tstrex {tmp}, {newval}, [{slock}]\n\tteq {tmp}, #0\n\tbne 1b",
        lockval = out(reg) lockval, newval = out(reg) newval, tmp = out(reg) tmp,
        slock = in(reg) core::ptr::addr_of_mut!((*lock).slock), shift = const (1 << TICKET_SHIFT),
        options(nostack)
    );

    while lockval.tickets.next != lockval.tickets.owner {
        wfe();
        lockval.tickets.owner = READ_ONCE((*lock).tickets.owner);
    }
    smp_mb();
}

#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    let mut contended: usize;
    let mut res: usize;
    let mut slock: u32;
    prefetchw(core::ptr::addr_of_mut!((*lock).slock));
    loop {
        core::arch::asm!(
            "ldrex {slock}, [{ptr}]\n\tmov {res}, #0\n\tsubs {contended}, {slock}, {slock}, ror #16\n\taddeq {slock}, {slock}, {shift}\n\tstrexeq {res}, {slock}, [{ptr}]",
            slock = out(reg) slock, contended = out(reg) contended, res = out(reg) res,
            ptr = in(reg) core::ptr::addr_of_mut!((*lock).slock), shift = const (1 << TICKET_SHIFT), options(nostack)
        );
        if res == 0 { break; }
    }
    if contended == 0 { smp_mb(); 1 } else { 0 }
}

#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    smp_mb();
    (*lock).tickets.owner = (*lock).tickets.owner.wrapping_add(1);
    dsb_sev();
}

#[inline]
pub fn arch_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
    (lock.tickets.owner == lock.tickets.next) as i32
}

#[inline]
pub unsafe fn arch_spin_is_locked(lock: *mut arch_spinlock_t) -> i32 {
    (! (arch_spin_value_unlocked(READ_ONCE(*lock)) != 0)) as i32
}

#[inline]
pub unsafe fn arch_spin_is_contended(lock: *mut arch_spinlock_t) -> i32 {
    let tickets: __raw_tickets = READ_ONCE((*lock).tickets);
    ((tickets.next.wrapping_sub(tickets.owner)) > 1) as i32
}

#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize;
    prefetchw(core::ptr::addr_of_mut!((*rw).lock));
    core::arch::asm!("1: ldrex {tmp}, [{ptr}]\n\tteq {tmp}, #0\n\tstrexeq {tmp}, {value}, [{ptr}]\n\tteq {tmp}, #0\n\tbne 1b", tmp = out(reg) tmp, ptr = in(reg) core::ptr::addr_of_mut!((*rw).lock), value = in(reg) 0x80000000u32, options(nostack));
    smp_mb();
}

#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut contended: usize; let mut res: usize;
    prefetchw(core::ptr::addr_of_mut!((*rw).lock));
    loop {
        core::arch::asm!("ldrex {contended}, [{ptr}]\n\tmov {res}, #0\n\tteq {contended}, #0\n\tstrexeq {res}, {value}, [{ptr}]", contended = out(reg) contended, res = out(reg) res, ptr = in(reg) core::ptr::addr_of_mut!((*rw).lock), value = in(reg) 0x80000000u32, options(nostack));
        if res == 0 { break; }
    }
    if contended == 0 { smp_mb(); 1 } else { 0 }
}

#[inline]
pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    smp_mb();
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*rw).lock), 0);
    dsb_sev();
}

#[inline]
pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize; let mut tmp2: usize;
    prefetchw(core::ptr::addr_of_mut!((*rw).lock));
    core::arch::asm!("1: ldrex {tmp}, [{ptr}]\n\tadds {tmp}, {tmp}, #1\n\tstrexpl {tmp2}, {tmp}, [{ptr}]\n\tmi wfe\n\trsbspl {tmp}, {tmp2}, #0\n\tbmi 1b", tmp = out(reg) tmp, tmp2 = out(reg) tmp2, ptr = in(reg) core::ptr::addr_of_mut!((*rw).lock), options(nostack));
    smp_mb();
}

#[inline]
pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut tmp: usize; let mut tmp2: usize;
    smp_mb(); prefetchw(core::ptr::addr_of_mut!((*rw).lock));
    core::arch::asm!("1: ldrex {tmp}, [{ptr}]\n\tsub {tmp}, {tmp}, #1\n\tstrex {tmp2}, {tmp}, [{ptr}]\n\tteq {tmp2}, #0\n\tbne 1b", tmp = out(reg) tmp, tmp2 = out(reg) tmp2, ptr = in(reg) core::ptr::addr_of_mut!((*rw).lock), options(nostack));
    if tmp == 0 { dsb_sev(); }
}

#[inline]
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut contended: u32; let mut res: usize;
    prefetchw(core::ptr::addr_of_mut!((*rw).lock));
    loop {
        core::arch::asm!("ldrex {contended}, [{ptr}]\n\tmov {res}, #0\n\tadds {contended}, {contended}, #1\n\tstrexpl {res}, {contended}, [{ptr}]", contended = out(reg) contended, res = out(reg) res, ptr = in(reg) core::ptr::addr_of_mut!((*rw).lock), options(nostack));
        if res == 0 { break; }
    }
    if contended < 0x80000000 { smp_mb(); 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
