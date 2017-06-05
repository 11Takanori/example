(define (count-pairs x)
  (define visited '())
  (define (count-pairs-sub x)
    (cond ((not (pair? x)) 0)
          ((memq x visited) 0)
          (else
            (set! visited (cons x visited))
            (+ 1
               (count-pairs-sub (car x))
               (count-pairs-sub (cdr x))))))
    (count-pairs-sub x))
