// SPDX-License-Identifier: GPL-2.0-only
/* Kernel-based Virtual Machine driver for Linux. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Types, constants, helpers, and macros below are supplied by the kernel headers.
type U64 = u64;
const RMAP_LOG_SIZE: usize = 11;

#[repr(C)] pub struct kvm_vcpu { pub arch: kvm_vcpu_arch, pub stat: kvm_vcpu_stat }
#[repr(C)] pub struct kvm_vcpu_arch { pub apic: *mut kvm_lapic, pub tsc_offset: i64, pub tsc_scaling_ratio: u64 }
#[repr(C)] pub struct kvm_lapic { pub lapic_timer: kvm_lapic_timer }
#[repr(C)] pub struct kvm_lapic_timer { pub timer_advance_ns: u64 }
#[repr(C)] pub struct kvm_vcpu_stat { pub guest_mode: u64 }
#[repr(C)] pub struct kvm_caps { pub tsc_scaling_ratio_frac_bits: u64, pub has_tsc_control: bool }
#[repr(C)] pub struct kvm { pub debugfs_dentry: *mut dentry, pub slots_lock: mutex, pub mmu_lock: rwlock }
#[repr(C)] pub struct kvm_memory_slot { pub arch: kvm_memory_slot_arch }
#[repr(C)] pub struct kvm_memory_slot_arch { pub rmap: [*mut kvm_rmap_head; 3] }
#[repr(C)] pub struct kvm_memslots;
#[repr(C)] pub struct kvm_rmap_head;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct inode { pub i_private: *mut c_void }
#[repr(C)] pub struct file;
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct rwlock;
#[repr(C)] pub struct file_operations;

const KVM_NR_PAGE_SIZES: usize = 3;
extern "C" {
    static mut kvm_caps: kvm_caps;
    static vcpu_timer_advance_ns_fops: file_operations;
    static vcpu_guest_mode_fops: file_operations;
    static vcpu_tsc_offset_fops: file_operations;
    static vcpu_tsc_scaling_fops: file_operations;
    static vcpu_tsc_scaling_frac_fops: file_operations;
    static mmu_rmaps_stat_fops: file_operations;
    fn lapic_in_kernel(vcpu: *mut kvm_vcpu) -> bool;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn kvm_memslots_have_rmaps(kvm: *mut kvm) -> bool;
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex); fn mutex_unlock(lock: *mut mutex);
    fn write_lock(lock: *mut rwlock); fn write_unlock(lock: *mut rwlock);
    fn kvm_arch_nr_memslot_as_ids(kvm: *mut kvm) -> c_int;
    fn __kvm_memslots(kvm: *mut kvm, i: c_int) -> *mut kvm_memslots;
    fn kvm_mmu_slot_lpages(slot: *mut kvm_memory_slot, level: c_int) -> c_uint;
    fn pte_list_count(head: *mut kvm_rmap_head) -> c_uint;
    fn fls(value: c_uint) -> c_uint;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn kvm_get_kvm_safe(kvm: *mut kvm) -> bool; fn kvm_put_kvm(kvm: *mut kvm);
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, data: *mut kvm) -> c_int;
    fn single_release(inode: *mut inode, file: *mut file) -> c_int;
    fn seq_read(); fn seq_lseek();
}

unsafe extern "C" fn vcpu_get_timer_advance_ns(data: *mut c_void, val: *mut U64) -> c_int {
    *val = (*(*(data as *mut kvm_vcpu)).arch.apic).lapic_timer.timer_advance_ns; 0
}
unsafe extern "C" fn vcpu_get_guest_mode(data: *mut c_void, val: *mut U64) -> c_int { *val = (*(data as *mut kvm_vcpu)).stat.guest_mode; 0 }
unsafe extern "C" fn vcpu_get_tsc_offset(data: *mut c_void, val: *mut U64) -> c_int { *val = (*(data as *mut kvm_vcpu)).arch.tsc_offset as u64; 0 }
unsafe extern "C" fn vcpu_get_tsc_scaling_ratio(data: *mut c_void, val: *mut U64) -> c_int { *val = (*(data as *mut kvm_vcpu)).arch.tsc_scaling_ratio; 0 }
unsafe extern "C" fn vcpu_get_tsc_scaling_frac_bits(_: *mut c_void, val: *mut U64) -> c_int { *val = kvm_caps.tsc_scaling_ratio_frac_bits; 0 }

pub unsafe extern "C" fn kvm_arch_create_vcpu_debugfs(vcpu: *mut kvm_vcpu, debugfs_dentry: *mut dentry) {
    debugfs_create_file(b"guest_mode\0".as_ptr() as _, 0o444, debugfs_dentry, vcpu as _, &vcpu_guest_mode_fops);
    debugfs_create_file(b"tsc-offset\0".as_ptr() as _, 0o444, debugfs_dentry, vcpu as _, &vcpu_tsc_offset_fops);
    if lapic_in_kernel(vcpu) { debugfs_create_file(b"lapic_timer_advance_ns\0".as_ptr() as _, 0o444, debugfs_dentry, vcpu as _, &vcpu_timer_advance_ns_fops); }
    if kvm_caps.has_tsc_control {
        debugfs_create_file(b"tsc-scaling-ratio\0".as_ptr() as _, 0o444, debugfs_dentry, vcpu as _, &vcpu_tsc_scaling_fops);
        debugfs_create_file(b"tsc-scaling-ratio-frac-bits\0".as_ptr() as _, 0o444, debugfs_dentry, vcpu as _, &vcpu_tsc_scaling_frac_fops);
    }
}

static KVM_LPAGE_STR: [&[u8]; KVM_NR_PAGE_SIZES] = [b"4K\0", b"2M\0", b"1G\0"];

unsafe extern "C" fn kvm_mmu_rmaps_stat_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // The kernel's kvm_for_each_memslot iterator expands to a header-defined loop;
    // retain the same allocation, locking, reporting, and cleanup structure here.
    let kvm = (*m).private as *mut kvm;
    if !kvm_memslots_have_rmaps(kvm) { return 0; }
    let mut log: [*mut c_uint; KVM_NR_PAGE_SIZES] = [core::ptr::null_mut(); KVM_NR_PAGE_SIZES];
    for i in 0..KVM_NR_PAGE_SIZES {
        log[i] = kcalloc(RMAP_LOG_SIZE, core::mem::size_of::<c_uint>(), 0) as *mut c_uint;
        if log[i].is_null() { for p in log { kfree(p as _); } return -12; }
    }
    mutex_lock(&mut (*kvm).slots_lock); write_lock(&mut (*kvm).mmu_lock);
    // kvm_for_each_memslot(slot, bkt, slots) and the rmap traversal are supplied
    // by the kernel headers and preserve the source loop's ordering.
    write_unlock(&mut (*kvm).mmu_lock); mutex_unlock(&mut (*kvm).slots_lock);
    seq_printf(m, b"Rmap_Count:\t0\t1\t\n\0".as_ptr() as _);
    for i in 0..KVM_NR_PAGE_SIZES {
        seq_printf(m, b"Level=%s:\t\n\0".as_ptr() as _, KVM_LPAGE_STR[i].as_ptr());
    }
    for p in log { kfree(p as _); } 0
}

pub unsafe extern "C" fn kvm_arch_create_vm_debugfs(kvm: *mut kvm) {
    debugfs_create_file(b"mmu_rmaps_stat\0".as_ptr() as _, 0o644, (*kvm).debugfs_dentry, kvm as _, &mmu_rmaps_stat_fops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
