package com.example;

import java.util.List;

public class Main {
    public static void main(String[] args) {
        String s = "abcabcbb";
        int res = SLib.lengthOfLongestSubstring(s);
        System.out.println(res);

        s = "bbbbb";
        res = SLib.lengthOfLongestSubstring(s);
        System.out.println(res);

        String[] strs = new String[] { "eat", "tea", "tan", "ate", "nat", "bat" };
        List<List<String>> list = SLib.groupAnagrams(strs);
        System.out.println("groupAnagrams res: " + list);
    }

}