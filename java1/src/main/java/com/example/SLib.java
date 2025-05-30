package com.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;

public class SLib {

    // 554. 砖墙
    static int leastBricks(List<List<Integer>> wall) {
        int h = wall.size();
        HashMap<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < h; i++) {
            int s = 0;
            for (int j = 0; j < wall.get(i).size() - 1; j++) {
                s += wall.get(i).get(j);
                map.put(s, map.getOrDefault(s, 0) + 1);
            }
        }
        int max = 0;
        for (int v : map.values()) {
            max = Math.max(max, v);
        }
        return h - max;
    }

    // 25. K 个一组翻转链表
    static ListNode reverseKGroup(ListNode head, int k) {
        if (head.next == null) {
            return head;
        }
        Deque<ListNode> q = new LinkedList<>();
        ListNode node = head;
        ListNode newHead = new ListNode(0);
        ListNode newNode = newHead;
        while (node != null) {
            ListNode next = node.next;
            node.next = null;
            q.push(node);
            if (q.size() == k) {
                while (!q.isEmpty()) {
                    newNode.next = q.pop();
                    newNode = newNode.next;
                }
            }
            node = next;
        }
        while (!q.isEmpty()) {
            newNode.next = q.pollLast();
            newNode = newNode.next;
        }
        return newHead.next;
    }

    // 61. 旋转链表
    static ListNode rotateRight(ListNode head, int k) {
        ListNode n = head;
        int len = 0;
        while (n != null) {
            len++;
            n = n.next;
        }
        if (len == 0) {
            return head;
        }
        int k0 = k % len;
        if (k0 < 1) {
            return head;
        }
        k0 = len - k0;
        n = head;
        ListNode nh = null;
        for (int i = 1; i < len + 1; i++) {
            if (i == k0) {
                ListNode t = n.next;
                n.next = null;
                n = t;
                nh = n;
                continue;
            }
            if (i == len) {
                n.next = head;
            }
            n = n.next;
        }

        return nh;
    }

    // 116. 填充每个节点的下一个右侧节点指针
    static Node connect(Node root) {
        if (root == null) {
            return root;
        }

        Queue<Node> queue = new LinkedList<>();
        queue.add(root);

        while (!queue.isEmpty()) {
            int size = queue.size();

            for (int i = 0; i < size; i++) {
                Node node = queue.poll();

                if (i < size - 1) {
                    node.next = queue.peek();
                }

                if (node.left != null) {
                    queue.add(node.left);
                }
                if (node.right != null) {
                    queue.add(node.right);
                }
            }
        }

        return root;
    }

    // 946. 验证栈序列
    static boolean validateStackSequences(int[] pushed, int[] popped) {
        List<Integer> res = new LinkedList<>();
        for (int left = 0, right = 0; left < pushed.length; left++) {
            res.add(pushed[left]);
            while (!res.isEmpty() && res.getLast() == popped[right]) {
                res.removeLast();
                right++;
            }
        }
        return res.isEmpty();
    }

    // 899. 有序队列
    static String orderlyQueue(String s, int k) {
        if (k == 1) {
            String best = s;
            StringBuilder sb = new StringBuilder(s);
            int len = s.length();
            for (int i = 0; i < len; i++) {
                char c = sb.charAt(0);
                sb.deleteCharAt(0);
                sb.append(c);
                if (sb.toString().compareTo(best) < 0) {
                    best = sb.toString();
                }
            }
            return best;
        } else {
            char[] arr = s.toCharArray();
            Arrays.sort(arr);
            return new String(arr);
        }
    }

    // 406. 根据身高重建队列
    static int[][] reconstructQueue(int[][] people) {
        Arrays.sort(people, (a, b) -> b[0] != a[0] ? Integer.compare(b[0], a[0]) : Integer.compare(a[1], b[1]));
        List<int[]> list = new LinkedList<>();
        for (int i = 0; i < people.length; i++) {
            list.add(people[i][1], people[i]);
        }
        return list.toArray(new int[list.size()][]);
    }

    // 移除元素
    // 移除等于val的元素，然后返回不等于val的元素的个数
    static int removeElement(int[] nums, int val) {
        int len = nums.length;
        int k = 0;

        int one = 0;
        int two = 0;
        while (two < len) {
            if (nums[two] != val) {
                nums[one++] = nums[two];
                k++;
            }
            two++;
        }

        return k;
    }

    // 最接近的三数之和
    static int threeSumClosest(int[] nums, int target) {
        Arrays.sort(nums);
        int len = nums.length;
        int best = Integer.MAX_VALUE;

        // 先确定一个数a，遍历数组获取a: nums[i]
        for (int i = 0; i < len; i++) {
            // 重复的a跳过
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }

            // 已经对数组进行过排序，所以a之前的数据都是重复的，
            // b 为 a 的下一个，c 从末尾读取
            int b = i + 1, c = len - 1;
            while (b < c) {
                // 计算当前结果
                int sum = nums[i] + nums[b] + nums[c];

                // 特殊情况，等于target就是最优解了
                if (sum == target) {
                    return best;
                }

                // 当前结果更优则更新
                if (Math.abs(sum - target) < Math.abs(best - target)) {
                    best = sum;
                }

                // 移动指针
                // 如果当前结果比目标值大，则需要减少
                // 数组已经排序，所以缩短右边界，使用while处理重复值
                if (sum > target) {
                    int c0 = c - 1;
                    while (c0 > b && nums[c0] == nums[c]) {
                        c0--;
                    }
                    c = c0;
                } else {
                    int b0 = b;
                    while (b0 < c && nums[b] == nums[b0]) {
                        b0++;
                    }
                    b = b0;
                }
                // if (sum == target) {
                // return best;
                // } else if (sum > target) {
                // c--;
                // } else {
                // b++;
                // }
            }

        }

        return best;
    }

    // 分隔链表
    static ListNode partition(ListNode head, int x) {
        // 创建一个占位节点，这样可以使用next处理链表第一个节点
        ListNode one = new ListNode(0);
        ListNode oHead = one;
        ListNode two = new ListNode(0);
        ListNode tHead = two;
        while (head != null) {
            if (head.val < x) {
                one.next = head;
                one = one.next;
            } else {
                two.next = head;
                two = two.next;
            }
            head = head.next;
        }
        // 第二个链表指向的节点可能拥有下一个节点
        two.next = null;
        one.next = tHead.next;
        return oHead.next;
    }

    // 串联所有单词的子串
    static List<Integer> findSubstring(String s, String[] words) {
        int m = words.length;
        int n = words[0].length();
        int len = s.length();

        List<Integer> res = new ArrayList<>();

        // 初始滑动位置
        // |barfoofoo|barthefoobarman，第1次
        // b|arfoofoob|arthefoobarman，第2次
        // ba|rfoofooba|rthefoobarman，第3次
        // bar|foofoobar|thefoobarman，第n+1次，跟滑动窗口重叠
        for (int i = 0; i < n; i++) {
            if (i + m * n > len) {
                break;
            }

            // 对s创建窗口框住m个单词，每个单词长度为n
            // |barfoofoo|barthefoobarman
            // ["bar", "foo", "foo"]
            // 0单词下标i, 1单词下标i+n, 2单词下标i+n+n即i+j*n
            // 0单词末尾i+(n-1)即i+n-1, 1单词末尾i+n+(n-1)即i+2n-1, 2单词末尾i+n+n+(n-1)即i+3n-1即i+(j+1)*n
            // substring[)要+1
            Map<String, Integer> map = new HashMap<>();
            for (int j = 0; j < m; j++) {
                String word = s.substring(i + j * n, i + (j + 1) * n);
                map.put(word, map.getOrDefault(word, 0) + 1);
            }

            // 检查每个单词的命中
            // [ "bar", "foo", "the"]
            // [
            // "bar": 1
            // "foo": 2
            // ]
            // bar: 1-1=0 foo: 2-1=1 the: 0-1=-1
            for (String w : words) {
                map.put(w, map.getOrDefault(w, 0) - 1);
                if (map.get(w) == 0) {
                    map.remove(w);
                }
            }

            // 滑动窗口，每次滑动一个单词的长度n
            // |barfoofoo|barthefoobarman
            // bar|foofoobar|thefoobarman
            // ...
            // barfoofoobarthe|foobarman|
            //
            // len = 24, m * n = 9, substring[15,24)是最后的窗口位置，此时start=15，即<24-9+1=16
            for (int start = i; start < len - m * n + 1; start += n) {
                // start == i 时，前面已经进行过处理
                if (start != i) {
                    // 滑入的新单词下标start+m*n,因为start已经提前滑动，所以是start+m*n-n即start+(m-1)*n
                    // 滑出的新单词的末尾start+m*n+(n-1)，因为start已经提前滑动，所以是start+m*n-n+(n-1)即start+m*n-1
                    // substring[)要+1
                    String word = s.substring(start + (m - 1) * n, start + m * n);
                    map.put(word, map.getOrDefault(word, 0) + 1);
                    if (map.get(word) == 0) {
                        map.remove(word);
                    }

                    // 删除滑出窗口的 "bar"
                    word = s.substring(start - n, start);
                    map.put(word, map.getOrDefault(word, 0) - 1);
                    if (map.get(word) == 0) {
                        map.remove(word);
                    }
                }
                if (map.isEmpty()) {
                    res.add(start);
                }
            }

        }

        return res;
    }

    // 无重复字符的最长子串的长度
    static int lengthOfLongestSubstring(String s) {
        // 记录窗口框住的字符
        HashSet<Character> set = new HashSet<>();
        int res = 0;

        // 滑动窗口模板
        for (int left = 0, right = 0; right < s.length(); right++) {
            // 获取字符
            char c = s.charAt(right);

            // 检查字符是否重复
            while (set.contains(c)) {
                set.remove(s.charAt(left));
                left++;
            }

            res = Math.max(res, right - left + 1);
        }

        return res;

    }

    // 字母异位词分组
    static List<List<String>> groupAnagrams(String[] strs) {
        // 先使用hashmap对字符串进行分组，分完之后再转为二维数组
        HashMap<String, List<String>> map = new HashMap<>();

        for (int i = 0; i < strs.length; i++) {
            char[] chars = strs[i].toCharArray();
            Arrays.sort(chars);
            String key = new String(chars);

            List<String> list = map.getOrDefault(key, new ArrayList<>());
            list.add(strs[i]);
            map.put(key, list);
        }

        return new ArrayList<>(map.values());
    }
}
