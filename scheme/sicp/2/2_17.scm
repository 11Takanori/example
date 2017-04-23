(define squares (list 1 4 9 16 25))

(define odds (list 1 3 5 7))

(define (list-ref items n)
  (if (= n 0)
      (car items)
      (list-ref (cdr items) (- n 1))))

;recursive process
(define (length items)
  (print "items=" items)
  (if (null? items)
      0
      (+ 1 (length (cdr items)))))

;iterative process
(define (length items)
  (define (length-iter a count)
    (if (null? a)
        count
        (length-iter (cdr a) (+ 1 count))))
  (length-iter items 0))

(define (append list1 list2)
  (if (null? list1)
      list2
      (cons (car list1) (append (cdr list1) list2))))

;(cons (car list1) (append (cdr list1) list2))
;(append odds squares)
;(append (1 3 5 7) (1 4 9 16 25))
;(cons (1) (append (3 5 7) (1 4 9 16 25)))
;(cons (1) (cons (3) (append (5 7) (1 4 9 16 25))))
;(cons (1) (cons (3) (cons (5) (append (7) (1 4 9 16 25)))))
;(cons (1) (cons (3) (cons (5) (cons (7) (append () (1 4 9 16 25))))))
;(cons (1) (cons (3) (cons (5) (cons (7) (1 4 9 16 25)))))
;(cons (1) (cons (3) (cons (5) (7 1 4 9 16 25))))
;(cons (1) (cons (3) (5 7 1 4 9 16 25)))
;(cons (1) (3 5 7 1 4 9 16 25))
;(1 3 5 7 1 4 9 16 25)

(define (last-pair items)
  (if (= (length items) 1)
      (car items)
      (last-pair (cdr items))))
