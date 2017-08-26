class MessageCollector
  instance_methods.each do |meth|
    undef_method meth unless meth =~ /~(__|inspect)/
  end

  def messages
    @messages ||= []
  end

  def method_missing(sym, *args)
    messages << [sym, args]
    self
  end
end

class CommandCenter
  def start(command_string)
    self
  end

  def stop(command_string)
    self
  end
end

class Recorder
  def play_for(obj)
    @message_collecot.messages.inject(obj) do |result, message|
      result.send message.first, *message.last
    end
  end

  def record
    @message_collecot ||= MessageCollector.new
  end

  def to_s
    @message_collecot.messages.inject([]) do |result, message|
      result << "#{message.first}(args: #{message.last.inspect})"
    end.join(".")
  end
end

recorder = Recorder.new
recorder.record.start("LRMMMRL")
recorder.record.stop("LRMMMRL")
# recorder.play_for(CommandCenter.new)
