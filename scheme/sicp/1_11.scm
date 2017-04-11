;recursive process
(define (fr n)
  (cond ((< n 3) n)
  (else (+ (fr (- n 1))
           (* (fr (- n 2)) 2)
           (* (fr (- n 3)) 3)))))

;iterative process
(define (fi n)
  (define (f-iter a b c count max)
    (cond ((> count max) a)
          ((< max 3) n)
          (else (f-iter (+ a (* b 2) (* c 3))
                        a
                        b
                        (+ count 1)
                        max))))
  (f-iter 2 1 0 3 n))
