child_processes = 3
dead_processes  = 0

child_processes.times do
  fork do
    sleep 3
  end
end

trap(:CHLD) do
  puts Process.wait
  dead_processes += 1

  exit if dead_processes == child_processes
end

loop do
  (Math.sqrt(rand(44)) ** 8).floor
  sleep 1
end
