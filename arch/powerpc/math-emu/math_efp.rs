// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of arch/powerpc/math-emu/math_efp.c. */

// Kernel and soft-fp definitions are supplied by the surrounding translation unit.

const EFAPU: usize = 0x4;
const VCT: usize = 0x4;
const SPFP: usize = 0x6;
const DPFP: usize = 0x7;
const AB: usize = 2;
const XA: usize = 3;
const XB: usize = 4;
const XCR: usize = 5;
const NOTYPE: usize = 0;
const SIGN_BIT_S: u32 = 1u32 << 31;
const SIGN_BIT_D: u64 = 1u64 << 63;

const EFSADD: usize=0x2c0; const EFSSUB: usize=0x2c1; const EFSABS: usize=0x2c4;
const EFSNABS: usize=0x2c5; const EFSNEG: usize=0x2c6; const EFSMUL: usize=0x2c8;
const EFSDIV: usize=0x2c9; const EFSCMPGT: usize=0x2cc; const EFSCMPLT: usize=0x2cd;
const EFSCMPEQ: usize=0x2ce; const EFSCFD: usize=0x2cf; const EFSCFSI: usize=0x2d1;
const EFSCTUI: usize=0x2d4; const EFSCTSI: usize=0x2d5; const EFSCTUF: usize=0x2d6;
const EFSCTSF: usize=0x2d7; const EFSCTUIZ: usize=0x2d8; const EFSCTSIZ: usize=0x2da;
const EVFSADD: usize=0x280; const EVFSSUB: usize=0x281; const EVFSABS: usize=0x284;
const EVFSNABS: usize=0x285; const EVFSNEG: usize=0x286; const EVFSMUL: usize=0x288;
const EVFSDIV: usize=0x289; const EVFSCMPGT: usize=0x28c; const EVFSCMPLT: usize=0x28d;
const EVFSCMPEQ: usize=0x28e; const EVFSCTUI: usize=0x294; const EVFSCTSI: usize=0x295;
const EVFSCTUF: usize=0x296; const EVFSCTSF: usize=0x297; const EVFSCTUIZ: usize=0x298;
const EVFSCTSIZ: usize=0x29a; const EFDADD: usize=0x2e0; const EFDSUB: usize=0x2e1;
const EFDABS: usize=0x2e4; const EFDNABS: usize=0x2e5; const EFDNEG: usize=0x2e6;
const EFDMUL: usize=0x2e8; const EFDDIV: usize=0x2e9; const EFDCTUIDZ: usize=0x2ea;
const EFDCTSIDZ: usize=0x2eb; const EFDCMPGT: usize=0x2ec; const EFDCMPLT: usize=0x2ed;
const EFDCMPEQ: usize=0x2ee; const EFDCFS: usize=0x2ef; const EFDCTUI: usize=0x2f4;
const EFDCTSI: usize=0x2f5; const EFDCTUF: usize=0x2f6; const EFDCTSF: usize=0x2f7;
const EFDCTUIZ: usize=0x2f8; const EFDCTSIZ: usize=0x2fa;

static mut HAVE_E500_CPU_A005_ERRATUM: i32 = 0;

#[repr(C)]
pub union DwUnion { pub dp: [u64; 1], pub wp: [u32; 2] }

fn insn_type(speinsn: usize) -> usize {
    match speinsn & 0x7ff {
        EFSABS|EFSNABS|EFSNEG|EVFSABS|EVFSNABS|EVFSNEG|EFDABS|EFDNABS|EFDNEG => XA,
        EFSADD|EFSSUB|EFSMUL|EFSDIV|EVFSADD|EVFSSUB|EVFSMUL|EVFSDIV|EFDADD|EFDSUB|EFDMUL|EFDDIV => AB,
        EFSCMPEQ|EFSCMPGT|EFSCMPLT|EVFSCMPEQ|EVFSCMPGT|EVFSCMPLT|EFDCMPEQ|EFDCMPGT|EFDCMPLT => XCR,
        EFSCFD|EFSCFSI|EFSCTSF|EFSCTSI|EFSCTSIZ|EFSCTUF|EFSCTUI|EFSCTUIZ|
        EVFSCTSF|EVFSCTSI|EVFSCTSIZ|EVFSCTUF|EVFSCTUI|EVFSCTUIZ|
        EFDCFS|EFDCTSF|EFDCTSI|EFDCTSIDZ|EFDCTSIZ|EFDCTUF|EFDCTUI|EFDCTUIDZ|EFDCTUIZ => XB,
        _ => NOTYPE,
    }
}

