// SPDX-License-Identifier: LGPL-2.1+

// Dependencies supplied by the Linux IRQ and KUnit headers are intentionally
// referenced here rather than reimplemented.

unsafe extern "C" {
    fn irq_data_update_effective_affinity(data: *mut irq_data, dest: *const cpumask);
    fn irq_domain_alloc_descs(irq: i32, from: u32, cnt: u32, node: i32,
                              affd: *mut irq_affinity_desc) -> i32;
    fn irq_set_chip_and_handler(virq: i32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn irq_to_desc(virq: i32) -> *mut irq_desc;
    fn irq_settings_clr_norequest(desc: *mut irq_desc);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const core::ffi::c_char, data: *mut core::ffi::c_void) -> i32;
    fn disable_irq(virq: i32);
    fn enable_irq(virq: i32);
    fn free_irq(virq: i32, data: *mut core::ffi::c_void);
    fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data;
    fn irqd_is_activated(data: *const irq_data) -> bool;
    fn irqd_is_started(data: *const irq_data) -> bool;
    fn irqd_affinity_is_managed(data: *const irq_data) -> bool;
    fn irq_shutdown_and_deactivate(desc: *mut irq_desc);
    fn irq_activate(desc: *mut irq_desc) -> i32;
    fn irq_startup_managed(desc: *mut irq_desc);
    fn get_cpu_device(cpu: u32) -> *mut core::ffi::c_void;
    fn cpu_is_hotpluggable(cpu: u32) -> bool;
    fn cpu_online(cpu: u32) -> bool;
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn remove_cpu(cpu: u32) -> i32;
    fn add_cpu(cpu: u32) -> i32;
}

#[repr(C)]
pub struct irq_data;
#[repr(C)]
pub struct irq_desc { pub depth: u32, pub lock: raw_spinlock_t }
#[repr(C)]
pub struct cpumask;
#[repr(C)]
pub struct irq_affinity_desc { pub is_managed: u8, pub mask: cpumask }
#[repr(C)]
pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask, bool) -> i32>,
    pub flags: u32,
}
#[repr(C)]
pub struct raw_spinlock_t;
pub type irqreturn_t = i32;
pub type kunit = core::ffi::c_void;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQCHIP_SKIP_SET_WAKE: u32 = 1;
const NUMA_NO_NODE: i32 = -1;
const CPU_MASK_ALL: cpumask = unsafe { core::mem::zeroed() };

