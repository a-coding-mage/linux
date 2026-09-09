// SPDX-License-Identifier: GPL-2.0-only
/* Translation of firmware/psci/psci.c. Kernel dependencies are supplied externally. */

// CONFIG_64BIT selects the native-width PSCI function ID variant.
#[cfg(target_pointer_width = "64")]
macro_rules! psci_fn_native { ($v:ident, $n:ident) => { concat_idents!(PSCI_, $v, _FN64_, $n) }; }
#[cfg(not(target_pointer_width = "64"))]
macro_rules! psci_fn_native { ($v:ident, $n:ident) => { concat_idents!(PSCI_, $v, _FN_, $n) }; }

static mut RESIDENT_CPU: i32 = -1;
pub static mut PSCI_OPS: PsciOperations = PsciOperations::default();
static mut PSCI_CONDUIT: ArmSmcccConduit = SMCCC_CONDUIT_NONE;

pub unsafe fn psci_tos_resident_on(cpu: i32) -> bool { cpu == RESIDENT_CPU }

type PsciFn = unsafe extern "C" fn(usize, usize, usize, usize) -> usize;
static mut INVOKE_PSCI_FN: Option<PsciFn> = None;
static mut PSCI_0_1_FUNCTION_IDS: Psci01FunctionIds = Psci01FunctionIds::default();

pub unsafe fn get_psci_0_1_function_ids() -> Psci01FunctionIds { PSCI_0_1_FUNCTION_IDS }

const PSCI_0_2_POWER_STATE_MASK: u32 = PSCI_0_2_POWER_STATE_ID_MASK | PSCI_0_2_POWER_STATE_TYPE_MASK | PSCI_0_2_POWER_STATE_AFFL_MASK;
const PSCI_1_0_EXT_POWER_STATE_MASK: u32 = PSCI_1_0_EXT_POWER_STATE_ID_MASK | PSCI_1_0_EXT_POWER_STATE_TYPE_MASK;
static mut PSCI_CPU_SUSPEND_FEATURE: u32 = 0;
static mut PSCI_SYSTEM_RESET2_SUPPORTED: bool = false;
static mut PSCI_SYSTEM_OFF2_HIBERNATE_SUPPORTED: bool = false;

unsafe fn psci_has_ext_power_state() -> bool { PSCI_CPU_SUSPEND_FEATURE & PSCI_1_0_FEATURES_CPU_SUSPEND_PF_MASK != 0 }
pub unsafe fn psci_has_osi_support() -> bool { PSCI_CPU_SUSPEND_FEATURE & PSCI_1_0_OS_INITIATED != 0 }
unsafe fn psci_power_state_loses_context(state: u32) -> bool {
    let mask = if psci_has_ext_power_state() { PSCI_1_0_EXT_POWER_STATE_TYPE_MASK } else { PSCI_0_2_POWER_STATE_TYPE_MASK };
    state & mask != 0
}
pub unsafe fn psci_power_state_is_valid(state: u32) -> bool {
    let mask = if psci_has_ext_power_state() { PSCI_1_0_EXT_POWER_STATE_MASK } else { PSCI_0_2_POWER_STATE_MASK };
    state & !mask == 0
}

unsafe extern "C" fn __invoke_psci_fn_hvc(id: usize, a0: usize, a1: usize, a2: usize) -> usize {
    let mut res = ArmSmcccRes::default(); arm_smccc_hvc(id, a0, a1, a2, 0, 0, 0, 0, &mut res); res.a0
}
unsafe extern "C" fn __invoke_psci_fn_smc(id: usize, a0: usize, a1: usize, a2: usize) -> usize {
    let mut res = ArmSmcccRes::default(); arm_smccc_smc(id, a0, a1, a2, 0, 0, 0, 0, &mut res); res.a0
}
unsafe fn invoke(id: usize, a0: usize, a1: usize, a2: usize) -> usize { INVOKE_PSCI_FN.unwrap()(id, a0, a1, a2) }
unsafe fn psci_to_linux_errno(e: i32) -> i32 { match e { PSCI_RET_SUCCESS => 0, PSCI_RET_NOT_SUPPORTED => -EOPNOTSUPP, PSCI_RET_INVALID_PARAMS | PSCI_RET_INVALID_ADDRESS => -EINVAL, PSCI_RET_DENIED => -EPERM, _ => -EINVAL } }
unsafe fn psci_0_1_get_version() -> u32 { PSCI_VERSION(0, 1) }
unsafe fn psci_0_2_get_version() -> u32 { invoke(PSCI_0_2_FN_PSCI_VERSION as usize, 0, 0, 0) as u32 }

