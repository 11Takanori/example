class Foo
  attr_accessor :state

  def self.def_each(*method_names, &block)
    method_names.each do |method_name|
      define_method method_name do
        instance_exec method_name, &block
      end
    end
  end

  def_each :failure, :error, :success do |method_name|
    self.state = method_name
  end
end

class Bar
  attr_accessor :state
  
  def self.states(*args)
    args.each do |arg|
      define_method arg do
        self.state = arg
      end
    end
  end

  states :failure, :error, :success
end
