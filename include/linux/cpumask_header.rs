/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/cpumask.h. Included Linux dependencies are external. */

extern "C" {
    pub static mut __cpu_possible_mask: cpumask;
    pub static mut __cpu_online_mask: cpumask;
    pub static mut __cpu_enabled_mask: cpumask;
    pub static mut __cpu_present_mask: cpumask;
    pub static mut __cpu_active_mask: cpumask;
    pub static mut __cpu_dying_mask: cpumask;
    pub static mut __num_online_cpus: atomic_t;
    pub static mut __num_possible_cpus: u32;
    pub static mut cpus_booted_once_mask: cpumask_t;
    pub static cpu_all_bits: [c_ulong; BITS_TO_LONGS(NR_CPUS)];
    pub static cpu_bit_bitmap: [[c_ulong; BITS_TO_LONGS(NR_CPUS)]; BITS_PER_LONG + 1];
    pub fn cpumask_local_spread(i: u32, node: i32) -> u32;
    pub fn cpumask_any_and_distribute(a: *const cpumask, b: *const cpumask) -> u32;
    pub fn cpumask_any_distribute(a: *const cpumask) -> u32;
    pub fn init_cpu_present(src: *const cpumask);
    pub fn init_cpu_possible(src: *const cpumask);
    pub fn set_cpu_online(cpu: u32, online: bool);
    pub fn set_cpu_possible(cpu: u32, possible: bool);
}

pub type c_ulong = usize;

#[inline(always)] pub unsafe fn cpu_max_bits_warn(cpu: u32, bits: u32) { #[cfg(CONFIG_DEBUG_PER_CPU_MAPS)] { WARN_ON_ONCE(cpu >= bits); } }
#[inline(always)] pub unsafe fn cpumask_check(cpu: u32) -> u32 { cpu_max_bits_warn(cpu, small_cpumask_bits()); cpu }

#[inline(always)] pub unsafe fn cpumask_first(p: *const cpumask) -> u32 { find_first_bit(cpumask_bits(p), small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_first_zero(p: *const cpumask) -> u32 { find_first_zero_bit(cpumask_bits(p), small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_first_and(a: *const cpumask,b:*const cpumask)->u32 { find_first_and_bit(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_first_andnot(a:*const cpumask,b:*const cpumask)->u32 { find_first_andnot_bit(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_first_and_and(a:*const cpumask,b:*const cpumask,c:*const cpumask)->u32 { find_first_and_and_bit(cpumask_bits(a),cpumask_bits(b),cpumask_bits(c),small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_last(p:*const cpumask)->u32 { find_last_bit(cpumask_bits(p),small_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_next(n:i32,p:*const cpumask)->u32 { if n != -1 { cpumask_check(n as u32); } find_next_bit(cpumask_bits(p),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_next_zero(n:i32,p:*const cpumask)->u32 { if n != -1 { cpumask_check(n as u32); } find_next_zero_bit(cpumask_bits(p),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_next_and(n:i32,a:*const cpumask,b:*const cpumask)->u32 { if n!=-1 {cpumask_check(n as u32);} find_next_and_bit(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_next_andnot(n:i32,a:*const cpumask,b:*const cpumask)->u32 { if n!=-1 {cpumask_check(n as u32);} find_next_andnot_bit(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_next_and_wrap(n:i32,a:*const cpumask,b:*const cpumask)->u32 { if n!=-1 {cpumask_check(n as u32);} find_next_and_bit_wrap(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_next_wrap(n:i32,p:*const cpumask)->u32 { if n!=-1 {cpumask_check(n as u32);} find_next_bit_wrap(cpumask_bits(p),small_cpumask_bits(),n+1) }
#[inline(always)] pub unsafe fn cpumask_random(p:*const cpumask)->u32 { find_random_bit(cpumask_bits(p),nr_cpu_ids()) }

#[inline(always)] pub unsafe fn cpumask_any_but(mask:*const cpumask,cpu:i32)->u32 { let mut i=cpumask_first(mask); while i<nr_cpu_ids() && i!=cpu as u32 { break; } i }
#[inline(always)] pub unsafe fn cpumask_any_and_but(a:*const cpumask,b:*const cpumask,cpu:i32)->u32 { let i=cpumask_first_and(a,b); if i!=cpu as u32 {i} else {cpumask_next_and(cpu,a,b)} }
#[inline(always)] pub unsafe fn cpumask_any_andnot_but(a:*const cpumask,b:*const cpumask,cpu:i32)->u32 { let i=cpumask_first_andnot(a,b); if i!=cpu as u32 {i} else {cpumask_next_andnot(cpu,a,b)} }
#[inline(always)] pub unsafe fn cpumask_nth(n:u32,p:*const cpumask)->u32 { find_nth_bit(cpumask_bits(p),small_cpumask_bits(),cpumask_check(n)) }
#[inline(always)] pub unsafe fn cpumask_nth_and(n:u32,a:*const cpumask,b:*const cpumask)->u32 { find_nth_and_bit(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits(),cpumask_check(n)) }
#[inline(always)] pub unsafe fn cpumask_nth_and_andnot(n:u32,a:*const cpumask,b:*const cpumask,c:*const cpumask)->u32 { find_nth_and_andnot_bit(cpumask_bits(a),cpumask_bits(b),cpumask_bits(c),small_cpumask_bits(),cpumask_check(n)) }

