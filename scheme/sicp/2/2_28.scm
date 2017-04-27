(define x (list (list 1 2) (list 3 4)))

(define (fringe l)
  (cond ((null? l) ())
        ((pair? (car l)) (append (fringe (car l)) (fringe (car (cdr l)))))
        (else (append (list (car l)) (cdr l)))))

(define (fringe l)
  (cond ((null? l) l)
        ((pair? l) (append (fringe (car l))
                           (fringe (cdr l))))
        (else (list l))))

(fringe x) ; (1 2 3 4)
(fringe (list x x)) ; (1 2 3 4 1 2 3 4)
