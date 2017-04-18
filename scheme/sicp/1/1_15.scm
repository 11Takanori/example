(define (cube x) (* x x x))

(define (p x) (- (* 3 x) (* 4 (cube x))))

(define (sine angle)
  (if (not (> (abs angle) 0.1))
      angle
      (p (sine (/ angle 3.0)))))

(sine 12.15)

; (p (sine 4.05))
; (p (sine 1.3499)
; (p 0.44996)
; (- (* 3 0.44996) (* 4 (cube 0.44996)))
; (- (* 3 0.44996) (* 4 (* 0.44996 0.44996 0.44996)))
