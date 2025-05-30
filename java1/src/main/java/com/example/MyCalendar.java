package com.example;

import java.util.TreeMap;

public class MyCalendar {
    TreeMap<Integer, Integer> map;

    public MyCalendar() {
        map = new TreeMap<>();
    }

    public boolean book(int startTime, int endTime) {
        map.put(startTime, map.getOrDefault(startTime, 0) + 1);
        map.put(endTime, map.getOrDefault(endTime, 0) - 1);
        int res = 0;
        for (int k : map.keySet()) {
            res += map.get(k);
            if (res > 1) {
                map.put(startTime, map.getOrDefault(startTime, 0) - 1);
                map.put(endTime, map.getOrDefault(endTime, 0) + 1);
                return false;
            }
        }
        return true;
    }
}
