pid = fork { sleep 1 }

puts pid # 46324とする

sleep 5

# ps -ho pid,state -p 46324
# PID STAT
# 46324 Z+
# Z+はゾンビプロセスであることを示す
