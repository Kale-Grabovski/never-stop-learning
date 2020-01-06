import Data.Char as Char

toJadenCase :: String -> String
toJadenCase js = unwords (map (\x -> Char.toUpper (head x) : tail x) (words js))

-- best: toJadenCase = unwords . map (\(x:xs) -> toUpper x:xs) . words
