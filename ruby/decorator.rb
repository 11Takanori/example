class SimpleWriter
  def initialize(path)
    @file = File.open(path, 'w')
  end

  def write_line(line)
    @file.print(line)
    @file.print('\n')
  end

  def pos
    @file.pos
  end

  def rewind
    @file.rewind
  end

  def close
    @file.close
  end
end

class WriterDecorator
  def initialize(real_writer)
    @real_writer = real_writer
  end

  def write_line(line)
    @real_writer = real_writer
  end

  def pos
    @real_writer.pos
  end

  def rewind
    @real_writer.rewind
  end

  def close
    @real_writer.close
  end
end

class NumberingWriter < WriterDecorator
  def initialize(real_writer)
    super(real_writer)
    @line_number = 1
  end

  def write_line(line)
    @real_writer.write_line("#{@line_number} : #{line}")
  end
end

class TimestampingWriter < WriterDecorator
  def write_line(line)
    @real_writer.write_line("#{Time.new} : #{line}")
  end
end

# f = NumberingWriter.new(SimpleWriter.new('file1.txt'))
# f.write_line("hello")
# f.close

f = TimestampingWriter.new(SimpleWriter.new('file2.txt'))
f.write_line("hello")
f.close
