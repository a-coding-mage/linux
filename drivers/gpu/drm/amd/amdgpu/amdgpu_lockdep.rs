// SPDX-License-Identifier: MIT
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
 *
 * Lockdep annotation for AMDGPU lock ordering
 *
 * This module teaches lockdep the correct lock ordering to catch
 * potential deadlocks at development time rather than runtime.
 *
 * Based on dma-resv lockdep approach from:
 * drivers/dma-buf/dma-resv.c:dma_resv_lockdep()
 */

// C dependencies supplied by the surrounding kernel/AMDGPU translation unit.

#[cfg(CONFIG_LOCKDEP)]
#[repr(C)]
pub struct amdgpu_lockdep_dummy_locks {
    pub reset_lock: mutex,
    pub userq_sch_mutex: mutex,
    pub userq_mutex: mutex,
    pub notifier_lock: mutex,
    pub vram_lock: mutex,
    pub srbm_mutex: mutex,
    pub grbm_idx_mutex: mutex,
    pub mmio_idx_lock: spinlock_t,
}

#[cfg(CONFIG_LOCKDEP)]
extern "C" {
    static mut amdgpu_userq_sch_mutex_key: lock_class_key;
    static mut amdgpu_userq_mutex_key: lock_class_key;
    static mut amdgpu_notifier_lock_key: lock_class_key;
    static mut amdgpu_vram_lock_key: lock_class_key;
    static mut amdgpu_reset_sem_key: lock_class_key;
    static mut amdgpu_reset_lock_key: lock_class_key;
    static mut amdgpu_srbm_lock_key: lock_class_key;
    static mut amdgpu_grbm_lock_key: lock_class_key;
    static mut amdgpu_mmio_lock_key: lock_class_key;

    fn lockdep_set_class(lock: *mut core::ffi::c_void, key: *mut lock_class_key);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn amdgpu_reset_create_reset_domain(
        device: u32,
        name: *const core::ffi::c_char,
    ) -> *mut amdgpu_reset_domain;
    fn fs_reclaim_acquire(gfp: gfp_t);
    fn fs_reclaim_release(gfp: gfp_t);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn down_read(lock: *mut rw_semaphore);
    fn up_read(lock: *mut rw_semaphore);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn amdgpu_reset_put_reset_domain(domain: *mut amdgpu_reset_domain);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[cfg(CONFIG_LOCKDEP)]
extern "C" {
    pub fn amdgpu_lockdep_set_class(adev: *mut amdgpu_device);
}

#[cfg(CONFIG_LOCKDEP)]
#[no_mangle]
pub unsafe extern "C" fn amdgpu_lockdep_init() -> i32 {
    let mut reset_domain: *mut amdgpu_reset_domain = core::ptr::null_mut();
    let locks: *mut amdgpu_lockdep_dummy_locks;
    let mut flags: usize = 0;

    locks = kzalloc(
        core::mem::size_of::<amdgpu_lockdep_dummy_locks>(),
        GFP_KERNEL,
    ) as *mut amdgpu_lockdep_dummy_locks;
    if locks.is_null() {
        return -ENOMEM;
    }

    /* Initialize dummy reset domain */
    reset_domain = amdgpu_reset_create_reset_domain(SINGLE_DEVICE, c"lockdep_test".as_ptr());
    if reset_domain.is_null() {
        kfree(locks as *mut core::ffi::c_void);
        return -ENOMEM;
    }

    /* Initialize dummy locks */
    mutex_init(&mut (*locks).userq_sch_mutex);
    mutex_init(&mut (*locks).userq_mutex);
    mutex_init(&mut (*locks).notifier_lock);
    mutex_init(&mut (*locks).vram_lock);
    mutex_init(&mut (*locks).reset_lock);
    mutex_init(&mut (*locks).srbm_mutex);
    mutex_init(&mut (*locks).grbm_idx_mutex);
    spin_lock_init(&mut (*locks).mmio_idx_lock);

    /* Associate dummy locks with the same class keys used for real driver locks. */
    lockdep_set_class(&mut (*locks).userq_sch_mutex as *mut _ as *mut _, &mut amdgpu_userq_sch_mutex_key);
    lockdep_set_class(&mut (*locks).userq_mutex as *mut _ as *mut _, &mut amdgpu_userq_mutex_key);
    lockdep_set_class(&mut (*locks).notifier_lock as *mut _ as *mut _, &mut amdgpu_notifier_lock_key);
    lockdep_set_class(&mut (*locks).vram_lock as *mut _ as *mut _, &mut amdgpu_vram_lock_key);
    lockdep_set_class(&mut (*reset_domain).sem as *mut _ as *mut _, &mut amdgpu_reset_sem_key);
    lockdep_set_class(&mut (*locks).reset_lock as *mut _ as *mut _, &mut amdgpu_reset_lock_key);
    lockdep_set_class(&mut (*locks).srbm_mutex as *mut _ as *mut _, &mut amdgpu_srbm_lock_key);
    lockdep_set_class(&mut (*locks).grbm_idx_mutex as *mut _ as *mut _, &mut amdgpu_grbm_lock_key);
    lockdep_set_class(&mut (*locks).mmio_idx_lock as *mut _ as *mut _, &mut amdgpu_mmio_lock_key);

    /* Register fs_reclaim lock class before taking any locks. */
    fs_reclaim_acquire(GFP_KERNEL);
    fs_reclaim_release(GFP_KERNEL);

    /* Take locks in the correct order to train lockdep. */
    mutex_lock(&mut (*locks).userq_sch_mutex);
    mutex_lock(&mut (*locks).userq_mutex);
    mutex_lock(&mut (*locks).notifier_lock);
    mutex_lock(&mut (*locks).vram_lock);
    down_read(&mut (*reset_domain).sem);
    mutex_lock(&mut (*locks).reset_lock);
    mutex_lock(&mut (*locks).srbm_mutex);
    mutex_lock(&mut (*locks).grbm_idx_mutex);
    spin_lock_irqsave(&mut (*locks).mmio_idx_lock, &mut flags);

    /* Release in reverse order */
    spin_unlock_irqrestore(&mut (*locks).mmio_idx_lock, flags);
    mutex_unlock(&mut (*locks).grbm_idx_mutex);
    mutex_unlock(&mut (*locks).srbm_mutex);
    mutex_unlock(&mut (*locks).reset_lock);
    up_read(&mut (*reset_domain).sem);
    mutex_unlock(&mut (*locks).vram_lock);
    mutex_unlock(&mut (*locks).notifier_lock);
    mutex_unlock(&mut (*locks).userq_mutex);
    mutex_unlock(&mut (*locks).userq_sch_mutex);

    /* Cleanup */
    amdgpu_reset_put_reset_domain(reset_domain);
    kfree(locks as *mut core::ffi::c_void);
    pr_info(c"AMDGPU: Lockdep annotations initialized (9 lock levels)\n".as_ptr());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
