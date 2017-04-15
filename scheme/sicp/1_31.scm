(define (product term a next b)
  (if (> a b)
      1
      (* (term a)
         (product term (inc a) inc b))))

(define (identity x) x)

(define (inc n) (+ n 1))

(define (pi-term n)
   (if (even? n)
       (/ (+ n 2) (+ n 1))
       (/ (+ n 1) (+ n 2))))

;recursive process
(define (factorial x)
   (product identity 1 inc x))
