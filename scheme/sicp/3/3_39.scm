(define x 10)

(define s (make-serializer))

(parallel-execute (lambda () (set! x (* x x)))
                  (s lambda () (set! x (+ x 1))))

; 101
; 121
; 100 
