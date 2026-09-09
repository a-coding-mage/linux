// SPDX-License-Identifier: GPL-2.0-only
/* Just-In-Time compiler for eBPF bytecode on MIPS. */

// Kernel headers and bpf_jit_comp.h supply the constants, types, macros, and
// external functions referenced below.

#[inline]
fn converted(desc: u32) -> u32 { desc & JIT_DESC_CONVERT }
#[inline]
fn index(desc: u32) -> u32 { desc & !JIT_DESC_CONVERT }

pub unsafe fn push_regs(ctx: *mut jit_context, mask: u32, excl: u32, mut depth: i32) -> i32 {
    for reg in 0..(8 * core::mem::size_of::<u32>()) {
        if mask & (1u32 << reg) != 0 {
            if excl & (1u32 << reg) == 0 {
                if core::mem::size_of::<c_long>() == 4 { emit(ctx, sw, reg, depth, MIPS_R_SP); }
                else { emit(ctx, sd, reg, depth, MIPS_R_SP); }
            }
            depth += core::mem::size_of::<c_long>() as i32;
        }
    }
    (*ctx).stack_used = core::cmp::max((*ctx).stack_used, depth);
    depth
}

pub unsafe fn pop_regs(ctx: *mut jit_context, mask: u32, excl: u32, mut depth: i32) -> i32 {
    for reg in 0..(8 * core::mem::size_of::<u32>()) {
        if mask & (1u32 << reg) != 0 {
            if excl & (1u32 << reg) == 0 {
                if core::mem::size_of::<c_long>() == 4 { emit(ctx, lw, reg, depth, MIPS_R_SP); }
                else { emit(ctx, ld, reg, depth, MIPS_R_SP); }
            }
            depth += core::mem::size_of::<c_long>() as i32;
        }
    }
    depth
}

pub unsafe fn get_target(ctx: *mut jit_context, loc: u32) -> i32 {
    let idx = index((*ctx).descriptors[loc as usize]);
    if (*ctx).target.is_null() { return 0; }
    let pc = (*ctx).target.add((*ctx).jit_index) as usize;
    let addr = (*ctx).target.add(idx as usize) as usize;
    if (addr ^ pc) & !(MIPS_JMP_MASK as usize) != 0 { return -1; }
    (addr & MIPS_JMP_MASK as usize) as i32
}

pub unsafe fn get_offset(ctx: *const jit_context, off: i32) -> i32 {
    ((index((*ctx).descriptors[((*ctx).bpf_index as i32 + off) as usize]) as i32
        - (*ctx).jit_index as i32 - 1) * core::mem::size_of::<u32>() as i32)
}

pub unsafe fn emit_mov_i(ctx: *mut jit_context, dst: u8, imm: i32) {
    if (-0x8000..=0x7fff).contains(&imm) { emit(ctx, addiu, dst, MIPS_R_ZERO, imm); }
    else { emit(ctx, lui, dst, ((imm as u32 >> 16) as i16)); emit(ctx, ori, dst, dst, (imm as u16 & 0xffff)); }
    clobber_reg(ctx, dst);
}
pub unsafe fn emit_mov_r(ctx: *mut jit_context, dst: u8, src: u8) { emit(ctx, ori, dst, src, 0); clobber_reg(ctx, dst); }

pub fn valid_alu_i(op: u8, imm: i32) -> bool {
    match BPF_OP(op) {
        BPF_NEG | BPF_LSH | BPF_RSH | BPF_ARSH => true,
        BPF_ADD => !IS_ENABLED(CONFIG_CPU_DADDI_WORKAROUNDS) && (-0x8000..=0x7fff).contains(&imm),
        BPF_SUB => !IS_ENABLED(CONFIG_CPU_DADDI_WORKAROUNDS) && (-0x7fff..=0x8000).contains(&imm),
        BPF_AND | BPF_OR | BPF_XOR => (0..=0xffff).contains(&imm),
        BPF_MUL => imm == 0 || (imm > 0 && is_power_of_2(imm)),
        BPF_DIV | BPF_MOD => (imm as u32) <= 0x10000 && is_power_of_2(imm as u32),
        _ => false,
    }
}

