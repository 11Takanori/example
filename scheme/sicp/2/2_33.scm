(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
          (accumulate op initial (cdr sequence)))))

(define (map p sequence)
  (accumulate (lambda (x y) (cons (p x) y)) () sequence))

; (map square (list 1 2 3 4 5))
; (cons (square 1) (accumulate op () (2 3 4 5))))
; (cons (square 1) (cons (square 2) (accumulate op () (3 4 5))))
; (cons (square 1) (cons (square 2) (cons (square 3) (accumulate op () (4 5))))
; (cons (square 1) (cons (square 2) (cons (square 3) (cons (square 4) (accumulate op () (5))))
; (cons (square 1) (cons (square 2) (cons (square 3) (cons (square 4) (cons (square 5) ()))

(define (append seq1 seq2)
  (accumulate cons seq2 seq1))

;(append (list 1 2 3) (list 4 5 6))
;(cons (car (list 1 2 6)) (accumulate cons (list 4 5 6) (cdr (list 1 2 3))))
;(cons 1 (accumulate cons (list 4 5 6) (2 3)))
;(cons 1 (cons 2 (accumulate cons (list 4 5 6) (3))))
;(cons 1 (cons 2 (cons 3 (accumulate cons (list 4 5 6) ()))))
;(cons 1 (cons 2 (cons 3 (list 4 5 6)))

(define (length sequence)
  (accumulate (lambda (x y) (+ 1 y)) 0 sequence))
