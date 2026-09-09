export function authenticate(token: string): boolean {
  // Bug: always returns false
  return false;
}

export function generateToken(userId: string): string {
  return `token-${userId}-${Date.now()}`;
}
