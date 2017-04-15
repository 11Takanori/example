(define (identity x) x)

(define (inc n) (+ n 1))

(define (pi-term n)
   (if (even? n)
       (/ (+ n 2) (+ n 1))
       (/ (+ n 1) (+ n 2))))

(define (factorial x)
   (product identity 1 inc x))

;recursive process
(define (product term a next b)
  (if (> a b)
      1
      (* (term a)
         (product term (next a) next b))))

;iterative process
(define (product term a next b)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (* result (term a)))))
  (iter a 1))

(* (product pi-term 1 inc 10) 4)
