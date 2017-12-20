(derive ::clojure-job ::job)
(derive ::java-job ::job)
(derive ::senior-clojure-job ::clojure-job)
(derive ::senior-java-job ::java-job)

(defmulti accept :job)

(defmethod accept :java [candidate]
  (and (some #{:java} (:skills candidate))
       (> (:experience candidate) 1)))
