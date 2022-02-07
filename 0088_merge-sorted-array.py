class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        for a in range(n - 1, -1, -1):
            for b in range(m - 1, -1, -1):
                if nums1[b] >= nums2[a]:
                    for q in range(b, m + n):  # m + n - 1, a-1, -1):
                        nums1[q] = nums1[q - 1]
                    nums1[b - 1] = nums2[a]
                    break
                # else:
                #     nums1[b + 1] = nums2[a]
                #     break
