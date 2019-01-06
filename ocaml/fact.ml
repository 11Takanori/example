let rec fact1 n = if n = 1 then 1 else fact1 (n - 1) * n

(* 
# fact1 4;;
fact1 <-- 4
fact1 <-- 3
fact1 <-- 2
fact1 <-- 1
fact1 --> 1
fact1 --> 2
fact1 --> 6
fact1 --> 24
- : int = 24
*)

let fact2 n =
  let rec iterfact (i, res) =
    if i = n then res else iterfact (i + 1, res * i)
  in
  iterfact (1, n)

(* 
# fact2 4;;
fact2 <-- 4
fact2 --> 24
- : int = 24
 *)

let fact3 n =
  let rec tailfact (n, res) =
    if n = 1 then res else tailfact (n - 1, n * res)
  in
  tailfact (n, 1)

(* # fact3 4;;
fact3 <-- 4
fact3 --> 24
- : int = 24
 *)
