(defmulti interpret class)

(defmethod interpret java.lang.Long [n]
  (str "i" n "e"))

(defmethod interpret java.lang.String [n]
  (str (count s) ":" s))

(defmethod interpret clojure.lang.PersistentVector [v]
  (str "l"
       (apply str (map interpret v))
    "e"))

(defmethod interpret clojure.lang.PersistentArrayMap [m]
  (str "d"
       (apply str (map (fn [[k v]]
                        (str (interpret k)
                             (interpret v))) m))
       "e"))

       
(interpret {"user" "Bertie"}
           "number_of_downloaded_torrents" 623
     "number_of_uploaded_torrent" 0
     "donation_in_dollars" 0
     "preffered_categories" ["porn"
                                      "murder"
                             "scala"]
                 "pokemons")
