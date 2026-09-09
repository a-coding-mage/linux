// SPDX-License-Identifier: GPL-2.0

// Translated from the Linux kernel implementation. Types and functions
// supplied by the surrounding kernel are intentionally left as externals.

const UMWAIT_C02_ENABLE: u32 = 0;

#[inline]
const fn umwait_ctrl_val(max_time: u32, c02_disable: u32) -> u32 {
    (max_time & MSR_IA32_UMWAIT_CONTROL_TIME_MASK)
        | (c02_disable & MSR_IA32_UMWAIT_CONTROL_C02_DISABLE)
}

static mut umwait_control_cached: u32 = umwait_ctrl_val(100000, UMWAIT_C02_ENABLE);
static mut orig_umwait_control_cached: u32 = 0;

// DEFINE_MUTEX(umwait_lock)
static mut umwait_lock: Mutex = Mutex::new();

unsafe fn umwait_update_control_msr(_unused: *mut core::ffi::c_void) {
    lockdep_assert_irqs_disabled();
    wrmsrq(
        MSR_IA32_UMWAIT_CONTROL,
        core::ptr::read_volatile(&umwait_control_cached),
    );
}

unsafe fn umwait_cpu_online(_cpu: u32) -> i32 {
    local_irq_disable();
    umwait_update_control_msr(core::ptr::null_mut());
    local_irq_enable();
    0
}

unsafe fn umwait_cpu_offline(_cpu: u32) -> i32 {
    wrmsrq(MSR_IA32_UMWAIT_CONTROL, orig_umwait_control_cached);
    0
}

unsafe fn umwait_syscore_resume(_data: *mut core::ffi::c_void) {
    umwait_update_control_msr(core::ptr::null_mut());
}

#[repr(C)]
struct SyscoreOps {
    resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

#[repr(C)]
struct Syscore {
    ops: *const SyscoreOps,
}

static umwait_syscore_ops: SyscoreOps = SyscoreOps {
    resume: Some(umwait_syscore_resume),
};

static mut umwait_syscore: Syscore = Syscore {
    ops: &umwait_syscore_ops,
};

#[inline]
unsafe fn umwait_ctrl_c02_enabled(ctrl: u32) -> bool {
    (ctrl & MSR_IA32_UMWAIT_CONTROL_C02_DISABLE) == 0
}

#[inline]
fn umwait_ctrl_max_time(ctrl: u32) -> u32 {
    ctrl & MSR_IA32_UMWAIT_CONTROL_TIME_MASK
}

unsafe fn umwait_update_control(maxtime: u32, c02_enable: bool) {
    let mut ctrl = maxtime & MSR_IA32_UMWAIT_CONTROL_TIME_MASK;
    if !c02_enable {
        ctrl |= MSR_IA32_UMWAIT_CONTROL_C02_DISABLE;
    }
    core::ptr::write_volatile(&mut umwait_control_cached, ctrl);
    on_each_cpu(umwait_update_control_msr, core::ptr::null_mut(), 1);
}

unsafe fn enable_c02_show(
    _dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *mut i8,
) -> isize {
    let ctrl = core::ptr::read_volatile(&umwait_control_cached);
    sprintf(buf, "%d\0".as_ptr() as *const i8, umwait_ctrl_c02_enabled(ctrl) as i32)
}

unsafe fn enable_c02_store(
    _dev: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *const i8,
    count: usize,
) -> isize {
    let mut c02_enable = false;
    let ret = kstrtobool(buf, &mut c02_enable);
    if ret != 0 { return ret as isize; }
    mutex_lock(&mut umwait_lock);
    let ctrl = core::ptr::read_volatile(&umwait_control_cached);
    if c02_enable != umwait_ctrl_c02_enabled(ctrl) {
        umwait_update_control(ctrl, c02_enable);
    }
    mutex_unlock(&mut umwait_lock);
    count as isize
}

unsafe fn max_time_show(
    _kobj: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *mut i8,
) -> isize {
    let ctrl = core::ptr::read_volatile(&umwait_control_cached);
    sprintf(buf, "%u\0".as_ptr() as *const i8, umwait_ctrl_max_time(ctrl))
}

unsafe fn max_time_store(
    _kobj: *mut Device,
    _attr: *mut DeviceAttribute,
    buf: *const i8,
    count: usize,
) -> isize {
    let mut max_time = 0u32;
    let ret = kstrtou32(buf, 0, &mut max_time);
    if ret != 0 { return ret as isize; }
    if max_time & !MSR_IA32_UMWAIT_CONTROL_TIME_MASK != 0 { return -EINVAL as isize; }
    mutex_lock(&mut umwait_lock);
    let ctrl = core::ptr::read_volatile(&umwait_control_cached);
    if max_time != umwait_ctrl_max_time(ctrl) {
        umwait_update_control(max_time, umwait_ctrl_c02_enabled(ctrl));
    }
    mutex_unlock(&mut umwait_lock);
    count as isize
}

// DEVICE_ATTR_RW(enable_c02); DEVICE_ATTR_RW(max_time);
static mut umwait_attrs: [*mut Attribute; 3] = [
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
];

#[repr(C)]
struct AttributeGroup {
    attrs: *mut *mut Attribute,
    name: *const i8,
}

static mut umwait_attr_group: AttributeGroup = AttributeGroup {
    attrs: umwait_attrs.as_mut_ptr(),
    name: b"umwait_control\0".as_ptr() as *const i8,
};

unsafe fn umwait_init() -> i32 {
    let mut ret: i32;
    if !boot_cpu_has(X86_FEATURE_WAITPKG) { return -ENODEV; }
    rdmsrq(MSR_IA32_UMWAIT_CONTROL, &mut orig_umwait_control_cached);
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, b"umwait:online\0".as_ptr() as *const i8,
                            umwait_cpu_online, umwait_cpu_offline);
    if ret < 0 { return ret; }
    register_syscore(&mut umwait_syscore);
    let dev = bus_get_dev_root(&cpu_subsys);
    if !dev.is_null() {
        ret = sysfs_create_group((*dev).kobj(), &umwait_attr_group);
        put_device(dev);
    }
    ret
}

// device_initcall(umwait_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
