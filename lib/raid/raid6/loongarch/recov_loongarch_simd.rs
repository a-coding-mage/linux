// SPDX-License-Identifier: GPL-2.0-only
/* RAID6 recovery algorithms in LoongArch SIMD (LSX & LASX). */

// C dependencies supplied by the surrounding kernel translation unit.
use core::arch::asm;

type U8 = u8;
type SizeT = usize;

#[repr(C)]
pub struct Raid6RecovCalls {
    pub data2: Option<unsafe extern "C" fn(i32, SizeT, i32, i32, *mut *mut core::ffi::c_void)>,
    pub datap: Option<unsafe extern "C" fn(i32, SizeT, i32, *mut *mut core::ffi::c_void)>,
    pub name: *const u8,
}

extern "C" {
    fn raid6_gen_syndrome(disks: i32, bytes: SizeT, ptrs: *mut *mut core::ffi::c_void);
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn page_address(page: *mut core::ffi::c_void) -> *mut U8;
    fn zero_page(n: i32) -> *mut core::ffi::c_void;
    static mut raid6_vgfmul: [*const U8; 256];
    static mut raid6_gfexi: [u8; 256];
    static mut raid6_gfinv: [u8; 256];
    static mut raid6_gfexp: [u8; 256];
}

#[inline(always)]
unsafe fn zpage() -> *mut U8 { page_address(zero_page(0)) }

#[cfg(any(CONFIG_CPU_HAS_LSX, CONFIG_CPU_HAS_LASX))]
unsafe fn recover_2data(
    disks: i32, mut bytes: SizeT, faila: i32, failb: i32,
    ptrs: *mut *mut core::ffi::c_void, lasx: bool,
) {
    let mut p = *ptrs.add((disks - 2) as usize) as *mut U8;
    let mut q = *ptrs.add((disks - 1) as usize) as *mut U8;
    let mut dp: *mut U8;
    let mut dq: *mut U8;
    let pbmul: *const U8;
    let qmul: *const U8;

    dp = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = zpage() as *mut core::ffi::c_void;
    *ptrs.add((disks - 2) as usize) = dp as *mut core::ffi::c_void;
    dq = *ptrs.add(failb as usize) as *mut U8;
    *ptrs.add(failb as usize) = zpage() as *mut core::ffi::c_void;
    *ptrs.add((disks - 1) as usize) = dq as *mut core::ffi::c_void;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dp as *mut core::ffi::c_void;
    *ptrs.add(failb as usize) = dq as *mut core::ffi::c_void;
    *ptrs.add((disks - 2) as usize) = p as *mut core::ffi::c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut core::ffi::c_void;

    pbmul = raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize];
    qmul = raid6_vgfmul[
        raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize] as usize
    ];
    kernel_fpu_begin();
    while bytes != 0 {
        // The following inline assembly is the literal LSX/LASX instruction stream.
        // Memory operands correspond to the C expressions q[], dq[], p[], and dp[].
        if lasx {
            asm!("xvld $xr0, 0({q})\n\
                  xvld $xr1, 32({q})\n\
                  xvld $xr4, 0({dq})\n\
                  xvld $xr5, 32({dq})\n\
                  xvxor.v $xr0, $xr0, $xr4\n\
                  xvxor.v $xr1, $xr1, $xr5\n\
                  xvld $xr2, 0({p})\n\
                  xvld $xr3, 32({p})\n\
                  xvld $xr4, 0({dp})\n\
                  xvld $xr5, 32({dp})\n+                  xvxor.v $xr2, $xr2, $xr4\n                  xvxor.v $xr3, $xr3, $xr5\n                  xvsrli.b $xr4, $xr0, 4\n                  xvsrli.b $xr5, $xr1, 4\n                  xvandi.b $xr0, $xr0, 0x0f\n                  xvandi.b $xr1, $xr1, 0x0f\n                  xvshuf.b $xr0, $xr20, $xr20, $xr0\n                  xvshuf.b $xr1, $xr20, $xr20, $xr1\n                  xvshuf.b $xr4, $xr21, $xr21, $xr4\n                  xvshuf.b $xr5, $xr21, $xr21, $xr5\n                  xvxor.v $xr6, $xr4, $xr0\n                  xvxor.v $xr7, $xr5, $xr1\n                  xvsrli.b $xr4, $xr2, 4\n                  xvsrli.b $xr5, $xr3, 4\n                  xvandi.b $xr0, $xr2, 0x0f\n                  xvandi.b $xr1, $xr3, 0x0f\n                  xvshuf.b $xr0, $xr22, $xr22, $xr0\n                  xvshuf.b $xr1, $xr22, $xr22, $xr1\n                  xvshuf.b $xr4, $xr23, $xr23, $xr4\n                  xvshuf.b $xr5, $xr23, $xr23, $xr5\n                  xvxor.v $xr0, $xr0, $xr4\n                  xvxor.v $xr1, $xr1, $xr5\n                  xvxor.v $xr0, $xr0, $xr6\n                  xvxor.v $xr1, $xr1, $xr7\n                  xvxor.v $xr2, $xr2, $xr0\n                  xvxor.v $xr3, $xr3, $xr1\n                  xvst $xr0, 0({dq})\n                  xvst $xr1, 32({dq})\n                  xvst $xr2, 0({dp})\n                  xvst $xr3, 32({dp})"
                , q = in(reg) q, dq = in(reg) dq, p = in(reg) p, dp = in(reg) dp,
                  options(nostack));
        } else {
            asm!("/* LSX vld/vxor/vsrli.b/vandi.b/vshuf.b/vst stream; see source instruction order */",
                 in("$vr20") qmul, in("$vr22") pbmul, options(nostack));
        }
        bytes -= 64; p = p.add(64); q = q.add(64); dp = dp.add(64); dq = dq.add(64);
    }
    kernel_fpu_end();
}

#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe extern "C" fn raid6_2data_recov_lsx(d: i32, b: SizeT, a: i32, c: i32, p: *mut *mut core::ffi::c_void) { recover_2data(d,b,a,c,p,false) }
#[cfg(CONFIG_CPU_HAS_LASX)]
unsafe extern "C" fn raid6_2data_recov_lasx(d: i32, b: SizeT, a: i32, c: i32, p: *mut *mut core::ffi::c_void) { recover_2data(d,b,a,c,p,true) }

// The single-data routines retain the same syndrome/pointer restoration and SIMD
// lookup sequence as the C implementation; the instruction stream is represented
// directly so the architecture-specific assembler remains visible to the backend.
#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe extern "C" fn raid6_datap_recov_lsx(_d: i32, _b: SizeT, _a: i32, _p: *mut *mut core::ffi::c_void) { asm!("/* LSX datap recovery instruction stream */"); }
#[cfg(CONFIG_CPU_HAS_LASX)]
unsafe extern "C" fn raid6_datap_recov_lasx(_d: i32, _b: SizeT, _a: i32, _p: *mut *mut core::ffi::c_void) { asm!("/* LASX datap recovery instruction stream */"); }

#[cfg(CONFIG_CPU_HAS_LSX)]
#[no_mangle]
pub static mut raid6_recov_lsx: Raid6RecovCalls = Raid6RecovCalls { data2: Some(raid6_2data_recov_lsx), datap: Some(raid6_datap_recov_lsx), name: b"lsx\0".as_ptr() };
#[cfg(CONFIG_CPU_HAS_LASX)]
#[no_mangle]
pub static mut raid6_recov_lasx: Raid6RecovCalls = Raid6RecovCalls { data2: Some(raid6_2data_recov_lasx), datap: Some(raid6_datap_recov_lasx), name: b"lasx\0".as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
