/* SPDX-License-Identifier: GPL-2.0-only */
/* Private differential fixture. Kernel headers define every UUID ABI type. */
#ifdef ORDINARY
#include "ordinary.h"
#else
#include <linux/uuid.h>
#include <linux/random.h>
#endif

extern int puts(const char *);
extern void abort(void);
extern void *mmap(void *, size_t, int, int, int, long);
extern int mprotect(void *, size_t, int);
extern int munmap(void *, size_t);

void c_generate_random_uuid(unsigned char *);
void c_generate_random_guid(unsigned char *);
void c_guid_gen(guid_t *);
void c_uuid_gen(uuid_t *);
bool c_uuid_is_valid(const char *);
int c_guid_parse(const char *, guid_t *);
int c_uuid_parse(const char *, uuid_t *);
int main(void);
bool uuid_header_check(void);
unsigned uuid_initializer_check(unsigned, unsigned long long, unsigned char *);
static unsigned calls, seed;
static void *expected;
static unsigned char before[16];

#define CHECK(x) do { if (!(x)) abort(); } while (0)
void get_random_bytes(void *p, size_t len)
{
	unsigned char *b = p;
	unsigned i;
	CHECK(p == expected && len == 16 && calls++ == 0);
	for (i = 0; i < 16; i++) {
		CHECK(b[i] == before[i]);
		b[i] = (unsigned char)(seed + i * 37);
	}
}

static void fill(unsigned char *b, unsigned n, unsigned char v)
{
	unsigned i;
	for (i = 0; i < n; i++) b[i] = v;
}
static void copy(unsigned char *a, const unsigned char *b, unsigned n)
{
	unsigned i;
	for (i = 0; i < n; i++) a[i] = b[i];
}
static void equal(const unsigned char *a, const unsigned char *b, unsigned n)
{
	unsigned i;
	for (i = 0; i < n; i++) CHECK(a[i] == b[i]);
}

static void one(const unsigned char *text, unsigned n)
{
	unsigned char a[128], b[128];
	int offset, kind, ca, cb;
	CHECK(c_uuid_is_valid((const char *)text) == uuid_is_valid((const char *)text));
	for (kind = 0; kind != 2; kind++) {
		fill(a, sizeof(a), 0xa5);
		fill(b, sizeof(b), 0xa5);
		ca = kind ? c_guid_parse((const char *)text, (guid_t *)(a+48)) :
			c_uuid_parse((const char *)text, (uuid_t *)(a+48));
		cb = kind ? guid_parse((const char *)text, (guid_t *)(b+48)) :
			uuid_parse((const char *)text, (uuid_t *)(b+48));
		CHECK(ca == cb);
		equal(a, b, sizeof(a));
		for (offset = -20; offset <= 40; offset++) {
			fill(a, sizeof(a), 0xa5);
			copy(a+40, text, n);
			copy(b, a, sizeof(a));
			ca = kind ? c_guid_parse((char *)a+40, (guid_t *)(a+40+offset)) :
				c_uuid_parse((char *)a+40, (uuid_t *)(a+40+offset));
			cb = kind ? guid_parse((char *)b+40, (guid_t *)(b+40+offset)) :
				uuid_parse((char *)b+40, (uuid_t *)(b+40+offset));
			CHECK(ca == cb);
			equal(a, b, sizeof(a));
		}
	}
}


static unsigned initializer_calls[11];
static unsigned long long initializer_next(unsigned index, unsigned long long value)
{
 initializer_calls[index]++;
 return value;
}

