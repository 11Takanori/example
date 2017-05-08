(define (entry tree) (car tree))

(define (left-branch tree) (cadr tree))

(define (right-branch tree) (caddr tree))

(define (key record)
  (car record))

(define (value record)
  (cadr record))

(define (make-record key value)
  (list key value))

(define (lookup given-key set-of-records)
  (if (null? set-of-records)
      #f
      (let ((entry-of-record (entry set-of-records)))
           (let ((key-of-record (key entry-of-record)))
                (cond ((= given-key key-of-record) (value entry-of-record))
                      ((< given-key key-of-record) (lookup given-key (left-branch set-of-records)))
                      ((> given-key key-of-record) (lookup given-key (right-branch set-of-records))))))))
