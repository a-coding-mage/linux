SELinux policy database correction 02
====================================

This addendum accompanies the separate correction to the source-only policydb
checkpoint.  The original v1 archive and manifest remain intact; apply the
revision-02 overlay after v1, or consume the corrected complete source archive.

Rust allocation validity correction
-----------------------------------

Two filename-transition readers allocate filename_trans_datum with kmalloc.
Their stypes bitmap has not yet been initialized when it is passed to
ebitmap_init/read.  C accepts an address of that uninitialized storage, but
creating an intermediate Rust mutable reference to
an uninitialized ebitmap violates Rust reference validity requirements.

The corrected calls use ptr::addr_of_mut!((*datum).stypes) directly at
filename_trans_read_helper_compat's ebitmap_init boundary and
filename_trans_read_helper's ebitmap_read boundary.  No intermediate reference
is created.  The same allocation, initializer/parser call, error propagation,
ownership transfer, input bytes and cleanup behavior are retained.  No unrelated
policy logic is changed.

Rust's addr_of_mut documentation explains the validity distinction:
https://doc.rust-lang.org/std/ptr/macro.addr_of_mut.html

Original C functional-policy test provenance
-------------------------------------------

The v1 local-tree search reported only generic LSM ABI tests and no direct
maintained policydb parser suite.  That local-tree statement does not mean that
upstream maintained SELinux functional tests are unavailable.  The lead's
verified upstream test pin is SELinuxProject/selinux-testsuite commit
1186eaf201cf72926c490431cf81a10ef516e5bf:

* https://github.com/SELinuxProject/selinux-testsuite/tree/1186eaf201cf72926c490431cf81a10ef516e5bf
* tests/bounds/thread.c with bounds/test checks bounded and unbounded dynamic
  transitions and expected file authorization:
  https://github.com/SELinuxProject/selinux-testsuite/blob/1186eaf201cf72926c490431cf81a10ef516e5bf/tests/bounds/thread.c
* tests/glblub/default_range.c checks resulting security_compute_create
  contexts, including loaded-policy default/MLS behavior:
  https://github.com/SELinuxProject/selinux-testsuite/blob/1186eaf201cf72926c490431cf81a10ef516e5bf/tests/glblub/default_range.c
* tests/ioctl/test_ioctl.c exercises file ioctl permission paths:
  https://github.com/SELinuxProject/selinux-testsuite/blob/1186eaf201cf72926c490431cf81a10ef516e5bf/tests/ioctl/test_ioctl.c

These original C helpers and the accompanying policies add maintained
functional-policy test authority beyond local generic LSM syscall smoke tests.
The upstream tests/Makefile exports -g -O0 -Werror -Wall -Wextra
-Wno-unused-parameter -D_GNU_SOURCE, builds the C helpers and uses a Perl runner.

Future execution must use those original maintained C sources and their
maintained Makefiles/flags, adapting build/link paths only.  No translated test
replacement is an acceptance authority.  These functional-policy tests do not
by themselves close direct malformed-binary parser, allocation-failure unwind,
serialization/cursor parity or every supported policy-format version gap.
Retained-C differential coverage in those areas remains required.

Validation state
----------------

This correction is source-only.  No compiler, bindgen, make, test, policy action,
VM, configuration or security-setting operation was performed.  Configured ABI,
build/link and runtime behavior remain unverified.  The independent corrective
review outcome is recorded separately and must be considered before publication.
