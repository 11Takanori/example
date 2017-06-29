(define (squarer a b)
  (define (process-new-value)
    (if (has-value? b)
        (if (< (get-value b) 0)
            (error "squarer less than 0 -- SQUARE" (get-value b))
            (set-value! a
                       (sqrt (get-value b))
                       me))
    (if (has-value? a)
        (set-value! b
                    (* (get-value a) (get-value a))
                    me))))
  (define (process-forget-value)
    (forget-value! a me)
    (forget-value! b me)
    (process-new-value))
  (define (me request)
    (cond ((eq? request 'I-have-a-value)
           (process-new-value))
          ((eq? request 'I-lost-my-value)
           (process-forget-value))
          (else
            (error "Unknown request -- SQUARE" request))))
  (connect a me)
  (connect b me)
me)

(define A (make-connector))
(define B (make-connector))

(squarer A B)

(probe 'a A)
(probe 'b B)

(set-value! B 169 'user)
