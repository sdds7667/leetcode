import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;

class Solution {
    public long distinctNames(String[] ideas) {
        var suffix = new ArrayList<>();
        for (int i = 0; i < 26; i++)
            suffix.add(new HashSet<>());

        for (String idea : ideas)
            suffix.get(idea.charAt(0) - 'a').add(idea.substring(1));
        long answer = 0;
        for (int i = 0; i < 26; i++) {
            for (int j = i+1; j < 26; j++) {
                if (i == j) continue;
                var s1 = suffix.get(i);
                var s2 = suffix.get(j);
                if (s1.size() > s2.size()) {
                    var tmp = s1;
                    s1 = s2;
                    s2 = tmp;
                }

                var s = new HashSet<>(s1);
                s.retainAll(s2);

                answer += 2L * (s1.size() - s.size()) * (s2.size() - s.size());
            }
        }

        return answer;
    }
}
