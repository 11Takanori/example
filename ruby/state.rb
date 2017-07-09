class Yakushoku
  def calculate_salary(kihonkyu)
    kihonkyu
  end

  def up
  end

  def down
  end
end

class Tanto < Yakushoku
  def up
    Shunin.new
  end
end

class Shunin < Yakushoku
  def calculate_salary(kihonkyu)
    kihonkyu * 2 + 1
  end

  def down
    Tanto.new
  end
end


class Shanin
  attr_accessor :yakushoku

  def initialize
    @yakushoku = Yakushoku.new
  end

  def calculate_salary(kihonkyu)
    @yakushoku.calculate_salary(kihonkyu)
  end

  def up
    @yakushoku = @yakushoku.up
  end

  def down
    @yakushoku = @yakushoku.down
  end
end

shain = Shanin.new
shain.yakushoku = Tanto.new
puts shain.calculate_salary(100)
shain.up
puts shain.calculate_salary(100)
shain.down
puts shain.calculate_salary(100)
