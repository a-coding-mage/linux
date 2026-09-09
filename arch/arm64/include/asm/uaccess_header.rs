/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of the arm64 uaccess header. */

/* Symbols supplied by the surrounding kernel translation are intentionally external. */

unsafe extern "C" {
    fn __access_ok(ptr: *const core::ffi::c_void, size: usize) -> i32;
    fn untagged_addr(ptr: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn test_thread_flag(flag: i32) -> bool;
    fn system_uses_ttbr0_pan() -> bool;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn read_sysreg(reg: usize) -> usize;
    fn write_sysreg(value: usize, reg: usize);
    fn isb();
    fn mte_disable_tco();
    fn mte_enable_tco();
    fn mte_probe_user_range(addr: *const u8, size: usize) -> usize;
    fn system_supports_mte() -> bool;
    fn might_fault();
    fn __mte_enable_tco_async();
    fn __mte_disable_tco_async();
    fn kasan_check_write(dst: *mut core::ffi::c_void, size: usize);
    fn __arch_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn __arch_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn __arch_clear_user(to: *mut core::ffi::c_void, n: usize) -> usize;
    fn strncpy_from_user(dest: *mut u8, src: *const u8, count: isize) -> isize;
    fn strnlen_user(s: *const u8, n: isize) -> isize;
}

const PF_KTHREAD: usize = 0;
const TIF_TAGGED_ADDR: i32 = 0;
const TTBRx_EL1_ASID_MASK: usize = 0;
const RESERVED_SWAPPER_OFFSET: usize = 0;
const EFAULT: i32 = 14;

#[inline]
pub unsafe fn access_ok(addr: *const core::ffi::c_void, size: usize) -> bool {
    let mut addr = addr;
    /* CONFIG_ARM64_TAGGED_ADDR_ABI conditional: untag kernel-thread/tagged addresses. */
    if (false) && (false || test_thread_flag(TIF_TAGGED_ADDR)) {
        addr = untagged_addr(addr);
    }
    __access_ok(addr, size) != 0
}

#[inline]
unsafe fn __uaccess_ttbr0_disable() {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    let mut ttbr = read_sysreg(0);
    ttbr &= !TTBRx_EL1_ASID_MASK;
    write_sysreg(ttbr.wrapping_sub(RESERVED_SWAPPER_OFFSET), 1);
    write_sysreg(ttbr, 0);
    isb();
    local_irq_restore(flags);
}

#[inline]
unsafe fn __uaccess_ttbr0_enable() {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    let ttbr0 = 0usize; /* READ_ONCE(current_thread_info()->ttbr0) */
    let mut ttbr1 = read_sysreg(0);
    ttbr1 &= !TTBRx_EL1_ASID_MASK;
    ttbr1 |= ttbr0 & TTBRx_EL1_ASID_MASK;
    write_sysreg(ttbr1, 0);
    write_sysreg(ttbr0, 1);
    isb();
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn uaccess_ttbr0_disable() -> bool {
    /* CONFIG_ARM64_SW_TTBR0_PAN conditional. */
    if !system_uses_ttbr0_pan() { return false; }
    __uaccess_ttbr0_disable(); true
}

#[inline]
pub unsafe fn uaccess_ttbr0_enable() -> bool {
    if !system_uses_ttbr0_pan() { return false; }
    __uaccess_ttbr0_enable(); true
}

#[inline]
unsafe fn __uaccess_disable_hw_pan() { core::arch::asm!("nop"); }
#[inline]
unsafe fn __uaccess_enable_hw_pan() { core::arch::asm!("nop"); }

#[inline]
pub unsafe fn uaccess_disable_privileged() {
    mte_disable_tco();
    if uaccess_ttbr0_disable() { return; }
    __uaccess_enable_hw_pan();
}

#[inline]
pub unsafe fn uaccess_enable_privileged() {
    mte_enable_tco();
    if uaccess_ttbr0_enable() { return; }
    __uaccess_disable_hw_pan();
}

#[inline]
pub unsafe fn __uaccess_mask_ptr(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut safe_ptr: *mut core::ffi::c_void;
    core::arch::asm!("bic {0}, {1}, {2}", out(reg) safe_ptr, in(reg) ptr, const (1usize << 55));
    safe_ptr
}

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    uaccess_ttbr0_enable();
    let r = __arch_copy_from_user(to, __uaccess_mask_ptr(from), n);
    uaccess_ttbr0_disable(); r
}

#[inline]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    uaccess_ttbr0_enable();
    let r = __arch_copy_to_user(__uaccess_mask_ptr(to), from, n);
    uaccess_ttbr0_disable(); r
}

#[inline]
pub unsafe fn user_access_begin(ptr: *const core::ffi::c_void, len: usize) -> bool {
    if !access_ok(ptr, len) { return false; }
    uaccess_ttbr0_enable(); true
}
#[inline] pub unsafe fn user_access_end() { uaccess_ttbr0_disable(); }
#[inline] pub unsafe fn user_access_save() -> usize { 0 }
#[inline] pub unsafe fn user_access_restore(_enabled: usize) {}

#[inline]
pub unsafe fn __clear_user(to: *mut core::ffi::c_void, mut n: usize) -> usize {
    if access_ok(to, n) {
        uaccess_ttbr0_enable();
        n = __arch_clear_user(__uaccess_mask_ptr(to), n);
        uaccess_ttbr0_disable();
    }
    n
}

#[inline]
pub unsafe fn copy_from_user_flushcache(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> usize {
    kasan_check_write(dst, size);
    /* CONFIG_ARCH_HAS_UACCESS_FLUSHCACHE conditional; external implementation required. */
    __arch_copy_from_user(dst, __uaccess_mask_ptr(src), size)
}

#[inline]
pub unsafe fn probe_subpage_writeable(addr: *const u8, size: usize) -> usize {
    if !system_supports_mte() { return 0; }
    mte_probe_user_range(addr, size)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
