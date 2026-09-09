// SPDX-License-Identifier: GPL-2.0
/* Translation of proc.c. Kernel-provided types, functions, constants, and
 * configuration symbols are intentionally left as external dependencies. */

static mut root_irq_dir: *mut proc_dir_entry = core::ptr::null_mut();

#[cfg(CONFIG_SMP)]
enum AffinityType { Affinity, AffinityList, Effective, EffectiveList }

#[cfg(CONFIG_SMP)]
unsafe fn show_irq_affinity(typ: i32, m: *mut seq_file) -> i32 {
    let desc = irq_to_desc((*m).private as isize);
    let mask: *const cpumask;
    // C guard(raw_spinlock_irq)(&desc->lock);
    match typ {
        0 | 1 => { mask = (*desc).irq_common_data.affinity; if irq_move_pending(&(*desc).irq_data) { mask = irq_desc_get_pending_mask(desc); } }
        2 | 3 => {
            #[cfg(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK)]
            { mask = irq_data_get_effective_affinity_mask(&(*desc).irq_data); }
            #[cfg(not(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK))]
            { return -22; }
        }
        _ => return -22,
    }
    match typ {
        1 | 3 => seq_printf(m, "%*pbl\n", cpumask_pr_args(mask)),
        0 | 2 => seq_printf(m, "%*pb\n", cpumask_pr_args(mask)),
        _ => {}
    }
    0
}

#[cfg(CONFIG_SMP)]
unsafe fn irq_affinity_hint_proc_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let desc = irq_to_desc((*m).private as isize);
    let mut mask: cpumask_var_t = core::mem::zeroed();
    if !zalloc_cpumask_var(&mut mask, GFP_KERNEL) { return -12; }
    // C scoped_guard(raw_spinlock_irq, &desc->lock)
    if !(*desc).affinity_hint.is_null() { cpumask_copy(mask, (*desc).affinity_hint); }
    seq_printf(m, "%*pb\n", cpumask_pr_args(mask));
    free_cpumask_var(mask);
    0
}

#[cfg(CONFIG_SMP)]
static mut no_irq_affinity: i32 = 0;

#[cfg(CONFIG_SMP)]
unsafe fn irq_affinity_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 { show_irq_affinity(0, m) }
#[cfg(CONFIG_SMP)]
unsafe fn irq_affinity_list_proc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 { show_irq_affinity(1, m) }

#[cfg(all(CONFIG_SMP, not(CONFIG_AUTO_IRQ_AFFINITY)))]
unsafe fn irq_select_affinity_usr(_irq: u32) -> i32 { -22 }
#[cfg(all(CONFIG_SMP, CONFIG_AUTO_IRQ_AFFINITY))]
unsafe fn irq_select_affinity_usr(irq: u32) -> i32 { irq_select_affinity(irq) }

#[cfg(CONFIG_SMP)]
unsafe fn write_irq_affinity(typ: i32, file: *mut file, buffer: *const i8, count: usize, pos: *mut loff_t) -> isize {
    let irq = pde_data(file_inode(file)) as isize as u32;
    let mut new_value: cpumask_var_t = core::mem::zeroed();
    if !irq_can_set_affinity_usr(irq) || no_irq_affinity != 0 { return -1; }
    if !zalloc_cpumask_var(&mut new_value, GFP_KERNEL) { return -12; }
    let mut err = if typ != 0 { cpumask_parselist_user(buffer, count, new_value) } else { cpumask_parse_user(buffer, count, new_value) };
    if err == 0 {
        if !cpumask_intersects(new_value, cpu_online_mask) { err = if irq_select_affinity_usr(irq) != 0 { -22 } else { count as isize }; }
        else { err = irq_set_affinity(irq, new_value); if err == 0 { err = count as isize; } }
    }
    free_cpumask_var(new_value); err
}
#[cfg(CONFIG_SMP)]
unsafe fn irq_affinity_proc_write(f:*mut file,b:*const i8,c:usize,p:*mut loff_t)->isize { write_irq_affinity(0,f,b,c,p) }
#[cfg(CONFIG_SMP)]
unsafe fn irq_affinity_list_proc_write(f:*mut file,b:*const i8,c:usize,p:*mut loff_t)->isize { write_irq_affinity(1,f,b,c,p) }

unsafe fn irq_spurious_proc_show(m:*mut seq_file,_v:*mut core::ffi::c_void)->i32 { let d=irq_to_desc((*m).private as isize); seq_printf(m,"count %u\nunhandled %u\nlast_unhandled %u ms\n",(*d).irq_count,(*d).irqs_unhandled,jiffies_to_msecs((*d).last_unhandled)); 0 }

