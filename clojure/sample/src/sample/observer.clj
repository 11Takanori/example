(def observers (atom #{}))

(defn add [observer]
  (swap! observers conj observer))

(defn notify [user]
  (map #(apply % user) @observers))

(add (fn [u] (mail-service/send-to-fbi u)))
(add (fn [u] (db/block-user u)))

(defn add-money [user amount]
  (swap! user
    (fn [m]
      (update-in m [:balance] + amount)))
  (if (> amount 100) (notify)))
