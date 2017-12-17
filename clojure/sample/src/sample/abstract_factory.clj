(defn level-factory [wall-fn back-fn enemy-fn])

(defn make-stone-wall [])
(defn make-plasma-wall [])

(defn make-earth-back [])
(defn make-stars-back [])

(defn make-worm-scont [])
(defn make-ufo-soldier [])

(def underground-level-factory
  (partial level-factory
           make-stone-wall
           make-earth-back
           make-worm-scout))

(def space-level-factory
  (partial level-factory
           make-plasma-wall
           make-stars-back
           make-ufo-soldier))
