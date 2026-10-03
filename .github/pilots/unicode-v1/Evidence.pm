package Evidence;
# SPDX-License-Identifier: GPL-2.0-only
use strict; use warnings; use JSON::PP; use Digest::SHA qw(sha256_hex); use Encode qw(encode decode FB_CROAK);
our $JSON=JSON::PP->new->canonical->ascii;
our @STAGES=qw(install configure build fixture noop ownership boot boot-parse crc crc-parse chacha chacha-parse iov iov-parse);
our $SOURCE='0008179a1ee0b082fa3ead118187fc6f3569a9f2';
sub roles {
  my @r;
  for my $s(qw(pull container)){for my $kind(qw(command log status resources.tsv)){push @r,["host.$s.$kind","host/$s.$kind"]}}
  push @r,map {["host.$_","host/$_"]} qw(preflight.txt terminal.txt image.txt container.cid cleanup.inspect cleanup.stderr cleanup.status result.status);
  push @r,map {["runtime.$_","runtime/$_"]} qw(context.json source.commit source.lock source.before.txt source.after.txt apt-suites.tsv base-packages.tsv downloaded-packages.tsv installed-packages.tsv tool-versions.txt tool-binaries.sha256 config.full config-lock.json config-check.txt);
  for my $s(@STAGES){for my $kind(qw(command log status resources.tsv)){push @r,["runtime.$s.$kind","runtime/$s.$kind"]}}
  push @r,map {["runtime.$_","runtime/$_"]} qw(O-before-noop.sha256 O-after-noop.sha256 O-before-guest.sha256 O-after-guest.sha256 O-links-before-noop.tsv O-links-after-noop.tsv O-links-before-guest.tsv O-links-after-guest.tsv image-identities.sha256 owners.commands owners.symbols owners.archives owners.sha256 original-tests.commands original-tests.sha256 elf-ownership.txt ownership-summary.txt audit-integrated.receipt boot-packages.tsv boot-package-identities.tsv boot-layout.tsv boot-host-layout.tsv boot-files.sha256 boot-determinism.tsv boot-fixture.sha256 boot-firmware.sha256 boot-firmware-links.tsv boot-initramfs.list boot-inputs.sha256 boot-inputs.before.txt boot-inputs.after.txt boot-context.json boot-serial.sha256 boot-verification.json boot.admission.json member.status);
  for my $s(qw(crc chacha iov)){push @r,map {["runtime.$s.$_","runtime/$s.$_"]} qw(admission.json qemu-exit observer.txt observer.stderr upstream.json upstream.txt parser.status)}
  return @r;
}
sub may_be_empty { return $_[0] =~ /\A(?:host\.cleanup\.(?:inspect|stderr)|runtime\.(?:crc|chacha|iov)\.observer\.stderr)\z/; }
sub slurp {my($p)=@_;open my $f,'<:raw',$p or die "$p: $!";local $/;return <$f>}
sub exact_keys {my($h,@keys)=@_;die "not an object\n" unless ref($h) eq 'HASH';die "unexpected object fields\n" unless join(',',sort keys %$h) eq join(',',sort @keys)}
sub config {
 my($s)=@_;my %c;
 for(split /\n/,$s){my($k,$v);if(/^(CONFIG_\w+)=(.*)$/){($k,$v)=($1,$2)}elsif(/^# (CONFIG_\w+) is not set$/){($k,$v)=($1,'n')}else{next}die "duplicate config $k\n" if exists$c{$k};$c{$k}=$v}
 die "empty config\n" unless keys%c;return \%c;
}
sub hashes {
 my($s)=@_;my %h;
 for(split /\n/,$s){die "invalid hash entry\n" unless /^([0-9a-f]{64})  ([^\r\n]+)$/;my($v,$p)=($1,$2);die "duplicate hash path\n" if exists$h{$p};$h{$p}=$v}
 die "empty hash list\n" unless keys%h;return \%h;
}
sub resources {
 my($text)=@_;my @rows=split /\n/,$text;
 die "resource header\n" unless shift(@rows) eq "phase\tutc\tfree_bytes\tavailable_memory\tfree_inodes\tO_allocated_bytes\tevidence_allocated_bytes";
 die "resource endpoints\n" unless @rows>=2 && $rows[0]=~/^admission\t/ && $rows[-1]=~/^terminal\t/;
 for my $i(0..$#rows){my @v=split /\t/,$rows[$i];die "resource row\n" unless @v==7 && $v[0] eq ($i==0?'admission':$i==$#rows?'terminal':'live') && $v[1]=~/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/;
 for(@v[2..6]){die "resource number\n" unless /^\d+$/}
 die "resource cap/floor failure\n" unless $v[2]>=1073741824 && $v[3]>=536870912 && $v[4]>=10000 && $v[5]<=1342177280 && $v[6]<=134209536;
 die "resource admission failure\n" if $i==0 && ($v[2]<4294967296 || $v[3]<6442450944);
 }
}
1;
