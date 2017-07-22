# Refactoring: Ruby Edition p47

def total_charge
  result = 0
  @rentals.each do |element|
    result += element.charge
  end
  result
end

def total_charge2
  @rentals.map(&:charge).sum
end

def total_charge3
  @rentals.map(&:charge).inject(:+)
end
