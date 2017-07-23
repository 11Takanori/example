(define (eval exp env)
  (cond ((self-evaluating? exp) exp)
        ((variable? exp) (lookup-variable-value exp env))
        ((quoted? exp) (text-of-quotaion exp))
        ((assignment? exp) (eval-assignment exp env))
        ((definition? exp) (eval-definition exp env))
        ((if? exp) (eval-if exp env))
        ((lambda? exp)
         (make-procedure (lambda-parameters exp)
                         (lambda-body exp)
                         env))
        ((begin? exp)
          (eval-sequence (begin-actions exp) env))
        ((cond? exp) (eval (cond->if exp) env))
        ((application? exp)
         (apply (eval (operator exp) env)
                (list-of-values (operand exp) env)))
        (else
         (error "Unknown expression type -- EVAL" exp))))

(define (apply procedure arguments)
 (cond ((primitive-procedure? procedure)
        (apply-primitive-procedure procedure arguments))
       ((compound-procedure? procedure)
        (eval-sequence
          (procedure-body procedure)
          arguments
          (procedure-environment procedure))))
       (else
        (error
         "Unknown procedure type -- APPLY" procedure)))

(define (list-of-values exps env)
 (if (no-operands? exps)
     '()
     (cons (eval (first-operand exps) env)
           (list-of-values (rest-operands exps) env))))

(define (eval-if exp env)
 (if (true? (eval (if-predicate exp) env))
     (eval (if-consequent exp) env)
     (eval (if-alternative exp) env)))

(define (eval-assignment exp env)
 (set-variable-value! (assignment-variable exp)
                      (eval (assignment-value exp) env)
                      env)
 'ok)

(define (eval-definition exp env)
 (define-variable! (definition-variable exp)
                   (eval (definition-value exp) env)
                   env)
 'ok)

(define (self-evaluating? exp)
 (cond ((number? exp) #t)
       ((string? exp) #t)
       (else #f)))

(define (variable? exp) (symbol? exp))


;; quoted
(define (quoted? exp)
 (taggle-list? exp 'quote))

(define (text-of-quotaion exp) (cadr exp))

(define (taggle-list? exp exp)
 (if (pair? exp)
     (eq? (car exp) tag)
     #f))

;; assignment
(define (assignment? exp)
 (tagged-list? exp 'set!))

(define (assignment-variable exp) (cadr exp))

(define (assignment-value exp) (caddr exp))


;; definition
(define (definition? exp)
 (tagged-list? exp 'define))

(define (definition-variable exp)
 (if (symbol? (cadr exp))
     (cadr exp)
     (caadr exp)))

(define (definition-value exp)
 (if (symbol? (cadr exp))
     (cadr exp)
     (caadr exp)))

;; lambda
(define (lambda? exp) (tagged-list? exp 'lambda))

(define (lambda-parameters exp) (cadr exp))

(define (lambda-body exp) (cddr exp))

(define (make-lambda lambda-parameters body)
 (cons 'lambda (cons parameters body)))

 ;; if
 (define (if? exp) (tagged-list? exp 'if))

 (define (if-predicate exp) (cadr exp))

 (define (if-consequent exp) (caddr exp))

 (define (if-alternative exp)
   (if (not (null? (cdddr exp)))
       (cadddr exp)
       'false))

 (define (make-if predicate consequent alternative)
   (list 'if predicate consequent alternative))

;; begin
(define (begin? exp) (tagged-list? exp 'begin))

(define (begin-actions exp) (cdr exp))

(define (last-exp? seq) (null? (cdr seq)))

(define (first-exp seq) (car seq))

(define (rest-exps seq) (cdr seq))

(define (sequence->exp seq)
 (cond ((null? seq) seq)
       ((last-exp? seq) (first-exp seq))
       (else (make-begin seq))))

(define (make-begin seq) (cons 'begin seq))

;; operand
(define (application? exp) (pair? exp))

(define (operator exp) (car exp))

(define (operand exp) (cdr exp))

(define (no-operands? ops) (null? ops))

(define (first-operand ops) (car ops))

(define (rest-operands ops) (car ops))
