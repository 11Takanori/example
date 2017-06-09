(define (cycle? x)
  (define visited '())
  (define (iter x)
    (set! visited (cons x visited))
    (cond ((null? (cdr x)) #f)
          ((memq (cdr x) visited) #t)
          (else (iter (cdr x)))))
  (iter x))