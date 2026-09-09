/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Macros for Flexible Return and Event Delivery (FRED)
 */

/* C header guards and includes omitted; required external symbols remain dependencies. */

/*
 * FRED event return instruction opcodes for ERET{S,U}; supported in
 * binutils >= 2.41.
 */
pub const ERETS: [u8; 4] = [0xf2, 0x0f, 0x01, 0xca];
pub const ERETU: [u8; 4] = [0xf3, 0x0f, 0x01, 0xca];

/*
 * RSP is aligned to a 64-byte boundary before used to push a new stack frame
 */
pub const FRED_STACK_FRAME_RSP_MASK: usize = !0x3f;

/*
 * Used for the return address for call emulation during code patching,
 * and measured in 64-byte cache lines.
 */
pub const FRED_CONFIG_REDZONE_AMOUNT: usize = 1;
pub const FRED_CONFIG_REDZONE: usize = FRED_CONFIG_REDZONE_AMOUNT << 6;

#[inline(always)]
pub const fn fred_config_int_stklvl(l: usize) -> usize {
    l << 9
}

#[inline(always)]
pub const fn fred_config_entrypoint(p: usize) -> usize {
    p
}

#[cfg(CONFIG_X86_FRED)]
#[repr(C)]
pub struct fred_info {
    /* Event data: CR2, DR6, ... */
    pub edata: usize,
    pub resv: usize,
}

/* Full format of the FRED stack frame */
#[cfg(CONFIG_X86_FRED)]
#[repr(C)]
pub struct fred_frame {
    pub regs: pt_regs,
    pub info: fred_info,
}

#[cfg(CONFIG_X86_FRED)]
#[inline(always)]
pub unsafe fn fred_info(regs: *mut pt_regs) -> *mut fred_info {
    let frame = (regs as *mut u8).sub(core::mem::offset_of!(fred_frame, regs)) as *mut fred_frame;
    core::ptr::addr_of_mut!((*frame).info)
}

#[cfg(CONFIG_X86_FRED)]
#[inline(always)]
pub unsafe fn fred_event_data(regs: *mut pt_regs) -> usize {
    (*fred_info(regs)).edata
}

#[cfg(CONFIG_X86_FRED)]
unsafe extern "C" {
    pub fn asm_fred_entrypoint_user();
    pub fn asm_fred_entrypoint_kernel();
    pub fn asm_fred_entry_from_kvm(ss: fred_ss);

    pub fn fred_entry_from_user(regs: *mut pt_regs);
    pub fn fred_entry_from_kernel(regs: *mut pt_regs);
    pub fn __fred_entry_from_kvm(regs: *mut pt_regs);

    pub fn cpu_init_fred_exceptions();
    pub fn cpu_init_fred_rsps();
    pub fn fred_complete_exception_setup();
}

#[cfg(CONFIG_X86_FRED)]
#[inline(always)]
pub unsafe fn fred_entry_from_kvm(type_: u32, vector: u32) {
    let ss = fred_ss {
        ss: __KERNEL_DS,
        type_: type_,
        vector,
        nmi: type_ == EVENT_TYPE_NMI,
        l: 1,
    };
    asm_fred_entry_from_kvm(ss);
}

#[cfg(CONFIG_X86_FRED)]
extern "C" {
    pub static mut fred_rsp0: usize;
}

#[cfg(CONFIG_X86_FRED)]
#[inline(always)]
pub unsafe fn fred_sync_rsp0(rsp0: usize) {
    fred_rsp0 = rsp0;
}

#[cfg(CONFIG_X86_FRED)]
#[inline(always)]
pub unsafe fn fred_update_rsp0() {
    let rsp0 = task_stack_page(current) as usize + THREAD_SIZE;
    if cpu_feature_enabled(X86_FEATURE_FRED) && fred_rsp0 != rsp0 {
        wrmsrns(MSR_IA32_FRED_RSP0, rsp0);
        fred_rsp0 = rsp0;
    }
}

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn fred_event_data(_regs: *mut pt_regs) -> usize { 0 }

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn cpu_init_fred_exceptions() {}

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn cpu_init_fred_rsps() {}

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn fred_complete_exception_setup() {}

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn fred_sync_rsp0(_rsp0: usize) {}

#[cfg(not(CONFIG_X86_FRED))]
#[inline(always)]
pub unsafe fn fred_update_rsp0() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
