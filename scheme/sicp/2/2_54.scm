(define (equal? list1 list2)
  (cond ((and (null? list1) (null? list2)) #t)
        ((null? list1) #f)
        ((null? list2) #f)
        (else
         (and (eq? (car list1) (car list2))
              (equal? (cdr list1) (cdr list2))))))


(equal? '(this is a list) '(this is a list))
(equal? '(this is a list) '(this (is a) list))
