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

require "forwardable"

class WriterDecorator
  extend Forwardable

  def_delegators :@real_writer, :write_line, :pos, :rewind, :close

  def initialize(real_writer)
    @real_writer = real_writer
  end
end

module NumberingWriter
  attr_reader :line_number
  def write_line(line)
    @line_number = 1 unless @line_number
    super("#{@line_number} : #{line}")
    @line_number += 1
  end
end

module TimeStampingWriter
  def write_line(line)
    super("#{Time.new} : #{line}")
  end
end

f = SimpleWriter.new('file1.txt')
f.extend TimeStampingWriter
f.extend NumberingWriter
f.write_line('hello')
f.close