#[inline(always)] pub unsafe fn cpumask_set_cpu(cpu:u32,dst:*mut cpumask){set_bit(cpumask_check(cpu),cpumask_bits_mut(dst));}
#[inline(always)] pub unsafe fn __cpumask_set_cpu(cpu:u32,dst:*mut cpumask){__set_bit(cpumask_check(cpu),cpumask_bits_mut(dst));}
#[inline(always)] pub unsafe fn cpumask_clear_cpu(cpu:i32,dst:*mut cpumask){clear_bit(cpumask_check(cpu as u32),cpumask_bits_mut(dst));}
#[inline(always)] pub unsafe fn __cpumask_clear_cpu(cpu:i32,dst:*mut cpumask){__clear_bit(cpumask_check(cpu as u32),cpumask_bits_mut(dst));}
#[inline(always)] pub unsafe fn cpumask_test_cpu(cpu:i32,p:*const cpumask)->bool{test_bit(cpumask_check(cpu as u32),cpumask_bits(p))}
#[inline(always)] pub unsafe fn cpumask_test_and_set_cpu(cpu:i32,p:*mut cpumask)->bool{test_and_set_bit(cpumask_check(cpu as u32),cpumask_bits_mut(p))}
#[inline(always)] pub unsafe fn cpumask_test_and_clear_cpu(cpu:i32,p:*mut cpumask)->bool{test_and_clear_bit(cpumask_check(cpu as u32),cpumask_bits_mut(p))}

#[inline(always)] pub unsafe fn cpumask_setall(p:*mut cpumask){bitmap_fill(cpumask_bits_mut(p),nr_cpumask_bits());}
#[inline(always)] pub unsafe fn cpumask_clear(p:*mut cpumask){bitmap_zero(cpumask_bits_mut(p),large_cpumask_bits());}
#[inline(always)] pub unsafe fn cpumask_and(d:*mut cpumask,a:*const cpumask,b:*const cpumask)->bool{bitmap_and(cpumask_bits_mut(d),cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_or(d:*mut cpumask,a:*const cpumask,b:*const cpumask){bitmap_or(cpumask_bits_mut(d),cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_xor(d:*mut cpumask,a:*const cpumask,b:*const cpumask){bitmap_xor(cpumask_bits_mut(d),cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_andnot(d:*mut cpumask,a:*const cpumask,b:*const cpumask)->bool{bitmap_andnot(cpumask_bits_mut(d),cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_equal(a:*const cpumask,b:*const cpumask)->bool{bitmap_equal(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_intersects(a:*const cpumask,b:*const cpumask)->bool{bitmap_intersects(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_subset(a:*const cpumask,b:*const cpumask)->bool{bitmap_subset(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_empty(a:*const cpumask)->bool{bitmap_empty(cpumask_bits(a),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_full(a:*const cpumask)->bool{bitmap_full(cpumask_bits(a),nr_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_weight(a:*const cpumask)->u32{bitmap_weight(cpumask_bits(a),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_weight_and(a:*const cpumask,b:*const cpumask)->u32{bitmap_weight_and(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_weight_andnot(a:*const cpumask,b:*const cpumask)->u32{bitmap_weight_andnot(cpumask_bits(a),cpumask_bits(b),small_cpumask_bits())}
#[inline(always)] pub unsafe fn cpumask_shift_right(d:*mut cpumask,a:*const cpumask,n:i32){bitmap_shift_right(cpumask_bits_mut(d),cpumask_bits(a),n,small_cpumask_bits());}
#[inline(always)] pub unsafe fn cpumask_shift_left(d:*mut cpumask,a:*const cpumask,n:i32){bitmap_shift_left(cpumask_bits_mut(d),cpumask_bits(a),n,nr_cpumask_bits());}
#[inline(always)] pub unsafe fn cpumask_copy(d:*mut cpumask,a:*const cpumask){bitmap_copy(cpumask_bits_mut(d),cpumask_bits(a),large_cpumask_bits());}

pub macro_rules! cpumask_any { ($p:expr) => { cpumask_first($p) }; }
pub macro_rules! cpumask_any_and { ($a:expr,$b:expr) => { cpumask_first_and($a,$b) }; }
pub macro_rules! for_each_cpu { ($cpu:ident,$mask:expr) => { for $cpu in 0..small_cpumask_bits() { if unsafe { !test_bit($cpu,cpumask_bits($mask)) } { continue; } } }; }
pub macro_rules! num_online_cpus { () => { 1u32 }; }
pub macro_rules! num_possible_cpus { () => { 1u32 }; }
pub macro_rules! num_enabled_cpus { () => { 1u32 }; }
pub macro_rules! num_present_cpus { () => { 1u32 }; }
pub macro_rules! num_active_cpus { () => { 1u32 }; }
pub macro_rules! cpu_is_offline { ($cpu:expr) => { !cpu_online($cpu) }; }

