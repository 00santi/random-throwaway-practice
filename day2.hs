main = putStrLn "day two"

addThree :: Int -> Int -> Int -> Int
addThree a b c = a + b + c

isEven :: Int -> Bool
isEven i = i `mod` 2 == 0

swapPair :: (x, y) -> (y, x)
swapPair (x, y) = (y, x)

applyTwice :: (a -> a) -> a -> a
applyTwice f x = f (f x)

circleArea :: Double -> Double
circleArea r = let pie = 3.14159 in pie * r * r

quadratic :: Double -> Double -> Double -> Double -> Double
quadratic a b c x =
  let ax2 = a * x * x; bx  = b * x
  in  ax2 + bx + c

bmiCategory :: Double -> Double -> String
bmiCategory weight height = 
  if bmi < 18.5 then "underweight"
  else if bmi < 25.0 then "normal"
  else "overweight"
  where bmi = weight / (height * height)

initials :: String -> String -> String
initials first last = [f, '.', l, '.']
  where
    f = head first
    l = head last

calcRoots :: Double -> Double -> Double -> (Double, Double)
calcRoots a b c = (x1, x2)
  where
    disc = sqrt (b*b - 4*a*c)
    x1   = (-b + disc) / (2 * a)
    x2   = (-b - disc) / (2 * a)

describeList :: [a] -> String
describeList []  = "empty"
describeList [_] = "one element"
describeList _   = "many elements"
