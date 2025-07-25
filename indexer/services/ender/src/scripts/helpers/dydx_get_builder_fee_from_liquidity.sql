CREATE OR REPLACE FUNCTION dydx_get_builder_fee(fill_liquidity text, event_data jsonb) RETURNS numeric AS $$
/**
  Returns the builder fee given the liquidity side.

  (Note that no text should exist before the function declaration to ensure that exception line numbers are correct.)
*/
BEGIN
    IF fill_liquidity = 'TAKER' THEN
        RETURN dydx_from_jsonlib_long(event_data->'takerBuilderFee');
    ELSE
        RETURN dydx_from_jsonlib_long(event_data->'makerBuilderFee');
    END IF;
END;
$$ LANGUAGE plpgsql IMMUTABLE PARALLEL SAFE;

