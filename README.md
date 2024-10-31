# minkcut
A rust implementation of (approximative) min k-cut algorithms with Python bindings.

## Min k-cut problem
Wikipedia: https://en.wikipedia.org/wiki/Minimum_k-cut
Generally speaking, solving the min k-cut problem amounts to partitioning a graph into k partitions (disconnected components that are exhaustive) by deleting edges while minimizing the sum of the weight of the deleted edges. There is approximative and non-approximative solutions with polynomial runtime complexity and some specialised algorithms for pariticular ks.

Introductory reading: https://towardsdatascience.com/a-simple-and-fast-deterministic-algorithm-for-the-minimum-k-way-cut-74d7ee2a521a

### Explanation

- Partition graph into three disconnected components
- Additional constraint: components have to be roughly 80-10-10% of the nodes.
- Polynomial time algorithm

- k=3 in our case
- n=4200
- n^2 edges ~16mn

Jason Li
- Main result is that their algo runs in n^[(1 + o(1))*k]
  - o(1): faster than constant? (linear in n?)
- simple graphs: 
  - at most one edge between to vertices

Vazirani & Saran
- Each algorithm finds a k-cut having weight within a factor of (2  2=k) of the optimal.
  - What's the optimal? 
    - optimal min k-cut has score = sum of all deleted edges
    - theirs: upper bound on the approximation is (2 - 2/3) * optimal

