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

(define (identity x) x)

(define (inc n) (+ n 1))

(define (gcd a b)
  (if (= b 0)
      a
      (gcd b (mod a b))))

(define (product-of-coprimes n)
  (define (coprime? i)
     (= 1 (gcd i n)))
  (filtered-accumulate coprime? * 1 identity 1 inc (- n 1)))
