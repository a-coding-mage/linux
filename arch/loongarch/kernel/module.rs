// SPDX-License-Identifier: GPL-2.0+
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Kernel dependencies and build-time configuration are supplied externally.

type RelocRelaHandler = unsafe fn(*mut module, *mut u32, Elf_Addr, *mut c_long, *mut usize, c_uint) -> c_int;

unsafe fn rela_stack_push(stack_value: c_long, stack: *mut c_long, top: *mut usize) -> c_int {
    if *top >= RELA_STACK_DEPTH { return -ENOEXEC; }
    *stack.add(*top) = stack_value;
    *top += 1;
    pr_debug!("%s stack_value = 0x%lx\n", "rela_stack_push", stack_value);
    0
}

unsafe fn rela_stack_pop(stack_value: *mut c_long, stack: *mut c_long, top: *mut usize) -> c_int {
    if *top == 0 { return -ENOEXEC; }
    *top -= 1;
    *stack_value = *stack.add(*top);
    pr_debug!("%s stack_value = 0x%lx\n", "rela_stack_pop", *stack_value);
    0
}

unsafe fn apply_r_larch_none(_: *mut module, _: *mut u32, _: Elf_Addr, _: *mut c_long, _: *mut usize, _: c_uint) -> c_int { 0 }
unsafe fn apply_r_larch_error(me: *mut module, _: *mut u32, _: Elf_Addr, _: *mut c_long, _: *mut usize, typ: c_uint) -> c_int {
    pr_err!("%s: Unsupport relocation type %u, please add its support.\n", (*me).name, typ); -EINVAL
}
unsafe fn apply_r_larch_32(_: *mut module, location: *mut u32, v: Elf_Addr, _: *mut c_long, _: *mut usize, _: c_uint) -> c_int { *location = v as u32; 0 }

#[cfg(not(CONFIG_32BIT))]
unsafe fn apply_r_larch_64(_: *mut module, location: *mut u32, v: Elf_Addr, _: *mut c_long, _: *mut usize, _: c_uint) -> c_int { *(location as *mut Elf_Addr) = v; 0 }
#[cfg(CONFIG_32BIT)]
unsafe fn apply_r_larch_64(m: *mut module, l: *mut u32, v: Elf_Addr, s: *mut c_long, t: *mut usize, ty: c_uint) -> c_int { apply_r_larch_error(m,l,v,s,t,ty) }

unsafe fn apply_r_larch_sop_push_pcrel(_: *mut module, location: *mut u32, v: Elf_Addr, stack: *mut c_long, top: *mut usize, _: c_uint) -> c_int { rela_stack_push(v.wrapping_sub(location as Elf_Addr) as c_long, stack, top) }
unsafe fn apply_r_larch_sop_push_absolute(_: *mut module, _: *mut u32, v: Elf_Addr, stack: *mut c_long, top: *mut usize, _: c_uint) -> c_int { rela_stack_push(v as c_long, stack, top) }
unsafe fn apply_r_larch_sop_push_dup(_: *mut module, _: *mut u32, _: Elf_Addr, stack: *mut c_long, top: *mut usize, _: c_uint) -> c_int {
    let mut x = 0; let mut e = rela_stack_pop(&mut x, stack, top); if e != 0 { return e; }
    e = rela_stack_push(x, stack, top); if e != 0 { return e; } rela_stack_push(x, stack, top)
}

unsafe fn apply_r_larch_sop_push_plt_pcrel(m: *mut module, sh: *mut Elf_Shdr, l: *mut u32, mut v: Elf_Addr, s: *mut c_long, t: *mut usize, ty: c_uint) -> c_int {
    let mut off = v as isize - l as isize;
    if off >= SZ_128M || off < -SZ_128M { v = module_emit_plt_entry(m, sh, v); }
    apply_r_larch_sop_push_pcrel(m,l,v,s,t,ty)
}

unsafe fn apply_r_larch_sop(m: *mut module, _: *mut u32, _: Elf_Addr, s: *mut c_long, t: *mut usize, ty: c_uint) -> c_int {
    let mut a=0; let mut b=0; let mut c=0;
    if ty == R_LARCH_SOP_IF_ELSE { let e=rela_stack_pop(&mut c,s,t); if e!=0{return e;} }
    let e=rela_stack_pop(&mut b,s,t); if e!=0{return e;} let e=rela_stack_pop(&mut a,s,t); if e!=0{return e;}
    let x = match ty { R_LARCH_SOP_AND=>a&b, R_LARCH_SOP_ADD=>a.wrapping_add(b), R_LARCH_SOP_SUB=>a.wrapping_sub(b), R_LARCH_SOP_SL=>a<<b, R_LARCH_SOP_SR=>a>>b, R_LARCH_SOP_IF_ELSE=>if a!=0{b}else{c}, _=>{pr_err!("%s: Unsupport relocation type %u\n",(*m).name,ty);return -EINVAL;} };
    rela_stack_push(x,s,t)
}

