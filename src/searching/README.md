# Indexing and Searching

They are designed to find the index of an item in a collection.

## Linear search

It is a sequential search is made over all items one by one.
Every item is evaluated and if a equal to given item then index is returned,
otherwise the search continues till the end of the collection.

- Worst case performance O(n)
- Best case performance O(1)
- Average case performance O(n)

[Linear Search Visualizations](<https://www.cs.usfca.edu/~galles/visualization/Search.html>)

## Binary search

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

## find kth Smallest item

This approach is similar to the quick sort algorithm where we use the partition
on the input array recursively. But unlike quicksort, which processes both sides of
the array recursively, this algorithm works on only one side of the partition.
We recur for either the left or right side according to the position of pivot.

- Theoretically, this algorithm still has the complexity of O(n log n), but practically, you do not need to sort the entire array before you find k smallest elements.
