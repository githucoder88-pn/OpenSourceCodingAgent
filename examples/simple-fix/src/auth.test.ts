import { authenticate, generateToken } from './auth';

test('authenticate should return true for valid token', () => {
  const token = generateToken('user123');
  expect(authenticate(token)).toBe(true);
});
