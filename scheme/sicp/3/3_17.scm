(define (count-pairs x)
  (if (not (pair? x))
      0
      (+ (count-pairs (car x))
         (count-pairs (cdr x))
         1)))

; 3
(define a (cons 'x (cons 'x (cons 'x 'x))))

; 4
(define a (cons 'x 'x))
(define b (cons a 'y))
(define c (cons a b))

; 7
(define a (cons 'x 'x))
(define b (cons a a))
(define c (cons b b))

; ∞
(define a (cons 'x 'x))
(define b (cons a 'x))
(set-cdr! a b)
