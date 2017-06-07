(define (cyclic? x)
  (define (loop? x y)
    (cond ((not (pair? x)) #f)
          ((eq? x y) #t)
          ((not (pair? (cdr x))) #f)
          (else
            (loop? (cddr x) (cdr y)))))
  (cond ((not (pair? x)) #f)
        (else
          (loop? (cdr x) x))))
