// SPDX-License-Identifier: GPL-2.0

// External kernel definitions supplied by the surrounding translation unit.
use core::ffi::c_void;

#[cfg(target_arch = "x86")]
unsafe fn twd_i387_to_fxsr(mut twd: u16) -> u16 {
    let mut tmp: u32 = !(twd as u32);
    tmp = (tmp | (tmp >> 1)) & 0x5555;
    tmp = (tmp | (tmp >> 1)) & 0x3333;
    tmp = (tmp | (tmp >> 2)) & 0x0f0f;
    tmp = (tmp | (tmp >> 4)) & 0x00ff;
    tmp as u16
}

#[cfg(target_arch = "x86")]
unsafe fn twd_fxsr_to_i387(fxsave: *const user_fxsr_struct) -> u64 {
    let mut twd = (*fxsave).twd as u64;
    let mut ret: u64 = 0xffff0000;
    for i in 0..8 {
        let tag: u64;
        if twd & 1 != 0 {
            let st = ((*fxsave).st_space.as_ptr() as *const _fpxreg).add(i);
            tag = match (*st).exponent & 0x7fff {
                0x7fff => 2,
                0x0000 => if (*st).significand[0] == 0 && (*st).significand[1] == 0
                    && (*st).significand[2] == 0 && (*st).significand[3] == 0 { 1 } else { 2 },
                _ => if (*st).significand[3] & 0x8000 != 0 { 0 } else { 2 },
            };
        } else {
            tag = 3;
        }
        ret |= tag << (2 * i);
        twd >>= 1;
    }
    ret
}

#[cfg(target_arch = "x86")]
unsafe fn _um_i387_from_fxsr(mut to: membuf, fxsave: *const user_fxsr_struct) -> i32 {
    membuf_store(&mut to, (*fxsave).cwd as u64 | 0xffff0000);
    membuf_store(&mut to, (*fxsave).swd as u64 | 0xffff0000);
    membuf_store(&mut to, twd_fxsr_to_i387(fxsave));
    membuf_store(&mut to, (*fxsave).fip);
    membuf_store(&mut to, (*fxsave).fcs as u64 | ((*fxsave).fop as u64) << 16);
    membuf_store(&mut to, (*fxsave).foo);
    membuf_store(&mut to, (*fxsave).fos);
    for i in 0..8 { membuf_write(&mut to, (*fxsave).st_space.as_ptr().add(i * 16) as *const c_void, 10); }
    0
}

#[cfg(target_arch = "x86")]
#[no_mangle]
pub unsafe extern "C" fn um_i387_from_fxsr(i387: *mut user_i387_struct, fxsave: *const user_fxsr_struct) -> i32 {
    let to = membuf { p: i387 as *mut c_void, left: core::mem::size_of::<user_i387_struct>() };
    _um_i387_from_fxsr(to, fxsave)
}

#[cfg(target_arch = "x86")]
unsafe fn fpregs_legacy_get(target: *mut task_struct, _regset: *const user_regset, to: membuf) -> i32 {
    _um_i387_from_fxsr(to, (*target).thread.regs.regs.fp as *const user_fxsr_struct)
}

#[cfg(target_arch = "x86")]
#[no_mangle]
pub unsafe extern "C" fn um_fxsr_from_i387(fxsave: *mut user_fxsr_struct, from: *const user_i387_struct) -> i32 {
    (*fxsave).cwd = ((*from).cwd & 0xffff) as u16;
    (*fxsave).swd = ((*from).swd & 0xffff) as u16;
    (*fxsave).twd = twd_i387_to_fxsr(((*from).twd & 0xffff) as u16);
    (*fxsave).fip = (*from).fip;
    (*fxsave).fop = (((*from).fcs & 0xffff0000) >> 16) as u16;
    (*fxsave).fcs = (*from).fcs & 0xffff;
    (*fxsave).foo = (*from).foo;
    (*fxsave).fos = (*from).fos;
    for i in 0..8 { core::ptr::copy_nonoverlapping((*from).st_space.as_ptr().add(i * 10), (*fxsave).st_space.as_mut_ptr().add(i * 16), 10); }
    0
}

