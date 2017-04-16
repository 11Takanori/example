(define (square n) (* n n))

(define (smallest-divisor n)
  (find-divisor n 2))

(define (find-divisor n test-divisor)
  (cond ((> (square test-divisor) n) n)
        ((devides? test-divisor n) test-divisor)
        (else (find-divisor n (+ test-divisor 1)))))

(define (devides? a b)
  (= (mod b a) 0))

(define (prime? n)
  (= n (smallest-divisor n)))


(define (filtered-accumulate filter combiner null-value term a next b)
  (cond ((> a b) null-value)
        ((filter a) (combiner (term a)
                              (filtered-accumulate filter
                                                   combiner
                                                   null-value
                                                   term
                                                   (next a)
                                                    next
                                                    b)))
        (else (filtered-accumulate filter
                                   combiner
                                   null-value
                                   term
                                   (next a)
                                   next
                                   b))))
