module Sorting (merge) where

mergeAux :: (Ord a) => [a] -> [a] -> [a]
mergeAux [] [] = []
mergeAux items [] = items
mergeAux [] items = items
mergeAux (iteml:itemsl) (itemr:itemsr)
  | iteml < itemr = iteml : mergeAux itemsl (itemr:itemsr)
  | iteml > itemr = itemr : mergeAux (iteml:itemsl) itemsr
  | otherwise = iteml : itemr : mergeAux itemsl itemsr

merge :: (Ord a) => [a] -> [a]
merge [] = []
merge [item] = [item]
merge items = mergeAux sorteditemsl sorteditemsr
  where
    m = div (length items) 2
    (itemsl, itemsr) = splitAt m items
    sorteditemsl = merge itemsl
    sorteditemsr = merge itemsr
