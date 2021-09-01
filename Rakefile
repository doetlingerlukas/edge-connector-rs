# frozen_string_literal: true

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

desc 'compile binary'
task :build => :update_key do
  sh 'cross', 'build', '--release', '--target', TARGET
end
