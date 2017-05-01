(define (enumerate-interval l h)
  (if (> l h)
      ()
      (cons l (enumerate-interval (+ l 1) h))))

(define (accumulate op i s)
  (if (null? s)
      i
      (op (car s)
          (accumulate op i (cdr s)))))

(define (unique-paires j i)
  (accumulate append
              ()
              (map (lambda (x)
                     (map (lambda (y) (list x y))
                          (enumerate-interval (+ x 1) i)))
                   (enumerate-interval j i))))


(unique-paires 1 4)
