// SPDX-License-Identifier: GPL-2.0-or-later

// Symbols and types below are supplied by the corresponding architecture
// headers and other translation units.

extern "C" {
    fn update_user_segments(value: u32);
    fn mfsr(segment: u32) -> u32;
    fn isync();
    fn smp_processor_id() -> u32;
    fn pr_info(message: *const core::ffi::c_char);

    static mut init_mm: MmStruct;
    static mut current: *mut TaskStruct;
    static mut cur_cpu_spec: *mut CpuSpec;
    static boot_cpuid: u32;
}

extern "C" {
    static SR_KS: u32;
    static MMU_FTR_KUAP: u32;
}

#[repr(C)]
pub struct MmContext {
    pub sr0: u32,
}

#[repr(C)]
pub struct MmStruct {
    pub context: MmContext,
}

#[repr(C)]
pub struct ThreadStruct {
    pub sr0: u32,
}

#[repr(C)]
pub struct TaskStruct {
    pub thread: ThreadStruct,
}

#[repr(C)]
pub struct CpuSpec {
    pub mmu_features: u32,
}

pub unsafe fn setup_kuap(disabled: bool) {
    if !disabled {
        update_user_segments(mfsr(0) | SR_KS);
        isync(); // Context sync required after mtsr()
        init_mm.context.sr0 |= SR_KS;
        (*current).thread.sr0 |= SR_KS;
    }

    if smp_processor_id() != boot_cpuid {
        return;
    }

    if disabled {
        (*cur_cpu_spec).mmu_features &= !MMU_FTR_KUAP;
    } else {
        pr_info(b"Activating Kernel Userspace Access Protection\n\0".as_ptr()
            as *const core::ffi::c_char);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
