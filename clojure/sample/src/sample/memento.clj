(def textbox (atom {}))

(defn init-textbox []
  (reset! textbox {:text ""
                   :colorr :BLACK
                   :width 100}))

(def memento (atom nil))

(defn type-text [text]
  (swap! textbox
    (fn [m]
      (update-in m [:text] (fn [s] (str s text))))))

(defn save []
  (reset! memento (:text @textbox)))

(defn restore []
  (swap! textbox assoc :text @memento))

(init-textbox)
(type-text "A")
(type-text "B")
(save)
(type-text "C")
;; crach
(init-textbox)
(restore)
