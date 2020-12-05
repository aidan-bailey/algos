#ifndef __DYNAMIC_FIBONACCI_H_
#define __DYNAMIC_FIBONACCI_H_

namespace misc {

/**
 * Fibonacci sequence.
 *
 * @param n Index of fibonacci sequence to find
 * @return Fibonacci element n
 * */
int dynamic_fibonacci(int &n) {
  int fibo[n + 1];
  fibo[0] = 1;
  fibo[1] = 1;
  for (int i = 2; i < n; i++) {
    fibo[i] = fibo[i - 1] + fibo[i - 2];
  }
  return fibo[n - 1];
}
} // namespace misc

#endif // __DYNAMIC_FIBONACCI_H_
