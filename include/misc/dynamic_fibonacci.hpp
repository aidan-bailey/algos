#ifndef __DYNAMIC_FIBONACCI_H_
#define __DYNAMIC_FIBONACCI_H_

namespace dynamic {

/**
 * Fibonacci sequence.
 *
 * @param n Index of fibonacci sequence to find
 * @return Fibonacci element n
 * */
int fibonacci(int &n) {
  int fibo[n + 1];
  fibo[0] = 1;
  fibo[1] = 1;
  for (int i = 2; i < n; i++) {
    fibo[i] = fibo[i - 1] + fibo[i - 2];
  }
  return fibo[n];
}
} // namespace dynamic

#endif // __DYNAMIC_FIBONACCI_H_
