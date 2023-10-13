import Data.List (elemIndex, nub, sort)
import Data.Maybe (isNothing)
import Searching (linear, binary, ternary)
import Sorting (merge)
import Test.QuickCheck

searchTest :: ([Int] -> Int -> Maybe Int) -> [Int] -> Bool
searchTest func [] = isNothing (func [] (1 :: Int))
searchTest func items =
  func itemsunique lastitem == elemIndex lastitem itemsunique
    && func itemsunique firstitem == elemIndex firstitem itemsunique
  where
    itemsunique = sort (nub items)
    lastitem = last itemsunique
    firstitem = head itemsunique

sortTest :: ([Int] -> [Int]) -> [Int] -> Bool
sortTest func items = func items == sort items


main :: IO ()
main = do
  quickCheck linearTest
  quickCheck binaryTest
  quickCheck ternaryTest
  quickCheck mergeTest
  where
    linearTest = searchTest linear
    binaryTest = searchTest binary
    ternaryTest = searchTest ternary
    mergeTest = sortTest merge
