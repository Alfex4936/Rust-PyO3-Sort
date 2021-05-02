from bigO import BigO, algorithm
from timeit import default_timer
from rust_sort import r_bubble, r_sum, r_select


def bubble_test():
    print(">>> Bubble sort test")
    array1 = BigO.genRandomArray(7777)

    # Rust part
    start = default_timer()
    r_bubble(array1)  # inplace
    print(f"{default_timer() - start:.5f}s")

    isSorted, _ = BigO.isAscendingSorted(array1)
    assert isSorted == True

    array1 = BigO.genRandomArray(7777)

    # Python part
    start = default_timer()
    algorithm.bubbleSort(array1)  # inplace
    print(f"{default_timer() - start:.5f}s")

    isSorted, _ = BigO.isAscendingSorted(array1)
    assert isSorted == True


def selection_test():
    print(">>> Selection sort test")
    array1 = BigO.genRandomArray(7777)

    # Rust part
    start = default_timer()
    r_select(array1)  # inplace
    print(f"{default_timer() - start:.5f}s")

    isSorted, _ = BigO.isAscendingSorted(array1)
    assert isSorted == True

    array1 = BigO.genRandomArray(7777)

    # Python part
    start = default_timer()
    algorithm.selectionSort(array1)  # inplace
    print(f"{default_timer() - start:.5f}s")

    isSorted, _ = BigO.isAscendingSorted(array1)
    assert isSorted == True


def sum_test():
    print(">>> Sum test")
    l1 = [5, -1, 2, 3, 4]
    l2 = [99, 100]

    assert r_sum(l1) == 13
    assert r_sum(l2) == 199

    start = default_timer()
    sum(BigO.genRandomArray(1000000))
    print(f"{default_timer() - start:.5f}s")

    start = default_timer()
    r_sum(BigO.genRandomArray(1000000))
    print(f"{default_timer() - start:.5f}s")


if __name__ == "__main__":
    bubble_test()
    selection_test()
    sum_test()
