# Indexing and Searching

They are designed to find the index of an item in a collection.

## [Linear search](https://github.com/YumcoderCom/yumorithm/blob/main/src/searching/linear_search.rs)

It is a sequential search is made over all items one by one.
Every item is evaluated and if a equal to given item then index is returned,
otherwise the search continues till the end of the collection.

- Worst case performance O(n)
- Best case performance O(1)
- Average case performance O(n)

[Linear Search Visualizations](<https://www.cs.usfca.edu/~galles/visualization/Search.html>)

```
 ┌──────────────────┐
 │  -5 -2 0 7 9 10  │  search item = 7
 └───┬──▲───────────┘
     │  │
     └──┘

 ┌──────────────────┐
 │  -5 -2 0 7 9 10  │
 └──────┬─▲─────────┘
        │ │
        └─┘

 ┌──────────────────┐
 │  -5 -2 0 7 9 10  │
 └────────┬─▲───────┘
          │ │
          └─┘
```

## [Binary search](https://github.com/YumcoderCom/yumorithm/blob/main/src/searching/binary_search.rs)

It is a search algorithm that finds the position of a target value within a **sorted array**.
A binary search will start by examining the middle item.
If that item is the one we are searching for, we are done. If it is not the correct item,
we can use the ordered nature of the list to eliminate half of the remaining items. If the
item we are searching for is greater than the middle item, we know that the entire lower
half of the list as well as the middle item can be eliminated from further consideration.
The item, if it is in the list, must be in the upper half. We can then repeat the process
with the upper half. Start at the middle item and compare it against what we are looking for.
Again, we either find it or split the list in half, therefore eliminating another large part
of our possible search space.

[Linear Search Visualizations](<https://www.cs.usfca.edu/~galles/visualization/Search.html>)

 In the following example iterations to find 7 in the given array are illustrated:
 ```
 ┌──────────────────────────┐
 │  -5 -2 0 1 2 4 5 6 7 10  │   search item = 7
 └──┬─────────┬─────────┬───┘
    │         │         │
    │        mid        │
   low                 high

 ┌──────────────────────────┐
 │              4 5 6 7 10  │
 └──────────────┬───┬───┬───┘
                │   │   │
                │  mid  │
               low     high

 ┌──────────────────────────┐
 │                  6 7 10  │
 └──────────────────┬─┬─┬───┘
                    │ │ │
                    │ │ │
                   low│high
                      └─► mid
```

## [Find kth smallest item](https://github.com/YumcoderCom/yumorithm/blob/main/src/searching/kth_smallest.rs)

This approach is similar to the quick sort algorithm where we use the partition
on the input array recursively. But unlike quicksort, which processes both sides of
the array recursively, this algorithm works on only one side of the partition.
We recur for either the left or right side according to the position of pivot.

- Theoretically, this algorithm still has the complexity of O(n log n), but practically, you do not need to sort the entire array before you find k smallest elements.

```
 Partition the array A[left .. right] into two subarrays
 A[left .. pos] and A[pos + 1 .. right]
 such that each element of A[left .. pos] is less than each element of A[pos + 1 .. right]

 ┌──────────┬─┬──────────┐
 │   <= x   │x│    >= x  │
 └──────────┴─┴──────────┘

 ┌────────────────────┐
 │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
 └─┬─┬────────────────┘
   │ │
   i j

 ┌────────────────────┐
 │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
 └─┬────┬─────────────┘
   │    │
   i ─► j

 ┌────────────────────┐
 │ 6 10 13 5 8 3 2 11 │              pivot=a[0]=6
 └─┬───────┬──────────┘
   │       │
   i ────► j

             ┌─────┐
         ┌───▼─────▼──────────┐
         │ 6 5 13 10 8 3 2 11 │      pivot=a[0]=6
         └───┬─────┬──────────┘
             │     │
          ──►i     j

 ┌────────────────────┐
 │ 6 5 13 10 8 3 2 11 │              pivot=a[0]=6
 └───┬───────┬────────┘
     │       │
  ──►i       j

            ┌───────┐
      ┌─────▼───────▼──────┐
      │ 6 5 3 10 8 13 2 11 │         pivot=a[0]=6
      └─────┬───────┬──────┘
            │       │
       ───► i       j

              ┌───────┐
      ┌───────▼───────▼────┐
      │ 6 5 3 2 8 13 10 11 │         pivot=a[0]=6
      └───────┬───────┬────┘
              │       │
       ─────► i       j


 ┌────────────────────┐
 │ 6 5 3 2 8 13 10 11 │              pivot=a[0]=6
 └───────┬──────────┬─┘
         │          │
  ─────► i          j


        ┌─────┐
      ┌─▼─────▼────────────┐
      │ 2 5 3 6 8 13 10 11 │        swap pivot and x
      └───────┬──────────┬─┘
              │          │
       ─────► i          j
```