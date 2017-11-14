(ns sample.core
  (:gen-class))

(defn stack-consuming-fibo [n]
  (cond
    (= n 0) 0
    (= n 1) 1
    :else (+ (stack-consuming-fibo (- n 1))
             (stack-consuming-fibo (- n 2)))))

(defn tail-fibo [n]
 (letfn [(fib
          [current next n]
          (if (zero? n)
            current
            (fib next (+ current next) (dec n))))]
   (fib 0N 1N n)))

(defn recur-fibo [n]
 (letfn [(fib
          [current next n]
          (if (zero? n)
            current
            (recur next (+ current next) (dec n))))]
   (fib 0N 1N n)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!")
  (println (stack-consuming-fibo 10)))
