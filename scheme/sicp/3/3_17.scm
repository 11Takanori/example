(define (count-pairs x)
  (if (not (pair? x))
      0
      (+ (count-pairs (car x))
         (count-pairs (cdr x))
         1)))

; 3
(define x (cons 'x (cons 'x (cons 'x 'x))))

; 4
(define a (cons 'x 'x))
(define b (cons a 'y))
(define c (cons a b))
