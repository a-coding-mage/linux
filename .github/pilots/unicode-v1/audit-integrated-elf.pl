#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Finite adaptation of audit-elf.pl, audit-common-native.sh,
# verify-iov-registration.pl, verify-chacha-member-2b54-v2.pl,
# collect-ownership.sh, verify-combined-owner.pl and verify-final-pointers.pl.
# Reuses the existing Elf64 and OverflowPolicy modules without copying them.
# Observes finished non-LTO x86_64 files only; does not execute saved commands.
use strict; use warnings; use FindBin; use lib $FindBin::Bin;
use Elf64; use OverflowPolicy; use Digest::SHA qw(sha256_hex);
use Text::ParseWords qw(shellwords); use Fcntl qw(O_WRONLY O_CREAT O_EXCL);
@ARGV==2 or die "usage: audit-integrated-elf.pl c|rust /work/O\n";
my ($provider,$out)=@ARGV;
$provider =~ /\A(?:c|rust)\z/ && $out eq '/work/O' or die "unexpected member/output\n";
my $src='/src'; my $evidence='/work/evidence';
sub text { open my $f,'<:raw',$_[0] or die "$_[0]: $!\n"; local $/; return <$f>//'' }
sub digest { open my $f,'<:raw',$_[0] or die "$_[0]: $!\n"; return Digest::SHA->new(256)->addfile($f)->hexdigest }
sub command { open my $f,'-|',@_ or die "cannot run $_[0]: $!\n"; local $/; my $t=<$f>//''; close $f or die "read-only command failed: $_[0]\n"; return $t }
my %report;
for my $role(qw(owners.commands owners.symbols owners.archives owners.sha256 original-tests.commands original-tests.sha256 elf-ownership.txt ownership-summary.txt)) {
 sysopen(my $f,"$evidence/$role",O_WRONLY|O_CREAT|O_EXCL,0644) or die "new report $role: $!\n"; $report{$role}=$f;
}
sub emit { my ($role,@s)=@_; print {$report{$role}} @s or die "write $role failed\n" }
my (%tracked,%hash_role,%obj,%members,%compiler,%selected,%archive_cmd);
sub track {
 my($path,$role)=@_; $role//='owners.sha256'; -f $path && !-l $path or die "missing/nonregular input $path\n";
 $tracked{$path}//=digest($path); $hash_role{$path}{$role}=1; return $path;
}
sub input { text(track(@_)) }
sub object {
 my($path,$role)=@_; track("$out/$path",$role);
 if (!exists $obj{$path}) {
  my $o=Elf64->new("$out/$path");
  my $type=unpack('v',substr($o->{data},16,2));
  $type==($path eq 'vmlinux'?2:1) or die "unexpected ELF object kind $path\n";
  $obj{$path}=$o;
 }
 return $obj{$path};
}
sub safe_path { $_[0] =~ m{\A[A-Za-z0-9_.+-]+(?:/[A-Za-z0-9_.+-]+)*\z} && $_[0] !~ m{(?:^|/)\.\.?(/|$)} }
sub table {
 my($file,$header,$count)=@_; my @lines=split /\n/,input("$FindBin::Bin/$file");
 shift(@lines) eq $header or die "invalid manifest header $file\n";
 @lines==$count or die "invalid manifest row count $file\n";
 return map {my @r=split /\t/,$_,-1; @r==(split /\t/,$header) or die "invalid manifest row $file\n"; \@r} @lines;
}
my @rust=table('integrated-owners.tsv',"owner\tobject\tsource\tarchive\trepresentative_symbol\tRUST_MODFILE",24);
my @c=table('integrated-c-owners.tsv',"selector\tC_object\tC_source\tbuilt_in_archive\tC_representative_symbol",6);
my %switch=(bpf_lpm_trie=>'RUST_BPF_LPM_TRIE',bpf_token=>'RUST_BPF_TOKEN',power_process=>'RUST_POWER_PROCESS',blk_crypto_fallback=>'RUST_BLK_CRYPTO_FALLBACK',selinux_services=>'RUST_SELINUX_SERVICES',selinux_policydb=>'RUST_SELINUX_POLICYDB');
my (%c,%names,%objects);
for my $r(@c) { exists($c{$r->[0]}) and die "duplicate C selector\n"; $c{$r->[0]}=$r; }
join(',',sort keys %c) eq join(',',sort values %switch) or die "C selector set changed\n";
for my $r(@rust) {
 my($name,$path,$source,$archive,$sym,$mod)=@$r;
 !$names{$name}++ && !$objects{$path}++ or die "duplicate owner row\n";
 for($path,$source,$archive) {safe_path($_) or die "invalid owner path\n"}
 $source =~ /\.rs\z/ && $path =~ /\.o\z/ && $archive =~ /built-in\.a\z/ or die "invalid Rust row\n";
 if($provider eq 'c' && $switch{$name}) {
  my $v=$c{$switch{$name}}; $selected{$name}=[$v->[1],$v->[2],$v->[3],$v->[4],'-','C'];
 } else {$selected{$name}=[$path,$source,$archive,$sym,$mod,'Rust']}
}
my $config=input("$out/.config"); my $auto=input("$out/include/config/auto.conf");
my $rustcfg=input("$out/include/generated/rustc_cfg");
for my $key(qw(RUST RUST_OVERFLOW_CHECKS RUST_INIT_MAIN RUST_CRC16 RUST_SCATTERLIST KUNIT KUNIT_DEFAULT_ENABLED KUNIT_AUTORUN_ENABLED CRC_ENABLE_ALL_FOR_KUNIT CRC_KUNIT_TEST CRC_BENCHMARK CRC7 CRC16 CRC_T10DIF CRC32 CRC64 CRC_OPTIMIZATIONS CRC_T10DIF_ARCH CRC32_ARCH CRC64_ARCH TEST_IOV_ITER CRYPTO_LIB_ENABLE_ALL_FOR_KUNIT CRYPTO_LIB_CHACHA20POLY1305_KUNIT_TEST CRYPTO_LIB_CHACHA20POLY1305 CRYPTO_LIB_CHACHA CRYPTO_LIB_POLY1305 CRYPTO_LIB_UTILS)) {
 for($config,$auto) {/^CONFIG_\Q$key\E=y$/m or die "required config $key not y\n"}
 $rustcfg =~ /^--cfg=CONFIG_\Q$key\E(?:="y")?$/m or die "Rust config missing $key\n";
}
$config =~ /^CONFIG_KUNIT_DEFAULT_TIMEOUT=300$/m or die "original KUnit timeout changed\n";
for my $key(qw(KUNIT_ALL_TESTS KUNIT_TEST KUNIT_EXAMPLE_TEST CRYPTO_LIB_BENCHMARK RUST_KUNIT_TESTS RUST_KERNEL_DOCTESTS UNICODE LTO_CLANG LTO_GCC CFI_CLANG)) {
 $config =~ /^# CONFIG_\Q$key\E is not set$/m || ($config !~ /^CONFIG_\Q$key\E=/m && $key =~ /^(?:LTO_|CFI_)/) or die "unexpected config $key\n";
}
my @crypto_tests=$config =~ /^(CONFIG_CRYPTO_LIB_\w+_KUNIT_TEST=[ym])$/mg;
@crypto_tests==1 && $crypto_tests[0] eq 'CONFIG_CRYPTO_LIB_CHACHA20POLY1305_KUNIT_TEST=y' or die "unexpected crypto test selection\n";
for my $key(sort values %switch) {
 if($provider eq 'rust') {for($config,$auto){/^CONFIG_\Q$key\E=y$/m or die "missing selected Rust $key\n"}}
 else {$config =~ /^# CONFIG_\Q$key\E is not set$/m && $auto !~ /^CONFIG_\Q$key\E=/m or die "C control not selected $key\n"}
}
my $final=object('vmlinux');
my %final_symbols;
for my $s($final->symbols) {push @{$final_symbols{$s->{name}}},$s if $s->{section}>0 && $s->{section}<0xff00}
sub final_symbol {
 my($name)=@_;my $defs=$final_symbols{$name}//[];
 @$defs==1 or die "expected one final definition of $name, got ".scalar(@$defs)."\n";
 return $defs->[0];
}
my @undefined=grep {!$_->{section} && $_->{bind} && length($_->{name})} $final->symbols;
@undefined==0 or die "undefined final symbols\n";
sub command_file { my($path)=@_; $path =~ s{([^/]+)$}{.$1.cmd}; return $path }
sub savedcmd {
 my($path,$source,$language,$mod,$original)=@_;
 if(exists $compiler{$path}) {$compiler{$path} eq "$language:$source" or die "conflicting producer $path\n"; return}
 safe_path($path) && safe_path($source) or die "invalid producer path\n";
 my $role=$original?'original-tests.commands':'owners.commands';
 my $hash=$original?'original-tests.sha256':'owners.sha256';
 my $cf=command_file($path); my $body=input("$out/$cf",$hash);
 $body !~ m{/(?:workspace|home|root)/|/tmp/lupos} or die "nonpublic compiler provenance path\n";
 emit($role,"BEGIN $cf\n",$body,"\nEND $cf\n");
 my @lines=$body =~ /^savedcmd_\Q$path\E := ([^\n]+)$/mg;
 @lines==1 or die "missing/duplicate actual command $path\n";
 my @sources=$body =~ /^source_\Q$path\E := ([^\n]+)$/mg;
 @sources==1 && $sources[0] eq "$src/$source" or die "wrong actual source record $path\n";
 my @a=shellwords($lines[0]); @a or die "empty actual command\n";
 my @inputs=grep {$_ eq "$src/$source"} @a; @inputs==1 or die "wrong actual compiler input $path\n";
 if($language eq 'Rust') {
  my @compilers=grep {$a[$_] eq '/usr/bin/rustc'} 0..$#a;
  @compilers==1 && $compilers[0]==2 && $a[0] eq "OBJTREE=$out" && $a[1] =~ m{\ARUST_MODFILE=[A-Za-z0-9_./+-]+\z} or die "selected Rust compiler/environment not proven $path\n";
  my @emits=grep {/\A--emit=obj=/} @a;
  @emits==1 && $emits[0] eq "--emit=obj=$path" or die "wrong Rust emitted object $path\n";
  !grep(/\A--emit=llvm-bc=/,@a) or die "bitcode is not accepted ELF ownership\n";
  grep($_ eq "OBJTREE=$out",@a) or die "wrong Rust output tree\n";
  $mod eq '-' || grep($_ eq "RUST_MODFILE=$mod",@a) or die "wrong Rust module identity $path\n";
  # OverflowPolicy has one legacy command-key spelling. Adapt that key only in
  # memory; the actual command and every flag remain byte-for-byte unchanged.
  OverflowPolicy::check("savedcmd_fs/unicode/utf8-norm.o := $lines[0]\n");
  emit($role,"OVERFLOW_POLICY_PASS object=$path original_saved_command_sha256=",sha256_hex($lines[0])," key_adapter_only=1\n");
 } else {
  $a[0] =~ m{\A(?:/usr/bin/)?clang(?:-19)?\z} or die "actual C/ASM Clang compiler missing $path\n";
  my @o=grep {$a[$_] eq '-o'} 0..$#a;
  @o==1 && defined($a[$o[0]+1]) && $a[$o[0]+1] eq $path or die "wrong actual C/ASM output $path\n";
  grep($_ eq '-c',@a) or die "C/ASM compilation selector missing\n";
  !grep(/\.rs\z/,@a) or die "translated C/ASM producer selected\n";
  if($language eq 'C') {grep($_ eq '-fno-strict-overflow',@a) or die "original C overflow policy missing\n";!grep($_ eq '-fstrict-overflow',@a) or die "conflicting C overflow policy\n"}
  !grep(/DEBUG_CHACHA20POLY1305_SLOW_CHUNK_TEST|^-[DU](?:CRC_KUNIT_|IRQ_TEST_)/,@a) or die "altered original crypto workload\n";
 }
 track("$src/$source",$hash); object($path,$hash);
 $compiler{$path}="$language:$source";
 emit($role,"ACTUAL_COMPILER $path language=$language source=$src/$source saved_command_sha256=",sha256_hex($lines[0]),"\n");
}
sub archive {
 my($path)=@_; return $members{$path} if exists $members{$path};
 track("$out/$path"); my $list=command('llvm-ar','t',"$out/$path");
 my @m=split /\n/,$list; @m or die "empty archive $path\n";
 !grep(m{(?:^|/)\.rust-listing(?:-elf)?/},@m) or die "inspection-only object in native archive\n";
 for(@m) {s{^\Q$out\E/}{};safe_path($_) or die "unexpected archive path $path\n"}
 emit('owners.archives',"BEGIN $path\n",$list,"END $path\n");
 my $cf=command_file($path); my $body=input("$out/$cf");
 $body !~ m{/(?:workspace|home|root)/|/tmp/lupos} or die "nonpublic archive provenance path\n";
 emit('owners.commands',"BEGIN $cf\n",$body,"\nEND $cf\n");
 my @saved=$body =~ /^savedcmd_\Q$path\E := ([^\n]+)$/mg; @saved==1 or die "missing/duplicate archive command $path\n";
 $archive_cmd{$path}=$saved[0]; return $members{$path}=\@m;
}
archive('vmlinux.a');
sub member {
 my($path,$archive)=@_; for my $a($archive,'vmlinux.a') {
  my $m=archive($a); my $n=grep {$_ eq $path} @$m;
  $n==1 or die "missing/duplicate selected archive leaf $a $path\n";
  if ($a ne 'vmlinux.a') {
   (my $directory=$a)=~s{[^/]+$}{}; (my $relative=$path)=~s{^\Q$directory\E}{};
   $archive_cmd{$a} =~ /(?:^|\s)(?:\Q$path\E|\Q$relative\E)(?:\s|$)/ or die "selected leaf absent from actual archive command $a $path\n";
  }
  emit('owners.archives',"UNIQUE_MEMBER $a $path\n");
 }
}
for my $cf(qw(.built-in.a.cmd .built-in-fixup.a.cmd .vmlinux.o.cmd .vmlinux.unstripped.cmd .vmlinux.cmd)) {
 my $body=input("$out/$cf"); $body !~ m{/(?:workspace|home|root)/|/tmp/lupos} or die "nonpublic link provenance path\n"; emit('owners.commands',"BEGIN $cf\n",$body,"\nEND $cf\n");
 $cf ne '.vmlinux.o.cmd' || $body =~ /--whole-archive\s+vmlinux\.a/ or die "final whole archive link missing\n";
}
my %owner_symbols;
sub owner {
 my($path,$symbol)=@_; my $o=object($path); my $s=$o->one($symbol); my $f=final_symbol($symbol);
 $s->{type}==$f->{type} && ($s->{type}==1 || $s->{type}==2) && $s->{size}>0 && $s->{size}==$f->{size} or die "wrong owner symbol type/size $path $symbol\n";
 exists($owner_symbols{$symbol}) && $owner_symbols{$symbol} ne $path and die "conflicting claimed owner $symbol\n";
 $owner_symbols{$symbol}=$path;
 emit('owners.symbols',sprintf("OWNER %s %s object_type=%d binding=%d bytes=%d final=0x%016x final_binding=%d\n",$path,$symbol,$s->{type},$s->{bind},$s->{size},$f->{value},$f->{bind}));
}
my %imports_done;
sub imports {
 my($path)=@_;return if $imports_done{$path}++;my $o=object($path);my $n=0;
 for my $s($o->symbols) {
  next if $s->{section} || !$s->{bind} || !length($s->{name});
  my $f=final_symbol($s->{name}); $n++;
  emit('owners.symbols',"IMPORT_RESOLVED $path $s->{name} final_binding=$f->{bind}\n");
 }
 emit('owners.symbols',"IMPORT_COUNT $path $n\n");
}
for my $name(sort keys %selected) {
 my($path,$source,$a,$symbol,$mod,$language)=@{$selected{$name}};
 savedcmd($path,$source,$language,$mod,0); member($path,$a);owner($path,$symbol); imports($path);
 emit('owners.symbols',"SELECTED_ROW $name language=$language object=$path\n");
}
# Retained original C bridge and consumer; the regression still targets Rust SG.
savedcmd('net/ipv6/ah6_helpers.o','net/ipv6/ah6_helpers.c','C','-',0);member('net/ipv6/ah6_helpers.o','net/ipv6/built-in.a');imports('net/ipv6/ah6_helpers.o');
savedcmd('lib/scatterlist_helpers.o','lib/scatterlist_helpers.c','C','-',0);member('lib/scatterlist_helpers.o','lib/built-in.a');imports('lib/scatterlist_helpers.o');
savedcmd('lib/crypto/chacha20poly1305.o','lib/crypto/chacha20poly1305.c','C','-',0);member('lib/crypto/chacha20poly1305.o','lib/crypto/built-in.a');
savedcmd('lib/crc/crc16.o','lib/crc/crc16.rs','Rust','lib/crc/crc16',0);member('lib/crc/crc16.o','lib/crc/built-in.a');owner('lib/crc/crc16.o','crc16');imports('lib/crc/crc16.o');
!-e "$out/lib/scatterlist.o" or die "opposing C scatterlist object present\n";
!grep {$_ eq 'lib/scatterlist.o'} @{$members{'vmlinux.a'}} or die "opposing scatterlist archive leaf\n";
# Discover only actual native Rust lib objects already in the final archive.
# Host tools, bindings, .rmeta/.rlib and unlinked stale outputs cannot count.
my $generic_count=0;
for my $path(@{$members{'vmlinux.a'}}) {
 next unless $path =~ m{^lib/.*\.o$};
 my $cf=command_file($path); next unless -f "$out/$cf";
 my $body=text("$out/$cf"); my @s=$body =~ /^source_\Q$path\E := \/src\/(\S+\.rs)$/mg;
 next unless @s; @s==1 or die "ambiguous generic Rust source\n";
 savedcmd($path,$s[0],'Rust','-',0);
 # A parent built-in archive flattens descendants, but its saved command
 # names their archives. Select the immediate archive that actually names
 # this primitive leaf, preserving both command and final-link proofs.
 (my $directory=$path)=~s{[^/]+$}{};
 (my $leaf=$path)=~s{^\Q$directory\E}{};
 my @owning_archives;
 for my $candidate("${directory}built-in.a","${directory}lib.a") {
  next unless -f "$out/$candidate";
  my $list=archive($candidate);
  next unless (grep {$_ eq $path} @$list)==1;
  next unless $archive_cmd{$candidate} =~ /(?:^|\s)(?:\Q$path\E|\Q$leaf\E)(?:\s|$)/;
  push @owning_archives,$candidate;
 }
 @owning_archives==1 or die "missing/ambiguous immediate Rust archive $path\n";
 member($path,$owning_archives[0]);imports($path);
 my $o=object($path);my @defs=grep {$_->{bind}==1 && ($_->{type}==1||$_->{type}==2) && $_->{section}>0 && $_->{section}<0xff00 && $_->{size}>0 && $_->{name}!~/^(?:__pfx_|__export_symbol_)/} $o->symbols;
 @defs or die "no real generic Rust owner $path\n";
 for my $s(@defs) {owner($path,$s->{name})}
 $generic_count++;
}
$generic_count>=2 or die "missing retained Rust libraries\n";
my @test_rows=(
 ['crc','lib/crc/tests/crc_kunit.o','lib/crc/tests/crc_kunit.c','lib/crc/tests/built-in.a','89612b6fa9b63a63a0d8ae8826812c6210dacbe9a914bd06d3dbc326f3fc41c9'],
 ['chacha20poly1305','lib/crypto/tests/chacha20poly1305_kunit.o','lib/crypto/tests/chacha20poly1305_kunit.c','lib/crypto/tests/built-in.a','2695d1049e5937b18cf2fb805a9a560ae581c7f40f0ac6b16ec3fee44197903b'],
 ['iov_iter','lib/tests/kunit_iov_iter.o','lib/tests/kunit_iov_iter.c','lib/tests/built-in.a','a731cda5f898cdc5aeb82444ec4c3b3683a1a3074e7beec9810b23ae169f523a']);
for my $file(qw(include/kunit/test.h include/kunit/run-in-irq-context.h lib/crypto/tests/test-utils.h scripts/Makefile.build lib/crypto/Makefile lib/crypto/Kconfig lib/Makefile lib/Kconfig lib/crc/Makefile lib/crc/Kconfig)) {track("$src/$file",'original-tests.sha256')}
for my $r(@test_rows) {
 my($suite,$path,$source,$a,$sha)=@$r;
 digest("$src/$source") eq $sha or die "original C test source changed $suite\n";
 savedcmd($path,$source,'C','-',1);member($path,$a);
 for my $file('Makefile','Kconfig') {(my $dir=$source)=~s{[^/]+$}{};track("$src/$dir$file",'original-tests.sha256') if -f "$src/$dir$file"}
}
digest("$src/lib/crypto/chacha20poly1305.c") eq '02f7d1ac78ffdf019588d135b5f89160ec53d7c8e97cbfc18d626124240fc71d' or die "original C crypto consumer changed\n";
# Preserve selected sibling CRC C and assembly producers and their native leaf
# ownership; they remain the original implementations in both pair members.
my @crc_c=(['crc7','crc7_be'],['crc-t10dif-main','crc_t10dif_update'],['crc32-main','crc32_le'],['crc64-main','crc64_be']);
for my $r(@crc_c) {my($stem,$symbol)=@$r;savedcmd("lib/crc/$stem.o","lib/crc/$stem.c",'C','-',0);member("lib/crc/$stem.o",'lib/crc/built-in.a');owner("lib/crc/$stem.o",$symbol)}
for my $stem(qw(crc16-msb-pclmul crc32-pclmul crc32c-3way crc64-pclmul)) {
 my $path="lib/crc/x86/$stem.o";savedcmd($path,"lib/crc/x86/$stem.S",'ASM','-',0);member($path,'lib/crc/built-in.a');
 for my $s(object($path)->symbols) {next unless $s->{bind} && $s->{section}>0 && $s->{section}<0xff00 && $s->{type}==2 && $s->{size}>0 && $s->{name}!~/^__pfx_/;owner($path,$s->{name})}
}
# The original Elf64 auditor's relocated-data proof, generalized to three
# original C suites. Every pointer and every non-pointer byte is preserved.
sub target_name {
 my($o,$r,$type)=@_;my $s=$r->{symbol};
 $s->{section}>0 && $s->{section}<0xff00 or die "unexpected external table target\n";
 my $value=$s->{value}+$r->{addend};
 my @n=grep {(!defined($type)||$_->{type}==$type) && ($_->{type}==1||$_->{type}==2) && $_->{section}==$s->{section} && $_->{value}==$value && $_->{size}>0} $o->symbols;
 @n==1 or die "ambiguous named table relocation\n";return $n[0]{name};
}
sub relocated_data {
 my($o,$name)=@_;my $s=$o->one($name);my $f=final_symbol($name);
 $s->{type}==1 && $f->{type}==1 && $s->{size}==$f->{size} or die "table type/size changed $name\n";
 my $expected=$o->symbol_bytes($s);my $actual=$final->symbol_bytes($f);my @targets;
 for my $r($o->rels_in($s)) {
  $r->{type}==1 or die "unsupported table relocation $name\n";
  my $at=$r->{offset}-$s->{value};$at>=0 && $at+8<=length($actual) or die "table pointer out of bounds\n";
  my $t=$r->{symbol};$t->{section}>0 && $t->{section}<0xff00 or die "external table relocation\n";
  my $v=$t->{value}+$r->{addend};my @named=grep {($_->{type}==1||$_->{type}==2) && $_->{section}==$t->{section} && $_->{value}==$v && $_->{size}>0} $o->symbols;
  my $ptr=unpack('Q<',substr($actual,$at,8));
  if(@named==1) {my $dest=final_symbol($named[0]{name});$ptr==$dest->{value} or die "wrong final table pointer $name -> $named[0]{name}\n";push @targets,$named[0]{name}}
  elsif(!@named) {my $str=Elf64::cstr($o->section_bytes($t->{section}),$v);$final->string_at($ptr) eq $str or die "wrong final table string $name\n";push @targets,"string:$str"}
  else {die "ambiguous table target $name\n"}
  substr($expected,$at,8)=substr($actual,$at,8);
 }
 $expected eq $actual or die "table non-pointer bytes changed $name\n";
 emit('elf-ownership.txt',"FINAL_DATA $name ",join(',',@targets),"\n");return @targets;
}
my %case_count=(crc=>16,chacha20poly1305=>1,iov_iter=>17);
my %case_file=(crc=>'crc-cases.txt',chacha20poly1305=>'chacha-cases.txt',iov_iter=>'iov-cases.txt');
my %case_names;
for my $suite(sort keys %case_file) {
 my @n=split /\n/,input("$FindBin::Bin/$case_file{$suite}",'original-tests.sha256');
 @n==$case_count{$suite} or die "original count changed $suite\n";$case_names{$suite}=\@n;
}
sub registration {
 my($path,$suite,$array,$name,$callbacks)=@_;my $o=object($path);my @cases=@{$case_names{$name}};my $n=@cases;
 my $s=$o->one($array);my $f=final_symbol($array);
 $s->{size}>0 && $s->{size}%($n+1)==0 && $s->{size}==$f->{size} or die "case table shape changed $name\n";
 my $stride=$s->{size}/($n+1);$stride>=24 && $stride%8==0 or die "invalid native case stride\n";
 my $bytes=$final->symbol_bytes($f);
 for my $i(0..$#cases) {
  my $case=$cases[$i];my $c=$o->one($case);my $fc=final_symbol($case);
  $c->{type}==2 && $fc->{type}==2 && $c->{size}>0 && $c->{size}==$fc->{size} or die "original C case body changed $case\n";
  my @r=grep {$_->{offset}==$s->{value}+$i*$stride} $o->rels_in($s);
  @r==1 && $r[0]{type}==1 && target_name($o,$r[0],2) eq $case or die "incorrect original case relocation $case\n";
  unpack('Q<',substr($bytes,$i*$stride,8))==$fc->{value} or die "wrong final original case pointer\n";
  $final->string_at(unpack('Q<',substr($bytes,$i*$stride+8,8))) eq $case or die "wrong final original case name\n";
  unpack('Q<',substr($bytes,$i*$stride+16,8))==0 or die "unexpected parameterized original case\n";
  emit('elf-ownership.txt',"REGISTRATION $name ",($i+1)," $case stride=$stride final_original_C_body=1\n");
 }
 substr($bytes,$n*$stride,$stride) eq "\0"x$stride or die "extra case/nonzero terminator\n";
 my @actual=relocated_data($o,$array);
 my @functions=grep {$_!~/^string:/} @actual;
 my @names=map {s/^string://r} grep {/^string:/} @actual;
 join(',',@functions) eq join(',',@cases) && join(',',@names) eq join(',',@cases) or die "original function/name table order changed\n";
 my @s=relocated_data($o,$suite);my $os=$o->one($suite);my $fs=final_symbol($suite);
 substr($final->symbol_bytes($fs),0,length($name)+1) eq "$name\0" or die "suite identity changed\n";
 for my $target($array,@$callbacks) {my $count=grep {$_ eq $target} @s;$count==1 or die "missing/duplicate suite pointer $target\n"}
 my @sections=grep {$_->{name} eq '.kunit_test_suites'} @{$o->{sections}};@sections==1 or die "missing object suite section\n";
 my @r=grep {$_->{section}==$sections[0]{index} && $_->{type}==1 && eval {target_name($o,$_,1) eq $suite}} @{$o->{rels}};
 @r==1 or die "missing/duplicate original object suite registration\n";
 my $start=final_symbol('__kunit_suites_start')->{value};my $end=final_symbol('__kunit_suites_end')->{value};
 $end>$start && ($end-$start)%8==0 or die "invalid KUnit registry bounds\n";
 my @p=unpack('Q<*',$final->at($start,$end-$start));my $nreg=grep {$_==$fs->{value}} @p;
 $nreg==1 or die "missing/duplicate final suite registration\n";
 emit('elf-ownership.txt',"FINAL_SUITE_REGISTRY $name unique=1 case_count=$n\n");
}
registration('lib/crc/tests/crc_kunit.o','crc_test_suite','crc_test_cases','crc',[qw(crc_suite_init crc_suite_exit)]);
registration('lib/crypto/tests/chacha20poly1305_kunit.o','chacha20poly1305_test_suite','chacha20poly1305_test_cases','chacha20poly1305',[]);
registration('lib/tests/kunit_iov_iter.o','iov_kunit_suite','iov_kunit_cases','iov_iter',[]);
my %edges;my %all_callers;
sub direct_calls {
 my($path,$wanted)=@_;my $o=object($path);my %count;
 for my $r(@{$o->{rels}}) {
  my $target=$r->{symbol}{name};next unless $wanted->{$target};
  next unless $o->{sections}[$r->{section}]{flags}&4;
  ($r->{type}==2 || $r->{type}==4) && $r->{addend}==-4 or die "unexpected wanted code relocation $path $target\n";
  my @c=grep {$_->{type}==2 && $_->{section}==$r->{section} && $_->{name}!~/^__pfx_/ && $r->{offset}>$_->{value} && $r->{offset}+4<=$_->{value}+$_->{size}} $o->symbols;
  @c==1 or die "ambiguous actual caller $path $target\n";my $c=$c[0];my $fc=final_symbol($c->{name});my $ft=final_symbol($target);
  $fc->{type}==2 && $ft->{type}==2 && $fc->{size}==$c->{size} or die "final call symbol shape changed\n";
  my $opcode=substr($o->section_bytes($r->{section}),$r->{offset}-1,1);
  $opcode eq "\xe8" || $opcode eq "\xe9" or die "wanted relocation not direct call/jump\n";
  my $field=$fc->{value}+$r->{offset}-$c->{value};
  $final->at($field-1,1) eq $opcode or die "final direct opcode changed\n";
  unpack('V',$final->at($field,4))==(($ft->{value}-$field-4)&0xffffffff) or die "actual final call target mismatch\n";
  $edges{$c->{name}}{$target}=1;$count{$target}++;$all_callers{$path}{$target}{$c->{name}}++;
  emit('elf-ownership.txt',sprintf("FINAL_CALL %s %s -> %s field=0x%016x target=0x%016x\n",$path,$c->{name},$target,$field,$ft->{value}));
 }
 for my $t(sort keys %$wanted) {$count{$t} or die "no original direct call $path -> $t\n"}
}
direct_calls('lib/crc/tests/crc_kunit.o',{map {$_=>1} qw(crc16 crc7_be crc_t10dif_update crc32_le crc32_be crc32c crc64_be crc64_nvme)});
direct_calls('lib/tests/kunit_iov_iter.o',{map {$_=>1} qw(extract_iter_to_sg sg_copy_to_buffer sg_init_table)});
my @crypto=qw(chacha20poly1305_encrypt_sg_inplace chacha20poly1305_decrypt_sg_inplace);
my @sg=qw(sg_nents sg_miter_start sg_miter_next sg_miter_stop sg_copy_buffer);
direct_calls('lib/crypto/tests/chacha20poly1305_kunit.o',{map {$_=>1} (@crypto,'sg_init_one')});
direct_calls('lib/crypto/chacha20poly1305.o',{map {$_=>1} @sg});
for my $sym(@sg,qw(sg_init_one extract_iter_to_sg sg_copy_to_buffer sg_init_table)) {owner('lib/scatterlist_rust.o',$sym)}
for my $sym(@crypto) {owner('lib/crypto/chacha20poly1305.o',$sym)}
for my $sym(qw(crc32_be crc32c)) {owner('lib/crc/crc32-main.o',$sym)}
owner('lib/crc/crc64-main.o','crc64_nvme');
# Require the real Rust init/main.o relocation, discover its mangled owner, and
# check the linked call bytes. The former C kernel_init_freeable is not used.
track("$src/init/main_freeable.rs");
direct_calls('init/main.o',{'kunit_run_all_tests'=>1});
my @init_callers=sort keys %{$all_callers{'init/main.o'}{'kunit_run_all_tests'}};
@init_callers==1 && $init_callers[0] =~ /\A_R/ && $init_callers[0] =~ /kernel_init_freeable/ or die "expected unique actual Rust init freeable caller\n";
my $start=object('init/main.o')->one('start_kernel');my $final_start=final_symbol('start_kernel');
$start->{type}==2 && $final_start->{type}==2 && $start->{size}>0 && $start->{size}==$final_start->{size} or die "real start_kernel not unique/function\n";
emit('elf-ownership.txt',"REAL_START_KERNEL owner=init/main.o source=init/main.rs final_definitions=1\nACTUAL_RUST_INIT_KUNIT_CALL caller=$init_callers[0] target=kunit_run_all_tests\n");
# Variant data retains every original C wrapper, including CRC16; compare all
# scalar bytes and the linked wrapper function pointers, not symbol names alone.
my $crc=object('lib/crc/tests/crc_kunit.o');
for my $v(qw(crc7_be crc16 crc_t10dif crc32_le crc32_be crc32c crc64_be crc64_nvme)) {
 my @t=relocated_data($crc,"crc_variant_$v");my $n=grep {$_ eq "${v}_wrapper"} @t;
 $n==1 or die "wrong original variant wrapper pointer $v\n";
}
# Reuse the historical same-section instruction proof. Object disassembly only
# locates actual call/jump instructions; both source and final bytes are checked.
sub internal_calls {
 my($path)=@_;my $o=object($path);my $dis=command('objdump','-d','-w','--no-show-raw-insn',"$out/$path");my $caller;
 for my $line(split /\n/,$dis) {
  if($line =~ /^[0-9a-f]+ <([^>]+)>:$/) {$caller=$1;next}
  next unless defined $caller && $caller!~/^__pfx_/;
  next unless $line =~ /^\s*([0-9a-f]+):\s+(?:callq?|jmpq?)\s+([0-9a-f]+)\s+<([^>]+)>/;
  my($pc,$address,$target)=(hex($1),hex($2),$3);next if $target =~ /[+-]0x/;
  my @c=grep {$_->{type}==2 && $_->{section}>0 && $_->{name} eq $caller} $o->symbols;
  my @t=grep {$_->{type}==2 && $_->{section}>0 && $_->{name} eq $target} $o->symbols;
  next unless @c==1 && @t==1 && $c[0]{section}==$t[0]{section} && $t[0]{value}==$address;
  # A relocation placeholder disassembles as an apparent local target; it is
  # not a same-section resolved call and was handled through relocation above.
  next if grep {$_->{section}==$c[0]{section} && $_->{offset}==$pc+1} @{$o->{rels}};
  my $code=substr($o->section_bytes($c[0]{section}),$pc,5);
  next unless substr($code,0,1) eq "\xe8" || substr($code,0,1) eq "\xe9";
  $pc+5+unpack('l<',substr($code,1,4))==$address or die "source local direct-call mismatch\n";
  my $fc=final_symbol($caller);my $ft=final_symbol($target);my $where=$fc->{value}+$pc-$c[0]{value};
  my $actual=$final->at($where,5);substr($actual,0,1) eq substr($code,0,1) or die "final local opcode changed\n";
  unpack('V',substr($actual,1,4))==(($ft->{value}-$where-5)&0xffffffff) or die "final local call changed\n";
  $edges{$caller}{$target}=1;emit('elf-ownership.txt',"FINAL_INTERNAL_CALL $path $caller -> $target\n");
 }
}
internal_calls('lib/crypto/chacha20poly1305.o');internal_calls('lib/crc/tests/crc_kunit.o');
sub path_to {
 my($from,$to,$seen)=@_;return [$from] if $from eq $to;return if $seen->{$from}++;
 for my $next(sort keys %{$edges{$from}//{}}) {my $p=path_to($next,$to,{%$seen});return [$from,@$p] if $p}return;
}
for my $entry(@crypto) {
 my $p=path_to('test_chacha20poly1305',$entry,{});$p or die "original case does not reach C crypto entry\n";
 emit('elf-ownership.txt',"STATIC_PATH ",join(' -> ',@$p),"\n");
 for my $target(@sg) {my $p=path_to($entry,$target,{});$p or die "unproven C-crypto to Rust-SG path $entry -> $target\n";emit('elf-ownership.txt',"STATIC_PATH ",join(' -> ',@$p),"\n")}
}
# Supplemental CRC16 dispatch proof retained from verify-final-pointers.pl.
# Fail closed if compiler instruction layout differs; do not silently drop it.
sub immediate {
 my($o,$caller,$target,$opcode)=@_;my $s=$o->one($caller);
 my @r=grep {$_->{type}==11 && eval {target_name($o,$_,undef) eq $target}} $o->rels_in($s);
 @r==1 or die "missing original 32S data pointer $caller -> $target\n";my $r=$r[0];
 my $pattern=pack('H*',$opcode);my $before=substr($o->section_bytes($s->{section}),$r->{offset}-length($pattern),length($pattern));
 $before eq $pattern or die "unexpected original pointer opcode\n";
 my $fc=final_symbol($caller);my $ft=final_symbol($target);my $field=$fc->{value}+$r->{offset}-$s->{value};
 $final->at($field-length($pattern),length($pattern)) eq $pattern or die "final pointer opcode changed\n";
 my $value=unpack('Q<',pack('q<',unpack('l<',$final->at($field,4))));
 $value==$ft->{value} or die "final immediate pointer target mismatch\n";
 emit('elf-ownership.txt',"FINAL_32S_POINTER $caller -> $target\n");
}
immediate($crc,'crc16_test','crc_variant_crc16','48c7c6');
$edges{'crc16_test'}{'crc_test'} or die "original CRC16 case-to-helper jump not proven\n";
for my $row(['crc_test','4d8b5e10'],['crc_irq_test_func','4c8b5810']) {
 my($name,$hex)=@$row;my $s=$crc->one($name);my $f=final_symbol($name);my $bytes=$crc->symbol_bytes($s);my $pattern=pack('H*',$hex);my $at=index($bytes,$pattern);
 $at>=0 && index($bytes,$pattern,$at+1)==-1 or die "unique original variant.func load missing $name\n";
 $final->at($f->{value}+$at,length($pattern)) eq $pattern or die "final variant.func load changed\n";
 emit('elf-ownership.txt',"FINAL_VARIANT_FUNC_LOAD $name offset=16 bytes=$hex\n");
}
# Retain native CRC16 export ownership from the original combined observer.
my $crc_provider=object('lib/crc/crc16.o');
my @export_sections=grep {$_->{name} eq '.export_symbol'} @{$crc_provider->{sections}};
@export_sections==1 or die "native CRC16 export section missing\n";
my @export_rels=grep {$_->{section}==$export_sections[0]{index} && $_->{type}==1 && $_->{symbol}{name} eq 'crc16' && $_->{addend}==0} @{$crc_provider->{rels}};
@export_rels==1 or die "native CRC16 export address relocation not unique\n";
my $export_label=$crc_provider->one('__export_symbol_crc16');
$export_rels[0]{offset}-$export_label->{value}==8 or die "native CRC16 export shape changed\n";
substr($crc_provider->section_bytes($export_sections[0]{index}),$export_label->{value},8) eq "\0"x8 or die "CRC16 export license/namespace changed\n";
emit('elf-ownership.txt',"NATIVE_EXPORT crc16 address_relocation=unique license=empty namespace=empty\n");
# Check unique input definitions in the real final-link archive, in addition to
# unique final symbols and exact selected-member presence. This prevents another
# same-named C leaf from satisfying a Rust claim (and vice versa).
my $nm=command('llvm-nm','--print-file-name','--defined-only','--format=posix',"$out/vmlinux.a");
my %definitions;
for my $line(split /\n/,$nm) {
 next unless $line =~ /\A(.+):\s+(\S+)\s+([A-Za-z?])(?:\s|\z)/;
 my($where,$sym,$type)=($1,$2,$3);next unless exists $owner_symbols{$sym};
 push @{$definitions{$sym}},[$where,$type,$line];
}
for my $sym(sort keys %owner_symbols) {
 my $path=$owner_symbols{$sym};my $d=$definitions{$sym}//[];
 @$d==1 or die "archive owner not unique $sym\n";
 # LLVM versions use archive(member), archive[member], or archive:member.
 my $where=$d->[0][0];
 $where =~ m{(?:[(/:\[]|\A)(?:\Q$out\E/)?\Q$path\E[)\]]?\z} or die "wrong archive defining member $sym\n";
 emit('owners.symbols',"ARCHIVE_DEFINITION $d->[0][2]\n");
}
# Keep hashes and proofs tied to unchanged inputs over the entire read-only
# observation. The outer runner separately checks complete O inventories.
my $source_sha=command('git','-C',$src,'rev-parse','HEAD');chomp $source_sha;
$source_sha =~ /\A[0-9a-f]{40}\z/ or die "invalid source identity\n";
command('git','-C',$src,'status','--porcelain=v1','--untracked-files=no') eq '' or die "source tracked files changed\n";
for my $path(sort keys %tracked) {
 digest($path) eq $tracked{$path} or die "input changed during ownership observation $path\n";
 for my $role(sort keys %{$hash_role{$path}}) {emit($role,"$tracked{$path}  $path\n")}
}
my $rust_count=$provider eq 'rust'?24:18;my $c_count=24-$rust_count;
emit('elf-ownership.txt',"INTEGRATED_ELF_OWNERSHIP_PASS manifest_owners=24 original_C_suites=3 original_C_cases=34 actual_init_kunit_call=1\n");
emit('ownership-summary.txt',"provider=$provider\nsource_sha=$source_sha\nmanifest_owners=24\nselected_manifest_Rust_owners=$rust_count\nselected_manifest_C_owners=$c_count\ngeneric_Rust_libraries=$generic_count\noriginal_C_suites=3\noriginal_C_cases=34\nactual_init_kunit_call=pass\ninput_hashes_after=unchanged\nverdict=PASS\n");
for my $role(sort keys %report) {close $report{$role} or die "close report $role failed\n"}
require JSON::PP;
my $receipt={schema=>1,provider=>$provider,source_sha=>$source_sha,owners=>24,original_C_suites=>3,original_C_cases=>34,actual_init_kunit_call=>JSON::PP::true(),verdict=>'PASS'};
sysopen(my $rf,"$evidence/audit-integrated.receipt",O_WRONLY|O_CREAT|O_EXCL,0644) or die "new audit receipt: $!\n";
print {$rf} JSON::PP->new->canonical->encode($receipt),"\n" or die "write receipt failed\n";
close $rf or die "close receipt failed\n";
print "INTEGRATED_OWNER_AUDIT_PASS provider=$provider owners=24 original_C_cases=34\n";
