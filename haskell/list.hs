-- 5 chapter

-- listsum 3 => 1 + 2 + 3 => 6
listsum :: Int -> Int
listsum n = sum [x | x <- [1..n]]

-- grid 1 2 => [(0,0),(0,1),(0,2),(1,0),(1,1),(1,2)]
grid :: Int -> Int -> [(Int, Int)]
grid m n = [(x, y) | x <- [0..m], y <- [0..n]]

-- square 2 => [(0,1),(0,2),(1,0),(1,2),(2,0),(2,1)]
square :: Int -> [(Int, Int)]
square n = [(x, y) | (x, y) <- grid n n, x /= y]

-- repl 3 True => [True,True,True]
repl :: Int -> a -> [a]
repl n el = [el | _ <- [0..n]]

-- pyths 10 => [(3,4,5),(4,3,5),(6,8,10),(8,6,10)] -- x^2 + y^2 = z^2
pyths :: Int -> [(Int, Int, Int)]
pyths n = [(x, y, z) | x <- [1..n], y <- [1..n], z <- [1..n], x * x + y * y == z * z]

