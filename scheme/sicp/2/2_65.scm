(define (union-tree tree1 tree2)
  (let ((set1 (tree->list tree1))
        (set2 (tree->list tree2)))
      (list->tree (union-set set1 set2))))

(union-tree (list->tree (list 1 2 3 4 5)) (list->tree (list 2 4 6)))

(define (intersection-tree tree1 tree2)
  (let ((set1 (tree->list tree1))
        (set2 (tree->list tree2)))
      (list->tree (intersection-set set1 set2))))

(intersection-tree (list->tree (list 1 2 3 4 5)) (list->tree (list 2 4 6)))
