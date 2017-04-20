(define (make-segment p1 p2)
  (cons p1 p2))

(define (start-segment s)
  (car s))

(define (end-segment s)
  (cdr s))

(define (make-point x y)
  (cons x y))

(define (x-point p)
  (car p))

(define (y-point p)
  (cdr p))

(define (print-point p)
  (newline)
  (display "(")
  (display (x-point p))
  (display ",")
  (display (y-point p))
  (display ")"))

(define (midpoint-segment s)
  (define (mid-point a b)
    (/ (+ a b) 2))
  (make-point (mid-point (x-point (start-segment s)) (x-point (end-segment s)))
              (mid-point (y-point (start-segment s)) (y-point (end-segment s)))))

(print-point (midpoint-segment
              (make-segment (make-point 2 4) (make-point 4 8))))
  
