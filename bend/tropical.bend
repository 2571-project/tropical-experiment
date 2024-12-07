type Tropical:
  NegInf
  Int { val: u24 }

Tropical/zero = (Tropical/Int 0)

max (l: u24) (r: u24): u24
max  l        r      = if (< l r) { r } else { l }

Tropical/sum (left: Tropical)  (rght: Tropical):   Tropical
Tropical/sum  Tropical/NegInf   rght            =  rght
Tropical/sum  left              Tropical/NegInf =  left
Tropical/sum (Tropical/Int l)  (Tropical/Int r) = (Tropical/Int (max l r))

Tropical/mul (left: Tropical) (rght: Tropical):   Tropical
Tropical/mul  Tropical/NegInf  rght            =  Tropical/NegInf
Tropical/mul  left             Tropical/NegInf =  Tropical/NegInf
Tropical/mul (Tropical/Int l) (Tropical/Int r) = (Tropical/Int (+ l r))

write_u24/list (n: u24) (acc: (List u24)): (List u24)
write_u24/list  n        acc             =
  if (>= n 10) {
    (write_u24/list (/ n 10) (List/Cons (+ (% n 10) 48) acc))
  } else {
    (List/Cons (+ (% n 10) 48) acc)
  }

write_u24/diff_list (n: u24): ((List u24) -> (List u24))
write_u24/diff_list  n      =
  if (>= n 10) {
    (DiffList/append (write_u24/diff_list (/ n 10)) (+ (% n 10) 48))
  } else {
    (DiffList/wrap (+ (% n 10) 48))
  }

write_u24 (n: u24):  String
write_u24  n      = (String/decode_utf8 (write_u24/list n List/Nil))

Tropical/write/diff_list (trop: Tropical):  ((List u24) -> (List u24))
Tropical/write/diff_list Tropical/NegInf  = (DiffList/cons (DiffList/cons (DiffList/cons (DiffList/wrap 158) 136) 226) 45)
Tropical/write/diff_list (Tropical/Int n) = (write_u24/diff_list n)

Tropical/write (trop: Tropical):   String
Tropical/write  Tropical/NegInf  = "-∞"
Tropical/write (Tropical/Int n)  = (write_u24 n)

# readme: an important property to formalize is (Node (Leaf a) (NegInf)) = (Leaf a)
# but it's not true in (Node (Node a b) (NegInf)) because that would unshift the degree
# of b
type Poly:
  Leaf { val: Tropical }
  Node { ~left, ~right }

Poly/add_id:   Poly
Poly/add_id = (Poly/Leaf Tropical/NegInf)

Poly/mul_id:   Poly
Poly/mul_id = (Poly/Leaf (Tropical/Int 0))

Poly/mul_var (var: Nat)   (poly: Poly):       Poly
Poly/mul_var _ (Poly/Leaf Tropical/NegInf) = (Poly/add_id)
Poly/mul_var (Nat/Zero)   (Poly/Leaf val)  = (Poly/Node Poly/add_id (Poly/Leaf val))
Poly/mul_var (Nat/Succ n) (Poly/Leaf val)  = (Poly/Node (Poly/mul_var n (Poly/Leaf val)) Poly/add_id)
Poly/mul_var (Nat/Zero)   poly             = (Poly/Node Poly/add_id poly)
Poly/mul_var (Nat/Succ n) (Poly/Node l r)  = (Poly/Node (Poly/mul_var n l) (Poly/mul_var n r))

Poly/monomial (deg: (List Nat))                  (coef: Tropical):   Poly
Poly/monomial (List/Nil)                          _               = (Poly/add_id)
Poly/monomial (List/Cons (Nat/Zero)   List/Nil)   coef            = (Poly/Leaf coef)
Poly/monomial (List/Cons (Nat/Zero)   tail)       coef            = (Poly/Node (Poly/monomial tail coef) Poly/add_id)
Poly/monomial (List/Cons (Nat/Succ n) tail)       coef            = (Poly/Node Poly/add_id (Poly/monomial (List/Cons n tail) coef))

Poly/sum (l: Poly)       (r: Poly):        Poly
Poly/sum (Poly/Leaf Tropical/NegInf)   r = r
Poly/sum l   (Poly/Leaf Tropical/NegInf) = l
Poly/sum (Poly/Leaf lll) (Poly/Leaf rrr) = (Poly/Leaf (Tropical/sum lll rrr))
Poly/sum (Poly/Leaf val) (Poly/Node l r) = (Poly/Node (Poly/sum l (Poly/Leaf val)) r)
Poly/sum (Poly/Node l r) (Poly/Leaf val) = (Poly/Node (Poly/sum l (Poly/Leaf val)) r)
Poly/sum (Poly/Node a b) (Poly/Node c d) = (Poly/Node (Poly/sum a c) (Poly/sum b d))

Nat/sum (l: Nat)    (r: Nat):   Nat
Nat/sum (Nat/Zero)   r       =  r
Nat/sum (Nat/Succ l) r       = (Nat/Succ (Nat/sum l r))

Degree/sum (l: (List Nat))   (r: (List Nat)):    (List Nat)
Degree/sum (List/Nil)         r                =  r
Degree/sum  l                (List/Nil)        =  l
Degree/sum (List/Cons lh lt) (List/Cons rh rt) = (List/Cons (Nat/sum lh rh) (Degree/sum lt rt))

# todo(?): create a datatype equivalent to bool but with variants Left & Right
Degree/step (deg: (List Nat))  (dir: u24):  (List Nat)
Degree/step (List/Nil)          dir       = (Degree/step (List/Cons Nat/Zero List/Nil) dir)
Degree/step (List/Cons n tail)  dir       =
 if (== 0 dir) {
   (List/Cons Nat/Zero (List/Cons n tail))
 } else {
   (List/Cons (Nat/Succ n) tail)
 }

