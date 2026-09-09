// SPDX-License-Identifier: GPL-2.0-only

// Kernel headers and architecture-provided symbols are supplied by other files.

#[repr(C)]
struct X86VirtOps {
    feature: i32,
    enable_virtualization_cpu: Option<unsafe extern "C" fn() -> i32>,
    disable_virtualization_cpu: Option<unsafe extern "C" fn() -> i32>,
    emergency_disable_virtualization_cpu: Option<unsafe extern "C" fn()>,
}

static mut VIRT_OPS: X86VirtOps = X86VirtOps {
    feature: 0,
    enable_virtualization_cpu: None,
    disable_virtualization_cpu: None,
    emergency_disable_virtualization_cpu: None,
};

#[no_mangle]
pub static mut virt_rebooting: bool = false;

static mut KVM_EMERGENCY_CALLBACK: Option<unsafe extern "C" fn()> = None;
static mut VIRTUALIZATION_NR_USERS: i32 = 0;

pub unsafe extern "C" fn x86_virt_register_emergency_callback(
    callback: unsafe extern "C" fn(),
) {
    if KVM_EMERGENCY_CALLBACK.is_some() {
        return;
    }
    KVM_EMERGENCY_CALLBACK = Some(callback);
}

pub unsafe extern "C" fn x86_virt_unregister_emergency_callback(
    callback: unsafe extern "C" fn(),
) {
    if KVM_EMERGENCY_CALLBACK != Some(callback) {
        return;
    }
    KVM_EMERGENCY_CALLBACK = None;
    synchronize_rcu();
}

unsafe fn x86_virt_invoke_kvm_emergency_callback() {
    // rcu_dereference_raw() is used in the original because RCU may not be
    // watching the crashing CPU and panic context cannot guarantee correctness.
    if let Some(callback) = KVM_EMERGENCY_CALLBACK {
        callback();
    }
}

#[cfg(CONFIG_KVM_INTEL)]
static mut ROOT_VMCS: *mut Vmcs = core::ptr::null_mut();

