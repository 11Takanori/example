2.times do
  fork do
    abort "Finishied"
  end
end

puts Process.wait
sleep 5

# カーネルは終了したプロセスの情報をキューに入れておく
# キューに溜まっていた２つめの子プロセスの終了情報を取得できる
puts Process.waits
