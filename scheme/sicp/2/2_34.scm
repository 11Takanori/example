(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
          (accumulate op initial (cdr sequence)))))

(define (horner-eval x coefficient-sequence)
  (accumulate (lambda (this-coff higher-terms) (+ (* higher-terms x) this-coff))
              0
              coefficient-sequence))

;(horner-eval 2 (list 1 3 0 5 0 1))
;(+ (* (accumulate op 0 (cdr (list 1 3 0 5 0 1))) 2) (car (list 1 3 0 5 0 1))
;(+ (* (accumulate op 0 (3 0 5 0 1)) 2) 1)
;(+ (* (+ (* (accumulate op 0 (0 5 0 1)) 2) 3) 2) 1)
;(+ (* (+ (* (+ (* (accumulate op 0 (5 0 1)) 2) 0) 2) 3) 2) 1)
;(+ (* (+ (* (+ (* (+ (* (accumulate op 0 (0 1)) 2) 5) 2) 0) 2) 3) 2) 1)
;(+ (* (+ (* (+ (* (+ (* (+ (* 1 2) 0) 2) 5) 2) 0) 2) 3) 2) 1)
