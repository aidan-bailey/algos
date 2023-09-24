
import Test.QuickCheck

import Searching (linear, binary)
import Data.List (findIndex)


--searchTemplate :: Eq a => ([a] -> a -> Int) -> [Int] -> Bool
linearTest :: [Int] -> Bool
linearTest items
  | null items = linear items 1 == Nothing
  | otherwise = linear items (last items) == findIndex (\x -> x == last items) items

binaryTest :: [Int] -> Bool
binaryTest items
  | null items = binary items 1 == Nothing
  | Just (binary items (last items)) =
  | otherwise = (items !! binary items (last items)) == (last items)

main :: IO ()
main = do
  quickCheck binaryTest
