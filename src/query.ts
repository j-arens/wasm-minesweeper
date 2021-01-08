export type GameParams = {
  cols: number;
  rows: number;
  difficulty: number;
};

export function parseQueryParams(params: string): GameParams {
  let parsed = new URLSearchParams(params);

  return {
    cols: parseAndValidateInt(parsed.get('cols'), {
      max: 50,
      min: 4,
      default: 9,
    }),

    difficulty: parseAndValidateInt(parsed.get('difficulty'), {
      max: 9,
      min: 1,
      default: 2,
    }),

    rows: parseAndValidateInt(parsed.get('rows'), {
      max: 50,
      min: 4,
      default: 9,
    }),
  };
}

type IntSchema = {
  default: number;
  max: number;
  min: number;
};

function parseAndValidateInt(value: string | null, schema: IntSchema): number {
  return Math.max(
    Math.min(Number.parseInt(`${value || schema.default}`, 10), schema.max),
    schema.min,
  );
}
