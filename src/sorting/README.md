# Sorting algorithms

They are designed to put element of a list in the order

## [bubble sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

It is a comparison-based algorithm in which each pair of adjacent elements is compared
and the elements are swapped if they are not in order. After each iteration, at least one value moves at the end

- Worst case performance O(n^2)
- Best case performance O(n)

```

 ┌────────────────┐
 │ 14 33 27 35 10 │   bubble sort first iteration
 └─┬───▲──────────┘
   │   │
   └───┘

 ┌────────────────┐
 │ 14 33 27 35 10 │   27 < 33, so exchange them
 └────┬──▲────────┘
      │  │
      └──┘

 ┌────────────────┐
 │ 14 27 33 35 10 │   33 < 35
 └───────┬──▲─────┘
         │  │
         └──┘

  ┌────────────────┐
  │ 14 27 33 10 35 │  35 > 10, swapped them
  └──────────┬──▲──┘
             │  │
             └──┘
 After the first iteration the largest number be at last index of the array

```

[Visualizations](https://www.hackerearth.com/practice/algorithms/sorting/bubble-sort/visualize/)

[Visualizations](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)

## [Selection Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/se;ection_sort.rs)

It is a comparison-based algorithm in which sorts an array by repeatedly finding
the minimum element (considering ascending order) from unsorted part and putting
it at the right place

```
 ┌───────────┬───┬────────────┐
 │  sorted   │ x │  unsorted  │
 └───────────┴───┴────────────┘
 
 ┌──────────────┐
 │ 12 11 13 5 6 │ find the min 1..4
 └─┬────────────┘
   │  unsorted
   x
 
   ┌───────┐
 ┌─▼───────▼────┐
 │ 5 11 13 12 6 │  swap(5, 12)
 └────┬─────────┘
      │  unsorted
      x
 ┌──────────────┐
 │ 5 6  13 12 11│  swap(6, 11)
 └───────┬──────┘
  sorted │ unsorted
         x
 
 ┌──────────────┐
 │ 5 6 11 12 13 │ swap(11, 13)
 └─────────┬────┘
    sorted │
           x
 
 ┌──────────────┐
 │ 5 6 11 12 13 │
 └───────────┬──┘
             │
             x
```

[Visualizations](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)

## [Insertion Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/insertion_sort.rs)

Insertion Sort Algorithm
It is a comparison-based algorithm that builds the final sorted array one at a time.
The List is virtually split into a sorted and an unsorted part.
Values from the unsorted part are picked and placed at the correct
position in the sorted part.

- For small set of data it is quite efficient
- It's stable that is it does not change the relative order of elements with equal keys
- Worst case performance O(n^2)
- Best case performance O(n)

```
 ┌───────────┬───┬────────────┐
 │  sorted   │ x │  unsorted  │
 └───────────┴───┴────────────┘

 ┌──────────────┐
 │ 12 11 13 5 6 │
 └─┬────────────┘
   │  unsorted
   x

   ┌───┐
 ┌─▼───▼────────┐
 │ 12 11 13 5 6 │  from x position to start, search and put 11 in the right position
 └────┬─────────┘
      │  unsorted
      x

 ┌──────────────┐
 │ 11 12 13 5 6 │  13 will remain at its positions because all element are smaller then 13
 └───────┬──────┘
  sorted │ unsorted
         x

     ┌─────┐
 ┌───▼──▼──▼────┐
 │ 5 11 12 13 6 │
 └──────────┬───┘
    sorted  │
            x

     ┌───────┐
 ┌───▼──▼─▼──▼──┐
 │ 5 6 11 12 13 │
 └───────────┬──┘
             │
             x

```

[Visualizations](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)

## [Merge Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n)
  
[Visualizations](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)

## [Quick Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n)

[Visualizations](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)

## [Counting Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n
## [Radix Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n
## [Heap Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n
## [Bucket Sort](https://github.com/YumcoderCom/yumorithm/blob/main/src/sorting/bubble_sort.rs)

- Worst case performance O(n^2)
- Best case performance O(n
