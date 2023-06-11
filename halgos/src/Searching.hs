module Searching (linear, binary) where

linearAux :: Eq a => [a] -> a -> Int -> Maybe Int
linearAux [] _ _ = Nothing
linearAux (curitem : items) item index
  | curitem == item = Just index
  | otherwise = linearAux items item (index + 1)

-- | The `linear` method performs linear search on a list for an item, returning an index if found, or `Nothing` if not.
linear :: Eq a => [a] -> a -> Maybe Int
linear items item = linearAux items item 0

binaryAux :: Ord a => [a] -> a -> Int -> Maybe Int
binaryAux [] _ _ = Nothing
binaryAux items item index
  | curitem == item = Just (mid + index)
  | item < curitem = binaryAux (take mid items) item index
  | item > curitem = binaryAux (drop (mid + 1) items) item (index + mid)
    where
      mid = div (length items) 2
      curitem = items !! mid

binary :: Ord a => [a] -> a -> Maybe Int
binary items item = binaryAux items item 0
