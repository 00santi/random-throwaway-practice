main = putStrLn "day 1 practice problems"


double_me x = x + x
doubleMe x = x * 2

double_us x y = double_me x + double_me y
doubleUs x y = x + x + y + y

boom_bangs list = [ if x < 10 then "BANG" else "BOOM" | x <- list ,odd x ]

my_abs x = if x < 0 then -1 * x else x

evens = [ x | x <- [1..100], even x ]
squares = [ x * x | x <- [1..10] ]
len list = sum [1 | _ <- list]
remove_lowercase str = [ c | c <- str, c `elem` ['A'..'Z']] 

add_vectors v1 v2 = (fst v1 + fst v2, snd v1 + snd v2)
first t = fst t

zip_plus l1 l2 = [ x + y | (x, y) <- zip l1 l2 ]

bmis l1 = [ fst x / (snd x * snd x) | x <- l1 ]
