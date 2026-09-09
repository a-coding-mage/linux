/* SPDX-License-Identifier: GPL-2.0 */
/* Support for Floating Point and Vector Instructions */

/* C dependencies: asm/fpu-insn-asm.h, linux/instrumented.h,
 * linux/kmsan.h, and asm/asm-extable.h. */

#[inline(always)]
pub unsafe fn fpu_cefbr(f1: u8, val: i32) {
    core::arch::asm!("cefbr {f1}, {val}", f1 = const f1, val = in(reg) val, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_cgebr(f2: u8, mode: u8) -> usize {
    let mut val: usize;
    core::arch::asm!("cgebr {val}, {mode}, {f2}", val = lateout(reg) val, f2 = const f2, mode = const mode, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn fpu_debr(f1: u8, f2: u8) {
    core::arch::asm!("debr {f1}, {f2}", f1 = const f1, f2 = const f2, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_ld(fpr: u16, reg: *mut freg_t) {
    instrument_read(reg.cast(), core::mem::size_of::<freg_t>());
    core::arch::asm!("ld {fpr}, [{reg}]", fpr = const fpr, reg = in(reg) reg, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_ldgr(f1: u8, val: u32) {
    core::arch::asm!("ldgr {f1}, {val}", f1 = const f1, val = in(reg) val, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_lfpc(fpc: *mut u32) {
    instrument_read(fpc.cast(), core::mem::size_of::<u32>());
    core::arch::asm!("lfpc [{fpc}]", fpc = in(reg) fpc, options(nostack));
}

/// Load floating point control register safely; an invalid value traps and is
/// redirected by the external exception-table mechanism.
#[inline(always)]
pub unsafe fn fpu_lfpc_safe(fpc: *mut u32) {
    instrument_read(fpc.cast(), core::mem::size_of::<u32>());
    core::arch::asm!("lfpc [{fpc}]\n0: nopr %r7", fpc = in(reg) fpc, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_std(fpr: u16, reg: *mut freg_t) {
    instrument_write(reg.cast(), core::mem::size_of::<freg_t>());
    core::arch::asm!("std {fpr}, [{reg}]", fpr = const fpr, reg = in(reg) reg, options(nostack));
}

#[inline(always)]
pub unsafe fn fpu_sfpc(fpc: u32) { core::arch::asm!("sfpc {fpc}", fpc = in(reg) fpc, options(nostack)); }

#[inline(always)]
pub unsafe fn fpu_stfpc(fpc: *mut u32) {
    instrument_write(fpc.cast(), core::mem::size_of::<u32>());
    core::arch::asm!("stfpc [{fpc}]", fpc = inout(reg) fpc => _, options(nostack));
}

macro_rules! fpu_vec3 {
    ($name:ident, $insn:literal) => {
        #[inline(always)] pub unsafe fn $name(v1: u8, v2: u8, v3: u8) {
            core::arch::asm!($insn, v1 = const v1, v2 = const v2, v3 = const v3, options(nostack));
        }
    };
}
fpu_vec3!(fpu_vab, "VAB {v1},{v2},{v3}");
fpu_vec3!(fpu_vcksm, "VCKSM {v1},{v2},{v3}");
fpu_vec3!(fpu_vesravb, "VESRAVB {v1},{v2},{v3}");
fpu_vec3!(fpu_vgfmg, "VGFMG {v1},{v2},{v3}");
fpu_vec3!(fpu_vn, "VN {v1},{v2},{v3}");
fpu_vec3!(fpu_vsrlb, "VSRLB {v1},{v2},{v3}");
fpu_vec3!(fpu_vx, "VX {v1},{v2},{v3}");

#[inline(always)] pub unsafe fn fpu_vgfmag(v1:u8,v2:u8,v3:u8,v4:u8) { core::arch::asm!("VGFMAG {v1},{v2},{v3},{v4}", v1=const v1,v2=const v2,v3=const v3,v4=const v4, options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vl(v1:u8, vxr:*const core::ffi::c_void) { instrument_read(vxr, 16); core::arch::asm!("VL {v1},[{vxr}]", v1=const v1,vxr=in(reg) vxr, options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vleib(v:u8,val:i16,index:u8) { core::arch::asm!("VLEIB {v},{val},{index}",v=const v,val=const val,index=const index,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vleig(v:u8,val:i16,index:u8) { core::arch::asm!("VLEIG {v},{val},{index}",v=const v,val=const val,index=const index,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vlgvf(v:u8,index:u16)->u64 { let mut val:u64; core::arch::asm!("VLGVF {val},{v},{index}",val=lateout(reg) val,v=const v,index=const index,options(nostack)); val }
#[inline(always)] pub unsafe fn fpu_vll(v1:u8,index:u32,vxr:*const core::ffi::c_void) { let size=core::cmp::min(index.wrapping_add(1),16); instrument_read(vxr,size as usize); core::arch::asm!("VLL {v1},{index},[{vxr}]",v1=const v1,index=in(reg) index,vxr=in(reg) vxr,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vlr(v1:u8,v2:u8) { core::arch::asm!("VLR {v1},{v2}",v1=const v1,v2=const v2,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vlvgf(v:u8,val:u32,index:u16) { core::arch::asm!("VLVGF {v},{val},{index}",v=const v,val=in(reg) val,index=const index,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vperm(v1:u8,v2:u8,v3:u8,v4:u8) { core::arch::asm!("VPERM {v1},{v2},{v3},{v4}",v1=const v1,v2=const v2,v3=const v3,v4=const v4,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vrepib(v1:u8,i2:i16) { core::arch::asm!("VREPIB {v1},{i2}",v1=const v1,i2=const i2,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vst(v1:u8,vxr:*mut core::ffi::c_void) { instrument_write(vxr,16); core::arch::asm!("VST {v1},[{vxr}]",v1=const v1,vxr=in(reg) vxr,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vstl(v1:u8,index:u32,vxr:*mut core::ffi::c_void) { let size=core::cmp::min(index.wrapping_add(1),16); instrument_write(vxr,size as usize); core::arch::asm!("VSTL {v1},{index},[{vxr}]",v1=const v1,index=in(reg) index,vxr=in(reg) vxr,options(nostack)); kmsan_unpoison_memory(vxr,size as usize); }
#[inline(always)] pub unsafe fn fpu_vupllf(v1:u8,v2:u8) { core::arch::asm!("VUPLLF {v1},{v2}",v1=const v1,v2=const v2,options(nostack)); }
#[inline(always)] pub unsafe fn fpu_vzero(v:u8) { core::arch::asm!("VZERO {v}",v=const v,options(nostack)); }

#[inline(always)]
pub unsafe fn fpu_vlm(v1: u8, v3: u8, vxrs: *const core::ffi::c_void) -> u8 {
    let count = v3.wrapping_sub(v1).wrapping_add(1);
    instrument_read(vxrs, count as usize * 16);
    core::arch::asm!("VLM {v1},{v3},[{vxrs}]", v1=const v1, v3=const v3, vxrs=in(reg) vxrs, options(nostack));
    count
}

#[inline(always)]
pub unsafe fn fpu_vstm(v1: u8, v3: u8, vxrs: *mut core::ffi::c_void) -> u8 {
    let count = v3.wrapping_sub(v1).wrapping_add(1);
    instrument_write(vxrs, count as usize * 16);
    core::arch::asm!("VSTM {v1},{v3},[{vxrs}]", v1=const v1, v3=const v3, vxrs=in(reg) vxrs, options(nostack));
    count
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
