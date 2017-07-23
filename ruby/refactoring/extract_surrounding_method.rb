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
