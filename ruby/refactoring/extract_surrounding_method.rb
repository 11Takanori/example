# 1
# def change(amount, credit_card_number)
#   begin
#     connection = CreditCardServer.connect(...)
#     connection.send(amount, credit_card_number)
#   rescue IOError => e
#     Logger.log "Could not submit order #{@order_number} to the surver #{e}"
#     return nil
#   ensure
#     connection.close
#   end
# end

def change(amount, credit_card_number)
  connect do |connection|
    connection.send(amount, credit_card_number)
  end
end

def connect
  begin
    connection = CreditCardServer.connect(...)
    yield connection
  rescue IOError => e
    Logger.log "Could not submit order #{@order_number} to the surver #{e}"
    return nil
  ensure
    connection.close
  end
end

# 2
# def number_of_living_descendants
#   chilidren.inject(0) do |count, child|
#     count += 1 if child.alive?
#     count + child.number_of_living_descendants
#   end
# end
#
# def number_of_descendants_named(name)
#   chilidren.inject(0) do |count, child|
#     count += 1 if child.name == name
#     count + child.number_of_descendants_named(name)
#   end
# end

def number_of_living_descendants
  count_descendants_matching { |d| d.alive? }
end

def number_of_descendants_named(name)
  count_descendants_matching { |d| d.name == name }
end

def count_descendants_matching(&block)
  chilidren.sum do |count, child|
    count += 1 if yield child
    count + child.count_descendants_matching(&block)
  end
end
