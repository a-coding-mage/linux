/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <uapi/asm/pal.h>

unsafe extern "C" {
    pub fn halt() -> !;
}

#[inline(always)]
pub unsafe fn __halt() {
    core::arch::asm!("call_pal {0} #halt", const PAL_halt, options(nostack));
}

#[inline(always)]
pub unsafe fn imb() {
    core::arch::asm!("call_pal {0} #imb", const PAL_imb, options(nostack));
}

#[inline(always)]
pub unsafe fn draina() {
    core::arch::asm!("call_pal {0} #draina", const PAL_draina, options(nostack));
}

#[inline(always)]
pub unsafe fn cflush(mut arg0: libc::c_ulong) {
    core::arch::asm!(
        "call_pal {1} # cflush",
        inout("$16") arg0,
        const PAL_cflush,
        clobber_abi("C"),
    );
}

#[inline(always)]
pub unsafe fn rdmces() -> libc::c_ulong {
    let r0: libc::c_ulong;
    core::arch::asm!("call_pal {1} # rdmces", out("$0") r0, const PAL_rdmces, clobber_abi("C"));
    r0
}

#[inline(always)]
pub unsafe fn rdps() -> libc::c_ulong {
    let r0: libc::c_ulong;
    core::arch::asm!("call_pal {1} # rdps", out("$0") r0, const PAL_rdps, clobber_abi("C"));
    r0
}

#[inline(always)]
pub unsafe fn rdusp() -> libc::c_ulong {
    let r0: libc::c_ulong;
    core::arch::asm!("call_pal {1} # rdusp", out("$0") r0, const PAL_rdusp, clobber_abi("C"));
    r0
}

#[inline(always)]
pub unsafe fn swpipl(mut arg0: libc::c_ulong) -> libc::c_ulong {
    let r0: libc::c_ulong;
    core::arch::asm!("call_pal {2} # swpipl", inout("$16") arg0, out("$0") r0, const PAL_swpipl, clobber_abi("C"));
    r0
}

#[inline(always)]
pub unsafe fn whami() -> libc::c_ulong {
    let r0: libc::c_ulong;
    core::arch::asm!("call_pal {1} # whami", out("$0") r0, const PAL_whami, clobber_abi("C"));
    r0
}

#[inline(always)]
pub unsafe fn wrent(arg0: *mut core::ffi::c_void, mut arg1: libc::c_ulong) {
    core::arch::asm!("call_pal {2} # wrent", inout("$16") arg0, inout("$17") arg1, const PAL_wrent, clobber_abi("C"));
}

#[inline(always)]
pub unsafe fn wripir(mut arg0: libc::c_ulong) { core::arch::asm!("call_pal {1} # wripir", inout("$16") arg0, const PAL_wripir, clobber_abi("C")); }
#[inline(always)]
pub unsafe fn wrkgp(mut arg0: libc::c_ulong) { core::arch::asm!("call_pal {1} # wrkgp", inout("$16") arg0, const PAL_wrkgp, clobber_abi("C")); }
#[inline(always)]
pub unsafe fn wrmces(mut arg0: libc::c_ulong) { core::arch::asm!("call_pal {1} # wrmces", inout("$16") arg0, const PAL_wrmces, clobber_abi("C")); }
#[inline(always)]
pub unsafe fn wrperfmon(mut arg0: libc::c_ulong, mut arg1: libc::c_ulong) -> libc::c_ulong { let r0: libc::c_ulong; core::arch::asm!("call_pal {3} # wrperfmon", inout("$16") arg0, inout("$17") arg1, out("$0") r0, const PAL_wrperfmon, clobber_abi("C")); r0 }
#[inline(always)]
pub unsafe fn wrusp(mut arg0: libc::c_ulong) { core::arch::asm!("call_pal {1} # wrusp", inout("$16") arg0, const PAL_wrusp, clobber_abi("C")); }
#[inline(always)]
pub unsafe fn wrvptptr(mut arg0: libc::c_ulong) { core::arch::asm!("call_pal {1} # wrvptptr", inout("$16") arg0, const PAL_wrvptptr, clobber_abi("C")); }
#[inline(always)]
pub unsafe fn wtint(mut arg0: libc::c_ulong) -> libc::c_ulong { let r0: libc::c_ulong; core::arch::asm!("call_pal {2} # wtint", inout("$16") arg0, out("$0") r0, const PAL_wtint, clobber_abi("C")); r0 }

/* TB routines.. */
#[inline(always)]
pub unsafe fn tbi(x: libc::c_ulong, y: libc::c_ulong) {
    let mut r16 = x;
    let mut r17 = y;
    core::arch::asm!("call_pal {2} #__tbi", inout("$16") r16, inout("$17") r17, const PAL_tbi, clobber_abi("C"));
}
#[inline(always)] pub unsafe fn tbisi(x: libc::c_ulong) { tbi(1, x); }
#[inline(always)] pub unsafe fn tbisd(x: libc::c_ulong) { tbi(2, x); }
#[inline(always)] pub unsafe fn tbis(x: libc::c_ulong) { tbi(3, x); }
#[inline(always)] pub unsafe fn tbiap() { let mut r16: libc::c_ulong = libc::c_ulong::MAX; core::arch::asm!("call_pal {1} #__tbi", inout("$16") r16, const PAL_tbi, clobber_abi("C")); }
#[inline(always)] pub unsafe fn tbia() { let mut r16: libc::c_ulong = libc::c_ulong::MAX - 1; core::arch::asm!("call_pal {1} #__tbi", inout("$16") r16, const PAL_tbi, clobber_abi("C")); }

/* QEMU Cserv routines.. */
#[inline(always)] pub unsafe fn qemu_get_walltime() -> libc::c_ulong { let mut a0: libc::c_ulong = 3; let v0: libc::c_ulong; core::arch::asm!("call_pal {2} # cserve get_time", out("$0") v0, inout("$16") a0, const PAL_cserve, clobber_abi("C")); v0 }
#[inline(always)] pub unsafe fn qemu_get_alarm() -> libc::c_ulong { let mut a0: libc::c_ulong = 4; let v0: libc::c_ulong; core::arch::asm!("call_pal {2} # cserve get_alarm", out("$0") v0, inout("$16") a0, const PAL_cserve, clobber_abi("C")); v0 }
#[inline(always)] pub unsafe fn qemu_set_alarm_rel(expire: libc::c_ulong) { let mut a0: libc::c_ulong = 5; let mut a1 = expire; core::arch::asm!("call_pal {2} # cserve set_alarm_rel", inout("$16") a0, inout("$17") a1, const PAL_cserve, clobber_abi("C")); }
#[inline(always)] pub unsafe fn qemu_set_alarm_abs(expire: libc::c_ulong) { let mut a0: libc::c_ulong = 6; let mut a1 = expire; core::arch::asm!("call_pal {2} # cserve set_alarm_abs", inout("$16") a0, inout("$17") a1, const PAL_cserve, clobber_abi("C")); }
#[inline(always)] pub unsafe fn qemu_get_vmtime() -> libc::c_ulong { let mut a0: libc::c_ulong = 7; let v0: libc::c_ulong; core::arch::asm!("call_pal {2} # cserve get_time", out("$0") v0, inout("$16") a0, const PAL_cserve, clobber_abi("C")); v0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
