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

(define (make-rectangle start end)
  (cons start end))

(define (width rectangle)
  (let ((start (start-segment rectangle))
        (end (end-segment rectangle)))
       (abs (- (x-point start) (x-point end)))))

(define (height rectangle)
  (let ((start (start-segment rectangle))
        (end (end-segment rectangle)))
       (abs (- (y-point start) (y-point end)))))

(define (perimeter-rectangle rectangle)
  (let ((w (width rectangle))
        (h (height rectangle)))
       (* (+ w h) 2)))

(define (area-rectangle rectangle)
  (let ((w (width rectangle))
        (h (height rectangle)))
       (* w h)))

(define p1 (make-point 4 0))
(define p2 (make-point 0 2))
(define r1 (make-rectangle p1 p2))
(area-rectangle r1)
