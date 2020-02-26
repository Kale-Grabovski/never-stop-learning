-- Generates random string of passed length.
-- Possible characters: [A-Za-z0-9]
-- Example: SELECT rand_string(16); -> nFtJoToluihDo2u7
CREATE FUNCTION rand_string(_len integer) RETURNS text LANGUAGE plpgsql
AS
$$
DECLARE
    _hash text;
BEGIN
    SELECT array_to_string(
       array(
           SELECT
               (array[
                   chr((65 + round(random() * 25))::int),
                   chr((97 + round(random() * 25))::int),
                   chr((48 + round(random() * 9))::int)
                   ])[round(random() * 2) + 1]
           INTO _hash
           FROM generate_series(1, _len)
       ), ''
    );
    RETURN _hash;
END;
$$;

