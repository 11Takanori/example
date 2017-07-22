# class Select
#   def options
#     @options ||= []
#   end
#
#   def add_option(arg)
#     options << arg
#   end
# end
#
# select = Select.new
# select.add_option(1)
# select.add_option(2)
# select.add_option(3)
# select.add_option(4)

class Select
  def self.with_option(option)
    select = new
    select.options << option
    select
  end

  def options
    @options ||= []
  end

  def and(arg)
    options << arg
    self
  end
end

select = Select.with_option(1).and(2).and(3).and(4)
