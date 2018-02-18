#!/usr/bin/env ruby
container_name = ARGV[0]
raise unless container_name
Dir.mkdir "sys/fs/cgroup/cpu/#{container_name}" rescue puts("skip")
File.write "sys/fs/cgroup/cpu/#{container_name}/tasks", $$.to_s
Dir.chroot "root/#{container_name}"
Dir.chdir "/"
system "mount -t proc proc /proc"
system "hostname #{container_name}.example.jp"

exec "bash -l"
