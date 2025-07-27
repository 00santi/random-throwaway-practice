main = putStrLn "Hello, World!"

add :: Int -> Int -> Int
add x y = x + y
-- add 5 = (\y -> 5 + y)
addFive = add 5

multiplyThree :: Int -> Int -> Int -> Int
multiplyThree a b c = a * b * c
doubleAndMultiply :: Int -> Int
doubleAndMultiply x = multiplyThree 2 2 x

isUpperAlpha :: Char -> Bool
isUpperAlpha c = elem c ['A'..'Z']

greaterThan10 :: Int -> Bool
greaterThan10 a = a > 10


mapInc = map (\x -> 1+x)
mulByTen = map (\x -> 10*x)
