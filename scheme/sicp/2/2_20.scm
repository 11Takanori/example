(define (append list1 list2)
  (if (null? list1)
      list2
      (cons (car list1) (append (cdr list1) list2))))

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
