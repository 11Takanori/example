r, w = IO.pipe
w.write('hoge')
w.close # IO#readがEOFまで読み込まないようにしている
puts r.read
