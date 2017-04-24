(define (same-parity a . z)
  (define (iter items answer)
    (if (null? items)
        answer
        (iter (cdr items)
              (if (= (mod (car items) 2)
                    (mod a 2))
                  (append answer (list (car items)))
                  answer))))
  (iter z (list a)))

(same-parity 1 2 3 4 5)