unsafe fn apply_r_larch_sop_imm_field(m: *mut module, location: *mut u32, _: Elf_Addr, s: *mut c_long, t: *mut usize, ty: c_uint) -> c_int {
    let mut x=0; let e=rela_stack_pop(&mut x,s,t); if e!=0{return e;}
    let i = location as *mut loongarch_instruction;
    match ty {
        R_LARCH_SOP_POP_32_U_10_12|R_LARCH_SOP_POP_32_S_10_12 => { if !((ty==R_LARCH_SOP_POP_32_U_10_12 && unsigned_imm_check(x,12)) || (ty!=R_LARCH_SOP_POP_32_U_10_12 && signed_imm_check(x,12))) { return -ENOEXEC; } (*i).reg2i12_format.immediate=x&0xfff; },
        R_LARCH_SOP_POP_32_S_10_16 => { if !signed_imm_check(x,16){return -ENOEXEC;} (*i).reg2i16_format.immediate=x&0xffff; },
        R_LARCH_SOP_POP_32_S_10_16_S2 => { if x%4!=0 || !signed_imm_check(x,18){return -ENOEXEC;} (*i).reg2i16_format.immediate=(x>>2)&0xffff; },
        R_LARCH_SOP_POP_32_S_5_20 => { if !signed_imm_check(x,20){return -ENOEXEC;} (*i).reg1i20_format.immediate=x&0xfffff; },
        R_LARCH_SOP_POP_32_S_0_5_10_16_S2 => { if x%4!=0 || !signed_imm_check(x,23){return -ENOEXEC;} x>>=2; (*i).reg1i21_format.immediate_l=x&0xffff; (*i).reg1i21_format.immediate_h=(x>>16)&0x1f; },
        R_LARCH_SOP_POP_32_S_0_10_10_16_S2 => { if x%4!=0 || !signed_imm_check(x,28){return -ENOEXEC;} x>>=2; (*i).reg0i26_format.immediate_l=x&0xffff; (*i).reg0i26_format.immediate_h=(x>>16)&0x3ff; },
        R_LARCH_SOP_POP_32_U => { if !unsigned_imm_check(x,32){return -ENOEXEC;} *location=x as u32; },
        _=>{pr_err!("%s: Unsupport relocation type %u\n",(*m).name,ty);return -EINVAL;}
    } 0
}

unsafe fn apply_r_larch_add_sub(m:*mut module,l:*mut u32,v:Elf_Addr,_:*mut c_long,_:*mut usize,ty:c_uint)->c_int {
    match ty { R_LARCH_ADD32=>*(l as *mut i32)=(*(l as *mut i32)).wrapping_add(v as i32), R_LARCH_SUB32=>*(l as *mut i32)=(*(l as *mut i32)).wrapping_sub(v as i32),
        R_LARCH_ADD64=>*(l as *mut i64)=(*(l as *mut i64)).wrapping_add(v as i64), R_LARCH_SUB64=>*(l as *mut i64)=(*(l as *mut i64)).wrapping_sub(v as i64), _=>{pr_err!("%s: Unsupport relocation type %u\n",(*m).name,ty);return -EINVAL;} } 0
}
unsafe fn apply_r_larch_b26(m:*mut module,sh:*mut Elf_Shdr,l:*mut u32,mut v:Elf_Addr,_:*mut c_long,_:*mut usize,ty:c_uint)->c_int { let mut o=v as isize-l as isize; if o>=SZ_128M||o< -SZ_128M{v=module_emit_plt_entry(m,sh,v);o=v as isize-l as isize;} if o&3!=0||!signed_imm_check(o as c_long,28){pr_err!("module %s: jump offset = 0x%llx dangerous R_LARCH_B26 (%u) relocation\n",(*m).name,o,ty);return -ENOEXEC;} let i=l as *mut loongarch_instruction; o>>=2;(*i).reg0i26_format.immediate_l=o&0xffff;(*i).reg0i26_format.immediate_h=(o>>16)&0x3ff;0 }
unsafe fn apply_r_larch_pcadd(m:*mut module,l:*mut u32,mut v:Elf_Addr,_:*mut c_long,_:*mut usize,ty:c_uint)->c_int { let i=l as *mut loongarch_instruction; match ty {R_LARCH_PCADD_LO12=>{(*i).reg2i12_format.immediate=v&0xfff;},R_LARCH_PCADD_HI20=>{let x=((v+0x800) as isize-l as isize)>>12;(*i).reg1i20_format.immediate=(x as Elf_Addr)&0xfffff;},_=>{pr_err!("%s: Unsupport relocation type %u\n",(*m).name,ty);return -EINVAL;}}0 }
unsafe fn apply_r_larch_pcala(m:*mut module,l:*mut u32,mut v:Elf_Addr,_:*mut c_long,_:*mut usize,ty:c_uint)->c_int { let i=l as *mut loongarch_instruction; match ty {R_LARCH_PCALA_LO12=>(*i).reg2i12_format.immediate=v&0xfff,R_LARCH_PCALA_HI20=>(*i).reg1i20_format.immediate=(((v+0x800)&!0xfff) as isize-(l as isize&!0xfff)) as Elf_Addr>>12&0xfffff,_=>{pr_err!("%s: Unsupport relocation type %u\n",(*m).name,ty);return -EINVAL;}}0 }
unsafe fn apply_r_larch_32_pcrel(_: *mut module,l:*mut u32,v:Elf_Addr,_:*mut c_long,_:*mut usize,_:c_uint)->c_int{*l=v.wrapping_sub(l as Elf_Addr) as u32;0}
#[cfg(not(CONFIG_32BIT))] unsafe fn apply_r_larch_64_pcrel(_: *mut module,l:*mut u32,v:Elf_Addr,_:*mut c_long,_:*mut usize,_:c_uint)->c_int{*(l as *mut u64)=v.wrapping_sub(l as Elf_Addr) as u64;0}

unsafe fn apply_relocate_add(_: *mut Elf_Shdr, _: *const c_char, _: c_uint, _: c_uint, _: *mut module) -> c_int { 0 }
unsafe fn module_finalize(_: *const Elf_Ehdr, _: *const Elf_Shdr, _: *mut module) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
