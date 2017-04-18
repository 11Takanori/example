;recursive process
(define (fr n)
  (cond ((< n 3) n)
  (else (+ (fr (- n 1))
           (* (fr (- n 2)) 2)
           (* (fr (- n 3)) 3)))))

;iterative process
(define (fi n)
  (define (f-iter a b c count max)
    (cond ((> count max) a)
          ((< max 3) n)
          (else (f-iter (+ a (* b 2) (* c 3))
                        a
                        b
                        (+ count 1)
                        max))))
  (f-iter 2 1 0 3 n))

; f(n)=f(n-1)+2f(n-2)+3f(n-3)
; f(1)=1
; f(2)=2
; f(3)=f(2)+2f(1)+3f(0)
;     =4
; f(4)=f(3)+2f(2)+3f(1)
;     =4+4+3
;     =11
; f(5)=f(4)+2f(3)+3f(2)
;     =11+8+6
;     =25
