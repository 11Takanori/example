(define (+ a b)
  (if (= a 0)
    b
    (inc (+ (dec a) b))))

;(+ 5 4)
;(inc (+ (dec 5) 4))
;(inc ((inc (+ 4 4))))
;(inc ((inc (inc (+ 3 4))))
;(inc ((inc (inc (inc (+ 2 4)))))
;(inc ((inc (inc (inc (int (+ 1 4))))))
;(inc ((inc (inc (inc (int (int (+ 0 4)))))))
;(inc ((inc (inc (inc (int 4))))))
;(inc ((inc (inc (inc 5)))))
;(inc ((inc (inc 6))))
;(inc ((inc 7)))
;(inc 8)
;(9)
;再帰的

(define (+ a b)
  (if (= a 0)
    b
    (+ (dec a) (inc b))))

;(+ 5 4)
;(+ (dec 5) (inc 4))
;(+ 4 (5)
;(+ (dec 4) (inc 5))
;(+ 3 6)
;(+ (dec 3) (inc 6))
;(+ 2 7)
;(+ (dec 2) (inc 7))
;(+ 1 8)
;(+ (dec 1) (inc 8))
;(+ 0 9)
;(9)
;反復的
