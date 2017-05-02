(define (make-vect x y)
  (cons x y))

(define (make-frame original edge1 edge2)
  (list original edge1 edge2))

(define (original frame)
  (car frame))

(define (edge1 frame)
  (cadr frame))

(define (edge2 frame)
  (caddr frame))

(define v0 (make-vect 1 2))
(define v1 (make-vect 2 4))
(define v2 (make-vect 4 8))
(define f (make-frame v0 v1 v2))

(original f)

f ; => ((1 . 2) (2 . 4) (4 . 8))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(define (make-frame original edge1 edge2)
  (cons original (cons edge1 edge2)))

(define (origin frame)
  (car frame))

(define (edge1 frame)
  (cadr frame))

(define (edge2 frame)
  (caddr frame))

(define v0 (make-vect 1 2))
(define v1 (make-vect 2 4))
(define v2 (make-vect 4 8))
(define f (make-frame v0 v1 v2))

(original f)

f ; => ((1 . 2) (2 . 4) 4 . 8)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
(cons 1 2) ; => (1 . 2)
(cons 1 (cons 2 3)) ; => (1 2 . 3)
(cons (cons 1 2) (cons 3 4)) ; => ((1 . 2) 3 . 4)
(cons 1 (list 2 3)) ; => (1 2 3)
(cons (list 1 2) 3) ; => ((1 2) . 3)
(cons (list 1 2) (list 3 4)) ; => ((1 2) 3 4)
