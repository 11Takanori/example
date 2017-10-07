r, w = IO.pipe

fork do
  r.close

  10.times { w.puts 'hoge'}
end

w.close
while message = r.gets
  $stdout.puts message
end
