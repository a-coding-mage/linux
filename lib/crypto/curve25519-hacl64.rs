// SPDX-License-Identifier: GPL-2.0 OR MIT
// Faithful low-level translation of curve25519-hacl64.c.

type U128 = u128;
type U64 = u64;
type U32 = u32;
type U8 = u8;

#[inline(always)] unsafe fn u64_eq_mask(a: U64,b: U64)->U64 { let x=a^b; let m=!x+1; let q=x|m; (q>>63)-1 }
#[inline(always)] unsafe fn u64_gte_mask(a: U64,b: U64)->U64 { let q=(a^b)|((a-b)^b); ((a^q)>>63)-1 }
#[inline(always)] unsafe fn modulo_carry_top(b:*mut U64){let x=*b.add(4);let y=*b;*b.add(4)=x&0x7ffffffffffff;*b=y+19*(x>>51);}
#[inline(always)] unsafe fn fproduct_copy_from_wide_(o:*mut U64,i:*mut U128){for n in 0..5{*o.add(n)=*i.add(n) as U64;}}
#[inline(always)] unsafe fn fproduct_sum_scalar_multiplication_(o:*mut U128,i:*mut U64,s:U64){for n in 0..5{*o.add(n)+=(*i.add(n) as U128)*(s as U128);}}
#[inline(always)] unsafe fn fproduct_carry_wide_(t:*mut U128){for n in 0..4{let x=*t.add(n);*t.add(n)=x&0x7ffffffffffff;*t.add(n+1)+=x>>51;}}
#[inline(always)] unsafe fn fmul_shift_reduce(o:*mut U64){let t=*o.add(4);for n in (1..5).rev(){*o.add(n)=*o.add(n-1);}*o=t;*o*=19;}
#[inline(always)] unsafe fn fmul_mul_shift_reduce_(o:*mut U128,i:*mut U64,s:*mut U64){for n in 0..4{fproduct_sum_scalar_multiplication_(o,i,*s.add(n));fmul_shift_reduce(i);}fproduct_sum_scalar_multiplication_(o,i,*s.add(4));}
#[inline(always)] unsafe fn fmul_fmul(o:*mut U64,i:*mut U64,s:*mut U64){let mut a=[0u64;5];for n in 0..5{a[n]=*i.add(n)}let mut t=[0u128;5];fmul_mul_shift_reduce_(t.as_mut_ptr(),a.as_mut_ptr(),s);fproduct_carry_wide_(t.as_mut_ptr());let b4=t[4];t[4]=b4&0x7ffffffffffff;t[0]+=19*((b4>>51) as u128);fproduct_copy_from_wide_(o,t.as_mut_ptr());let x=*o;*o&=0x7ffffffffffff;*o.add(1)+=x>>51;}
#[inline(always)] unsafe fn fsquare_fsquare__(t:*mut U128,o:*mut U64){let r0=*o;let r1=*o.add(1);let r2=*o.add(2);let r3=*o.add(3);let r4=*o.add(4);let d0=r0*2;let d1=r1*2;let d2=r2*2*19;let d419=r4*19;let d4=d419*2;t.write((r0 as u128*r0 as u128)+(d4 as u128*r1 as u128)+(d2 as u128*r3 as u128));t.add(1).write((d0 as u128*r1 as u128)+(d4 as u128*r2 as u128)+((r3*19) as u128*r3 as u128));t.add(2).write((d0 as u128*r2 as u128)+(r1 as u128*r1 as u128)+(d4 as u128*r3 as u128));t.add(3).write((d0 as u128*r3 as u128)+(d1 as u128*r2 as u128)+(r4 as u128*d419 as u128));t.add(4).write((d0 as u128*r4 as u128)+(d1 as u128*r3 as u128)+(r2 as u128*r2 as u128));}
#[inline(always)] unsafe fn fsquare_fsquare_(t:*mut U128,o:*mut U64){fsquare_fsquare__(t,o);fproduct_carry_wide_(t);let b4=*t.add(4);*t.add(4)=b4&0x7ffffffffffff;*t+=19*((b4>>51) as u128);fproduct_copy_from_wide_(o,t);let x=*o;*o&=0x7ffffffffffff;*o.add(1)+=x>>51;}
#[inline(always)] unsafe fn fsquare_fsquare_times_(o:*mut U64,t:*mut U128,n:U32){fsquare_fsquare_(t,o);for _ in 1..n{fsquare_fsquare_(t,o)}}
#[inline(always)] unsafe fn fsquare_fsquare_times(o:*mut U64,i:*mut U64,n:U32){for k in 0..5{*o.add(k)=*i.add(k)}let mut t=[0u128;5];fsquare_fsquare_times_(o,t.as_mut_ptr(),n)}
#[inline(always)] unsafe fn fsquare_fsquare_times_inplace(o:*mut U64,n:U32){let mut t=[0u128;5];fsquare_fsquare_times_(o,t.as_mut_ptr(),n)}
#[inline(always)] unsafe fn crecip_crecip(out:*mut U64,z:*mut U64){let mut b=[0u64;20];let a=b.as_mut_ptr();let t=a.add(5);let q=a.add(10);fsquare_fsquare_times(a,z,1);fsquare_fsquare_times(t,a,2);fmul_fmul(q,t,z);fmul_fmul(a,q,a);fsquare_fsquare_times(t,a,1);fmul_fmul(q,t,q);fsquare_fsquare_times(t,q,5);let b1=a.add(10);let c=a.add(15);fmul_fmul(b1,t,b1);fsquare_fsquare_times(t,b1,10);fmul_fmul(c,t,b1);fsquare_fsquare_times(t,c,20);fmul_fmul(t,t,c);fsquare_fsquare_times_inplace(t,10);fmul_fmul(b1,t,b1);fsquare_fsquare_times(t,b1,50);fmul_fmul(c,a.add(5),b1);fsquare_fsquare_times(a.add(5),c,100);fmul_fmul(a.add(5),a.add(5),c);fsquare_fsquare_times_inplace(a.add(5),50);fmul_fmul(a.add(5),a.add(5),b1);fsquare_fsquare_times_inplace(a.add(5),5);fmul_fmul(out,a.add(5),a);}
#[inline(always)] unsafe fn fsum(a:*mut U64,b:*mut U64){for n in 0..5{*a.add(n)+=*b.add(n)}}
#[inline(always)] unsafe fn fdifference(a:*mut U64,b:*mut U64){let c=[0x3fffffffffff68,0x3ffffffffffff8,0x3ffffffffffff8,0x3ffffffffffff8,0x3ffffffffffff8];for n in 0..5{*a.add(n)=*b.add(n)+c[n]-*a.add(n)}}
#[inline(always)] unsafe fn fscalar(o:*mut U64,b:*mut U64,s:U64){let mut t=[0u128;5];for n in 0..5{t[n]=*b.add(n) as u128*s as u128}fproduct_carry_wide_(t.as_mut_ptr());let x=t[4];t[4]=x&0x7ffffffffffff;t[0]+=19*((x>>51) as u128);fproduct_copy_from_wide_(o,t.as_mut_ptr())}
#[inline(always)] unsafe fn fmul(o:*mut U64,a:*mut U64,b:*mut U64){fmul_fmul(o,a,b)}
#[inline(always)] unsafe fn crecip(o:*mut U64,i:*mut U64){crecip_crecip(o,i)}
#[inline(always)] unsafe fn point_swap_conditional5(a:*mut U64,b:*mut U64,s:U64){for n in (0..5).rev(){let x=s&(*a.add(n)^*b.add(n));*a.add(n)^=x;*b.add(n)^=x}}
#[inline(always)] unsafe fn point_swap_conditional(a:*mut U64,b:*mut U64,s:U64){let x=0-s;point_swap_conditional5(a,b,x);point_swap_conditional5(a.add(5),b.add(5),x)}
#[inline(always)] unsafe fn point_copy(o:*mut U64,i:*mut U64){for n in 0..10{*o.add(n)=*i.add(n)}}
#[inline(always)] unsafe fn addanddouble_fmonty(pp:*mut U64,ppq:*mut U64,p:*mut U64,pq:*mut U64,qmqp:*mut U64){let mut ox=[0u64;40];let qx=qmqp;let x2=pp;let z2=pp.add(5);let x3=ppq;let z3=ppq.add(5);let x=p;let z=p.add(5);let xp=pq;let zp=pq.add(5);let ox0=ox.as_mut_ptr();let oxp0=ox0.add(5);let xxp=ox0.add(25);let zzp=ox0.add(30);for n in 0..5{*ox0.add(n)=*x.add(n);*oxp0.add(n)=*xp.add(n)}fsum(x,z);fdifference(z,ox0);fsum(xp,zp);fdifference(zp,oxp0);fmul(xxp,xp,z);fmul(zzp,x,zp);let xx=ox0.add(15);let zz=ox0.add(20);for n in 0..5{*xxp.add(n)=*xxp.add(n)}fsum(xxp,zzp);fdifference(zzp,xxp);fsquare_fsquare_times(x3,xxp,1);fsquare_fsquare_times(ox0.add(35),zzp,1);fmul(z3,ox0.add(35),qx);fsquare_fsquare_times(xx,x,1);fsquare_fsquare_times(zz,z,1);fmul(x2,xx,zz);fdifference(zz,xx);fscalar(ox0.add(10),zz,121665);fsum(ox0.add(10),xx);fmul(z2,ox0.add(10),zz)}
#[inline(always)] unsafe fn ladder_step(nq:*mut U64,nqpq:*mut U64,nq2:*mut U64,nqpq2:*mut U64,q:*mut U64,byt:U8){let bit=(byt>>7) as U64;point_swap_conditional(nq,nqpq,bit);addanddouble_fmonty(nq2,nqpq2,nq,nqpq,q);point_swap_conditional(nq2,nqpq2,(byt>>7) as U64)}
#[inline(always)] unsafe fn ladder_double(nq:*mut U64,nqpq:*mut U64,nq2:*mut U64,nqpq2:*mut U64,q:*mut U64,b:U8){ladder_step(nq,nqpq,nq2,nqpq2,q,b);ladder_step(nq2,nqpq2,nq,nqpq,q,b<<1)}
#[inline(always)] unsafe fn ladder_small(nq:*mut U64,nqpq:*mut U64,nq2:*mut U64,nqpq2:*mut U64,q:*mut U64,mut b:U8,mut i:U32){while i>0{ladder_double(nq,nqpq,nq2,nqpq2,q,b);b<<=2;i-=1}}
#[inline(always)] unsafe fn ladder_big(n:*mut U8,nq:*mut U64,nqpq:*mut U64,nq2:*mut U64,nqpq2:*mut U64,q:*mut U64,mut i:U32){while i>0{i-=1;ladder_small(nq,nqpq,nq2,nqpq2,q,*n.add(i),4)}}
unsafe fn ladder_cmult(r:*mut U64,n:*mut U8,q:*mut U64){let mut p=[0u64;40];point_copy(p.as_mut_ptr().add(10),q);p[0]=1;ladder_big(n,p.as_mut_ptr(),p.as_mut_ptr().add(10),p.as_mut_ptr().add(20),p.as_mut_ptr().add(30),q,32);point_copy(r,p.as_mut_ptr())}
#[inline(always)] unsafe fn get64(p:*const U8)->U64{let mut x=0;for n in 0..8{x|=(*p.add(n) as u64)<<(8*n)}x}
#[inline(always)] unsafe fn put64(p:*mut U8,x:U64){for n in 0..8{*p.add(n)=(x>>(8*n)) as u8}}
#[inline(always)] unsafe fn format_fexpand(o:*mut U64,i:*const U8){let a=[get64(i),get64(i.add(6)),get64(i.add(12)),get64(i.add(19)),get64(i.add(24))];*o=a[0]&0x7ffffffffffff;*o.add(1)=a[1]>>3&0x7ffffffffffff;*o.add(2)=a[2]>>6&0x7ffffffffffff;*o.add(3)=a[3]>>1&0x7ffffffffffff;*o.add(4)=a[4]>>12&0x7ffffffffffff}
#[inline(always)] unsafe fn format_fcontract(o:*mut U8,i:*mut U64){for _ in 0..2{for n in 0..4{let c=*i.add(n)>>51;*i.add(n)&=0x7ffffffffffff;*i.add(n+1)+=c}let c=*i.add(4)>>51;*i.add(4)&=0x7ffffffffffff;*i+=19*c}let m=u64_gte_mask(*i,0x7ffffffffffed)&u64_eq_mask(*i.add(1),0x7ffffffffffff)&u64_eq_mask(*i.add(2),0x7ffffffffffff)&u64_eq_mask(*i.add(3),0x7ffffffffffff)&u64_eq_mask(*i.add(4),0x7ffffffffffff);*i-=0x7ffffffffffed&m;for n in 1..5{*i.add(n)-=0x7ffffffffffff&m}put64(o,*i|(*i.add(1)<<51));put64(o.add(8),*i.add(1)>>13|(*i.add(2)<<38));put64(o.add(16),*i.add(2)>>26|(*i.add(3)<<25));put64(o.add(24),*i.add(3)>>39|(*i.add(4)<<12))}
#[inline(always)] unsafe fn format_scalar_of_point(o:*mut U8,p:*mut U64){let mut z=[0u64;10];crecip(z.as_mut_ptr(),p.add(5));fmul(z.as_mut_ptr().add(5),p,z.as_mut_ptr());format_fcontract(o,z.as_mut_ptr().add(5))}
extern "C" { fn curve25519_clamp_secret(e:*mut U8); }
#[no_mangle] pub unsafe extern "C" fn curve25519_generic(out:*mut U8,secret:*const U8,base:*const U8){let mut b=[0u64;10];format_fexpand(b.as_mut_ptr(),base);b[5]=1;let mut e=[0u8;32];for n in 0..32{e[n]=*secret.add(n)}curve25519_clamp_secret(e.as_mut_ptr());let mut p=[0u64;15];p[0]=1;ladder_cmult(p.as_mut_ptr(),e.as_mut_ptr(),b.as_mut_ptr());format_scalar_of_point(out,p.as_mut_ptr());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
