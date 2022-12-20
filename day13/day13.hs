com1 [] [] = True
-- When list starts with 10 convert it to char ':' (ASCII-value = '9'+1)
com1 ('1':'0':xs) y = com1 (':':xs) y
com1 x ('1':'0':ys) = com1 x (':':ys)
-- remove paranthesis and commas
com1 ('[':xs) ('[':ys) = com1 xs ys
com1 (']':xs) (']':ys) = com1 xs ys
com1 (',':xs) (',':ys) = com1 xs ys
-- if one list runs out before the other cases
com1 (']':xs) (y:ys)    = True
com1 (x:xs)   (']':ys)  = False
-- wrap single item into a list for comparison
com1 x@('[':xs) (y:',':ys)  = com1 x              ('[':y:']':ys)
com1 x@('[':xs) (y:']':ys)  = com1 x              ('[':y:']':ys)
com1 (y:',':ys) x@('[':xs)  = com1 ('[':y:']':ys) x
com1 (y:']':ys) x@('[':xs)  = com1 ('[':y:']':ys) x
-- comapre the chars
com1 (a:xs) (b:ys)
  | right = True
  | wrong = False
  | otherwise = continue
  where right = a < b
        wrong = b < a
        continue = com1 xs ys

