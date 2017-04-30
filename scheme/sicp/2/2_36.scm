(define s (list (list 1 2 3) (list 4 5 6) (list 7 8 9) (list 10 11 12)))

(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
          (accumulate op initial (cdr sequence)))))

(define (accumulate-n op initial sequence)
  (if (null? (car sequence))
      ()
      (cons (accumulate op initial (map car sequence))
            (accumulate-n op initial (map cdr sequence)))))

(accumulate-n + 0 s) ;(22 26 30)
