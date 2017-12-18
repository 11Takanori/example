(def galahad {:name "Galahad"
              :speed 1.0
              :hp 100
              :attack-bow-fn attack-with-bow
              :attack-sword-fn attack-with-sword
              :block-fn block-with-sheild})

(defn make-knight-with-more-hp [knight]
  (update-in knight [:hp] + 50))

(defn make-knight-with-power-armor [knight]
  (update-in knight [:block-fn]
             (fn [block-fn]
               (fn []
                 (block-fn)
                 (block-with-power-armor)))))
