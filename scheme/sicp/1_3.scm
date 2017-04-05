(define (square x) (* x x))

(define (sum-of-square x y)
  (+ (square x) (square y)))

(define (hoge x y z)
 (cond ((>= x y z) (sum-of-square x y))
       ((>= x y z) (sum-of-square x y))
       ((>= x z y) (sum-of-square x z))
       ((>= z x y) (sum-of-square x z))
       ((>= y z x) (sum-of-square y z))
       ((>= z y x) (sum-of-square y z))))
