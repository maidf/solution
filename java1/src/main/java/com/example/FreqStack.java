package com.example;

import java.util.Deque;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Map;

// 895. 最大频率栈
public class FreqStack {
    // 记录每个值的数量
    Map<Integer, Integer> freq;
    // 每个数量设置一个队列
    Map<Integer, Deque<Integer>> group;
    // 记录数量最高的队列key
    int max;

    public FreqStack() {
        freq = new HashMap<>();
        group = new HashMap<>();
        max = 0;
    }

    public void push(int val) {
        // 获取该值的频率+1
        int k = freq.getOrDefault(val, 0) + 1;
        freq.put(val, k);

        // 往对应的队列添加值
        Deque<Integer> q = group.getOrDefault(k, new LinkedList<>());
        q.addFirst(val);
        group.put(k, q);

        // 更新队列key
        max = Math.max(max, k);
    }

    public int pop() {
        // 取出频率最高队列的第一个值
        int val = group.get(max).poll();
        // 减少该值的数量
        freq.put(val, freq.get(val) - 1);
        // 更新key
        if (group.get(max).isEmpty()) {
            max--;
        }
        return val;
    }
}
