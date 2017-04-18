(define (square x) (* x x))

(define (compose f g)
  (lambda (x) (f (g x))))

;recursive process
(define (repeated f n)
  (if (< n 1)
      (lambda (x) x)
      (compose f (repeated f (- n 1)))))

;iterative process
(define (repeated f n)
  (define (iter n result)
    (if (< n 1)
        result
        (iter (- n 1) (compose f result))))
  (iter n (lambda (x) x)))

((repeated square 2) 5)
