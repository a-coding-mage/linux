/* SPDX-License-Identifier: GPL-2.0 */
/* In-kernel FPU support functions. */

/* C header includes and build-time kernel dependencies are supplied externally. */

pub const KERNEL_FPC_BIT: u32 = 0;
pub const KERNEL_VXR_V0V7_BIT: u32 = 1;
pub const KERNEL_VXR_V8V15_BIT: u32 = 2;
pub const KERNEL_VXR_V16V23_BIT: u32 = 3;
pub const KERNEL_VXR_V24V31_BIT: u32 = 4;

pub const KERNEL_FPC: i32 = 1 << KERNEL_FPC_BIT;
pub const KERNEL_VXR_V0V7: i32 = 1 << KERNEL_VXR_V0V7_BIT;
pub const KERNEL_VXR_V8V15: i32 = 1 << KERNEL_VXR_V8V15_BIT;
pub const KERNEL_VXR_V16V23: i32 = 1 << KERNEL_VXR_V16V23_BIT;
pub const KERNEL_VXR_V24V31: i32 = 1 << KERNEL_VXR_V24V31_BIT;
pub const KERNEL_VXR_LOW: i32 = KERNEL_VXR_V0V7 | KERNEL_VXR_V8V15;
pub const KERNEL_VXR_MID: i32 = KERNEL_VXR_V8V15 | KERNEL_VXR_V16V23;
pub const KERNEL_VXR_HIGH: i32 = KERNEL_VXR_V16V23 | KERNEL_VXR_V24V31;
pub const KERNEL_VXR: i32 = KERNEL_VXR_LOW | KERNEL_VXR_HIGH;
pub const KERNEL_FPR: i32 = KERNEL_FPC | KERNEL_VXR_LOW;

extern "C" {
    pub fn load_fpu_state(state: *mut fpu, flags: i32);
    pub fn save_fpu_state(state: *mut fpu, flags: i32);
    pub fn __kernel_fpu_begin(state: *mut kernel_fpu, flags: i32);
    pub fn __kernel_fpu_end(state: *mut kernel_fpu, flags: i32);
    pub fn __kernel_fpu_invalid_size();
}

pub unsafe fn save_vx_regs(vxrs: *mut __vector128) {
    fpu_vstm(0, 15, vxrs.add(0));
    fpu_vstm(16, 31, vxrs.add(16));
}

pub unsafe fn load_vx_regs(vxrs: *mut __vector128) {
    fpu_vlm(0, 15, vxrs.add(0));
    fpu_vlm(16, 31, vxrs.add(16));
}

pub unsafe fn __save_fp_regs(fprs: *mut freg_t, offset: usize) {
    fpu_std(0, fprs.add(0 * offset)); fpu_std(1, fprs.add(1 * offset));
    fpu_std(2, fprs.add(2 * offset)); fpu_std(3, fprs.add(3 * offset));
    fpu_std(4, fprs.add(4 * offset)); fpu_std(5, fprs.add(5 * offset));
    fpu_std(6, fprs.add(6 * offset)); fpu_std(7, fprs.add(7 * offset));
    fpu_std(8, fprs.add(8 * offset)); fpu_std(9, fprs.add(9 * offset));
    fpu_std(10, fprs.add(10 * offset)); fpu_std(11, fprs.add(11 * offset));
    fpu_std(12, fprs.add(12 * offset)); fpu_std(13, fprs.add(13 * offset));
    fpu_std(14, fprs.add(14 * offset)); fpu_std(15, fprs.add(15 * offset));
}

pub unsafe fn __load_fp_regs(fprs: *mut freg_t, offset: usize) {
    fpu_ld(0, fprs.add(0 * offset)); fpu_ld(1, fprs.add(1 * offset));
    fpu_ld(2, fprs.add(2 * offset)); fpu_ld(3, fprs.add(3 * offset));
    fpu_ld(4, fprs.add(4 * offset)); fpu_ld(5, fprs.add(5 * offset));
    fpu_ld(6, fprs.add(6 * offset)); fpu_ld(7, fprs.add(7 * offset));
    fpu_ld(8, fprs.add(8 * offset)); fpu_ld(9, fprs.add(9 * offset));
    fpu_ld(10, fprs.add(10 * offset)); fpu_ld(11, fprs.add(11 * offset));
    fpu_ld(12, fprs.add(12 * offset)); fpu_ld(13, fprs.add(13 * offset));
    fpu_ld(14, fprs.add(14 * offset)); fpu_ld(15, fprs.add(15 * offset));
}

pub unsafe fn save_fp_regs(fprs: *mut freg_t) { __save_fp_regs(fprs, 1); }
pub unsafe fn load_fp_regs(fprs: *mut freg_t) { __load_fp_regs(fprs, 1); }

