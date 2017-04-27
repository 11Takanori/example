(define x (list (list 1 2) (list 3 4)))

(define (reverse l)
  (if (null? (cdr l))
      l
      (append (reverse (cdr l))
              (list (car l)))))

(define (deep-reverse l)
  (if (null? (cdr l))
      (list (reverse (car l)))
      (append (deep-reverse (cdr l))
              (list (reverse (car l))))))

(define (deep-reverse l)
  (cond ((null? l) ())
        ((pair? (car l))
        (append (deep-reverse (cdr l))
                (list (deep-reverse (car l)))))
        (else
        (append (deep-reverse (cdr l))
                (list (car l))))))
(deep-reverse x)