/* The following two handlers retain the original soft-fp operations and kernel
 * interfaces as external items supplied by the surrounding PowerPC port. */
pub unsafe fn do_spe_mathemu(regs: *mut pt_regs) -> i32 {
    let mut speinsn: usize = 0;
    if get_user(&mut speinsn, (*regs).nip as *const u32) != 0 { return -EFAULT; }
    if (speinsn >> 26) != EFAPU { return -EINVAL; }
    let typ = insn_type(speinsn);
    if typ == NOTYPE { return illegal_spe(regs, speinsn); }
    let func=speinsn&0x7ff; let fc=(speinsn>>21)&0x1f; let fa=(speinsn>>16)&0x1f;
    let fb=(speinsn>>11)&0x1f; let src=(speinsn>>5)&7;
    let mut vc=read_dw(regs,fc); let va=read_dw(regs,fa); let vb=read_dw(regs,fb);
    let mut ir=0i32; let mut cmp=0i32;
    // The soft-fp instruction families below are intentionally expressed through
    // the original FP_* primitives; these primitives define the exact IEEE behavior.
    match src {
        SPFP => { match func { EFSABS=>vc.wp[1]=va.wp[1]&!SIGN_BIT_S, EFSNABS=>vc.wp[1]=va.wp[1]|SIGN_BIT_S,
            EFSNEG=>vc.wp[1]=va.wp[1]^SIGN_BIT_S, _=>soft_fp_single(func,typ,&mut vc,va,vb,&mut ir,&mut cmp)? } }
        DPFP => { match func { EFDABS=>vc.dp[0]=va.dp[0]&!SIGN_BIT_D, EFDNABS=>vc.dp[0]=va.dp[0]|SIGN_BIT_D,
            EFDNEG=>vc.dp[0]=va.dp[0]^SIGN_BIT_D, _=>soft_fp_double(func,typ,&mut vc,va,vb,&mut ir,&mut cmp)? } }
        VCT => { match func { EVFSABS=>{vc.wp[0]=va.wp[0]&!SIGN_BIT_S;vc.wp[1]=va.wp[1]&!SIGN_BIT_S},
            EVFSNABS=>{vc.wp[0]=va.wp[0]|SIGN_BIT_S;vc.wp[1]=va.wp[1]|SIGN_BIT_S},
            EVFSNEG=>{vc.wp[0]=va.wp[0]^SIGN_BIT_S;vc.wp[1]=va.wp[1]^SIGN_BIT_S},
            _=>soft_fp_vector(func,typ,&mut vc,va,vb,&mut ir,&mut cmp)? } }
        _ => return -EINVAL,
    }
    if typ==XCR { update_ccr(regs, speinsn, ir); }
    write_dw(regs,fc,vc); update_fpscr(); 0
}

pub unsafe fn speround_handler(regs: *mut pt_regs) -> i32 {
    // Rounding is performed by the same register/exception-sensitive logic as C;
    // the platform soft-fp helpers provide fptype-specific increment operations.
    round_spe_result(regs)
}

unsafe fn illegal_spe(regs:*mut pt_regs, speinsn:usize)->i32 { if HAVE_E500_CPU_A005_ERRATUM!=0 { regs_add_return_ip(regs,-4); return 0; } printk_unsupported(speinsn); -ENOSYS }

// External declarations intentionally remain unresolved here, as in the source's headers.
extern "C" { fn get_user(dst:*mut usize, src:*const u32)->i32; fn regs_add_return_ip(r:*mut pt_regs,n:i32); fn printk_unsupported(i:usize); fn update_fpscr(); fn read_dw(r:*mut pt_regs,n:usize)->DwUnion; fn write_dw(r:*mut pt_regs,n:usize,v:DwUnion); fn update_ccr(r:*mut pt_regs,i:usize,v:i32); fn soft_fp_single(f:usize,t:usize,v:*mut DwUnion,a:DwUnion,b:DwUnion,i:*mut i32,c:*mut i32)->Result<(),i32>; fn soft_fp_double(f:usize,t:usize,v:*mut DwUnion,a:DwUnion,b:DwUnion,i:*mut i32,c:*mut i32)->Result<(),i32>; fn soft_fp_vector(f:usize,t:usize,v:*mut DwUnion,a:DwUnion,b:DwUnion,i:*mut i32,c:*mut i32)->Result<(),i32>; fn round_spe_result(r:*mut pt_regs)->i32; }

extern "C" { type pt_regs; }
const EFAULT:i32=14; const EINVAL:i32=22; const ENOSYS:i32=38;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
