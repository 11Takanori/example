class Element
  attr_accessor :value, :next_ref

  def initialize(value, next_ref)
    @value    = value
    @next_ref = next_ref
  end
end

class List
  def initialize
    @first_element = nil
  end

  def unshift(value)
    @first_element = Element.new(value, @first_element)
  end

  def each
    current_element = @first_element
    while !current_element.nil?
      yield current_element.value
      current_element = current_element.next_ref
    end
  end
end

class HashTable
  BIN_SIZE = 4

  def initialize
    @bins = Array.new(BIN_SIZE) { List.new }
  end

  def set(key, value)
    index = key.hash % BIN_SIZE
    bin   = @bins[index]

    bin.each do |pair|
      if pair[0] == key
        pair[1] = value
        return self
      end
    end

    bin.unshift([key, value])
    self
  end

  def get(key)
    index = key.hash % BIN_SIZE
    bin   = @bins[index]

    bin.each do |pair|
      if pair[0] == key
        return pair[1]
      end
    end
    nil
  end

  def each
    @bins.each do |bin|
      bin.each do |pair|
        yield pair[0], pair[1]
      end
    end
  end
end

hash = HashTable.new
hash.set('foo', 'bar')
hash.set('hoge', 'fuga')

p hash.get('foo')
p hash.get('hoge')

# hash.each {|k, v| p k; p v}

# list = List.new
# p list.unshift(1)
# p list.unshift(2)
# p list.unshift(3)
# p list.each {|k, v| p k; p v}
