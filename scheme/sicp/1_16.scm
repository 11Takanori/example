;recursive process
(define (expt b n)
  (if (= n 0)
      1
      (* b (expt b (- n 1)))))

;(expt 2 3)
;(* 2 (expt 2 2))
;(* 2 (* 2 (expt 2 1))
;(* 2 (* 2 (* 2 (expt 2 0)))
;(* 2 (* 2 (* 2 1)))


;iterative process
(define (expt b n)
  (define (expt-iter b counter product)
    (if (= counter 0)
        product
        (expt-iter b
                   (- counter 1)
                   (* b product))))
  (expt-iter b n 1))

;(expt 2 3)
;(expt-iter 2 3 1)
;(expt-iter 2 2 2)
;(expt-iter 2 1 4)
;(expt-iter 2 0 8)
