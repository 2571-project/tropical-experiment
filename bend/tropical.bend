type Tropical:
  NegInf
  Int { val: u24 }

Tropical/zero = (Tropical/Int 0)

def max(l: u24, r: u24) -> u24:
  if l < r:
    return r
  else:
    return l

Tropical/sum (l: Tropical)    (r: Tropical):     Tropical
Tropical/sum Tropical/NegInf  r                = r
Tropical/sum l                Tropical/NegInf  = l
Tropical/sum (Tropical/Int l) (Tropical/Int r) = (Tropical/Int (max l r))

Tropical/mul (l: Tropical)    (r: Tropical):     Tropical
Tropical/mul Tropical/NegInf  r                = Tropical/NegInf
Tropical/mul l                Tropical/NegInf  = Tropical/NegInf
Tropical/mul (Tropical/Int l) (Tropical/Int r) = (Tropical/Int (+ l r))

type Poly:
  Leaf { val: Tropical }
  Node { ~left, ~right }

Poly/add_id:   Poly
Poly/add_id = (Poly/Leaf Tropical/NegInf)

Poly/mul_id:   Poly
Poly/mul_id = (Poly/Leaf (Tropical/Int 0))

Poly/mul_var (var: Nat)   (poly: Poly):         Poly
Poly/mul_var _ (Poly/Leaf Tropical/NegInf) = (Poly/add_id)
Poly/mul_var (Nat/Zero)   (Poly/Leaf val)  = (Poly/Node Poly/add_id (Poly/Leaf val))
Poly/mul_var (Nat/Succ n) (Poly/Leaf val)  = (Poly/Node (Poly/mul_var n (Poly/Leaf val)) Poly/add_id)
Poly/mul_var (Nat/Zero)   poly             = (Poly/Node Poly/add_id poly)
Poly/mul_var (Nat/Succ n) (Poly/Node l r)  = (Poly/Node (Poly/mul_var n l) (Poly/mul_var n r))

Poly/eval_origin (poly: Poly):     Tropical
Poly/eval_origin (Poly/Leaf val) = val
Poly/eval_origin (Poly/Node l r) = (Tropical/sum (Poly/eval_origin l) (Poly/eval_origin r))

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
Poly/mul (Poly/Node a b) (Poly/Node c d) = (Poly/Node (Poly/mul a c) (Poly/sum (Poly/mul b c) (Poly/Node (Poly/mul a d) (Poly/mul b d))))

Poly/monomial (deg: (List Nat))                  (coef: Tropical):   Poly
Poly/monomial (List/Nil)                          _               = (Poly/add_id)
Poly/monomial (List/Cons (Nat/Zero)   List/Nil)   coef            = (Poly/Leaf coef)
Poly/monomial (List/Cons (Nat/Zero)   tail)       coef            = (Poly/Node (Poly/monomial tail coef) Poly/add_id)
Poly/monomial (List/Cons (Nat/Succ n) tail)       coef            = (Poly/Node Poly/add_id (Poly/monomial (List/Cons n tail) coef))

Poly/get (poly: Poly)    (deg: (List Nat)):               Poly
Poly/get  p              (List/Nil)                    =  p
Poly/get (Poly/Leaf val) (List/Cons (Nat/Succ n) tail) = (Poly/add_id)
Poly/get (Poly/Leaf val) (List/Cons (Nat/Zero)   tail) = (Poly/get (Poly/Leaf val) tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Zero)   tail) = (Poly/get l tail)
Poly/get (Poly/Node l r) (List/Cons (Nat/Succ n) tail) = (Poly/get r (List/Cons n tail))

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
