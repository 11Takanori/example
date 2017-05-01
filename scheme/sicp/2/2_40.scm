(define (enumerate-interval l h)
  (if (> l h)
      ()
      (cons l (enumerate-interval (+ l 1) h))))

(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
          (accumulate op initial (cdr sequence)))))

(define (unique-paires j i)
  (accumulate append
              ()
              (map (lambda (x)
                     (map (lambda (y) (list x y))
                          (enumerate-interval (+ x 1) i)))
                   (enumerate-interval j i))))


(unique-paires 1 4)