#[cfg(CONFIG_KVM_INTEL)]
unsafe fn x86_virt_cpu_vmxon() -> i32 {
    let vmxon_pointer = __pa(ROOT_VMCS as *const _);
    let mut msr: u64 = 0;
    cr4_set_bits(X86_CR4_VMXE);
    // The VMXON instruction and exception-table recovery are architecture
    // assembly supplied by the kernel.
    if vmxon_arch(vmxon_pointer) != 0 {
        warn_once(rdmsrq_safe(MSR_IA32_FEAT_CTL, &mut msr) != 0, msr);
        cr4_clear_bits(X86_CR4_VMXE);
        return -EFAULT;
    }
    0
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe extern "C" fn x86_vmx_enable_virtualization_cpu() -> i32 {
    if cr4_read_shadow() & X86_CR4_VMXE != 0 {
        return -EBUSY;
    }
    intel_pt_handle_vmx(1);
    let r = x86_virt_cpu_vmxon();
    if r != 0 {
        intel_pt_handle_vmx(0);
    }
    r
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe extern "C" fn x86_vmx_disable_virtualization_cpu() -> i32 {
    let mut r = -EIO;
    if vmxoff_arch() == 0 {
        r = 0;
    }
    cr4_clear_bits(X86_CR4_VMXE);
    intel_pt_handle_vmx(0);
    r
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe extern "C" fn x86_vmx_emergency_disable_virtualization_cpu() {
    virt_rebooting = true;
    if __read_cr4() & X86_CR4_VMXE == 0 {
        return;
    }
    x86_virt_invoke_kvm_emergency_callback();
    x86_vmx_disable_virtualization_cpu();
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe extern "C" fn x86_vmx_exit() {
    free_page(ROOT_VMCS as usize);
    ROOT_VMCS = core::ptr::null_mut();
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe fn __x86_vmx_init() -> i32 {
    let vmx_ops = X86VirtOps {
        feature: X86_FEATURE_VMX,
        enable_virtualization_cpu: Some(x86_vmx_enable_virtualization_cpu),
        disable_virtualization_cpu: Some(x86_vmx_disable_virtualization_cpu),
        emergency_disable_virtualization_cpu: Some(x86_vmx_emergency_disable_virtualization_cpu),
    };
    if !cpu_feature_enabled(X86_FEATURE_VMX) {
        return -EOPNOTSUPP;
    }
    let basic_msr = rdmsrq(MSR_IA32_VMX_BASIC);
    if vmx_basic_vmcs_size(basic_msr) > PAGE_SIZE {
        return -EIO;
    }
    let rev_id = vmx_basic_vmcs_revision_id(basic_msr);
    let page = alloc_pages_node(cpu_to_node(raw_smp_processor_id()), GFP_KERNEL | __GFP_ZERO, 0);
    if page.is_null() {
        x86_vmx_exit();
        return -ENOMEM;
    }
    let vmcs = page_address(page) as *mut Vmcs;
    (*vmcs).hdr.revision_id = rev_id;
    ROOT_VMCS = vmcs;
    VIRT_OPS = vmx_ops;
    0
}

#[cfg(CONFIG_KVM_INTEL)]
unsafe extern "C" fn x86_vmx_init() -> i32 {
    let r = __x86_vmx_init();
    if r != 0 { setup_clear_cpu_cap(X86_FEATURE_VMX); }
    r
}

#[cfg(not(CONFIG_KVM_INTEL))]
unsafe extern "C" fn x86_vmx_init() -> i32 { -EOPNOTSUPP }
#[cfg(not(CONFIG_KVM_INTEL))]
unsafe extern "C" fn x86_vmx_exit() {}

#[cfg(CONFIG_KVM_AMD)]
unsafe extern "C" fn x86_svm_enable_virtualization_cpu() -> i32 {
    let efer = rdmsrq(MSR_EFER);
    if efer & EFER_SVME != 0 { return -EBUSY; }
    wrmsrq(MSR_EFER, efer | EFER_SVME);
    0
}

#[cfg(CONFIG_KVM_AMD)]
unsafe extern "C" fn x86_svm_disable_virtualization_cpu() -> i32 {
    let mut r = -EIO;
    if stgi_arch() == 0 { r = 0; }
    let efer = rdmsrq(MSR_EFER);
    wrmsrq(MSR_EFER, efer & !EFER_SVME);
    r
}

#[cfg(CONFIG_KVM_AMD)]
unsafe extern "C" fn x86_svm_emergency_disable_virtualization_cpu() {
    virt_rebooting = true;
    let efer = rdmsrq(MSR_EFER);
    if efer & EFER_SVME == 0 { return; }
    x86_virt_invoke_kvm_emergency_callback();
    x86_svm_disable_virtualization_cpu();
}

#[cfg(CONFIG_KVM_AMD)]
unsafe extern "C" fn x86_svm_init() -> i32 {
    let svm_ops = X86VirtOps {
        feature: X86_FEATURE_SVM,
        enable_virtualization_cpu: Some(x86_svm_enable_virtualization_cpu),
        disable_virtualization_cpu: Some(x86_svm_disable_virtualization_cpu),
        emergency_disable_virtualization_cpu: Some(x86_svm_emergency_disable_virtualization_cpu),
    };
    if !cpu_feature_enabled(X86_FEATURE_SVM) || cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        return -EOPNOTSUPP;
    }
    VIRT_OPS = svm_ops;
    0
}

#[cfg(not(CONFIG_KVM_AMD))]
unsafe extern "C" fn x86_svm_init() -> i32 { -EOPNOTSUPP }

pub unsafe extern "C" fn x86_virt_get_ref(feat: i32) -> i32 {
    if VIRT_OPS.feature == 0 || VIRT_OPS.feature != feat { return -EOPNOTSUPP; }
    VIRTUALIZATION_NR_USERS = VIRTUALIZATION_NR_USERS.wrapping_add(1);
    if VIRTUALIZATION_NR_USERS > 1 { return 0; }
    let r = (VIRT_OPS.enable_virtualization_cpu.unwrap())();
    if r != 0 { VIRTUALIZATION_NR_USERS = VIRTUALIZATION_NR_USERS.wrapping_sub(1); }
    r
}

pub unsafe extern "C" fn x86_virt_put_ref(_feat: i32) {
    if VIRTUALIZATION_NR_USERS == 0 { return; }
    VIRTUALIZATION_NR_USERS -= 1;
    if VIRTUALIZATION_NR_USERS != 0 { return; }
    let r = (VIRT_OPS.disable_virtualization_cpu.unwrap())();
    if r != 0 && !virt_rebooting { bug(); }
}

pub unsafe extern "C" fn x86_virt_emergency_disable_virtualization_cpu() -> i32 {
    if VIRT_OPS.feature == 0 { return -EOPNOTSUPP; }
    lockdep_assert_irqs_disabled();
    (VIRT_OPS.emergency_disable_virtualization_cpu.unwrap())();
    0
}

pub unsafe extern "C" fn x86_virt_init() {
    let has_vmx = x86_vmx_init() == 0;
    let has_svm = x86_svm_init() == 0;
    if has_vmx && has_svm {
        x86_vmx_exit();
        VIRT_OPS = X86VirtOps { feature: 0, enable_virtualization_cpu: None, disable_virtualization_cpu: None, emergency_disable_virtualization_cpu: None };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