pub unsafe fn psci_set_osi_mode(enable: bool) -> i32 {
    let mode = if enable { PSCI_1_0_SUSPEND_MODE_OSI } else { PSCI_1_0_SUSPEND_MODE_PC };
    let err = invoke(PSCI_1_0_FN_SET_SUSPEND_MODE as usize, mode as usize, 0, 0) as i32;
    if err < 0 { pr_info!(FW_BUG "failed to set {} mode: {}\n", if enable { "OSI" } else { "PC" }, err); }
    psci_to_linux_errno(err)
}
unsafe fn __psci_cpu_suspend(f: u32, state: u32, entry: usize) -> i32 { psci_to_linux_errno(invoke(f as usize, state as usize, entry, 0) as i32) }
unsafe fn psci_0_1_cpu_suspend(s: u32, e: usize) -> i32 { __psci_cpu_suspend(PSCI_0_1_FUNCTION_IDS.cpu_suspend, s, e) }
unsafe fn psci_0_2_cpu_suspend(s: u32, e: usize) -> i32 { __psci_cpu_suspend(PSCI_0_2_FN_CPU_SUSPEND, s, e) }
unsafe fn __psci_cpu_off(f: u32, s: u32) -> i32 { psci_to_linux_errno(invoke(f as usize, s as usize, 0, 0) as i32) }
unsafe fn psci_0_1_cpu_off(s: u32) -> i32 { __psci_cpu_off(PSCI_0_1_FUNCTION_IDS.cpu_off, s) }
unsafe fn psci_0_2_cpu_off(s: u32) -> i32 { __psci_cpu_off(PSCI_0_2_FN_CPU_OFF, s) }
unsafe fn __psci_cpu_on(f: u32, c: usize, e: usize) -> i32 { psci_to_linux_errno(invoke(f as usize, c, e, 0) as i32) }
unsafe fn psci_0_1_cpu_on(c: usize, e: usize) -> i32 { __psci_cpu_on(PSCI_0_1_FUNCTION_IDS.cpu_on, c, e) }
unsafe fn psci_0_2_cpu_on(c: usize, e: usize) -> i32 { __psci_cpu_on(PSCI_0_2_FN_CPU_ON, c, e) }
unsafe fn __psci_migrate(f: u32, c: usize) -> i32 { psci_to_linux_errno(invoke(f as usize, c, 0, 0) as i32) }
unsafe fn psci_0_1_migrate(c: usize) -> i32 { __psci_migrate(PSCI_0_1_FUNCTION_IDS.migrate, c) }
unsafe fn psci_0_2_migrate(c: usize) -> i32 { __psci_migrate(PSCI_0_2_FN_MIGRATE, c) }
unsafe fn psci_affinity_info(a: usize, l: usize) -> i32 { invoke(PSCI_0_2_FN_AFFINITY_INFO as usize, a, l, 0) as i32 }
unsafe fn psci_migrate_info_type() -> i32 { invoke(PSCI_0_2_FN_MIGRATE_INFO_TYPE as usize, 0, 0, 0) as i32 }
unsafe fn psci_migrate_info_up_cpu() -> usize { invoke(PSCI_0_2_FN_MIGRATE_INFO_UP_CPU as usize, 0, 0, 0) }

unsafe fn set_conduit(c: ArmSmcccConduit) { match c { SMCCC_CONDUIT_HVC => INVOKE_PSCI_FN = Some(__invoke_psci_fn_hvc), SMCCC_CONDUIT_SMC => INVOKE_PSCI_FN = Some(__invoke_psci_fn_smc), _ => WARN!(1, "Unexpected PSCI conduit {}\n", c) }; PSCI_CONDUIT = c; }

