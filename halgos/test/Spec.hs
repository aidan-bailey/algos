import Data.List (elemIndex, nub, sort)
import Data.Maybe (isNothing)
import Searching (linear, binary)
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

main :: IO ()
main = do
  quickCheck linearTest
  quickCheck binaryTest
  where
    linearTest = searchTest linear
    binaryTest = searchTest binary
