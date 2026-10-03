package CompareEvidence;
# SPDX-License-Identifier: GPL-2.0-only
use OverflowPolicy (); use strict; use warnings; use Evidence; use Digest::SHA qw(sha256_hex); use Encode qw(encode decode FB_CROAK); use Text::ParseWords qw(shellwords);
use Time::Local qw(timegm);
my $IMAGE='docker.io/library/debian:trixie-slim@sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c';
sub canonical_json {
 my($text)=@_; my $v=$Evidence::JSON->decode($text);
 die "noncanonical/duplicate protocol JSON\n" unless $Evidence::JSON->encode($v) eq $text; return $v;
}
sub runner_line {
 my($line)=@_;$line=~s/\r$//;
 die "NUL within runner text\n" if index($line,"\0")>=0;
 # A downloaded runner segment may start with one UTF-8 BOM before its
 # timestamp. This never normalizes protocol text or decoded payload bytes.
 $line=~s/^\xEF\xBB\xBF(?=\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:\.\d+)?Z )//;
 die "misplaced transport BOM\n" if $line=~/\xEF\xBB\xBF|\x{FEFF}/;
 if($line=~/^(\d{4})-(\d\d)-(\d\d)T(\d\d):(\d\d):(\d\d)(?:\.\d+)?Z /){
  my($year,$month,$day,$hour,$minute,$second)=($1,$2,$3,$4,$5,$6);
  die "malformed runner timestamp\n" unless $month>=1 && $month<=12 && $day>=1 && $hour<=23 && $minute<=59 && $second<=59;
  eval {timegm($second,$minute,$hour,$day,$month-1,$year)};
  die "malformed runner timestamp\n" if $@;
  $line=~s/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:\.\d+)?Z //;
 }elsif($line=~/^\d{4}-/){die "malformed runner timestamp\n"}
 return $line;
}
sub parse {
 my($raw,$provider,$receipt)=@_;
 my @roles=Evidence::roles(); my(%files,@manifest); my($state,$index,$terminal)=('outside',0,0); my($head,$chunk,$bytes);
 for my $line(split /\n/,$raw,-1){
  $line=runner_line($line);
  if($state eq 'outside'){
   if($line=~/^UEV3 BEGIN (.+)$/){
    my $h=canonical_json($1); Evidence::exact_keys($h,qw(schema provider run_id trigger_sha run_attempt source_sha));
    die "wrong protocol context\n" unless $h->{schema}==3 && $h->{provider} eq $provider && "$h->{run_id}" eq "$receipt->{run_id}" && $h->{trigger_sha} eq $receipt->{trigger_sha} && $h->{run_attempt}==1 && $h->{source_sha} eq $Evidence::SOURCE;
    $state='between'; next;
   }
  } elsif($state eq 'between'){
   if($line=~/^UEV3 FILE (.+)$/){
    die "extra evidence role\n" unless $index<@roles;
    $head=canonical_json($1);Evidence::exact_keys($head,qw(role path bytes sha256));
    die "missing/reordered/duplicate/arbitrary evidence role\n" unless $head->{role} eq $roles[$index][0] && $head->{path} eq $roles[$index][1];
    die "invalid payload metadata\n" unless "$head->{bytes}"=~/^(?:0|[1-9][0-9]*)$/ && ($head->{bytes}>0 || Evidence::may_be_empty($head->{role})) && $head->{bytes}<=134217728 && $head->{sha256}=~/^[0-9a-f]{64}$/;
    ($chunk,$bytes)=(0,'');$state='file';next;
   }
   if($line=~/^UEV3 END (.+)$/){
    my $end=canonical_json($1);Evidence::exact_keys($end,qw(files manifest_sha256 complete));
    die "incomplete evidence role set\n" unless $index==@roles && $end->{files}==@roles && $end->{complete};
    die "manifest digest mismatch\n" unless $end->{manifest_sha256} eq sha256_hex(join("\n",@manifest)."\n");
    $state='ended';next;
   }
   die "unexpected evidence framing\n";
  } elsif($state eq 'file'){
   if($line=~/^UEV3 DATA (.+)$/){
    my $part=canonical_json($1);Evidence::exact_keys($part,qw(index text));
    die "BOM within evidence payload\n" if !ref($part->{text}) && $part->{text}=~/\x{FEFF}/;
    die "NUL within evidence payload\n" if !ref($part->{text}) && index($part->{text},"\0")>=0;
    die "reordered/invalid chunk\n" unless $part->{index}==$chunk && !ref($part->{text}) && length($part->{text})>0 && length($part->{text})<=4096;
    $bytes.=encode('UTF-8',$part->{text});$chunk++;die "payload exceeds declaration\n" if length($bytes)>$head->{bytes};next;
   }
   if($line=~/^UEV3 END_FILE ([a-zA-Z0-9_.-]+) ([0-9]+)$/){
    die "file footer mismatch\n" unless $1 eq $head->{role} && $2==$chunk && ($chunk>0 || Evidence::may_be_empty($head->{role}));
    die "truncated/corrupt payload\n" unless length($bytes)==$head->{bytes} && sha256_hex($bytes) eq $head->{sha256};
    $files{$head->{role}}=decode('UTF-8',$bytes,FB_CROAK);
    push @manifest,join("\t",$head->{role},$head->{path},$head->{bytes},$head->{sha256},$chunk);
    $index++;$state='between';next;
   }
   die "unexpected text within evidence payload\n";
  } elsif($state eq 'ended'){
   if($line=~/^UNICODE_PILOT_V3_TERMINAL /){
    die "failed/duplicate terminal result\n" unless $line eq "UNICODE_PILOT_V3_TERMINAL provider=$provider exit=0 cleanup=0 emission=0 post_resource=0" && !$terminal++;
    next;
   }
  }
  die "unexpected/reordered protocol or legacy result\n" if $line=~/^(UEV3|UNICODE_PILOT|CONTAINER_TERMINAL|BEGIN_PAIR_RECEIPT|END_PAIR_RECEIPT|BEGIN_COMPLETE_MEMBER_LOG|END_COMPLETE_MEMBER_LOG)/;
  die "runner reported failure\n" if $line=~/Process completed with exit code ([1-9][0-9]*)/;
 }
 die "missing/truncated/endless evidence envelope\n" unless $state eq 'ended' && $terminal==1;
 return \%files;
}
sub assert_keys {my($hash,@wanted)=@_;die "wrong named hash roles\n" unless join('\n',sort keys%$hash) eq join('\n',sort@wanted)}
sub command {
 my($text,@want)=@_;my @actual=shellwords($text);
 die "unexpected command array\n" unless join("\0",@actual) eq join("\0",@want);
}
sub stage {
 my($files,$scope,$name,@cmd)=@_;
 die "failed/incomplete stage $scope.$name\n" unless $files->{"$scope.$name.status"}=~/\Astage=\Q$name\E native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=([1-9][0-9]*)\n\z/;
 my $allowance=$1;my %limits=(pull=>600,install=>1200,configure=>600,build=>3600,fixture=>360,noop=>600,ownership=>600,boot=>360,'boot-parse'=>120,crc=>1020,'crc-parse'=>300,chacha=>1020,'chacha-parse'=>300,iov=>1020,'iov-parse'=>300);
 die "wrong stage allowance\n" unless $name eq 'container' ? ($allowance>1560 && $allowance<=6000) : (exists($limits{$name}) && $allowance==$limits{$name});
 die "empty stage log\n" unless $files->{"$scope.$name.log"}=~/\S/;
 Evidence::resources($files->{"$scope.$name.resources.tsv"});
 command($files->{"$scope.$name.command"},@cmd) if @cmd;
}
sub validate {
 my($f,$provider,$receipt,$dir)=@_;
 my $context=$Evidence::JSON->decode($f->{'runtime.context.json'});
 Evidence::exact_keys($context,qw(provider run_id trigger_sha run_attempt source_sha job_started_epoch job_deadline_epoch container_started_epoch container_deadline_epoch));
 die "runtime context mismatch\n" unless $context->{provider} eq $provider && "$context->{run_id}" eq "$receipt->{run_id}" && $context->{trigger_sha} eq $receipt->{trigger_sha} && $context->{run_attempt}==1 && $context->{source_sha} eq $Evidence::SOURCE;
 for(qw(job_started_epoch job_deadline_epoch container_started_epoch container_deadline_epoch)){die "invalid deadline value\n" unless "$context->{$_}"=~/^[1-9][0-9]{9,11}$/}
 die "unreviewed aggregate/job budget\n" unless $context->{job_deadline_epoch}==$context->{job_started_epoch}+6900 && $context->{container_started_epoch}>=$context->{job_started_epoch} && $context->{container_deadline_epoch}<=$context->{container_started_epoch}+6000 && $context->{container_deadline_epoch}<=$context->{job_deadline_epoch}-180;
 my $container_allowance=$context->{container_deadline_epoch}-$context->{container_started_epoch};
 die "container allowance differs from deadline\n" unless $f->{'host.container.status'}=~/timeout_seconds=\Q$container_allowance\E\n\z/;
 die "host result/cleanup failed\n" unless $f->{'host.cleanup.status'} eq "cleanup_exit=0\n" && $f->{'host.result.status'} eq "native_exit=0 cleanup_exit=0\n";
 die "missing owned container identity\n" unless $f->{'host.container.cid'}=~/\A[0-9a-f]{64}\z/;
 my $owned_cid=$f->{'host.container.cid'};
 my $verified_absence=()=$f->{'host.cleanup.inspect'}=~/^cleanup_verified_absent=\Q$owned_cid\E$/mg;
 die "container absence not verified\n" unless $verified_absence==1;
 for my $p(qw(preflight terminal)){
  my $phase=$p eq 'preflight'?'initial':'terminal';
  die "host admission/terminal missing\n" unless $f->{"host.$p.txt"}=~/RESOURCE_STAGE=\Q$phase\E UTC=/ && $f->{"host.$p.txt"}=~/CAPS O_allocated_bytes=(\d+) evidence_allocated_bytes=(\d+)/ && $1<=1342177280 && $2<=134209536;
 }
 die "wrong image\n" unless $f->{'host.image.txt'}=~/sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c/ && $f->{'host.image.txt'}=~/amd64/;
 stage($f,'host','pull','docker','pull','--platform','linux/amd64',$IMAGE);
 stage($f,'host','container');
 my @docker=shellwords($f->{'host.container.command'});
 die "wrong Docker execution boundaries\n" unless @docker==36 && join(' ',@docker[0..7]) eq "docker run --rm --platform linux/amd64 --name unicode-pilot-$receipt->{run_id}-$provider --cidfile";
 die "wrong Docker label\n" unless $docker[9] eq '--label' && $docker[10] eq "unicode_pilot_run=$receipt->{run_id}";
 my @roots;
 for my $j(0..2){my $i=11+$j*2;die "unexpected Docker mount\n" unless $docker[$i] eq '--mount';my($src,$dst,$ro)=$docker[$i+1]=~/^type=bind,src=(\/[a-zA-Z0-9_.\/-]+),dst=(\/[a-z]+)(,readonly)?$/;die "invalid Docker bind\n" unless defined$src;
  my @expected=('/src','/pilot','/work');die "wrong Docker destination\n" unless $dst eq $expected[$j] && ($j==2?!defined($ro):defined($ro));
  my $suffix=$j==0?'/linux':$j==1?'/pilot/.github/pilots/unicode-v1':'/work';die "unexpected source mount\n" unless $src=~s/\Q$suffix\E$//;push @roots,$src;
 }
 die "mount roots differ\n" unless $roots[0] eq $roots[1] && $roots[0] eq $roots[2] && $docker[8] eq "$roots[0]/evidence/container.cid";
 my @tail=('--env',"PILOT_PROVIDER=$provider",'--env','PILOT_RUN_ATTEMPT=1','--env',"PILOT_RUN_ID=$receipt->{run_id}",'--env',"PILOT_TRIGGER_SHA=$receipt->{trigger_sha}",'--env',"PILOT_JOB_STARTED_EPOCH=$context->{job_started_epoch}",'--env',"PILOT_JOB_DEADLINE_EPOCH=$context->{job_deadline_epoch}",'--env',"PILOT_CONTAINER_STARTED_EPOCH=$context->{container_started_epoch}",'--env',"PILOT_CONTAINER_DEADLINE_EPOCH=$context->{container_deadline_epoch}",$IMAGE,'bash','/pilot/run-container.sh');
 die "unexpected Docker environment/command\n" unless join("\0",@docker[17..$#docker]) eq join("\0",@tail);
 stage($f,'runtime','install','bash','/pilot/install-public-tools.sh');
 my @make=('make','-C','/src','O=/work/O','ARCH=x86_64','LLVM=1','HOST_TOOLS_LANG=rust','RUSTC=/usr/bin/rustc','HOSTRUSTC=/usr/bin/rustc','BINDGEN=/usr/bin/bindgen','-j2');
 my @targets=qw(vmlinux modules bzImage usr_gen_init_cpio);
 stage($f,'runtime','configure',@make,'olddefconfig');
 stage($f,'runtime','build',@make,@targets);stage($f,'runtime','noop',@make,@targets);
 stage($f,'runtime','fixture','bash','/pilot/prepare-boot.sh');
 stage($f,'runtime','ownership','bash','/pilot/audit-integrated.sh',$provider,'/work/O');
 stage($f,'runtime','boot','timeout','--verbose','--signal=TERM','--kill-after=10','240','qemu-system-x86_64','-L','/work/O/boot-inputs/qemu-data','-machine','q35','-accel','tcg,thread=single','-cpu','max','-smp','1','-m','2048','-nic','none','-nodefaults','-display','none','-serial','stdio','-monitor','none','-no-reboot','-kernel','/work/O/arch/x86/boot/bzImage','-initrd','/work/O/boot-inputs/initramfs.cpio.gz','-append','console=ttyS0,115200 rdinit=/init panic=-1 oops=panic nokaslr kunit.enable=0');
 stage($f,'runtime','boot-parse','perl','/pilot/verify-boot.pl','/work/evidence/boot.log','/work/evidence/boot.status','/work/evidence/source.commit','/work/O/include/config/kernel.release','/work/evidence/boot-inputs.sha256');
 admission($f,'boot',$context,240,360,120);
 die "build did not finish bzImage\n" unless $f->{'runtime.build.log'}=~/Kernel: arch\/x86\/boot\/bzImage is ready/;
 for(qw(configure noop)){die "missing make result\n" unless $f->{"runtime.$_.log"}=~/make.*(?:Entering|Leaving) directory/}
 my $lock=Evidence::slurp("$dir/public-source.sha256");
 die "wrong source lock\n" unless $f->{'runtime.source.lock'} eq $lock;
 my $source_hashes=Evidence::hashes($lock);
 my $source_ok=join('',map{"$_: OK\n"}map{/^\w+  (.+)$/?$1:die 'lock'}split /\n/,$lock)."SOURCE_VERIFIED=$Evidence::SOURCE\n";
 die "missing source verification\n" unless $f->{'runtime.source.before.txt'} eq $source_ok && $f->{'runtime.source.after.txt'} eq $source_ok && $f->{'runtime.source.commit'} eq "$Evidence::SOURCE\n";
 my $config_lock=Evidence::slurp("$dir/config-lock.json");
 die "wrong frozen config lock\n" unless $f->{'runtime.config-lock.json'} eq $config_lock;
 my $ident=$Evidence::JSON->decode($config_lock);Evidence::exact_keys($ident,qw(schema c_sha256 rust_sha256));
 die "config lock schema\n" unless $ident->{schema}==1;
 for(qw(c_sha256 rust_sha256)){die "unfrozen config digest\n" unless $ident->{$_}=~/^[0-9a-f]{64}$/}
 open my $config_fh,'-|','gzip','-dc','--',"$dir/integrated-rust.config.gz" or die $!;
 my $rust_config=do {local $/;<$config_fh>};close $config_fh or die "frozen config decompression failed\n";
 die "wrong frozen Rust config bytes\n" unless sha256_hex($rust_config) eq $ident->{rust_sha256};
 die "wrong actual frozen config bytes\n" unless sha256_hex(encode('UTF-8',$f->{'runtime.config.full'})) eq $ident->{"${provider}_sha256"};
 my $want=Evidence::config($rust_config);my $got=Evidence::config($f->{'runtime.config.full'});
 for my $key (selectors()){die "missing selected new provider\n" unless ($want->{$key}//'') eq 'y';$want->{$key}=$provider eq 'rust'?'y':'n'}
 die "config symbol/value drift\n" unless $Evidence::JSON->encode($want) eq $Evidence::JSON->encode($got);
 die "config observer receipt absent\n" unless $f->{'runtime.config-check.txt'} eq "CONFIG_EXACT_FROZEN_MATCH provider=$provider symbols=".scalar(keys%$got)."\n";
 for my $key(qw(CONFIG_RUST_INIT_MAIN CONFIG_RUST_CRC16 CONFIG_RUST_SCATTERLIST CONFIG_RUST_OVERFLOW_CHECKS CONFIG_ACPI CONFIG_KUNIT CONFIG_CRC_KUNIT_TEST CONFIG_CRC_ENABLE_ALL_FOR_KUNIT CONFIG_CRC_BENCHMARK CONFIG_TEST_IOV_ITER CONFIG_CRYPTO_LIB_ENABLE_ALL_FOR_KUNIT CONFIG_CRYPTO_LIB_CHACHA20POLY1305_KUNIT_TEST)){die "regression prerequisite disabled $key\n" unless ($got->{$key}//'') eq 'y'}
 die "changed original KUnit timeout\n" unless ($got->{CONFIG_KUNIT_DEFAULT_TIMEOUT}//'') eq '300';
 for my $key(qw(CONFIG_KUNIT_ALL_TESTS CONFIG_KUNIT_TEST CONFIG_KUNIT_EXAMPLE_TEST CONFIG_RUST_KUNIT_TESTS CONFIG_RUST_KERNEL_DOCTESTS CONFIG_UNICODE CONFIG_CRYPTO_LIB_BENCHMARK)){die "unreviewed test/profile selection $key\n" unless ($got->{$key}//'n') eq 'n'}
 my $suites="trixie\t0584fba32e13e0ab8285fb16c27adea1ec03a73669c18702821094fd6ca86675\ntrixie-updates\tbde606f5b1303e2c864d2118fca1a07c8aca59e097d10180f4e1ea1e7f53c99c\ntrixie-security\tf44452462d1d78526274ee79a391cd8e8df9dc327717dabc39a9d868eb7e4114\n";
 die "missing/wrong signed suite identities\n" unless $f->{'runtime.apt-suites.tsv'} eq $suites;
 for my $suite(qw(trixie trixie-updates trixie-security)){die "missing actual apt suite hash check\n" unless $f->{'runtime.install.log'}=~/_dists_\Q$suite\E_InRelease: OK/m}
 my %installed;
 for(split /\n/,$f->{'runtime.installed-packages.tsv'}){my($n,$v,$a)=split /\t/;die "installed package record\n" unless defined$a && $n=~/^[a-z0-9+.:_-]+$/ && $v=~/^\S+$/ && $a=~/^(amd64|all)$/;$n=~s/:amd64$//;die "duplicate package\n" if exists$installed{$n};$installed{$n}=$v}
 my @pins=split /\n/,Evidence::slurp("$dir/public-packages-candidate.tsv");shift@pins;for(@pins){my($n,$v)=split /\t/;die "missing/wrong pinned package $n\n" unless ($installed{$n}//'') eq $v}
 die "missing dependency closure\n" unless $f->{'runtime.base-packages.tsv'}=~/^base-files\t/m && $f->{'runtime.downloaded-packages.tsv'}=~/^[a-z0-9+.-]+\t\S+\t(?:amd64|all)\t[0-9a-f]{64}$/m;
 my $versions=$f->{'runtime.tool-versions.txt'};for(qr/clang version 19\.1\.7/,qr/rustc 1\.85\.1/,qr/bindgen 0\.71\.1/,qr/v1\.30/,qr/QEMU emulator version 10\.0\.13/){die "wrong/missing tool version\n" unless $versions=~$_}
 die "missing compiler path equivalence\n" unless $versions=~m{^RUSTC_PATH_EQUIVALENCE\ bare\=rustc\ resolved\=\/usr\/bin\/rustc\ requested\=\/usr\/bin\/rustc$}m;
 my $tools=Evidence::hashes($f->{'runtime.tool-binaries.sha256'});assert_keys($tools,qw(/usr/bin/rustc /usr/bin/rustfmt /usr/bin/bindgen /usr/bin/pahole /usr/bin/qemu-system-x86_64 /usr/share/qemu/qboot.rom));
 die "unapproved Rust compiler\n" unless $tools->{'/usr/bin/rustc'} eq 'b4e139165f4f075f9a3fb4b20e7e4898472f08304961706072752d4aa11522b1';
 my $images=Evidence::hashes($f->{'runtime.image-identities.sha256'});
 assert_keys($images,qw(/work/O/arch/x86/boot/bzImage /work/O/vmlinux /work/O/vmlinux.unstripped /work/O/vmlinux.o /work/O/vmlinux.a /work/O/.config /work/O/usr/gen_init_cpio /work/O/Module.symvers /work/O/modules.order /work/O/include/config/kernel.release));
 my $whole=Evidence::hashes($f->{'runtime.O-before-noop.sha256'});
 for my $path(keys%$whole){die "unsafe output path\n" unless $path=~m{^\./[^\x00-\x20\\]+$} && $path!~m{(?:^|/)\.\.(?:/|$)}}
 for my $kind(qw(after-noop before-guest after-guest)){
  die "output changed\n" unless $f->{"runtime.O-$kind.sha256"} eq $f->{'runtime.O-before-noop.sha256'};
  die "output symlinks changed\n" unless $f->{"runtime.O-links-$kind.tsv"} eq $f->{'runtime.O-links-before-noop.tsv'};
 }
 die "missing symlink inventory\n" unless $f->{'runtime.O-links-before-noop.tsv'}=~/^path\ttarget\n/;
 for my $path(keys%$images){my$key=$path;$key=~s!^/work/O/!./!;die "image inventory not bound\n" unless ($whole->{$key}//'') eq $images->{$path}}
 die "config identity differs\n" unless $images->{'/work/O/.config'} eq $ident->{"${provider}_sha256"};
 # The existing UEV3 framing authenticates these complete selected command
 # reports against the observed run; every command also binds to O hashes.
 for my $role(qw(owners.commands original-tests.commands)){
  die "missing selected source commands\n" unless $f->{"runtime.$role"}=~/^source_\S+ := /m;
 }
 my $owner=$Evidence::JSON->decode($f->{'runtime.audit-integrated.receipt'});
 Evidence::exact_keys($owner,qw(schema provider source_sha owners original_C_suites original_C_cases actual_init_kunit_call verdict));
 die "incomplete integrated ownership verdict\n" unless $owner->{schema}==1 && $owner->{provider} eq $provider && $owner->{source_sha} eq $Evidence::SOURCE && $owner->{owners}==24 && $owner->{original_C_suites}==3 && $owner->{original_C_cases}==34 && $owner->{actual_init_kunit_call} && $owner->{verdict} eq 'PASS';
 my %commands;
 for my $role(qw(owners.commands original-tests.commands)){
  my $text=$f->{"runtime.$role"};my $blocks=0;
  while($text=~/^BEGIN ([A-Za-z0-9_.+\/-]+)\n(.*?)\nEND \1\n/msg){
   my($path,$body)=($1,$2);die "duplicate command sidecar\n" if exists$commands{$path};
   die "command sidecar not bound to O\n" unless ($whole->{"./$path"}//'') eq sha256_hex(encode('UTF-8',$body));
   $commands{$path}=$body;$blocks++;
  }
  die "empty command report\n" unless $blocks;
 }
 for my $role(qw(owners.sha256 original-tests.sha256)){
  my $hashes=Evidence::hashes($f->{"runtime.$role"});
  for my $path(keys%$hashes){
   if($path=~m{^/work/O/(.+)$}){die "audited object not bound to O\n" unless ($whole->{"./$1"}//'') eq $hashes->{$path}}
   elsif($path=~m{^/src/(.+)$}){die "audited source digest mismatch\n" if exists($source_hashes->{$1}) && $source_hashes->{$1} ne $hashes->{$path}}
   elsif($path=~m{^/pilot/([A-Za-z0-9_.+-]+)$}){die "audited helper digest mismatch\n" unless sha256_hex(Evidence::slurp("$dir/$1")) eq $hashes->{$path}}
   else{die "unexpected audited input path\n"}
  }
 }
 my @rows=split /\n/,Evidence::slurp("$dir/integrated-owners.tsv");shift@rows;
 my @crows=split /\n/,Evidence::slurp("$dir/integrated-c-owners.tsv");shift@crows;
 my %crows=map{my@v=split /\t/;($v[0],\@v)}@crows;
 my %switch=(bpf_lpm_trie=>'RUST_BPF_LPM_TRIE',bpf_token=>'RUST_BPF_TOKEN',power_process=>'RUST_POWER_PROCESS',blk_crypto_fallback=>'RUST_BLK_CRYPTO_FALLBACK',selinux_services=>'RUST_SELINUX_SERVICES',selinux_policydb=>'RUST_SELINUX_POLICYDB');
 die "manifest size changed\n" unless @rows==24 && keys(%crows)==6;
 my $selected_rows=()=$f->{'runtime.owners.symbols'}=~/^SELECTED_ROW /mg;
 die "extra/missing selected owner rows\n" unless $selected_rows==24;
 for(@rows){
  my($name,$object,$source,$archive,$symbol,$mod)=split /\t/;my$language='Rust';
  if($provider eq 'c' && $switch{$name}){my$c=$crows{$switch{$name}}//die 'missing C owner';($object,$source,$archive,$symbol)=@$c[1..4];$language='C'}
  die "selected owner mismatch\n" unless $f->{'runtime.owners.symbols'}=~/^SELECTED_ROW \Q$name\E language=\Q$language\E object=\Q$object\E$/m;
  die "selected final symbol missing\n" unless $f->{'runtime.owners.symbols'}=~/^OWNER \Q$object $symbol\E object_type=[12] binding=[0-9]+ bytes=[1-9][0-9]* final=0x[0-9a-f]+ final_binding=[0-9]+$/m;
  for my$a($archive,'vmlinux.a'){die "selected archive leaf missing\n" unless $f->{'runtime.owners.archives'}=~/^UNIQUE_MEMBER \Q$a $object\E$/m}
  (my$cf=$object)=~s{([^/]+)$}{.$1.cmd};my$body=$commands{$cf}//die 'missing owner command';
  die "wrong selected source\n" unless $body=~/^source_\Q$object\E := \/src\/\Q$source\E$/m;
  my($saved)=$body=~/^savedcmd_\Q$object\E := (.*)$/m;die "missing owner saved command\n" unless defined$saved;
  if($language eq 'Rust'){die "wrong actual Rust compiler\n" unless $saved=~m{(?:^| )/usr/bin/rustc };OverflowPolicy::check("savedcmd_fs/unicode/utf8-norm.o := $saved\n")}
  else{die "wrong actual C compiler/overflow policy\n" unless $saved=~/^clang(?:-19)? / && $saved=~/ -fno-strict-overflow /}
 }
 for my $pair(['lib/crc/tests/crc_kunit.o','lib/crc/tests/crc_kunit.c'],['lib/crypto/tests/chacha20poly1305_kunit.o','lib/crypto/tests/chacha20poly1305_kunit.c'],['lib/tests/kunit_iov_iter.o','lib/tests/kunit_iov_iter.c']){
  my($object,$source)=@$pair;(my$cf=$object)=~s{([^/]+)$}{.$1.cmd};my$body=$commands{$cf}//die 'missing original test command';
  die "original C producer/flags not proven\n" unless $body=~/^source_\Q$object\E := \/src\/\Q$source\E$/m && $body=~/^savedcmd_\Q$object\E := clang(?:-19)? .* -fno-strict-overflow .* -c -o /m;
 }
 my$elf=$f->{'runtime.elf-ownership.txt'};
 die "missing real startup owner\n" unless $elf=~/^REAL_START_KERNEL owner=init\/main.o source=init\/main.rs final_definitions=1$/m;
 die "missing actual Rust KUnit startup call proof\n" unless $elf=~/^ACTUAL_RUST_INIT_KUNIT_CALL caller=_R\S*kernel_init_freeable\S* target=kunit_run_all_tests$/m && $elf=~/^FINAL_CALL init\/main.o _R\S+ -> kunit_run_all_tests field=0x[0-9a-f]+ target=0x[0-9a-f]+$/m;
 die "incomplete final ELF proof\n" unless $elf=~/^INTEGRATED_ELF_OWNERSHIP_PASS manifest_owners=24 original_C_suites=3 original_C_cases=34 actual_init_kunit_call=1$/m;
 my $boot=$Evidence::JSON->decode($f->{'runtime.boot-verification.json'});
 Evidence::exact_keys($boot,qw(result checks scope known_open_gate identity));
 die "boot result not passing\n" unless $boot->{result} eq 'PASS_BOOT_SMOKE_ONLY';
 my @checks=qw(terminal exit_zero not_forced serial_hash_matches correct_source_commit pid1_begin_exactly_once correct_guest_release success_exactly_once poweroff_marker_exactly_once kernel_powered_down boot_only_scope no_faults bash_posix_shell mount_/proc mount_/sys mount_/dev mount_/run mount_/tmp);
 Evidence::exact_keys($boot->{checks},@checks);
 for(@checks){die "failed historical boot check $_\n" unless JSON::PP::is_bool($boot->{checks}{$_}) && $boot->{checks}{$_}}
 Evidence::exact_keys($boot->{identity},qw(source_sha kernel_release serial_sha256 immutable_inputs_sha256 initramfs_sha256 initramfs_bytes));
 my $boot_context=$Evidence::JSON->decode($f->{'runtime.boot-context.json'});
 Evidence::exact_keys($boot_context,qw(source_sha kernel_release immutable_inputs_sha256));
 die "boot source/release identity mismatch\n" unless $boot->{identity}{source_sha} eq $Evidence::SOURCE && $boot_context->{source_sha} eq $Evidence::SOURCE && $boot_context->{kernel_release} eq $boot->{identity}{kernel_release};
 my $serial_hash=sha256_hex(encode('UTF-8',$f->{'runtime.boot.log'}));
 die "boot serial identity mismatch\n" unless $boot->{identity}{serial_sha256} eq $serial_hash && $f->{'runtime.boot-serial.sha256'} eq "$serial_hash  /work/evidence/boot.log\n";
 die "boot input ledger not sealed\n" unless $boot_context->{immutable_inputs_sha256} eq sha256_hex(encode('UTF-8',$f->{'runtime.boot-inputs.sha256'})) && $boot->{identity}{immutable_inputs_sha256} eq $boot_context->{immutable_inputs_sha256};
 my $inputs=Evidence::hashes($f->{'runtime.boot-inputs.sha256'});
 for my $path(keys%$inputs){
  if($path=~m{^/work/O/}){(my$key=$path)=~s!^/work/O/!./!;my$hash=$whole->{$key};
   # Firmware is a recorded symlink, so its target hash has a separate ledger.
   next if $path=~m{^/work/O/boot-inputs/qemu-data/};
   die "immutable input not bound to complete O inventory\n" unless defined($hash) && $hash eq $inputs->{$path};
  }
 }
 for my $path(keys%$images){next unless exists $inputs->{$path};die "boot image differs from linked image\n" unless $inputs->{$path} eq $images->{$path}}
 my $checks_text=join('',map{"$_: OK\n"}map{/^\w+  (.+)$/?$1:die 'boot ledger'}split /\n/,$f->{'runtime.boot-inputs.sha256'});
 die "missing pre/post boot input verification\n" unless $f->{'runtime.boot-inputs.before.txt'} eq $checks_text && $f->{'runtime.boot-inputs.after.txt'} eq $checks_text;
 die "new fixture package pins differ\n" unless $f->{'runtime.boot-packages.tsv'} eq Evidence::slurp("$dir/boot-packages.tsv");
 my @package_rows=split /\n/,$f->{'runtime.boot-packages.tsv'};shift@package_rows;
 my $package_identity="Package\tVersion\tArchitecture\tSize\tSHA256\n";
 for(@package_rows){my@v=split /\t/;die "guest pin columns\n" unless @v==7;$package_identity.=join("\t",@v[0,1,2,3,5])."\n"}
 die "guest actual package identity mismatch\n" unless $f->{'runtime.boot-package-identities.tsv'} eq $package_identity;
 my $det=$f->{'runtime.boot-determinism.tsv'};
 die "fixture determinism record malformed\n" unless $det=~/\Aartifact\tsize_bytes\tfirst_sha256\trepeated_sha256\ninitramfs.cpio\t([1-9][0-9]*)\t([0-9a-f]{64})\t\2\ninitramfs.cpio.gz\t([1-9][0-9]*)\t([0-9a-f]{64})\t\4\n\z/;
 my($cpio_bytes,$cpio_hash,$gzip_bytes,$gzip_hash)=($1,$2,$3,$4);
 die "wrong sealed boot archive\n" unless $inputs->{'/work/O/boot-inputs/initramfs.cpio'} eq $cpio_hash && $inputs->{'/work/O/boot-inputs/initramfs.cpio.gz'} eq $gzip_hash && $boot->{identity}{initramfs_sha256} eq $gzip_hash && $boot->{identity}{initramfs_bytes}==$gzip_bytes;
 die "fixture component cap violated\n" unless $cpio_bytes+$gzip_bytes<=268435456;
 # Repeat original raw serial and upstream topology checks independently of
 # supplemental observer success text. One suite per unchanged guest command.
 for my $spec(['crc','crc','crc-cases.txt',16],['chacha','chacha20poly1305','chacha-cases.txt',1],['iov','iov_iter','iov-cases.txt',17]){
  my($name,$suite,$casefile,$count)=@$spec;
  stage($f,'runtime',$name,'timeout','--signal=TERM','--kill-after=10','900','qemu-system-x86_64','-L','/usr/share/qemu','-nodefaults','-m','2048','-kernel','/work/O/arch/x86/boot/bzImage','-append',"console=ttyS0 kunit.enable=1 kunit.autorun=1 kunit.filter_glob=$suite kunit_shutdown=reboot panic=-1 oops=panic nokaslr",'-no-reboot','-nographic','-accel','tcg,thread=single','-cpu','max','-smp','1','-serial','stdio','-nic','none','-bios','qboot.rom');
  stage($f,'runtime',"$name-parse",'bash','/pilot/parse-guest.sh',$suite,$name);
  admission($f,$name,$context,900,1020,300);
  die "guest native receipt failed\n" unless $f->{"runtime.$name.qemu-exit"} eq "0\n";
  my@cases=split /\n/,Evidence::slurp("$dir/$casefile");die "case manifest length\n" unless @cases==$count;
  my@want=('KTAP version 1','1..1','    KTAP version 1',"    # Subtest: $suite","    1..$count",(map{"    ok ".($_+1)." $cases[$_]"}0..$#cases),"ok 1 $suite");
  my$at=0;my$serial=$f->{"runtime.$name.log"};
  for(split /\n/,$serial){s/\r$//;s/^\[\s*\d+\.\d+\]\s?//;
   die "nonpassing original guest diagnostic\n" if /\bnot ok\b|#\s*SKIP\b|Bail out!|Kernel panic|Oops:|BUG:|KASAN:|UBSAN:/i;
   if(/^\s*(?:KTAP version|1\.\.|(?:not )?ok\s+|# Subtest:)/){die "missing/reordered/extra original case\n" unless $at<@want && $_ eq $want[$at++];}
  }
  die "incomplete original suite\n" unless $at==@want;
  die "failed observer/parser\n" unless $f->{"runtime.$name.parser.status"} eq "observer_exit=0 parser_exit=0\n" && $f->{"runtime.$name.observer.stderr"} eq '';
  my$hash=sha256_hex(encode('UTF-8',$serial));die "observer serial unbound\n" unless $f->{"runtime.$name.observer.txt"}=~/^serial_sha256=\Q$hash\E$/m;
  my$json=$Evidence::JSON->decode($f->{"runtime.$name.upstream.json"});
  die "upstream result topology\n" unless $json->{name} eq 'KUnit Test Group' && ref($json->{sub_groups}) eq 'ARRAY' && @{$json->{sub_groups}}==1 && ref($json->{test_cases}) eq 'ARRAY' && !@{$json->{test_cases}};
  my$group=$json->{sub_groups}[0];die "upstream wrong suite\n" unless $group->{name} eq $suite && ref($group->{sub_groups}) eq 'ARRAY' && !@{$group->{sub_groups}} && ref($group->{test_cases}) eq 'ARRAY' && @{$group->{test_cases}}==$count;
  for my$i(0..$#cases){die "upstream wrong/failed case\n" unless $group->{test_cases}[$i]{name} eq $cases[$i] && $group->{test_cases}[$i]{status} eq 'PASS'}
  for my$node($json,$group){for(qw(tests passed)){die "upstream wrong count\n" unless $node->{misc}{$_}==$count}for(qw(failed crashed skipped errors)){die "upstream failed result\n" unless defined($node->{misc}{$_}) && $node->{misc}{$_}==0}}
  die "upstream parser result missing\n" unless $f->{"runtime.$name.upstream.txt"}=~/Testing complete\. Ran \Q$count\E tests: passed: \Q$count\E/;
 }
 die "incomplete member verdict\n" unless $f->{'runtime.member.status'} eq "provider=$provider source=$Evidence::SOURCE scope=incremental-six-core config=pass noop=pass ownership=pass boot=pass original_C_cases=34 original_C_suites=3 parser=pass invariance=pass new_provider_functional_suites=UNRUN\n";
 return $got;
}
sub selectors {qw(CONFIG_RUST_BPF_LPM_TRIE CONFIG_RUST_BPF_TOKEN CONFIG_RUST_POWER_PROCESS CONFIG_RUST_BLK_CRYPTO_FALLBACK CONFIG_RUST_SELINUX_SERVICES CONFIG_RUST_SELINUX_POLICYDB)}
sub admission {
 my($f,$name,$context,$native,$stage,$parser)=@_;my$a=$Evidence::JSON->decode($f->{"runtime.$name.admission.json"});
 Evidence::exact_keys($a,qw(now job_remaining container_remaining required_seconds guest_timeout guest_kill_grace guest_stage_allowance parser_seconds cleanup_seconds evidence_seconds));
 for(keys%$a){die "invalid guest budget number\n" unless "$a->{$_}"=~/^[0-9]+$/}
 my$required=$stage+$parser+60+180;
 die "changed guest timeout/reserve\n" unless $a->{guest_timeout}==$native && $a->{guest_kill_grace}==10 && $a->{guest_stage_allowance}==$stage && $a->{parser_seconds}==$parser && $a->{cleanup_seconds}==60 && $a->{evidence_seconds}==180 && $a->{required_seconds}==$required;
 die "insufficient guest budget\n" unless $a->{now}>=$context->{container_started_epoch} && $a->{container_remaining}==$context->{container_deadline_epoch}-$a->{now} && $a->{job_remaining}==$context->{job_deadline_epoch}-$a->{now} && $a->{container_remaining}>=$required && $a->{job_remaining}>=$required;
}
sub compare {
 my($receipt,$craw,$rraw,$dir)=@_;
 Evidence::exact_keys($receipt,qw(schema repository branch event run_id run_attempt trigger_sha decoding jobs));
 die "wrong trusted download receipt context\n" unless $receipt->{schema}==1 && $receipt->{repository} eq 'a-coding-mage/linux' && $receipt->{branch} eq 'feat/rust-translation-lupos' && $receipt->{event} eq 'push' && "$receipt->{run_id}"=~/^[1-9][0-9]*$/ && $receipt->{run_attempt}==1 && $receipt->{trigger_sha}=~/^[0-9a-f]{40}$/ && $receipt->{decoding} eq 'connector-decoded-text-utf8' && ref($receipt->{jobs}) eq 'HASH';
 Evidence::exact_keys($receipt->{jobs},qw(c rust));my(%files,%config);
 for my$provider(qw(c rust)){
  my$raw=$provider eq 'c'?$craw:$rraw;my$job=$receipt->{jobs}{$provider};Evidence::exact_keys($job,qw(job_id name status conclusion text_sha256));
  die "wrong/incomplete GitHub job receipt\n" unless "$job->{job_id}"=~/^[1-9][0-9]*$/ && $job->{name} eq "integrated-$provider" && $job->{status} eq 'completed' && $job->{conclusion} eq 'success' && $job->{text_sha256} eq sha256_hex($raw);
  $files{$provider}=parse($raw,$provider,$receipt);$config{$provider}=validate($files{$provider},$provider,$receipt,$dir);
 }
 die "same job supplied twice\n" if $receipt->{jobs}{c}{job_id}==$receipt->{jobs}{rust}{job_id};
 for my$role(qw(source.commit source.lock source.before.txt source.after.txt config-lock.json apt-suites.tsv base-packages.tsv downloaded-packages.tsv installed-packages.tsv tool-versions.txt tool-binaries.sha256 original-tests.commands original-tests.sha256 boot-packages.tsv boot-package-identities.tsv boot-layout.tsv boot-host-layout.tsv boot-files.sha256 boot-determinism.tsv boot-fixture.sha256 boot-firmware.sha256 boot-firmware-links.tsv boot-initramfs.list)){
  die "C/R common-input mismatch $role\n" unless $files{c}{"runtime.$role"} eq $files{rust}{"runtime.$role"};
 }
 for my$key(selectors()){
  die "not the required incremental C/R selection\n" unless $config{c}{$key} eq 'n' && $config{rust}{$key} eq 'y';
  delete$config{c}{$key};delete$config{rust}{$key};
 }
 die "C/R config drift beyond six selectors\n" unless $Evidence::JSON->encode($config{c}) eq $Evidence::JSON->encode($config{rust});
 return 1;
}
1;