unsafe extern "C" fn noop_handler(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t {
    IRQ_HANDLED
}

unsafe extern "C" fn noop(_data: *mut irq_data) {}
unsafe extern "C" fn noop_ret(_data: *mut irq_data) -> u32 { 0 }

unsafe extern "C" fn noop_affinity(data: *mut irq_data, dest: *const cpumask, _force: bool) -> i32 {
    irq_data_update_effective_affinity(data, dest);
    0
}

static mut FAKE_IRQ_CHIP: irq_chip = irq_chip {
    name: b"fake\0".as_ptr() as *const core::ffi::c_char,
    irq_startup: Some(noop_ret), irq_shutdown: Some(noop), irq_enable: Some(noop),
    irq_disable: Some(noop), irq_ack: Some(noop), irq_mask: Some(noop), irq_unmask: Some(noop),
    irq_set_affinity: Some(noop_affinity), flags: IRQCHIP_SKIP_SET_WAKE,
};

unsafe fn irq_test_setup_fake_irq(_test: *mut kunit, affd: *mut irq_affinity_desc) -> i32 {
    let virq = irq_domain_alloc_descs(-1, 1, 0, NUMA_NO_NODE, affd);
    irq_set_chip_and_handler(virq, &raw mut FAKE_IRQ_CHIP, handle_simple_irq);
    let desc = irq_to_desc(virq);
    irq_settings_clr_norequest(desc);
    virq
}

unsafe fn irq_disable_depth_test(test: *mut kunit) {
    let virq = irq_test_setup_fake_irq(test, core::ptr::null_mut());
    let desc = irq_to_desc(virq);
    let ret = request_irq(virq, noop_handler, 0, b"test_irq\0".as_ptr() as _, core::ptr::null_mut());
    assert!(ret == 0);
    assert!((*desc).depth == 0);
    disable_irq(virq); assert!((*desc).depth == 1);
    enable_irq(virq); assert!((*desc).depth == 0);
    free_irq(virq, core::ptr::null_mut());
}

unsafe fn irq_free_disabled_test(test: *mut kunit) {
    let virq = irq_test_setup_fake_irq(test, core::ptr::null_mut());
    let desc = irq_to_desc(virq);
    assert!(request_irq(virq, noop_handler, 0, b"test_irq\0".as_ptr() as _, core::ptr::null_mut()) == 0);
    assert!((*desc).depth == 0);
    disable_irq(virq); assert!((*desc).depth == 1);
    free_irq(virq, core::ptr::null_mut()); assert!((*desc).depth >= 1);
    assert!(request_irq(virq, noop_handler, 0, b"test_irq\0".as_ptr() as _, core::ptr::null_mut()) == 0);
    assert!((*desc).depth == 0); free_irq(virq, core::ptr::null_mut());
}

unsafe fn irq_shutdown_depth_test(test: *mut kunit) {
    let mut affinity = irq_affinity_desc { is_managed: 1, mask: CPU_MASK_ALL };
    let virq = irq_test_setup_fake_irq(test, &raw mut affinity);
    let desc = irq_to_desc(virq); let data = irq_desc_get_irq_data(desc);
    assert!(request_irq(virq, noop_handler, 0, b"test_irq\0".as_ptr() as _, core::ptr::null_mut()) == 0);
    assert!(irqd_is_activated(data)); assert!(irqd_is_started(data)); assert!(irqd_affinity_is_managed(data));
    assert!((*desc).depth == 0); disable_irq(virq); assert!((*desc).depth == 1);
    irq_shutdown_and_deactivate(desc);
    assert!(!irqd_is_activated(data)); assert!(!irqd_is_started(data));
    assert!(irq_activate(desc) == 0); irq_startup_managed(desc);
    assert!((*desc).depth == 1); enable_irq(virq); assert!((*desc).depth == 0);
    free_irq(virq, core::ptr::null_mut());
}

unsafe fn irq_cpuhotplug_test(test: *mut kunit) {
    let mut affinity = irq_affinity_desc { is_managed: 1, mask: CPU_MASK_ALL };
    cpumask_copy(&raw mut affinity.mask, cpumask_of(1));
    let virq = irq_test_setup_fake_irq(test, &raw mut affinity);
    let desc = irq_to_desc(virq); let data = irq_desc_get_irq_data(desc);
    assert!(request_irq(virq, noop_handler, 0, b"test_irq\0".as_ptr() as _, core::ptr::null_mut()) == 0);
    assert!(irqd_is_activated(data)); assert!(irqd_is_started(data)); assert!(irqd_affinity_is_managed(data));
    assert!((*desc).depth == 0); disable_irq(virq); assert!((*desc).depth == 1);
    assert!(remove_cpu(1) == 0); assert!((*desc).depth >= 1); assert!(add_cpu(1) == 0);
    assert!((*desc).depth == 1); enable_irq(virq);
    assert!(irqd_is_activated(data)); assert!(irqd_is_started(data)); assert!((*desc).depth == 0);
    free_irq(virq, core::ptr::null_mut());
}

extern "C" { fn handle_simple_irq(); }

// KUnit registration: irq_test_cases contains
// irq_disable_depth_test, irq_free_disabled_test, irq_shutdown_depth_test,
// and irq_cpuhotplug_test, and is registered as "irq_test_cases".
// MODULE_DESCRIPTION("IRQ unit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
