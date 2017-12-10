(defn make-coffee
  [& {:keys [name amount water milk sugar cinnamon]
      :or {name "" amount 0 water 0 milk 0 sugar 0 cinnamon 0}}]
  {:pre [(not (empty? name))
         (> amount 0)
         (> water 0)]})

(make-coffee :name     "Royale Coffee"
             :amount   15
             :water    100
             :milk     10
             :cinnamon 3)
