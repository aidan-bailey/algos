module Searching (linear, binary) where

linearAux :: Eq a => [a] -> a -> Int -> Maybe Int
linearAux [] _ _ = Nothing
linearAux (curitem : items) item index
  | curitem == item = Just index
  | otherwise = linearAux items item (index + 1)

-- | The `linear` method performs linear search on a list for an item, returning an index if found, or `Nothing` if not.
linear :: Eq a => [a] -> a -> Maybe Int
linear items item = linearAux items item 0

binaryAux :: Ord a => [a] -> a -> Int -> Int -> Maybe Int
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
binary :: Ord a => [a] -> a -> Maybe Int
binary items item = binaryAux items item 0 (length items - 1)