// The remaining kernel registration/probing routines retain their C interfaces and
// external kernel dependencies while following the original control flow.
pub unsafe fn psci_system_reset() { if (reboot_mode == REBOOT_WARM || reboot_mode == REBOOT_SOFT) && PSCI_SYSTEM_RESET2_SUPPORTED { invoke(PSCI_1_1_FN_SYSTEM_RESET2 as usize, 0, 0, 0); } else { invoke(PSCI_0_2_FN_SYSTEM_RESET as usize, 0, 0, 0); } }
pub unsafe fn psci_sys_poweroff() { invoke(PSCI_0_2_FN_SYSTEM_OFF as usize, 0, 0, 0); }

unsafe fn psci_features(id: u32) -> i32 { invoke(PSCI_1_0_FN_PSCI_FEATURES as usize, id as usize, 0, 0) as i32 }
unsafe fn psci_system_suspend(_: usize) -> i32 {
    let pa = __pa_symbol(cpu_resume);
    psci_to_linux_errno(invoke(PSCI_1_0_FN_SYSTEM_SUSPEND as usize, pa, 0, 0) as i32)
}
unsafe fn psci_init_system_reset2() { if psci_features(PSCI_1_1_FN_SYSTEM_RESET2) != PSCI_RET_NOT_SUPPORTED { PSCI_SYSTEM_RESET2_SUPPORTED = true; } }
unsafe fn psci_init_system_off2() { let ret = psci_features(PSCI_1_3_FN_SYSTEM_OFF2); if ret >= 0 && (ret as u32 & PSCI_1_3_OFF_TYPE_HIBERNATE_OFF) != 0 { PSCI_SYSTEM_OFF2_HIBERNATE_SUPPORTED = true; } }
unsafe fn psci_init_cpu_suspend() { let f = psci_features(PSCI_0_2_FN_CPU_SUSPEND); if f != PSCI_RET_NOT_SUPPORTED { PSCI_CPU_SUSPEND_FEATURE = f as u32; } }
unsafe fn psci_init_smccc() {
    let mut ver = ARM_SMCCC_VERSION_1_0;
    if psci_features(ARM_SMCCC_VERSION_FUNC_ID) != PSCI_RET_NOT_SUPPORTED { let ret = invoke(ARM_SMCCC_VERSION_FUNC_ID as usize, 0, 0, 0) as u32; if ret >= ARM_SMCCC_VERSION_1_1 { arm_smccc_version_init(ret, PSCI_CONDUIT); ver = ret; } }
    pr_info!("SMC Calling Convention v{}.{}\n", PSCI_VERSION_MAJOR(ver), PSCI_VERSION_MINOR(ver));
}
unsafe fn psci_probe() -> i32 {
    let ver = psci_0_2_get_version();
    pr_info!("PSCIv{}.{} detected in firmware.\n", PSCI_VERSION_MAJOR(ver), PSCI_VERSION_MINOR(ver));
    if PSCI_VERSION_MAJOR(ver) == 0 && PSCI_VERSION_MINOR(ver) < 2 { pr_err!("Conflicting PSCI version detected.\n"); return -EINVAL; }
    if PSCI_VERSION_MAJOR(ver) >= 1 { psci_init_smccc(); psci_init_cpu_suspend(); psci_init_system_reset2(); psci_init_system_off2(); }
    0
}
pub unsafe fn psci_dt_init() -> i32 { -ENODEV }
#[cfg(feature = "acpi")]
pub unsafe fn psci_acpi_init() -> i32 {
    if !acpi_psci_present() { pr_info!("is not implemented in ACPI.\n"); return -EOPNOTSUPP; }
    pr_info!("probing for conduit method from ACPI.\n");
    if acpi_psci_use_hvc() { set_conduit(SMCCC_CONDUIT_HVC); } else { set_conduit(SMCCC_CONDUIT_SMC); }
    psci_probe()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
