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

;; mutable pattern
(defprotocol IStringBuilder
  (append [this s])
  (to-string [this]))

(deftype ClojureStringBuilder [charray ^:volatile-mutable last-pos]
  IStringBuilder
  (append [this s]
    (let [cs (char-array s)]
      (doseq [i (range (count cs))]
        (aset charray (+ last-pos i) (aget cs i))))
    (set! last-pos (+ last-pos (count s))))
  (to-string [this] (apply str (take last-pos charray))))

(defn new-string-builder []
  (ClojureStringBuilder. (char-array 100) 0))

(def sb (new-string-builder))
(append sb "Toby Wong")
(to-string sb)
(append sb " ")
(append sb "Toby Chung")
