/* Private differential fixture; lib/sort.c is compiled unchanged separately. */
#include <linux/sort.h>
void original_sort(void *, size_t, size_t, cmp_func_t, swap_func_t);
void original_sort_nonatomic(void *, size_t, size_t, cmp_func_t, swap_func_t);
void original_sort_r(void *, size_t, size_t, cmp_r_func_t, swap_r_func_t, const void *);
void original_sort_r_nonatomic(void *, size_t, size_t, cmp_r_func_t, swap_r_func_t, const void *);
void rust_consumer(unsigned, void *, size_t, size_t, cmp_func_t, swap_func_t, cmp_r_func_t, swap_r_func_t, const void *);
int cmp_int_domains(void);

static unsigned char data[2][32768] __attribute__((aligned(16)));
static size_t trace[2][200000];
static unsigned active;
static size_t used[2];
static unsigned char *base;
static const unsigned context = 0x13579;
static void finish(int status) __attribute__((noreturn));
static void finish(int status) {
#if __SIZEOF_POINTER__ == 8
    __asm__ volatile("syscall" : : "a"(60L), "D"((long)status) : "rcx", "r11", "memory");
#else
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#endif
    __builtin_unreachable();
}
void fixture_panic(void) { finish(90); }
void *memcpy(void *d, const void *s, size_t n) { unsigned char *a=d; const unsigned char *b=s; for(size_t i=0;i<n;i++)a[i]=b[i];return d; }
void *memset(void *d, int c, size_t n) { unsigned char *a=d;for(size_t i=0;i<n;i++)a[i]=(unsigned char)c;return d; }
void *memmove(void *d, const void *s, size_t n) { unsigned char *a=d;const unsigned char *b=s;if(a>b){while(n){--n;a[n]=b[n];}}else{for(size_t i=0;i<n;i++)a[i]=b[i];}return d; }
static void event(size_t tag, const void *a, const void *b, size_t n) {
    if(used[active]+4 >= 200000)finish(91);
    size_t *p=trace[active]+used[active];used[active]+=4;
    p[0]=tag;p[1]=(const unsigned char *)a-base;p[2]=(const unsigned char *)b-base;p[3]=n;
}
void cond_resched(void) { event(3,base,base,0); }
int __cond_resched(void) { cond_resched(); return 0; }
int cmp_callback(const void *a,const void *b) {
    event(1,a,b,0);
    unsigned char x=*(const unsigned char *)a,y=*(const unsigned char *)b;
    return (x>y)-(x<y);
}
int cmpr_callback(const void *a,const void *b,const void *p) { if(p!=&context)finish(92); return cmp_callback(a,b); }
void swap_callback(void *a,void *b,int n) {
    event(2,a,b,(size_t)n);
    unsigned char *x=a,*y=b;
    while(n){--n;unsigned char t=x[n];x[n]=y[n];y[n]=t;}
}
void swapr_callback(void *a,void *b,int n,const void *p) { if(p!=&context)finish(93);swap_callback(a,b,n); }
typedef void (*sort_t)(void *,size_t,size_t,cmp_func_t,swap_func_t);
typedef void (*sortr_t)(void *,size_t,size_t,cmp_r_func_t,swap_r_func_t,const void *);
static sort_t volatile plain[4]={original_sort,original_sort_nonatomic,sort,sort_nonatomic};
static sortr_t volatile ctx[4]={original_sort_r,original_sort_r_nonatomic,sort_r,sort_r_nonatomic};
static void invoke(unsigned mode,unsigned rust,unsigned consumer,void *b,size_t n,size_t s,unsigned custom,unsigned nullcmp) {
    cmp_func_t c=nullcmp?NULL:cmp_callback;
    cmp_r_func_t cr=nullcmp?NULL:cmpr_callback;
    swap_func_t w=custom?swap_callback:NULL;
    swap_r_func_t wr=custom?swapr_callback:NULL;
    if(rust&&consumer)rust_consumer(mode,b,n,s,c,w,cr,wr,&context);
    else if(mode<2)plain[rust*2+mode](b,n,s,c,w);
    else ctx[rust*2+mode-2](b,n,s,cr,wr,&context);
}
static void one(unsigned mode,unsigned consumer,size_t n,size_t size,unsigned custom,unsigned offset,unsigned seed) {
    for(unsigned side=0;side<2;side++) {
        for(size_t i=0;i<sizeof(data[side]);i++)data[side][i]=(unsigned char)((i*53+seed*31)^(i>>3));
        for(size_t i=0;i<n;i++)data[side][offset+i*size]=(unsigned char)((i*seed+seed/3)%17);
        active=side;base=data[side]+offset;used[side]=0;
        invoke(mode,side,consumer,base,n,size,custom,0);
    }
    if(used[0]!=used[1])finish(10);
    for(size_t i=0;i<used[0];i++)if(trace[0][i]!=trace[1][i])finish(11);
    for(size_t i=0;i<sizeof(data[0]);i++)if(data[0][i]!=data[1][i])finish(12);
    for(size_t i=1;i<n;i++)if(data[1][offset+(i-1)*size]>data[1][offset+i*size])finish(13);
}
static int tests(void) {
    if(!cmp_int_domains())return 14;
    for(unsigned mode=0;mode<4;mode++)for(unsigned consumer=0;consumer<2;consumer++) {
        for(unsigned side=0;side<2;side++) {
            active=side;base=data[side];used[side]=0;
            invoke(mode,side,consumer,NULL,0,(size_t)-1,0,1);
            invoke(mode,side,consumer,NULL,1,(size_t)-1,1,1);
            invoke(mode,side,consumer,NULL,(size_t)-1,0,0,1);
            if(used[side])return 15;
        }
        for(unsigned custom=0;custom<2;custom++)for(unsigned off=0;off<4;off++)
        for(unsigned size=1;size<=17;size++)for(unsigned n=0;n<=65;n++)
            one(mode,consumer,n,size,custom,off,1+n%11);
        one(mode,consumer,1023,16,0,0,19);
        one(mode,consumer,1023,17,1,1,7);
    }
    return 0;
}
unsigned wrong_cmp(const void *a,const void *b) { return (unsigned)cmp_callback(a,b); }
unsigned wrong_cmpr(const void *a,const void *b,const void *p) { return (unsigned)cmpr_callback(a,b,p); }
void wrong_swap(void *a,void *b,unsigned n) { swap_callback(a,b,(int)n); }
void wrong_swapr(void *a,void *b,unsigned n,const void *p) { swapr_callback(a,b,(int)n,p); }
void rust_negative(unsigned,void *,cmp_func_t,cmp_r_func_t,const void *);
#ifndef NEGATIVE
#define NEGATIVE 0
#endif
void _start(void) {
    if(!NEGATIVE)finish(tests());
    active=0;base=data[0];base[0]=2;base[1]=1;
    if(NEGATIVE==1)plain[2](base,2,1,(cmp_func_t)wrong_cmp,NULL);
    if(NEGATIVE==2)plain[2](base,2,1,cmp_callback,(swap_func_t)wrong_swap);
    if(NEGATIVE==3)ctx[2](base,2,1,(cmp_r_func_t)wrong_cmpr,NULL,&context);
    if(NEGATIVE==4)ctx[2](base,2,1,cmpr_callback,(swap_r_func_t)wrong_swapr,&context);
    if(NEGATIVE>=5)rust_negative(NEGATIVE-5,base,cmp_callback,cmpr_callback,&context);
    finish(base[0]!=1||base[1]!=2);
}