pub unsafe fn rewrite_alu_i(op: u8, mut imm: i32, alu: *mut u8, val: *mut i32) -> bool {
    let mut act = true;
    match BPF_OP(op) {
        BPF_LSH|BPF_RSH|BPF_ARSH|BPF_ADD|BPF_SUB|BPF_OR|BPF_XOR => act = imm != 0,
        BPF_MUL => if imm == 1 { act = false } else if imm == 0 { op = BPF_AND } else { op = BPF_LSH; imm = ilog2(imm.abs()) },
        BPF_DIV => if imm == 1 { act = false } else { op = BPF_RSH; imm = ilog2(imm) },
        BPF_MOD => { op = BPF_AND; imm -= 1 },
        _ => {},
    }
    *alu = op; *val = imm; act
}

pub unsafe fn emit_alu_i(ctx: *mut jit_context, dst: u8, imm: i32, op: u8) {
    match BPF_OP(op) {
        BPF_NEG => emit(ctx, subu, dst, MIPS_R_ZERO, dst), BPF_AND => emit(ctx, andi, dst, dst, imm as u16),
        BPF_OR => emit(ctx, ori, dst, dst, imm as u16), BPF_XOR => emit(ctx, xori, dst, dst, imm as u16),
        BPF_LSH => emit(ctx, sll, dst, dst, imm), BPF_RSH => emit(ctx, srl, dst, dst, imm),
        BPF_ARSH => emit(ctx, sra, dst, dst, imm), BPF_ADD => emit(ctx, addiu, dst, dst, imm),
        BPF_SUB => emit(ctx, addiu, dst, dst, -imm), _ => {},
    } clobber_reg(ctx, dst);
}

pub unsafe fn emit_alu_r(ctx: *mut jit_context, dst: u8, src: u8, op: u8) {
    match BPF_OP(op) {
        BPF_AND=>emit(ctx,and,dst,dst,src), BPF_OR=>emit(ctx,or,dst,dst,src), BPF_XOR=>emit(ctx,xor,dst,dst,src),
        BPF_LSH=>emit(ctx,sllv,dst,dst,src), BPF_RSH=>emit(ctx,srlv,dst,dst,src), BPF_ARSH=>emit(ctx,srav,dst,dst,src),
        BPF_ADD=>emit(ctx,addu,dst,dst,src), BPF_SUB=>emit(ctx,subu,dst,dst,src),
        BPF_MUL=>if cpu_has_mips32r1||cpu_has_mips32r6 {emit(ctx,mul,dst,dst,src)} else {emit(ctx,multu,dst,src);emit(ctx,mflo,dst)},
        BPF_DIV=>if cpu_has_mips32r6 {emit(ctx,divu_r6,dst,dst,src)} else {emit(ctx,divu,dst,src);emit(ctx,mflo,dst)},
        BPF_MOD=>if cpu_has_mips32r6 {emit(ctx,modu,dst,dst,src)} else {emit(ctx,divu,dst,src);emit(ctx,mfhi,dst)}, _=>{},
    } clobber_reg(ctx,dst);
}

pub unsafe fn emit_atomic_r(ctx:*mut jit_context,dst:u8,src:u8,off:i16,code:u8){
    LLSC_sync(ctx); emit(ctx,ll,MIPS_R_T9,off,dst);
    match code { BPF_ADD|BPF_ADD_FETCH=>emit(ctx,addu,MIPS_R_T8,MIPS_R_T9,src), BPF_AND|BPF_AND_FETCH=>emit(ctx,and,MIPS_R_T8,MIPS_R_T9,src), BPF_OR|BPF_OR_FETCH=>emit(ctx,or,MIPS_R_T8,MIPS_R_T9,src), BPF_XOR|BPF_XOR_FETCH=>emit(ctx,xor,MIPS_R_T8,MIPS_R_T9,src), BPF_XCHG=>emit(ctx,move,MIPS_R_T8,src), _=>{} }
    emit(ctx,sc,MIPS_R_T8,off,dst); emit(ctx,LLSC_beqz,MIPS_R_T8,-16-LLSC_offset); emit(ctx,nop);
    if code & BPF_FETCH != 0 { emit(ctx,move,src,MIPS_R_T9); clobber_reg(ctx,src); }
}

