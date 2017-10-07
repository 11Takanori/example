r, w = IO.pipe
w.write('hoge')
w.close
puts r.read