pub unsafe fn save_fp_regs_vx(vxrs: *mut __vector128) {
    let fprs = core::ptr::addr_of_mut!((*vxrs).high) as *mut freg_t;
    __save_fp_regs(fprs, core::mem::size_of::<__vector128>() / core::mem::size_of::<freg_t>());
}

pub unsafe fn load_fp_regs_vx(vxrs: *mut __vector128) {
    let fprs = core::ptr::addr_of_mut!((*vxrs).high) as *mut freg_t;
    __load_fp_regs(fprs, core::mem::size_of::<__vector128>() / core::mem::size_of::<freg_t>());
}

pub unsafe fn load_user_fpu_regs() {
    let thread = &mut (*current).thread;
    if thread.ufpu_flags == 0 { return; }
    load_fpu_state(&mut thread.ufpu, thread.ufpu_flags);
    thread.ufpu_flags = 0;
}

pub unsafe fn __save_user_fpu_regs(thread: *mut thread_struct, flags: i32) {
    save_fpu_state(&mut (*thread).ufpu, flags);
    __atomic_or(flags, &mut (*thread).ufpu_flags);
}

pub unsafe fn save_user_fpu_regs() {
    let thread = &mut (*current).thread;
    let mask = __atomic_or(KERNEL_FPC | KERNEL_VXR, &mut thread.kfpu_flags);
    let flags = !READ_ONCE(thread.ufpu_flags) & (KERNEL_FPC | KERNEL_VXR);
    if flags != 0 { __save_user_fpu_regs(thread, flags); }
    barrier();
    WRITE_ONCE(thread.kfpu_flags, mask);
}

pub unsafe fn _kernel_fpu_begin(state: *mut kernel_fpu, flags: i32) {
    let thread = &mut (*current).thread;
    let mask = __atomic_or(flags, &mut thread.kfpu_flags);
    (*state).hdr.mask = mask;
    let uflags = READ_ONCE(thread.ufpu_flags);
    if (uflags & flags) != flags { __save_user_fpu_regs(thread, !uflags & flags); }
    if mask & flags != 0 { __kernel_fpu_begin(state, flags); }
}

pub unsafe fn _kernel_fpu_end(state: *mut kernel_fpu, flags: i32) {
    let mask = (*state).hdr.mask;
    if mask & flags != 0 { __kernel_fpu_end(state, flags); }
    barrier();
    WRITE_ONCE((*current).thread.kfpu_flags, mask);
}

pub unsafe fn kernel_fpu_check_size(flags: i32, size: u32) {
    let mut cnt = 0;
    if flags & KERNEL_VXR_V0V7 != 0 { cnt += 8; }
    if flags & KERNEL_VXR_V8V15 != 0 { cnt += 8; }
    if flags & KERNEL_VXR_V16V23 != 0 { cnt += 8; }
    if flags & KERNEL_VXR_V24V31 != 0 { cnt += 8; }
    if cnt != size { __kernel_fpu_invalid_size(); }
}

pub unsafe fn save_kernel_fpu_regs(thread: *mut thread_struct) {
    if (*thread).kfpu_flags == 0 { return; }
    save_fpu_state(&mut (*thread).kfpu, (*thread).kfpu_flags);
}

pub unsafe fn restore_kernel_fpu_regs(thread: *mut thread_struct) {
    if (*thread).kfpu_flags == 0 { return; }
    load_fpu_state(&mut (*thread).kfpu, (*thread).kfpu_flags);
}

pub unsafe fn convert_vx_to_fp(fprs: *mut freg_t, vxrs: *mut __vector128) {
    for i in 0..__NUM_FPRS { (*fprs.add(i)).ui = (*vxrs.add(i)).high; }
}

pub unsafe fn convert_fp_to_vx(vxrs: *mut __vector128, fprs: *mut freg_t) {
    for i in 0..__NUM_FPRS { (*vxrs.add(i)).high = (*fprs.add(i)).ui; }
}

pub unsafe fn fpregs_store(fpregs: *mut _s390_fp_regs, fpu: *mut fpu) {
    (*fpregs).pad = 0;
    (*fpregs).fpc = (*fpu).fpc;
    convert_vx_to_fp(core::ptr::addr_of_mut!((*fpregs).fprs) as *mut freg_t, (*fpu).vxrs.as_mut_ptr());
}

pub unsafe fn fpregs_load(fpregs: *mut _s390_fp_regs, fpu: *mut fpu) {
    (*fpu).fpc = (*fpregs).fpc;
    convert_fp_to_vx((*fpu).vxrs.as_mut_ptr(), core::ptr::addr_of_mut!((*fpregs).fprs) as *mut freg_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
