(define (make-leaf symbol weight)
  (list 'leaf symbol weight))

(define (leaf? object)
  (eq? (car object) 'leaf))

(define (symbol-leaf x) (cadr x))

(define (weight-leaf x) (caddr x))

(define (make-code-tree left right)
  (list left
        right
        (append (symbols left) (symbols right))
        (+ (weight left) (weight right))))

(define (left-branch tree) (car tree))

(define (right-branch tree) (cadr tree))

(define (symbols tree)
  (if (leaf? tree)
      (list (symbol-leaf tree))
      (caddr tree)))

(define (weight tree)
  (if (leaf? tree)
      (weight-leaf tree)
      (cadddr tree)))

(define (encode message tree)
  (if (null? message)
      ()
      (append (encode-symbol (car message) tree)
              (encode (cdr message) tree))))

(define (encode-symbol char tree)
  (define (list-have? l e)
    (cond ((null? l) #f)
          ((equal? (car l) e) #t)
          (else (list-have? (cdr l) e))))
  (if (leaf? tree)
      ()
      (cond ((list-have? (symbols (left-branch tree)) char)
             (cons 0 (encode-symbol char (left-branch tree))))
            ((list-have? (symbols (right-branch tree)) char)
             (cons 1 (encode-symbol char (right-branch tree))))
            (else (error "error")))))

(define (encode-symbol symbol tree)
  (define (enc-iter tree)
    (if (leaf? tree)
        ()
        (if (memq symbol (symbols (left-branch tree)))
            (cons 0 (enc-iter (left-branch tree)))
            (cons 1 (enc-iter (right-branch tree))))))
  (if (memq symbol (symbols tree))
      (enc-iter tree)
      (error "error")))

(define sample-tree
  (make-code-tree (make-leaf 'A 4)
                  (make-code-tree
                    (make-leaf 'B 2)
                    (make-code-tree (make-leaf 'D 1)
                                    (make-leaf 'C 1)))))

(encode '(A D A B B C A) sample-tree)