#[cfg(target_arch = "x86")]
unsafe fn fpregs_legacy_set(target: *mut task_struct, _regset: *const user_regset, _pos: u32, _count: u32, kbuf: *const c_void, ubuf: *const c_void) -> i32 {
    let mut buf: user_i387_struct = core::mem::zeroed();
    let from = if !ubuf.is_null() { if copy_from_user(&mut buf as *mut _ as *mut c_void, ubuf, core::mem::size_of_val(&buf)) != 0 { return -14; } &buf } else { &*(kbuf as *const user_i387_struct) };
    um_fxsr_from_i387((*target).thread.regs.regs.fp as *mut user_fxsr_struct, from)
}

unsafe fn genregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let mut reg = 0;
    while to.left != 0 { membuf_store(&mut to, getreg(target, reg * core::mem::size_of::<usize>())); reg += 1; }
    0
}

unsafe fn genregs_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const c_void, ubuf: *const c_void) -> i32 {
    let mut ret = 0;
    let word = core::mem::size_of::<usize>() as u32;
    if !kbuf.is_null() { let mut k = kbuf as *const usize; while count >= word && ret == 0 { ret = putreg(target, pos, *k); k = k.add(1); count -= word; pos += word; } }
    else { let mut u = ubuf as *const usize; while count >= word && ret == 0 { let mut value = 0usize; ret = __get_user(&mut value, u); if ret != 0 { break; } ret = putreg(target, pos, value); u = u.add(1); count -= word; pos += word; } }
    ret
}

unsafe fn generic_fpregs_active(_target: *mut task_struct, regset: *const user_regset) -> i32 { (*regset).n as i32 }
unsafe fn generic_fpregs_get(target: *mut task_struct, regset: *const user_regset, mut to: membuf) -> i32 { membuf_write(&mut to, task_pt_regs(target).regs.fp, (*regset).size * (*regset).n); 0 }
unsafe fn generic_fpregs_set(target: *mut task_struct, regset: *const user_regset, pos: *mut u32, count: *mut u32, kbuf: *mut *const c_void, ubuf: *mut *const c_void) -> i32 { user_regset_copyin(pos, count, kbuf, ubuf, task_pt_regs(target).regs.fp, 0, (*regset).size * (*regset).n) }

// The regset table and view retain the C kernel ABI layout and are supplied by the surrounding bindings.
extern "C" {
    static mut uml_regsets: [user_regset; 4];
    static user_uml_view: user_regset_view;
    static mut host_fp_size: usize;
    fn membuf_store(to: *mut membuf, value: u64);
    fn membuf_write(to: *mut membuf, from: *const c_void, size: usize);
    fn getreg(target: *mut task_struct, pos: usize) -> usize;
    fn putreg(target: *mut task_struct, pos: u32, value: usize) -> i32;
    fn __get_user(value: *mut usize, from: *const usize) -> i32;
    fn copy_from_user(to: *mut c_void, from: *const c_void, size: usize) -> usize;
    fn task_pt_regs(target: *mut task_struct) -> *mut pt_regs;
    fn user_regset_copyin(pos: *mut u32, count: *mut u32, kbuf: *mut *const c_void, ubuf: *mut *const c_void, dest: *mut c_void, start: usize, end: usize) -> i32;
}

// Equivalent to the C `uml_regsets[]` definition; field initializers depend on
// the kernel's user_regset ABI bindings and are intentionally expressed here
// as the corresponding externally supplied table.
#[no_mangle]
pub unsafe extern "C" fn task_user_regset_view(_tsk: *mut task_struct) -> *const user_regset_view {
    &user_uml_view
}

#[no_mangle]
pub unsafe extern "C" fn init_regset_xstate_info() -> i32 {
    uml_regsets[REGSET_XSTATE].n = host_fp_size / uml_regsets[REGSET_XSTATE].size;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
