(defn mage-handle-chest [chest])

(defn mage-attack [enemies]
  (if (> (cout enemies) 10)
    (do (casto-spell "Freeze Nova")
        (cast-spell "Teleport"))
    (doseq [e enemies]
      (cast-spell "Fireball" e))))

(defn move-to [character location
               & {:keys [handle-chest attack]
                  :or {handle-chest (fn [chest])
                       attack (fn [enemies (run-away)])}}])

(defmulti move
  (fn [character location] (:class character)))

(defmethod move :mage [character location]
  (move-to character location
    :handle-chest mage-handle-chest
    :attack       mage-attack))
