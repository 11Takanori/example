module Join
  def initialize(table, options)
    @table     = table
    @condition = options[:on]
  end

  def self.included(klass) # ここのselfはJoin
    klass.class_eval do
      def self.joins_for_table(table_name) # ここのselfはincludeしたクラス
        # 何かする
      end
    end
  end

  def self.joins_for_table2(table_name)　# ここのselfはincludeしたクラス
    # 何かする
  end

  def to_sql
    "#{join_type} JOIN #{@table} ON #{@condition}"
  end
end


class LeftOuterJoin
  include Join

  def join_type
    "LEFT OUTER"
  end
end

class InnerJoin
  include Join

  def join_type
    "INNER"
  end
end

InnerJoin.new("users", on: "user_id = users.id")
InnerJoin.joins_for_table("user")