pub unsafe fn emit_cmpxchg_r(ctx:*mut jit_context,dst:u8,src:u8,res:u8,off:i16){ LLSC_sync(ctx); emit(ctx,ll,MIPS_R_T9,off,dst); emit(ctx,bne,MIPS_R_T9,res,12); emit(ctx,move,MIPS_R_T8,src); emit(ctx,sc,MIPS_R_T8,off,dst); emit(ctx,LLSC_beqz,MIPS_R_T8,-20-LLSC_offset); emit(ctx,move,res,MIPS_R_T9); clobber_reg(ctx,res); }

pub unsafe fn emit_bswap_r(ctx:*mut jit_context,dst:u8,width:u32){ let tmp=MIPS_R_T8; let msk=MIPS_R_T9; match width { 32=>if cpu_has_mips32r2||cpu_has_mips32r6 {emit(ctx,wsbh,dst,dst);emit(ctx,rotr,dst,dst,16)} else {emit(ctx,sll,tmp,dst,16);emit(ctx,srl,dst,dst,16);emit(ctx,or,dst,dst,tmp);emit(ctx,lui,msk,0xff);emit(ctx,ori,msk,msk,0xff);emit(ctx,and,tmp,dst,msk);emit(ctx,sll,tmp,tmp,8);emit(ctx,srl,dst,dst,8);emit(ctx,and,dst,dst,msk);emit(ctx,or,dst,dst,tmp)}, 16=>if cpu_has_mips32r2||cpu_has_mips32r6 {emit(ctx,wsbh,dst,dst);emit(ctx,andi,dst,dst,0xffff)} else {emit(ctx,andi,tmp,dst,0xff00);emit(ctx,srl,tmp,tmp,8);emit(ctx,andi,dst,dst,0xff);emit(ctx,sll,dst,dst,8);emit(ctx,or,dst,dst,tmp)}, _=>{} } clobber_reg(ctx,dst); }

pub fn valid_jmp_i(op:u8,imm:i32)->bool{match op{JIT_JNOP=>true,BPF_JSET|JIT_JNSET=>(0..=0xffff).contains(&imm),BPF_JGE|BPF_JLT|BPF_JSGE|BPF_JSLT=>(-0x8000..=0x7fff).contains(&imm),BPF_JGT|BPF_JLE|BPF_JSGT|BPF_JSLE=>(-0x8001..=0x7ffe).contains(&imm),_=>false}}
fn invert_jmp(op:u8)->u8{match op{BPF_JA=>JIT_JNOP,BPF_JEQ=>BPF_JNE,BPF_JNE=>BPF_JEQ,BPF_JSET=>JIT_JNSET,BPF_JGT=>BPF_JLE,BPF_JGE=>BPF_JLT,BPF_JLT=>BPF_JGE,BPF_JLE=>BPF_JGT,BPF_JSGT=>BPF_JSLE,BPF_JSGE=>BPF_JSLT,BPF_JSLT=>BPF_JSGE,BPF_JSLE=>BPF_JSGT,_=>0}}

pub unsafe fn setup_jmp(ctx:*mut jit_context, bpf_op:u8,bpf_off:i16,jit_op:*mut u8,jit_off:*mut i32){let d=&mut (*ctx).descriptors[(*ctx).bpf_index as usize];let mut op=bpf_op;let mut off=0;if index(*d)!=0&&bpf_op!=JIT_JNOP{if bpf_op==BPF_JA{*d|=JIT_DESC_CONVERT} if converted(*d)==0{let target=(*ctx).bpf_index as i32+bpf_off as i32+1;let origin=(*ctx).bpf_index as i32+1;off=(index((*ctx).descriptors[target as usize]) as i32-index((*ctx).descriptors[origin as usize]) as i32+1)*4}if converted(*d)!=0||off < -0x20000||off>0x1ffff{off=12;op=invert_jmp(bpf_op);(*ctx).changes+= (converted(*d)==0) as u32;*d|=JIT_DESC_CONVERT}}*jit_off=off;*jit_op=op}

