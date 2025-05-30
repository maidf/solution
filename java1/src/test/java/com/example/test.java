package com.example;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.Executors;

import org.junit.jupiter.api.Test;

public class test {
    @Test
    void testLeastBricks() {
        List<List<Integer>> wall = new ArrayList<>() {
            {
                add(Arrays.asList(1, 2, 2, 1));
                add(Arrays.asList(3, 1, 2));
                add(Arrays.asList(1, 3, 2));
                add(Arrays.asList(2, 4));
                add(Arrays.asList(3, 1, 2));
                add(Arrays.asList(1, 3, 1, 1));
            }
        };
        int res = SLib.leastBricks(wall);
        assertEquals(2, res);
    }

    @Test
    void testReverseKGroup() {
        ListNode head = new ListNode(0);
        ListNode node = head;
        node.next = new ListNode(1);
        node = node.next;
        node.next = new ListNode(2);
        node = node.next;
        node.next = new ListNode(3);
        node = node.next;
        node.next = new ListNode(4);
        node = node.next;
        node.next = new ListNode(5);
        ListNode res = SLib.reverseKGroup(head.next, 3);
        while (res != null) {
            System.out.println("res: " + res.val);
            res = res.next;
        }
    }

    @Test
    void testVir() {
        // 测试：创建 10,000 个睡眠 1 秒的任务
        long start = System.currentTimeMillis();
        try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
            for (int i = 0; i < 10_000; i++) {
                executor.submit(() -> {
                    Thread.sleep(1000);
                    System.out.println(String.format("hello %s", Math.random() * 10000));
                    return "Done";
                });
            }
        }
        long duration = System.currentTimeMillis() - start;
        System.out.println("Virtual threads: " + duration + " ms");
    }

    @Test
    void testMyCalendar() {
        MyCalendar myCalendar = new MyCalendar();
        boolean res = false;
        res = myCalendar.book(20, 29);
        assertEquals(true, res);
        res = myCalendar.book(13, 22);
        assertEquals(false, res);
        res = myCalendar.book(44, 50);
        assertEquals(true, res);
        res = myCalendar.book(1, 7);
        assertEquals(true, res);
        res = myCalendar.book(2, 10);
        assertEquals(false, res);
    }

    @Test
    void testRotateRight() {
        ListNode node = new ListNode(0);
        ListNode head = node;
        node.next = new ListNode(1);
        node = node.next;
        node.next = new ListNode(2);
        node = node.next;
        node.next = new ListNode(3);
        node = node.next;
        node.next = new ListNode(4);
        node = node.next;
        node.next = new ListNode(5);
        node = node.next;
        ListNode res = SLib.rotateRight(head.next, 2);
        while (res != null) {
            System.out.println(String.format("res: %s", res.val));
            res = res.next;
        }
    }

    @Test
    void testReconstructQueue() {
        int[][] people = {
                { 7, 0 },
                { 4, 4 },
                { 7, 2 },
                { 5, 0 },
                { 6, 1 },
                { 5, 2 },
                { 7, 1 },
        };
        // people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]];
        // people[0][0] = 7;
        // people[0][1] = 0;
        // people[1][0] = 4;
        // people[1][1] = 4;
        // people[2][0] = 7;
        // people[2][1] = 2;
        // people[3][0] = 5;
        // people[3][1] = 0;
        // people[4][0] = 6;
        // people[4][1] = 1;
        // people[5][0] = 5;
        // people[5][1] = 2;
        // people[6][0] = 7;
        // people[6][1] = 1;
        int[][] res = SLib.reconstructQueue(people);
        for (int[] res2 : res) {
            System.out.println(String.format("res: [%d, %d]", res2[0], res2[1]));
        }
    }

    @Test
    void testThree1() {
        int[] nums = new int[] { 0, 3, 97, 102, 200 };
        Integer res = SLib.threeSumClosest(nums, 300);
        Integer expect = 300;
        assertEquals(expect, res);
    }

    @Test
    void testListNode1() {
        ListNode node = new ListNode(1,
                new ListNode(4, new ListNode(3, new ListNode(2, new ListNode(5, new ListNode(2, null))))));
        ListNode res = SLib.partition(node, 3);
        while (res != null) {
            System.out.println(res.val);
            res = res.next;
        }
    }

    @Test
    void test4() {
        String[] words = new String[] { "a" };
        List<Integer> res = SLib.findSubstring(new String("a"), words);
        List<Integer> expect = new ArrayList<>(List.of(0));
        assertEquals(expect, res);
    }

    @Test
    void test3() {
        String[] words = new String[] { "word", "good", "best", "good" };
        List<Integer> res = SLib.findSubstring(new String("wordgoodgoodgoodbestword"), words);
        List<Integer> expect = new ArrayList<>(List.of(8));
        assertEquals(expect, res);
    }

    @Test
    void test2() {
        String[] words = new String[] { "bar", "foo", "the" };
        List<Integer> res = SLib.findSubstring(new String("barfoofoobarthefoobarman"), words);
        List<Integer> expect = new ArrayList<>(List.of(6, 9, 12));
        assertEquals(expect, res);
    }

    @Test
    void test1() {
        String[] words = new String[] { "foo", "bar" };
        List<Integer> res = SLib.findSubstring(new String("barfoothefoobarman"), words);
        List<Integer> expect = new ArrayList<>(List.of(0, 9));
        assertEquals(expect, res);
        // System.out.println("res: " + res);
    }
}
