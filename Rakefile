# frozen_string_literal: true

require 'open3'

TARGET = ENV['TARGET'] || 'arm-unknown-linux-gnueabihf'

RPI = ENV['RPI'] || 'edge-device.local'
HOST = "pi@#{RPI}"

def ssh(*args)
  sh 'ssh', HOST, *args
end

desc 'retrieve faasd key from host'
task :update_key do
  out, err, status = Open3.capture3 'ssh', HOST, *<<~SH
    sudo cat /var/lib/faasd/secrets/basic-auth-password
  SH

  File.write 'faas-key', out
end

desc 'connect faas-cli to host'
task :login do
  sh 'faas-cli', 'login', '-u', 'admin', '-s', '--gateway', "http://#{RPI}:8080", :in=>'faas-key'
end

desc 'compile binary'
task :build => :update_key do
  sh 'cross', 'build', '--release', '--target', TARGET
end

task :deploy => :build do
  sh 'scp', "target/#{TARGET}/release/edge-connector", "#{HOST}:~/edge-connector"
  ssh <<~SH
    sudo mv ./edge-connector /usr/local/bin/edge-connector
    sudo chmod 755 /usr/local/bin/edge-connector
  SH
end

task :run => :deploy do
  ssh 'killall', 'edge-connector' rescue nil
  ssh 'env', 'RUST_LOG=info', 'edge-connector'
end