pub unsafe fn setup_jmp_i(ctx:*mut jit_context,imm:i32,width:u8,mut op:u8,off:i16,jit_op:*mut u8,jit_off:*mut i32){let mut always=false;let mut never=false;match op{BPF_JSET|BPF_JLT=>never=imm==0,BPF_JGE=>always=imm==0,BPF_JGT=>never=imm as u32==U32_MAX,BPF_JLE=>always=imm as u32==U32_MAX,BPF_JSGT=>never=imm==S32_MAX&&width==32,BPF_JSGE=>always=imm==S32_MIN&&width==32,BPF_JSLT=>never=imm==S32_MIN&&width==32,BPF_JSLE=>always=imm==S32_MAX&&width==32,_=>{}}if never{op=JIT_JNOP}if always{op=BPF_JA}setup_jmp(ctx,op,off,jit_op,jit_off)}
pub unsafe fn setup_jmp_r(ctx:*mut jit_context,same:bool,mut op:u8,off:i16,jit_op:*mut u8,jit_off:*mut i32){match op{BPF_JEQ|BPF_JGE|BPF_JLE|BPF_JSGE|BPF_JSLE=>if same{op=BPF_JA},BPF_JNE|BPF_JLT|BPF_JGT|BPF_JSGT|BPF_JSLT=>if same{op=JIT_JNOP},_=>{}}setup_jmp(ctx,op,off,jit_op,jit_off)}

pub unsafe fn finish_jmp(ctx:*mut jit_context,op:u8,bpf_off:i16)->i32{if op!=JIT_JNOP{emit(ctx,nop)}if converted((*ctx).descriptors[(*ctx).bpf_index as usize])!=0{let target=get_target(ctx,((*ctx).bpf_index as i32+bpf_off as i32+1) as u32);if target<0{return -1}emit(ctx,j,target);emit(ctx,nop)}0}

