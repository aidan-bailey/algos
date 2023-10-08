import Data.List (elemIndex, nub)
import Data.Maybe (isNothing)
import Searching (linear)
import Test.QuickCheck

searchTest :: ([Int] -> Int -> Maybe Int) -> [Int] -> Bool
searchTest func [] = isNothing (func [] (1 :: Int))
searchTest func items =
  func itemsunique lastitem == elemIndex lastitem itemsunique
  where
    itemsunique = nub items
    lastitem = last itemsunique

main :: IO ()
main = do
  quickCheck linearTest
  where
    linearTest = searchTest linear
