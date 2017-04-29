(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
          (accumulate op initial (cdr sequence)))))

(define (cout-leave x)
  (cond ((null? x) 0)
        ((not (pair? x)) 1)
        (else (+ (cout-leave (car x))
                 (cout-leave (cdr x))))))

(define (count-leaves t)
  (accumulate + 0 (map (lambda (x) (if (pair? x) (count-leaves x) 1)) t))) 