pub unsafe fn emit_jmp_i(ctx:*mut jit_context,dst:u8,imm:i32,off:i32,op:u8){match op{JIT_JNOP=>{},BPF_JSET=>{emit(ctx,andi,MIPS_R_T9,dst,imm as u16);emit(ctx,bnez,MIPS_R_T9,off)},JIT_JNSET=>{emit(ctx,andi,MIPS_R_T9,dst,imm as u16);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JGT=>{emit(ctx,sltiu,MIPS_R_T9,dst,imm+1);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JGE=>{emit(ctx,sltiu,MIPS_R_T9,dst,imm);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JLT=>{emit(ctx,sltiu,MIPS_R_T9,dst,imm);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JLE=>{emit(ctx,sltiu,MIPS_R_T9,dst,imm+1);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JSGT=>{emit(ctx,slti,MIPS_R_T9,dst,imm+1);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JSGE=>{emit(ctx,slti,MIPS_R_T9,dst,imm);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JSLT=>{emit(ctx,slti,MIPS_R_T9,dst,imm);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JSLE=>{emit(ctx,slti,MIPS_R_T9,dst,imm+1);emit(ctx,bnez,MIPS_R_T9,off)},_=>{}}}
pub unsafe fn emit_jmp_r(ctx:*mut jit_context,dst:u8,src:u8,off:i32,op:u8){match op{JIT_JNOP=>{},BPF_JEQ=>emit(ctx,beq,dst,src,off),BPF_JNE=>emit(ctx,bne,dst,src,off),BPF_JSET=>{emit(ctx,and,MIPS_R_T9,dst,src);emit(ctx,bnez,MIPS_R_T9,off)},JIT_JNSET=>{emit(ctx,and,MIPS_R_T9,dst,src);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JGT=>{emit(ctx,sltu,MIPS_R_T9,src,dst);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JGE=>{emit(ctx,sltu,MIPS_R_T9,dst,src);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JLT=>{emit(ctx,sltu,MIPS_R_T9,dst,src);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JLE=>{emit(ctx,sltu,MIPS_R_T9,src,dst);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JSGT=>{emit(ctx,slt,MIPS_R_T9,src,dst);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JSGE=>{emit(ctx,slt,MIPS_R_T9,dst,src);emit(ctx,beqz,MIPS_R_T9,off)},BPF_JSLT=>{emit(ctx,slt,MIPS_R_T9,dst,src);emit(ctx,bnez,MIPS_R_T9,off)},BPF_JSLE=>{emit(ctx,slt,MIPS_R_T9,src,dst);emit(ctx,bnez,MIPS_R_T9,off)},_=>{}}}
pub unsafe fn emit_ja(ctx:*mut jit_context,off:i16)->i32{let t=get_target(ctx,(*ctx).bpf_index+off as u32+1);if t<0{-1}else{emit(ctx,j,t);emit(ctx,nop);0}}
pub unsafe fn emit_exit(ctx:*mut jit_context)->i32{let t=get_target(ctx,(*ctx).program.len);if t<0{-1}else{emit(ctx,j,t);emit(ctx,nop);0}}

unsafe fn build_body(ctx:*mut jit_context)->i32{let prog=(*ctx).program;(*ctx).stack_used=0;let mut i=0;while i<(*prog).len{let insn=&(*prog).insnsi[i as usize];let desc=&mut (*ctx).descriptors[i as usize];access_reg(ctx,insn.src_reg);access_reg(ctx,insn.dst_reg);(*ctx).bpf_index=i;if (*ctx).target.is_null(){(*ctx).changes+=(index(*desc)!=(*ctx).jit_index) as u32;*desc&=JIT_DESC_CONVERT;*desc|=(*ctx).jit_index;}let ret=build_insn(insn,ctx);if ret<0{return ret}if ret>0{i+=1;if (*ctx).target.is_null(){(*ctx).descriptors[(i+1) as usize]=(*ctx).jit_index;}}i+=1;}(*ctx).descriptors[(*prog).len as usize]=(*ctx).jit_index;0}
unsafe fn set_convert_flag(ctx:*mut jit_context,enable:bool){let flag=if enable{JIT_DESC_CONVERT}else{0};for i in 0..=(*ctx).program.len{(*ctx).descriptors[i as usize]=index((*ctx).descriptors[i as usize])|flag;}}
unsafe fn jit_fill_hole(area:*mut core::ffi::c_void,mut size:u32){let mut p=area as *mut u32;while size>=4{uasm_i_break(&mut p,BRK_BUG);size-=4;}}

pub fn bpf_jit_needs_zext()->bool{true}

pub unsafe fn bpf_int_jit_compile(env:*mut bpf_verifier_env,prog:*mut bpf_prog)->*mut bpf_prog{
    let mut header:*mut bpf_binary_header=core::ptr::null_mut();let mut ctx:jit_context=core::mem::zeroed();ctx.program=prog;
    if !(*prog).jit_requested{return prog;}ctx.descriptors=kcalloc((*prog).len+1,core::mem::size_of::<u32>(),GFP_KERNEL);if ctx.descriptors.is_null(){return prog;}
    if build_body(&mut ctx)<0{return prog;}ctx.jit_index=0;build_prologue(&mut ctx);let tmp_idx=ctx.jit_index;let mut tries=JIT_MAX_ITERATIONS;
    loop{ctx.jit_index=tmp_idx;ctx.changes=0;if tries==2{set_convert_flag(&mut ctx,true)}if build_body(&mut ctx)<0{return prog;}if !(ctx.changes>0&&{tries-=1;tries>0}){break;}}
    if WARN_ONCE(ctx.changes>0,"JIT offsets failed to converge"){return prog;}build_epilogue(&mut ctx,MIPS_R_RA);let image_size=core::mem::size_of::<u32>()*ctx.jit_index;let mut image_ptr:*mut u8=core::ptr::null_mut();header=bpf_jit_binary_alloc(image_size,&mut image_ptr,core::mem::size_of::<u32>(),jit_fill_hole);if header.is_null(){return prog;}
    ctx.target=image_ptr as *mut u32;ctx.jit_index=0;build_prologue(&mut ctx);if build_body(&mut ctx)<0{return prog;}build_epilogue(&mut ctx,MIPS_R_RA);set_convert_flag(&mut ctx,false);bpf_prog_fill_jited_linfo(prog,ctx.descriptors.add(1));if bpf_jit_binary_lock_ro(header)!=0{return prog;}flush_icache_range(header as usize,ctx.target.add(ctx.jit_index) as usize);if bpf_jit_enable>1{bpf_jit_dump((*prog).len,image_size,2,ctx.target)}(*prog).bpf_func=ctx.target as *mut core::ffi::c_void;(*prog).jited=1;(*prog).jited_len=image_size;kfree(ctx.descriptors);prog
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
