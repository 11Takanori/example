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
