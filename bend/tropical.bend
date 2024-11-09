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

write_u24/go (n: u24) (acc: (List u24)): (List u24)
write_u24/go  n        acc             =
  if (> n 10) {
    (write_u24/go (/ n 10) (List/Cons (+ (% n 10) 48) acc))
  } else {
    (List/Cons (+ (% n 10) 48) acc)
  }

write_u24 (n: u24):  String
write_u24  n      = (String/decode_utf8 (write_u24/go n List/Nil))

Tropical/write (trop: Tropical):   String
Tropical/write  Tropical/NegInf  = "-∞"
Tropical/write (Tropical/Int n)  = (write_u24 n)

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

Poly/mul (l: Poly)       (r: Poly):         Poly
Poly/mul (Poly/Leaf Tropical/NegInf) _   = (Poly/Leaf Tropical/NegInf)
Poly/mul  _  (Poly/Leaf Tropical/NegInf) = (Poly/Leaf Tropical/NegInf)
Poly/mul (Poly/Leaf lll) (Poly/Leaf rrr) = (Poly/Leaf (Tropical/mul lll rrr))
Poly/mul (Poly/Leaf val) (Poly/Node l r) = (Poly/Node (Poly/mul (Poly/Leaf val) l) (Poly/mul (Poly/Leaf val) r))
Poly/mul (Poly/Node l r) (Poly/Leaf val) = (Poly/Node (Poly/mul (Poly/Leaf val) l) (Poly/mul (Poly/Leaf val) r))
Poly/mul (Poly/Node a b) (Poly/Node c d) = (Poly/Node (Poly/mul a c) (Poly/Node (Poly/sum (Poly/mul a d) (Poly/mul b c)) (Poly/mul b d)))
# example calculation for the rule above:
#
#                       (l)                  (*)                 (r)
#           (a)                     (b)       |         (c)             (d)
#       (w)     (x)             (y)     (z)   |     (g)     (i)     (j)     (k)
#   |----------------------------------------------------------------------------|
#
#       (* l r) = (* (V a b) (V c d))
#               = (V (* a c) (V (+ (* a d) (* b c)) (* b d)))
#
#       (* a c) = (* (V w x) (V g i))
#               = (V (* w g) (V (+ (* w i) (* x g)) (* x i)))
#
#       (* b d) = (* (V y z) (V j k))
#               = (V (* y j) (V (+ (* y k) (* z j)) (* z k)))
#
#       (* a d) = (* (V w x) (V j k))
#               = (V (* w j) (V (+ (* w k) (* x j)) (* x k)))
#               = s
#
#       (* b c) = (* (V y z) (V g i))
#               = (V (* y g) (V (+ (* y i) (* z g)) (* z i)))
#               = t
#
#       (+ s t) = (+ ((V (* w j) (V (+ (* w k) (* x j)) (* x k))) (V (* y g) (V (+ (* y i) (* z g)) (* z i))))
#               = (V (+ (* w j) (* y g)) (+ (V (+ (* w k) (* x j)) (* x k)) (V (+ (* y i) (* z g)) (* z i))))
#               = (V (+ (* w j) (* y g)) (V (+ (+ (* w k) (* x j)) (+ (* y i) (* z g))) (+ (* x k) (* z i))))
#
#       (* l r) = (V (* a c) (V (+ s t) (* b d)))
#               = (V
#                   (V
#                       (* w g)
#                       (V
#                           (+ (* w i) (* x g))
#                           (* x i)
#                       )
#                   )
#                   (V
#                       (V
#                           (+ (* w j) (* y g))
#                           (V
#                               (+ (+ (* w k) (* x j)) (+ (* y i) (* z g)))
#                               (+ (* x k) (* z i))
#                           )
#                       )
#                       (V
#                           (* y j)
#                           (V
#                               (+ (* y k) (* z j))
#                               (* z k)
#                           )
#                       )
#                   )
#               )

# todo: make this parallel (i.e. balance the expression tree, that is currently a list)
Poly/pow (poly: Poly) (exp: Nat):     Poly
Poly/pow  p            Nat/Zero    = (Poly/mul_id)
Poly/pow  p           (Nat/Succ n) = (Poly/mul p (Poly/pow p n))

Poly/eval_origin (poly: Poly):     Tropical
Poly/eval_origin (Poly/Leaf val) = val
Poly/eval_origin (Poly/Node l r) = (Tropical/sum (Poly/eval_origin l) (Poly/eval_origin r))

Poly/get (poly: Poly)    (deg: (List Nat)):               Poly
Poly/get  p              (List/Nil)                    =  p
Poly/get (Poly/Leaf val) (List/Cons (Nat/Succ n) tail) = (Poly/add_id)
Poly/get (Poly/Leaf val) (List/Cons (Nat/Zero)   tail) = (Poly/get (Poly/Leaf val) tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Zero)   tail) = (Poly/get l tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Succ n) tail) = (Poly/get r (List/Cons n tail))

# todo: complete this
# Poly/write/go (p: Poly)       (deg: (List Nat)): ((List u24) -> (List u24))
# Poly/write/go (Poly/Leaf val)  deg             = (DiffList/cons ())
# Poly/write/go (Poly/Node l r)  deg             = (DiffList/concat (Poly/write/go l (Nat/Succ var)) (Poly/write/go r var))

# Poly/write (p: Poly):  String
# Poly/write  p       = (String/decode_utf8 (DiffList/to_list (Poly/write/go p Nat/Zero)))

main = (Poly/get
  (Poly/mul
    (Poly/monomial [(Nat/Succ Nat/Zero), (Nat/Succ (Nat/Succ Nat/Zero))] (Tropical/Int 2))
    (Poly/sum
      (Poly/monomial [(Nat/Succ Nat/Zero), (Nat/Succ Nat/Zero)] (Tropical/Int 1))
      (Poly/monomial [(Nat/Succ Nat/Zero), (Nat/Succ (Nat/Succ Nat/Zero))] (Tropical/Int 2))
    )
  )
  [(Nat/Succ (Nat/Succ Nat/Zero)), (Nat/Succ (Nat/Succ (Nat/Succ (Nat/Succ Nat/Zero))))]
)