extern "C" {
    pub fn cpumask_parse_user(buf:*const i8,len:i32,dst:*mut cpumask)->i32;
    pub fn cpumask_parselist_user(buf:*const i8,len:i32,dst:*mut cpumask)->i32;
    pub fn bitmap_parse(buf:*const i8,len:usize,dst:*mut c_ulong,bits:u32)->i32;
    pub fn bitmap_parselist(buf:*const i8,dst:*mut c_ulong,bits:u32)->i32;
    pub fn alloc_cpumask_var_node(mask:*mut cpumask_var_t,flags:gfp_t,node:i32)->bool;
    pub fn alloc_bootmem_cpumask_var(mask:*mut cpumask_var_t);
    pub fn free_cpumask_var(mask:cpumask_var_t);
    pub fn free_bootmem_cpumask_var(mask:cpumask_var_t);
}
#[inline(always)] pub unsafe fn cpulist_parse(buf:*const i8,dst:*mut cpumask)->i32 { bitmap_parselist(buf,cpumask_bits_mut(dst),nr_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_size()->u32 { bitmap_size(large_cpumask_bits()) }
#[inline(always)] pub unsafe fn cpumask_available(_mask:cpumask_var_t)->bool { true }
#[inline(always)] pub unsafe fn alloc_cpumask_var(_mask:*mut cpumask_var_t,_flags:gfp_t)->bool { true }
#[inline(always)] pub unsafe fn zalloc_cpumask_var(mask:*mut cpumask_var_t,_flags:gfp_t)->bool { cpumask_clear(*mask); true }
#[inline(always)] pub unsafe fn alloc_cpumask_var_node(_mask:*mut cpumask_var_t,_flags:gfp_t,_node:i32)->bool { true }
#[inline(always)] pub unsafe fn zalloc_cpumask_var_node(mask:*mut cpumask_var_t,_flags:gfp_t,_node:i32)->bool { cpumask_clear(*mask); true }
#[inline(always)] pub unsafe fn to_cpumask(bitmap:*const c_ulong)->*const cpumask { bitmap as *const cpumask }
#[inline(always)] pub unsafe fn cpumask_of(cpu:u32)->*const cpumask { get_cpu_mask(cpu) }
#[inline(always)] pub unsafe fn num_online_cpus_fn()->u32 { 1 }
#[inline(always)] pub unsafe fn num_possible_cpus_fn()->u32 { 1 }
pub macro_rules! set_cpu_enabled { ($cpu:expr,$v:expr) => { assign_bit(unsafe{cpumask_check($cpu)},cpumask_bits_mut(unsafe{&mut __cpu_enabled_mask}),$v) }; }
pub macro_rules! set_cpu_present { ($cpu:expr,$v:expr) => { assign_bit(unsafe{cpumask_check($cpu)},cpumask_bits_mut(unsafe{&mut __cpu_present_mask}),$v) }; }
pub macro_rules! set_cpu_active { ($cpu:expr,$v:expr) => { assign_bit(unsafe{cpumask_check($cpu)},cpumask_bits_mut(unsafe{&mut __cpu_active_mask}),$v) }; }
pub macro_rules! set_cpu_dying { ($cpu:expr,$v:expr) => { assign_bit(unsafe{cpumask_check($cpu)},cpumask_bits_mut(unsafe{&mut __cpu_dying_mask}),$v) }; }

#[inline(always)] pub unsafe fn get_cpu_mask(cpu:u32)->*const cpumask { let p=cpu_bit_bitmap[(1 + cpu as usize % BITS_PER_LONG)]; let _=p; to_cpumask(&cpu_bit_bitmap[1 + cpu as usize % BITS_PER_LONG]) }
#[inline(always)] pub unsafe fn cpu_online(cpu:u32)->bool{cpu==0}
#[inline(always)] pub unsafe fn cpu_possible(cpu:u32)->bool{cpu==0}
#[inline(always)] pub unsafe fn cpu_enabled(cpu:u32)->bool{cpu==0}
#[inline(always)] pub unsafe fn cpu_present(cpu:u32)->bool{cpu==0}
#[inline(always)] pub unsafe fn cpu_active(cpu:u32)->bool{cpu==0}
#[inline(always)] pub unsafe fn cpu_dying(_cpu:u32)->bool{false}

pub const CPUMAP_FILE_MAX_BYTES: usize = if (NR_CPUS*9)/32 > PAGE_SIZE {(NR_CPUS*9)/32-1} else {PAGE_SIZE};
pub const CPULIST_FILE_MAX_BYTES: usize = if (NR_CPUS*7)/2 > PAGE_SIZE {(NR_CPUS*7)/2} else {PAGE_SIZE};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
