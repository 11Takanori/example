(define (make-from-real-imag x y)
  (define (dispatch op)
    (cond ((eq? op 'real-part) x)
          ((eq? op 'imag-part) y)
          ((eq? op 'magnitude)
                (sqrt (+ (square x) (square y))))
          ((eq? op 'angle) (atan y x))
          (else
            (error "Unknown op -- MAKE-FROM-REAL-IMAG" op))))
  dispatch)

(define (apply-generic op arg) (arg op))


(define (make-from-mag-ang mag angle)
  (define (dispatch op)
    (cond ((eq? op 'real-part) (* mag (cos a)))
          ((eq? op 'imag-part) (* mag (sin a)))
          ((eq? op 'magnitude) mag)
          ((eq? op 'angle) angle)
          (else (erro "Unknown op -- MAKE-FROM-MAG-ANG" op))))
  dispatch)
