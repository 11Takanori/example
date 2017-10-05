child_processes = 3
dead_processes  = 0

child_processes.times do
  fork do
    sleep 3
  end
end

# $stdoutの出力を同期モードにする
# putsを呼び出した後にシグナルハンドラが中断された場合にはThreadError例外が送出されるようになる
$stdout.sync = true

trap(:CHLD) do
  begin
    # waitの第２引数に`Process::WNOHANG`を渡すことで、
    # 終了を待つ子プロセスが無ければブロックしないようにカーネルに指示する
    while pid = Process.wait(-1, Process::WNOHANG)
      puts pid
      dead_processes += 1
    end
  rescue Errno::ECHILD
  end
end

loop do
  exit if dead_processes == child_processes

  sleep 1
end
