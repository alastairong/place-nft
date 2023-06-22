export function isString(value: string | unknown): value is string {
  return value !== undefined
}

export function toByteArray(hexString: string): number[] {
    const result = []

    /* eslint-disable @typescript-eslint/no-magic-numbers */
    for (let i = 0; i < hexString.length; i += 2) {
      result.push(parseInt(hexString.substr(i, 2), 16))
    }
    /* eslint-enable @typescript-eslint/no-magic-numbers */

    return result
  }

  