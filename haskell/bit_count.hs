countBits :: Int -> Int
countBits 0 = 0
countBits 1 = 1
countBits n = n `rem` 2 + countBits (n `div` 2)

