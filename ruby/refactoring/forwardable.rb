require 'forwardable'

class Hoge
  extend Forwardable
  attr_accessor :arr
  def_delegator :arr, :push

  def initialize
    @arr = []
  end
end

class Hoge2
  extend Forwardable
  def_delegator :@arr, :push

  def initialize
    @arr = []
  end
end

# 毎回@arrが初期化されるので✕
class Hoge3
  extend Forwardable
  def_delegator :arr, :push

  def arr
    @arr = []
  end
end