unsafe fn name_unique(irq:u32,new_action:*mut irqaction)->bool { let d=irq_to_desc(irq as isize); let mut action:*mut irqaction=core::ptr::null_mut(); for_each_action_of_desc!(d, action); while !action.is_null() { if action!=new_action && !(*action).name.is_null() && strcmp((*new_action).name,(*action).name)==0{return false;} action=(*action).next; } true }

pub unsafe fn register_handler_proc(irq:u32,action:*mut irqaction) { let d=irq_to_desc(irq as isize); if (*d).dir.is_null()||!(*action).dir.is_null()||(*action).name.is_null()||!name_unique(irq,action){return;} (*action).dir=proc_mkdir((*action).name,(*d).dir); }

pub unsafe fn register_irq_proc(irq:u32,desc:*mut irq_desc) { if root_irq_dir.is_null()||(*desc).irq_data.chip==&no_irq_chip as *const _ as *mut _ {return;} // guard(mutex)(&register_lock)
    if !(*desc).dir.is_null(){return;} let mut name=[0i8;11]; snprintf(name.as_mut_ptr(),11 as _,b"%u\0".as_ptr() as _,irq); (*desc).dir=proc_mkdir(name.as_ptr(),root_irq_dir); if (*desc).dir.is_null(){return;}
    #[cfg(CONFIG_SMP)] { let mut mode:umode_t=S_IRUGO; if irq_can_set_affinity_usr((*desc).irq_data.irq){mode|=S_IWUSR;} proc_create_data(b"smp_affinity\0".as_ptr() as _,mode,(*desc).dir,&irq_affinity_proc_ops,irq as usize as *mut _); proc_create_single_data(b"affinity_hint\0".as_ptr() as _,0o444,(*desc).dir,irq_affinity_hint_proc_show,irq as usize as *mut _); proc_create_data(b"smp_affinity_list\0".as_ptr() as _,mode,(*desc).dir,&irq_affinity_list_proc_ops,irq as usize as *mut _); }
    proc_create_single_data(b"spurious\0".as_ptr() as _,0o444,(*desc).dir,irq_spurious_proc_show,irq as usize as *mut _);
}

