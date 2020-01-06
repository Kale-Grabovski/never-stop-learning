halve :: [a] -> ([a], [a])
halve xs = (take l xs, drop l xs)
    where l = length xs `div` 2
