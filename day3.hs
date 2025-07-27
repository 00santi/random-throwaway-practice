import Data.List (sortBy)
main = putStrLn "day 3"


elem' :: (Eq a) => a -> [a] -> Bool
elem' a [] = False
elem' a (x:xs) = if a == x then True else elem' a xs


nub' :: (Eq a) => [a] -> [a]
nub' [] = []
nub' (x:xs) = if x `elem` xs then nub' xs else x:nub' xs


asc :: [Int] -> Bool
asc [] = True
asc [_] = True
asc (x:y:xs) = x <= y && asc (y:xs)


doublePlusOne :: [Int] -> [Int]
doublePlusOne l = map (\x -> 2 * x + 1) l


myFilter :: (a -> Bool) -> [a] -> [a]
myFilter f [] = []
myFilter f (x:xs) = if f x then x:myFilter f xs else myFilter f xs


applyTwice :: (a -> a) -> a -> a
applyTwice f x = f (f x)


sortByLength :: [String] -> [String]
sortByLength l = sortBy (\a b -> compare (length a) (length b)) l


makeAdder :: Int -> (Int -> Int)
makeAdder x = (\y -> y + x)
