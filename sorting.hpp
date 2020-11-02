namespace sort {

// TODO BubbleSort

// TODO SelectionSort

/**
 * Basic insertion sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void insertion_sort(T *arr, const int size) {
  for (int i = 0; i < size; i++) {

    // store currently selected element
    int const temp = arr[i];
    int index = i;

    // compare to element on the left and switcheroo if required
    while (index > 0 && temp < arr[index - 1]) {
      *(arr + index) = arr[index - 1];
      index--;
    }

    // move select element to new position
    *(arr + index) = temp;
  }
}

/**
 * Basic merge sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 * @returns Pointer to sorted array.
 */
template <typename T> T *merge_sort(T *arr, const int size) {

  // basecase
  if (size == 1)
    return arr;

  // acquire sizes
  int sizeA = size / 2;
  int sizeB = size - sizeA;

  // initialize split array pointers
  T *arrA = new T[sizeA];
  T *arrB = new T[sizeB];

  // copy arrays
  for (int i = 0, a = 0, b = 0; i < size; i++)
    i < sizeA ? *(arrA + (a++)) = arr[i] : *(arrB + (b++)) = arr[i];

  // merge sort both arrays
  arrA = merge_sort(arrA, sizeA);
  arrB = merge_sort(arrB, sizeB);

  // merging
  T *arrC = new T[size];
  int i = 0, a = 0, b = 0;
  while (a < sizeA && b < sizeB)
    arrA[a] < arrB[b] ? *(arrC + i++) = arrA[a++] : *(arrC + i++) = arrB[b++];
  while (a < sizeA)
    *(arrC + i++) = arrA[a++];
  while (b < sizeB)
    *(arrC + i++) = arrB[b++];

  // memory cleanup
  delete[] arrA;
  delete[] arrB;

  // return sorted array
  return arrC;
}

/**
 * Basic quick sort.
 *
 * @param arr Array to be sorted.
 * @param size Size of array.
 */
template <typename T> void quick_sort(T *arr, const int size) {

  // basecase
  if (size <= 1)
    return;

  // acquire pivot
  int pivotIndex = 0;
  T pivot = arr[pivotIndex];
  int finalIndex = pivotIndex + 1;

  // partition
  for (int j = pivotIndex + 1; j < size; j++) {
    if (arr[j] < pivot) {
      // if probed element is less than the pivot, swap it into the pivots final
      // index range
      T temp = arr[j];
      *(arr + j) = arr[finalIndex];
      *(arr + finalIndex++) = temp;
    }
  }

  // decrement final index to actual position
  finalIndex--;

  // place pivot in final destination
  T temp = arr[finalIndex];
  *(arr + finalIndex) = pivot;
  *(arr + pivotIndex) = temp;

  // recursively quicksort the resulting partitions
  quick_sort<T>(arr, finalIndex);
  quick_sort<T>(arr + finalIndex, size - (finalIndex + 1));
}

// TODO CountingSort

// TODO RadixSort

// TODO HeapSort

// TODO BucketSort

} // namespace sort
