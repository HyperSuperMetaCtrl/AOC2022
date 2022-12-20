import System.IO
import Control.Exception (handle)
import Data.List.Split
import Data.Text (replace)
import Data.List
import Data.Maybe
main = do
  -- part 2
  res <- part2
  print res
  --part 1
  -- res <- mainloop 1 []
  -- print . sum. reverse $  res


getpair = do
  end <- isEOF
  if end
    then do
    return ("f","f")
    else do
    a <- getLine
    b <- getLine
    empty <- getLine
    return (a, b)

mainloop index list = do
  a <- getpair
  if a == ("f","f") then return list
    else
    if uncurry com1 a
    then do mainloop (index+1) (index:list)
    else do mainloop (index+1) list

deleteNewlines ('\n':'\n':old)= '\n':deleteNewlines old
deleteNewlines (x:xs) = x:deleteNewlines xs
deleteNewlines "" = ""

part2 = do
  handle <- openFile "input.txt" ReadMode
  contents <- hGetContents handle
  let s1 = deleteNewlines (contents ++ "\n[[2]]\n" ++ "[[6]]")
      s2 = splitOn "\n" s1
      s3 = sortBy com2 s2
      idx1 = elemIndex "[[2]]" s3
      idx2 = elemIndex "[[6]]" s3
  case idx1 of
    Just x -> case idx2 of
                Just y -> print ((x+1)*y)
                Nothing -> print ""
    Nothing -> print ""
  -- print s3
  hClose handle


com1 [] [] = False
com1  _ [] = True
com1 [] _ = False
-- When list starts with 10 convert it to char ':' (ASCII-value = '9'+1)
com1 ('1':'0':xs) y = com1 (':':xs) y
com1 x ('1':'0':ys) = com1 x (':':ys)
-- remove paranthesis and commas
com1 ('[':xs) ('[':ys) = com1 xs ys
com1 (']':xs) (']':ys) = com1 xs ys
com1 (',':xs) y = com1 xs y
com1 x (',':ys) = com1 x  ys
-- if one list runs out before the other cases
com1 (']':xs) y         = True
com1 x        (']':ys)  = False
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

com2 [] [] = GT
com2  _ [] = LT
com2 [] _ = GT
-- 2hen list starts with 10 convert it to char ':' (ASCII-value = '9'+1)
com2 ('1':'0':xs) y = com2 (':':xs) y
com2 x ('1':'0':ys) = com2 x (':':ys)
-- 2emove paranthesis and commas
com2 ('[':xs) ('[':ys) = com2 xs ys
com2 (']':xs) (']':ys) = com2 xs ys
com2 (',':xs) y = com2 xs y
com2 x (',':ys) = com2 x  ys
-- 2f one list runs out before the other cases
com2 (']':xs) y         = LT
com2 x        (']':ys)  = GT
-- 2rap single item into a list for comparison
com2 x@('[':xs) (y:',':ys)  = com2 x              ('[':y:']':ys)
com2 x@('[':xs) (y:']':ys)  = com2 x              ('[':y:']':ys)
com2 (y:',':ys) x@('[':xs)  = com2 ('[':y:']':ys) x
com2 (y:']':ys) x@('[':xs)  = com2 ('[':y:']':ys) x
-- comapre the chars
com2 (a:xs) (b:ys)
  | right = LT
  | wrong = GT
  | otherwise = continue
  where right = a < b
        wrong = b < a
        continue = com2 xs ys
