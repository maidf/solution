package com.example;

import java.util.TreeMap;

public class MyCalendarThree {
    TreeMap<Integer, Integer> diff;

    public MyCalendarThree() {
        diff = new TreeMap<>();
    }

    public int book(int start, int end) {
        diff.put(start, diff.getOrDefault(start, 0) + 1);
        diff.put(end, diff.getOrDefault(end, 0) - 1);

        int active = 0;
        int k = 0;
        for (int time : diff.keySet()) {
            active += diff.get(time);
            k = Math.max(k, active);
        }
        return k;
    }
}