Poly/inner_mul (lp: Poly) (ld: (List Nat)) (rp: Poly) (rd: (List Nat)):  Poly
Poly/inner_mul (Poly/Leaf lll) ld (Poly/Leaf rrr) rd = (Poly/monomial
  (Degree/sum (List/reverse ld) (List/reverse rd))
  (Tropical/mul lll rrr)
)
Poly/inner_mul lp ld (Poly/Node a b) rd = (Poly/sum
  (Poly/inner_mul lp ld a (Degree/step rd 0))
  (Poly/inner_mul lp ld b (Degree/step rd 1))
)
Poly/inner_mul (Poly/Node a b) ld rp rd = (Poly/sum
  (Poly/inner_mul a (Degree/step ld 0) rp rd)
  (Poly/inner_mul b (Degree/step ld 1) rp rd)
)

Poly/mul (l: Poly) (r:Poly):  Poly
Poly/mul  l         r      = (Poly/inner_mul l (List/Cons Nat/Zero List/Nil) r (List/Cons Nat/Zero List/Nil))

# todo: make this parallel (i.e. balance the expression tree, that is currently a list)
Poly/pow (poly: Poly) (exp: Nat):     Poly
Poly/pow  p            Nat/Zero    = (Poly/mul_id)
Poly/pow  p           (Nat/Succ n) = (Poly/mul p (Poly/pow p n))

Poly/eval_origin (poly: Poly):     Tropical
Poly/eval_origin (Poly/Leaf val) = val
Poly/eval_origin (Poly/Node l r) = (Tropical/sum (Poly/eval_origin l) (Poly/eval_origin r))

Poly/get (poly: Poly)    (deg: (List Nat)):               Poly
Poly/get  poly           (List/Nil)                    =  poly
Poly/get (Poly/Leaf val) (List/Cons (Nat/Succ n) tail) = (Poly/add_id)
Poly/get (Poly/Leaf val) (List/Cons (Nat/Zero)   tail) = (Poly/get (Poly/Leaf val) tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Zero)   tail) = (Poly/get l tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Succ n) tail) = (Poly/get r (List/Cons n tail))

Nat/to_u24 (nat: Nat):   u24
Nat/to_u24  Nat/Zero    = 0
Nat/to_u24 (Nat/Succ n) = (+ 1 (Nat/to_u24 n))

# readme: ideally this would know if it's a 3+ var poly to then only print x_i
# but that would require a bit of refactoring
Poly/write/var (var: u24): ((List u24) -> (List u24))
Poly/write/var  var      = switch var {
  0: (DiffList/wrap 120)
  1: (DiffList/wrap 121)
  2: (DiffList/wrap 122)
  _: (DiffList/cons 120 (DiffList/cons 95 (write_u24/diff_list var)))
}

Poly/write/degree (deg: Nat): ((List u24) -> (List u24))
Poly/write/degree  deg = (DiffList/cons (write_u24/diff_list (Nat/to_u24 deg)) 94)

Poly/write/monomial (var: u24) (deg: (List Nat)):            ((List u24) -> (List u24))
Poly/write/monomial  _         (List/Nil)                  = (DiffList/new)
Poly/write/monomial  var       (List/Cons (Nat/Zero)   dt) = (Poly/write/monomial (+ 1 var) dt)
Poly/write/monomial  var       (List/Cons (Nat/Succ n) dt) = (DiffList/concat
  (Poly/write/var var)
  (DiffList/concat
    (Poly/write/degree (Nat/Succ n))
    (Poly/write/monomial (+ 1 var) dt)
  )
)

Poly/write/crawl (p: Poly)                    (deg: (List Nat)): ((List u24) -> (List u24))
Poly/write/crawl (Poly/Leaf  Tropical/NegInf)  _               = (DiffList/new)
Poly/write/crawl (Poly/Leaf (Tropical/Int n))  deg             = (DiffList/concat
  (write_u24/diff_list n)
  (Poly/write/monomial 0 (List/reverse deg))
)
Poly/write/crawl (Poly/Node a b)               deg             = (DiffList/concat
  (Poly/write/crawl a (Degree/step deg 0))
  (DiffList/concat
    (DiffList/cons (DiffList/cons (DiffList/wrap 32) 43) 32)
    (Poly/write/crawl b (Degree/step deg 1))
  )
)

Poly/write (p: Poly):  String
Poly/write  p       = (String/decode_utf8 (DiffList/to_list (Poly/write/crawl p List/Nil)))

main = (Poly/write
  (Poly/mul
    (Poly/pow
      (Poly/Node
        (Poly/Node
          (Poly/Leaf (Tropical/Int 0))
          (Poly/Leaf (Tropical/Int 3))
        )
        (Poly/Node
          (Poly/Node
            (Poly/Leaf (Tropical/Int 5))
            (Poly/Leaf (Tropical/Int 1))
          )
          (Poly/Leaf (Tropical/Int 1))
        )
      )
      (Nat/Succ (Nat/Succ Nat/Zero))
    )
    (Poly/pow
      (Poly/Node
        (Poly/Node
          (Poly/Leaf (Tropical/Int 3))
          (Poly/Leaf (Tropical/Int 0))
        )
        (Poly/Node
          (Poly/Leaf (Tropical/Int 1))
          (Poly/Leaf (Tropical/Int 5))
        )
      )
      (Nat/Succ (Nat/Succ (Nat/Succ Nat/Zero)))
    )
  )
)
