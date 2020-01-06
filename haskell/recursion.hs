-- 6 chapter

-- sumdown 3 => 3 + 2 + 1 + 0 => 6
sumdown :: Int -> Int
sumdown 0 = 0
sumdown n = n + sumdown (n - 1)

-- 2 ^ 3 = 8
eexp :: Int -> Int -> Int
eexp _ 0 = 1
eexp n t = n * eexp n (t - 1)

