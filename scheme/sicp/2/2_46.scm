(define (make-vect x y)
  (cons x y))

(define (xcor-vect v)
  (car v))

(define (ycor-vect v)
  (cdr v))

(define (add-vect vect-a vect-b)
  (make-vect
    (+ (xcor-vect vect-a) (xcor-vect vect-b))
    (+ (ycor-vect vect-a) (ycor-vect vect-b))))

(define (sub-vect vect-a vect-b)
  (make-vect
    (- (xcor-vect vect-a) (xcor-vect vect-b))
    (- (ycor-vect vect-a) (ycor-vect vect-b))))

(define (scale-vect s v)
  (make-vect
    (* s (xcor-vect v))
    (* s (ycor-vect v))))
