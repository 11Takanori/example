(define (inc n) (+ n 1))

(define (cube n) (* n n n))

(define (sum-cubes a b)
  (sum cube a inc b))

;recursive process
(define (sum term a next b)
  (if (> a b)
      0
      (+ (term a)
         (sum term (next a) next b))))

;iterative process
(define (sum term a next b)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (+ result (term a)))))
  (iter a 0))

(sum-cubes 1 10)
