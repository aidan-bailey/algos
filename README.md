# Algos
A collection of common algorithms implemented in a variety of languages.

## Algorithms

### Searching

| Name           | Type               | Worst             | Average            | Best   | Space  |
|----------------|--------------------|-------------------|--------------------|--------|--------|
| Linear Search  | Brute-force        | $O(n)$            | $O(n)$             | $O(1)$ | $O(1)$ |
| Binary Search  | Decrease & Conquer | $O(\log_{2}{n})$  | $O(\log_{2}{n})$   | $O(1)$ | $O(1)$ |
| Ternary Search | Decrease & Conquer | $O(\log_{3}{n})$  | $O(\log_{3}{n})$   | $O(1)$ | $O(1)$ |
| K-ary Search   | Decrease & Conquer | $O(log_{k+1}{n})$ | $O(\log_{k+1}{n})$ | $O(1)$ | $O(1)$ |

### Sorting

| Name           | Method       | Worst             | Average           | Best              | Space            |
|----------------|--------------|-------------------|-------------------|-------------------|------------------|
| Insertion Sort | Insertion    | $O(n^2)$          | $O(n^2)$          | $O(n)$            | $O(1)$           |
| Selection Sort | Insertion    | $n^2$             | $n^2$             | $n^2$             | $O(1)$           |
| Merge Sort     | Merging      | $O(n\log_{2}{n})$ | $O(n\log_{2}{n})$ | $O(n\log_{2}{n})$ | $O(n)$           |
| Quick Sort     | Partitioning | $O(n^2)$          | $O(n\log_{2}{n})$ | $O(n\log_{2}{n})$ | $O(\log_{2}{n})$ |
| Shell Sort     | Insertion    | $O(n^{3/2})$      | $O(n^{4/3})$      | $O(n\log_{2}{n})$ | $O(1)$           |
| Bubble Sort    | Exchanging   | $O(n^2)$          | $O(n^2)$          | $O(n)$            | $O(1)$           |
| Comb Sort      | Exchanging   | $O(n^2)$          | $O(n^2)$          | $O(n\log_{2}{n})$ | $O(1)$           |
| Exchange Sort  | Exchanging   | $O(n^2)$          | $O(n^2)$          | $O(n^2)$          | $O(1)$           |

## Languages

- [ ] [https://github.com/aidanjbailey/algos/tree/master/algos-rs](Rust)
- [ ] [https://github.com/aidanjbailey/algos/tree/master/cpp-algos](C++)
- [ ] Haskell
- [ ] Python
- [ ] Julia
- [ ] Scala
