// Translated from linux/sched/isolation.h.
// Dependencies corresponding to the C includes are supplied externally.

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HkType {
    // Inverse of boot-time isolcpus= argument
    HK_TYPE_DOMAIN_BOOT,
    // Same as HK_TYPE_DOMAIN_BOOT but also includes the
    // inverse of cpuset isolated partitions. As such it
    // is always a subset of HK_TYPE_DOMAIN_BOOT.
    HK_TYPE_DOMAIN,
    // Inverse of boot-time isolcpus=managed_irq argument
    HK_TYPE_MANAGED_IRQ,
    // Inverse of boot-time nohz_full= or isolcpus=nohz arguments
    HK_TYPE_KERNEL_NOISE,
    HK_TYPE_MAX,

    // HK_TYPE_KTHREAD is now an alias of HK_TYPE_DOMAIN
    HK_TYPE_KTHREAD = HkType::HK_TYPE_DOMAIN as isize,

    // The following housekeeping types are only set by the nohz_full
    // boot commandline option. So they can share the same value.
    HK_TYPE_TICK = HkType::HK_TYPE_KERNEL_NOISE as isize,
    HK_TYPE_TIMER = HkType::HK_TYPE_KERNEL_NOISE as isize,
    HK_TYPE_RCU = HkType::HK_TYPE_KERNEL_NOISE as isize,
    HK_TYPE_MISC = HkType::HK_TYPE_KERNEL_NOISE as isize,
    HK_TYPE_WQ = HkType::HK_TYPE_KERNEL_NOISE as isize,
}

// CONFIG_CPU_ISOLATION selects the externally provided implementations below.
#[cfg(CONFIG_CPU_ISOLATION)]
extern "C" {
    pub static housekeeping_overridden: StaticKeyFalse;
    pub fn housekeeping_any_cpu(type_: HkType) -> i32;
    pub fn housekeeping_cpumask(type_: HkType) -> *const Cpumask;
    pub fn housekeeping_enabled(type_: HkType) -> bool;
    pub fn housekeeping_affine(t: *mut TaskStruct, type_: HkType);
    pub fn housekeeping_test_cpu(cpu: i32, type_: HkType) -> bool;
    pub fn housekeeping_update(isol_mask: *mut Cpumask) -> i32;
    pub fn housekeeping_init();
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub unsafe fn housekeeping_any_cpu(_type_: HkType) -> i32 {
    smp_processor_id()
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub unsafe fn housekeeping_cpumask(_type_: HkType) -> *const Cpumask {
    &cpu_possible_mask
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub fn housekeeping_enabled(_type_: HkType) -> bool {
    false
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub unsafe fn housekeeping_affine(_t: *mut TaskStruct, _type_: HkType) {}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub fn housekeeping_test_cpu(_cpu: i32, _type_: HkType) -> bool {
    true
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub unsafe fn housekeeping_update(_isol_mask: *mut Cpumask) -> i32 {
    0
}

#[cfg(not(CONFIG_CPU_ISOLATION))]
#[inline]
pub unsafe fn housekeeping_init() {}

#[inline]
pub unsafe fn housekeeping_cpu(cpu: i32, type_: HkType) -> bool {
    #[cfg(CONFIG_CPU_ISOLATION)]
    {
        if static_branch_unlikely(&housekeeping_overridden) {
            return housekeeping_test_cpu(cpu, type_);
        }
    }
    true
}

#[inline]
pub unsafe fn cpu_is_isolated(cpu: i32) -> bool {
    !housekeeping_test_cpu(cpu, HkType::HK_TYPE_DOMAIN)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
