module CustomInitializers
  def hash_initializer(*attribute_names)
    define_method(:initialize) do |args|
      attribute_names.each do |attribute_name|
        instance_variable_set "@#{attribute_name}", args[attribute_name]
      end
    end
  end
end

Class.send :include, CustomInitializers

class Hoge
  attr_reader :foo, :bar, :foo2
  hash_initializer :foo, :bar, :foo2
end

hoge = Hoge.new(foo: 1, bar: "bar")
p hoge.foo
p hoge.bar
p hoge.foo2