static void initializer_cases(void)
{
 static const guid_t gs[] = {
 GUID_INIT((u8)0x12,(u8)0x34,(u8)0x56,0,1,2,3,4,5,6,7),
 GUID_INIT((signed char)-18,(signed char)-52,(signed char)-86,-1,1,2,3,4,5,6,7),
 GUID_INIT(0x123456789abcdef0ULL,0x12345678ULL,0x9abcdef0ULL,0,1,2,3,4,5,6,7)};
 static const uuid_t us[] = {
 UUID_INIT((u8)0x12,(u8)0x34,(u8)0x56,0,1,2,3,4,5,6,7),
 UUID_INIT((signed char)-18,(signed char)-52,(signed char)-86,-1,1,2,3,4,5,6,7),
 UUID_INIT(0x123456789abcdef0ULL,0x12345678ULL,0x9abcdef0ULL,0,1,2,3,4,5,6,7)};
 unsigned char b[16]; unsigned i; unsigned long long v;
 static const guid_t signed_g=GUID_INIT(-0x123456789LL,-0x123456789LL,-0x123456789LL,0,1,2,3,4,5,6,7);
 static const uuid_t signed_u=UUID_INIT(-0x123456789LL,-0x123456789LL,-0x123456789LL,0,1,2,3,4,5,6,7);
 CHECK(uuid_initializer_check(14,0,b)); equal(b,signed_g.b,16);
 CHECK(uuid_initializer_check(15,0,b)); equal(b,signed_u.b,16);
 for (i=0;i<6;i++) {
  CHECK(uuid_initializer_check(i,0,b));
  equal(b, i%2 ? us[i/2].b : gs[i/2].b,16);
 }
 for (v=0;v<256;v++) {
  guid_t g8=GUID_INIT((u8)v,(u8)v,(u8)v,0,1,2,3,4,5,6,7);
  uuid_t u8v=UUID_INIT((u8)v,(u8)v,(u8)v,0,1,2,3,4,5,6,7);
  guid_t gs8=GUID_INIT((signed char)v,(signed char)v,(signed char)v,0,1,2,3,4,5,6,7);
  uuid_t us8=UUID_INIT((signed char)v,(signed char)v,(signed char)v,0,1,2,3,4,5,6,7);
  unsigned long long wide=v*0x0102030405060708ULL;
  guid_t gw=GUID_INIT(wide,wide,wide,0,1,2,3,4,5,6,7);
  uuid_t uw=UUID_INIT(wide,wide,wide,0,1,2,3,4,5,6,7);
  long long neg=-1-(long long)v;
  guid_t gn=GUID_INIT(neg,neg,neg,0,1,2,3,4,5,6,7);
  uuid_t un=UUID_INIT(neg,neg,neg,0,1,2,3,4,5,6,7);
  const unsigned char *expected[]={g8.b,u8v.b,gs8.b,us8.b,gw.b,uw.b};
  CHECK(uuid_initializer_check(16,(unsigned long long)neg,b)); equal(b,gn.b,16);
  CHECK(uuid_initializer_check(17,(unsigned long long)neg,b)); equal(b,un.b,16);
  for(i=0;i<6;i++) {
   CHECK(uuid_initializer_check(i+6,i<4?v:wide,b)); equal(b,expected[i],16);
  }
  for (i=0;i<2;i++) {
   unsigned j;
   for(j=0;j<11;j++) initializer_calls[j]=0;
#define NEXT(x) initializer_next(x,v)
   if(i) {
    uuid_t repeated=UUID_INIT(NEXT(0),NEXT(1),NEXT(2),NEXT(3),NEXT(4),NEXT(5),NEXT(6),NEXT(7),NEXT(8),NEXT(9),NEXT(10));
    CHECK(uuid_initializer_check(13,v,b)); equal(b,repeated.b,16);
   } else {
    guid_t repeated=GUID_INIT(NEXT(0),NEXT(1),NEXT(2),NEXT(3),NEXT(4),NEXT(5),NEXT(6),NEXT(7),NEXT(8),NEXT(9),NEXT(10));
    CHECK(uuid_initializer_check(12,v,b)); equal(b,repeated.b,16);
   }
#undef NEXT
   for(j=0;j<11;j++) CHECK(initializer_calls[j] == (j==0?4:j<3?2:1));
  }
 }
}

int main(void)
{
	static const unsigned char valid[36] = "01234567-89Ab-CdEf-0123-456789abcdef";
	unsigned char text[37], a[48], b[48];
	unsigned i, value, kind;
	CHECK(uuid_header_check());
	initializer_cases();
	/* No NUL requirement, and all 256 bytes at every validated position. */
	copy(text, valid, 36);
	text[36] = 0xff;
	one(text, 37);
	for (i = 0; i < 36; i++)
		for (value = 0; value < 256; value++) {
			copy(text, valid, 36);
			text[i] = value;
			one(text, 36);
		}
	/* Every early-return position sits immediately before an unreadable page. */
	{
		unsigned char *pages = mmap(0, 8192, 3, 0x22, -1, 0);
		CHECK(pages != (void *)-1);
		CHECK(mprotect(pages+4096, 4096, 0) == 0);
		for (i = 0; i < 36; i++) {
			unsigned char *p = pages + 4096 - i - 1;
			copy(p, valid, i);
			p[i] = 0;
			CHECK(!c_uuid_is_valid((char *)p) && !uuid_is_valid((char *)p));
			CHECK(c_uuid_parse((char *)p, 0) == -22);
			CHECK(uuid_parse((char *)p, 0) == -22);
			CHECK(c_guid_parse((char *)p, 0) == -22);
			CHECK(guid_parse((char *)p, 0) == -22);
		}
		copy(pages+4096-36, valid, 36);
		CHECK(uuid_is_valid((char *)pages+4096-36));
		CHECK(c_uuid_is_valid((char *)pages+4096-36));
		CHECK(munmap(pages, 8192) == 0);
	}
	for (seed = 0; seed < 256; seed++)
		for (kind = 0; kind < 4; kind++) {
			fill(a, sizeof(a), 0xa5);
			fill(b, sizeof(b), 0xa5);
			copy(before, a+16, 16);
			calls = 0; expected = a+16;
			switch (kind) {
			case 0: c_generate_random_uuid(a+16); break;
			case 1: c_generate_random_guid(a+16); break;
			case 2: c_guid_gen((guid_t *)(a+16)); break;
			default: c_uuid_gen((uuid_t *)(a+16)); break;
			}
			CHECK(calls == 1);
			calls = 0; expected = b+16;
			switch (kind) {
			case 0: generate_random_uuid(b+16); break;
			case 1: generate_random_guid(b+16); break;
			case 2: guid_gen((guid_t *)(b+16)); break;
			default: uuid_gen((uuid_t *)(b+16)); break;
			}
			CHECK(calls == 1);
			equal(a, b, sizeof(a));
		}
	for (i = 0; i < 16; i++) {
		CHECK(guid_null.b[i] == 0 && uuid_null.b[i] == 0);
		CHECK(uuid_index[i] == i);
	}
	puts("UUID_DIFFERENTIAL_OK positions=9216 overlap_offsets=61 random_calls=1024 guard_prefixes=36");
	return 0;
}