pub unsafe fn unregister_irq_proc(irq:u32,desc:*mut irq_desc){if root_irq_dir.is_null()||(*desc).dir.is_null(){return;} remove_proc_entry(b"spurious\0".as_ptr() as _,(*desc).dir);let mut n=[0i8;11];snprintf(n.as_mut_ptr(),11 as _,b"%u\0".as_ptr() as _,irq);remove_proc_entry(n.as_ptr(),root_irq_dir);}
pub unsafe fn unregister_handler_proc(_irq:u32,a:*mut irqaction){proc_remove((*a).dir);}
pub unsafe fn init_irq_proc(){root_irq_dir=proc_mkdir(b"irq\0".as_ptr() as _,core::ptr::null_mut());if root_irq_dir.is_null(){return;}let mut irq=0;let mut d=core::ptr::null_mut();for_each_irq_desc!(irq,d);}
pub unsafe fn irq_proc_update_valid(desc:*mut irq_desc){let mut set=_IRQ_PROC_VALID;if irq_settings_is_hidden(desc)||irq_desc_is_chained(desc)||(*desc).action.is_null(){set=0;}irq_settings_update_proc_valid(desc,set);}

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
const ARCH_PROC_IRQDESC: *mut core::ffi::c_void = 0x1111 as *mut _;
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
pub unsafe fn arch_show_interrupts(_p:*mut seq_file,_prec:i32)->i32 { 0 }
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
static mut irq_proc_constraints: irq_proc_constraints_t = irq_proc_constraints_t { print_header:true, num_prec:4, chip_width:8 };
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
#[repr(C)] struct irq_proc_constraints_t { print_header:bool, num_prec:u32, chip_width:u32 }

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
pub unsafe fn irq_proc_calc_prec(){let mut prec=4u32;let mut n=10000u32;while prec<10&&n<=total_nr_irqs{prec+=1;n*=10;}if prec>irq_proc_constraints.num_prec{irq_proc_constraints.num_prec=prec;}}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
pub unsafe fn irq_proc_update_chip(chip:*const irq_chip){let len=if !chip.is_null()&&!(*chip).name.is_null(){strlen((*chip).name)}else{0};if len==0||len<=irq_proc_constraints.chip_width as usize{return;}if len>irq_proc_constraints.chip_width as usize{irq_proc_constraints.chip_width=len as u32;}}

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe fn irq_proc_emit_zero_counts(p:*mut seq_file,mut zeros:u32){while zeros!=0{let n=core::cmp::min(zeros,256);seq_write(p,b"          0".as_ptr() as _,(n*11) as _);zeros-=n;}}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe fn irq_proc_emit_count(p:*mut seq_file,cnt:u32,zeros:u32)->u32{if cnt==0{return zeros+1;}irq_proc_emit_zero_counts(p,zeros);seq_put_decimal_ull_width(p,b" \0".as_ptr() as _,cnt as u64,10);0}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
pub unsafe fn irq_proc_emit_counts(p:*mut seq_file,cnts:*mut u32){let mut zeros=0;let mut cpu=0;for_each_online_cpu!(cpu){zeros=irq_proc_emit_count(p,per_cpu!(*cnts,cpu),zeros);}irq_proc_emit_zero_counts(p,zeros);}

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe fn irq_seq_show(p:*mut seq_file,_v:*mut core::ffi::c_void)->i32{let c=&mut *( (*p).private as *mut irq_proc_constraints_t);let d=_v as *mut irq_desc;if c.print_header{seq_printf(p,"%*s",c.num_prec+8,"");let mut cpu=0;for_each_online_cpu!(cpu){seq_printf(p,"CPU%-8d",cpu);}seq_putc(p,'\n' as i32);c.print_header=false;}if d as *mut _==ARCH_PROC_IRQDESC{return arch_show_interrupts(p,c.num_prec as i32);}seq_put_decimal_ull_width(p,b"\0".as_ptr() as _,irq_desc_get_irq(d) as u64,c.num_prec);seq_putc(p,':' as i32);if irq_settings_is_per_cpu(d)||irq_settings_is_per_cpu_devid(d)||(*d).tot_count!=0{irq_proc_emit_counts(p,&mut (*d).kstat_irqs.cnt);}else{irq_proc_emit_zero_counts(p,num_online_cpus());}seq_write(p,b"  ".as_ptr() as _,2);if !(*d).irq_data.chip.is_null()&&!(*(*d).irq_data.chip).name.is_null(){seq_printf(p,"%-*s",c.chip_width,(*(*d).irq_data.chip).name);}else{seq_printf(p,"%-*s",c.chip_width,"None");}seq_putc(p,'\n' as i32);0}

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
static irq_seq_ops: seq_operations = seq_operations { start:irq_seq_start, next:irq_seq_next, stop:irq_seq_stop, show:irq_seq_show };
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe extern "C" fn irq_seq_start(f:*mut seq_file,pos:*mut loff_t)->*mut core::ffi::c_void{if *pos==0{let c=&mut *((*f).private as *mut irq_proc_constraints_t);c.num_prec=irq_proc_constraints.num_prec;c.chip_width=irq_proc_constraints.chip_width;c.print_header=true;}irq_seq_next_desc(pos)}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe extern "C" fn irq_seq_next(_f:*mut seq_file,v:*mut core::ffi::c_void,pos:*mut loff_t)->*mut core::ffi::c_void{if !v.is_null()&&v!=ARCH_PROC_IRQDESC{irq_desc_put_ref(v);}*pos+=1;irq_seq_next_desc(pos)}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe extern "C" fn irq_seq_stop(_f:*mut seq_file,v:*mut core::ffi::c_void){if !v.is_null()&&v!=ARCH_PROC_IRQDESC{irq_desc_put_ref(v);}}
#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe fn irq_seq_next_desc(pos:*mut loff_t)->*mut core::ffi::c_void{if *pos>total_nr_irqs{return core::ptr::null_mut();}let d=irq_find_desc_at_or_after(*pos as u32);if !d.is_null(){*pos=irq_desc_get_irq(d) as loff_t;if irq_settings_proc_valid(d)&&irq_desc_get_ref(d){return d as *mut _;}*pos+=1;}else{*pos=total_nr_irqs;ARCH_PROC_IRQDESC}}

#[cfg(CONFIG_GENERIC_IRQ_SHOW)]
unsafe fn irq_proc_init()->i32{proc_create_seq_private(b"interrupts\0".as_ptr() as _,0,core::ptr::null_mut(),&irq_seq_ops,core::mem::size_of::<irq_proc_constraints_t>(),core::ptr::null_mut());0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
