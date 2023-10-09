module Searching (linear, binary, ternary) where

linearAux :: (Eq a) => [a] -> a -> Int -> Maybe Int
linearAux [] _ _ = Nothing
linearAux (curitem : items) item index
  | curitem == item = Just index
  | otherwise = linearAux items item (index + 1)

-- | The `linear` method performs linear search on a list for an item, returning an index if found, or `Nothing` if not.
linear :: (Eq a) => [a] -> a -> Maybe Int
linear items item = linearAux items item 0

binaryAux :: (Ord a) => [a] -> a -> Int -> Int -> Maybe Int
binaryAux [] _ _ _ = Nothing
binaryAux items item l r
  | l > r = Nothing
  | curitem == item = Just mid
  | item < curitem = binaryAux items item l (mid - 1)
  | item > curitem = binaryAux items item (mid + 1) r
  where
    mid = div (l + r) 2
    curitem = items !! mid

-- | The `binary` method performs binary search on a list for an item, returning an index if found, or `Nothing` if not.
binary :: (Ord a) => [a] -> a -> Maybe Int
binary items item = binaryAux items item 0 (length items - 1)

ternaryAux :: (Ord a) => [a] -> a -> Int -> Int -> Maybe Int
ternaryAux [] _ _ _ = Nothing
ternaryAux items item l r
  | l > r = Nothing
  | item1 == item = Just mid1
  | item2 == item = Just mid2
  | item < item1 = binaryAux items item l (mid1 - 1)
  | item < item2 = binaryAux items item (mid1 + 1) (mid2 - 1)
  | item > item2 = binaryAux items item (mid2 + 1) r
  where
    delta = div (r - l) 3
    mid1 = l + delta
    mid2 = r - delta
    item1 = items !! mid1
    item2 = items !! mid2

ternary :: (Ord a) => [a] -> a -> Maybe Int
ternary items item = ternaryAux items item 0 (length items - 1)
